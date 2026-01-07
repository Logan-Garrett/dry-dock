# eframe Popups & Windows Guide

## Table of Contents
- [Overview](#overview)
- [Window Types](#window-types)
- [Creating Basic Windows](#creating-basic-windows)
- [Closing Windows](#closing-windows)
- [Modal Dialogs](#modal-dialogs)
- [Context Menus](#context-menus)
- [Tooltips](#tooltips)
- [Complete Examples](#complete-examples)
- [Best Practices](#best-practices)

---

## Overview

In eframe/egui, there are several types of overlay UI elements you can create:

1. **Windows** - Draggable, closeable floating windows
2. **Modal Dialogs** - Blocking popups that require user action
3. **Context Menus** - Right-click menus
4. **Tooltips** - Hover information
5. **Popups** - Area-based temporary UI elements

All of these are created **every frame** in your `update()` method using immediate mode patterns.

---

## Window Types

### Basic Window
Floating, draggable window that can be minimized/closed:

```rust
egui::Window::new("My Window")
    .show(ctx, |ui| {
        ui.label("Window content");
    });
```

### Closeable Window
Window with an X button that user can close:

```rust
struct MyApp {
    show_window: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Closeable Window")
            .open(&mut self.show_window)  // Adds close button
            .show(ctx, |ui| {
                ui.label("This window can be closed!");
            });
    }
}
```

### Fixed Size Window
Window that cannot be resized:

```rust
egui::Window::new("Fixed Size")
    .resizable(false)
    .default_size([300.0, 200.0])
    .show(ctx, |ui| {
        ui.label("Cannot resize this window");
    });
```

### Window with All Options
```rust
egui::Window::new("Advanced Window")
    .open(&mut self.show_window)           // Add close button
    .resizable(true)                        // Allow resizing
    .collapsible(true)                      // Allow minimizing
    .title_bar(true)                        // Show title bar
    .scroll(true)                           // Enable scrolling if content overflows
    .default_width(400.0)                   // Initial width
    .default_height(300.0)                  // Initial height
    .min_width(200.0)                       // Minimum width
    .max_width(800.0)                       // Maximum width
    .default_pos([100.0, 100.0])           // Initial position
    .fixed_pos([100.0, 100.0])             // Force position (not draggable)
    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])  // Anchor to screen position
    .show(ctx, |ui| {
        ui.heading("Window Content");
        ui.label("This has all the options!");
    });
```

---

## Creating Basic Windows

### Example 1: Toggle Window

```rust
struct MyApp {
    show_settings: bool,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            show_settings: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Main UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Main Window");
            
            // Button to show settings window
            if ui.button("Open Settings").clicked() {
                self.show_settings = true;
            }
        });
        
        // Settings window (only shown if show_settings is true)
        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)  // User can close with X
                .show(ctx, |ui| {
                    ui.label("Setting 1");
                    ui.label("Setting 2");
                    
                    // Manual close button
                    if ui.button("Close").clicked() {
                        self.show_settings = false;
                    }
                });
        }
    }
}
```

### Example 2: Multiple Windows

```rust
struct MyApp {
    show_settings: bool,
    show_about: bool,
    show_help: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Window", |ui| {
                    if ui.button("Settings").clicked() {
                        self.show_settings = true;
                    }
                    if ui.button("About").clicked() {
                        self.show_about = true;
                    }
                    if ui.button("Help").clicked() {
                        self.show_help = true;
                    }
                });
            });
        });
        
        // Settings window
        if self.show_settings {
            egui::Window::new("Settings")
                .open(&mut self.show_settings)
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.label("Settings content");
                });
        }
        
        // About window
        if self.show_about {
            egui::Window::new("About")
                .open(&mut self.show_about)
                .resizable(false)
                .collapsible(false)
                .default_width(250.0)
                .show(ctx, |ui| {
                    ui.label("My App v1.0");
                    ui.label("© 2026");
                });
        }
        
        // Help window
        if self.show_help {
            egui::Window::new("Help")
                .open(&mut self.show_help)
                .default_size([400.0, 500.0])
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("Help documentation goes here...");
                    });
                });
        }
        
        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Main Content");
        });
    }
}
```

---

## Closing Windows

### Method 1: Using `open` Parameter (Recommended)

The `open` parameter automatically adds an X button and handles closing:

```rust
struct MyApp {
    show_window: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("My Window")
            .open(&mut self.show_window)  // User clicks X → sets to false
            .show(ctx, |ui| {
                ui.label("Content");
            });
    }
}
```

### Method 2: Manual Close Button

```rust
if self.show_window {
    egui::Window::new("My Window")
        .show(ctx, |ui| {
            ui.label("Content");
            
            if ui.button("Close").clicked() {
                self.show_window = false;
            }
        });
}
```

### Method 3: Conditional Rendering

Only show the window when the boolean is true:

```rust
// Window only exists when show_window is true
if self.show_window {
    egui::Window::new("My Window")
        .show(ctx, |ui| {
            ui.label("This window has no X button");
            ui.label("Use the button below to close");
            
            if ui.button("Close Window").clicked() {
                self.show_window = false;
            }
        });
}
```

### Method 4: Detect Window Close

Check if the window was closed by inspecting the return value:

```rust
let response = egui::Window::new("My Window")
    .open(&mut self.show_window)
    .show(ctx, |ui| {
        ui.label("Content");
    });

// Check if window was closed
if let Some(inner_response) = response {
    // Window is still open
    if !self.show_window {
        println!("Window was just closed!");
    }
}
```

### Method 5: ESC Key to Close

```rust
if self.show_window {
    // Check for ESC key
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        self.show_window = false;
    }
    
    egui::Window::new("My Window")
        .open(&mut self.show_window)
        .show(ctx, |ui| {
            ui.label("Press ESC to close");
        });
}
```

---

## Modal Dialogs

Modal dialogs block interaction with the main UI until closed. In egui, this is achieved using area overlays and dimming.

### Simple Modal Dialog

```rust
struct MyApp {
    show_modal: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Main UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Main Content");
            
            if ui.button("Show Modal").clicked() {
                self.show_modal = true;
            }
        });
        
        // Modal dialog
        if self.show_modal {
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
            
            // Modal window
            egui::Window::new("Confirm Action")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("Are you sure you want to continue?");
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("Yes").clicked() {
                            self.show_modal = false;
                            // Perform action
                        }
                        if ui.button("No").clicked() {
                            self.show_modal = false;
                        }
                    });
                });
        }
    }
}
```

### Modal with Result

```rust
struct MyApp {
    show_modal: bool,
    modal_result: Option<bool>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Delete File").clicked() {
                self.show_modal = true;
                self.modal_result = None;
            }
            
            // Handle result
            if let Some(confirmed) = self.modal_result {
                if confirmed {
                    ui.label("✓ File deleted");
                } else {
                    ui.label("✗ Cancelled");
                }
                self.modal_result = None;
            }
        });
        
        if self.show_modal {
            // Dimmed background
            egui::Area::new("dim".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    let rect = ctx.screen_rect();
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        egui::Color32::from_black_alpha(180),
                    );
                });
            
            egui::Window::new("⚠ Warning")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.label("This action cannot be undone.");
                    ui.label("Delete this file?");
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("Delete").clicked() {
                            self.modal_result = Some(true);
                            self.show_modal = false;
                        }
                        if ui.button("Cancel").clicked() {
                            self.modal_result = Some(false);
                            self.show_modal = false;
                        }
                    });
                });
        }
    }
}
```

### Blocking Modal (Cannot Close Without Action)

```rust
if self.show_error_modal {
    // Dim and block background
    egui::Area::new("error_overlay".into())
        .fixed_pos(egui::pos2(0.0, 0.0))
        .interactable(true)  // Captures clicks
        .show(ctx, |ui| {
            let rect = ctx.screen_rect();
            ui.painter().rect_filled(
                rect,
                0.0,
                egui::Color32::from_black_alpha(220),
            );
        });
    
    // Error modal (no close button, must click OK)
    egui::Window::new("❌ Error")
        .collapsible(false)
        .resizable(false)
        .title_bar(true)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.label("An error occurred!");
            ui.label(self.error_message.as_str());
            ui.add_space(10.0);
            
            // Only way to close is clicking OK
            if ui.button("OK").clicked() {
                self.show_error_modal = false;
            }
        });
}
```

---

## Context Menus

Context menus appear on right-click or button press.

### Right-Click Context Menu

```rust
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Right-click anywhere for menu");
            
            // Detect right-click
            ui.interact(
                ui.max_rect(),
                ui.id(),
                egui::Sense::click(),
            ).context_menu(|ui| {
                if ui.button("Copy").clicked() {
                    println!("Copy");
                    ui.close_menu();
                }
                if ui.button("Paste").clicked() {
                    println!("Paste");
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Delete").clicked() {
                    println!("Delete");
                    ui.close_menu();
                }
            });
        });
    }
}
```

### Context Menu on Specific Widget

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    for (i, item) in self.items.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.label(item);
        }).context_menu(|ui| {
            if ui.button("Edit").clicked() {
                self.edit_item(i);
                ui.close_menu();
            }
            if ui.button("Delete").clicked() {
                self.delete_item(i);
                ui.close_menu();
            }
            if ui.button("Duplicate").clicked() {
                self.duplicate_item(i);
                ui.close_menu();
            }
        });
    }
});
```

### Button-Triggered Popup Menu

```rust
struct MyApp {
    show_menu: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let button = ui.button("☰ Menu");
            
            if button.clicked() {
                self.show_menu = !self.show_menu;
            }
            
            if self.show_menu {
                let button_rect = button.rect;
                egui::Area::new("dropdown_menu".into())
                    .fixed_pos(button_rect.left_bottom())
                    .show(ctx, |ui| {
                        egui::Frame::popup(&ctx.style()).show(ui, |ui| {
                            ui.set_min_width(150.0);
                            
                            if ui.button("Option 1").clicked() {
                                self.show_menu = false;
                            }
                            if ui.button("Option 2").clicked() {
                                self.show_menu = false;
                            }
                            if ui.button("Option 3").clicked() {
                                self.show_menu = false;
                            }
                        });
                    });
                
                // Close menu if clicked outside
                if ctx.input(|i| i.pointer.any_click()) {
                    // Check if click was outside menu
                    self.show_menu = false;
                }
            }
        });
    }
}
```

---

## Tooltips

Tooltips show information on hover.

### Basic Tooltip

```rust
ui.label("Hover me").on_hover_text("This is a tooltip!");
```

### Rich Tooltip with Formatting

```rust
ui.label("Hover for details").on_hover_ui(|ui| {
    ui.heading("Detailed Information");
    ui.label("This tooltip can contain:");
    ui.label("• Multiple lines");
    ui.label("• Rich formatting");
    ui.separator();
    ui.label(egui::RichText::new("Colored text").color(egui::Color32::RED));
});
```

### Tooltip at Pointer

```rust
ui.button("Help").on_hover_text_at_pointer("This appears at cursor");
```

### Custom Tooltip Position

```rust
let response = ui.button("Hover me");

if response.hovered() {
    egui::show_tooltip_at_pointer(ctx, response.id.with("tooltip"), |ui| {
        ui.label("Custom positioned tooltip");
    });
}
```

---

## Complete Examples

### Example 1: Settings Dialog

```rust
use eframe::egui;

struct MyApp {
    show_settings: bool,
    username: String,
    enable_notifications: bool,
    volume: f32,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            show_settings: false,
            username: "User".to_string(),
            enable_notifications: true,
            volume: 0.5,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Application");
            
            if ui.button("⚙ Settings").clicked() {
                self.show_settings = true;
            }
            
            ui.separator();
            ui.label(format!("Current user: {}", self.username));
            ui.label(format!("Notifications: {}", 
                if self.enable_notifications { "On" } else { "Off" }));
            ui.label(format!("Volume: {:.0}%", self.volume * 100.0));
        });
        
        // Settings window
        if self.show_settings {
            egui::Window::new("⚙ Settings")
                .open(&mut self.show_settings)
                .resizable(false)
                .collapsible(false)
                .default_width(350.0)
                .show(ctx, |ui| {
                    ui.heading("User Settings");
                    ui.separator();
                    
                    egui::Grid::new("settings_grid")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            ui.label("Username:");
                            ui.text_edit_singleline(&mut self.username);
                            ui.end_row();
                            
                            ui.label("Notifications:");
                            ui.checkbox(&mut self.enable_notifications, "");
                            ui.end_row();
                            
                            ui.label("Volume:");
                            ui.add(egui::Slider::new(&mut self.volume, 0.0..=1.0)
                                .show_value(false));
                            ui.end_row();
                        });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            // Save settings
                            self.show_settings = false;
                        }
                        
                        if ui.button("Cancel").clicked() {
                            // Revert changes
                            self.show_settings = false;
                        }
                    });
                });
        }
    }
}
```

### Example 2: Confirmation Modal

```rust
use eframe::egui;

