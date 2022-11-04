use clap::Parser;

mod cmdconfig;
mod collections;
mod gotopaths;
mod menu;
mod exec_config;

static EXCHANGE_PATH: &str = "/tmp/gotochoice";

fn main() {

    let parsed_args = exec_config::ExecConfig::parse();

    let gotopaths_file = match parsed_args.goto_paths_location {
        Some(s) => s,
        None => std::env::var("HOME").expect("Cannot retrieve the home directory.") + "/.gotopaths"
    };

    let config = cmdconfig::Config::with_pathfile(gotopaths_file, parsed_args.selected_option).unwrap();

    match config.execute() {
        Ok(_) => (),
        Err(_) => {
            eprintln!("Something went wrong.")
        }
    }
}
