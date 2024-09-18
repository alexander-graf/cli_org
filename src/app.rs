use crate::cli::get_cli_commands;
use crate::manpage::get_manpage;
use eframe::egui;
use egui::{text::LayoutJob, FontId, TextFormat, Align};
use syntect::easy::HighlightLines;
use syntect::highlighting::{ThemeSet, Style};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use log::error;
use webbrowser;
use urlencoding;

#[derive(Default)]
pub struct MyApp {
    pub commands: Vec<String>,
    pub filtered_commands: Vec<String>,
    pub selected_command: Option<String>,
    pub manpage: String,
    pub search_query: String,
    pub scroll_to_top: bool,
    pub scroll_to_bottom: bool,
    pub manpage_search_query: String,
}

impl MyApp {
    pub fn new() -> Self {
        let commands = get_cli_commands();
        Self {
            filtered_commands: commands.clone(),
            commands,
            selected_command: None,
            manpage: String::new(),
            search_query: String::new(),
            scroll_to_top: false,
            scroll_to_bottom: false,
            manpage_search_query: String::new(),
        }
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
    }

    pub fn select_next(&mut self) {
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

    pub fn select_previous(&mut self) {
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

    pub fn update_manpage(&mut self, command: &str) {
        match get_manpage(command) {
            Ok(manpage) => {
                self.manpage = manpage;
                self.scroll_to_top = true;
                self.scroll_to_bottom = false;
            },
            Err(e) => error!("Failed to fetch manpage: {}", e),
        }
    }

    pub fn filter_manpage(&self) -> String {
        if self.manpage_search_query.is_empty() {
            return self.manpage.clone();
        }

        self.manpage
            .lines()
            .filter(|line| line.contains(&self.manpage_search_query))
            .collect::<Vec<&str>>()
            .join("\n")
    }

    pub fn open_perplexity_search(&self) {
        if let Some(command) = &self.selected_command {
            let query = format!("{} zeige mir beispielanwendungen für diesen befehl", command);
            let url = format!("https://www.perplexity.ai/search?q={}", urlencoding::encode(&query));
            if webbrowser::open(&url).is_err() {
                error!("Failed to open web browser");
            }
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Beispiel").clicked() {
                self.open_perplexity_search();
            }
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

            ui.horizontal(|ui| {
                ui.label("Suche in Manpage:");
                if ui.text_edit_singleline(&mut self.manpage_search_query).changed() {
                    self.scroll_to_top = true;
                    self.scroll_to_bottom = false;
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
                    let filtered_manpage = self.filter_manpage();
                    let syntax_set = SyntaxSet::load_defaults_newlines();
                    let theme_set = ThemeSet::load_defaults();
                    let syntax = syntax_set.find_syntax_plain_text();
                    let mut h = HighlightLines::new(syntax, &theme_set.themes["base16-ocean.dark"]);

                    let mut job = LayoutJob::default();
                    for line in LinesWithEndings::from(&filtered_manpage) {
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