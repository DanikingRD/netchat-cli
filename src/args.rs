use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Daniking")]
pub struct NetchatArgs {
    /// Wether to run in server or client mode
    #[clap(subcommand)]
    pub mode: Mode,

    /// The username that will be displayed to other clients
    pub username: String,
}

#[derive(Debug, Subcommand)]
pub enum Mode {
    Connect(ClientArgs),
    Serve(ServerArgs),
}

impl Mode {
    pub fn as_str(&self) -> &str {
        match self {
            Mode::Connect(_) => "connect",
            Mode::Serve(_) => "serve",
        }
    }
    pub fn is_connect(&self) -> bool {
        match self {
            Mode::Connect(_) => true,
            Mode::Serve(_) => false,
        }
    }
}

#[derive(Debug, Args)]
pub struct ServerArgs {}

#[derive(Debug, Args)]
pub struct ClientArgs {
    /// The port of the target server
    #[clap(short)]
    pub port: u16,
    /// The IP address of the target server, defaults to localhost
    #[clap(short, default_value = "127.0.0.1")]
    pub addr: String,
}
