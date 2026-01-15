// src/ui/modals/modal_factory.rs
use eframe::egui;
use crate::ui::modals::*;
use std::sync::{Arc, Mutex};
use crate::ui::screens::ScreenFactory;
use crate::app::ActiveScreen;

/// Type alias for boxed modal trait objects
type BoxedModal = Box<dyn Modal>;

/// Factory for creating and managing modals dynamically
pub struct ModalFactory {
    /// Currently active modal (if any)
    active_modal: Option<(ActiveModal, BoxedModal)>,
    /// Reference to screen factory for triggering reloads
    screen_factory: Option<Arc<Mutex<ScreenFactory>>>,
}

impl ModalFactory {
    /// Create a new modal factory
    pub fn new() -> Self {
        Self {
            active_modal: None,
            screen_factory: None,
        }
    }

    /// Set the screen factory reference for triggering reloads
    pub fn set_screen_factory(&mut self, screen_factory: Arc<Mutex<ScreenFactory>>) {
        self.screen_factory = Some(screen_factory);
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
            ActiveModal::ManageFeeds => Box::new(ManageFeedsModal::default()),
            ActiveModal::UpdateBookmark(id) => Box::new(UpdateBookmarkModal::new(id)),
            ActiveModal::UpdateNote(id) => Box::new(UpdateNoteModal::new(id)),
            ActiveModal::ViewNote(id) => Box::new(ViewNoteModal::new(id)),
            ActiveModal::LogModal => Box::new(LogModal::default()),
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
            // Dim background - use screen_rect for full coverage
            egui::Area::new("modal_overlay".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .order(egui::Order::Middle)
                .interactable(true)
                .show(ctx, |ui| {
                    let screen_rect = ctx.content_rect();
                    ui.allocate_space(screen_rect.size());
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
                ActiveModal::UpdateNote(_) => ([800.0, 600.0], true),
                ActiveModal::ViewNote(_) => ([900.0, 700.0], true),
                ActiveModal::ManageFeeds => ([600.0, 500.0], true),
                ActiveModal::LogModal => ([900.0, 900.0], true),
                ActiveModal::Settings => ([600.0, 400.0], true),
                _ => ([400.0, 300.0], false),
            };

            egui::Window::new(&title)
                .collapsible(false)
                .resizable(resizable)
                .default_size(default_size)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .order(egui::Order::Foreground)
                .show(ctx, |ui| {
                    should_close = modal.render(ui);
                });

            if should_close {
                // Determine which screen needs to be reloaded based on modal type
                let screen_to_reload = match modal_type {
                    ActiveModal::CreateNote => Some(ActiveScreen::Notes),
                    ActiveModal::UpdateNote(_) => Some(ActiveScreen::Notes),
                    ActiveModal::AddFeed => Some(ActiveScreen::Feeds),
                    ActiveModal::ManageFeeds => Some(ActiveScreen::Feeds),
                    ActiveModal::AddBookmark => Some(ActiveScreen::Bookmarks),
                    ActiveModal::UpdateBookmark(_) => Some(ActiveScreen::Bookmarks),
                    _ => None,
                };

                // Trigger screen reload if applicable
                if let Some(screen) = screen_to_reload {
                    if let Some(factory) = &self.screen_factory {
                        if let Ok(mut screen_factory) = factory.lock() {
                            screen_factory.clear_screen(screen);
                        }
                    }
                }

                self.close_modal();

                // If we close a modal I want to refresh the data.
                ctx.request_repaint();
            }
        }
    }
}

impl Default for ModalFactory {
    fn default() -> Self {
        Self::new()
    }
}
