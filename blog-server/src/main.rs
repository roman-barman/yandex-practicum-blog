use crate::server::Server;

mod configuration;
mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = configuration::Configuration::read_configuration()?;
    let server = Server::start(config).await?;
    let _ = tokio::spawn(server.run_until_shutdown()).await?;
    Ok(())
}
