# Changelog

## [1.1.0] - 2025-05-24

### Added
- HealthCheck trait implementation for registry monitoring
- `mcp-server.toml` manifest for ADK registry onboarding
- Structured tracing with `tracing-subscriber` (env-filter)

### Changed
- Edition upgraded to Rust 2024
- Added `adk-mcp-sdk` HealthCheck integration


## [1.0.0] - 2026-05-23

### Added

- **11 MCP tools** — session state, events, scoped memory, store/update/delete/redact, replay snapshots, resume, terminate
- **5 memory types** — turn_memory, session_state, profile_memory, project_memory, artifact_memory
- **Typed session state** — versioned workflow state with node tracking
- **Scoped memory retrieval** — by subject, type, query, and purpose
- **Replay snapshots** — freeze session context for deterministic replay
- **HITL resume/terminate** — with audit reasons and event logging
- **Memory versioning** — updates create new versions, deletions are logical
- **Redaction** — creates redacted version without mutating history
- **In-memory store** — for development and testing
- **rmcp 1.7** — latest MCP protocol SDK
