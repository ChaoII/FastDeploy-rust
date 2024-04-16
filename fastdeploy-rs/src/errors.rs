#[derive(Debug, Copy, Clone)]
pub enum FastDeployError {
    /// Failed to create a new context.
    InitError,
    PredictError,
}


impl std::fmt::Display for FastDeployError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use FastDeployError::*;
        match self {
            InitError => write!(f, "Failed to create a new whisper context."),
            PredictError => write!(f, "model predict error")
        }
    }
}

impl std::error::Error for FastDeployError {}