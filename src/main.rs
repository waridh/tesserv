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

    /**
    Starts the server to serve. Requires a configuration file with the
    description of the assignment endpoints.
     */
    Serve { config: PathBuf, port: Option<u16> },

    /** verifies the correctness of a configuration file */
    Verify { path: PathBuf },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Serve { port, config } => {
            let run_port = if let Some(x) = port { x } else { 3030 };
            let config_ref = config.as_path();
            match adapter::tesserv_config::TesservConfig::try_from_cmd_line(config_ref) {
                Err(e) => {
                    println!(
                        "failed to parse the configuration file at {}\ngot following message {}",
                        config_ref.display(),
                        e
                    );
                    std::process::exit(-1)
                }
                Ok(x) => run_server(run_port, x, None).await,
            }
        }
        Commands::Hash { path } => {
            let hash = hash_file(path);
            match hash {
                Ok(x) => println!("{}", x),
                Err(()) => {
                    eprintln!("unable to hash file");
                    std::process::exit(-1)
                }
            }
        }
        Commands::Verify { path } => {
            let path_ref = path.as_path();
            let built = adapter::tesserv_config::TesservConfig::try_from_cmd_line(path_ref);
            println!("{:?}", built);
            match built {
                Err(e) => {
                    println!(
                        "failed to parse the configuration file at {}\ngot following message {}",
                        path_ref.display(),
                        e
                    );
                    std::process::exit(-1)
                }
                Ok(x) => {
                    println!(
                        "verification successful. Found the following data structure:\n{:?}",
                        x
                    );
                    std::process::exit(0)
                }
            }
        }
    }
}
