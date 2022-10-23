use std::{error::Error, fmt};

pub enum LocalError {
    SerializationError,
    RequestError,
    
    NoIDProvided,
    InsertFailed,
    UpdateFailed,
    GetFailed,

    RelationshipFailed,
}

impl Error for LocalError {}

impl std::fmt::Debug for LocalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerializationError => write!(f, "SerializationError: Error while deserializing a resposne from database"),
            Self::RequestError => write!(f, "RequestError: Error while makign a request to the database"),
            
            Self::NoIDProvided => write!(f, "NoIDProvided: Expected a unique ID but got None"),
            Self::InsertFailed => write!(f, "InsertFailed: Did not receive expected database response"),
            Self::UpdateFailed => write!(f, "UpdateFailed: Did not receive expected database response"),
            Self::GetFailed => write!(f, "GetFailed"),
            
            Self::RelationshipFailed => write!(f, "RelationshipFailed"),
        }
    }
}

impl fmt::Display for LocalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SerializationError => write!(f, "SerializationError: Error while deserializing a resposne from database"),
            Self::RequestError => write!(f, "RequestError: Error while makign a request to the database"),
            
            Self::NoIDProvided => write!(f, "NoIDProvided: Expected a unique ID but got None"),
            Self::InsertFailed => write!(f, "InsertFailed: Did not receive expected database response"),
            Self::UpdateFailed => write!(f, "UpdateFailed: Did not receive expected database response"),
            Self::GetFailed => write!(f, "GetFailed"),
            
            Self::RelationshipFailed => write!(f, "RelationshipFailed"),
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
