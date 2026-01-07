# Project Structure & Migration Guide

## Table of Contents
- [Why Restructure?](#why-restructure)
- [Current vs Recommended Structure](#current-vs-recommended-structure)
- [Module Organization Strategies](#module-organization-strategies)
- [Step-by-Step Migration](#step-by-step-migration)
- [Rust Module System Basics](#rust-module-system-basics)
- [Best Practices](#best-practices)
- [Complete Example](#complete-example)

---

## Why Restructure?

**Yes, you would definitely benefit from restructuring!** Here's why:

### Current Issues
❌ **Single file getting large** - Everything in `main.rs` makes it hard to navigate  
❌ **Hard to find code** - All modals, configs, and logic mixed together  
❌ **Difficult to test** - Can't easily test individual components  
❌ **No separation of concerns** - UI, business logic, and data all intertwined  
❌ **Hard to collaborate** - Multiple people editing same file causes conflicts  

### Benefits of Restructuring
✅ **Better organization** - Each module has a clear purpose  
✅ **Easier to navigate** - Find code quickly by feature  
✅ **Testable** - Test modules independently  
✅ **Reusable** - Share code between features  
✅ **Scalable** - Easy to add new features without cluttering  
✅ **Clear boundaries** - UI separated from business logic  

---

## Current vs Recommended Structure

### Current Structure
```
dry-dock/
├── src/
│   └── main.rs          # Everything in one file (400+ lines)
├── docs/
├── AppConfig.json
└── Cargo.toml
```

### Recommended Structure (Feature-Based)
```
dry-dock/
├── src/
│   ├── main.rs                 # Entry point (minimal, ~50 lines)
│   ├── app.rs                  # Main app struct and update loop
│   ├── config.rs               # Configuration loading
│   │
│   ├── ui/                     # UI components
│   │   ├── mod.rs              # UI module definition
│   │   ├── menu.rs             # Menu bar
│   │   ├── panels.rs           # Panels (central, bottom)
│   │   └── styles.rs           # Styling utilities
│   │
│   ├── modals/                 # All modals
│   │   ├── mod.rs              # Modal trait + manager
│   │   ├── add_feed.rs         # Add feed modal
│   │   ├── create_note.rs      # Create note modal
│   │   └── refresh_feeds.rs    # Refresh feeds modal
│   │
│   ├── features/               # Business logic
│   │   ├── mod.rs
│   │   ├── feeds/              # RSS feed feature
│   │   │   ├── mod.rs
│   │   │   ├── models.rs       # Feed, FeedItem structs
│   │   │   ├── repository.rs   # Database operations
│   │   │   └── fetcher.rs      # HTTP fetching
│   │   └── notes/              # Notes feature
│   │       ├── mod.rs
│   │       ├── models.rs       # Note struct
│   │       └── repository.rs   # Database operations
│   │
│   ├── database/               # Database layer
│   │   ├── mod.rs
│   │   ├── connection.rs       # Connection pooling
│   │   └── migrations.rs       # Schema migrations
│   │
│   └── utils/                  # Utilities
│       ├── mod.rs
│       └── time.rs             # Date/time helpers
│
├── docs/
├── AppConfig.json
└── Cargo.toml
```

### Alternative Structure (Layer-Based)
```
dry-dock/
├── src/
│   ├── main.rs                 # Entry point
│   ├── app.rs                  # App struct
│   │
│   ├── models/                 # Data models
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   ├── feed.rs
│   │   └── note.rs
│   │
│   ├── ui/                     # Presentation layer
│   │   ├── mod.rs
│   │   ├── menu.rs
│   │   ├── panels.rs
│   │   └── modals/
│   │       ├── mod.rs
│   │       ├── add_feed.rs
│   │       └── create_note.rs
│   │
│   ├── services/               # Business logic
│   │   ├── mod.rs
│   │   ├── feed_service.rs
│   │   └── note_service.rs
│   │
│   └── repositories/           # Data access
│       ├── mod.rs
│       ├── feed_repository.rs
│       └── note_repository.rs
│
└── Cargo.toml
```

**Recommendation:** Use **Feature-Based** for your project. It's more intuitive and scales better.

---

## Module Organization Strategies

### 1. Feature-Based (Recommended)
Group by feature/domain:
- `features/feeds/` - Everything related to RSS feeds
- `features/notes/` - Everything related to notes
- `ui/` - All UI components
- `modals/` - All modals

**Pros:**
- ✅ Easy to understand what goes where
- ✅ Features can be developed independently
- ✅ Clear domain boundaries
- ✅ Can easily remove/add features

**Cons:**
- ❌ Some code duplication possible

### 2. Layer-Based
Group by technical layer:
- `models/` - All data structures
- `ui/` - All UI code
- `services/` - All business logic
- `repositories/` - All database code

**Pros:**
- ✅ Clear separation of concerns
- ✅ Easy to swap implementations

**Cons:**
- ❌ Related code spread across directories
- ❌ Hard to see feature boundaries

### 3. Hybrid (Best for Growing Projects)
Mix both approaches:
```
src/
├── ui/              # Layer
├── features/        # Feature groups
│   ├── feeds/       # Feature
│   └── notes/
└── database/        # Layer
```

---

## Step-by-Step Migration

### Phase 1: Create Module Structure

```bash
# Create directories
mkdir -p src/ui
mkdir -p src/modals
mkdir -p src/features/feeds
mkdir -p src/features/notes
mkdir -p src/database
mkdir -p src/utils
```

### Phase 2: Create Module Files

```bash
# Create mod.rs files
touch src/ui/mod.rs
touch src/ui/menu.rs
touch src/ui/panels.rs
touch src/ui/styles.rs

touch src/modals/mod.rs
touch src/modals/add_feed.rs
touch src/modals/create_note.rs
touch src/modals/refresh_feeds.rs

touch src/features/mod.rs
touch src/features/feeds/mod.rs
touch src/features/notes/mod.rs

touch src/database/mod.rs
touch src/utils/mod.rs
touch src/utils/time.rs

touch src/app.rs
touch src/config.rs
```

### Phase 3: Move Code Incrementally

#### Step 1: Extract Config

**Create `src/config.rs`:**
```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Config {
    pub app_name: String,
    pub version: String,
    pub is_vsync_enabled: bool,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_data = fs::read_to_string("AppConfig.json")?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    }
}
```

**Update `src/main.rs`:**
```rust
mod config;
use config::Config;

fn main() -> Result<(), eframe::Error> {
    let config = Config::load()
        .expect("Unable to load config file");
    
    // ... rest of main
}
```

#### Step 2: Extract Modal Trait

**Create `src/modals/mod.rs`:**
```rust
use eframe::egui;

pub mod add_feed;
pub mod create_note;
pub mod refresh_feeds;

pub use add_feed::AddFeedModal;
pub use create_note::CreateNoteModal;
pub use refresh_feeds::RefreshFeedsModal;

/// Trait that all modals must implement
pub trait Modal {
    fn title(&self) -> &str;
    fn render(&mut self, ui: &mut egui::Ui) -> bool; // Returns true if should close
}

/// Enum to represent the active modal state
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ActiveModal {
    #[default]
    None,
    AddFeed,
    CreateNote,
    RefreshFeeds,
}
```

#### Step 3: Extract Individual Modals

**Create `src/modals/add_feed.rs`:**
```rust
use super::Modal;
use eframe::egui;

#[derive(Default)]
pub struct AddFeedModal {
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

impl AddFeedModal {
    pub fn clear(&mut self) {
        self.url.clear();
    }
}
```

**Create `src/modals/create_note.rs`:**
```rust
use super::Modal;
use eframe::egui;
use chrono::Utc;

#[derive(Default)]
pub struct CreateNoteModal {
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
        ui.add_space(5.0);

        ui.label("Note Details:");
        ui.text_edit_multiline(&mut self.details);
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            if ui.button("Create").clicked() {
                println!("Creating Note...");
                println!("Title: {}", self.title);
                println!("Details: {}", self.details);
                println!("Note Date: {}", Utc::now().to_rfc3339());
                should_close = true;
            }
            if ui.button("Cancel").clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
}

impl CreateNoteModal {
    pub fn clear(&mut self) {
        self.title.clear();
        self.details.clear();
    }
}
```

**Create `src/modals/refresh_feeds.rs`:**
```rust
use super::Modal;
use eframe::egui;

#[derive(Default)]
pub struct RefreshFeedsModal;

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
```

#### Step 4: Extract UI Components

**Create `src/ui/mod.rs`:**
```rust
pub mod menu;
pub mod panels;
pub mod styles;

pub use menu::render_menu;
pub use panels::{render_central_panel, render_bottom_panel};
```

**Create `src/ui/menu.rs`:**
```rust
use eframe::egui;
use crate::config::Config;
use crate::modals::ActiveModal;

pub fn render_menu(ui: &mut egui::Ui, config: &Config, active_modal: &mut ActiveModal) {
    // Load Menu Styling
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Button, 
        egui::FontId::new(24.0, egui::FontFamily::Proportional)
    );

    ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);

    // App Menu
    ui.menu_button(config.app_name.as_str(), |ui| {
        let exit_button = egui::Button::new("Exit")
            .min_size(egui::vec2(100.0, 30.0));
        
        if ui.add(exit_button).clicked() {
            std::process::exit(0);
        }
    });

    ui.separator();

    // RSS Menu
    ui.menu_button("RSS", |ui| {
        let fetch_feeds_button = egui::Button::new("Force Refresh Feeds")
            .min_size(egui::vec2(100.0, 30.0));

        let add_feed_button = egui::Button::new("Add New Feed")
            .min_size(egui::vec2(100.0, 30.0));

        if ui.add(add_feed_button).clicked() {
            *active_modal = ActiveModal::AddFeed;
        }

        if ui.add(fetch_feeds_button).clicked() {
            *active_modal = ActiveModal::RefreshFeeds;
        }
    });

    ui.separator();

    // Notes Menu
    ui.menu_button("Notes", |ui| {
        let new_note_button = egui::Button::new("Create New Note")
            .min_size(egui::vec2(100.0, 30.0));
        
        if ui.add(new_note_button).clicked() {
            *active_modal = ActiveModal::CreateNote;
        }
    });
}
```

**Create `src/ui/panels.rs`:**
```rust
use eframe::egui;
use crate::config::Config;

pub fn render_central_panel(ui: &mut egui::Ui, config: &Config) {
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(18.0, egui::FontFamily::Proportional)
    );

    ui.spacing_mut().item_spacing = egui::vec2(10.0, 10.0);

    if ui.button("Click me").clicked() {
        println!("App Name: {}", config.app_name);
        println!("Button clicked!");
    }
}

pub fn render_bottom_panel(ui: &mut egui::Ui, config: &Config) {
    use crate::utils::time::current_year;
    
    ui.horizontal_centered(|ui| {
        ui.label(format!("Version: {}", config.version));
        ui.separator();
        ui.label(format!("© {} Dry Dock. All rights reserved.", current_year()));
    });
}
```

**Create `src/ui/styles.rs`:**
```rust
use eframe::egui;

/// Apply consistent button styling
pub fn style_button(ui: &mut egui::Ui) {
    ui.style_mut().text_styles.insert(
        egui::TextStyle::Button, 
        egui::FontId::new(24.0, egui::FontFamily::Proportional)
    );
    ui.spacing_mut().button_padding = egui::vec2(10.0, 10.0);
}
```

#### Step 5: Extract Utils

**Create `src/utils/mod.rs`:**
```rust
pub mod time;
```

**Create `src/utils/time.rs`:**
```rust
use chrono::Datelike;

pub fn current_year() -> String {
    chrono::Utc::now().year().to_string()
}
```

#### Step 6: Extract App Logic

**Create `src/app.rs`:**
```rust
use eframe::egui;
use crate::config::Config;
use crate::modals::{ActiveModal, Modal, AddFeedModal, CreateNoteModal, RefreshFeedsModal};
use crate::ui;

const AUTO_SAVE_INTERVAL_SECS: u64 = 60;

pub struct MyApp {
    config: Config,
    active_modal: ActiveModal,
    
    // Modal instances
    add_feed_modal: AddFeedModal,
    create_note_modal: CreateNoteModal,
    refresh_feeds_modal: RefreshFeedsModal,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self { 
            config,
            active_modal: ActiveModal::None,
            add_feed_modal: AddFeedModal::default(),
            create_note_modal: CreateNoteModal::default(),
            refresh_feeds_modal: RefreshFeedsModal::default(),
        }
    }
    
    fn render_active_modal(&mut self, ctx: &egui::Context) {
        if self.active_modal == ActiveModal::None {
            return;
        }

        // Dim background
        egui::Area::new("modal_overlay".into())
            .fixed_pos(egui::pos2(0.0, 0.0))
            .show(ctx, |ui| {
                let screen_rect = ctx.screen_rect();
                ui.painter().rect_filled(
                    screen_rect,
                    0.0,
                    egui::Color32::from_black_alpha(200),
                );
            });

        // Render appropriate modal
        let should_close = match &mut self.active_modal {
            ActiveModal::AddFeed => {
                self.render_modal_window(ctx, &mut self.add_feed_modal)
            }
            ActiveModal::CreateNote => {
                self.render_modal_window(ctx, &mut self.create_note_modal)
            }
            ActiveModal::RefreshFeeds => {
                self.render_modal_window(ctx, &mut self.refresh_feeds_modal)
            }
            ActiveModal::None => false,
        };

        if should_close {
            self.active_modal = ActiveModal::None;
        }
    }
    
    fn render_modal_window<M: Modal>(&mut self, ctx: &egui::Context, modal: &mut M) -> bool {
        let mut should_close = false;
        
        egui::Window::new(modal.title())
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                should_close = modal.render(ui);
            });
        
        should_close
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui::render_menu(ui, &self.config, &mut self.active_modal);
            });
        });

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::render_central_panel(ui, &self.config);
        });

        // Bottom panel
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui::render_bottom_panel(ui, &self.config);
        });

        // Modals
        self.render_active_modal(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Cleanup
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Save state
    }

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(AUTO_SAVE_INTERVAL_SECS)
    }
}
```

#### Step 7: Simplify main.rs

**Update `src/main.rs`:**
```rust
mod app;
mod config;
mod modals;
mod ui;
mod utils;

use app::MyApp;
use config::Config;

fn main() -> Result<(), eframe::Error> {
    let config = Config::load()
        .expect("Unable to load config file");

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false)
            .with_fullscreen(true),
        vsync: config.is_vsync_enabled,
        persist_window: true,
        ..Default::default()
    };
    
    eframe::run_native(
        &format!("{} v{}", config.app_name, config.version),
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc, config.clone())))),
    )
}
```

---

## Rust Module System Basics

### Module Declaration

```rust
// In src/main.rs or src/lib.rs
mod config;           // Looks for src/config.rs
mod ui;              // Looks for src/ui/mod.rs or src/ui.rs
mod modals;          // Looks for src/modals/mod.rs

// Use items from modules
use config::Config;
use modals::ActiveModal;
```

### Module File Structure

```rust
// Option 1: Single file module
// src/config.rs
pub struct Config { }

// Option 2: Directory module
// src/ui/mod.rs
pub mod menu;    // Looks for src/ui/menu.rs
pub mod panels;  // Looks for src/ui/panels.rs

// Re-export for convenience
pub use menu::render_menu;
```

### Visibility Rules

```rust
// Private by default
struct PrivateStruct { }

// Public - accessible from outside module
pub struct PublicStruct { }

// Public within crate only
pub(crate) struct CratePublicStruct { }

// Public within parent module
pub(super) struct ParentPublicStruct { }
```

### Importing

```rust
// Absolute path from crate root
use crate::config::Config;

// Relative path
use super::Modal;  // Parent module
use self::inner;   // Current module

// Multiple items
use crate::modals::{Modal, ActiveModal, AddFeedModal};

// Rename
use crate::config::Config as AppConfig;

// Glob import (use sparingly)
use crate::modals::*;
```

---

## Best Practices

### 1. Keep main.rs Minimal

```rust
// main.rs should only:
// 1. Declare modules
// 2. Initialize application
// 3. Handle command-line args (if any)

mod app;
mod config;
// ...

fn main() -> Result<(), eframe::Error> {
    let app = MyApp::initialize()?;
    app.run()
}
```

### 2. Use mod.rs as Module Index

```rust
// src/modals/mod.rs

pub mod add_feed;
pub mod create_note;

// Re-export commonly used items
pub use add_feed::AddFeedModal;
pub use create_note::CreateNoteModal;

// Define shared types in mod.rs
pub trait Modal {
    fn render(&mut self, ui: &mut egui::Ui) -> bool;
}
```

### 3. Group Related Functionality

```rust
// Good: Feature-based
src/
├── features/
│   └── feeds/
│       ├── models.rs      # Feed, FeedItem
│       ├── repository.rs  # Database operations
│       └── service.rs     # Business logic

// Bad: Scattered
src/
├── models.rs      # All models mixed
├── database.rs    # All database code mixed
└── logic.rs       # All logic mixed
```

### 4. Avoid Deep Nesting

```rust
// Good: Max 3 levels
src/features/feeds/models.rs

// Bad: Too deep
src/features/rss/feeds/data/models/feed.rs
```

### 5. Make Modules Self-Contained

```rust
// Each module should have its own types and logic
// src/features/feeds/mod.rs

mod models;
mod repository;
mod service;

pub use models::Feed;
pub use service::FeedService;

// Private - internal to this module
use repository::FeedRepository;
```

### 6. Use Prelude Pattern (Optional)

```rust
// src/prelude.rs
pub use crate::config::Config;
pub use crate::modals::{Modal, ActiveModal};
pub use crate::app::MyApp;

// In other files
use crate::prelude::*;
```

---

## Complete Example

### Final File Structure

```
dry-dock/
├── src/
│   ├── main.rs              # 20 lines - entry point
│   ├── app.rs               # 100 lines - main app
│   ├── config.rs            # 30 lines - config
│   │
│   ├── ui/
│   │   ├── mod.rs           # 5 lines - exports
│   │   ├── menu.rs          # 60 lines
│   │   ├── panels.rs        # 40 lines
│   │   └── styles.rs        # 20 lines
│   │
│   ├── modals/
│   │   ├── mod.rs           # 20 lines - trait + enum
│   │   ├── add_feed.rs      # 40 lines
│   │   ├── create_note.rs   # 50 lines
│   │   └── refresh_feeds.rs # 30 lines
│   │
│   └── utils/
│       ├── mod.rs           # 3 lines
│       └── time.rs          # 10 lines
│
├── docs/
├── AppConfig.json
└── Cargo.toml

Total: ~428 lines across 14 files
Previous: ~400 lines in 1 file
```

### Migration Checklist

- [ ] Create directory structure
- [ ] Create all `mod.rs` files
- [ ] Extract `config.rs` and test
- [ ] Extract `modals/` module and test
- [ ] Extract `ui/` module and test
- [ ] Extract `utils/` module and test
- [ ] Move app logic to `app.rs`
- [ ] Simplify `main.rs`
- [ ] Run `cargo build` to verify
- [ ] Run `cargo test` to verify
- [ ] Run application to verify UI works
- [ ] Delete old commented code
- [ ] Update documentation

---

## Testing Strategy

### Before Migration
```bash
# Run app to ensure it works
cargo run

# Note any issues
```

### During Migration
```bash
# After each step
cargo build
cargo clippy
cargo fmt

# Test the app still runs
cargo run
```

### After Migration
```bash
# Full check
cargo clean
cargo build --release
cargo test
cargo run
```

---

## Common Issues & Solutions

### Issue 1: "Module not found"
```rust
// Solution: Check file paths match module declarations
mod ui;  // Needs src/ui.rs OR src/ui/mod.rs
```

### Issue 2: "Private type in public interface"
```rust
// Solution: Make types public
pub struct Config { }  // Add 'pub'
```

### Issue 3: "Circular dependencies"
```rust
// Solution: Use trait objects or extract shared types
// Create src/types.rs for shared types
```

### Issue 4: "Use of moved value"
```rust
// Solution: Use references or clone
fn render(&self, config: &Config)  // Reference, not ownership
```

---

## Benefits Summary

| Aspect | Before | After |
|--------|--------|-------|
| **Navigation** | Scroll through 400 lines | Jump to specific file |
| **Testing** | Hard to isolate | Test each module |
| **Reusability** | Copy-paste code | Import modules |
| **Collaboration** | Merge conflicts | Separate files |
| **Maintainability** | Find in wall of text | Clear organization |
| **Scalability** | Adds to main.rs | New files/modules |

---

## Next Steps

1. **Start Small** - Extract config first (easiest)
2. **Test Often** - After each extraction
3. **Commit Frequently** - One module per commit
4. **Document** - Add module-level comments
5. **Refactor Later** - Get it working first, optimize second

---

## Additional Resources

- [Rust Module System](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Project Structure Examples](https://github.com/rust-unofficial/patterns/blob/main/patterns/structural/mod.md)

---

Good luck with your migration! Start with extracting the config, then modals, then UI components. You'll see immediate benefits! 🚀
