use std::fmt;
use std::error;

#[derive(Debug)]
pub enum TableError {
    NonRenderableField(String),
    InvalidFieldName(String),
}

impl fmt::Display for TableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            TableError::InvalidFieldName(field_name) => format!("Invalid field name given: '{}'.", field_name),
            TableError::NonRenderableField(field_name) => format!("Could not render field '{}' for which no HTML representation is defined.", field_name),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for TableError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            TableError::InvalidFieldName(_) => "Invalid field name given.",
            TableError::NonRenderableField(_) => "Field has no HTML representation defined.",
        }
    }
}

pub type Result<T> = std::result::Result<T, TableError>;