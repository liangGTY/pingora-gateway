use gateway::server_startup::ServerStartup;

fn main() {
    env_logger::init();
    ServerStartup::new().start()
}