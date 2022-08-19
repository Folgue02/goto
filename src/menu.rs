use crate::gotopaths::GotoPaths;
use colored::Colorize;
use std::io::{stdin, stdout, Write};

pub struct GotoMenu {
    pub paths: GotoPaths,
}

impl GotoMenu {
    pub fn new(gps: GotoPaths) -> Self {
        Self { paths: gps }
    }

    pub fn show(&self) -> Option<String> {
        eprintln!("{}", "GOTO?".red());

        loop {
            // Display options
            // TODO: Change with cli flags.
            self.paths.display_options(true);

            // Ask for input
            let mut choice = String::new();
            print!(
                "{}",
                "Select the path you want by typing the chars related to it: ".green()
            );
            stdout().flush().unwrap();
            stdin().read_line(&mut choice).unwrap();
            choice = choice.trim().to_string();
            if let Some(p) = self.paths.option_paths.get(&choice) {
                return Some(p.clone());
            } else if choice == "q" {
                return None;
            } else {
                eprintln!(
                    "'{}' its not related to any path. If you want to quit you can use 'q'",
                    choice
                );
                continue;
            }
        }
    }
}
