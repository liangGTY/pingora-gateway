use std::num::NonZeroUsize;
use lru::LruCache;
use pingora_core::protocols::raw_connect::connect;
use pingora_load_balancing::Backend;
use pingora_load_balancing::health_check::HealthCheck;
use tonic::client::GrpcService;
use tonic::codegen::InterceptedService;
use tonic::Request;
use tonic::transport::Channel;
use tonic_health::pb::health_client::HealthClient;
use tonic_health::pb::HealthCheckRequest;


pub struct GrpcHealthCheck {
    /// Number of successful check to flip from unhealthy to healthy.
    pub consecutive_success: usize,
    /// Number of failed check to flip from healthy to unhealthy.
    pub consecutive_failure: usize,

    client_cache: LruCache<Backend, HealthClient<Channel>>,
}

impl GrpcHealthCheck {}

impl HealthCheck for GrpcHealthCheck {
    async fn check(&self, target: &Backend) -> pingora::Result<()> {

        let conn = tonic::transport::Endpoint::from_static(target.to_string().into()).connect().await?;
        let mut client = HealthClient::new(conn);



        let x = client.check(Request::new(HealthCheckRequest {
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