use crate::error::MemoryError;
use crate::types::*;

#[async_trait::async_trait]
pub trait SessionStore: Send + Sync {
    async fn get_session(&self, session_id: &str) -> Result<Session, MemoryError>;
    async fn put_session(&self, session: Session) -> Result<(), MemoryError>;
    async fn update_session(&self, session: Session) -> Result<(), MemoryError>;

    async fn list_events(&self, session_id: &str, event_types: Option<&[EventType]>, limit: usize) -> Result<Vec<SessionEvent>, MemoryError>;
    async fn put_event(&self, event: SessionEvent) -> Result<(), MemoryError>;

    async fn put_snapshot(&self, snapshot: ReplaySnapshot) -> Result<(), MemoryError>;
    async fn get_snapshot(&self, snapshot_id: &str) -> Result<ReplaySnapshot, MemoryError>;
}

#[async_trait::async_trait]
pub trait MemoryStore: Send + Sync {
    async fn retrieve(&self, subject_type: &str, subject_id: &str, memory_types: Option<&[MemoryType]>, query: Option<&str>, limit: usize) -> Result<Vec<MemoryEntry>, MemoryError>;
    async fn get_memory(&self, memory_id: &str) -> Result<MemoryEntry, MemoryError>;
    async fn put_memory(&self, entry: MemoryEntry) -> Result<(), MemoryError>;
    async fn update_memory(&self, entry: MemoryEntry) -> Result<(), MemoryError>;
    async fn delete_memory(&self, memory_id: &str) -> Result<(), MemoryError>;
    async fn list_by_session(&self, session_id: &str) -> Result<Vec<MemoryEntry>, MemoryError>;
}
