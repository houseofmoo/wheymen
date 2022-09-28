use super::{helper::{get_all_results, get_first_result}, relate};
use crate::{
    model::{data::Workout, db::InsertWorkoutRow, error::LocalError, shared_types::DbResult},
    resource::client::DbClient,
};

pub async fn get_all_workouts(user_id: &String, client: &DbClient) -> DbResult<Vec<Workout>> {
    let query = format!(
        "SELECT * FROM workouts WHERE user_id=\"{}\" ORDER BY category FETCH routines;",
        user_id
    );
    let result = client.send_query::<Workout>(query).await?;

    match get_all_results::<Workout>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_all_unrelated_workouts(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Vec<Workout>> {
    let query = format!(
        "SELECT * FROM workouts WHERE user_id=\"{}\" AND <-workout<-routines.id CONTAINSNOT \"{}\" ORDER BY category;",
        user_id, routine_id
    );
    let result = client.send_query::<Workout>(query).await?;

    match get_all_results::<Workout>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_workout(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<Workout> {
    let query = format!("SELECT * FROM {} WHERE user_id=\"{}\"", workout_id, user_id);
    let result = client.send_query::<Workout>(query).await?;

    match get_first_result::<Workout>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn insert_workout(
    user_id: &String,
    workout_row: &InsertWorkoutRow,
    routine_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<Workout> {
    let json = serde_json::json!(workout_row);
    let query = format!("INSERT INTO workouts {};", json);
    let result = client.send_query::<Workout>(query).await?;

    let id = match get_first_result::<Workout>(result) {
        Some(r) => r.id,
        None => return Err(LocalError::InsertFailed),
    };

    relate::delete_all_workout_relationships(&id, &client).await?;
    relate::create_many_workout_relationships(&id, &routine_ids, &client).await?;
    get_workout(&user_id, &id, &client).await
}

pub async fn update_workout(
    user_id: &String,
    workout_row: &Workout,
    routine_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<Workout> {
    let json = serde_json::json!(workout_row);
    let query = format!("UPDATE {} CONTENT {}", workout_row.id, json);
    let result = client.send_query::<Workout>(query).await?;

    let id = match get_first_result::<Workout>(result) {
        Some(r) => r.id,
        None => return Err(LocalError::InsertFailed),
    };

    relate::delete_all_workout_relationships(&id, &client).await?;
    relate::create_many_workout_relationships(&id, &routine_ids, &client).await?;
    get_workout(&user_id, &id, &client).await
}

pub async fn delete_workout(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<Workout> {
    let query = format!("DELETE {} WHERE user_id=\"{}\";", workout_id, user_id);
    client.send_query::<Workout>(query).await?;
    Ok(None) // delete request returns nothing
}
