use chrono::Datelike;
use eframe::egui;
use serde::{Deserialize, Serialize};

const AUTO_SAVE_INTERVAL_SECS: u64 = 60;

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Config {
    app_name: String,
    version: String,
    is_vsync_enabled: bool,
}

/// Enum to represent the active modal state
/// Every modal in the app should be represented here.
#[derive(Debug, Clone, PartialEq)]
enum ActiveModal {
    None,
    AddFeed,
    CreateNote,
    RefreshFeeds,
}

/// Trait that all modals must implement
trait Modal {
    fn title(&self) -> &str;
    fn render(&mut self, ui: &mut egui::Ui) -> bool; // Returns true if should close
}

/// Add Feed Modal
#[derive(Default)]
struct AddFeedModal {
    url: String,
}

impl Modal for AddFeedModal {
    fn title(&self) -> &str {
        "Add New RSS Feed"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        ui.label("Enter RSS Feed URL:");
        ui.add_space(5.0);
        ui.text_edit_singleline(&mut self.url);
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            if ui.button("Add").clicked() {
                println!("Adding feed: {}", self.url);
                // TODO: Add feed logic
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
}

/// Refresh Feeds Modal
#[derive(Default)]
struct RefreshFeedsModal;

impl Modal for RefreshFeedsModal {
    fn title(&self) -> &str {
        "Refresh Feeds"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        ui.label("Refreshing all RSS feeds...");
        ui.add_space(10.0);

        if ui.button("Close").clicked() {
            should_close = true;
        }
        
        should_close
    }
}

/// Create Note Modal
#[derive(Default)]
struct CreateNoteModal {
    title: String,
    details: String,
}

impl Modal for CreateNoteModal {
    fn title(&self) -> &str {
        "Create New Note"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        ui.label("Note Title:");
        ui.text_edit_singleline(&mut self.title);
        ui.add_space(10.0);

        // Note Details.
        ui.label("Note Details:");
        egui::ScrollArea::vertical()
            .id_salt("note_details_scroll")
            .max_height(ui.available_height() - 50.0) // Leave space for buttons
            .show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.details)
                        .desired_width(f32::INFINITY)
                        .desired_rows(20)
                        .font(egui::TextStyle::Monospace)
                );
            });
        
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            if ui.button("Create").clicked() {
                println!("Creating Note...");
                println!("Title: {}", self.title);
                println!("Details: {}", self.details);
                println!("Note Date: {}", chrono::Utc::now().to_rfc3339());
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
}

struct MyApp {
    // Load App Config
    config: Config,
    // Single state for all modals
    active_modal: ActiveModal,
    // Dynamic modal instances (Similar to Enums and are needed here and there but this is for state)
    add_feed_modal: AddFeedModal,
    create_note_modal: CreateNoteModal,
    refresh_feeds_modal: RefreshFeedsModal,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self { 
            config,
            active_modal: ActiveModal::None,
            add_feed_modal: AddFeedModal::default(),
            create_note_modal: CreateNoteModal::default(),
            refresh_feeds_modal: RefreshFeedsModal::default(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {        
        // Lets Load the Menu Bar
        egui::TopBottomPanel::top("menu_bar")
        .show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                // Load Menu
                load_menu(ui, &self.config, &mut self.active_modal);
            });
        });

        // Central Panel
        egui::CentralPanel::default()
        .show(ctx, |ui| {
            // Load Central Panel
            load_central_panel(ui, &self.config);
        });

        // Bottom Panel
        egui::TopBottomPanel::bottom("bottom_panel")
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                // Version Info
                ui.label(format!("Version: {}", self.config.version));
                // Separator
                ui.separator();
                // Copyright Info
                ui.label(format!("© {} Dry Dock. All rights reserved.", load_current_year()));
            });
        });

        // Render active modal (if any)
        self.render_active_modal(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Nothing to clean up as of now.
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Nothing to save as of now.
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        // No auto-save interval set.
        std::time::Duration::from_secs(AUTO_SAVE_INTERVAL_SECS)
    }
}

