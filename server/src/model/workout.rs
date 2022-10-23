use super::shared_types::{UserRef, WorkoutHistoryRef, WorkoutRef, Set};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Workout {
    pub id: WorkoutRef,
    pub user_id: UserRef,
    pub name: String,
    pub category: String,
    pub note: String,
}
pub type WorkoutRow = Workout;

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
    pub selected_routine_ids: Option<Vec<String>>,
    pub unselected_routine_ids: Option<Vec<String>>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutHistoryRow {
    pub id: WorkoutHistoryRef,
    pub user_id: UserRef,
    pub workout_id: WorkoutRef,
    pub name: String,
    pub completed_on: String,
    pub sets: Vec<Set>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertWorkoutHistoryRow {
    pub user_id: UserRef,
    pub workout_id: WorkoutRef,
    pub name: String,
    pub completed_on: String,
    pub sets: Vec<Set>,
}