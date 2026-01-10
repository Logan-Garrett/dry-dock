// src/ui/modals/settings_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;

#[derive(Default)]
pub struct SettingsModal;

impl Modal for SettingsModal {
    fn title(&self) -> &str {
        "Settings"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        ui.label("Settings modal is under construction.");
        
        if ui.button("Close").clicked() {
            should_close = true;
        }
        
        should_close
    }
}
