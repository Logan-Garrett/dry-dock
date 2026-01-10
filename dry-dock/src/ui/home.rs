// src/ui/home.rs
use eframe::egui;
use crate::models::Config;
use crate::ui::styles::Theme;

/// Renders the home/welcome screen
pub fn render_home(ui: &mut egui::Ui, config: &Config) {
    Theme::apply_body_style(ui);

    ui.vertical_centered(|ui| {
        ui.add_space(Theme::SPACING_XL * 3.0);
        
        // App icon/emoji
        ui.label(egui::RichText::new("⚓").size(96.0));
        ui.add_space(Theme::SPACING_LARGE);
        
        // Welcome message
        ui.heading(egui::RichText::new(format!("Welcome to {}", config.app_name))
            .size(Theme::FONT_SIZE_HEADING * 1.2)
            .strong()
            .color(Theme::TEXT_PRIMARY));
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        ui.label(egui::RichText::new(format!("Version {}", config.version))
            .size(Theme::FONT_SIZE_SMALL)
            .color(Theme::TEXT_MUTED));
        
        ui.add_space(Theme::SPACING_XL);
        
        ui.label(egui::RichText::new("Use the menu above to get started")
            .size(Theme::FONT_SIZE_BODY)
            .color(Theme::TEXT_SECONDARY));
    });
}
