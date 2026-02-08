use uuid::Uuid;

#[derive(clap::Parser, Debug)]
pub(super) struct Args {
    #[clap(subcommand)]
    pub command: Command,

    /// The address of the server
    #[clap(short, long)]
    pub address: String,

    /// Use gRPC instead of HTTP
    #[clap(short, long)]
    pub grpc: bool,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub(super) enum Command {
    /// Login to the server
    Login { username: String, password: String },
    /// Register a new user
    RegisterUser {
        username: String,
        password: String,
        email: String,
    },
    /// Create a new post
    CreatePost { title: String, content: String },
    /// Update a post
    UpdatePost {
        id: Uuid,
        title: String,
        content: String,
    },
    /// Delete a post
    DeletePost { id: Uuid },
    /// Get a post
    GetPost { id: Uuid },
    /// Get a list of posts
    GetPostsList { limit: usize, offset: usize },
}
