mod dynamic_server_discovery;
pub mod backends_manager;

use dashmap::DashMap;
use pingora_core::server::ShutdownWatch;
use pingora_core::services::background::{BackgroundService, GenBackgroundService};
use pingora_load_balancing::Backends;