// src/ui/modals/create_note_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;
use crate::services::NoteService;
use crate::ui::styles::Theme;

#[derive(Default)]
pub struct CreateNoteModal {
    title: String,
    details: String,
}

impl Modal for CreateNoteModal {
    fn title(&self) -> &str {
        "Create New Note"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
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

        // Note Details.
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
            if ui.add(Theme::primary_button("Create Note")).clicked() {
                if let Err(e) = NoteService::create_note(&self.title, &self.details) {
                    println!("Error creating note: {}", e);
                }
                should_close = true;
            }
            if ui.add(Theme::button("Cancel")).clicked() {
                should_close = true;
            }
        });

        // Clean up fields if closing
        if should_close {
            self.title.clear();
            self.details.clear();
        }
        
        should_close
    }
}
