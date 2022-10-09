pub type WorkoutRef = String;
pub type RelationshipRef = String;
pub type UserRef = String;
pub type RoutineRef = String;
pub type WorkoutHistoryRef = String;
pub type RoutineHistoryRef = String;
pub type SessionRef = String;

pub type DbResult<T> = std::result::Result<Option<T>, super::error::LocalError>;
