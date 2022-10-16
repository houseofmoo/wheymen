use super::helper::{get_all_results, get_first_result, get_iso_time_now};
use crate::{
    model::{
        db::Table,
        error::LocalError,
        session::{InsertSession, Session, SessionWorkout},
        shared_types::DbResult,
    },
    resource::client::DbClient,
};

pub async fn get_all_sessions(user_id: &String, client: &DbClient) -> DbResult<Vec<Session>> {
    let query = format!(
        "SELECT * FROM {} WHERE user_id=\"{}\" ORDER BY start_time;",
        Table::Sessions.name(),
        user_id
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
    // delete any session that already exists
    delete_all_sessions(user_id, client).await?;

    // get the routine
    let routine = match super::routine::get_routine(user_id, routine_id, client).await? {
        Some(r) => r,
        None => return Err(LocalError::GetFailed),
    };

    // create a session from the routine
    let json = serde_json::json!(InsertSession {
        user_id: user_id.to_string(),
        routine_id: routine.id,
        routine_name: routine.name,
        routine_note: routine.note,
        start_time: get_iso_time_now(),
        duration_in_sec: 0,
        workouts: routine
            .workouts
            .into_iter()
            .map(|w| {
                SessionWorkout {
                    workout_id: w.id,
                    workout_name: w.name,
                    workout_note: w.note,
                    sets: vec![],
                }
            })
            .collect()
    });
    let query = format!("INSERT INTO {} {};", Table::Sessions.name(), json);
    let result = client.send_query::<Session>(query).await?;

    match get_first_result::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Err(LocalError::InsertFailed),
    }
}

pub async fn delete_session(
    user_id: &String,
    session_id: &String,
    client: &DbClient,
) -> DbResult<Session> {
    let query = format!("DELETE {} WHERE user_id=\"{}\";", session_id, user_id);
    client.send_query::<Session>(query).await?;
    Ok(None)
}

pub async fn delete_all_sessions(user_id: &String, client: &DbClient) -> DbResult<Session> {
    let query = format!(
        "DELETE {} WHERE user_id=\"{}\";",
        Table::Sessions.name(),
        user_id
    );
    client.send_query::<Session>(query).await?;
    Ok(None)
}

pub async fn update_session(session: &Session, client: &DbClient) -> DbResult<Session> {
    let json = serde_json::json!(session);
    let query = format!("UPDATE {} CONTENT {};", session.id, json);
    let result = client.send_query::<Session>(query).await?;

    match get_first_result::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}

pub async fn complete_session(user_id: &String, session: Session, client: &DbClient) -> DbResult<Session> {
    // update the routine with the workouts in the session
    let workout_ids: Vec<String> = session.workouts.into_iter().map(|w| w.workout_id.clone()).collect();
    super::routine::set_workouts(user_id, &session.routine_id, workout_ids, client).await?;

    // let mut workout_ids: Vec<String> = vec![];
    // for workout in session.workouts {
    //     workout_ids.push(workout.workout_id.clone());
    // }
    // super::routine::set_workouts(user_id, &session.routine_id, workout_ids, client).await?;

    // create WorkoutHistory objects for each workout and store them
    // create RoutineHistory object for routine and store it
    // return nothing?

    for workout in session.workouts {

    }

    let json = serde_json::json!(session);
    let query = format!("UPDATE {} CONTENT {};", session.id, json);
    let result = client.send_query::<Session>(query).await?;

    match get_first_result::<Session>(result) {
        Some(r) => Ok(Some(r)),
        None => Ok(None),
    }
}
