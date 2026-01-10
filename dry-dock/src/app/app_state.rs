// src/app/app_state.rs
use crate::models::Config;
use crate::app::ActiveScreen;
use crate::ui::modals::*;
use crate::ui::screens::ScreenFactory;

pub struct AppState {
    pub config: Config,
    
    // Modal factory for dynamic modal management
    pub modal_factory: ModalFactory,
    
    // Screen factory for dynamic screen management
    pub screen_factory: ScreenFactory,
}

impl AppState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            modal_factory: ModalFactory::new(),
            screen_factory: ScreenFactory::new(),
        }
    }
    
    /// Open a modal
    pub fn open_modal(&mut self, modal_type: ActiveModal) {
        self.modal_factory.open_modal(modal_type);
    }
    
    /// Close the active modal
    pub fn close_modal(&mut self) {
        self.modal_factory.close_modal();
    }
    
    /// Set the active screen
    pub fn set_active_screen(&mut self, screen: ActiveScreen) {
        self.screen_factory.set_active_screen(screen);
    }
    
    /// Get the active screen
    pub fn get_active_screen(&self) -> ActiveScreen {
        self.screen_factory.get_active_screen()
    }
}
