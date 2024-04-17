use std::str::Utf8Error;

#[derive(Debug, Copy, Clone)]
pub enum FastDeployError {
    /// Failed to create a new context.
    InitError,
    PredictError,
    InvalidUtf8 {
        error_len: Option<usize>,
        valid_up_to: usize,
    },
}

impl From<Utf8Error> for FastDeployError {
    fn from(e: Utf8Error) -> Self {
        Self::InvalidUtf8 {
            error_len: e.error_len(),
            valid_up_to: e.valid_up_to(),
        }
    }
}


impl std::fmt::Display for FastDeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use FastDeployError::*;
        match self {
            InitError => write!(f, "Failed to create a new whisper context."),
            PredictError => write!(f, "model predict error"),
            InvalidUtf8 {
                valid_up_to,
                error_len: Some(len),
            } => write!(
                f,
                "Invalid UTF-8 detected in a string from Whisper. Index: {}, Length: {}.",
                valid_up_to, len
            ),
            InvalidUtf8 {
                valid_up_to,
                error_len: None,
            } => write!(
                f,
                "Invalid UTF-8 detected in a string from Whisper. Index: {}.",
                valid_up_to
            ),
        }
    }
}

impl std::error::Error for FastDeployError {}