use serde::{Deserialize, Serialize};

use super::shared_types::{RelationshipRef, RoutineRef, UserRef, WorkoutRef};

#[derive(Debug, Deserialize, Serialize)]
pub struct DbResponse<T> {
    pub time: String,
    pub status: String,
    pub result: Vec<T>,
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
pub struct WorkoutRow {
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

// What a graph edge (relationship) looks like
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Relationship {
    pub id: RelationshipRef,
    #[serde(alias = "in")]
    pub routine_in: RoutineRef,
    #[serde(alias = "out")]
    pub workout_out: WorkoutRef,
    pub user_id: UserRef,
}
