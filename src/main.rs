use std::process::Command;
use eframe::egui;
use egui::{text::LayoutJob, FontId, TextFormat, Align};
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use log::{error, debug};

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CLI Organizer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    );
}

#[derive(Default)]
struct MyApp {
    commands: Vec<String>,
    filtered_commands: Vec<String>,
    selected_command: Option<String>,
    manpage: String,
    search_query: String,
    scroll_to_top: bool,
    scroll_to_bottom: bool,
}

impl MyApp {
    fn new() -> Self {
        let commands = get_cli_commands();
        Self {
            filtered_commands: commands.clone(),
            commands,
            selected_command: None,
            manpage: String::new(),
            search_query: String::new(),
            scroll_to_top: false,
            scroll_to_bottom: false,
        }
    }

    fn filter_commands(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_commands = self.commands.clone();
        } else {
            self.filtered_commands = self.commands.iter()
                .filter(|cmd| cmd.contains(&self.search_query))
                .cloned()
                .collect();
        }
        self.filtered_commands.sort_by_key(|cmd| cmd.len());
    }

    fn select_next(&mut self) {
        let mut update_needed = false;
        if let Some(selected) = &self.selected_command {
            if let Some(index) = self.filtered_commands.iter().position(|cmd| cmd == selected) {
                if index + 1 < self.filtered_commands.len() {
                    self.selected_command = Some(self.filtered_commands[index + 1].clone());
                    update_needed = true;
                }
            }
        } else if !self.filtered_commands.is_empty() {
            self.selected_command = Some(self.filtered_commands[0].clone());
            update_needed = true;
        }
        if update_needed {
            if let Some(command) = &self.selected_command {
                self.update_manpage(&command.clone());
            }
        }
    }

    fn select_previous(&mut self) {
        let mut update_needed = false;
        if let Some(selected) = &self.selected_command {
            if let Some(index) = self.filtered_commands.iter().position(|cmd| cmd == selected) {
                if index > 0 {
                    self.selected_command = Some(self.filtered_commands[index - 1].clone());
                    update_needed = true;
                }
            }
        } else if !self.filtered_commands.is_empty() {
            self.selected_command = Some(self.filtered_commands[0].clone());
            update_needed = true;
        }
        if update_needed {
            if let Some(command) = &self.selected_command {
                self.update_manpage(&command.clone());
            }
        }
    }

    fn update_manpage(&mut self, command: &str) {
        match get_manpage(command) {
            Ok(manpage) => {
                self.manpage = manpage;
                self.scroll_to_top = true;
                self.scroll_to_bottom = false;
            },
            Err(e) => error!("Failed to fetch manpage: {}", e),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Programme");

            ui.horizontal(|ui| {
                ui.label("Suche:");
                if ui.text_edit_singleline(&mut self.search_query).changed() {
                    self.filter_commands();
                }
            });

            let scroll_area = egui::ScrollArea::vertical();
            scroll_area.show(ui, |ui| {
                for command in self.filtered_commands.clone() {
                    if ui.selectable_label(self.selected_command.as_ref() == Some(&command), &command).clicked() {
                        self.selected_command = Some(command.clone());
                        self.update_manpage(&command);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Manpage");

            ui.horizontal(|ui| {
                if ui.button("⬆ Scroll to Top").clicked() {
                    self.scroll_to_top = true;
                    self.scroll_to_bottom = false;
                }
                if ui.button("⬇ Scroll to Bottom").clicked() {
                    self.scroll_to_bottom = true;
                    self.scroll_to_top = false;
                }
            });

            let mut scroll_area = egui::ScrollArea::vertical().id_source("manpage_scroll_area");
            
            if self.scroll_to_top {
                scroll_area = scroll_area.vertical_scroll_offset(0.0);
                self.scroll_to_top = false;
            }

            scroll_area.show(ui, |ui| {
                if self.manpage.is_empty() {
                    ui.label("Nicht verfügbar");
                } else {
                    let syntax_set = SyntaxSet::load_defaults_newlines();
                    let theme_set = ThemeSet::load_defaults();
                    let syntax = syntax_set.find_syntax_plain_text();
                    let mut h = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);

                    let mut job = LayoutJob::default();
                    for line in LinesWithEndings::from(&self.manpage) {
                        let ranges: Vec<(Style, &str)> = h.highlight_line(line, &syntax_set).unwrap();
                        for (style, text) in ranges {
                            let color = egui::Color32::from_rgb(style.foreground.r, style.foreground.g, style.foreground.b);
                            job.append(text, 0.0, TextFormat::simple(FontId::default(), color));
                        }
                    }
                    ui.label(job);

                    if self.scroll_to_bottom {
                        ui.scroll_to_cursor(Some(Align::BOTTOM));
                        self.scroll_to_bottom = false;
                    }
                }
            });
        });

        ctx.input(|i| {
            if i.key_pressed(egui::Key::ArrowDown) {
                self.select_next();
            }
            if i.key_pressed(egui::Key::ArrowUp) {
                self.select_previous();
            }
        });
    }
}

fn get_cli_commands() -> Vec<String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg("compgen -c")
        .output()
        .expect("Failed to execute command");
    let commands = String::from_utf8_lossy(&output.stdout);
    let result: Vec<String> = commands.lines().map(|s| s.to_string()).collect();
    debug!("Fetched {} CLI commands", result.len());
    result
}

fn get_manpage(command: &str) -> Result<String, std::io::Error> {
    debug!("Fetching manpage for command: {}", command);
    let output = Command::new("man")
        .arg(command)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(std::io::Error::new(std::io::ErrorKind::Other, error.to_string()))
    }
}
