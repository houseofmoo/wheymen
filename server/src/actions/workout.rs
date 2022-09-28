use super::{
    helper::{get_all_results, get_first_result},
};
use crate::{
    model::{
        db::{InsertWorkoutRow, WorkoutRow},
        error::LocalError,
        shared_types::DbResult,
    },
    resource::client::DbClient,
};

pub async fn get_all_workouts(user_id: &String, client: &DbClient) -> DbResult<Vec<WorkoutRow>> {
    let query = format!(
        "SELECT * FROM workouts WHERE user_id=\"{}\" ORDER BY category;",
        user_id
    );
    let result = client.send_query::<WorkoutRow>(query).await?;

    match get_all_results::<WorkoutRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_all_workout_unrelated_to_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Vec<WorkoutRow>> {
    // get the routine then filter manually?

    let query = format!(
        "SELECT * FROM workouts WHERE user_id=\"{}\" AND <-workout<-routines.id CONTAINSNOT \"{}\" ORDER BY category;",
        user_id, routine_id
    );
    let result = client.send_query::<WorkoutRow>(query).await?;

    match get_all_results::<WorkoutRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_workout(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    let query = format!("SELECT * FROM {} WHERE user_id=\"{}\"", workout_id, user_id);
    let result = client.send_query::<WorkoutRow>(query).await?;

    match get_first_result::<WorkoutRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn insert_workout(
    workout_row: &InsertWorkoutRow,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    let json = serde_json::json!(workout_row);
    let query = format!("INSERT INTO workouts {};", json);
    let result = client.send_query::<WorkoutRow>(query).await?;

    match get_first_result::<WorkoutRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Err(LocalError::InsertFailed),
    }
}

pub async fn update_workout(
    workout_row: &WorkoutRow,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    let json = serde_json::json!(workout_row);
    let query = format!("UPDATE {} CONTENT {}", workout_row.id, json);
    let result = client.send_query::<WorkoutRow>(query).await?;

    match get_first_result::<WorkoutRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Err(LocalError::InsertFailed),
    }
}

pub async fn delete_workout(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    let query = format!("DELETE {} WHERE user_id=\"{}\";", workout_id, user_id);
    client.send_query::<WorkoutRow>(query).await?;
    Ok(None)
}
