use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Active,
    Paused,
    Completed,
    Terminated,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MemoryType {
    TurnMemory,
    SessionState,
    ProfileMemory,
    ProjectMemory,
    ArtifactMemory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DataClass {
    Public,
    Internal,
    Pii,
    Financial,
    Health,
    LegalSensitive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    SessionStarted,
    TurnAdded,
    ToolCalled,
    ToolResultReceived,
    MemoryRetrieved,
    MemoryStored,
    ArtifactCreated,
    PolicyGateTriggered,
    ApprovalRequested,
    SessionPaused,
    SessionResumed,
    SessionTerminated,
    SnapshotCreated,
}

/// Core session record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub agent_id: String,
    pub agent_version: Option<String>,
    pub workflow_id: Option<String>,
    pub status: SessionStatus,
    pub current_node: Option<String>,
    pub state_version: u32,
    pub workflow_state: serde_json::Value,
    pub memory_refs: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub data_classes: Vec<DataClass>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A memory entry — scoped, typed, versioned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub memory_id: String,
    pub memory_type: MemoryType,
    pub subject_type: String,
    pub subject_id: String,
    pub content: String,
    pub data_class: DataClass,
    pub source_session_id: Option<String>,
    pub source_artifact_ids: Vec<String>,
    pub confidence: Option<f64>,
    pub version: u32,
    pub status: MemoryStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MemoryStatus {
    Active,
    Redacted,
    Deleted,
    Stale,
}

/// Session event — operational, not raw trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEvent {
    pub event_id: String,
    pub session_id: String,
    pub sequence: u64,
    pub event_type: EventType,
    pub actor: String,
    pub summary: String,
    pub data_class: Option<DataClass>,
    pub created_at: DateTime<Utc>,
}

/// Replay snapshot — frozen session context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplaySnapshot {
    pub snapshot_id: String,
    pub session_id: String,
    pub state_version: u32,
    pub workflow_state: serde_json::Value,
    pub memory_refs: Vec<String>,
    pub artifact_refs: Vec<String>,
    pub agent_id: String,
    pub agent_version: Option<String>,
    pub created_at: DateTime<Utc>,
}
