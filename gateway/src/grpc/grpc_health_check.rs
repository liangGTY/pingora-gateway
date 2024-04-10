use std::cell::{Cell, RefCell};
use std::sync::Arc;
use async_trait::async_trait;
use lru::LruCache;
use pingora_load_balancing::Backend;
use pingora_load_balancing::health_check::HealthCheck;
use tonic::Request;
use tonic::transport::Channel;
use tonic_health::pb::health_client::HealthClient;
use tonic_health::pb::HealthCheckRequest;


pub struct GrpcHealthCheck {
    /// Number of successful check to flip from unhealthy to healthy.
    pub consecutive_success: usize,
    /// Number of failed check to flip from healthy to unhealthy.
    pub consecutive_failure: usize,

    client_cache: RefCell<LruCache<Backend, HealthClient<Channel>>>,
}

impl GrpcHealthCheck {}

#[async_trait]
impl HealthCheck for GrpcHealthCheck {
    async fn check(&self, target: &Backend) -> pingora_error::Result<()> {
        let health_client = self.client_cache
            .borrow_mut()
            .get_or_insert_mut(target.clone(), async || {
                let conn = tonic::transport::Endpoint::from_static("").connect().await;

                return HealthClient::new(conn.unwrap());
            });

        let x = health_client.check(Request::new(HealthCheckRequest {
            service: "".to_string()
        })).await;

        Ok(())
    }

    fn health_threshold(&self, success: bool) -> usize {
        if success {
            self.consecutive_success
        } else {
            self.consecutive_failure
        }
    }
}