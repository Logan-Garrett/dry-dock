use eframe::egui::{self};
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::dal::db_context::*;
use crate::services::*;
use crate::common::helper::*;

// Declare modules
mod dal;
mod common;
mod services;

const AUTO_SAVE_INTERVAL_SECS: u64 = 60;

#[derive(Deserialize, Debug, Clone, Serialize)]
struct Config {
    app_name: String,
    version: String,
    icon_path: String,
    is_vsync_enabled: bool,
}

/// Enum to represent the active screen
/// Every screen in the app should be represented here.
#[derive(Debug, Clone, PartialEq)]
enum ActiveScreen {
    None,
    Feeds,
    Notes,
    Assistant,
    Terminal,
    Bookmarks,
}

// Trait that all screens must implement
trait Screen {
    fn title(&self) -> &str;
    fn render(&mut self, ui: &mut egui::Ui); // Renders the screen
}

#[derive(Default)]
struct BookmarksScreen {
    bookmarks: Vec<(i32, String, String, String)>, // (id, name, path, created_at)
}

impl BookmarksScreen {
    fn title(&self) -> &str {
        "Bookmarks Manager"
    }

    // Add a render method that takes active_modal as parameter
    fn render(&mut self, ui: &mut egui::Ui, active_modal: &mut ActiveModal) {
        ui.heading(self.title());
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            // Create Add Bookmark Button
            let add_bookmark_button = egui::Button::new("Add New Bookmark")
                .min_size(egui::vec2(150.0, 30.0));

            if ui.add(add_bookmark_button).clicked() {
                *active_modal = ActiveModal::AddBookmark;
                self.bookmarks.clear();
            }

            // Create Refresh Button
            let refresh_button = egui::Button::new("Refresh")
                .min_size(egui::vec2(100.0, 30.0));

            if ui.add(refresh_button).clicked() {
                self.bookmarks.clear();
            }
        });

        // Separator
        ui.separator();
        
        // Load bookmarks if empty
        if self.bookmarks.is_empty() {
            match bookmark_service::fetch_all_bookmarks() {
                Ok(bookmarks) => {
                    self.bookmarks = bookmarks;
                }
                Err(e) => {
                    ui.label(format!("Error loading bookmarks: {}", e));
                    return;
                }
            }
        }

        // Track bookmark to delete
        let mut id_to_delete: Option<i32> = None;

        // Display bookmarks
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                for (_id, name, path, created_at) in &self.bookmarks {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.label(format!("Name: {}", name));
                                ui.label(format!("Path/URL: {}", path));
                                ui.label(format!("Created At: {}", created_at));
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                // Delete bookmark button
                                let delete_button = egui::Button::new("Delete Bookmark")
                                    .min_size(egui::vec2(120.0, 30.0));
                                if ui.add(delete_button).clicked() {
                                    id_to_delete = Some(*_id);
                                }
                                
                                // Open bookmark button
                                let open_button = egui::Button::new("Open")
                                    .min_size(egui::vec2(80.0, 30.0));
                                if ui.add(open_button).clicked() {
                                    bookmark_service::open_bookmark_path(path);
                                }
                            });
                        });
                    });

                    ui.add_space(10.0);
                    ui.separator();
                }
            });

        // Delete bookmark after iteration
        if let Some(id) = id_to_delete {
            match bookmark_service::delete_bookmark(id) {
                Ok(_) => {
                    println!("Bookmark deleted successfully.");
                    // Remove from local list
                    self.bookmarks.retain(|(bm_id, _, _, _)| *bm_id != id);

                    // Now force a UI refresh by clearing and reloading bookmarks
                    self.bookmarks.clear();
                }
                Err(e) => {
                    println!("Error deleting bookmark: {}", e);
                }
            }
        }
    }
}

#[derive(Default)]
struct FeedsScreen {
    // Store feed items instead of feeds for now.
    feed_items: Vec<(i32, String, String, String, i64)>, // (id, title, link, description, pub_date)
}

