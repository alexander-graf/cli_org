mod app;
mod cli;
mod manpage;

fn main() {
    env_logger::init();
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CLI Organizer",
        options,
        Box::new(|_cc| Ok(Box::new(app::MyApp::new()))),
    );
}