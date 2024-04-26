use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand::Rng;
use std::{error::Error, io::Seek};
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
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
        } else if let Err(e) = reading_result {
            return  Err(FileError::BadFileDescriptorError(e));
        }


        let mut salt = [0u8; 16];
        OsRng.fill(&mut salt);

        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

        let header = format!("PAPM-Vault\n{}\n{}\n",password_hash, salt);

        file.seek(io::SeekFrom::Start(0))?; // Garantir que estamos no início do arquivo
        file.write_all(header.as_bytes())?;

        return Ok(Self { password, file })
    }

    /// Open an already configured vault file.
    pub fn open(password: String, mut file: File) -> Result<String, FileError> {
        let mut readed = String::new();
        let res = file.read_to_string(&mut readed);
        return Ok(readed)
    }
}




#[derive(Debug)]
pub enum FileError {
    CanceledError(),
    BadFileDescriptorError(io::Error),
    ReadError(io::Error),
    InvalidUtf8(String),
    PasswordHashError(argon2::password_hash::Error),
}

impl Error for FileError {}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileError::CanceledError() => write!(f, "Operation canceled"),
            FileError::BadFileDescriptorError(ref path) => write!(f, "The file couldn't be read: {}", path),
            FileError::ReadError(ref err) => write!(f, "Error reading file: {}", err),
            FileError::InvalidUtf8(_) => write!(f, "File content is not valid UTF-8"),
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
