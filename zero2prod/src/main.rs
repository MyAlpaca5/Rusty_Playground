use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::{configurations::get_configuration, startup::run};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read the configuration file");

    let connection_str = configuration.database.get_connection_string();
    let connection_pool = PgPool::connect(&connection_str)
        .await
        .expect("Failed to create connection pool");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind the address");

    run(listener, connection_pool)?.await
}
