use colored::Colorize;
use std::collections::HashMap;

pub struct GotoPaths {
    pub option_paths: HashMap<String, String>,
}

impl GotoPaths {
    // TODO: Use &str instead of String
    pub fn from_string(content: String) -> Self {
        let mut option_paths = HashMap::new();
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

    /*
    pub fn get_menu_options(&self) -> (Vec<String>, &Vec<String>) {
        let mut options = Vec::new();
        for path in &self.paths {
            let mut length: usize = 1;
            let last_node = match path.split('/').last() {
                Some(s) => s.trim(),
                None => continue,
            };

            if last_node.len() == 0 {
                // No last path
                options.push(last_node.to_string());
                continue;
            }

            loop {
                if options.contains(&last_node[..length].to_string()) {
                    // There is already an option like this.
                    length += 1;
                    continue;
                } else {
                    options.push(last_node[..length].to_string());
                    break;
                }
            }

            /*
            loop {
                if options.contains(&path[..length].to_string()) || &path[..length] == "q" {
                    // That option already exists. || The option cannot be Q
                    length += 1;
                    continue;
                } else if length >= path.len() {
                    // Path repeated too many times (Give up on trying to add options to a path that its constantly repeated.)
                    // TODO: Look for a better way of handling this.
                    break;
                } else {
                    options.push(path[..length].to_string());
                    break;
                }
            }
            */
        }

        (options, &self.paths)
    }
    */
    /// Displays the option related to the path which is displayed at the left.
    pub fn display_options(&self) {
        for (k, v) in &self.option_paths {
            eprintln!("[{}] {}", k.green(), v)
        }
    }
}
