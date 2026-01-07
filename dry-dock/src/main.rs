use eframe::egui;
use serde::{Deserialize, Serialize};

const AUTO_SAVE_INTERVAL_SECS: u64 = 60;

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Config {
    app_name: String,
    version: String,
    is_vsync_enabled: bool,
}

struct MyApp;

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            ui.heading("Welcome to Dry Dock!");
        });

        egui::CentralPanel::default()
        .show(ctx, |ui| {
            // Seperator
            ui.separator();

            if ui.button("Click me")
            .clicked() {
                println!("Button clicked!");
            }

            if ui.button("Exit Dry Dock")
            .clicked() {
                // Exit the Process
                std::process::exit(0);
            }
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Nothing to clean up as of now.
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Nothing to save as of now.
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        // No auto-save interval set.
        std::time::Duration::from_secs(AUTO_SAVE_INTERVAL_SECS)
    }
}

fn main() -> Result<(), eframe::Error> {
    // Load Config File.
    let config_data = std::fs::read_to_string("AppConfig.json")
        .expect("Unable to read config file");

    // Parse Config File.
    let config: Config = serde_json::from_str(&config_data)
        .expect("Unable to parse config file");

    // Configures The Window.
    let options =  eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_fullscreen(true),

        // Enable Vysnc If Configured.
        vsync: config.is_vsync_enabled,

        // Persist Window
        persist_window: true,
        
        ..Default::default()
    };

    // Enable Vysnc If Configured.
    
    // Runs The Egui Application.
    eframe::run_native(
        &format!("{} v{}", config.app_name, config.version),
        options,
        Box::new(|cc| 
            Ok(
                Box::new(
                    MyApp::new(cc)
                )
            )
        ),
    )
}
