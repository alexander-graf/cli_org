use std::process::Command;

fn main() {
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

            egui::ScrollArea::vertical().show(ui, |ui| {
                for command in &self.filtered_commands {
                    if ui.selectable_label(self.selected_command.as_ref() == Some(command), command).clicked() {
                        self.selected_command = Some(command.clone());
                        self.manpage = get_manpage(command);
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Manpage");
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.manpage.is_empty() {
                    ui.label("Nicht verfügbar");
                } else {
                    ui.label(&self.manpage);
                }
            });
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
    commands.lines().map(|s| s.to_string()).collect()
}

fn get_manpage(command: &str) -> String {
    let output = Command::new("man")
        .arg(command)
        .output()
        .expect("Failed to execute command");
    String::from_utf8_lossy(&output.stdout).to_string()
}