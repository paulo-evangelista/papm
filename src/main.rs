use clap::{Parser, Subcommand};
use std::{env, fs, io::{Read, Write}, ops::Deref, os::fd::AsFd};
use vault::Vault;

mod vault;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, help = "The password to use for authentication")]
    password: Option<String>,

    #[arg(short, long, help = "The file to process", default_value_t = String::from("~/.zshrc"))]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Get a password", long_about = None)]
    Get {
        #[arg(help = "The name of the password to retrieve")]
        name: String,
    },
    #[command(about = "Create a new password vault in the given file, encrypted with the provided password", long_about = None)]
    CreateVault {
        #[arg(help = "The file path" , default_value_t = String::from("~/.papm"))]
        file: String,
    },
    Set {
        // Arguments for 'set' command
    },
    #[command(about = "Check if a Vault is correctly configured", long_about = None)]
    Check {
        #[arg(help = "The file path" , default_value_t = String::from("~/.papm"))]
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let password = cli.password.or_else(|| env::var("PAPM_PASSWORD").ok()).unwrap_or_else(||{
        eprintln!("-> No password provided!\n   You can provide the password using the '--password' flag or the 'PAPM_PASSWORD' environment variable.");
        // Exit the program
        std::process::exit(1);
    
    });

    match &cli.command {
        Commands::Get { name } => {
            println!("Retrieving password for: {}", name);
            // Implement the logic to retrieve the password
        }
        Commands::Set { .. } => {
            // Implementation for 'set' command
        }
        Commands::CreateVault { file } => {
            let canonical_path = fs::canonicalize(file).unwrap_or_else(|_| {
                eprintln!("-> The file does not exist. Make sure to create it before running this command.");
                std::process::exit(1);
            });

            match fs::OpenOptions::new().write(true).read(true).open(canonical_path) {
                Ok(opened_file) => {

                    match Vault::new(password, opened_file) {
                        Ok(vault) => {
                            println!("-> Vault created successfully! Remember to keep your password safe.");
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    
                    }

                }
                Err(e) => {
                    eprintln!("-> Error opening the file: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Check { file } => {
            let canonical_path = fs::canonicalize(file).unwrap_or_else(|_| {
                eprintln!("-> The file does not exist. Make sure to create it before running this command.");
                std::process::exit(1);
            });

            match fs::OpenOptions::new().write(true).read(true).open(canonical_path) {
                Ok(opened_file) => {

                    match Vault::open(password, opened_file) {
                        Ok(content) => {
                            println!("{}", content);
                        }
                        Err(e) => {
                            eprintln!("{}", e);
                            std::process::exit(1);
                        }
                    
                    }

                }
                Err(e) => {
                    eprintln!("-> Error opening the file: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
