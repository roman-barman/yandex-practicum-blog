use blog_client::{BlogClient, CreatePostCommand};
use clap::Parser;
use std::path::PathBuf;

const TOKEN_FILE: &str = ".blog_token";
const HOME_ENV: &str = "HOME";
const USERPROFILE_ENV: &str = "USERPROFILE";

mod args;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();
    let address = if args.address.starts_with("http://") {
        args.address.to_string()
    } else {
        format!("http://{}", args.address)
    };
    let protocol = if args.grpc {
        blog_client::Protocol::Grpc(address)
    } else {
        blog_client::Protocol::Http(address)
    };
    let mut client = blog_client::Client::new(protocol).await?;

    match args.command {
        args::Command::Login { username, password } => {
            let token = client
                .login(blog_client::LoginCommand::new(username, password))
                .await?;
            save_token(&token)?;
            println!("login successful");
        }
        args::Command::RegisterUser {
            username,
            password,
            email,
        } => {
            client
                .register_user(blog_client::RegisterUserCommand::new(
                    username, password, email,
                ))
                .await?;
            println!("user registered successfully");
        }
        args::Command::CreatePost { title, content } => {
            let token = read_token()?;
            let post = client
                .create_post(blog_client::AuthorizedCommand::new(
                    CreatePostCommand::new(title, content),
                    token.as_str(),
                ))
                .await?;
            println!("post created successfully");
            println!("{}", post);
        }
        args::Command::UpdatePost { id, title, content } => {
            let token = read_token()?;
            let post = client
                .update_post(blog_client::AuthorizedCommand::new(
                    blog_client::UpdatePostCommand::new(id, title, content),
                    token.as_str(),
                ))
                .await?;
            println!("post updated successfully");
            println!("{}", post);
        }
        args::Command::DeletePost { id } => {
            let token = read_token()?;
            client
                .delete_post(blog_client::AuthorizedCommand::new(
                    blog_client::DeletePostCommand::new(id),
                    token.as_str(),
                ))
                .await?;
            println!("post deleted successfully");
        }
        args::Command::GetPost { id } => {
            let post = client
                .get_post(blog_client::GetPostCommand::new(id))
                .await?;
            println!("{}", post);
        }
        args::Command::GetPostsList { limit, offset } => {
            let posts = client
                .get_post_list(blog_client::GetPostsListCommand::new(limit, offset))
                .await?;
            println!("{}", posts);
        }
    }

    Ok(())
}

fn save_token(token: &str) -> anyhow::Result<()> {
    let home = std::env::var(HOME_ENV)
        .or_else(|_| std::env::var(USERPROFILE_ENV))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    let path = PathBuf::from(home).join(TOKEN_FILE);
    std::fs::write(&path, token)?;
    Ok(())
}

fn read_token() -> anyhow::Result<String> {
    let home = std::env::var(HOME_ENV)
        .or_else(|_| std::env::var(USERPROFILE_ENV))
        .map_err(|_| anyhow::anyhow!("Could not find home directory"))?;
    let path = PathBuf::from(home).join(TOKEN_FILE);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "You must login first. Run `blog-cli login` to do so."
        ));
    }
    let token = std::fs::read_to_string(&path)?;
    Ok(token)
}
