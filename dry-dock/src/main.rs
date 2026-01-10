// src/main.rs
use eframe::egui;

mod models;
mod dal;
mod common;
mod services;
mod ui;
mod app;

use models::Config;
use app::AppState;
use common::helper::*;
use dal::db_context::initialize_database;
use ui::{menu, home};

const AUTO_SAVE_INTERVAL_SECS: u64 = 60;

struct DryDockApp {
    state: AppState,
}

impl DryDockApp {
    fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        // Down the road the Settings modal can allow switching between light/dark themes and other options

        // Set dark theme
        let mut style = (*cc.egui_ctx.style()).clone();
        style.visuals = egui::Visuals::dark();
        style.visuals.window_fill = ui::styles::Theme::BG_DARK;
        style.visuals.panel_fill = ui::styles::Theme::BG_DARK;
        style.visuals.extreme_bg_color = ui::styles::Theme::BG_DARKER;
        style.visuals.faint_bg_color = ui::styles::Theme::CARD_BG;
        style.visuals.window_stroke = egui::Stroke::new(1.0, ui::styles::Theme::BORDER_COLOR);
        cc.egui_ctx.set_style(style);
        
        Self { 
            state: AppState::new(config, cc.egui_ctx.clone()),
        }
    }
}

impl eframe::App for DryDockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Menu Bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                let config_clone = self.state.config.clone();
                menu::render_menu(ui, &config_clone, &mut self.state);
            });
        });

        // Central Panel - show home if no active screen
        if self.state.get_active_screen() == app::ActiveScreen::None {
            egui::CentralPanel::default().show(ctx, |ui| {
                home::render_home(ui, &self.state.config);
            });
        }

        // Bottom Panel
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(format!("Version: {}", self.state.config.version));
                ui.separator();
                ui.label(format!("© {} Dry Dock. All rights reserved.", load_current_year()));
            });
        });

        // Render active screen (if any)
        self.state.render_active_screen(ctx);

        // Render active modal (if any)
        self.state.render_active_modal(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Cleanup if needed
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save state if needed
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(AUTO_SAVE_INTERVAL_SECS)
    }
}

fn main() -> Result<(), eframe::Error> {
    // Load config file from multiple possible locations
    let config_data = load_config_file()
        .expect("Unable to read config file from any location");

    // Parse config file
    let mut config: Config = serde_json::from_str(&config_data)
        .expect("Unable to parse config file");

    // Update icon path to point to bundled resource if needed
    config.icon_path = load_icon_path(&config.icon_path);

    // Initialize database
    let db_path = get_database_path("DryDock");
    initialize_database(&db_path)
        .expect("Failed to initialize database");

    // Load application icon
    let icon_image = image::open(&config.icon_path)
        .expect("Failed to load application icon")
        .to_rgba8();
    
    let (icon_width, icon_height) = icon_image.dimensions();
    let icon = eframe::egui::IconData {
        rgba: icon_image.into_raw(),
        width: icon_width,
        height: icon_height,
    };

    // Configure the window
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_icon(icon)
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_fullscreen(true),
        vsync: config.is_vsync_enabled,
        persist_window: true,
        ..Default::default()
    };
    
    // Run the application
    eframe::run_native(
        &format!("{} v{}", config.app_name, config.version),
        options,
        Box::new(|cc| Ok(Box::new(DryDockApp::new(cc, config.clone())))),
    )
}