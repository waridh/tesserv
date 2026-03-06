/*!
Entrypoint to the tesserv program.
Configures the command line interface
 */

use clap::{Parser, Subcommand};
use hasher::hash_file;
use server_runtime::run_server;
use std::path::PathBuf;

mod adapter;
mod hasher;
mod server_runtime;
mod types;

/// Server sided command line tooling
#[derive(Parser, Debug)]
#[command(name = "tesserv")]
#[command(version, about = "System for validating programming assignments", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/** Denotes the different subcommands that are available for the user to invoke
 */
#[derive(Debug, Subcommand)]
enum Commands {
    /** utility that returns the hash of a target file */
    Hash { path: PathBuf },
    /** Starts the server to serve */
    Serve { port: Option<u16> },
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
