use super::{
    shared_types::{RoutineRef, UserRef, WorkoutRef},
    workout::Workout,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
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
