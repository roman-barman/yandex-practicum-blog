use crate::infrastructure::initialize_tracing_subscribe;
use crate::server::Server;

mod api;
mod application;
mod configuration;
mod domain;
mod infrastructure;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = configuration::Configuration::read_configuration()?;
    initialize_tracing_subscribe(config.get_server_configuration().get_log_level())?;
    let server = Server::start(config).await?;
    let _ = tokio::spawn(server.run_until_shutdown()).await?;
    Ok(())
}