impl Screen for FeedsScreen {
    fn title(&self) -> &str {
        "RSS Feed Items"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading(self.title());
        ui.add_space(10.0);
        
        // Add refresh button
        if ui.button("Refresh All Feeds").clicked() {
            match crate::services::rss_service::refresh_all_feeds() {
                Ok(msg) => println!("{}", msg),
                Err(e) => println!("Error refreshing feeds: {}", e),
            }
            // Reload items
            self.feed_items.clear();
        }
        
        ui.separator();
        
        // Load feed items if empty
        if self.feed_items.is_empty() {
            // Come Back to this becuase I may want more or less....
            match get_feed_items(10000) {
                Ok(items) => {
                    self.feed_items = items;
                }
                Err(e) => {
                    ui.label(format!("Error loading feed items: {}", e));
                    return;
                }
            }
        }

        // Display feed items
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                for (_id, title, link, description, pub_date) in &self.feed_items {
                    ui.group(|ui| {
                        ui.hyperlink_to(title, link);
                        
                        // Format date
                        let datetime = chrono::DateTime::from_timestamp(*pub_date, 0);
                        if let Some(dt) = datetime {
                            ui.label(format!("Published: {}", dt.format("%Y-%m-%d %H:%M")));
                        }
                        
                        // Show truncated description
                        let desc = if description.len() > 200 {
                            format!("{}...", &description[..200])
                        } else {
                            description.clone()
                        };
                        ui.label(desc);
                    });
                    ui.add_space(10.0);
                    ui.separator();
                }
            });
    }
} 

#[derive(Default)]
struct NotesScreen {
    notes: Vec<(i32, String, String, i64, Option<i64>)>, // (id, title, details, created_at, updated_at) // May need a model down the road....
}

impl Screen for NotesScreen {
    fn title(&self) -> &str {
        "Notes"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) {
        // Add a refresh button (May not keep this here later)
        let refresh_notes_button = egui::Button::new("Refresh Notes")
            .min_size(egui::vec2(120.0, 30.0));
        if ui.add(refresh_notes_button).clicked() {
            match get_notes() {
                Ok(notes) => {
                    self.notes = notes;
                }
                Err(e) => {
                    println!("Error refreshing notes: {}", e);
                }
            }
        }
        
        // Separator
        ui.separator();

        // Load notes if empty (or add a refresh button later)
        if self.notes.is_empty() {
            match get_notes() {
                Ok(notes) => {
                    self.notes = notes;
                }
                Err(e) => {
                    ui.label(format!("Error loading notes: {}", e));
                    return;
                }
            }
        }

        // Display notes
        let mut id_to_delete: Option<i32> = None;
        
        ui.vertical(|ui| {
            for (id, title, details, created_at, updated_at) in &self.notes {
                // Load Notes Styling
                ui.style_mut().text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional)
                );
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            // ui.label(format!("ID: {}", id)); // I dont want this right now.
                            ui.label(format!("Title: {}", title));
                            ui.label(format!("Details: {}", details));
                            ui.label(format!("Created At: {}", created_at));
                            if let Some(updated) = updated_at {
                                ui.label(format!("Updated At: {}", updated));
                            }
                        });
                        
                        // Push delete button to the right
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                            let delete_note_button = egui::Button::new("Delete Note")
                                .min_size(egui::vec2(100.0, 30.0));
                            
                            // We may want to add a confirmation dialog later.
                            // Make it a generic one that can be used across the app.
                            if ui.add(delete_note_button).clicked() {
                                id_to_delete = Some(*id);
                            }
                        });
                    });
                });
                ui.add_space(10.0);
                // Separator between notes
                ui.separator();
            }
        });
        
        // Delete note after iteration
        if let Some(id) = id_to_delete {
            match delete_note(id) {
                Ok(_) => {
                    println!("Note deleted successfully.");
                    // Remove from local list
                    self.notes.retain(|(note_id, _, _, _, _)| *note_id != id);

                    // Now force a UI refresh by clearing and reloading notes
                    self.notes.clear();
                }
                Err(e) => {
                    println!("Error deleting note: {}", e);
                }
            }
        }
    }
}

