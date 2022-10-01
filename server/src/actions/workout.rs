use super::helper::{get_all_results, get_first_result};
use crate::{
    model::{
        error::LocalError,
        shared_types::DbResult,
        workout::{InsertWorkoutRow, WorkoutRow},
    },
    resource::client::DbClient,
};

pub async fn get_all_workouts(user_id: &String, client: &DbClient) -> DbResult<Vec<WorkoutRow>> {
    let query = format!(
        "SELECT * FROM workouts WHERE user_id=\"{}\" ORDER BY category, name;",
        user_id
    );
    let result = client.send_query::<WorkoutRow>(query).await?;

    match get_all_results::<WorkoutRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_all_workouts_unrelated_to_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Vec<WorkoutRow>> {
    // get the routine, if cannot find this routine just return all workouts
    let routine = match super::routine::get_routine_row(&user_id, &routine_id, &client).await {
        Ok(r) => match r {
            Some(r) => r,
            None => return get_all_workouts(&user_id, &client).await,
        },
        Err(e) => return Err(e),
    };

    // get all workouts for the user
    let workouts = match get_all_workouts(&user_id, &client).await {
        Ok(w) => match w {
            Some(w) => w,
            None => return Ok(None),
        },
        Err(e) => return Err(e),
    };

    // filter workouts that exist in the routine
    let filtered: Vec<WorkoutRow> = workouts
        .into_iter()
        .filter(|w| !routine.workouts.contains(&w.id))
        .collect();

    // return the filtered collection
    if filtered.len() > 0 {
        Ok(Some(filtered))
    } else {
        Ok(None)
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
    selected_ids: &Vec<String>,
    unselected_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    let json = serde_json::json!(workout_row);
    let query = format!("INSERT INTO workouts {};", json);
    let result = client.send_query::<WorkoutRow>(query).await?;

    let workout = match get_first_result::<WorkoutRow>(result) {
        Some(w) => w,
        None => return Err(LocalError::InsertFailed),
    };

    super::routine::add_workout_to_many_routines(&selected_ids, &workout.id, &client).await?;
    super::routine::remove_workout_from_many_routines(&unselected_ids, &workout.id, &client)
        .await?;
    Ok(Some(workout))
}

pub async fn update_workout(
    workout_row: &WorkoutRow,
    selected_ids: &Vec<String>,
    unselected_ids: &Vec<String>,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    let json = serde_json::json!(workout_row);
    let query = format!("UPDATE {} CONTENT {}", workout_row.id, json);
    let result = client.send_query::<WorkoutRow>(query).await?;

    let workout = match get_first_result::<WorkoutRow>(result) {
        Some(w) => w,
        None => return Err(LocalError::InsertFailed),
    };

    super::routine::add_workout_to_many_routines(&selected_ids, &workout.id, &client).await?;
    super::routine::remove_workout_from_many_routines(&unselected_ids, &workout.id, &client)
        .await?;
    Ok(Some(workout))
}

pub async fn delete_workout(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<WorkoutRow> {
    let query = format!("DELETE {} WHERE user_id=\"{}\";", workout_id, user_id);

    match super::routine::get_all_routine_rows(&user_id, &client).await {
        Ok(r) => match r {
            Some(r) => {
                // remove workout id from routines then delete workout
                let routine_ids = r.into_iter().map(|r| r.id).collect();
                super::routine::remove_workout_from_many_routines(
                    &routine_ids,
                    &workout_id,
                    &client,
                )
                .await?;
                client.send_query::<WorkoutRow>(query).await?;
                Ok(None)
            }
            None => {
                // just delete workout
                client.send_query::<WorkoutRow>(query).await?;
                Ok(None)
            }
        },
        Err(e) => Err(e),
    }
}
