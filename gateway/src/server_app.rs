use async_trait::async_trait;
use log::info;
use pingora_core::prelude::HttpPeer;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};

pub struct ServerApp {}

#[async_trait]
impl ProxyHttp for ServerApp {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora_error::Result<Box<HttpPeer>> {
        // todo!("load-blance");
        let addr = ("localhost", 50051);
        let mut peer = Box::new(HttpPeer::new(addr, false, "one.one.one.one".to_string()));
        peer.options.set_http_version(2, 2);
        Ok(peer)
    }

    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> pingora_error::Result<()>
        where
            Self::CTX: Send + Sync,
    {
        // replace existing header if any
        upstream_response
            .insert_header("Server", "MyGateway")
            .unwrap();
        // because we don't support h3
        upstream_response.remove_header("alt-svc");
        println!("{:?}", upstream_response);
        Ok(())
    }
}