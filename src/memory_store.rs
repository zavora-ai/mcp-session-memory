use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::MemoryError;
use crate::store::{MemoryStore, SessionStore};
use crate::types::*;

pub struct InMemorySessionStore {
    sessions: Arc<RwLock<HashMap<String, Session>>>,
    events: Arc<RwLock<Vec<SessionEvent>>>,
    snapshots: Arc<RwLock<HashMap<String, ReplaySnapshot>>>,
}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            snapshots: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait::async_trait]
impl SessionStore for InMemorySessionStore {
    async fn get_session(&self, session_id: &str) -> Result<Session, MemoryError> {
        self.sessions.read().await.get(session_id).cloned()
            .ok_or_else(|| MemoryError::NotFound(session_id.into()))
    }

    async fn put_session(&self, session: Session) -> Result<(), MemoryError> {
        self.sessions.write().await.insert(session.session_id.clone(), session);
        Ok(())
    }

    async fn update_session(&self, session: Session) -> Result<(), MemoryError> {
        let mut sessions = self.sessions.write().await;
        if !sessions.contains_key(&session.session_id) {
            return Err(MemoryError::NotFound(session.session_id));
        }
        sessions.insert(session.session_id.clone(), session);
        Ok(())
    }

    async fn list_events(&self, session_id: &str, event_types: Option<&[EventType]>, limit: usize) -> Result<Vec<SessionEvent>, MemoryError> {
        let events = self.events.read().await;
        let filtered: Vec<_> = events.iter()
            .filter(|e| e.session_id == session_id)
            .filter(|e| event_types.is_none_or(|types| types.contains(&e.event_type)))
            .rev().take(limit).cloned().collect();
        Ok(filtered)
    }

    async fn put_event(&self, event: SessionEvent) -> Result<(), MemoryError> {
        self.events.write().await.push(event);
        Ok(())
    }

    async fn put_snapshot(&self, snapshot: ReplaySnapshot) -> Result<(), MemoryError> {
        self.snapshots.write().await.insert(snapshot.snapshot_id.clone(), snapshot);
        Ok(())
    }

    async fn get_snapshot(&self, snapshot_id: &str) -> Result<ReplaySnapshot, MemoryError> {
        self.snapshots.read().await.get(snapshot_id).cloned()
            .ok_or_else(|| MemoryError::NotFound(snapshot_id.into()))
    }
}

pub struct InMemoryMemoryStore {
    entries: Arc<RwLock<HashMap<String, MemoryEntry>>>,
}

impl InMemoryMemoryStore {
    pub fn new() -> Self {
        Self { entries: Arc::new(RwLock::new(HashMap::new())) }
    }
}

#[async_trait::async_trait]
impl MemoryStore for InMemoryMemoryStore {
    async fn retrieve(&self, subject_type: &str, subject_id: &str, memory_types: Option<&[MemoryType]>, query: Option<&str>, limit: usize) -> Result<Vec<MemoryEntry>, MemoryError> {
        let entries = self.entries.read().await;
        let results: Vec<_> = entries.values()
            .filter(|e| e.status == MemoryStatus::Active)
            .filter(|e| e.subject_type == subject_type && e.subject_id == subject_id)
            .filter(|e| memory_types.is_none_or(|types| types.contains(&e.memory_type)))
            .filter(|e| query.is_none_or(|q| e.content.to_lowercase().contains(&q.to_lowercase())))
            .take(limit).cloned().collect();
        Ok(results)
    }

    async fn get_memory(&self, memory_id: &str) -> Result<MemoryEntry, MemoryError> {
        self.entries.read().await.get(memory_id).cloned()
            .ok_or_else(|| MemoryError::NotFound(memory_id.into()))
    }

    async fn put_memory(&self, entry: MemoryEntry) -> Result<(), MemoryError> {
        self.entries.write().await.insert(entry.memory_id.clone(), entry);
        Ok(())
    }

    async fn update_memory(&self, entry: MemoryEntry) -> Result<(), MemoryError> {
        let mut entries = self.entries.write().await;
        if !entries.contains_key(&entry.memory_id) {
            return Err(MemoryError::NotFound(entry.memory_id));
        }
        entries.insert(entry.memory_id.clone(), entry);
        Ok(())
    }

    async fn delete_memory(&self, memory_id: &str) -> Result<(), MemoryError> {
        let mut entries = self.entries.write().await;
        if let Some(entry) = entries.get_mut(memory_id) {
            entry.status = MemoryStatus::Deleted;
            entry.updated_at = chrono::Utc::now();
            Ok(())
        } else {
            Err(MemoryError::NotFound(memory_id.into()))
        }
    }

    async fn list_by_session(&self, session_id: &str) -> Result<Vec<MemoryEntry>, MemoryError> {
        let entries = self.entries.read().await;
        Ok(entries.values()
            .filter(|e| e.source_session_id.as_deref() == Some(session_id) && e.status == MemoryStatus::Active)
            .cloned().collect())
    }
}
