use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;

use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let app_address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(app_address).expect("Failed to assign the port");
    run(listener)?.await
}