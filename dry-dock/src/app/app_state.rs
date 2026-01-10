use eframe::egui;

// src/app/app_state.rs
use crate::models::Config;
use crate::app::ActiveScreen;
use crate::ui::modals::*;
use crate::ui::screens::ScreenFactory;
use crate::services::rss_service::refresh_all_feeds;

pub struct AppState {
    pub config: Config,
    
    // Modal factory for dynamic modal management
    pub modal_factory: ModalFactory,
    
    // Screen factory for dynamic screen management
    pub screen_factory: ScreenFactory,
}

impl AppState {
    pub fn new(config: Config, ctx: egui::Context) -> Self {        
        // Start background services with the context
        BackgroundServiceManager::start_rss_reloader(ctx);

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

/// Background Service Manager for handling background tasks
pub struct BackgroundServiceManager {
    // None as of now.
}

impl BackgroundServiceManager {
    pub fn start_rss_reloader(ctx: egui::Context) -> () {
        std::thread::spawn(move || {
            println!("RSS Reloader background service started. Will refresh every 5 minutes.");
            
            loop {
                // Wait 5 minutes
                std::thread::sleep(std::time::Duration::from_secs(300));
                
                // Refresh feeds
                match refresh_all_feeds() {
                    Ok(items_added) => {
                        println!("RSS Feeds refreshed, {} new items added.", items_added);
                        println!("Requesting UI repaint...");
                        ctx.request_repaint();
                        println!("UI repaint requested");
                    },
                    Err(e) => {
                        eprintln!("Error refreshing RSS feeds: {}", e);
                    }
                }
            }
        });
    }

    pub fn start_daily_backup() -> () {
        // Placeholder for future daily backup service
        // Allow a user to configure a backup locatio to ship this to???
    }

    pub fn start_daily_assitant_message_backup_scraper() -> () {
        // Placeholder for future daily assistant message backup scraper
        // Scans DB and loads into a JSON blog single log and wips the other messages for a new day.
    }
}