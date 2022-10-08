use super::shared_types::{UserRef, WorkoutHistoryRef, WorkoutRef};
use serde::{Deserialize, Serialize};

pub type WorkoutRow = Workout;

#[derive(Debug, Deserialize, Serialize)]
pub struct Workout {
    pub id: WorkoutRef,
    pub user_id: UserRef,
    pub name: String,
    pub category: String,
    pub note: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertWorkoutRow {
    pub user_id: UserRef,
    pub name: String,
    pub category: String,
    pub note: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpsertWorkoutRow<T> {
    pub workout_row: T,
    pub selected_routine_ids: Vec<String>,
    pub unselected_routine_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutHistory {
    pub id: WorkoutHistoryRef,
    pub workout: Workout,
    pub user_id: UserRef,
    pub completed_on: String,
    pub sets: Vec<Set>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutHistoryRow {
    pub id: WorkoutHistoryRef,
    pub workout: WorkoutRef,
    pub user_id: UserRef,
    pub completed_on: String,
    pub sets: Vec<Set>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Set {
    pub weight: i32,
    pub reps: i32,
}
