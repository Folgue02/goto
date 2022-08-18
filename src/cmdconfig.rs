use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;

use crate::gotopaths::GotoPaths;

pub struct Config {
    path_file: String,
}

impl Config {
    pub fn default() -> io::Result<Self> {
        Self::with_pathfile(format!("{}/.gotopaths", std::env::var("HOME").unwrap()))
    }
    pub fn with_pathfile(path_file: String) -> io::Result<Self> {
        if let Ok(p) = fs::metadata(&path_file) {
            if !p.is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "The paths file doesn't correspond to a directory!",
                ));
            } else if p.permissions().mode() & 0o400 == 0 {
                // Check if it can be read
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "Cannot read file (not readable)",
                ));
            } else {
                return Ok(Self {
                    path_file: std::fs::canonicalize(&path_file)
                        .unwrap()
                        .to_string_lossy()
                        .to_string(),
                });
            }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The file doesn't exist.",
            ));
        }
    }

    pub fn get_gotopaths(&self) -> GotoPaths {
        let mut file_content = String::new();
        File::open(&self.path_file)
            .unwrap()
            .read_to_string(&mut file_content)
            .expect("Cannot read content of gotopaths file.");
        GotoPaths::from_string(file_content)
    }
}
