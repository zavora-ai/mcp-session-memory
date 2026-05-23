# Session Memory MCP Server

[![Crates.io](https://img.shields.io/crates/v/mcp-session-memory.svg)](https://crates.io/crates/mcp-session-memory)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![ADK-Rust Enterprise](https://img.shields.io/badge/ADK--Rust-Enterprise-purple.svg)](https://enterprise.adk-rust.com)

Policy-controlled access to live and historical agent session context for [ADK-Rust Enterprise](https://enterprise.adk-rust.com). Typed session state, scoped memory recall, replay snapshots, and resumable HITL workflows.

## Tools (11)

| Tool | Purpose | Risk Class |
|------|---------|------------|
| `get_session_state` | Read current typed session state | Read-only |
| `list_session_events` | Inspect operational events | Read-only |
| `retrieve_memory` | Search scoped memory by subject and purpose | Read-only |
| `store_memory` | Write approved memory entries | Internal write |
| `update_memory` | Correct existing memory (new version) | Internal write |
| `delete_memory` | Remove sensitive/incorrect entries | Identity/Security |
| `redact_memory` | Redact PII from memory | Internal write |
| `list_memory_refs` | List memory that influenced a session | Read-only |
| `create_replay_snapshot` | Freeze session state for replay | Internal write |
| `resume_session` | Resume paused HITL workflow | External write |
| `terminate_session` | Stop session with audit reason | External write |

## Example Prompts & Outputs

### Get session state

**Prompt:** "What's the current state of the refund session?"

```json
{
  "session_id": "ses_demo_001",
  "agent_id": "support_agent",
  "status": "paused",
  "current_node": "manager_approval_gate",
  "state_version": 3,
  "workflow_state": {
    "refund_amount": 89.99,
    "eligibility": "approved",
    "approval_required": true
  },
  "memory_refs": ["mem_demo_001"],
  "data_classes": ["financial"]
}
```

### Retrieve scoped memory

**Prompt:** "What do we know about customer 1187?"

**Tool call:** `retrieve_memory`
```json
{ "subject_type": "customer", "subject_id": "cust_1187" }
```

**Output:**
```json
[
  {
    "memory_id": "mem_demo_001",
    "memory_type": "profile_memory",
    "content": "Customer prefers email follow-up. Previous refund approved in March 2026.",
    "data_class": "pii",
    "confidence": 0.91,
    "version": 1,
    "status": "active"
  }
]
```

### Store a new memory

**Prompt:** "Remember that this customer's preferred language is Swahili"

```json
{
  "memory_type": "profile_memory",
  "subject_type": "customer",
  "subject_id": "cust_1187",
  "content": "Preferred language: Swahili",
  "data_class": "pii",
  "confidence": 0.95
}
```

**Output:**
```json
{ "memory_id": "mem_a1b2c3d4...", "status": "stored" }
```

### Resume a paused workflow

**Prompt:** "The manager approved the refund, resume the session"

```json
{
  "session_id": "ses_demo_001",
  "resume_payload": { "approval_decision": "approved", "approver": "manager_jane" },
  "reason": "Manager approved refund"
}
```

**Output:**
```json
{ "session_id": "ses_demo_001", "status": "active", "resumed": true }
```

### Create replay snapshot

**Prompt:** "Freeze this session state for replay testing"

```json
{ "session_id": "ses_demo_001" }
```

**Output:**
```json
{
  "snapshot_id": "snap_f7a2b1c4...",
  "session_id": "ses_demo_001",
  "state_version": 3
}
```

### Terminate with audit reason

**Prompt:** "Stop this session — suspected policy violation"

```json
{
  "session_id": "ses_demo_001",
  "reason": "Suspected policy violation during refund flow",
  "reason_code": "policy_violation"
}
```

**Output:**
```json
{ "session_id": "ses_demo_001", "status": "terminated", "reason": "Suspected policy violation..." }
```

## Memory Types

| Type | Description | Mutability |
|------|-------------|-----------|
| `turn_memory` | Recent conversational context | Mutable during session |
| `session_state` | Typed workflow state | Versioned |
| `profile_memory` | User/customer facts across sessions | Policy-governed |
| `project_memory` | Workspace decisions and preferences | Policy-governed |
| `artifact_memory` | References to artifacts (not content) | References only |

## Installation

```bash
git clone https://github.com/zavora-ai/mcp-session-memory
cd mcp-session-memory
cargo build --release
```

### MCP Client Config

```json
{
  "mcpServers": {
    "session-memory": {
      "command": "/path/to/mcp-session-memory"
    }
  }
}
```

## Contributors

<!-- ALL-CONTRIBUTORS-LIST:START -->
| [<img src="https://github.com/jkmaina.png" width="80px;" alt=""/><br /><sub><b>James Karanja Maina</b></sub>](https://github.com/jkmaina) |
|:---:|
<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Apache-2.0 — see [LICENSE](LICENSE) for details.

---

Part of the [ADK-Rust Enterprise](https://enterprise.adk-rust.com) MCP server ecosystem.

Built with ❤️ by [Zavora AI](https://zavora.ai)
