use std::sync::Arc;
use chrono::Utc;
use mcp_session_memory::{memory_store::*, server::SessionMemoryServer, store::*, types::*};
use rmcp::{ServiceExt, transport::stdio};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider().install_default().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting Session Memory MCP server");

    let sessions = Arc::new(InMemorySessionStore::new());
    let memory = Arc::new(InMemoryMemoryStore::new());

    // Seed a demo session
    let demo_session = Session {
        session_id: "ses_demo_001".into(),
        agent_id: "support_agent".into(),
        agent_version: Some("v1.2.0".into()),
        workflow_id: Some("refund_flow".into()),
        status: SessionStatus::Paused,
        current_node: Some("manager_approval_gate".into()),
        state_version: 3,
        workflow_state: serde_json::json!({
            "refund_amount": 89.99,
            "eligibility": "approved",
            "approval_required": true
        }),
        memory_refs: vec!["mem_demo_001".into()],
        artifact_refs: vec![],
        data_classes: vec![DataClass::Financial],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    sessions.put_session(demo_session).await?;

    // Seed a demo memory entry
    let demo_memory = mcp_session_memory::MemoryEntry {
        memory_id: "mem_demo_001".into(),
        memory_type: MemoryType::ProfileMemory,
        subject_type: "customer".into(),
        subject_id: "cust_1187".into(),
        content: "Customer prefers email follow-up. Previous refund approved in March 2026.".into(),
        data_class: DataClass::Pii,
        source_session_id: Some("ses_demo_001".into()),
        source_artifact_ids: vec![],
        confidence: Some(0.91),
        version: 1,
        status: MemoryStatus::Active,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    memory.put_memory(demo_memory).await?;

    tracing::info!("Loaded demo session and memory");

    let server = SessionMemoryServer::new(sessions, memory);
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
