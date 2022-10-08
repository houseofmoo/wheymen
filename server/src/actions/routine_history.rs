use super::helper::{get_all_results, get_first_result};
use crate::{
    model::{routine::RoutineHistoryRow, shared_types::DbResult, workout::WorkoutHistoryRow},
    resource::client::DbClient,
};
use chrono::Utc;

pub async fn clean_routine_history(
    user_id: &String,
    client: &DbClient,
) -> DbResult<RoutineHistoryRow> {
    let current_datetime = Utc::now();
    let query = format!(
        "SELECT * FROM routine_history WHERE user_id=\"{}\" AND completed_on + 26w < \"{}\";",
        user_id, current_datetime
    );
    let result = client.send_query::<RoutineHistoryRow>(query).await?;

    let row = match get_all_results::<RoutineHistoryRow>(result) {
        Some(r) => r,
        None => return Ok(None),
    };
    let routine_history_ids: Vec<String> = row.into_iter().map(|r| r.id).collect();

    let query = format!(
        "DELETE {} WHERE user_id=\"{}\";",
        routine_history_ids.join(", "),
        user_id
    );
    client.send_query::<RoutineHistoryRow>(query).await?;
    Ok(None)
}

pub async fn get_routine_history(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Vec<WorkoutHistoryRow>> {
    Ok(None)
}

pub async fn insert_routine_history(
    routine_history_row: RoutineHistoryRow,
    client: &DbClient,
) -> DbResult<WorkoutHistoryRow> {
    Ok(None)
}

pub async fn delete_all_history_for_routine(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<WorkoutHistoryRow> {
    Ok(None)
}