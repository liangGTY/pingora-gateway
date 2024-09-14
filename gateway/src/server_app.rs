use std::net::ToSocketAddrs;
use std::time::Duration;
use async_trait::async_trait;
use log::info;
use pingora_core::prelude::HttpPeer;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};
use tonic::codegen::Bytes;
use crate::backends::backends_manager::BackendsManager;

pub struct ServerApp {
    pub backends_manager: BackendsManager,
}

#[async_trait]
impl ProxyHttp for ServerApp {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora_error::Result<Box<HttpPeer>> {
        // let backend = self.backends_manager.get_or_create_backends("".into()).select("".into(), 0).unwrap();
        // let mut peer = Box::new(HttpPeer::new(backend, false, "one.one.one.one".to_string()));

        let mut peer = Box::new(HttpPeer::new(("127.0.0.1", 17137), false, "one.one.one.one".to_string()));
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
    //
    // fn upstream_response_body_filter(&self, _session: &mut Session, _body: &mut Option<Bytes>, _end_of_stream: bool, _ctx: &mut Self::CTX) {
    //
    // }
    //
    // fn response_body_filter(&self, _session: &mut Session, _body: &mut Option<Bytes>, _end_of_stream: bool, _ctx: &mut Self::CTX) -> pingora_error::Result<Option<Duration>>
    // where
    //     Self::CTX: Send + Sync
    // {
    //
    // }
}