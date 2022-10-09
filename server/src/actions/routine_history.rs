use super::helper::{get_all_results, get_first_result, get_iso_time_now};
use crate::{
    model::{
        db::Table, routine::RoutineHistoryRow, shared_types::DbResult, workout::WorkoutHistoryRow,
    },
    resource::client::DbClient,
};

pub async fn clean_routine_history(
    user_id: &String,
    client: &DbClient,
) -> DbResult<RoutineHistoryRow> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" AND completed_on + 26w < \"{}\";",
        Table::RoutineHistory.name(),
        user_id,
        get_iso_time_now()
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

pub async fn delete_routine_history(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<WorkoutHistoryRow> {
    Ok(None)
}