/// Enum to represent the active modal state
/// Every modal in the app should be represented here.
#[derive(Debug, Clone, PartialEq)]
enum ActiveModal {
    None,
    AddFeed,
    CreateNote,
    AddBookmark,
    Settings,
}

/// Trait that all modals must implement
trait Modal {
    fn title(&self) -> &str;
    fn render(&mut self, ui: &mut egui::Ui) -> bool; // Returns true if should close
}

#[derive(Default)]
struct AddBookmarkModal {
    name: String,
    location: String,
}

impl Modal for AddBookmarkModal {
    fn title(&self) -> &str {
        "Add New Bookmark"
    }

    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;

        ui.label("Bookmark Name:");
        ui.add_space(5.0);
        ui.text_edit_singleline(&mut self.name);
        ui.label("Bookmark Location (URL or File Path):");
        ui.add_space(5.0);
        ui.text_edit_singleline(&mut self.location);
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Add").clicked() {
                if let Err(e) = bookmark_service::add_new_bookmark(&self.name, &self.location) {
                    println!("Error adding bookmark: {}", e);
                } else {
                    // println!("Bookmark added successfully.");
                }
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });

        // Reset fields if closing
        if should_close {
            self.name.clear();
            self.location.clear();
        }

        should_close
    }
}

#[derive(Default)]
struct SettingsModal;

impl Modal for SettingsModal {
    fn title(&self) -> &str {
        "Settings"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        ui.label("Settings modal is under construction.");
        
        if ui.button("Close").clicked() {
            should_close = true;
        }
        
        should_close
    }
}

/// Add Feed Modal
#[derive(Default)]
struct AddFeedModal {
    feed_title: String,
    url: String,
}

impl Modal for AddFeedModal {
    fn title(&self) -> &str {
        "Add New RSS Feed"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        ui.label("Enter RSS Feed Title:");
        ui.add_space(5.0);
        ui.text_edit_singleline(&mut self.feed_title);
        ui.label("Enter RSS Feed URL:");
        ui.add_space(5.0);
        ui.text_edit_singleline(&mut self.url);
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            if ui.button("Add").clicked() {
                if let Err(e) = add_feed(&self.url, &self.feed_title) {
                    println!("Error adding feed: {}", e);
                } else {
                    // println!("Feed added successfully.");
                }
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });

        // Reset fields if closing
        if should_close {
            self.feed_title.clear();
            self.url.clear();
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
            .max_height(ui.available_height() - 20.0) 
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
                // DEBUG if needed I Guess
                // println!("Creating Note...");
                // println!("Title: {}", self.title);
                // println!("Details: {}", self.details);
                if let Err(e) = create_note(&self.title, &self.details) {
                    println!("Error creating note: {}", e);
                } else {
                    // println!("Note created successfully.");
                }
                // Find a way to throw success/failure messages later.
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });

        // Clean up fields if closing
        if should_close {
            self.title.clear();
            self.details.clear();
        }
        
        should_close
    }
}

