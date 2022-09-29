use super::shared_types::{UserRef, WorkoutRef};
use serde::{Deserialize, Serialize};

pub type WorkoutRow = Workout; // a workout and wokrout row are the same type but have different meanings

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
