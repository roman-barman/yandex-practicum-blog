use clap::Parser;
use blog_client::BlogClient;

mod args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    let protocol = if args.grpc { blog_client::Protocol::Grpc(args.address) } else { blog_client::Protocol::Http(args.address) };
    let mut client = blog_client::Client::new(protocol).await?;

    match args.command {
        args::Command::Login { username, password } => {
            let _ = client.login(blog_client::LoginCommand::new(username, password)).await?;
            println!("login successful");
        }
    }

    Ok(())
}
