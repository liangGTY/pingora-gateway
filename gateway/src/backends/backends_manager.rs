use std::cell::Ref;
use std::sync::Arc;
use async_trait::async_trait;
use dashmap::DashMap;
use pingora_core::server::ShutdownWatch;
use pingora_core::services::background::BackgroundService;
use pingora_load_balancing::{Backends, LoadBalancer};
use pingora_load_balancing::health_check::TcpHealthCheck;
use pingora_load_balancing::prelude::RoundRobin;
use pingora_load_balancing::selection::{BackendIter, BackendSelection};
use crate::backends::dynamic_server_discovery::DynamicServerDiscovery;

struct BackendName {}

pub struct BackendsManager {
    backend_mappings: DashMap<String, Arc<LoadBalancer<RoundRobin>>>,
}

impl BackendsManager {
    pub fn new() -> Self {
        Self {
            backend_mappings: DashMap::new()
        }
    }

    pub fn get_or_create_backends(&self, name: &String)
                                  -> Arc<LoadBalancer<RoundRobin>>
    {
        let arc = self.backend_mappings.entry("s".into())
            .or_insert_with(|| {
                let discovery = Box::new(DynamicServerDiscovery {});
                let mut backends = Backends::new(discovery);
                backends.set_health_check(TcpHealthCheck::new());
                let balancer = LoadBalancer::from_backends(backends);
                Arc::new(balancer)
            })
            .value()
            .clone();
        arc
    }
}

#[async_trait]
impl BackgroundService for BackendsManager {
    async fn start(&self, shutdown: ShutdownWatch) {
        todo!()
    }
}