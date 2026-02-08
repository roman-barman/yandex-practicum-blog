#[derive(clap::Parser, Debug)]
pub(super) struct Args {
    #[clap(subcommand)]
    pub command: Command,

    /// The address of the server
    #[clap(short, long)]
    pub address: String,

    /// Use gRPC instead of HTTP
    #[clap(short, long)]
    pub grpc: bool
}

#[derive(clap::Subcommand, Debug, Clone)]
pub(super) enum Command {
    /// Login to the server
    Login {
        username: String,
        password: String
    }
}
