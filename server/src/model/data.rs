use super::{shared_types::{RoutineHistoryRef, RoutineRef, UserRef, WorkoutHistoryRef, WorkoutRef}};
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
pub struct Workout {
    pub id: WorkoutRef,
    pub user_id: UserRef,
    pub name: String,
    pub category: String,
    pub note: String,
}

// TODO: still have no decided how we want to deal with history
// believe we want RoutineHistory to be a steak in the ground
// of what workouts done regardless if the routine has changed since
// the history was recorded
// - this means that RoutineHistory and WorkoutHistory are immutable
// - which might be weird if the user changes the name of a workout and we
// show the old name
#[derive(Debug, Deserialize, Serialize)]
pub struct RoutineHistory {
    pub id: RoutineHistoryRef,
    pub user_id: UserRef,              // user history is tied to
    pub routine_id: RoutineRef,        // routine history is tied to
    pub workouts: Vec<WorkoutHistory>, // workout history objs containing workouts performed during this routine
    pub date: String,                  // date routine was completed
    pub duration_in_sec: i32,          // length of time routine took to complete
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkoutHistory {
    pub id: WorkoutHistoryRef,
    pub user_id: UserRef,       // user history is tied to
    pub workout_id: WorkoutRef, // workout this history is tied to
    pub routine_id: RoutineRef, // routine that selected when this workout was performed
    pub name: String,           // name of workout at time of completion
    pub category: String,       // category of this workout
    pub date: String,           // date workout was completed
    pub lifts: Option<Vec<Lift>>,
    pub cardio: Option<Vec<Cardio>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Lift {
    pub weight: i32,
    pub reps: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cardio {
    pub duration: i32,
    pub distance: i32,
}
