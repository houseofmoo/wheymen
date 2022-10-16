use super::shared_types::{RoutineRef, SessionRef, UserRef, WorkoutRef};
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: SessionRef,
    pub user_id: UserRef,
    pub routine_id: RoutineRef,
    pub routine_name: String,
    pub routine_note: String,
    pub start_time: String,
    pub duration_in_sec: i32,
    pub workouts: Vec<SessionWorkout>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertSession {
    pub user_id: UserRef,
    pub routine_id: RoutineRef,
    pub routine_name: String,
    pub routine_note: String,
    pub start_time: String,
    pub duration_in_sec: i32,
    pub workouts: Vec<SessionWorkout>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionWorkout {
    pub workout_id: WorkoutRef,
    pub workout_name: String,
    pub workout_note: String,
    pub sets: Vec<SessionSet>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionSet {
    #[serde(deserialize_with = "de_weight")]
    pub weight: f32,
    pub reps: i32,
    pub complete: bool,
}

fn de_weight<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f32,
        _ => return Err(de::Error::custom("wrong type")),
    })
}
