// src/ui/modals/modal_trait.rs
use eframe::egui;

/// Trait that all modals must implement
pub trait Modal {
    fn title(&self) -> &str;
    fn render(&mut self, ui: &mut egui::Ui) -> bool; // Returns true if should close
}
