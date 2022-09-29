use super::helper::{get_all_results, get_first_result};
use crate::model::{
    data::Routine, db::InsertRoutineRow, db::RoutineRow, error::LocalError, shared_types::DbResult,
};
use crate::resource::client::DbClient;

pub async fn get_all_routines(user_id: &String, client: &DbClient) -> DbResult<Vec<Routine>> {
    let query = format!(
        "SELECT * FROM routines WHERE user_id=\"{}\" FETCH workouts;",
        user_id
    );
    let result = client.send_query::<Routine>(query).await?;

    match get_all_results::<Routine>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" FETCH workouts;",
        routine_id, user_id
    );
    let result = client.send_query::<Routine>(query).await?;

    match get_first_result::<Routine>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_routine_row(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\";",
        routine_id, user_id
    );
    let result = client.send_query::<RoutineRow>(query).await?;

    match get_first_result::<RoutineRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn insert_routine(
    user_id: &String,
    routine_row: &InsertRoutineRow,
    client: &DbClient,
) -> DbResult<Routine> {
    let json = serde_json::json!(routine_row);
    let query = format!("INSERT INTO routines {};", json);
    let result = client.send_query::<RoutineRow>(query).await?;

    match get_first_result::<RoutineRow>(result) {
        Some(r) => get_routine(&user_id, &r.id, &client).await,
        None => Err(LocalError::InsertFailed),
    }
}

pub async fn update_routine(
    user_id: &String,
    routine_row: &RoutineRow,
    client: &DbClient,
) -> DbResult<Routine> {
    let json = serde_json::json!(routine_row);
    let query = format!("UPDATE {} CONTENT {};", routine_row.id, json);
    let result = client.send_query::<RoutineRow>(query).await?;

    match get_first_result::<RoutineRow>(result) {
        Some(r) => get_routine(&user_id, &r.id, &client).await,
        None => Err(LocalError::UpdateFailed),
    }
}

pub async fn delete_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    let query = format!("DELETE {} WHERE user_id=\"{}\";", routine_id, user_id);
    client.send_query::<Routine>(query).await?;
    Ok(None)
}

pub async fn add_workout_to_many_routines(
    routine_ids: &Vec<String>,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    if routine_ids.len() <= 0 {
        return Ok(None);
    }

    let routine_ids_str = routine_ids.join(",");
    let query = format!(
        "UPDATE {} SET workouts += [\"{}\"];",
        routine_ids_str, workout_id,
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}

pub async fn remove_workout_from_many_routines(
    routine_ids: &Vec<String>,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    if routine_ids.len() <= 0 {
        return Ok(None);
    }

    let routine_ids_str = routine_ids.join(",");
    let query = format!(
        "UPDATE {} SET workouts -= [\"{}\"];",
        routine_ids_str, workout_id,
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}
