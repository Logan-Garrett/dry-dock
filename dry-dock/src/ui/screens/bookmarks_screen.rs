// src/ui/screens/bookmarks_screen.rs
use eframe::egui;
use crate::services::bookmark_service;
use crate::models::Bookmark;
use crate::ui::modals::ActiveModal;
use crate::ui::styles::Theme;

#[derive(Default)]
pub struct BookmarksScreen {
    bookmarks: Vec<Bookmark>,
}

impl BookmarksScreen {
    pub fn title(&self) -> &str {
        "Bookmarks Manager"
    }

    pub fn render(&mut self, ui: &mut egui::Ui, modal_opener: &mut dyn FnMut(ActiveModal)) {
        Theme::apply_body_style(ui);
        
        // Header
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.heading(egui::RichText::new(self.title()).strong());
        ui.add_space(Theme::SPACING_LARGE);

        // Action buttons row
        ui.horizontal(|ui| {
            // Create Add Bookmark Button
            if ui.add(Theme::primary_button("Add Bookmark")).clicked() {
                modal_opener(ActiveModal::AddBookmark);
                self.bookmarks.clear();
            }

            // Create Refresh Button
            if ui.add(Theme::button("Refresh")).clicked() {
                self.bookmarks.clear();
            }
        });

        ui.add_space(Theme::SPACING_MEDIUM);
        ui.separator();
        ui.add_space(Theme::SPACING_MEDIUM);
        
        // Load bookmarks if empty
        if self.bookmarks.is_empty() {
            match bookmark_service::fetch_all_bookmarks() {
                Ok(bookmarks) => {
                    self.bookmarks = bookmarks.into_iter()
                        .map(|(id, name, path, created_at)| {
                            Bookmark::new(id, name, path, created_at)
                        })
                        .collect();
                }
                Err(e) => {
                    ui.colored_label(Theme::DANGER_COLOR, format!("Error loading bookmarks: {}", e));
                    return;
                }
            }
        }

        // Show empty state
        if self.bookmarks.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(Theme::SPACING_XL);
                ui.label(egui::RichText::new("Bookmarks Manager").size(48.0));
                ui.add_space(Theme::SPACING_SMALL);
                ui.label(egui::RichText::new("No bookmarks yet").size(Theme::FONT_SIZE_SUBHEADING).color(Theme::TEXT_PRIMARY));
                ui.add_space(Theme::SPACING_SMALL);
                ui.label(egui::RichText::new("Click 'Add Bookmark' to create your first bookmark").color(Theme::TEXT_SECONDARY));
            });
            return;
        }

        // Track bookmark to delete
        let mut id_to_delete: Option<i32> = None;

        // Display bookmarks in cards
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                for bookmark in &self.bookmarks {
                    Theme::card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_min_width(ui.available_width() - 280.0);
                                ui.label(egui::RichText::new(&bookmark.name)
                                    .size(Theme::FONT_SIZE_SUBHEADING)
                                    .strong()
                                    .color(Theme::TEXT_PRIMARY));
                                ui.add_space(Theme::SPACING_SMALL);
                                ui.label(egui::RichText::new(format!("Path: {}", &bookmark.path))
                                    .size(Theme::FONT_SIZE_SMALL)
                                    .color(Theme::TEXT_SECONDARY));
                                ui.add_space(Theme::SPACING_SMALL);
                                ui.label(egui::RichText::new(format!("Created: {}", &bookmark.created_at))
                                    .size(Theme::FONT_SIZE_SMALL)
                                    .color(Theme::TEXT_MUTED));
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Delete bookmark button
                                if ui.add(Theme::danger_button("Delete")).clicked() {
                                    id_to_delete = Some(bookmark.id);
                                }
                                
                                ui.add_space(Theme::SPACING_SMALL);
                                
                                // Open bookmark button
                                if ui.add(Theme::success_button("Open")).clicked() {
                                    bookmark_service::open_bookmark_path(&bookmark.path);
                                }
                            });
                        });
                    });
                }
            });

        // Delete bookmark after iteration
        if let Some(id) = id_to_delete {
            match bookmark_service::delete_bookmark(id) {
                Ok(_) => {
                    println!("Bookmark deleted successfully.");
                    self.bookmarks.retain(|bm| bm.id != id);
                    self.bookmarks.clear();
                }
                Err(e) => {
                    println!("Error deleting bookmark: {}", e);
                }
            }
        }
    }
}
