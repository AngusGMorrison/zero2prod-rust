use std::net::TcpListener;

use sqlx::PgPool;
use zero2prod::{config::get_config, startup::run, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::new_subscriber("zero2prod".into(), "info".into());
    telemetry::init_subscriber(subscriber);

    let cfg = get_config().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", cfg.app_port);
    let listener = TcpListener::bind(address)?;
    let conn_pool = PgPool::connect(&cfg.db.conn_string())
        .await
        .expect("Failed to connect to Postgres");
    run(listener, conn_pool)?.await
}
