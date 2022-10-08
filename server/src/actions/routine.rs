use super::helper::{get_all_results, get_first_result};
use crate::model::{
    error::LocalError,
    routine::{InsertRoutineRow, Routine, RoutineRow},
    shared_types::DbResult, db::Table,
};
use crate::resource::client::DbClient;

// Get all routines for a user (fetch)
pub async fn get_all_routines(user_id: &String, client: &DbClient) -> DbResult<Vec<Routine>> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" ORDER BY name FETCH workouts;",
        Table::Routines.name(), user_id
    );
    let result = client.send_query::<Routine>(query).await?;

    match get_all_results::<Routine>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

// Get a specific routine (fetch)
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

// Get a specific routine row (no fetch)
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

// Insert a new routine
pub async fn insert_routine(
    user_id: &String,
    routine_row: &InsertRoutineRow,
    client: &DbClient,
) -> DbResult<Routine> {
    let json = serde_json::json!(routine_row);
    let query = format!(
        "INSERT INTO {} {};", 
        Table::Routines.name(), json
    );
    let result = client.send_query::<RoutineRow>(query).await?;

    match get_first_result::<RoutineRow>(result) {
        Some(r) => get_routine(&user_id, &r.id, &client).await,
        None => Err(LocalError::InsertFailed),
    }
}

// Updates an existing routine
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

// Deletes a routine
pub async fn delete_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    let query = format!("DELETE {} WHERE user_id=\"{}\";", routine_id, user_id);
    client.send_query::<Routine>(query).await?;
    Ok(None)
}

// Adds a workout reference to provided list of routines
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
        routine_ids.join(","), workout_id,
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}

// Removes a workout reference from the provided list of routines
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
        routine_ids.join(","), workout_id,
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}

// Removes a workout reference from all routines associated to a user
pub async fn remove_workout_from_all_user_routines(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<RoutineRow> {
    let query = format!(
        "UPDATE {} SET workouts -= [\"{}\"] WHERE user_id=\"{}\";",
        Table::Routines.name(), user_id, workout_id
    );
    client.send_query::<RoutineRow>(query).await?;
    Ok(None)
}