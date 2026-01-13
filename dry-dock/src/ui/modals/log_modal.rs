// src/ui/modals/log_modal.rs
use eframe::egui;
use crate::ui::modals::modal_trait::Modal;

// Pull log_service
use crate::services::log_service;

#[derive(Default)]
pub struct LogModal {
    search_query: String,
}

impl Modal for LogModal {
    fn title(&self) -> &str {
        "Logs"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;

        // Search Box
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            ui.label("Search Logs:");
            ui.text_edit_singleline(&mut self.search_query);
        });

        // Load and display logs (filtered by search if query exists)
        ui.add_space(10.0);
        let logs = if self.search_query.trim().is_empty() {
            log_service::get_all_logs()
        } else {
            log_service::search_logs(&self.search_query)
        };
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (id, level, message, timestamp) in logs {
                ui.horizontal(|ui| {
                    ui.label(format!("Id: [{}] - [{}] [{}] - {}", id, timestamp, level, message));
                });
            }
        });
        
        if ui.button("Close").clicked() {
            should_close = true;
        }
        
        should_close
    }
}