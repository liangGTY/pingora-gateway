use std::cell::Ref;
use async_trait::async_trait;
use dashmap::DashMap;
use pingora_core::server::ShutdownWatch;
use pingora_core::services::background::BackgroundService;
use pingora_load_balancing::Backends;
use pingora_load_balancing::health_check::TcpHealthCheck;
use crate::backends::dynamic_server_discovery::DynamicServerDiscovery;

struct BackendName {}

pub struct BackendsManager {
    backend_mappings: DashMap<String, Backends>,
}

impl BackendsManager {
    pub fn new() -> Self {
        Self {
            backend_mappings: DashMap::new()
        }
    }

    pub fn get_or_create_backends(&self, name: &String) -> Ref<Backends> {
        let ref_mut = self.backend_mappings.entry("s".into())
            .or_insert_with(|| {
                let discoverys = DynamicServerDiscovery {};


                let x = Box::new(discoverys);

                let mut backends = Backends::new(x);

                backends.set_health_check(TcpHealthCheck::new());

                backends
            })
            .set_health_check()
    }
}

#[async_trait]
impl BackgroundService for BackendsManager {
    async fn start(&self, shutdown: ShutdownWatch) {
        todo!()
    }
}