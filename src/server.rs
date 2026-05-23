use std::sync::Arc;
use chrono::Utc;
use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::{MemoryStore, SessionStore};
use crate::types::*;

#[derive(Clone)]
pub struct SessionMemoryServer {
    sessions: Arc<dyn SessionStore>,
    memory: Arc<dyn MemoryStore>,
}

impl SessionMemoryServer {
    pub fn new(sessions: Arc<dyn SessionStore>, memory: Arc<dyn MemoryStore>) -> Self {
        Self { sessions, memory }
    }
}

// --- Input types ---

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct GetSessionStateInput { pub session_id: String }

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListSessionEventsInput {
    pub session_id: String,
    #[serde(default)] pub event_types: Option<Vec<EventType>>,
    #[serde(default)] pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct RetrieveMemoryInput {
    pub subject_type: String,
    pub subject_id: String,
    #[serde(default)] pub query: Option<String>,
    #[serde(default)] pub memory_types: Option<Vec<MemoryType>>,
    #[serde(default)] pub limit: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct StoreMemoryInput {
    pub memory_type: MemoryType,
    pub subject_type: String,
    pub subject_id: String,
    pub content: String,
    #[serde(default)] pub data_class: Option<DataClass>,
    #[serde(default)] pub source_session_id: Option<String>,
    #[serde(default)] pub confidence: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct UpdateMemoryInput {
    pub memory_id: String,
    pub content: String,
    #[serde(default)] pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct DeleteMemoryInput {
    pub memory_id: String,
    #[serde(default)] pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct RedactMemoryInput {
    pub memory_id: String,
    pub redacted_content: String,
    #[serde(default)] pub fields_removed: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ListMemoryRefsInput { pub session_id: String }

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct CreateReplaySnapshotInput { pub session_id: String }

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct ResumeSessionInput {
    pub session_id: String,
    #[serde(default)] pub resume_payload: Option<serde_json::Value>,
    #[serde(default)] pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct TerminateSessionInput {
    pub session_id: String,
    pub reason: String,
    #[serde(default)] pub reason_code: Option<String>,
}

// --- Tool implementations ---

#[tool_router(server_handler)]
impl SessionMemoryServer {
    #[tool(description = "Read current typed session state")]
    async fn get_session_state(&self, Parameters(i): Parameters<GetSessionStateInput>) -> String {
        match self.sessions.get_session(&i.session_id).await {
            Ok(s) => serde_json::to_string_pretty(&s).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Inspect trace events, turns, tools, retries, approvals")]
    async fn list_session_events(&self, Parameters(i): Parameters<ListSessionEventsInput>) -> String {
        let types = i.event_types.as_deref();
        match self.sessions.list_events(&i.session_id, types, i.limit.unwrap_or(20)).await {
            Ok(events) => serde_json::to_string_pretty(&events).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Search scoped memory for relevant facts")]
    async fn retrieve_memory(&self, Parameters(i): Parameters<RetrieveMemoryInput>) -> String {
        let types = i.memory_types.as_deref();
        match self.memory.retrieve(&i.subject_type, &i.subject_id, types, i.query.as_deref(), i.limit.unwrap_or(8)).await {
            Ok(entries) => serde_json::to_string_pretty(&entries).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Write approved memory entries")]
    async fn store_memory(&self, Parameters(i): Parameters<StoreMemoryInput>) -> String {
        let now = Utc::now();
        let memory_id = format!("mem_{}", Uuid::new_v4().simple());
        let entry = MemoryEntry {
            memory_id: memory_id.clone(),
            memory_type: i.memory_type,
            subject_type: i.subject_type,
            subject_id: i.subject_id,
            content: i.content,
            data_class: i.data_class.unwrap_or(DataClass::Internal),
            source_session_id: i.source_session_id,
            source_artifact_ids: vec![],
            confidence: i.confidence,
            version: 1,
            status: MemoryStatus::Active,
            created_at: now, updated_at: now,
        };
        match self.memory.put_memory(entry).await {
            Ok(()) => serde_json::to_string_pretty(&serde_json::json!({
                "memory_id": memory_id, "status": "stored"
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Correct an existing memory entry (creates new version)")]
    async fn update_memory(&self, Parameters(i): Parameters<UpdateMemoryInput>) -> String {
        match self.memory.get_memory(&i.memory_id).await {
            Ok(mut entry) => {
                entry.content = i.content;
                entry.version += 1;
                entry.updated_at = Utc::now();
                match self.memory.update_memory(entry).await {
                    Ok(()) => serde_json::to_string_pretty(&serde_json::json!({
                        "memory_id": i.memory_id, "status": "updated"
                    })).unwrap(),
                    Err(e) => format!("Error: {}", e),
                }
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Remove sensitive or incorrect memory entries")]
    async fn delete_memory(&self, Parameters(i): Parameters<DeleteMemoryInput>) -> String {
        match self.memory.delete_memory(&i.memory_id).await {
            Ok(()) => serde_json::to_string_pretty(&serde_json::json!({
                "memory_id": i.memory_id, "status": "deleted",
                "reason": i.reason.unwrap_or_else(|| "not specified".into())
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Redact PII or sensitive details from memory")]
    async fn redact_memory(&self, Parameters(i): Parameters<RedactMemoryInput>) -> String {
        match self.memory.get_memory(&i.memory_id).await {
            Ok(mut entry) => {
                entry.content = i.redacted_content;
                entry.status = MemoryStatus::Redacted;
                entry.version += 1;
                entry.updated_at = Utc::now();
                match self.memory.update_memory(entry).await {
                    Ok(()) => serde_json::to_string_pretty(&serde_json::json!({
                        "memory_id": i.memory_id, "status": "redacted",
                        "fields_removed": i.fields_removed.unwrap_or_default()
                    })).unwrap(),
                    Err(e) => format!("Error: {}", e),
                }
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "List memory references that influenced a session")]
    async fn list_memory_refs(&self, Parameters(i): Parameters<ListMemoryRefsInput>) -> String {
        match self.memory.list_by_session(&i.session_id).await {
            Ok(entries) => {
                let refs: Vec<_> = entries.iter().map(|e| serde_json::json!({
                    "memory_id": e.memory_id,
                    "memory_type": e.memory_type,
                    "subject": format!("{}:{}", e.subject_type, e.subject_id),
                    "data_class": e.data_class,
                })).collect();
                serde_json::to_string_pretty(&refs).unwrap()
            }
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Freeze session state for replay")]
    async fn create_replay_snapshot(&self, Parameters(i): Parameters<CreateReplaySnapshotInput>) -> String {
        let session = match self.sessions.get_session(&i.session_id).await {
            Ok(s) => s,
            Err(e) => return format!("Error: {}", e),
        };
        let snapshot_id = format!("snap_{}", Uuid::new_v4().simple());
        let snapshot = ReplaySnapshot {
            snapshot_id: snapshot_id.clone(),
            session_id: i.session_id.clone(),
            state_version: session.state_version,
            workflow_state: session.workflow_state.clone(),
            memory_refs: session.memory_refs.clone(),
            artifact_refs: session.artifact_refs.clone(),
            agent_id: session.agent_id.clone(),
            agent_version: session.agent_version.clone(),
            created_at: Utc::now(),
        };
        match self.sessions.put_snapshot(snapshot).await {
            Ok(()) => serde_json::to_string_pretty(&serde_json::json!({
                "snapshot_id": snapshot_id, "session_id": i.session_id,
                "state_version": session.state_version
            })).unwrap(),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[tool(description = "Resume paused HITL workflow")]
    async fn resume_session(&self, Parameters(i): Parameters<ResumeSessionInput>) -> String {
        let mut session = match self.sessions.get_session(&i.session_id).await {
            Ok(s) => s,
            Err(e) => return format!("Error: {}", e),
        };
        if session.status != SessionStatus::Paused {
            return format!("Session {} is not paused (status: {:?})", i.session_id, session.status);
        }
        session.status = SessionStatus::Active;
        session.state_version += 1;
        session.updated_at = Utc::now();
        if let Some(payload) = i.resume_payload {
            if let Some(obj) = session.workflow_state.as_object_mut() {
                obj.insert("resume_payload".into(), payload);
            }
        }
        let _ = self.sessions.update_session(session).await;
        let _ = self.sessions.put_event(SessionEvent {
            event_id: format!("evt_{}", Uuid::new_v4().simple()),
            session_id: i.session_id.clone(), sequence: 0,
            event_type: EventType::SessionResumed, actor: "operator".into(),
            summary: i.reason.unwrap_or_else(|| "Session resumed".into()),
            data_class: None, created_at: Utc::now(),
        }).await;
        serde_json::to_string_pretty(&serde_json::json!({
            "session_id": i.session_id, "status": "active", "resumed": true
        })).unwrap()
    }

    #[tool(description = "Stop active session with audit reason")]
    async fn terminate_session(&self, Parameters(i): Parameters<TerminateSessionInput>) -> String {
        let mut session = match self.sessions.get_session(&i.session_id).await {
            Ok(s) => s,
            Err(e) => return format!("Error: {}", e),
        };
        session.status = SessionStatus::Terminated;
        session.updated_at = Utc::now();
        let _ = self.sessions.update_session(session).await;
        let _ = self.sessions.put_event(SessionEvent {
            event_id: format!("evt_{}", Uuid::new_v4().simple()),
            session_id: i.session_id.clone(), sequence: 0,
            event_type: EventType::SessionTerminated, actor: "operator".into(),
            summary: format!("{} ({})", i.reason, i.reason_code.unwrap_or_else(|| "manual".into())),
            data_class: None, created_at: Utc::now(),
        }).await;
        serde_json::to_string_pretty(&serde_json::json!({
            "session_id": i.session_id, "status": "terminated", "reason": i.reason
        })).unwrap()
    }
}
