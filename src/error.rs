#[derive(Debug)]
pub enum BackupError {
    Io(std::io::Error),
    InvalidScheduleIndex,
    ConfigSaveError(std::io::Error),
    Parse(String),
    Path(String),
    Execute(String),
    Other(String),
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::Io(e) => write!(f, "I/O error: {}", e),
            BackupError::InvalidScheduleIndex => write!(f, "Invalid schedule index"),
            BackupError::ConfigSaveError(e) => write!(f, "Failed to save config: {}", e),
            BackupError::Parse(msg) => write!(f, "Parse error: {}", msg),
            BackupError::Path(msg) => write!(f, "Path error: {}", msg),
            BackupError::Execute(msg) => write!(f, "Execution error: {}", msg),
            BackupError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl From<std::io::Error> for BackupError {
    fn from(e: std::io::Error) -> Self {
        BackupError::Io(e)
    }
}
