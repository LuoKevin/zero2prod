use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //Panic if no config
    let configuration = get_configuration().expect("No config available, idiot.");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener)?.await
}
