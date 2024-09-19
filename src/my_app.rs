use crate::cli::get_cli_commands;
use crate::manpage::get_manpage;
use crate::search_history::{load_search_history, save_search_history};
use serde::{Deserialize, Serialize};
use log::error;
use std::fs;
use dirs;
use serde_json;

#[derive(Default, Serialize, Deserialize)]
pub struct MyApp {
    pub commands: Vec<String>,
    pub filtered_commands: Vec<String>,
    pub selected_command: Option<String>,
    pub manpage: String,
    pub search_query: String,
    pub scroll_to_top: bool,
    pub scroll_to_bottom: bool,
    pub manpage_search_query: String,
    pub scroll_to_selected: bool,
    pub search_history: Vec<String>,
}

impl MyApp {
    pub fn new() -> Self {
        let commands = get_cli_commands();
        let mut app = Self {
            filtered_commands: commands.clone(),
            commands,
            selected_command: None,
            manpage: String::new(),
            search_query: String::new(),
            scroll_to_top: false,
            scroll_to_bottom: false,
            manpage_search_query: String::new(),
            scroll_to_selected: false,
            search_history: Vec::new(),
        };
        app.load_search_history();
        app
    }

    pub fn filter_commands(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_commands = self.commands.clone();
        } else {
            self.filtered_commands = self.commands.iter()
                .filter(|cmd| cmd.contains(&self.search_query))
                .cloned()
                .collect();
        }
        self.filtered_commands.sort_by_key(|cmd| cmd.len());
        
        if !self.filtered_commands.is_empty() {
            let first_command = self.filtered_commands[0].clone();
            self.selected_command = Some(first_command.clone());
            self.scroll_to_selected = true;
            self.update_manpage(&first_command);
        } else {
            self.selected_command = None;
            self.manpage.clear();
        }
    }
    
    

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