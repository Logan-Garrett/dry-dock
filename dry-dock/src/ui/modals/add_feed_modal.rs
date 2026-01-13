// src/ui/modals/add_feed_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;
use crate::dal::FeedsRepository;
use crate::ui::styles::Theme;
use crate::services::log_service;

#[derive(Default)]
pub struct AddFeedModal {
    feed_title: String,
    url: String,
}

impl Modal for AddFeedModal {
    fn title(&self) -> &str {
        "Add RSS Feed"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        Theme::apply_body_style(ui);
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        ui.label(egui::RichText::new("Feed Title").size(Theme::FONT_SIZE_BODY).strong());
        ui.add_space(Theme::SPACING_SMALL);
        let title_edit = egui::TextEdit::singleline(&mut self.feed_title)
            .hint_text("e.g., Tech News")
            .desired_width(f32::INFINITY)
            .margin(egui::vec2(8.0, 8.0));
        ui.add(title_edit);
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        ui.label(egui::RichText::new("Feed URL").size(Theme::FONT_SIZE_BODY).strong());
        ui.add_space(Theme::SPACING_SMALL);
        let url_edit = egui::TextEdit::singleline(&mut self.url)
            .hint_text("https://example.com/rss")
            .desired_width(f32::INFINITY)
            .margin(egui::vec2(8.0, 8.0));
        ui.add(url_edit);
        
        ui.add_space(Theme::SPACING_LARGE);
        
        ui.horizontal(|ui| {
            if ui.add(Theme::primary_button("Add Feed")).clicked() {
                if let Err(e) = FeedsRepository::create(&self.url, &self.feed_title) {
                    log_service::add_log_entry("ERROR", &format!("Error adding feed: {}", e));
                }
                should_close = true;
            }
            if ui.add(Theme::button("Cancel")).clicked() {
                should_close = true;
            }
        });

        // Reset fields if closing
        if should_close {
            self.feed_title.clear();
            self.url.clear();
        }
        
        should_close
    }
}
