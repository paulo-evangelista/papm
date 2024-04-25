use clap::{Parser, Subcommand};
use std::{env, fs, io::{self, Read, Write}, panic::catch_unwind};

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
    #[command(about = "Create a new password storage in the given file, encrypted with the provided password", long_about = None)]
    CreateStorage {
        #[arg(help = "The file path" , default_value_t = String::from("~/.papm"))]
        file: String,
    },
    Set {
        // Arguments for 'set' command
    },
}

fn main() {
    let cli = Cli::parse();

    let password = cli.password.or_else(|| env::var("PAPM_PASSWORD").ok());

    if password.is_none() {
        eprintln!("-> No password provided!\n   You can provide the password using the '--password' flag or the 'PAPM_PASSWORD' environment variable.");
        // Exit the program
        std::process::exit(1);
    }

    match &cli.command {
        Commands::Get { name } => {
            println!("Retrieving password for: {}", name);
            // Implement the logic to retrieve the password
        },
        Commands::Set { .. } => {
            // Implementation for 'set' command
        },
        Commands::CreateStorage {file} => {
            let canonical_path = fs::canonicalize(file).unwrap_or_else(|_| {
                eprintln!("-> Error getting the canonical path for: {}", file);
                std::process::exit(1);
            });

            let file_contents = fs::read(canonical_path).unwrap_or_else(|_| {
                eprintln!("-> Error reading file: {}", file);
                std::process::exit(1);
            });

            if !file_header.is_empty() {
                print!("-> Attention! The file is not empty. Do you want to overwrite it? (y/N): ");
                std::io::stdout().flush().unwrap();
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim().to_lowercase() == "y" {
                    println!("-> ok, overwriting...");
                } else {
                    println!("-> Cancelled.");
                    std::process::exit(0);
                }
            }
        },
    }
}

fn read_first_10_bytes(file_path: &str) -> Option<String> {
    let mut file = fs::File::open(file_path);
    let mut buffer = vec![0; 20]; // Cria um buffer para 10 bytes
    fs::File::read_exact(&mut buffer).is_err(); // Lê exatamente 10 bytes
    match String::from_utf8(buffer) {
        Ok(s) => Some(s),
        Err(_) => None
    }
}