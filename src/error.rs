#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("access denied: {0}")]
    AccessDenied(String),
    #[error("session not resumable: {0}")]
    NotResumable(String),
    #[error("deletion blocked: {0}")]
    DeletionBlocked(String),
    #[error("internal error: {0}")]
    Internal(String),
}
