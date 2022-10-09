use std::{error::Error, fmt};

pub enum LocalError {
    SerializationError,
    RequestError,
    InsertFailed,
    UpdateFailed,
    RelationshipFailed,
    GetFailed,
}

impl Error for LocalError {}

impl std::fmt::Debug for LocalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerializationError => write!(f, "SerializationError"),
            Self::RequestError => write!(f, "RequestError"),
            Self::InsertFailed => write!(f, "InsertFailed"),
            Self::UpdateFailed => write!(f, "UpdateFailed"),
            Self::RelationshipFailed => write!(f, "RelationshipFailed"),
            Self::GetFailed => write!(f, "GetFailed"),
        }
    }
}

impl fmt::Display for LocalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerializationError => write!(f, "SerializationError"),
            Self::RequestError => write!(f, "RequestError"),
            Self::InsertFailed => write!(f, "InsertFailed"),
            Self::UpdateFailed => write!(f, "UpdateFailed"),
            Self::RelationshipFailed => write!(f, "RelationshipFailed"),
            Self::GetFailed => write!(f, "GetFailed"),
        }
    }
}

impl From<reqwest::Error> for LocalError {
    fn from(_: reqwest::Error) -> Self {
        return LocalError::RequestError;
    }
}

impl From<serde_json::error::Error> for LocalError {
    fn from(_: serde_json::error::Error) -> Self {
        return LocalError::SerializationError;
    }
}
