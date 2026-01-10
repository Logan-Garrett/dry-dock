// src/ui/screens/screen_factory.rs
use eframe::egui;
use std::collections::HashMap;
use crate::app::ActiveScreen;
use crate::ui::screens::*;
use crate::ui::modals::ActiveModal;

/// Type alias for screen trait objects
type BoxedScreen = Box<dyn ScreenRenderer>;

/// Trait for screens that can be rendered dynamically
pub trait ScreenRenderer {
    fn render(&mut self, ui: &mut egui::Ui, modal_opener: &mut dyn FnMut(ActiveModal));
}

/// Wrapper for FeedsScreen to implement ScreenRenderer
struct FeedsScreenWrapper(FeedsScreen);

impl ScreenRenderer for FeedsScreenWrapper {
    fn render(&mut self, ui: &mut egui::Ui, modal_opener: &mut dyn FnMut(ActiveModal)) {
        self.0.render(ui, modal_opener);
    }
}

/// Wrapper for NotesScreen to implement ScreenRenderer
struct NotesScreenWrapper(NotesScreen);

impl ScreenRenderer for NotesScreenWrapper {
    fn render(&mut self, ui: &mut egui::Ui, modal_opener: &mut dyn FnMut(ActiveModal)) {
        self.0.render(ui, modal_opener);
    }
}

/// Wrapper for BookmarksScreen to implement ScreenRenderer
struct BookmarksScreenWrapper(BookmarksScreen);

impl ScreenRenderer for BookmarksScreenWrapper {
    fn render(&mut self, ui: &mut egui::Ui, modal_opener: &mut dyn FnMut(ActiveModal)) {
        self.0.render(ui, modal_opener);
    }
}

/// Factory for creating and managing screens dynamically
pub struct ScreenFactory {
    screens: HashMap<ActiveScreen, BoxedScreen>,
    current_screen: ActiveScreen,
}

impl ScreenFactory {
    /// Create a new screen factory with all screens initialized
    /// Add more screens here as they are implemented
    pub fn new() -> Self {
        let mut screens: HashMap<ActiveScreen, BoxedScreen> = HashMap::new();
        
        screens.insert(
            ActiveScreen::Feeds,
            Box::new(FeedsScreenWrapper(FeedsScreen::default())),
        );
        screens.insert(
            ActiveScreen::Notes,
            Box::new(NotesScreenWrapper(NotesScreen::default())),
        );
        screens.insert(
            ActiveScreen::Bookmarks,
            Box::new(BookmarksScreenWrapper(BookmarksScreen::default())),
        );

        Self {
            screens,
            current_screen: ActiveScreen::None,
        }
    }

    /// Set the active screen
    pub fn set_active_screen(&mut self, screen: ActiveScreen) {
        self.current_screen = screen;
    }

    /// Get the current active screen
    pub fn get_active_screen(&self) -> ActiveScreen {
        self.current_screen.clone()
    }

    /// Render the active screen
    pub fn render(&mut self, ctx: &egui::Context, modal_opener: &mut dyn FnMut(ActiveModal)) {
        match self.current_screen {
            ActiveScreen::None => {
                // No screen active, home will be shown
            }
            ActiveScreen::Assistant => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Assistant");
                    ui.label("Assistant screen is under construction.");
                });
            }
            ActiveScreen::Terminal => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("Terminal");
                    ui.label("In development.");
                });
            }
            ActiveScreen::Feeds | ActiveScreen::Notes | ActiveScreen::Bookmarks => {
                if let Some(screen) = self.screens.get_mut(&self.current_screen) {
                    egui::CentralPanel::default().show(ctx, |ui| {
                        screen.render(ui, modal_opener);
                    });
                }
            }
        }
    }
}

impl Default for ScreenFactory {
    fn default() -> Self {
        Self::new()
    }
}