struct MyApp {
    items: Vec<String>,
    show_delete_confirm: bool,
    item_to_delete: Option<usize>,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            items: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ],
            show_delete_confirm: false,
            item_to_delete: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Items");
            
            let mut clicked_delete = None;
            
            for (i, item) in self.items.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(item);
                    if ui.button("🗑 Delete").clicked() {
                        clicked_delete = Some(i);
                    }
                });
            }
            
            // Trigger confirmation modal
            if let Some(idx) = clicked_delete {
                self.item_to_delete = Some(idx);
                self.show_delete_confirm = true;
            }
        });
        
        // Confirmation modal
        if self.show_delete_confirm {
            // Dim background
            egui::Area::new("modal_bg".into())
                .fixed_pos(egui::pos2(0.0, 0.0))
                .show(ctx, |ui| {
                    let rect = ctx.screen_rect();
                    ui.painter().rect_filled(
                        rect,
                        0.0,
                        egui::Color32::from_black_alpha(200),
                    );
                });
            
            // Modal dialog
            egui::Window::new("⚠ Confirm Delete")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    if let Some(idx) = self.item_to_delete {
                        ui.label(format!("Delete '{}'?", self.items[idx]));
                        ui.label("This action cannot be undone.");
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            if ui.button("✓ Delete").clicked() {
                                self.items.remove(idx);
                                self.show_delete_confirm = false;
                                self.item_to_delete = None;
                            }
                            
                            if ui.button("✗ Cancel").clicked() {
                                self.show_delete_confirm = false;
                                self.item_to_delete = None;
                            }
                        });
                    }
                });
        }
    }
}
```

### Example 3: Multi-Window Application

```rust
use eframe::egui;

