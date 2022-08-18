use std::fs::File;
use std::io::Write;

mod cmdconfig;
mod gotopaths;
mod menu;

static EXCHANGE_PATH: &str = "/tmp/gotochoice";

use menu::GotoMenu;

fn main() {
    let config = cmdconfig::Config::default().unwrap();
    // Gotopaths
    let gps = config.get_gotopaths();

    let menu = GotoMenu::new(gps);

    match menu.show() {
        Some(choice) => {
            File::create(EXCHANGE_PATH)
                .unwrap()
                .write_all(choice.as_bytes())
                .expect("Cannot right to exchange path.");
        }
        None => eprintln!("No path chosen"),
    }
}
