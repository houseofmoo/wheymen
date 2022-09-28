use crate::model::db::{Relationship, WorkoutRow};
use crate::model::{data::Routine, error::LocalError, shared_types::DbResult};
use crate::resource::client::DbClient;

use super::helper::{get_all_results, get_first_result};

#[allow(dead_code)]
pub async fn create_relationship(
    user_id: &String,
    routine_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    match does_relationship_exist(routine_id, workout_id, &user_id, &client).await {
        Ok(exists) => {
            if exists {
                return Ok(None);
            }
        }
        Err(e) => return Err(e),
    };

    let query = format!(
        "RELATE {}->workout->{} SET user_id=\"{}\";",
        routine_id, workout_id, user_id
    );
    let result = client.send_query::<Relationship>(query).await?;

    if result.len() > 0 && result[0].result.len() > 0 {
        Ok(None)
    } else {
        Err(LocalError::RelationshipFailed)
    }
}

#[allow(dead_code)]
pub async fn create_many_routine_relationships(
    user_id: &String,
    routine_id: &String,
    workout_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<Routine> {
    for workout_id in workout_ids {
        match create_relationship(&user_id, &routine_id, &workout_id, &client).await {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(None)
}

#[allow(dead_code)]
pub async fn create_many_workout_relationships(
    user_id: &String,
    workout_id: &String,
    routine_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    for routine_id in routine_ids {
        match create_relationship(&user_id, &routine_id, &workout_id, &client).await {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }
    Ok(None)
}

#[allow(dead_code)]
pub async fn delete_all_routine_relationships(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    let query = format!(
        "DELETE workout WHERE in=\"{}\" AND user_id=\"{}\";",
        routine_id, user_id
    );
    client.send_query::<Relationship>(query).await?;
    Ok(None)
}

#[allow(dead_code)]
pub async fn delete_all_workout_relationships(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<Routine> {
    let query = format!(
        "DELETE workout WHERE out=\"{}\" AND user_id=\"{}\";",
        workout_id, user_id
    );
    client.send_query::<Relationship>(query).await?;
    Ok(None)
}

#[allow(dead_code)]
async fn does_relationship_exist(
    user_id: &String,
    routine_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> Result<bool, LocalError> {
    let query = format!(
        "SELECT * FROM workout WHERE in=\"{}\" AND out=\"{}\" AND user_id=\"{}\";",
        routine_id, workout_id, user_id
    );
    let result = client.send_query::<Relationship>(query).await?;
    return Ok(result.len() > 0 && result[0].result.len() > 0);
}

// Example of getting all routines and their related workouts
#[allow(dead_code)]
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

// Example of getting a routine and the related workouts
#[allow(dead_code)]
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

// return all workouts that a routine does not contain using relationships
#[allow(dead_code)]
pub async fn get_all_unrelated_workouts(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Vec<WorkoutRow>> {
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