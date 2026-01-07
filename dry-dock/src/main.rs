use chrono::Datelike;
use eframe::egui;
use serde::{Deserialize, Serialize};

const AUTO_SAVE_INTERVAL_SECS: u64 = 60;

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Config {
    app_name: String,
    version: String,
    is_vsync_enabled: bool,
}

struct MyApp {
    // Load App Config
    config: Config,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self { config }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {        
        // Lets Load the Menu Bar
        egui::TopBottomPanel::top("menu_bar")
        .show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // Load Menu
                load_menu(ui, &self.config);
            });
        });

        // Central Panel
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            // Load Central Panel
            load_central_panel(ui, &self.config);
        });

        // Bottom Panel
        egui::TopBottomPanel::bottom("bottom_panel")
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // Version Info
                ui.label(format!("Version: {}", self.config.version));
                // Separator
                ui.separator();
                // Copyright Info
                ui.label(format!("© {} Dry Dock. All rights reserved.", load_current_year()));
            });
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
                    MyApp::new(cc, config.clone())
                )
            )
        ),
    )
}


// Helpers.
fn load_menu(ui: &mut egui::Ui, config: &Config) {
    // Load Menu Styling
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Button, 
      egui::FontId::new(24.0, 
     egui::FontFamily::Proportional)
    );

    // Load Menu Padding For Buttom
    ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);

    // Load Menu Button
    ui.menu_button(config.app_name.as_str(), |ui| {
        // Create Exit Button
        let exit_button = egui::Button::new("Exit")
        .min_size(egui::vec2(100.0, 30.0));
        
        // Load Exit Button
        if ui.add(exit_button)
        .clicked() {
            std::process::exit(0);
        }
    });
}

fn load_central_panel(ui: &mut egui::Ui, config: &Config) {
    // Load Central Panel Styling
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Heading, 
      egui::FontId::new(24.0, 
     egui::FontFamily::Proportional)
    );

    // Load Central Panel Spacing
    ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

    // Body Content
    if ui.button("Click me")
    .clicked() {
        println!("App Name: {}", config.app_name);
        println!("Button clicked!");
    }
}

fn load_current_year() -> String {
    let current_year = chrono::Utc::now().year();
    current_year.to_string()
}