use std::net::TcpListener;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber(
        "zero2prod".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);
    //Panic if no config
    let configuration = get_configuration().expect("No config available, idiot.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let address = format!("{}:{}",
                          configuration.application.host, configuration.application.port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
