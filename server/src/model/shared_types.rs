pub type WorkoutRef = String;
pub type RelationshipRef = String;
pub type UserRef = String;
pub type RoutineRef = String;

#[allow(dead_code)]
pub type WorkoutHistoryRef = String;
#[allow(dead_code)]
pub type RoutineHistoryRef = String;

pub type DbResult<T> = std::result::Result<Option<T>, super::error::LocalError>;
