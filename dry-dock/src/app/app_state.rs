use std::process::Command;
use std::sync::{Arc, Mutex};

// src/app/app_state.rs
use crate::models::Config;
use crate::app::ActiveScreen;
use crate::ui::modals::*;
use crate::ui::screens::ScreenFactory;
use crate::services::rss_service::refresh_all_feeds;
use crate::services::log_service;

pub struct AppState {
    pub config: Config,
    
    // Modal factory for dynamic modal management
    pub modal_factory: ModalFactory,
    
    // Screen factory for dynamic screen management
    pub screen_factory: Arc<Mutex<ScreenFactory>>,
}

impl AppState {
    pub fn new(config: Config) -> Self {        
        // Prep
        let screen_factory = Arc::new(Mutex::new(ScreenFactory::new()));
        
        // Start background services with the context and screen factory reference so I can handle UI updates
        // whenever I so please. Models I dont care about and maybe the access to services.
        BackgroundServiceManager::start_rss_reloader(screen_factory.clone());

        BackgroundServiceManager::start_llama_server();

        // Create modal factory and give it access to screen factory
        let mut modal_factory = ModalFactory::new();
        modal_factory.set_screen_factory(screen_factory.clone());

        Self {
            config,
            modal_factory,
            screen_factory,
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
        if let Ok(mut factory) = self.screen_factory.lock() {
            factory.set_active_screen(screen);
        }
    }
    
    /// Get the active screen
    pub fn get_active_screen(&self) -> ActiveScreen {
        self.screen_factory.lock()
            .map(|factory| factory.get_active_screen())
            .unwrap_or(ActiveScreen::None)
    }
}

/// Background Service Manager for handling background tasks
pub struct BackgroundServiceManager {
    // None as of now.
}

impl BackgroundServiceManager {
    pub fn start_rss_reloader(screen_factory: Arc<Mutex<ScreenFactory>>) -> () {
        std::thread::spawn(move || {
            log_service::add_log_entry("INFO", "RSS Reloader background service started. Will refresh every 5 minutes.");
            
            loop {
                // Wait 5 minutes
                std::thread::sleep(std::time::Duration::from_secs(300));
                
                // Create a tokio runtime for async operations
                let runtime = tokio::runtime::Runtime::new().unwrap();
                
                // Refresh feeds using async
                match runtime.block_on(refresh_all_feeds()) {
                    Ok(items_added) => {
                        log_service::add_log_entry("INFO", &format!("RSS Feeds refreshed, {} new items added.", items_added));
                        
                        // Clear the feeds screen to trigger reload on next render
                        if let Ok(mut factory) = screen_factory.lock() {
                            factory.clear_screen(ActiveScreen::Feeds);
                            log_service::add_log_entry("INFO", "Feeds screen cleared, will reload on next render");
                        }
                    },
                    Err(e) => {
                        log_service::add_log_entry("ERROR", &format!("Error refreshing RSS feeds: {}", e));
                    }
                }
            }
        });
    }

    pub fn start_daily_backup() -> () {
        // Placeholder for future daily backup service
        // Allow a user to configure a backup locatio to ship this to???
    }

    pub fn start_llama_server() -> () {
        // Once we start create a quick client to check if we are up and if not log error
        std::thread::spawn(move || {
            let llama_path = crate::common::helper::load_llama_path();
            if llama_path.is_empty() {
                log_service::add_log_entry("ERROR", "Llama path not found. Cannot start Llama Server.");
                return;
            }
            
            log_service::add_log_entry("INFO", &format!("Found Ollama binary at: {}", llama_path));
            
            // Check if server is already running before trying to start it
            let client = reqwest::blocking::Client::new();
            let already_running = client.get("http://localhost:11434/api/tags")
                .timeout(std::time::Duration::from_secs(2))
                .send()
                .map(|r| r.status().is_success())
                .unwrap_or(false);
            
            if already_running {
                log_service::add_log_entry("INFO", "Ollama server is already running. Skipping startup.");
            } else {
                // Make sure the binary is executable (macOS/Linux)
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(metadata) = std::fs::metadata(&llama_path) {
                        let mut perms = metadata.permissions();
                        perms.set_mode(0o755);
                        let _ = std::fs::set_permissions(&llama_path, perms);
                    }
                }
                
                // Start the ollama serve command to run the server
                match Command::new(&llama_path)
                    .arg("serve")
                    .spawn()
                {
                    Ok(_) => {
                        log_service::add_log_entry("INFO", "Ollama server started successfully.");
                    }
                    Err(e) => {
                        log_service::add_log_entry("ERROR", &format!("Failed to start Llama Server: {}", e));
                        return;
                    }
                }

                log_service::add_log_entry("INFO", "Llama Server background service started. Checking server status.");

                // Wait a few seconds to allow server to start
                std::thread::sleep(std::time::Duration::from_secs(5));
            } 

            // Check if server is responding by calling the API
            let client = reqwest::blocking::Client::new();
            let res = client.get("http://localhost:11434/api/tags")
                .send();

            match res {
                Ok(response) => {
                    if response.status().is_success() {
                        log_service::add_log_entry("INFO", "Llama Server is running and responding.");
                        
                        // Check if gemma3 model is available, if not pull it
                        if let Ok(text) = response.text() {
                            if !text.contains("gemma3") {
                                log_service::add_log_entry("INFO", "Model gemma3 not found. Pulling it now...");
                                
                                // Pull the model in background
                                match Command::new(&llama_path)
                                    .arg("pull")
                                    .arg("gemma3")
                                    .spawn()
                                {
                                    Ok(_) => {
                                        log_service::add_log_entry("INFO", "Started pulling gemma3 model. This may take a while...");
                                    }
                                    Err(e) => {
                                        log_service::add_log_entry("ERROR", &format!("Failed to pull model: {}", e));
                                    }
                                }
                            } else {
                                log_service::add_log_entry("INFO", "Model gemma3 is available.");
                            }
                        }
                    } else {
                        log_service::add_log_entry("ERROR", "Llama Server is not responding correctly.");
                    }
                }
                Err(e) => {
                    log_service::add_log_entry("ERROR", &format!("Failed to connect to Llama Server: {}", e));
                }
            }
        });
    }

    pub fn start_daily_assitant_message_backup_scraper() -> () {
        // Placeholder for future daily assistant message backup scraper
        // Scans DB and loads into a JSON blog single log and wips the other messages for a new day.
    }
}