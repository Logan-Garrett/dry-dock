// src/ui/screens/assistant_screen.rs
use eframe::egui;
use crate::ui::screens::screen_trait::Screen;
use crate::models::{ChatMessage, MessageRole};
use crate::services::{AssistantService, log_service};
use crate::ui::styles::Theme;
use crate::ui::modals::ActiveModal;
use std::sync::mpsc::{channel, Sender, Receiver};

enum AsyncResponse {
    Message(String),
    Error(String),
    ServerStatus(bool),
}

pub struct AssistantScreen {
    messages: Vec<ChatMessage>,
    input_text: String,
    is_loading: bool,
    server_status_checked: bool,
    server_available: bool,
    response_rx: Receiver<AsyncResponse>,
    response_tx: Sender<AsyncResponse>,
}

impl Default for AssistantScreen {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            messages: Vec::new(),
            input_text: String::new(),
            is_loading: false,
            server_status_checked: false,
            server_available: false,
            response_rx: rx,
            response_tx: tx,
        }
    }
}

impl Screen for AssistantScreen {
    fn title(&self) -> &str {
        "AI Assistant"
    }
}

impl AssistantScreen {
    pub fn clear_for_reload(&mut self) {
        // Nothing to reload for assistant screen
    }

    pub fn render(&mut self, ui: &mut egui::Ui, _modal_opener: &mut dyn FnMut(ActiveModal)) {
        Theme::apply_body_style(ui);
        
        // Check for async responses
        while let Ok(response) = self.response_rx.try_recv() {
            match response {
                AsyncResponse::Message(content) => {
                    self.messages.push(ChatMessage::assistant(content));
                    self.is_loading = false;
                    ui.ctx().request_repaint();
                }
                AsyncResponse::Error(error) => {
                    self.messages.push(ChatMessage::assistant(format!("Error: {}", error)));
                    self.is_loading = false;
                    ui.ctx().request_repaint();
                }
                AsyncResponse::ServerStatus(available) => {
                    self.server_available = available;
                    self.server_status_checked = true;
                    ui.ctx().request_repaint();
                }
            }
        }
        
        // Request repaint if loading to show spinner animation
        if self.is_loading {
            ui.ctx().request_repaint();
        }
        
        // Header
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.horizontal(|ui| {
            ui.heading(egui::RichText::new(self.title()).strong());
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Server status indicator
                let status_text = if !self.server_status_checked {
                    "Checking..."
                } else if self.server_available {
                    "● Online"
                } else {
                    "● Offline"
                };
                let status_color = if !self.server_status_checked {
                    Theme::TEXT_SECONDARY
                } else if self.server_available {
                    egui::Color32::from_rgb(76, 175, 80) // Green
                } else {
                    Theme::DANGER_COLOR
                };
                ui.label(egui::RichText::new(status_text).color(status_color));
            });
        });
        ui.add_space(Theme::SPACING_LARGE);

        // Check server status once
        if !self.server_status_checked {
            self.check_server_status_async();
            self.server_status_checked = true;
        }

        // Action buttons
        ui.horizontal(|ui| {
            if ui.add(Theme::button("Clear Chat")).clicked() {
                self.messages.clear();
                log_service::add_log_entry("INFO", "Chat history cleared");
            }
            
            if ui.add(Theme::button("Refresh Status")).clicked() {
                self.server_status_checked = false;
            }
        });
        
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.separator();
        ui.add_space(Theme::SPACING_MEDIUM);

        if !self.server_available && self.server_status_checked {
            // Show offline message
            ui.vertical_centered(|ui| {
                ui.add_space(Theme::SPACING_XL);
                ui.label(egui::RichText::new("⚠️").size(48.0));
                ui.add_space(Theme::SPACING_SMALL);
                ui.label(egui::RichText::new("Ollama Server Offline").size(Theme::FONT_SIZE_SUBHEADING).color(Theme::TEXT_PRIMARY));
                ui.add_space(Theme::SPACING_SMALL);
                ui.label(egui::RichText::new("The AI assistant requires the Ollama server to be running.").color(Theme::TEXT_SECONDARY));
                ui.label(egui::RichText::new("Please check the logs for more information.").color(Theme::TEXT_SECONDARY));
            });
            return;
        }

        // Chat messages area
        let available_height = ui.available_height();
        let input_height = 120.0;
        let messages_height = available_height - input_height - Theme::SPACING_LARGE * 2.0;

        egui::ScrollArea::vertical()
            .max_height(messages_height)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                if self.messages.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.label(egui::RichText::new("💬").size(48.0));
                        ui.add_space(Theme::SPACING_SMALL);
                        ui.label(egui::RichText::new("Start a conversation").size(Theme::FONT_SIZE_SUBHEADING).color(Theme::TEXT_PRIMARY));
                        ui.add_space(Theme::SPACING_SMALL);
                        ui.label(egui::RichText::new("Type a message below to chat with the AI assistant").color(Theme::TEXT_SECONDARY));
                    });
                } else {
                    for message in &self.messages {
                        self.render_message(ui, message);
                        ui.add_space(Theme::SPACING_MEDIUM);
                    }
                    
                    // Show loading indicator
                    if self.is_loading {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("🤖").size(Theme::FONT_SIZE_SUBHEADING));
                            ui.add_space(Theme::SPACING_SMALL);
                            ui.spinner();
                            ui.label(egui::RichText::new("Thinking...").color(Theme::TEXT_SECONDARY));
                        });
                    }
                }
            });

        ui.add_space(Theme::SPACING_LARGE);

        // Input area
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(ui.available_width() - 80.0);
                
                let text_edit = egui::TextEdit::multiline(&mut self.input_text)
                    .hint_text("Type your message here...")
                    .desired_width(f32::INFINITY)
                    .desired_rows(3)
                    .font(egui::TextStyle::Body);
                
                let response = ui.add(text_edit);
                
                // Handle Enter key to send (Shift+Enter for new line)
                if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && !i.modifiers.shift) {
                    if !self.input_text.trim().is_empty() && !self.is_loading {
                        self.send_message();
                    }
                }
            });
            
            ui.vertical(|ui| {
                ui.add_space(20.0);
                let button = if self.is_loading {
                    ui.add_enabled(false, Theme::primary_button("Sending..."))
                } else {
                    ui.add_enabled(!self.input_text.trim().is_empty(), Theme::primary_button("Send"))
                };
                
                if button.clicked() {
                    self.send_message();
                }
            });
        });
    }

    fn render_message(&self, ui: &mut egui::Ui, message: &ChatMessage) {
        let (icon, bg_color, align) = match message.role {
            MessageRole::User => ("👤", egui::Color32::from_rgb(33, 150, 243), egui::Align::Max),
            MessageRole::Assistant => ("🤖", egui::Color32::from_rgb(76, 175, 80), egui::Align::Min)
        };

        ui.with_layout(egui::Layout::top_down(align), |ui| {
            egui::Frame::new()
                .fill(bg_color.linear_multiply(0.1))
                .corner_radius(8.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_max_width(ui.available_width() * 0.75);
                    
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(icon).size(Theme::FONT_SIZE_SUBHEADING));
                        ui.label(egui::RichText::new(message.role.as_str()).strong().color(bg_color));
                    });
                    
                    ui.add_space(Theme::SPACING_SMALL);
                    
                    ui.label(egui::RichText::new(&message.content)
                        .size(Theme::FONT_SIZE_BODY)
                        .color(Theme::TEXT_PRIMARY));
                });
        });
    }

    fn send_message(&mut self) {
        let user_message = self.input_text.trim().to_string();
        if user_message.is_empty() {
            return;
        }

        // Add user message to chat
        self.messages.push(ChatMessage::user(user_message.clone()));
        self.input_text.clear();
        self.is_loading = true;

        // Clone messages for async task
        let messages_clone = self.messages.clone();
        let tx = self.response_tx.clone();

        // Spawn async task to get response
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            match runtime.block_on(AssistantService::send_message(&messages_clone)) {
                Ok(response) => {
                    log_service::add_log_entry("INFO", "Received AI response");
                    let _ = tx.send(AsyncResponse::Message(response));
                }
                Err(e) => {
                    log_service::add_log_entry("ERROR", &format!("AI request failed: {}", e));
                    let _ = tx.send(AsyncResponse::Error(e));
                }
            }
        });
    }

    fn check_server_status_async(&mut self) {
        let tx = self.response_tx.clone();
        std::thread::spawn(move || {
            let runtime = tokio::runtime::Runtime::new().unwrap();
            let is_available = runtime.block_on(AssistantService::check_server_status());
            
            if is_available {
                log_service::add_log_entry("INFO", "Ollama server is available");
            } else {
                log_service::add_log_entry("WARNING", "Ollama server is not available");
            }
            let _ = tx.send(AsyncResponse::ServerStatus(is_available));
        });
    }
}
