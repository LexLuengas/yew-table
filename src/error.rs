//! Data Errors

use std::error;
use std::fmt;

/// Data errors
#[derive(Debug)]
#[allow(dead_code)]
pub enum Error {
    /// The field does not have render implementation
    NonRenderableField(String),
    /// Invalid field name provided
    InvalidFieldName(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            Self::InvalidFieldName(field_name) => {
                format!("Invalid field name given: '{}'.", field_name)
            }
            Self::NonRenderableField(field_name) => format!(
                "Could not render field '{}' for which no HTML representation is defined.",
                field_name
            ),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            Self::InvalidFieldName(_) => "Invalid field name given.",
            Self::NonRenderableField(_) => "Field has no HTML representation defined.",
        }
    }
}

/// Result type symplification
pub type Result<T> = std::result::Result<T, Error>;
