// src/ui/modals/update_bookmark_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;
use crate::dal::BookmarksRepository;
use crate::ui::styles::Theme;
use crate::services::log_service;

pub struct UpdateBookmarkModal {
    bookmark_id: i32,
    name: String,
    location: String,
    loaded: bool,
}

impl UpdateBookmarkModal {
    pub fn new(bookmark_id: i32) -> Self {
        Self {
            bookmark_id,
            name: String::new(),
            location: String::new(),
            loaded: false,
        }
    }
}

impl Modal for UpdateBookmarkModal {
    fn title(&self) -> &str {
        "Update Bookmark"
    }

    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        // Load bookmark data on first render
        if !self.loaded {
            match BookmarksRepository::get_by_id(self.bookmark_id) {
                Ok((_, name, location, _)) => {
                    self.name = name;
                    self.location = location;
                    self.loaded = true;
                }
                Err(e) => {
                    ui.colored_label(Theme::DANGER_COLOR, format!("Error loading bookmark: {}", e));
                    return true; // Close modal on error
                }
            }
        }

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
            if ui.add(Theme::primary_button("Update Bookmark")).clicked() {
                if let Err(e) = BookmarksRepository::update(self.bookmark_id, &self.name, &self.location) {
                    log_service::add_log_entry("ERROR", &format!("Error updating bookmark: {}", e));
                } 
                should_close = true;
            }
            if ui.add(Theme::button("Cancel")).clicked() {
                should_close = true;
            }
        });

        should_close
    }
}
