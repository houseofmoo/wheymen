use serde::{Deserialize, Serialize};

use super::{shared_types::{RelationshipRef, RoutineRef, UserRef, WorkoutRef}, data::Workout};

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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertRoutineRow {
    pub user_id: UserRef,
    pub name: String,
    pub days: Vec<String>,
    pub last_completed: String,
    pub note: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertRoutine {
    pub routine: InsertRoutineRow,
    pub workout_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateRoutine {
    pub routine: RoutineRow,
    pub workout_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertWorkout {
    pub workout: InsertWorkoutRow,
    pub routine_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateWorkout {
    pub workout: Workout,
    pub routine_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InsertWorkoutRow {
    pub user_id: UserRef,
    pub name: String,
    pub category: String,
    pub note: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Relationship {
    pub id: RelationshipRef,
    #[serde(alias = "in")]
    pub routine_in: RoutineRef,
    #[serde(alias = "out")]
    pub workout_out: WorkoutRef,
}
