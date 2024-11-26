use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum DaycareError {
    Internal,
    UnrecognizedToken,
}

impl Display for DaycareError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaycareError::Internal => write!(f, "Internal daycare error"),
            DaycareError::UnrecognizedToken => write!(f, "Unrecognized token"),
        }
    }
}

impl Error for DaycareError {}
