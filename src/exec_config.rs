use clap::Parser;

/// Execution configuration used by clap
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct ExecConfig {
    /// Path to go to (if nothing its passed, then it will be prompted through stdin)
    #[arg(value_enum)]
    pub selected_option: Option<String>,

    /// Location of the gotopaths file to use
    #[arg(short, long)]
    pub goto_paths_location: Option<String>,
}
