// src/ui/modals/update_note_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;
use crate::services::NoteService;
use crate::services::log_service;
use crate::ui::styles::Theme;

pub struct UpdateNoteModal {
    note_id: i32,
    title: String,
    details: String,
    loaded: bool,
}

impl UpdateNoteModal {
    pub fn new(note_id: i32) -> Self {
        Self {
            note_id,
            title: String::new(),
            details: String::new(),
            loaded: false,
        }
    }
}

impl Modal for UpdateNoteModal {
    fn title(&self) -> &str {
        "Update Note"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        // Load note data on first render
        if !self.loaded {
            match NoteService::get_note_by_id(self.note_id) {
                Ok(note) => {
                    self.title = note.title;
                    self.details = note.details;
                    self.loaded = true;
                }
                Err(e) => {
                    ui.colored_label(Theme::DANGER_COLOR, format!("Error loading note: {}", e));
                    log_service::add_log_entry("ERROR", &format!("Error loading note: {}", e));
                    return true; // Close modal on error
                }
            }
        }
        
        Theme::apply_body_style(ui);
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        ui.label(egui::RichText::new("Note Title").size(Theme::FONT_SIZE_BODY).strong());
        ui.add_space(Theme::SPACING_SMALL);
        let title_edit = egui::TextEdit::singleline(&mut self.title)
            .hint_text("Enter a title for your note")
            .desired_width(f32::INFINITY)
            .margin(egui::vec2(8.0, 8.0));
        ui.add(title_edit);
        
        ui.add_space(Theme::SPACING_MEDIUM);

        // Note Details
        ui.label(egui::RichText::new("Note Details").size(Theme::FONT_SIZE_BODY).strong());
        ui.add_space(Theme::SPACING_SMALL);
        egui::ScrollArea::vertical()
            .id_salt("note_details_scroll")
            .max_height(ui.available_height() - 80.0) 
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.details)
                        .hint_text("Write your note here...")
                        .desired_width(f32::INFINITY)
                        .desired_rows(20)
                        .font(egui::TextStyle::Body)
                );
            });
        
        ui.add_space(Theme::SPACING_LARGE);
        
        ui.horizontal(|ui| {
            if ui.add(Theme::primary_button("Update Note")).clicked() {
                if let Err(e) = NoteService::update_note(self.note_id, &self.title, &self.details) {
                    println!("Error updating note: {}", e);
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