impl MyApp {
    /// Render the active modal dynamically
    fn render_active_modal(&mut self, ctx: &egui::Context) {
        if self.active_modal == ActiveModal::None {
            return;
        }

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

        // Get the appropriate modal instance
        // As you add models add here as this is basically our Model Factory
        // Got to love Factory Patterns :)
        let (_title, should_close) = match &mut self.active_modal {
            ActiveModal::AddFeed => {
                let modal = &mut self.add_feed_modal;
                let title = modal.title().to_string();
                let mut should_close = false;
                
                egui::Window::new(&title)
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        should_close = modal.render(ui);
                    });
                
                (title, should_close)
            }
            ActiveModal::CreateNote => {
                let modal = &mut self.create_note_modal;
                let title = modal.title().to_string();
                let mut should_close = false;

                egui::Window::new(&title)
                    .collapsible(false)
                    .resizable(true) // Notes can be larger
                    .default_size([800.0, 600.0]) // Large default size for markdown
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        should_close = modal.render(ui);
                    });
                
                (title, should_close)
            }
            ActiveModal::RefreshFeeds => {
                let modal = &mut self.refresh_feeds_modal;
                let title = modal.title().to_string();
                let mut should_close = false;
                
                egui::Window::new(&title)
                    .collapsible(false)
                    .resizable(false)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ctx, |ui| {
                        should_close = modal.render(ui);
                    });
                
                (title, should_close)
            }
            ActiveModal::None => (String::new(), false),
        };

        // Close modal if requested
        if should_close {
            self.active_modal = ActiveModal::None;
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    // Load Config File.
    let config_data = std::fs::read_to_string("AppConfig.json")
        .expect("Unable to read config file");

    // Parse Config File.
    let config: Config = serde_json::from_str(&config_data)
        .expect("Unable to parse config file");

    // Load Startup Related Stuff.
    // * Sqlite Database
    // * Sqlite Migrations & Structures
    // * What Else?

    // Load Background Workers
    // RSS Feed Fetcher

    // Need to find a good Cron Scheduler for Rust.

    // Maybe Load Custom Icon(s) Here?

    // Configures The Window.
    let options =  eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_fullscreen(true),

        // Enable Vysnc If Configured.
        vsync: config.is_vsync_enabled,

        // Persist Window
        persist_window: true,
        
        ..Default::default()
    };
    
    // Runs The Egui Application.
    eframe::run_native(
        &format!("{} v{}", config.app_name, config.version),
        options,
        Box::new(|cc| 
            Ok(
                Box::new(
                    MyApp::new(cc, config.clone())
                )
            )
        ),
    )
}


// Load Menu
fn load_menu(ui: &mut egui::Ui, config: &Config, active_modal: &mut ActiveModal) {
    // Load Menu Styling
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Button, 
      egui::FontId::new(24.0, 
     egui::FontFamily::Proportional)
    );

    // Load Menu Padding For Buttom
    ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);

    // Load App Menu Button
    ui.menu_button(config.app_name.as_str(), |ui| {
        // Create Exit Button
        let exit_button = egui::Button::new("Exit")
        .min_size(egui::vec2(100.0, 30.0));
        
        // Load Exit Button
        if ui.add(exit_button)
        .clicked() {
            std::process::exit(0);
        }
    });

    // Separator
    ui.separator();

    // Load RSS Menu Button
    ui.menu_button("RSS", |ui| {
        // Create Fetch Feeds Button
        let fetch_feeds_button = egui::Button::new("Force Refresh Feeds")
            .min_size(egui::vec2(100.0, 30.0));

        // Create Add New Feed Button
        let add_feed_button = egui::Button::new("Add New Feed")
            .min_size(egui::vec2(100.0, 30.0));

        // Add New Feed Button
        if ui.add(add_feed_button)
        .clicked() {
            println!("Adding New RSS Feed...");
            *active_modal = ActiveModal::AddFeed;
        }

        // Load Fetch Feeds Button
        if ui.add(fetch_feeds_button)
        .clicked() {
            println!("Fetching RSS Feeds...");
            *active_modal = ActiveModal::RefreshFeeds;
        }
    });

    // Separator
    ui.separator();

    // Load Notes Menu Button
    ui.menu_button("Notes", |ui| {
        // Create New Note Button
        let new_note_button = egui::Button::new("Create New Note")
            .min_size(egui::vec2(100.0, 30.0));
        
        // Load New Note Button
        if ui.add(new_note_button)
        .clicked() {
            *active_modal = ActiveModal::CreateNote;
        }
    });
}

// Load Central Panel
fn load_central_panel(ui: &mut egui::Ui, config: &Config) {
    // Load Central Panel Styling
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(18.0, egui::FontFamily::Proportional)
    );

    // Load Central Panel Spacing
    ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

    // Body Content
    if ui.button("Click me")
    .clicked() {
        println!("App Name: {}", config.app_name);
        println!("Button clicked!");
    }
}

// Helper to load the current year as a string
fn load_current_year() -> String {
    let current_year = chrono::Utc::now().year();
    current_year.to_string()
}