struct MyApp {
    windows: Windows,
}

struct Windows {
    show_palette: bool,
    show_layers: bool,
    show_properties: bool,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            windows: Windows {
                show_palette: true,
                show_layers: true,
                show_properties: false,
            },
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Window", |ui| {
                    ui.checkbox(&mut self.windows.show_palette, "Color Palette");
                    ui.checkbox(&mut self.windows.show_layers, "Layers");
                    ui.checkbox(&mut self.windows.show_properties, "Properties");
                });
            });
        });
        
        // Main canvas
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Canvas Area");
            ui.label("Main content goes here");
        });
        
        // Palette window
        if self.windows.show_palette {
            egui::Window::new("🎨 Color Palette")
                .open(&mut self.windows.show_palette)
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.color_edit_button_rgb(&mut [1.0, 0.0, 0.0]);
                        ui.color_edit_button_rgb(&mut [0.0, 1.0, 0.0]);
                        ui.color_edit_button_rgb(&mut [0.0, 0.0, 1.0]);
                    });
                });
        }
        
        // Layers window
        if self.windows.show_layers {
            egui::Window::new("📚 Layers")
                .open(&mut self.windows.show_layers)
                .default_width(200.0)
                .show(ctx, |ui| {
                    ui.label("Layer 1");
                    ui.label("Layer 2");
                    ui.label("Background");
                });
        }
        
        // Properties window
        if self.windows.show_properties {
            egui::Window::new("⚙ Properties")
                .open(&mut self.windows.show_properties)
                .default_width(250.0)
                .show(ctx, |ui| {
                    egui::Grid::new("props").show(ui, |ui| {
                        ui.label("Width:");
                        ui.label("800");
                        ui.end_row();
                        
                        ui.label("Height:");
                        ui.label("600");
                        ui.end_row();
                    });
                });
        }
    }
}
```

---

## Best Practices

### 1. Use Boolean Flags for Window State

```rust
struct MyApp {
    show_settings: bool,
    show_about: bool,
    show_help: bool,
}
```

### 2. Prefer `open()` for User-Closeable Windows

```rust
egui::Window::new("My Window")
    .open(&mut self.show_window)  // Adds X button automatically
    .show(ctx, |ui| {
        // Content
    });
