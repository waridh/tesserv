use clap::{Parser, Subcommand};
use hasher::hash_file;
use server_runtime::run_server;
use std::path::PathBuf;

mod hasher;
pub mod log_store;
mod server_runtime;
pub mod types;

/// Server sided command line tooling
#[derive(Parser, Debug)]
#[command(name = "tesserv")]
#[command(version, about = "System for validating programming assignments", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/* Denotes the different subcommands that are available for the user to invoke
 */
#[derive(Debug, Subcommand)]
enum Commands {
    /** Starts the server to serve */
    Serve {
        port: Option<u16>,
    },
    Hash {
        path: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Serve { port } => {
            let run_port = if let Some(x) = port { x } else { 3030 };
            run_server(run_port, None).await;
        }
        Commands::Hash { path } => {
            let hash = hash_file(path);
            match hash {
                Ok(x) => println!("{}", x),
                Err(()) => eprintln!("unable to hash file"),
            }
        }
    }
}
