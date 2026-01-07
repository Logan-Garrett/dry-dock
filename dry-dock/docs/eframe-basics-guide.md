# eframe Basics: A Beginner's Guide

## Table of Contents
1. [What is eframe?](#what-is-eframe)
2. [Key Concepts](#key-concepts)
3. [Basic Structure](#basic-structure)
4. [The App Trait](#the-app-trait)
5. [NativeOptions](#nativeoptions)
6. [CreationContext](#creationcontext)
7. [The Update Method](#the-update-method)
8. [Frame Object](#frame-object)
9. [UI Layouts & Widgets](#ui-layouts--widgets)
10. [State Management](#state-management)
11. [Complete Example](#complete-example)
12. [Common Patterns](#common-patterns)
13. [Next Steps](#next-steps)

---

## What is eframe?

**eframe** (egui framework) is the official application framework for egui. While **egui** is the immediate-mode GUI library that provides widgets and drawing capabilities, **eframe** handles:

- Window creation and management
- Event loops (input handling, rendering)
- Cross-platform support (Windows, macOS, Linux, Web)
- Integration with rendering backends (glow, wgpu)
- State persistence between sessions

Think of it this way:
- **egui** = the UI toolkit (buttons, labels, text inputs)
- **eframe** = the application framework (runs your app, manages the window)

---

## Key Concepts

### Immediate Mode GUI
eframe/egui uses **immediate mode**, meaning:
- Your UI code runs **every frame** (60 times per second)
- You don't store widget objects - you create them on-demand
- No callbacks - check button clicks inline with `if ui.button("Click").clicked() { }`
- State is stored in your app struct, not in the GUI

### The App Lifecycle
1. **Creation** - Your app is instantiated once via `new()`
2. **Update Loop** - `update()` is called every frame (60 FPS)
3. **Shutdown** - Optional cleanup via `on_exit()`

---

## Basic Structure

Every eframe app follows this pattern:

```rust
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // 1. Configure window options
    let native_options = eframe::NativeOptions::default();
    
    // 2. Run the app
    eframe::run_native(
        "My App Name",           // Window title
        native_options,          // Window configuration
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),  // App creator
    )
}

// 3. Define your app struct
struct MyApp {
    name: String,
    age: u32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup code here
        Self {
            name: "World".to_owned(),
            age: 30,
        }
    }
}

// 4. Implement the App trait
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Your GUI code goes here - runs every frame
    }
}
```

---

## The App Trait

The `eframe::App` trait is the core of your application. You must implement it:

```rust
pub trait App {
    /// Called each time the UI needs repainting (every frame, ~60 FPS)
    /// This is where you build your user interface
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    
    /// Optional: Called once before shutdown
    fn on_exit(&mut self, gl: Option<&glow::Context>) {}
    
    /// Optional: Save state before shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {}
    
    /// Optional: Auto-save interval (None = never auto-save)
    fn auto_save_interval(&self) -> Duration {
        Duration::from_secs(30)
    }
    
    /// Optional: Customize when app should repaint
    fn persist_window(&self) -> bool { true }
}
```

**Most important:** You must implement `update()` - everything else is optional.

---

## NativeOptions

`NativeOptions` configures your application window:

```rust
let native_options = eframe::NativeOptions {
    // Initial window size (in logical pixels)
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_min_inner_size([400.0, 300.0])
        .with_max_inner_size([1920.0, 1080.0])
        .with_resizable(true)
        .with_fullscreen(false)
        .with_maximized(false)
        .with_transparent(false)
        .with_decorations(true)  // Window title bar and borders
        .with_icon(icon_data),    // Custom window icon
    
    // Enable or disable vsync (vertical sync)
    vsync: true,
    
    // Multisampling anti-aliasing (0, 2, 4, 8, or 16)
    multisampling: 0,
    
    // Hardware acceleration preference
    hardware_acceleration: eframe::HardwareAcceleration::Preferred,
    
    // Rendering backend (Glow or Wgpu)
    renderer: eframe::Renderer::Glow,
    
    // Enable persistence (saving app state)
    persist_window: true,
    
    // Follow system theme (light/dark)
    follow_system_theme: true,
    
    // Default theme if not following system
    default_theme: eframe::Theme::Dark,
    
    // Where to store app state
    // Default: platform-specific (e.g., ~/.local/share/your-app on Linux)
    ..Default::default()
};
```

**Common configurations:**

```rust
// Minimal window
let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([400.0, 300.0]),
    ..Default::default()
};

// Fullscreen app
let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_fullscreen(true),
    ..Default::default()
};

// High-DPI, high-quality rendering
let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_inner_size([1280.0, 720.0]),
    multisampling: 4,
    vsync: true,
    ..Default::default()
};
```

---

## CreationContext

The `CreationContext` is passed to your app's constructor and provides one-time setup access:

```rust
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 1. Customize visual style
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        
        // 2. Install custom fonts
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
        );
        cc.egui_ctx.set_fonts(fonts);
        
        // 3. Restore previous state (requires "persistence" feature)
        let mut name = "World".to_string();
        if let Some(storage) = cc.storage {
            name = eframe::get_value(storage, "name").unwrap_or(name);
        }
        
        // 4. Access rendering context (for custom 3D rendering)
        #[cfg(feature = "glow")]
        if let Some(gl) = &cc.gl {
            // Initialize custom OpenGL resources
        }
        
        Self { name }
    }
}
```

**Key fields:**
- `egui_ctx: &egui::Context` - The egui context for setup
- `storage: Option<&dyn Storage>` - Load saved state
- `gl: Option<Arc<glow::Context>>` - OpenGL context (if using glow)
- `wgpu_render_state: Option<&RenderState>` - WGPU state (if using wgpu)

---

## The Update Method

The `update()` method is called every frame (~60 FPS). This is where you build your UI:

```rust
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // ctx = egui context (controls repaint, input, style, etc.)
        // frame = window information (size, close requests, etc.)
        
        // Create a central panel (fills the whole window)
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui = User Interface builder
            // Add widgets here
            ui.heading("My Application");
            ui.label("Hello, World!");
        });
    }
}
```

**Parameters:**
- `&mut self` - Your app state (mutable)
- `ctx: &egui::Context` - egui context for the frame
- `frame: &mut eframe::Frame` - Window/frame information

**Important:** Every time `update()` runs, you rebuild the entire UI from scratch. This sounds expensive, but egui is highly optimized for this pattern.

---

## Frame Object

The `Frame` object provides window-level information and controls:

```rust
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    // Check if window close was requested
    if frame.close_requested() {
        // Cleanup or show confirmation dialog
        // Return false to cancel close
    }
    
    // Get window information
    let info = frame.info();
    println!("Window size: {:?}", info.window_info.size);
    println!("CPU usage: {:?}", info.cpu_usage);
    
    // Force window to close
    frame.close();
    
    // Access OpenGL context (glow backend)
    #[cfg(feature = "glow")]
    if let Some(gl) = frame.gl() {
        // Custom OpenGL rendering
    }
    
    // Access WGPU render state (wgpu backend)
    #[cfg(feature = "wgpu")]
    if let Some(render_state) = frame.wgpu_render_state() {
        // Custom WGPU rendering
    }
}
```

---

## UI Layouts & Widgets

egui organizes UI into **panels** and **containers**:

### Panels (Top-level containers)

```rust
fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    // Top panel (menu bar area)
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    // Handle open
                }
                if ui.button("Quit").clicked() {
                    frame.close();
                }
            });
        });
    });
    
    // Left side panel
    egui::SidePanel::left("left_panel").show(ctx, |ui| {
        ui.heading("Settings");
        ui.label("Configuration options here");
    });
    
    // Bottom panel
    egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Status: Ready");
        });
    });
    
    // Central panel (takes remaining space)
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Main Content");
        // Your main UI here
    });
}
```

### Common Widgets

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    // Text
    ui.heading("Large Text");
    ui.label("Normal text");
    ui.monospace("Monospace text");
    ui.hyperlink_to("Click me", "https://www.rust-lang.org");
    
    // Button
    if ui.button("Click me").clicked() {
        println!("Button clicked!");
    }
    
    // Checkbox
    let mut checked = self.some_bool;
    ui.checkbox(&mut checked, "Enable feature");
    
    // Text input
    ui.text_edit_singleline(&mut self.name);
    ui.text_edit_multiline(&mut self.description);
    
    // Slider
    ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
    
    // Drag value
    ui.add(egui::DragValue::new(&mut self.speed).speed(0.1));
    
    // Combo box / dropdown
    egui::ComboBox::from_label("Select option")
        .selected_text(format!("{:?}", self.selected))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut self.selected, Option1, "Option 1");
            ui.selectable_value(&mut self.selected, Option2, "Option 2");
        });
    
    // Radio buttons
    ui.radio_value(&mut self.choice, Choice::A, "Choice A");
    ui.radio_value(&mut self.choice, Choice::B, "Choice B");
    
    // Color picker
    ui.color_edit_button_rgb(&mut self.color);
    
    // Separator
    ui.separator();
    
    // Spacing
    ui.add_space(10.0);
});
```

### Layout Containers

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    // Horizontal layout
    ui.horizontal(|ui| {
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.name);
    });
    
    // Vertical layout (default)
    ui.vertical(|ui| {
        ui.label("Line 1");
        ui.label("Line 2");
    });
    
    // Grid layout
    egui::Grid::new("my_grid").show(ui, |ui| {
        ui.label("Row 1, Col 1");
        ui.label("Row 1, Col 2");
        ui.end_row();
        
        ui.label("Row 2, Col 1");
        ui.label("Row 2, Col 2");
        ui.end_row();
    });
    
    // Scroll area
    egui::ScrollArea::vertical().show(ui, |ui| {
        for i in 0..1000 {
            ui.label(format!("Item {}", i));
        }
    });
    
    // Collapsing header (expandable section)
    ui.collapsing("Advanced Options", |ui| {
        ui.label("Hidden content");
    });
    
    // Group (visually grouped content)
    ui.group(|ui| {
        ui.label("Grouped content");
    });
});
```

---

## State Management

Your app struct holds all application state:

```rust
struct MyApp {
    // Simple fields
    name: String,
    age: u32,
    enabled: bool,
    
    // Collections
    items: Vec<String>,
    
    // Enums for choices
    mode: Mode,
    
    // Optional state
    cached_result: Option<String>,
}

enum Mode {
    Edit,
    View,
    Debug,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Directly modify state
            ui.text_edit_singleline(&mut self.name);
            ui.add(egui::Slider::new(&mut self.age, 0..=120));
            
            if ui.button("Add Item").clicked() {
                self.items.push(format!("Item {}", self.items.len()));
            }
            
            // Display state
            ui.label(format!("Hello, {}! Age: {}", self.name, self.age));
            
            for item in &self.items {
                ui.label(item);
            }
        });
    }
    
    // Optional: Save state to disk
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, "name", &self.name);
        eframe::set_value(storage, "age", &self.age);
    }
}
```

---

## Complete Example

Here's a fully functional todo list application:

```rust
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Todo App",
        options,
        Box::new(|cc| Ok(Box::new(TodoApp::new(cc)))),
    )
}

struct TodoApp {
    todos: Vec<Todo>,
    new_todo_text: String,
}

struct Todo {
    text: String,
    completed: bool,
}

impl TodoApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            todos: vec![
                Todo { text: "Learn Rust".to_string(), completed: false },
                Todo { text: "Build GUI app".to_string(), completed: false },
            ],
            new_todo_text: String::new(),
        }
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 My Todo List");
            ui.separator();
            
            // Input area for new todos
            ui.horizontal(|ui| {
                ui.label("New todo:");
                let response = ui.text_edit_singleline(&mut self.new_todo_text);
                
                if ui.button("Add").clicked() || 
                   (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if !self.new_todo_text.trim().is_empty() {
                        self.todos.push(Todo {
                            text: self.new_todo_text.clone(),
                            completed: false,
                        });
                        self.new_todo_text.clear();
                    }
                }
            });
            
            ui.separator();
            
            // Display todos
            let mut todos_to_remove = vec![];
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                for (i, todo) in self.todos.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        // Checkbox to mark complete
                        ui.checkbox(&mut todo.completed, "");
                        
                        // Todo text (strikethrough if completed)
                        if todo.completed {
                            ui.label(egui::RichText::new(&todo.text).strikethrough());
                        } else {
                            ui.label(&todo.text);
                        }
                        
                        // Delete button
                        if ui.button("🗑").clicked() {
                            todos_to_remove.push(i);
                        }
                    });
                }
            });
            
            // Remove deleted todos (in reverse to maintain indices)
            for &i in todos_to_remove.iter().rev() {
                self.todos.remove(i);
            }
            
            ui.separator();
            
            // Statistics
            let total = self.todos.len();
            let completed = self.todos.iter().filter(|t| t.completed).count();
            ui.label(format!("📊 {} of {} tasks completed", completed, total));
        });
    }
}
```

---

## Common Patterns

### Pattern 1: Responding to Events

```rust
// Button click
if ui.button("Save").clicked() {
    self.save_data();
}

// Enter key in text field
let response = ui.text_edit_singleline(&mut self.input);
if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
    self.process_input();
}

// Keyboard shortcuts
if ui.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) {
    self.save();
}
```

### Pattern 2: Conditional UI

```rust
// Show different UI based on mode
match self.mode {
    Mode::Edit => {
        ui.text_edit_multiline(&mut self.content);
    }
    Mode::View => {
        ui.label(&self.content);
    }
}

// Only show if enabled
if self.debug_mode {
    ui.label(format!("Debug info: {:?}", self));
}
```

### Pattern 3: Windows and Popups

```rust
// Separate window
egui::Window::new("Settings")
    .open(&mut self.show_settings)  // Toggleable
    .resizable(true)
    .default_width(300.0)
    .show(ctx, |ui| {
        ui.label("Settings content");
    });

// Modal popup
if self.show_confirm_dialog {
    egui::Window::new("Confirm")
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("Are you sure?");
            ui.horizontal(|ui| {
                if ui.button("Yes").clicked() {
                    self.confirmed = true;
                    self.show_confirm_dialog = false;
                }
                if ui.button("No").clicked() {
                    self.show_confirm_dialog = false;
                }
            });
        });
}
```

### Pattern 4: Tooltips and Help

```rust
ui.label("Hover for info").on_hover_text("This is helpful information");

ui.button("?").on_hover_text_at_pointer("Click for help");
```

### Pattern 5: Custom Styling

```rust
// Temporary style change
ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
ui.label("Large text");

// Custom colors
ui.visuals_mut().override_text_color = Some(egui::Color32::RED);
ui.label("Red text");

// Rich text (inline styling)
ui.label(
    egui::RichText::new("Styled Text")
        .color(egui::Color32::BLUE)
        .size(20.0)
        .strong()
);
```

---

## Next Steps

1. **Experiment** - Modify the todo app example above
2. **Read Examples** - Check out [eframe examples](https://github.com/emilk/egui/tree/main/examples)
3. **Explore Widgets** - Browse [egui widget gallery](https://www.egui.rs/#demo)
4. **Learn egui Context** - Read [egui::Context docs](https://docs.rs/egui/latest/egui/struct.Context.html)
5. **Add Features** - Try implementing:
   - File saving/loading
   - Custom themes
   - Keyboard shortcuts
   - Multiple windows
   - Integration with async code

---

## Additional Resources

- **Official Docs**: https://docs.rs/eframe
- **egui Docs**: https://docs.rs/egui
- **Live Demo**: https://www.egui.rs/#demo
- **Template Project**: https://github.com/emilk/eframe_template
- **Discord**: https://discord.gg/JFcEma9bJq
- **GitHub Discussions**: https://github.com/emilk/egui/discussions

---

## Tips for Learning

1. **Start Simple** - Begin with a single panel and a few widgets
2. **Iterate Quickly** - The immediate mode paradigm makes experimentation easy
3. **Read the Output** - Use `println!()` or `ui.label()` to debug state
4. **Check Examples** - The official examples cover most use cases
5. **Don't Over-Structure** - Keep your code simple; let Rust help you refactor later

Happy coding! 🦀
