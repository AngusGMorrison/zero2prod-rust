use std::net::TcpListener;

use zero2prod::{config::get_config, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cfg = get_config().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", cfg.app_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
