// src/ui/modals/modal_factory.rs
use eframe::egui;
use crate::ui::modals::*;

/// Type alias for boxed modal trait objects
type BoxedModal = Box<dyn Modal>;

/// Factory for creating and managing modals dynamically
pub struct ModalFactory {
    /// Currently active modal (if any)
    active_modal: Option<(ActiveModal, BoxedModal)>,
}

impl ModalFactory {
    /// Create a new modal factory
    pub fn new() -> Self {
        Self {
            active_modal: None,
        }
    }

    /// Open a modal by type
    pub fn open_modal(&mut self, modal_type: ActiveModal) {
        if modal_type == ActiveModal::None {
            self.active_modal = None;
            return;
        }

        let modal: BoxedModal = match modal_type {
            ActiveModal::AddFeed => Box::new(AddFeedModal::default()),
            ActiveModal::CreateNote => Box::new(CreateNoteModal::default()),
            ActiveModal::AddBookmark => Box::new(AddBookmarkModal::default()),
            ActiveModal::Settings => Box::new(SettingsModal),
            ActiveModal::None => return,
        };

        self.active_modal = Some((modal_type.clone(), modal));
    }

    /// Close the currently active modal
    pub fn close_modal(&mut self) {
        self.active_modal = None;
    }

    /// Render the active modal (if any)
    pub fn render(&mut self, ctx: &egui::Context) {
        if let Some((modal_type, modal)) = &mut self.active_modal {
            // Dim background
            egui::Area::new("modal_overlay".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    let screen_rect = ctx.content_rect();
                    ui.painter().rect_filled(
                        screen_rect,
                        0.0,
                        egui::Color32::from_black_alpha(200),
                    );
                });

            // Render the modal window
            let title = modal.title().to_string();
            let mut should_close = false;

            let (default_size, resizable) = match modal_type {
                ActiveModal::CreateNote => ([800.0, 600.0], true),
                ActiveModal::Settings => ([600.0, 400.0], true),
                _ => ([400.0, 300.0], false),
            };

            egui::Window::new(&title)
                .collapsible(false)
                .resizable(resizable)
                .default_size(default_size)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    should_close = modal.render(ui);
                });

            if should_close {
                self.close_modal();
            }
        }
    }
}

impl Default for ModalFactory {
    fn default() -> Self {
        Self::new()
    }
}
