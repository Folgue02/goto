mod cmdconfig;
mod collections;
mod gotopaths;
mod menu;

static EXCHANGE_PATH: &str = "/tmp/gotochoice";

fn main() {
    let cli_args = std::env::args().collect::<Vec<String>>();
    let gotopaths_file =
        std::env::var("HOME").expect("Cannot retrieve the home directory.") + "/.gotopaths";
    let mut selected_option: Option<String> = None;

    if cli_args.len() > 1 {
        selected_option = Some(cli_args[1].to_string());
    }

    let config = cmdconfig::Config::with_pathfile(gotopaths_file, selected_option).unwrap();

    match config.execute() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Something went wrong.")
        }
    }
    /*
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
    */
}
