use super::shared_types::{RoutineRef, SessionRef, UserRef, WorkoutRef};
use serde::{Deserialize, Serialize};

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
    pub weight: i32,
    pub reps: i32,
    pub complete: bool,
}
