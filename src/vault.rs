use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::{error::Error, io::Seek};

// remove dead code warming
#[allow(dead_code)]
pub struct Vault {
    password: String,
    file: File,
}

impl Vault {
    /// Create and configure a new Vault in a existing file. To open an already configured vault, use `Vault::open()`.
    pub fn new(password: String, mut file: File) -> Result<Self, FileError> {
        let mut buffer = vec![0; 1];
        let reading_result = file.read_exact(&mut buffer);
        if let Ok(_) = reading_result {
            println!(
                "-> The file is not empty. Do you want to overwrite it? (IRREVERSIBLE!) (y/N)"
            );
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim().to_lowercase() == "y" {
                println!("-> ok, overwriting...");
                buffer.clear();
                file.set_len(0).unwrap();
            } else {
                return Err(FileError::CanceledError());
            };
        } // else if let Err(e) = reading_result {
          //     return  Err(FileError::BadFileDescriptorError(e));
          // }

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)?
            .to_string();

        let header = format!("PAPM-Vault\n{}\n{}\n", password_hash, salt);

        file.seek(io::SeekFrom::Start(0))?;
        file.write_all(header.as_bytes())?;

        return Ok(Self { password, file });
    }

    /// Open an already configured vault file.
    pub fn open(password: String, mut file: File) -> Result<String, FileError> {
        let mut readed = String::new();
        let _ = file.read_to_string(&mut readed);
        let (papm_header, readed) = get_next_line(&readed).ok_or(FileError::InvalidFileError())?;
        let (pass_hash, _) = get_next_line(&readed).ok_or(FileError::InvalidFileError())?;
        if papm_header != "PAPM-Vault" {
            return Err(FileError::InvalidFileError());
        };

        let argon2 = Argon2::default();

        match PasswordHash::new(pass_hash) {
            Ok(parsed_password) => {
                match argon2.verify_password(password.as_bytes(), &parsed_password){
                    Ok(_) => {
                        return Ok(" -> Vault configured and ready!".to_string());
                    }
                    Err(e) => {
                        if e == argon2::password_hash::Error::Password {
                            return Ok(" -> The vault seems to be configured, but was not created with the current password.".to_string());
                        }
                        println!("Error: {}", e);
                        return Err(FileError::PasswordHashError(e));
                    }
                
                }
            }
            Err(e) => {
                return Err(FileError::PasswordHashError(e));
            }
        }

    }
}

fn get_next_line(data: &str) -> Option<(&str, &str)> {
    for (i, c) in data.chars().enumerate() {
        if c == '\n' {
            return Some((&data[..i], &data[i + 1..]));
        }
    }

    None
}

#[derive(Debug)]
pub enum FileError {
    CanceledError(),
    InvalidFileError(),
    BadFileDescriptorError(io::Error),
    ReadError(io::Error),
    PasswordHashError(argon2::password_hash::Error),
}

impl Error for FileError {}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileError::CanceledError() => write!(f, "Operation canceled"),
            FileError::InvalidFileError() => write!(f, "Invalid file."),
            FileError::BadFileDescriptorError(ref path) => {
                write!(f, "The file couldn't be read: {}", path)
            }
            FileError::ReadError(ref err) => write!(f, "Error reading file: {}", err),
            FileError::PasswordHashError(e) => write!(f, "Error hashing password: {}", e),
        }
    }
}

impl From<io::Error> for FileError {
    fn from(err: io::Error) -> Self {
        FileError::ReadError(err)
    }
}

impl From<argon2::password_hash::Error> for FileError {
    fn from(err: argon2::password_hash::Error) -> Self {
        FileError::PasswordHashError(err)
    }
}
