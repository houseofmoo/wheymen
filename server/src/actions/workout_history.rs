use super::helper::{get_all_results, get_first_result, get_iso_time_now};
use crate::{
    model::{db::Table, shared_types::DbResult, workout::WorkoutHistoryRow},
    resource::client::DbClient,
};

pub async fn clean_workout_history(
    user_id: &String,
    client: &DbClient,
) -> DbResult<WorkoutHistoryRow> {
    // get all workout histories that are more than 26 weeks
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" AND completed_on + 26w < \"{}\";",
        Table::WorkoutHistory.name(),
        user_id,
        get_iso_time_now()
    );
    let result = client.send_query::<WorkoutHistoryRow>(query).await?;

    // if anything matched the query, get the id's to those items
    let rows = match get_all_results::<WorkoutHistoryRow>(result) {
        Some(r) => r,
        None => return Ok(None),
    };
    let workout_history_ids: Vec<String> = rows.into_iter().map(|w| w.id).collect();

    // delete all id's that are older than 26 weeks
    let query = format!("DELETE {};", workout_history_ids.join(", "));
    client.send_query::<WorkoutHistoryRow>(query).await?;
    Ok(None)
}

pub async fn get_workout_history(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<Vec<WorkoutHistoryRow>> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" AND workout=\"{}\" ORDER BY completed_on FETCH workout;",
        Table::WorkoutHistory.name(), user_id, workout_id
    );
    let result = client.send_query::<WorkoutHistoryRow>(query).await?;

    match get_all_results::<WorkoutHistoryRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn insert_workout_history(
    workout_history_row: WorkoutHistoryRow,
    client: &DbClient,
) -> DbResult<WorkoutHistoryRow> {
    let json = serde_json::json!(workout_history_row);
    let query = format!("INSERT INTO {} {};", Table::WorkoutHistory.name(), json);
    let result = client.send_query::<WorkoutHistoryRow>(query).await?;

    match get_first_result::<WorkoutHistoryRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn delete_workout_history(
    user_id: &String,
    workout_id: &String,
    client: &DbClient,
) -> DbResult<WorkoutHistoryRow> {
    let query = format!(
        "DELETE {} WHERE user_id=\"{}\" AND workout_id=\"{}\";",
        Table::WorkoutHistory.name(),
        user_id,
        workout_id
    );
    let result = client.send_query::<WorkoutHistoryRow>(query).await?;

    match get_first_result::<WorkoutHistoryRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}