struct MyApp {
    // Load App Config
    config: Config,
    // Single state for active screen
    active_screen: ActiveScreen,
    // SCTREENS
    feeds_screen: FeedsScreen, // Maybe put these in a Vec or HashMap to not have to add new fields every time and just load some other way maybe via trait objects?
    notes_screen: NotesScreen,
    bookmarks_screen: BookmarksScreen,
    // Single state for all modals
    active_modal: ActiveModal,
    // Dynamic modal instances (Similar to Enums and are needed here and there but this is for state)
    add_feed_modal: AddFeedModal,
    create_note_modal: CreateNoteModal,
    add_bookmark_modal: AddBookmarkModal,
    // manage_feeds_modal: ManageFeedsModal,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self { 
            config,
            active_screen: ActiveScreen::None,
            active_modal: ActiveModal::None,
            feeds_screen: FeedsScreen::default(), // These Ttitles DUMB here but ehhhh for now.
            notes_screen: NotesScreen::default(),
            bookmarks_screen: BookmarksScreen::default(),
            add_feed_modal: AddFeedModal::default(),
            create_note_modal: CreateNoteModal::default(),
            add_bookmark_modal: AddBookmarkModal::default(),
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
                load_menu(ui, &self.config, &mut self.active_modal, &mut self.active_screen);
            });
        });

        // Central Panel - only show default content if no active screen
        if self.active_screen == ActiveScreen::None {
            egui::CentralPanel::default()
            .show(ctx, |ui| {
                // Load Central Panel
                load_central_panel(ui, &self.config);
            });
        }

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

        // IF we have an active screen, render it
        self.render_active_screen(ctx);
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
            ActiveModal::AddBookmark => {
                let modal = &mut self.add_bookmark_modal;
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
            ActiveModal::Settings => {
                let modal = &mut SettingsModal;
                let title = modal.title().to_string();
                let mut should_close = false;

                egui::Window::new(&title)
                    .collapsible(false)
                    .resizable(true)
                    .default_size([600.0, 400.0])
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

    fn render_active_screen(&mut self, ctx: &egui::Context) {
        match self.active_screen {
            ActiveScreen::Feeds => {
                egui::CentralPanel::default()
                    .show(ctx, |ui| {
                        self.feeds_screen.title();
                        self.feeds_screen.render(ui);
                    });
            }
            ActiveScreen::Notes => {
                egui::CentralPanel::default()
                    .show(ctx, |ui| {
                        self.notes_screen.title();
                        self.notes_screen.render(ui);
                    });
            }
            ActiveScreen::Assistant => {
                egui::CentralPanel::default()
                    .show(ctx, |ui| {
                        ui.heading("AI Assistant");
                        ui.label("AI Assistant screen is under construction.");
                    });
            }
            ActiveScreen::Terminal => {
                egui::CentralPanel::default()
                    .show(ctx, |ui| {
                         ui.heading("Terminal");
                         ui.label("In development.");
                    });
            }
            ActiveScreen::Bookmarks => {
                egui::CentralPanel::default()
                    .show(ctx, |ui| {
                        self.bookmarks_screen.title();
                        self.bookmarks_screen.render(ui, &mut self.active_modal);
                    });
            }
            ActiveScreen::None => {
                // No active screen to render
                // Home Page?
            }
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

    // Change this to load from config later.
    // May need a builder.rs to swap the DB Path based on OS or something or 
    // store them all in config and load based on OS.
    let db_path = get_database_path("DryDock");
    _ = initialize_database(&db_path)
        .expect("Failed to initialize database");

    // Load Background Workers
    // RSS Feed Fetcher

    // Need to find a good Cron Scheduler for Rust.

    // Load Application Icon
    let icon_image = image::open(&config.icon_path)
        .expect("Failed to load application icon")
        .to_rgba8();
    
    let (icon_width, icon_height) = icon_image.dimensions();
    let icon = eframe::egui::IconData {
        rgba: icon_image.into_raw(),
        width: icon_width,
        height: icon_height,
    };

    // Configures The Window.
    let options =  eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_icon(icon)
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
fn load_menu(ui: &mut egui::Ui, config: &Config, active_modal: &mut ActiveModal, active_screen: &mut ActiveScreen) {
    // Load Menu Styling
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Button, 
        egui::FontId::new(24.0, egui::FontFamily::Proportional)
    );

    // Load Menu Padding For Button
    ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);

    ui.menu_button(config.app_name.as_str(), |ui| {
        // Create settings button
        let settings_button = egui::Button::new("Settings")
            .min_size(egui::vec2(100.0, 30.0));

        // Create Exit Button
        let exit_button = egui::Button::new("Exit")
            .min_size(egui::vec2(100.0, 30.0));
        
        // Load Settings Button
        if ui.add(settings_button).clicked() {
            println!("Loading Settings...");
            // Load Settings Model.
            *active_modal = ActiveModal::Settings;
        }
        
        // Load Exit Button
        if ui.add(exit_button).clicked() {
            std::process::exit(0);
        }
    });

    // Separator
    ui.separator();

    // Load RSS Menu Button - Highlight if Feeds screen is active
    let rss_button = ui.menu_button("RSS", |ui| {
        // Create Load Feeds Screen Button
        let is_feeds_active = *active_screen == ActiveScreen::Feeds;
        let load_feeds_screen_button = egui::Button::new("View RSS Feeds")
            .min_size(egui::vec2(100.0, 30.0))
            .selected(is_feeds_active);

        // Create Add New Feed Button
        let add_feed_button = egui::Button::new("Add New RSS Feed")
            .min_size(egui::vec2(100.0, 30.0));

        // Manage Subscribed Feeds Button
        let manage_subscribed_feeds_button = egui::Button::new("RSS Subscribed Feeds")
            .min_size(egui::vec2(150.0, 30.0));

        // Load Feeds Screen Button
        if ui.add(load_feeds_screen_button).clicked() {
            println!("Loading RSS Feeds Screen...");
            *active_modal = ActiveModal::None; // Close any modals
            *active_screen = ActiveScreen::Feeds; // LOAD ME SCREEEEEN
        }

        // Add New Feed Button
        if ui.add(add_feed_button).clicked() {
            println!("Adding New RSS Feed...");
            *active_modal = ActiveModal::AddFeed;
        }

        // Manage Subscribed Feeds Button
        if ui.add(manage_subscribed_feeds_button).clicked() {
            println!("Managing Subscribed Feeds...");
            // *active_modal = ActiveModal::None; // Close any modals
            // *active_screen = ActiveScreen::Feeds; // LOAD ME SCREEEEEN
        }
    });

    // Highlight the RSS menu button if Feeds screen is active
    if *active_screen == ActiveScreen::Feeds {
        rss_button.response.highlight();
    }

    // Separator
    ui.separator();

    // Load Notes Menu Button - Highlight if Notes screen is active
    let notes_button = ui.menu_button("Notes", |ui| {
        // Create New Note Button
        let new_note_button = egui::Button::new("Create New Note")
            .min_size(egui::vec2(100.0, 30.0));

        // View Notes Screen Button
        let is_notes_active = *active_screen == ActiveScreen::Notes;
        let view_notes_screen_button = egui::Button::new("View Notes")
            .min_size(egui::vec2(100.0, 30.0))
            .selected(is_notes_active);
        
        // Load New Note Button
        if ui.add(new_note_button).clicked() {
            *active_modal = ActiveModal::CreateNote;
        }

        // Load View Notes Screen Button
        if ui.add(view_notes_screen_button).clicked() {
            println!("Loading Notes Screen...");
            *active_modal = ActiveModal::None; // Close any modals
            *active_screen = ActiveScreen::Notes; // LOAD ME SCREEEEEN
        }
    });

    // Highlight the Notes menu button if Notes screen is active
    if *active_screen == ActiveScreen::Notes {
        notes_button.response.highlight();
    }

    // Separator
    ui.separator();

    // Load AI Addsistant Menu Button which when clicked opens a screen and doesnt have 
    // sub buttons.
    let assistant_button = ui.button("Assistant");
    if assistant_button.clicked() {
        println!("Loading AI Assistant Screen...");
        *active_modal = ActiveModal::None; // Close any modals
        *active_screen = ActiveScreen::Assistant; // LOAD ME SCREEEEEN 
        // Maybe change this to a modal later?
    }

    if *active_screen == ActiveScreen::Assistant {
        assistant_button.highlight();
    }

    // Separator
    ui.separator();

    // Load Terminal Menu Button which when clicked opens a screen which we shall hook into some backgroudn
    // terminal maybe handled in settings.
    let terminal_button = ui.button("Terminal");
    if terminal_button.clicked() {
        println!("Loading Terminal Screen...");
        *active_modal = ActiveModal::None; // Close any modals
        *active_screen = ActiveScreen::Terminal; // LOAD ME SCREEEEEN
    }

    if *active_screen == ActiveScreen::Terminal {
        terminal_button.highlight();
    }

    // Separator
    ui.separator();

    // Losing Bookmarks and having to use diff browsers is ANNOYINg.
    // Store them all in the app and have a bookmarks manager.
    let bookmarks_button = ui.button("Bookmarks");
    if bookmarks_button.clicked() {
        println!("Loading Bookmarks Manager...");
        *active_screen = ActiveScreen::Bookmarks;
    }

    if *active_screen == ActiveScreen::Bookmarks {
        bookmarks_button.highlight();
    }
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
    // Welcome Message
    ui.heading(format!("Welcome to {}!", config.app_name));
    ui.add_space(10.0);
    ui.label("Use the menu above to navigate through the application.");
    ui.add_space(10.0);
}

// BELOW ARE DB Queries and Methods so IGNORE AS THEY
// WILL BE MOVED LATER TO THEIR RESPECTIVE Files and Folders nbut I am lazy.


/////////////////////////////////////////////////////
/// Notes Related DB Methods
/////////////////////////////////////////////////////
fn create_note(title: &str, details: &str) -> Result<(), String> {
    let conn = get_connection()?;

    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO notes (title, details, created_at) VALUES (?1, ?2, ?3)",
        params![title, details, now],
    )
    .map_err(|e| format!("Failed to create note: {}", e))?;

    Ok(())
}

fn delete_note(note_id: i32) -> Result<(), String> {
    let conn = get_connection()?;

    conn.execute(
        "DELETE FROM notes WHERE id = ?1",
        params![note_id],
    )
    .map_err(|e| format!("Failed to delete note: {}", e))?;

    Ok(())
}

fn get_notes() -> Result<Vec<(i32, String, String, i64, Option<i64>)>, String> {
    let conn = get_connection()?;

    let mut stmt = conn
        .prepare("SELECT id, title, details, created_at, updated_at FROM notes")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let notes = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i32>("id")?,
                row.get::<_, String>("title")?,
                row.get::<_, String>("details")?,
                row.get::<_, i64>("created_at")?,
                row.get::<_, Option<i64>>("updated_at")?,
            ))
        })
        .map_err(|e| format!("Failed to query notes: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect notes: {}", e))?;

    Ok(notes)
}

