// src/app/modal_renderer.rs
use eframe::egui;
use crate::app::AppState;

impl AppState {
    /// Render the active modal dynamically using the modal factory
    pub fn render_active_modal(&mut self, ctx: &egui::Context) {
        self.modal_factory.render(ctx);
    }
}
