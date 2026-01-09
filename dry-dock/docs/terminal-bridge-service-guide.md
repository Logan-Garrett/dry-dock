# Terminal Bridge Service Guide

## Overview

This guide explains how to implement a terminal bridge service in your eframe/egui application, enabling you to embed a fully functional terminal emulator within your GUI. This allows users to execute commands, run interactive applications (like vim, nano, htop), and have a complete terminal experience without leaving your application.

---

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Implementation Approaches](#implementation-approaches)
3. [Option 1: Using Portable PTY (Recommended)](#option-1-using-portable-pty-recommended)
4. [Option 2: Using alacritty_terminal](#option-2-using-alacritty_terminal)
5. [Rendering in egui](#rendering-in-egui)
6. [Handling Input](#handling-input)
7. [Complete Implementation Example](#complete-implementation-example)
8. [Advanced Features](#advanced-features)
9. [Bundling for Distribution](#bundling-for-distribution)

---

## Architecture Overview

A terminal bridge service consists of several key components:

```
┌─────────────────────────────────────────────┐
│           egui Application                  │
│  ┌───────────────────────────────────────┐  │
│  │     Terminal Widget (Renderer)        │  │
│  │  - Displays terminal output           │  │
│  │  - Captures keyboard/mouse input      │  │
│  └───────────────┬───────────────────────┘  │
│                  │                           │
│  ┌───────────────▼───────────────────────┐  │
│  │    Terminal Bridge Service            │  │
│  │  - Manages PTY (pseudo-terminal)      │  │
│  │  - Parses ANSI/VT sequences           │  │
│  │  - Handles terminal state             │  │
│  └───────────────┬───────────────────────┘  │
└──────────────────┼───────────────────────────┘
                   │
         ┌─────────▼──────────┐
         │   PTY Process      │
         │  (bash/zsh/shell)  │
         └────────────────────┘
```

### Key Components:

1. **PTY (Pseudo-Terminal)**: A bidirectional communication channel that emulates a terminal
2. **Terminal Emulator**: Parses ANSI escape codes and maintains terminal state (cursor position, colors, etc.)
3. **Renderer**: Draws the terminal content in egui
4. **Input Handler**: Captures keyboard/mouse events and sends them to the PTY

---

## Implementation Approaches

### Approach Comparison

| Feature | portable-pty | alacritty_terminal | Custom Implementation |
|---------|-------------|-------------------|----------------------|
| Cross-platform | ✅ Excellent | ✅ Good | ❌ Complex |
| ANSI Support | ✅ Via vte | ✅ Built-in | ⚠️ Manual |
| Difficulty | ⭐⭐ Easy | ⭐⭐⭐ Medium | ⭐⭐⭐⭐⭐ Hard |
| Dependencies | Minimal | Heavy | Minimal |
| Interactive Apps | ✅ Yes | ✅ Yes | ⚠️ Limited |
| Maintenance | ✅ Active | ✅ Very Active | ❌ You |

**Recommendation**: Use **portable-pty + vte** for most use cases.

---

## Option 1: Using Portable PTY (Recommended)

### Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
portable-pty = "0.8"
vte = "0.13"
crossbeam-channel = "0.5"
```

### Basic Architecture

```rust
use portable_pty::{native_pty_system, PtySize, CommandBuilder};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

pub struct TerminalBridge {
    // Channel to send input to PTY
    input_sender: Sender<Vec<u8>>,
    // Channel to receive output from PTY
    output_receiver: Receiver<Vec<u8>>,
    // Terminal state
    terminal_state: TerminalState,
}

pub struct TerminalState {
    // Grid of characters (rows x cols)
    pub grid: Vec<Vec<Cell>>,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub cols: usize,
    pub rows: usize,
}

#[derive(Clone)]
pub struct Cell {
    pub character: char,
    pub fg_color: [u8; 3],
    pub bg_color: [u8; 3],
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
}
```

### Implementation

```rust
use portable_pty::{native_pty_system, PtySize, CommandBuilder};
use vte::{Params, Parser, Perform};
use std::sync::{Arc, Mutex};
use std::thread;
use std::io::{Read, Write};

impl TerminalBridge {
    pub fn new(cols: u16, rows: u16) -> Result<Self, Box<dyn std::error::Error>> {
        // Create a PTY system
        let pty_system = native_pty_system();
        
        // Create a new PTY with specified size
        let pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        
        // Determine shell based on OS
        let shell = if cfg!(target_os = "windows") {
            "powershell.exe"
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
        };
        
        // Create command to spawn
        let cmd = CommandBuilder::new(shell);
        
        // Spawn the shell in the PTY
        let mut child = pair.slave.spawn_command(cmd)?;
        
        // Create channels for communication
        let (input_tx, input_rx) = std::sync::mpsc::channel::<Vec<u8>>();
        let (output_tx, output_rx) = std::sync::mpsc::channel::<Vec<u8>>();
        
        // Clone the writer for the input thread
        let mut writer = pair.master.take_writer()?;
        
        // Spawn thread to handle input (from GUI to PTY)
        thread::spawn(move || {
            while let Ok(data) = input_rx.recv() {
                if writer.write_all(&data).is_err() {
                    break;
                }
            }
        });
        
        // Clone the reader for the output thread
        let mut reader = pair.master.try_clone_reader()?;
        
        // Spawn thread to handle output (from PTY to GUI)
        thread::spawn(move || {
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break, // EOF
                    Ok(n) => {
                        if output_tx.send(buf[..n].to_vec()).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });
        
        // Initialize terminal state
        let terminal_state = TerminalState::new(cols as usize, rows as usize);
        
        Ok(Self {
            input_sender: input_tx,
            output_receiver: output_rx,
            terminal_state,
        })
    }
    
    pub fn send_input(&self, data: &[u8]) {
        let _ = self.input_sender.send(data.to_vec());
    }
    
    pub fn send_string(&self, s: &str) {
        self.send_input(s.as_bytes());
    }
    
    pub fn update(&mut self) {
        // Process all pending output
        while let Ok(data) = self.output_receiver.try_recv() {
            self.process_output(&data);
        }
    }
    
    fn process_output(&mut self, data: &[u8]) {
        // Parse ANSI sequences using vte
        let mut parser = Parser::new();
        let mut performer = TerminalPerformer::new(&mut self.terminal_state);
        
        for byte in data {
            parser.advance(&mut performer, *byte);
        }
    }
    
    pub fn resize(&mut self, cols: u16, rows: u16) {
        self.terminal_state.resize(cols as usize, rows as usize);
        // TODO: Send resize signal to PTY
    }
}

impl TerminalState {
    pub fn new(cols: usize, rows: usize) -> Self {
        let grid = vec![vec![Cell::default(); cols]; rows];
        Self {
            grid,
            cursor_row: 0,
            cursor_col: 0,
            cols,
            rows,
        }
    }
    
    pub fn resize(&mut self, cols: usize, rows: usize) {
        // Preserve existing content when resizing
        let mut new_grid = vec![vec![Cell::default(); cols]; rows];
        
        for row in 0..rows.min(self.rows) {
            for col in 0..cols.min(self.cols) {
                new_grid[row][col] = self.grid[row][col].clone();
            }
        }
        
        self.grid = new_grid;
        self.cols = cols;
        self.rows = rows;
        self.cursor_row = self.cursor_row.min(rows - 1);
        self.cursor_col = self.cursor_col.min(cols - 1);
    }
    
    pub fn write_char(&mut self, ch: char) {
        if self.cursor_col < self.cols {
            self.grid[self.cursor_row][self.cursor_col].character = ch;
            self.cursor_col += 1;
        }
    }
    
    pub fn newline(&mut self) {
        self.cursor_col = 0;
        if self.cursor_row < self.rows - 1 {
            self.cursor_row += 1;
        } else {
            // Scroll up
            self.grid.remove(0);
            self.grid.push(vec![Cell::default(); self.cols]);
        }
    }
    
    pub fn move_cursor(&mut self, row: usize, col: usize) {
        self.cursor_row = row.min(self.rows - 1);
        self.cursor_col = col.min(self.cols - 1);
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            character: ' ',
            fg_color: [200, 200, 200],
            bg_color: [0, 0, 0],
            bold: false,
            italic: false,
            underline: false,
        }
    }
}
```

### VTE Parser Implementation

```rust
use vte::{Params, Perform};

struct TerminalPerformer<'a> {
    state: &'a mut TerminalState,
    current_fg: [u8; 3],
    current_bg: [u8; 3],
    current_bold: bool,
    current_italic: bool,
    current_underline: bool,
}

impl<'a> TerminalPerformer<'a> {
    fn new(state: &'a mut TerminalState) -> Self {
        Self {
            state,
            current_fg: [200, 200, 200],
            current_bg: [0, 0, 0],
            current_bold: false,
            current_italic: false,
            current_underline: false,
        }
    }
}

impl<'a> Perform for TerminalPerformer<'a> {
    fn print(&mut self, c: char) {
        if c == '\r' {
            self.state.cursor_col = 0;
        } else if c == '\n' {
            self.state.newline();
        } else {
            self.state.write_char(c);
        }
    }
    
    fn execute(&mut self, byte: u8) {
        match byte {
            b'\n' => self.state.newline(),
            b'\r' => self.state.cursor_col = 0,
            b'\t' => {
                // Tab: move to next tab stop (every 8 columns)
                self.state.cursor_col = ((self.state.cursor_col / 8) + 1) * 8;
                self.state.cursor_col = self.state.cursor_col.min(self.state.cols - 1);
            }
            b'\x08' => {
                // Backspace
                if self.state.cursor_col > 0 {
                    self.state.cursor_col -= 1;
                }
            }
            _ => {}
        }
    }
    
    fn csi_dispatch(&mut self, params: &Params, _intermediates: &[u8], _ignore: bool, c: char) {
        match c {
            'H' | 'f' => {
                // Cursor position
                let row = params.iter().nth(0).and_then(|p| p.first()).unwrap_or(&1) - 1;
                let col = params.iter().nth(1).and_then(|p| p.first()).unwrap_or(&1) - 1;
                self.state.move_cursor(row as usize, col as usize);
            }
            'A' => {
                // Cursor up
                let n = params.iter().nth(0).and_then(|p| p.first()).unwrap_or(&1);
                if self.state.cursor_row >= *n as usize {
                    self.state.cursor_row -= *n as usize;
                } else {
                    self.state.cursor_row = 0;
                }
            }
            'B' => {
                // Cursor down
                let n = params.iter().nth(0).and_then(|p| p.first()).unwrap_or(&1);
                self.state.cursor_row = (self.state.cursor_row + *n as usize).min(self.state.rows - 1);
            }
            'C' => {
                // Cursor forward
                let n = params.iter().nth(0).and_then(|p| p.first()).unwrap_or(&1);
                self.state.cursor_col = (self.state.cursor_col + *n as usize).min(self.state.cols - 1);
            }
            'D' => {
                // Cursor back
                let n = params.iter().nth(0).and_then(|p| p.first()).unwrap_or(&1);
                if self.state.cursor_col >= *n as usize {
                    self.state.cursor_col -= *n as usize;
                } else {
                    self.state.cursor_col = 0;
                }
            }
            'J' => {
                // Erase in display
                let n = params.iter().nth(0).and_then(|p| p.first()).unwrap_or(&0);
                match n {
                    0 => {
                        // Clear from cursor to end of screen
                        for col in self.state.cursor_col..self.state.cols {
                            self.state.grid[self.state.cursor_row][col] = Cell::default();
                        }
                        for row in (self.state.cursor_row + 1)..self.state.rows {
                            for col in 0..self.state.cols {
                                self.state.grid[row][col] = Cell::default();
                            }
                        }
                    }
                    1 => {
                        // Clear from start to cursor
                        for row in 0..self.state.cursor_row {
                            for col in 0..self.state.cols {
                                self.state.grid[row][col] = Cell::default();
                            }
                        }
                        for col in 0..=self.state.cursor_col {
                            self.state.grid[self.state.cursor_row][col] = Cell::default();
                        }
                    }
                    2 => {
                        // Clear entire screen
                        for row in 0..self.state.rows {
                            for col in 0..self.state.cols {
                                self.state.grid[row][col] = Cell::default();
                            }
                        }
                    }
                    _ => {}
                }
            }
            'K' => {
                // Erase in line
                let n = params.iter().nth(0).and_then(|p| p.first()).unwrap_or(&0);
                match n {
                    0 => {
                        // Clear from cursor to end of line
                        for col in self.state.cursor_col..self.state.cols {
                            self.state.grid[self.state.cursor_row][col] = Cell::default();
                        }
                    }
                    1 => {
                        // Clear from start of line to cursor
                        for col in 0..=self.state.cursor_col {
                            self.state.grid[self.state.cursor_row][col] = Cell::default();
                        }
                    }
                    2 => {
                        // Clear entire line
                        for col in 0..self.state.cols {
                            self.state.grid[self.state.cursor_row][col] = Cell::default();
                        }
                    }
                    _ => {}
                }
            }
            'm' => {
                // SGR - Select Graphic Rendition (colors and styles)
                for param in params.iter() {
                    for &p in param {
                        match p {
                            0 => {
                                // Reset
                                self.current_fg = [200, 200, 200];
                                self.current_bg = [0, 0, 0];
                                self.current_bold = false;
                                self.current_italic = false;
                                self.current_underline = false;
                            }
                            1 => self.current_bold = true,
                            3 => self.current_italic = true,
                            4 => self.current_underline = true,
                            22 => self.current_bold = false,
                            23 => self.current_italic = false,
                            24 => self.current_underline = false,
                            30..=37 => {
                                // Foreground color
                                self.current_fg = ansi_color_to_rgb(p - 30);
                            }
                            40..=47 => {
                                // Background color
                                self.current_bg = ansi_color_to_rgb(p - 40);
                            }
                            90..=97 => {
                                // Bright foreground color
                                self.current_fg = ansi_bright_color_to_rgb(p - 90);
                            }
                            100..=107 => {
                                // Bright background color
                                self.current_bg = ansi_bright_color_to_rgb(p - 100);
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    fn hook(&mut self, _params: &Params, _intermediates: &[u8], _ignore: bool, _c: char) {}
    fn put(&mut self, _byte: u8) {}
    fn unhook(&mut self) {}
    fn osc_dispatch(&mut self, _params: &[&[u8]], _bell_terminated: bool) {}
    fn esc_dispatch(&mut self, _intermediates: &[u8], _ignore: bool, _byte: u8) {}
}

fn ansi_color_to_rgb(color: u16) -> [u8; 3] {
    match color {
        0 => [0, 0, 0],         // Black
        1 => [205, 49, 49],     // Red
        2 => [13, 188, 121],    // Green
        3 => [229, 229, 16],    // Yellow
        4 => [36, 114, 200],    // Blue
        5 => [188, 63, 188],    // Magenta
        6 => [17, 168, 205],    // Cyan
        7 => [229, 229, 229],   // White
        _ => [200, 200, 200],
    }
}

fn ansi_bright_color_to_rgb(color: u16) -> [u8; 3] {
    match color {
        0 => [102, 102, 102],   // Bright Black (Gray)
        1 => [241, 76, 76],     // Bright Red
        2 => [35, 209, 139],    // Bright Green
        3 => [245, 245, 67],    // Bright Yellow
        4 => [59, 142, 234],    // Bright Blue
        5 => [214, 112, 214],   // Bright Magenta
        6 => [41, 184, 219],    // Bright Cyan
        7 => [255, 255, 255],   // Bright White
        _ => [255, 255, 255],
    }
}
```

---

## Rendering in egui

### Basic Renderer

```rust
use eframe::egui;

pub fn render_terminal(ui: &mut egui::Ui, terminal: &mut TerminalBridge) {
    // Update terminal state with new output
    terminal.update();
    
    let state = &terminal.terminal_state;
    
    // Use monospace font
    let font_id = egui::FontId::monospace(14.0);
    
    // Calculate cell size based on font
    let galley = ui.fonts(|f| f.layout_no_wrap("X".to_string(), font_id.clone(), egui::Color32::WHITE));
    let cell_width = galley.rect.width();
    let cell_height = galley.rect.height();
    
    // Create a scrollable area
    egui::ScrollArea::both()
        .id_source("terminal_scroll")
        .show(ui, |ui| {
            let (response, painter) = ui.allocate_painter(
                egui::vec2(
                    cell_width * state.cols as f32,
                    cell_height * state.rows as f32,
                ),
                egui::Sense::click(),
            );
            
            let origin = response.rect.min;
            
            // Draw background
            painter.rect_filled(
                response.rect,
                0.0,
                egui::Color32::from_rgb(0, 0, 0),
            );
            
            // Draw each character
            for (row_idx, row) in state.grid.iter().enumerate() {
                for (col_idx, cell) in row.iter().enumerate() {
                    let x = origin.x + col_idx as f32 * cell_width;
                    let y = origin.y + row_idx as f32 * cell_height;
                    let pos = egui::pos2(x, y);
                    
                    // Draw background if not default
                    if cell.bg_color != [0, 0, 0] {
                        let bg_rect = egui::Rect::from_min_size(
                            pos,
                            egui::vec2(cell_width, cell_height),
                        );
                        painter.rect_filled(
                            bg_rect,
                            0.0,
                            egui::Color32::from_rgb(cell.bg_color[0], cell.bg_color[1], cell.bg_color[2]),
                        );
                    }
                    
                    // Draw character
                    if cell.character != ' ' {
                        let color = egui::Color32::from_rgb(
                            cell.fg_color[0],
                            cell.fg_color[1],
                            cell.fg_color[2],
                        );
                        
                        painter.text(
                            pos,
                            egui::Align2::LEFT_TOP,
                            cell.character.to_string(),
                            font_id.clone(),
                            color,
                        );
                    }
                }
            }
            
            // Draw cursor
            let cursor_x = origin.x + state.cursor_col as f32 * cell_width;
            let cursor_y = origin.y + state.cursor_row as f32 * cell_height;
            let cursor_rect = egui::Rect::from_min_size(
                egui::pos2(cursor_x, cursor_y),
                egui::vec2(cell_width, cell_height),
            );
            painter.rect_filled(
                cursor_rect,
                0.0,
                egui::Color32::from_rgba_unmultiplied(200, 200, 200, 128),
            );
            
            // Handle keyboard input
            if response.has_focus() {
                ui.input(|i| {
                    for event in &i.events {
                        if let egui::Event::Text(text) = event {
                            terminal.send_string(text);
                        } else if let egui::Event::Key { key, pressed: true, modifiers, .. } = event {
                            handle_key_event(terminal, *key, *modifiers);
                        }
                    }
                });
            }
            
            // Request focus on click
            if response.clicked() {
                response.request_focus();
            }
        });
}

fn handle_key_event(terminal: &TerminalBridge, key: egui::Key, modifiers: egui::Modifiers) {
    use egui::Key;
    
    let mut buf = Vec::new();
    
    match key {
        Key::Enter => buf.extend_from_slice(b"\r"),
        Key::Tab => buf.extend_from_slice(b"\t"),
        Key::Backspace => buf.extend_from_slice(b"\x7f"),
        Key::Escape => buf.extend_from_slice(b"\x1b"),
        Key::ArrowUp => buf.extend_from_slice(b"\x1b[A"),
        Key::ArrowDown => buf.extend_from_slice(b"\x1b[B"),
        Key::ArrowRight => buf.extend_from_slice(b"\x1b[C"),
        Key::ArrowLeft => buf.extend_from_slice(b"\x1b[D"),
        Key::Home => buf.extend_from_slice(b"\x1b[H"),
        Key::End => buf.extend_from_slice(b"\x1b[F"),
        Key::PageUp => buf.extend_from_slice(b"\x1b[5~"),
        Key::PageDown => buf.extend_from_slice(b"\x1b[6~"),
        Key::Delete => buf.extend_from_slice(b"\x1b[3~"),
        _ => {
            // Handle Ctrl+C, Ctrl+D, etc.
            if modifiers.ctrl {
                match key {
                    Key::C => buf.push(0x03), // ETX
                    Key::D => buf.push(0x04), // EOT
                    Key::Z => buf.push(0x1A), // SUB
                    _ => {}
                }
            }
        }
    }
    
    if !buf.is_empty() {
        terminal.send_input(&buf);
    }
}
```

---

## Complete Implementation Example

### File Structure

```
src/
├── services/
│   ├── mod.rs
│   └── terminal_bridge_service.rs
└── main.rs
```

### services/mod.rs

```rust
pub mod terminal_bridge_service;
```

### services/terminal_bridge_service.rs

```rust
use portable_pty::{native_pty_system, PtySize, CommandBuilder};
use vte::{Params, Parser, Perform};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;
use std::io::{Read, Write};

// ... (Include all the code from above: TerminalBridge, TerminalState, Cell, 
//      TerminalPerformer, and helper functions)
```

### Integration in main.rs

```rust
use eframe::egui;
mod services;
use services::terminal_bridge_service::{TerminalBridge, render_terminal};

struct MyApp {
    terminal: Option<TerminalBridge>,
    show_terminal: bool,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            terminal: None,
            show_terminal: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaints for terminal updates
        ctx.request_repaint();
        
        egui::TopBottomPanel::top("menu").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Toggle Terminal").clicked() {
                    self.show_terminal = !self.show_terminal;
                    
                    // Initialize terminal on first show
                    if self.show_terminal && self.terminal.is_none() {
                        match TerminalBridge::new(120, 30) {
                            Ok(terminal) => self.terminal = Some(terminal),
                            Err(e) => eprintln!("Failed to create terminal: {}", e),
                        }
                    }
                }
            });
        });
        
        if self.show_terminal {
            egui::CentralPanel::default().show(ctx, |ui| {
                if let Some(terminal) = &mut self.terminal {
                    render_terminal(ui, terminal);
                } else {
                    ui.label("Terminal failed to initialize");
                }
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Main Application");
                ui.label("Click 'Toggle Terminal' to show embedded terminal");
            });
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Terminal Bridge Demo",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}
```

---

## Option 2: Using alacritty_terminal

Alacritty is a high-performance terminal emulator written in Rust. You can use its terminal component.

### Dependencies

```toml
[dependencies]
alacritty_terminal = "0.24"
```

### Pros and Cons

**Pros:**
- Battle-tested (used by Alacritty terminal)
- Excellent ANSI support
- High performance
- Active development

**Cons:**
- Heavier dependency
- More complex API
- Tightly coupled to Alacritty's architecture

### Basic Usage

```rust
use alacritty_terminal::{Term, Config};
use alacritty_terminal::tty;

// This is more complex and requires significant integration work
// Recommended only if you need advanced features or performance
```

---

## Advanced Features

### 1. Copy/Paste Support

```rust
impl TerminalBridge {
    pub fn copy_selection(&self, start: (usize, usize), end: (usize, usize)) -> String {
        let mut result = String::new();
        let (start_row, start_col) = start;
        let (end_row, end_col) = end;
        
        for row in start_row..=end_row {
            let start_c = if row == start_row { start_col } else { 0 };
            let end_c = if row == end_row { end_col } else { self.terminal_state.cols };
            
            for col in start_c..end_c {
                result.push(self.terminal_state.grid[row][col].character);
            }
            if row < end_row {
                result.push('\n');
            }
        }
        
        result
    }
    
    pub fn paste(&self, text: &str) {
        self.send_string(text);
    }
}
```

### 2. Search Functionality

```rust
impl TerminalState {
    pub fn search(&self, query: &str) -> Vec<(usize, usize)> {
        let mut results = Vec::new();
        
        for (row_idx, row) in self.grid.iter().enumerate() {
            let line: String = row.iter().map(|c| c.character).collect();
            let mut start = 0;
            
            while let Some(pos) = line[start..].find(query) {
                results.push((row_idx, start + pos));
                start += pos + 1;
            }
        }
        
        results
    }
}
```

### 3. Scrollback Buffer

```rust
pub struct TerminalState {
    pub grid: Vec<Vec<Cell>>,
    pub scrollback: Vec<Vec<Cell>>, // Add this
    pub scrollback_limit: usize,
    // ... other fields
}

impl TerminalState {
    pub fn scroll_up(&mut self) {
        if !self.scrollback.is_empty() {
            let line = self.scrollback.pop().unwrap();
            self.grid.insert(0, line);
            let removed = self.grid.pop().unwrap();
            // Optionally store removed line
        }
    }
    
    pub fn add_to_scrollback(&mut self, line: Vec<Cell>) {
        self.scrollback.push(line);
        if self.scrollback.len() > self.scrollback_limit {
            self.scrollback.remove(0);
        }
    }
}
```

### 4. Multiple Terminal Tabs

```rust
struct TerminalTabs {
    terminals: Vec<TerminalBridge>,
    active_tab: usize,
}

impl TerminalTabs {
    fn render(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (idx, _) in self.terminals.iter().enumerate() {
                if ui.selectable_label(self.active_tab == idx, format!("Tab {}", idx + 1)).clicked() {
                    self.active_tab = idx;
                }
            }
            
            if ui.button("+").clicked() {
                if let Ok(term) = TerminalBridge::new(120, 30) {
                    self.terminals.push(term);
                    self.active_tab = self.terminals.len() - 1;
                }
            }
        });
        
        if let Some(terminal) = self.terminals.get_mut(self.active_tab) {
            render_terminal(ui, terminal);
        }
    }
}
```

### 5. Custom Themes

```rust
pub struct TerminalTheme {
    pub background: [u8; 3],
    pub foreground: [u8; 3],
    pub cursor: [u8; 3],
    pub selection: [u8; 3],
    pub ansi_colors: [[u8; 3]; 16],
}

impl TerminalTheme {
    pub fn dracula() -> Self {
        Self {
            background: [40, 42, 54],
            foreground: [248, 248, 242],
            cursor: [255, 184, 108],
            selection: [68, 71, 90],
            ansi_colors: [
                [33, 34, 44],      // Black
                [255, 85, 85],     // Red
                [80, 250, 123],    // Green
                [241, 250, 140],   // Yellow
                [189, 147, 249],   // Blue
                [255, 121, 198],   // Magenta
                [139, 233, 253],   // Cyan
                [248, 248, 242],   // White
                // Bright colors
                [98, 114, 164],
                [255, 110, 103],
                [90, 247, 142],
                [244, 249, 157],
                [202, 169, 250],
                [255, 146, 208],
                [154, 237, 254],
                [255, 255, 255],
            ],
        }
    }
}
```

### 6. Running Specific Commands

```rust
impl TerminalBridge {
    pub fn execute_command(&self, command: &str) {
        self.send_string(command);
        self.send_input(b"\n");
    }
    
    pub fn open_vim(&self, filename: &str) {
        self.execute_command(&format!("vim {}", filename));
    }
    
    pub fn run_cargo_build(&self) {
        self.execute_command("cargo build");
    }
}
```

---

## Bundling for Distribution

### macOS

Create an app bundle structure:

```
DryDock.app/
├── Contents/
│   ├── Info.plist
│   ├── MacOS/
│   │   └── dry-dock (binary)
│   └── Resources/
│       └── icon.icns
```

**Info.plist:**

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Dry Dock</string>
    <key>CFBundleDisplayName</key>
    <string>Dry Dock</string>
    <key>CFBundleIdentifier</key>
    <string>com.yourcompany.drydock</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleExecutable</key>
    <string>dry-dock</string>
</dict>
</plist>
```

### Windows

Use a tool like `cargo-wix` or create an installer with:
- NSIS
- InnoSetup
- WiX Toolset

### Linux

Create a `.desktop` file and package as:
- AppImage
- Flatpak
- Snap

---

## Performance Considerations

### 1. Limit Repaints

```rust
impl MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Only repaint when terminal is visible and has updates
        if self.show_terminal {
            if let Some(terminal) = &self.terminal {
                if terminal.has_pending_updates() {
                    ctx.request_repaint();
                }
            }
        }
    }
}
```

### 2. Optimize Rendering

- Only redraw changed cells
- Use texture caching for characters
- Implement dirty region tracking

### 3. Buffer Management

- Limit scrollback buffer size
- Use ring buffers for efficiency
- Implement lazy loading for large outputs

---

## Troubleshooting

### Common Issues

**1. Terminal not responding to input:**
- Ensure PTY is properly initialized
- Check that input thread is running
- Verify key codes are correct for your OS

**2. ANSI codes not rendering:**
- Verify VTE parser implementation
- Check color mappings
- Test with simple commands first (e.g., `echo -e "\e[31mRed\e[0m"`)

**3. Interactive apps (vim) not working:**
- Ensure terminal size is being reported correctly
- Verify arrow keys send correct escape sequences
- Check that terminal identifies as a proper terminal type

**4. Performance issues:**
- Limit repaint frequency
- Optimize rendering (only draw visible area)
- Profile with `cargo flamegraph`

---

## Testing

### Basic Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_terminal_creation() {
        let terminal = TerminalBridge::new(80, 24);
        assert!(terminal.is_ok());
    }
    
    #[test]
    fn test_ansi_parsing() {
        let mut state = TerminalState::new(80, 24);
        // Test parsing ANSI codes
    }
    
    #[test]
    fn test_cursor_movement() {
        let mut state = TerminalState::new(80, 24);
        state.move_cursor(5, 10);
        assert_eq!(state.cursor_row, 5);
        assert_eq!(state.cursor_col, 10);
    }
}
```

---

## Resources

- **portable-pty**: https://docs.rs/portable-pty
- **vte**: https://docs.rs/vte
- **ANSI Escape Codes**: https://en.wikipedia.org/wiki/ANSI_escape_code
- **Alacritty Source**: https://github.com/alacritty/alacritty
- **egui Examples**: https://github.com/emilk/egui

---

## Conclusion

Implementing a terminal bridge service requires:
1. PTY management for process communication
2. ANSI/VT parser for terminal control sequences
3. egui renderer for displaying terminal output
4. Input handler for keyboard/mouse events

The recommended approach is **portable-pty + vte** for most use cases, as it provides a good balance of features, cross-platform support, and maintainability.

Start with the basic implementation and gradually add features like:
- Scrollback buffer
- Copy/paste
- Search
- Multiple tabs
- Custom themes

This will give you a fully functional terminal emulator embedded in your GUI application!
