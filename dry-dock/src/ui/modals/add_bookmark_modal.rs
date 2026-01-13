// src/ui/modals/add_bookmark_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;
use crate::services::bookmark_service;
use crate::services::log_service;
use crate::ui::styles::Theme;

#[derive(Default)]
pub struct AddBookmarkModal {
    name: String,
    location: String,
}

impl Modal for AddBookmarkModal {
    fn title(&self) -> &str {
        "Add New Bookmark"
    }

    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;

        Theme::apply_body_style(ui);
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        ui.label(egui::RichText::new("Bookmark Name").size(Theme::FONT_SIZE_BODY).strong());
        ui.add_space(Theme::SPACING_SMALL);
        let name_edit = egui::TextEdit::singleline(&mut self.name)
            .hint_text("Enter a descriptive name")
            .desired_width(f32::INFINITY)
            .margin(egui::vec2(8.0, 8.0));
        ui.add(name_edit);
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        ui.label(egui::RichText::new("Location").size(Theme::FONT_SIZE_BODY).strong());
        ui.add_space(Theme::SPACING_SMALL);
        let location_edit = egui::TextEdit::singleline(&mut self.location)
            .hint_text("URL or file path")
            .desired_width(f32::INFINITY)
            .margin(egui::vec2(8.0, 8.0));
        ui.add(location_edit);
        
        ui.add_space(Theme::SPACING_LARGE);

        ui.horizontal(|ui| {
            if ui.add(Theme::primary_button("Add Bookmark")).clicked() {
                if let Err(e) = bookmark_service::add_new_bookmark(&self.name, &self.location) {
                    log_service::add_log_entry("ERROR", &format!("Error adding bookmark: {}", e));
                }
                should_close = true;
            }
            if ui.add(Theme::button("Cancel")).clicked() {
                should_close = true;
            }
        });

        // Reset fields if closing
        if should_close {
            self.name.clear();
            self.location.clear();
        }

        should_close
    }
}
