// src/ui/styles/theme.rs
use eframe::egui;

/// Theme constants for consistent styling across the application
pub struct Theme;

impl Theme {
    // Font sizes
    pub const FONT_SIZE_HEADING: f32 = 28.0;
    pub const FONT_SIZE_SUBHEADING: f32 = 20.0;
    pub const FONT_SIZE_BUTTON: f32 = 16.0;
    pub const FONT_SIZE_BODY: f32 = 15.0;
    pub const FONT_SIZE_SMALL: f32 = 13.0;
    
    // Spacing
    pub const SPACING_SMALL: f32 = 6.0;
    pub const SPACING_MEDIUM: f32 = 12.0;
    pub const SPACING_LARGE: f32 = 24.0;
    pub const SPACING_XL: f32 = 36.0;
    
    // Button sizes
    pub const BUTTON_HEIGHT: f32 = 36.0;
    pub const BUTTON_MIN_WIDTH: f32 = 120.0;
    pub const BUTTON_PADDING: egui::Vec2 = egui::vec2(16.0, 8.0);
    
    // Dark Theme Colors
    pub const BG_DARK: egui::Color32 = egui::Color32::from_rgb(18, 18, 20); // Main background
    pub const BG_DARKER: egui::Color32 = egui::Color32::from_rgb(12, 12, 14); // Deeper background
    pub const CARD_BG: egui::Color32 = egui::Color32::from_rgb(28, 28, 32); // Card background
    pub const BORDER_COLOR: egui::Color32 = egui::Color32::from_rgb(45, 45, 50); // Borders
    
    pub const TEXT_PRIMARY: egui::Color32 = egui::Color32::from_rgb(240, 240, 245); // Main text
    pub const TEXT_SECONDARY: egui::Color32 = egui::Color32::from_rgb(160, 160, 170); // Secondary text
    pub const TEXT_MUTED: egui::Color32 = egui::Color32::from_rgb(115, 115, 125); // Muted text
    
    // Accent colors - brighter for dark theme
    pub const PRIMARY_COLOR: egui::Color32 = egui::Color32::from_rgb(96, 165, 250); // Bright Blue
    pub const SUCCESS_COLOR: egui::Color32 = egui::Color32::from_rgb(74, 222, 128); // Bright Green
    pub const DANGER_COLOR: egui::Color32 = egui::Color32::from_rgb(248, 113, 113); // Bright Red
    
    /// Apply menu bar styling
    pub fn apply_menu_style(ui: &mut egui::Ui) {
        ui.style_mut().text_styles.insert(
            egui::TextStyle::Button, 
            egui::FontId::new(Self::FONT_SIZE_BUTTON, egui::FontFamily::Proportional)
        );
        ui.spacing_mut().button_padding = Self::BUTTON_PADDING;
    }
    
    /// Apply body text styling
    pub fn apply_body_style(ui: &mut egui::Ui) {
        ui.style_mut().text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(Self::FONT_SIZE_BODY, egui::FontFamily::Proportional)
        );
        ui.style_mut().text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(Self::FONT_SIZE_HEADING, egui::FontFamily::Proportional)
        );
        ui.spacing_mut().item_spacing = egui::vec2(Self::SPACING_MEDIUM, Self::SPACING_MEDIUM);
        ui.spacing_mut().button_padding = Self::BUTTON_PADDING;
    }
    
    /// Create a styled primary button
    pub fn primary_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).color(egui::Color32::from_rgb(10, 10, 15)))
            .min_size(egui::vec2(Self::BUTTON_MIN_WIDTH, Self::BUTTON_HEIGHT))
            .fill(Self::PRIMARY_COLOR)

    }
    
    /// Create a styled button with consistent dimensions
    pub fn button(text: &str) -> egui::Button<'_> {
        egui::Button::new(text)
            .min_size(egui::vec2(Self::BUTTON_MIN_WIDTH, Self::BUTTON_HEIGHT))

    }
    
    /// Create a styled danger button (red)
    pub fn danger_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).color(egui::Color32::from_rgb(10, 10, 15)))
            .min_size(egui::vec2(Self::BUTTON_MIN_WIDTH, Self::BUTTON_HEIGHT))
            .fill(Self::DANGER_COLOR)

    }
    
    /// Create a styled success button (green)
    pub fn success_button(text: &str) -> egui::Button<'_> {
        egui::Button::new(egui::RichText::new(text).color(egui::Color32::from_rgb(10, 10, 15)))
            .min_size(egui::vec2(Self::BUTTON_MIN_WIDTH, Self::BUTTON_HEIGHT))
            .fill(Self::SUCCESS_COLOR)

    }
    
    /// Create a card-style frame for dark theme
    pub fn card_frame() -> egui::Frame {
        egui::Frame::new()
            .fill(Self::CARD_BG)
            .inner_margin(12.0)
            .outer_margin(egui::vec2(0.0, 6.0))
            .stroke(egui::Stroke::new(1.0, Self::BORDER_COLOR))
    }
}
