use crate::server::Server;

mod server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = Server::start().await?;
    let _ = tokio::spawn(server.run_until_shutdown()).await?;
    Ok(())
}
