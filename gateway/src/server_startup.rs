use pingora_core::prelude::Opt;
use pingora_core::server::Server;
use pingora_core::services::listening::Service;
use pingora_proxy::HttpProxy;
use prometheus::register_int_counter;
use structopt::StructOpt;
use crate::backends::backends_manager::BackendsManager;
use crate::server_app::ServerApp;

pub struct ServerStartup {
    server: Server,
}

impl ServerStartup {
    pub fn new() -> ServerStartup {
        let mut server = Server::new(Some(Opt::from_args())).unwrap();

        server.bootstrap();

        let service = get_proxy_service(&server);

        server.add_service(service);

        Self {
            server
        }
    }

    pub fn start(&mut self) {
        self.server.run_forever();
    }
}

fn get_proxy_service(server: &Server) -> Service<HttpProxy<ServerApp>> {
    let mut proxy = pingora_proxy::http_proxy_service(
        &server.configuration,
        ServerApp {
            backends_manager: BackendsManager::new()
        },
    );
    let cert_path = format!("{}/tests/keys/server.pem", env!("CARGO_MANIFEST_DIR"));
    let key_path = format!("{}/tests/keys/server.key", env!("CARGO_MANIFEST_DIR"));

    let mut tls_settings = pingora_core::listeners::TlsSettings::intermediate(&cert_path, &key_path).unwrap();
    tls_settings.enable_h2();

    proxy.add_tls_with_settings("0.0.0.0:443", None, tls_settings);

    proxy
}