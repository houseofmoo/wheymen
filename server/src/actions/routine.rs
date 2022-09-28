use super::helper::{get_all_results, get_first_result};
use super::relate;
use crate::model::db::RoutineRow;
use crate::model::{
    data::Routine, db::InsertRoutineRow, error::LocalError, shared_types::DbResult,
};
use crate::resource::client::DbClient;

pub async fn get_all_routines(user_id: &String, client: &DbClient) -> DbResult<Vec<Routine>> {
    let query = format!(
        "SELECT *, ->workout->workouts AS workouts FROM routines WHERE user_id=\"{}\" FETCH workouts;",
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
        "SELECT *, ->workout->workouts AS workouts FROM {} WHERE user_id=\"{}\" FETCH workouts;",
        routine_id, user_id
    );
    let result = client.send_query::<Routine>(query).await?;

    match get_first_result::<Routine>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn insert_routine(
    user_id: &String,
    routine_row: &InsertRoutineRow,
    workout_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<Routine> {
    let json = serde_json::json!(routine_row);
    let query = format!("INSERT INTO routines {}", json);
    let result = client.send_query::<RoutineRow>(query).await?;

    let id = match get_first_result::<RoutineRow>(result) {
        Some(r) => r.id,
        None => return Err(LocalError::InsertFailed),
    };

    relate::delete_all_routine_relationships(&id, &client).await?;
    relate::create_many_routine_relationships(&id, &workout_ids, client).await?;
    get_routine(&user_id, &id, &client).await
}

pub async fn update_routine(
    user_id: &String,
    routine_row: &RoutineRow,
    workout_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<Routine> {
    let json = serde_json::json!(routine_row);
    let query = format!("UPDATE {} CONTENT {}", routine_row.id, json);
    let result = client.send_query::<RoutineRow>(query).await?;

    let id = match get_first_result::<RoutineRow>(result) {
        Some(r) => r.id,
        None => return Err(LocalError::UpdateFailed),
    };

    relate::delete_all_routine_relationships(&id, &client).await?;
    relate::create_many_routine_relationships(&id, &workout_ids, client).await?;
    get_routine(&user_id, &id, &client).await // return routine with updated workout list
}

pub async fn delete_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    let query = format!("DELETE {} WHERE user_id=\"{}\"", routine_id, user_id);
    client.send_query::<Routine>(query).await?;
    Ok(None) // delete returns nothing
}
