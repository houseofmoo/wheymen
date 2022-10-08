use super::shared_types::{RelationshipRef, RoutineRef, UserRef, WorkoutRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DbResponse<T> {
    pub time: String,
    pub status: String,
    pub result: Vec<T>,
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

pub enum Table {    // TODO: standardize table names so we dont screw up typing it
    Routines,
    RoutineHistory,
    Sessions,
    Workouts,
    WorkoutHistory,
}

impl Table {
    pub fn name(&self) -> &'static str {
        match self {
            Table::Routines => "routines",
            Table::RoutineHistory => "routine_history",
            Table::Sessions => "sessions",
            Table::Workouts => "workouts",
            Table::WorkoutHistory => "workout_history",
        }
    }
}