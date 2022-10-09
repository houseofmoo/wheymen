use chrono::Utc;
use super::helper::{get_all_results, get_first_result};
use crate::{
    model::{shared_types::DbResult, db::Table, session::{Session, InsertSession, SessionWorkout}, error::LocalError},
    resource::client::DbClient,
};

pub async fn get_all_sessions(
    user_id: &String,
    client: &DbClient,
) -> DbResult<Vec<Session>> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" ORDER BY start_time;",
        Table::Sessions.name(), user_id
    );
    let result = client.send_query::<Session>(query).await?;
    
    match get_all_results::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn get_session(
    user_id: &String,
    session_id: &String,
    client: &DbClient,
) -> DbResult<Session> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\";",
        session_id, user_id
    );
    let result = client.send_query::<Session>(query).await?;

    match get_first_result::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn start_session(
    user_id: &String,
    routine_id: &String,
    client: &DbClient,
) -> DbResult<Session> {
    let routine = match super::routine::get_routine(user_id, routine_id, client).await? {
        Some(r) =>  r,
        None => return Err(LocalError::GetFailed)
    };

    let json = serde_json::json!(InsertSession {
        user_id: user_id.to_string(),
        routine_id: routine.id,
        routine_name: routine.name,
        routine_note: routine.note,
        start_time: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        duration_in_sec: 0,
        workouts: routine.workouts.into_iter().map(|w| {
            SessionWorkout {
                workout_id: w.id,
                workout_name: w.name,
                workout_note: w.note,
                sets: vec![]
            }
        }).collect()
    });
    let query = format!("INSERT INTO {} {};", Table::Sessions.name(), json);
    let result = client.send_query::<Session>(query).await?;

    match get_first_result::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Err(LocalError::InsertFailed),
    }

}

pub async fn end_session(
    user_id: &String,
    session_id: &String,
    client: &DbClient,
) -> DbResult<Session> {
    let query = format!("DELETE {} WHERE user_id=\"{}\";", session_id, user_id);
    client.send_query::<Session>(query).await?;
    Ok(None)
}

pub async fn update_session(
    session: &Session,
    client: &DbClient,
) -> DbResult<Session> {
    let json = serde_json::json!(session);
    let query = format!("UPDATE {} CONTENT {};", session.id, json);
    let result = client.send_query::<Session>(query).await?;
    
    match get_first_result::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn find_session_by_routine_id(
    user_id: &String,
    routine_id: &String,
    client: &DbClient
) -> DbResult<Session> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" AND routine_id=\"{}\";", 
        Table::Sessions.name(), user_id, routine_id);
    let result = client.send_query::<Session>(query).await?;
    match get_first_result::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}