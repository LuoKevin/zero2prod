use std::net::TcpListener;
use env_logger::Env;
use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    //`init` calls set_logger. uses RUST_LOG or `info`
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //Panic if no config
    let configuration = get_configuration().expect("No config available, idiot.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("No can connect to Postgres :(");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
