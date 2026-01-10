// src/ui/screens/feeds_screen.rs
use eframe::egui;
use crate::ui::screens::screen_trait::Screen;
use crate::dal::FeedItemsRepository;
use crate::services::rss_service;
use crate::models::FeedItem;
use crate::ui::modals::ActiveModal;
use crate::ui::styles::Theme;

#[derive(Default)]
pub struct FeedsScreen {
    feed_items: Vec<FeedItem>,
}

impl Screen for FeedsScreen {
    fn title(&self) -> &str {
        "RSS Feed Items"
    }
}

impl FeedsScreen {
    pub fn render(&mut self, ui: &mut egui::Ui, modal_opener: &mut dyn FnMut(ActiveModal)) {
        Theme::apply_body_style(ui);
        
        // Header
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.heading(egui::RichText::new(self.title()).strong());
        ui.add_space(Theme::SPACING_LARGE);
        
        // Action buttons row
        ui.horizontal(|ui| {
            if ui.add(Theme::primary_button("Add RSS Feed")).clicked() {
                modal_opener(ActiveModal::AddFeed);
            }

            if ui.add(Theme::success_button("Refresh All")).clicked() {
                match rss_service::refresh_all_feeds() {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => println!("Error refreshing feeds: {}", e),
                }
                self.feed_items.clear();
            }
        });
        
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.separator();
        ui.add_space(Theme::SPACING_MEDIUM);
        
        // Load feed items if empty
        if self.feed_items.is_empty() {
            match FeedItemsRepository::get_latest(10000) {
                Ok(items) => {
                    self.feed_items = items.into_iter()
                        .map(|(id, title, link, description, pub_date)| {
                            FeedItem::new(id, title, link, description, pub_date)
                        })
                        .collect();
                }
                Err(e) => {
                    ui.colored_label(Theme::DANGER_COLOR, format!("Error loading feed items: {}", e));
                    return;
                }
            }
        }

        // Show empty state
        if self.feed_items.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(Theme::SPACING_XL);
                ui.label(egui::RichText::new("RSS Feed Items").size(48.0));
                ui.add_space(Theme::SPACING_SMALL);
                ui.label(egui::RichText::new("No RSS feeds yet").size(Theme::FONT_SIZE_SUBHEADING).color(Theme::TEXT_PRIMARY));
                ui.add_space(Theme::SPACING_SMALL);
                ui.label(egui::RichText::new("Click 'Add RSS Feed' to subscribe to your first feed").color(Theme::TEXT_SECONDARY));
            });
            return;
        }

        // Display feed items
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                for item in &self.feed_items {
                    Theme::card_frame().show(ui, |ui| {
                        ui.vertical(|ui| {
                            // Title as clickable link
                            ui.hyperlink_to(
                                egui::RichText::new(&item.title)
                                    .size(Theme::FONT_SIZE_SUBHEADING)
                                    .strong()
                                    .color(Theme::PRIMARY_COLOR),
                                &item.link
                            );
                            
                            ui.add_space(Theme::SPACING_SMALL);
                            
                            // Format date
                            let datetime = chrono::DateTime::from_timestamp(item.pub_date, 0);
                            if let Some(dt) = datetime {
                                ui.label(egui::RichText::new(format!("Date: {}", dt.format("%B %d, %Y at %H:%M")))
                                    .size(Theme::FONT_SIZE_SMALL)
                                    .color(Theme::TEXT_MUTED));
                            }
                            
                            ui.add_space(Theme::SPACING_SMALL);
                            
                            // Show truncated description
                            let desc = if item.description.len() > 300 {
                                format!("{}...", &item.description[..300])
                            } else {
                                item.description.clone()
                            };
                            ui.label(egui::RichText::new(desc)
                                .size(Theme::FONT_SIZE_BODY)
                                .color(Theme::TEXT_SECONDARY));
                        });
                    });
                }
            });
    }
}
