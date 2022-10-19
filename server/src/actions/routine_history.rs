use super::helper::{get_all_results, get_first_result, get_iso_time_now};
use crate::{
    model::{
        db::Table, 
        routine::{RoutineHistoryRow, InsertRoutineHistoryRow}, 
        shared_types::DbResult, workout::WorkoutHistoryRow,
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

pub async fn _get_routine_history(
    _user_id: &String,
    _routine_id: &String,
    _client: &DbClient,
) -> DbResult<Vec<RoutineHistoryRow>> {
    Ok(None)
}

pub async fn insert_routine_history(
    routine_history_row: InsertRoutineHistoryRow,
    client: &DbClient,
) -> DbResult<RoutineHistoryRow> {
    let json = serde_json::json!(routine_history_row);
    let query = format!("INSERT INTO {} {};", Table::RoutineHistory.name(), json);
    let result = client.send_query::<RoutineHistoryRow>(query).await?;

    match get_first_result::<RoutineHistoryRow>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn delete_routine_history(
    _user_id: &String,
    _routine_id: &String,
    _client: &DbClient,
) -> DbResult<WorkoutHistoryRow> {
    Ok(None)
}
