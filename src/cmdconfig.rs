use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;

use crate::gotopaths::GotoPaths;
use crate::menu::GotoMenu;

pub struct Config {
    path_file: String,
    selected_option: Option<String>,
}

impl Config {
    pub fn default() -> io::Result<Self> {
        Self::with_pathfile(
            format!("{}/.gotopaths", std::env::var("HOME").unwrap()),
            None,
        )
    }

    pub fn with_pathfile(path_file: String, selected_option: Option<String>) -> io::Result<Self> {
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
                    selected_option,
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

    pub fn execute(&self) -> Result<(), ()> {
        let gps = self.get_gotopaths();

        let menu = GotoMenu::new(gps.clone());

        if let Some(option) = &self.selected_option {
            if let Some(path) = gps.option_paths.get(option) {
                return match File::create(crate::EXCHANGE_PATH)
                    .expect("Cannot write to exchange path ")
                    .write_all(path.as_bytes())
                {
                    Ok(_) => Ok(()),
                    Err(_) => {
                        eprintln!("Cannot write to the exchange path.");
                        Err(())
                    }
                };
            } else {
                eprintln!("{} its not related to any path.", option);
                return Err(())
            }
        } else {
            // BUG: Doesn't show the gotopaths
            match menu.show() {
                Some(choice) => {
                    match File::create(crate::EXCHANGE_PATH)
                        .unwrap()
                        .write_all(choice.as_bytes())
                    {
                        Ok(_) => (),
                        Err(_) => return Err(()),
                    }
                }
                None => eprintln!("No path chosen"),
            }
        }
        Ok(())
    }
}
