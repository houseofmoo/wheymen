use super::shared_types::{UserRef, WorkoutHistoryRef, WorkoutRef, Set};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct Workout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<WorkoutRef>,
    pub user_id: UserRef,
    pub name: String,
    pub category: String,
    pub note: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutHistoryRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<WorkoutHistoryRef>,
    pub user_id: UserRef,
    pub workout_id: WorkoutRef,
    pub name: String,
    pub completed_on: String,
    pub sets: Vec<Set>,
}