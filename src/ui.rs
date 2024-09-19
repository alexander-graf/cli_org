use egui::{CentralPanel, ScrollArea, SidePanel, Align};
use syntect::highlighting::{ThemeSet, Style, HighlightLines};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;
use crate::my_app::MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Left panel for commands
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Programme");

            ui.horizontal(|ui| {
                ui.label("Suche:");
                if ui.text_edit_singleline(&mut self.search_query).changed() {
                    self.filter_commands();
                    self.add_to_search_history(self.search_query.clone());
                }
            });

            let scroll_area = egui::ScrollArea::vertical();
            scroll_area.show(ui, |ui| {
                for command in self.filtered_commands.clone() {
                    let is_selected = self.selected_command.as_ref() == Some(&command);
                    if ui.selectable_label(is_selected, &command).clicked() {
                        self.selected_command = Some(command.clone());
                        self.update_manpage(&command);
                    }
                    if is_selected && self.scroll_to_selected {
                        ui.scroll_to_cursor(Some(Align::Center));
                        self.scroll_to_selected = false;
                    }
                }
            });
        });

        // Central panel for manpage
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Manpage");

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
                            // Highlighting logic here
                        }
                    }
                }
            });
        });

        // Right panel for search history
        egui::SidePanel::right("history_panel").show(ctx, |ui| {
            ui.heading("Suchhistorie");
            for query in &self.search_history {
                if ui.button(query).clicked() {
                    self.search_query = query.clone();
                    self.filter_commands();
                    self.update_manpage(query);
                }
            }
        });
    }
}