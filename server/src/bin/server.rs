use async_trait::async_trait;
use log::info;
use prometheus::register_int_counter;
use structopt::StructOpt;

use pingora_core::server::configuration::Opt;
use pingora_core::server::Server;
use pingora_core::upstreams::peer::{HttpPeer, Peer};
use pingora_core::Result;
use pingora_http::{RequestHeader, ResponseHeader};
use pingora_proxy::{ProxyHttp, Session};
use tonic::Request;
use tonic_health::pb::health_client::HealthClient;
use tonic_health::pb::HealthCheckRequest;

pub struct Gateway {
    req_metric: prometheus::IntCounter,
}

#[async_trait]
impl ProxyHttp for Gateway {
    type CTX = ();
    fn new_ctx(&self) -> Self::CTX {}

    async fn upstream_peer(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> Result<Box<HttpPeer>> {
        todo!("load-blance");
        let addr = ("localhost", 50051);
        let mut peer = Box::new(HttpPeer::new(addr, false, "one.one.one.one".to_string()));
        peer.options.set_http_version(2, 2);
        Ok(peer)
    }

    async fn upstream_request_filter(&self, _session: &mut Session, _upstream_request: &mut RequestHeader, _ctx: &mut Self::CTX) -> Result<()> where Self::CTX: Send + Sync {
        info!("{_upstream_request:?}");
        Ok(())
    }


    async fn response_filter(
        &self,
        _session: &mut Session,
        upstream_response: &mut ResponseHeader,
        _ctx: &mut Self::CTX,
    ) -> Result<()>
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

    async fn logging(
        &self,
        session: &mut Session,
        _e: Option<&pingora_core::Error>,
        ctx: &mut Self::CTX,
    ) {
        let response_code = session
            .response_written()
            .map_or(0, |resp| resp.status.as_u16());
        info!(
            "{} response code: {response_code}",
            self.request_summary(session, ctx)
        );

        self.req_metric.inc();
    }
}

fn main() {
    env_logger::init();

    // read command line arguments
    let opt = Opt::from_args();
    let mut my_server = Server::new(Some(opt)).unwrap();
    my_server.bootstrap();

    let mut my_proxy = pingora_proxy::http_proxy_service(
        &my_server.configuration,
        Gateway {
            req_metric: register_int_counter!("reg_counter", "Number of requests").unwrap(),
        },
    );

    let cert_path = format!("{}/tests/keys/server.pem", env!("CARGO_MANIFEST_DIR"));
    let key_path = format!("{}/tests/keys/server.key", env!("CARGO_MANIFEST_DIR"));

    let mut tls_settings = pingora_core::listeners::TlsSettings::intermediate(&cert_path, &key_path).unwrap();
    tls_settings.enable_h2();

    my_proxy.add_tls_with_settings("0.0.0.0:6189", None, tls_settings);

    my_server.add_service(my_proxy);
    my_server.run_forever();
}