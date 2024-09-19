mod app;
mod cli;
mod manpage;

use eframe::NativeOptions;

fn main() {
    env_logger::init();
    let options = NativeOptions::default();
    eframe::run_native(
        "CLI Organizer",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::new()))),

    ).unwrap();
}
