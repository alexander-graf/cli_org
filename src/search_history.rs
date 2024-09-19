use std::fs;
use serde_json;
use crate::my_app::MyApp;

impl MyApp {
    pub fn add_to_search_history(&mut self, query: String) {
        if !self.search_history.contains(&query) {
            self.search_history.push(query);
            self.save_search_history();
        }
    }

    pub fn save_search_history(&self) {
        let path = dirs::config_dir().unwrap().join("cli_manpage_history.json");
        if let Ok(json) = serde_json::to_string(&self.search_history) {
            if let Err(e) = fs::write(&path, json) {
                println!("Failed to save search history: {}", e);
            }
        }
    }

    pub fn load_search_history(&mut self) {
        let path = dirs::config_dir().unwrap().join("cli_manpage_history.json");
        if path.exists() {
            if let Ok(json) = fs::read_to_string(&path) {
                if let Ok(history) = serde_json::from_str::<Vec<String>>(&json) {
                    self.search_history = history;
                } else {
                    println!("Failed to parse search history");
                }
            } else {
                println!("Failed to read search history");
            }
        }
    }
}