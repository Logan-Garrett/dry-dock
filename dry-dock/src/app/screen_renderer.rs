// src/app/screen_renderer.rs
use eframe::egui;
use crate::app::AppState;

impl AppState {
    pub fn render_active_screen(&mut self, ctx: &egui::Context) {
        let mut modal_opener = |modal_type| {
            self.modal_factory.open_modal(modal_type);
        };
        
        if let Ok(mut factory) = self.screen_factory.lock() {
            factory.render(ctx, &mut modal_opener);
        }
    }
}
