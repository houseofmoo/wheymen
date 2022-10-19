use super::{
    shared_types::{RoutineHistoryRef, RoutineRef, UserRef, WorkoutRef, Set},
    workout::Workout,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Routine {
    pub id: RoutineRef,
    pub user_id: UserRef,
    pub name: String,
    pub days: Vec<String>,
    pub last_completed: String,
    pub note: String,
    pub workouts: Vec<Workout>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoutineRow {
    pub id: RoutineRef,
    pub user_id: UserRef,
    pub name: String,
    pub days: Vec<String>,
    pub last_completed: String,
    pub note: String,
    pub workouts: Vec<WorkoutRef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertRoutineRow {
    pub user_id: UserRef,
    pub name: String,
    pub days: Vec<String>,
    pub last_completed: String,
    pub note: String,
    pub workouts: Vec<WorkoutRef>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoutineHistoryRow {
    pub id: RoutineHistoryRef,
    pub user_id: UserRef,
    pub routine_id: RoutineRef,
    pub name: String,
    pub completed_on: String,
    pub duration_in_sec: i32,
    pub workouts: Vec<RoutineHistoryWorkout>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertRoutineHistoryRow {
    pub user_id: UserRef,
    pub routine_id: RoutineRef,
    pub name: String,
    pub completed_on: String,
    pub duration_in_sec: i32,
    pub workouts: Vec<RoutineHistoryWorkout>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RoutineHistoryWorkout {
    pub name: String,
    pub sets: Vec<Set>,
}