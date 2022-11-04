use crate::gotopaths::GotoPaths;
use colored::Colorize;
use std::io::{stdin, stdout, Write};
/// Displays the menu and returns the path chosen by the user wrapped in a `Some`,
/// if the user quits from the menu this method will return a `None`
pub fn show(paths: &GotoPaths) -> Option<String> {
    eprintln!("{}", "GOTO?".red());

    loop {
        // Display options
        paths.display_options(true);

        // Ask for input
        let mut choice = String::new();
        print!(
            "{}",
            "Select the path you want by typing the chars related to it: ".green()
        );
        stdout().flush().unwrap();
        stdin().read_line(&mut choice).unwrap();
        choice = choice.trim().to_string();
        if let Some(p) = paths.option_paths.get(&choice) {
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
