// src/ui/screens/notes_screen.rs
use eframe::egui;
use crate::ui::screens::screen_trait::Screen;
use crate::services::NoteService;
use crate::models::Note;
use crate::ui::modals::ActiveModal;
use crate::ui::styles::Theme;

#[derive(Default)]
pub struct NotesScreen {
    notes: Vec<Note>,
    search_query: String,
}

impl Screen for NotesScreen {
    fn title(&self) -> &str {
        "Notes"
    }
}

impl NotesScreen {
    pub fn render(&mut self, ui: &mut egui::Ui, modal_opener: &mut dyn FnMut(ActiveModal)) {
        Theme::apply_body_style(ui);
        
        // Header
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.heading(egui::RichText::new(self.title()).strong());
        ui.add_space(Theme::SPACING_LARGE);

        // Action buttons row
        ui.horizontal(|ui| {
            if ui.add(Theme::primary_button("Create Note")).clicked() {
                modal_opener(ActiveModal::CreateNote);
            }

            if ui.add(Theme::button("Refresh")).clicked() {
                self.search_query.clear();
                match NoteService::get_all_notes() {
                    Ok(notes) => {
                        self.notes = notes;
                    }
                    Err(e) => {
                        println!("Error refreshing notes: {}", e);
                    }
                }
            }
        });
        
        ui.add_space(Theme::SPACING_MEDIUM);

        // Search field
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Search:").color(Theme::TEXT_PRIMARY));
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search by title or content...")
                    .desired_width(ui.available_width())
            );

            // Trigger search when text changes
            if response.changed() {
                if self.search_query.trim().is_empty() {
                    // Show all notes when search is empty
                    match NoteService::get_all_notes() {
                        Ok(notes) => {
                            self.notes = notes;
                        }
                        Err(e) => {
                            println!("Error loading notes: {}", e);
                        }
                    }
                } else {
                    // Search notes
                    match NoteService::search_notes(&self.search_query) {
                        Ok(notes) => {
                            self.notes = notes;
                        }
                        Err(e) => {
                            println!("Error searching notes: {}", e);
                        }
                    }
                }
            }
        });
        
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.separator();
        ui.add_space(Theme::SPACING_MEDIUM);

        // Load notes if empty (but not during an active search)
        if self.notes.is_empty() && self.search_query.trim().is_empty() {
            match NoteService::get_all_notes() {
                Ok(notes) => {
                    self.notes = notes;
                }
                Err(e) => {
                    ui.colored_label(Theme::DANGER_COLOR, format!("Error loading notes: {}", e));
                    return;
                }
            }
        }

        // Show empty state
        if self.notes.is_empty() {
            ui.vertical_centered(|ui| {
                ui.add_space(Theme::SPACING_XL);
                if !self.search_query.trim().is_empty() {
                    // Search returned no results
                    ui.label(egui::RichText::new("🔍").size(48.0));
                    ui.add_space(Theme::SPACING_SMALL);
                    ui.label(egui::RichText::new("No results found").size(Theme::FONT_SIZE_SUBHEADING).color(Theme::TEXT_PRIMARY));
                    ui.add_space(Theme::SPACING_SMALL);
                    ui.label(egui::RichText::new(format!("No notes match \"{}\"", self.search_query)).color(Theme::TEXT_SECONDARY));
                } else {
                    // No notes exist at all
                    ui.label(egui::RichText::new("Notes").size(48.0));
                    ui.add_space(Theme::SPACING_SMALL);
                    ui.label(egui::RichText::new("No notes yet").size(Theme::FONT_SIZE_SUBHEADING).color(Theme::TEXT_PRIMARY));
                    ui.add_space(Theme::SPACING_SMALL);
                    ui.label(egui::RichText::new("Click 'Create Note' to write your first note").color(Theme::TEXT_SECONDARY));
                }
            });
            return;
        }

        // Track note to delete
        let mut id_to_delete: Option<i32> = None;
        
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                for note in &self.notes {
                    Theme::card_frame().show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.set_min_width(ui.available_width() - 160.0);
                                ui.label(egui::RichText::new(&note.title)
                                    .size(Theme::FONT_SIZE_SUBHEADING)
                                    .strong()
                                    .color(Theme::TEXT_PRIMARY));
                                ui.add_space(Theme::SPACING_SMALL);
                                
                                // Truncate note details to 150 characters maybe more or less 
                                // down the road but this is fine for now....
                                let preview = if note.details.len() > 150 {
                                    format!("{}...", &note.details[..150])
                                } else {
                                    note.details.clone()
                                };
                                
                                ui.label(egui::RichText::new(&preview)
                                    .size(Theme::FONT_SIZE_BODY)
                                    .color(Theme::TEXT_SECONDARY));
                                ui.add_space(Theme::SPACING_SMALL);
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new(format!("Created: {}", note.created_at))
                                        .size(Theme::FONT_SIZE_SMALL)
                                        .color(Theme::TEXT_MUTED));
                                    if let Some(updated) = &note.updated_at {
                                        ui.label(egui::RichText::new(format!(" | Updated: {}", updated))
                                            .size(Theme::FONT_SIZE_SMALL)
                                            .color(Theme::TEXT_MUTED));
                                    }
                                });
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                // Delete note button
                                if ui.add(Theme::danger_button("Delete")).clicked() {
                                    id_to_delete = Some(note.id);
                                }
                                
                                ui.add_space(Theme::SPACING_SMALL);
                                
                                // Update note button
                                if ui.add(Theme::primary_button("Update")).clicked() {
                                    modal_opener(ActiveModal::UpdateNote(note.id));
                                }

                                ui.add_space(Theme::SPACING_SMALL);

                                // View note button (renders markdown)
                                if ui.add(Theme::success_button("View")).clicked() {
                                    modal_opener(ActiveModal::ViewNote(note.id));
                                }

                            });
                        });
                    });
                }
            });
        
        // Delete note after iteration
        if let Some(id) = id_to_delete {
            match NoteService::delete_note(id) {
                Ok(_) => {
                    println!("Note deleted successfully.");
                    self.notes.retain(|note| note.id != id);
                    self.notes.clear();
                }
                Err(e) => {
                    println!("Error deleting note: {}", e);
                }
            }
        }
    }
}