```

### 3. Wrap Modals in Conditional

```rust
if self.show_modal {
    // Dim background
    // Show window
}
```

### 4. Center Important Dialogs

```rust
egui::Window::new("Important")
    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
    .show(ctx, |ui| {
        // Content
    });
```

### 5. Disable Resizing for Dialogs

```rust
egui::Window::new("Confirmation")
    .resizable(false)
    .collapsible(false)
    .show(ctx, |ui| {
        // Content
    });
```

### 6. Close Menus After Action

```rust
if ui.button("Action").clicked() {
    self.perform_action();
    self.show_menu = false;  // Close menu
}
```

### 7. Use Tooltips for Help

```rust
ui.button("?")
    .on_hover_text("Click for more information");
```

### 8. Store Window State

```rust
#[derive(serde::Deserialize, serde::Serialize)]
struct MyApp {
    show_settings: bool,
    show_sidebar: bool,
}

impl eframe::App for MyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "show_settings", &self.show_settings);
        eframe::set_value(storage, "show_sidebar", &self.show_sidebar);
    }
}
```

### 9. Prevent Multiple Instances

```rust
// Only show one instance at a time
if !self.show_settings {
    if ui.button("Settings").clicked() {
        self.show_settings = true;
    }
}
```

### 10. Handle ESC Key for Modals

```rust
if self.show_modal {
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        self.show_modal = false;
    }
    
    // Show modal window
}
```

---

## Summary

### Creating Windows
```rust
egui::Window::new("Title")
    .open(&mut bool_flag)
    .show(ctx, |ui| { /* content */ });
```

### Closing Windows
- Add `.open(&mut bool)` for X button
- Set `bool_flag = false` manually
- User clicks X → egui sets bool to false
- Wrap in `if bool_flag { }` to conditionally show

### Modals
- Dim background with `egui::Area` + semi-transparent rect
- Center window with `.anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])`
- Disable resize/collapse: `.resizable(false).collapsible(false)`

### Context Menus
- Use `.context_menu(|ui| { })` on any response
- Call `ui.close_menu()` after action

### Tooltips
- `.on_hover_text("text")` for simple tooltips
- `.on_hover_ui(|ui| { })` for rich tooltips

---

## Additional Resources

- [egui Window API](https://docs.rs/egui/latest/egui/struct.Window.html)
- [egui Area API](https://docs.rs/egui/latest/egui/containers/struct.Area.html)
- [egui Examples](https://github.com/emilk/egui/tree/master/examples)
- [Live Demo](https://www.egui.rs/)

Happy windowing! 🪟
