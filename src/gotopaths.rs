use colored::Colorize;
use crate::collections::SortedHashMap;

#[derive(Clone)]
pub struct GotoPaths {
    pub option_paths: SortedHashMap<String, String>,
}

impl GotoPaths {
    // TODO: Use &str instead of String
    pub fn from_string(content: String) -> Self {
        let mut option_paths = SortedHashMap::new();
        for line in content.lines() {
            // The format for the gotopaths is <path>;<option>
            if let Some(sc_pos) = line.find(';') {
                let (p, option) = line.split_at(sc_pos);
                if option.len() < 2 {
                    // Doesn't contain an option (i.e. /home/user/Downloads;)
                    continue;
                } else {
                    let option = option[1..].to_string();
                    option_paths.insert(option, p.to_string());
                }
            } else {
                continue;
            }
        }

        Self { option_paths }
    }

    /// Displays the option related to the path which is displayed at the left.
    pub fn display_options(&self, verify_existance: bool) {
        for (k, v) in &self.option_paths {
            eprintln!(
                "[{}] {}",
                k.green(),
                if !verify_existance {
                    v.to_string()
                } else {
                    if std::path::Path::new(&v).is_dir() {
                        v.green().to_string()
                    } else {
                        format!("{} (doesn't exist)", v.red()).to_string()
                    }
                }
            )
        }
    }
}
