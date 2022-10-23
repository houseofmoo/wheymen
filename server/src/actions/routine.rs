use super::helper::{get_all_results, get_first_result};
use crate::model::{
    db::Table,
    error::LocalError,
    routine::{Routine, RoutineRow},
    shared_types::DbResult,
};
use crate::resource::client::DbClient;

pub async fn get_all_routines(user_id: &String, client: &DbClient) -> DbResult<Vec<Routine>> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" ORDER BY name FETCH workouts;",
        Table::Routines.name(),
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
    routine_row: &RoutineRow,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    let json = serde_json::json!(routine_row);
    let query = format!("INSERT INTO {} {};", Table::Routines.name(), json);
    let result = client.send_query::<RoutineRow>(query).await?;

    match get_first_result::<RoutineRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Err(LocalError::InsertFailed),
    }
}

pub async fn update_routine(
    user_id: &String,
    routine_row: &RoutineRow,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    if let Some(routine_id) = &routine_row.id {
        let json = serde_json::json!(routine_row);
        let query = format!("UPDATE {} CONTENT {};", routine_id, json);
        let result = client.send_query::<RoutineRow>(query).await?;
    
        match get_first_result::<RoutineRow>(result) {
            Some(r) => return Ok(Some(r)),
            None => return Err(LocalError::UpdateFailed),
        }
    }

    Err(LocalError::NoIDProvided)
}

pub async fn delete_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    // to prevent a session from using a dangling reference, just delete all sessions
    super::session::delete_all_sessions(user_id, client).await?;

    // delete history for this routine
    super::routine_history::delete_routine_history(user_id, routine_id, client).await?;

    // delete the routine
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

    let query = format!(
        "UPDATE {} SET workouts += [\"{}\"];",
        routine_ids.join(","),
        workout_id,
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

    let query = format!(
        "UPDATE {} SET workouts -= [\"{}\"];",
        routine_ids.join(","),
        workout_id,
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}

pub async fn remove_workout_from_all_user_routines(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    let query = format!(
        "UPDATE {} SET workouts -= [\"{}\"] WHERE user_id=\"{}\";",
        Table::Routines.name(),
        workout_id,
        user_id,
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}

pub async fn set_workouts(
    user_id: &String,
    routine_id: &String,
    workout_ids: Vec<String>,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    let query = format!(
        "UPDATE {} SET workouts={} WHERE user_id=\"{}\";",
        routine_id,
        serde_json::json!(workout_ids),
        user_id,
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}