/////////////////////////////////////////////////////
/// END Notes Related DB Methods
/////////////////////////////////////////////////////

/////////////////////////////////////////////////////
/// RSS Feed Related DB Methods
/////////////////////////////////////////////////////

fn add_feed(url: &str, title: &str) -> Result<(), String> {
    let conn = get_connection()?;

    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO feeds (title, url, created_at) VALUES (?1, ?2, ?3)",
        params![title, url, now],
    )
    .map_err(|e| format!("Failed to add feed: {}", e))?;

    Ok(())
}
/* 
// This doen the road to show what feeds I have.
fn get_feeds() -> Result<Vec<(i32, String, String, Option<i64>, i64)>, String> {
    let conn = get_connection()?;

    let mut stmt = conn
        .prepare("SELECT id, title, url, last_updated, created_at FROM feeds")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let feeds = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i32>("id")?,
                row.get::<_, String>("title")?,
                row.get::<_, String>("url")?,
                row.get::<_, Option<i64>>("last_updated")?,
                row.get::<_, i64>("created_at")?,
            ))
        })
        .map_err(|e| format!("Failed to query feeds: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect feeds: {}", e))?;

    Ok(feeds)
} */

fn get_feed_items(limit: i32) -> Result<Vec<(i32, String, String, String, i64)>, String> {
    let conn = get_connection()?;

    let mut stmt = conn
        .prepare("SELECT id, title, link, description, pub_date FROM feed_items ORDER BY pub_date DESC LIMIT ?1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let mut items = stmt
        .query_map(params![limit], |row| {
            Ok((
                row.get::<_, i32>("id")?,
                row.get::<_, String>("title")?,
                row.get::<_, String>("link")?,
                row.get::<_, String>("description")?,
                row.get::<_, i64>("pub_date")?,
            ))
        })
        .map_err(|e| format!("Failed to query feed items: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect feed items: {}", e))?;

    // Sort this by pub_date DESC to get latest items first.
    items.sort_by(|a, b| b.4.cmp(&a.4));
    
    Ok(items)
}