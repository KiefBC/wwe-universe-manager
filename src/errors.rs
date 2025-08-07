use serde::{Deserialize, Serialize};
use std::fmt;

/// Custom error types for the WWE Universe Manager application
/// 
/// **Note**: Many error constructors are future infrastructure for comprehensive error handling.

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    /// Network/Communication errors
    TauriInvocationError {
        command: String,
        details: String,
    },
    
    /// Serialization errors
    SerializationError {
        operation: String,
        details: String,
    },
    
    /// Data validation errors
    ValidationError {
        field: String,
        message: String,
    },
    
    /// Wrestler-related errors
    WrestlerError {
        wrestler_id: Option<i32>,
        operation: String,
        details: String,
    },
    
    /// Title-related errors
    TitleError {
        title_id: Option<i32>,
        operation: String,
        details: String,
    },
    
    /// Generic application errors
    Generic {
        operation: String,
        details: String,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::TauriInvocationError { command, details } => {
                write!(f, "Failed to execute command '{}': {}", command, details)
            }
            AppError::SerializationError { operation, details } => {
                write!(f, "Serialization error during {}: {}", operation, details)
            }
            AppError::ValidationError { field, message } => {
                write!(f, "Validation error for {}: {}", field, message)
            }
            AppError::WrestlerError { wrestler_id, operation, details } => {
                match wrestler_id {
                    Some(id) => write!(f, "Wrestler error (ID: {}): {} - {}", id, operation, details),
                    None => write!(f, "Wrestler error: {} - {}", operation, details),
                }
            }
            AppError::TitleError { title_id, operation, details } => {
                match title_id {
                    Some(id) => write!(f, "Title error (ID: {}): {} - {}", id, operation, details),
                    None => write!(f, "Title error: {} - {}", operation, details),
                }
            }
            AppError::Generic { operation, details } => {
                write!(f, "Error during {}: {}", operation, details)
            }
        }
    }
}

impl std::error::Error for AppError {}

impl From<String> for AppError {
    fn from(s: String) -> Self {
        AppError::Generic {
            operation: "unknown".to_string(),
            details: s,
        }
    }
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::Generic {
            operation: "unknown".to_string(),
            details: s.to_string(),
        }
    }
}

/// Helper function to create a Tauri invocation error
#[allow(dead_code)]
pub fn tauri_error(command: &str, details: String) -> AppError {
    AppError::TauriInvocationError {
        command: command.to_string(),
        details,
    }
}

/// Helper function to create a serialization error
#[allow(dead_code)]
pub fn serialization_error(operation: &str, details: String) -> AppError {
    AppError::SerializationError {
        operation: operation.to_string(),
        details,
    }
}

/// Helper function to create a validation error
#[allow(dead_code)]
pub fn validation_error(field: &str, message: &str) -> AppError {
    AppError::ValidationError {
        field: field.to_string(),
        message: message.to_string(),
    }
}

/// Helper function to create a wrestler error
#[allow(dead_code)]
pub fn wrestler_error(wrestler_id: Option<i32>, operation: &str, details: String) -> AppError {
    AppError::WrestlerError {
        wrestler_id,
        operation: operation.to_string(),
        details,
    }
}

/// Helper function to create a title error
#[allow(dead_code)]
pub fn title_error(title_id: Option<i32>, operation: &str, details: String) -> AppError {
    AppError::TitleError {
        title_id,
        operation: operation.to_string(),
        details,
    }
}