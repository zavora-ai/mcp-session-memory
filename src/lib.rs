//! # Session Memory MCP Server
//!
//! Policy-controlled access to live and historical agent session context:
//! typed session state, scoped memory recall, replay snapshots, and resumable workflows.

pub mod error;
pub mod store;
pub mod types;
pub mod memory_store;
pub mod server;

pub use error::MemoryError;
pub use store::{MemoryStore, SessionStore};
pub use types::*;
