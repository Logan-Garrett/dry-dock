// src/ui/modals/manage_feeds_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;
use crate::dal::FeedsRepository;
use crate::ui::styles::Theme;
use crate::services::log_service;

#[derive(Default)]
pub struct ManageFeedsModal {
    feeds: Vec<(i32, String, String)>, // (id, url, title)
    loaded: bool,
}

impl Modal for ManageFeedsModal {
    fn title(&self) -> &str {
        "Manage RSS Feeds"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        // Load feeds on first render
        if !self.loaded {
            match FeedsRepository::get_all() {
                Ok(feeds) => {
                    self.feeds = feeds;
                    self.loaded = true;
                }
                Err(e) => {
                    ui.colored_label(Theme::DANGER_COLOR, format!("Error loading feeds: {}", e));
                    return true;
                }
            }
        }
        
        Theme::apply_body_style(ui);
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        if self.feeds.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(Theme::SPACING_XL);
                ui.label(egui::RichText::new("No feeds subscribed yet")
                    .size(Theme::FONT_SIZE_SUBHEADING)
                    .color(Theme::TEXT_SECONDARY));
                ui.add_space(Theme::SPACING_SMALL);
                ui.label(egui::RichText::new("Add feeds from the Feeds screen")
                    .color(Theme::TEXT_MUTED));
            });
        } else {
            ui.label(egui::RichText::new(format!("{} Feed(s)", self.feeds.len()))
                .size(Theme::FONT_SIZE_BODY)
                .color(Theme::TEXT_SECONDARY));
            ui.add_space(Theme::SPACING_MEDIUM);
        }

        // Track feed to delete
        let mut id_to_delete: Option<i32> = None;
        
        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                for (feed_id, url, title) in &self.feeds {
                    Theme::card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_min_width(ui.available_width() - 140.0);
                                ui.label(egui::RichText::new(title)
                                    .size(Theme::FONT_SIZE_BODY)
                                    .strong()
                                    .color(Theme::TEXT_PRIMARY));
                                ui.add_space(Theme::SPACING_SMALL);
                                ui.label(egui::RichText::new(url)
                                    .size(Theme::FONT_SIZE_SMALL)
                                    .color(Theme::TEXT_MUTED));
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.add(Theme::danger_button("Delete")).clicked() {
                                    id_to_delete = Some(*feed_id);
                                }
                            });
                        });
                    });
                }
            });
        
        // Delete feed after iteration
        if let Some(id) = id_to_delete {
            match FeedsRepository::delete(id) {
                Ok(_) => {
                    log_service::add_log_entry("INFO", "Feed deleted successfully.");
                    self.feeds.retain(|(feed_id, _, _)| *feed_id != id);
                }
                Err(e) => {
                    log_service::add_log_entry("ERROR", &format!("Error deleting feed: {}", e));
                }
            }
        }

        ui.add_space(Theme::SPACING_LARGE);
        
        ui.horizontal(|ui| {
            if ui.add(Theme::button("Close")).clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
}
