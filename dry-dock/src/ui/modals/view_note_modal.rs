// src/ui/modals/view_note_modal.rs
use eframe::egui;
use pulldown_cmark::{Parser, Event, Tag, TagEnd, HeadingLevel};
use crate::ui::modals::modal_trait::Modal;
use crate::services::NoteService;
use crate::ui::styles::Theme;

pub struct ViewNoteModal {
    note_id: i32,
    title: String,
    details: String,
    loaded: bool,
}

impl ViewNoteModal {
    pub fn new(note_id: i32) -> Self {
        Self {
            note_id,
            title: String::new(),
            details: String::new(),
            loaded: false,
        }
    }

    fn render_markdown(&self, ui: &mut egui::Ui, markdown: &str) {
        let parser = Parser::new(markdown);
        let mut in_heading = false;
        let mut heading_level = 1;
        let mut in_code_block = false;
        let mut in_emphasis = false;
        let mut in_strong = false;
        let mut in_list = false;
        let mut list_depth = 0;
        let mut in_list_item = false;

        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    heading_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                }
                Event::End(TagEnd::Heading(_)) => {
                    in_heading = false;
                    ui.add_space(Theme::SPACING_MEDIUM);
                }
                Event::Start(Tag::Paragraph) => {
                    if !in_list_item {
                        // Only start a new line for paragraphs outside lists
                    }
                }
                Event::End(TagEnd::Paragraph) => {
                    if !in_list_item {
                        ui.add_space(Theme::SPACING_MEDIUM);
                    }
                }
                Event::Start(Tag::CodeBlock(_)) => {
                    in_code_block = true;
                    ui.add_space(Theme::SPACING_SMALL);
                }
                Event::End(TagEnd::CodeBlock) => {
                    in_code_block = false;
                    ui.add_space(Theme::SPACING_MEDIUM);
                }
                Event::Start(Tag::Emphasis) => {
                    in_emphasis = true;
                }
                Event::End(TagEnd::Emphasis) => {
                    in_emphasis = false;
                }
                Event::Start(Tag::Strong) => {
                    in_strong = true;
                }
                Event::End(TagEnd::Strong) => {
                    in_strong = false;
                }
                Event::Start(Tag::List(_)) => {
                    in_list = true;
                    list_depth += 1;
                    if list_depth == 1 {
                        ui.add_space(Theme::SPACING_SMALL);
                    }
                }
                Event::End(TagEnd::List(_)) => {
                    list_depth -= 1;
                    if list_depth == 0 {
                        in_list = false;
                        ui.add_space(Theme::SPACING_MEDIUM);
                    }
                }
                Event::Start(Tag::Item) => {
                    in_list_item = true;
                }
                Event::End(TagEnd::Item) => {
                    in_list_item = false;
                    ui.add_space(Theme::SPACING_SMALL);
                }
                Event::Text(text) => {
                    let text_str = text.to_string();
                    
                    if in_list_item && !in_heading {
                        // Render list item with bullet on the same line as text
                        ui.horizontal(|ui| {
                            ui.add_space((list_depth - 1) as f32 * 20.0);
                            ui.label(egui::RichText::new("•")
                                .size(Theme::FONT_SIZE_BODY)
                                .color(Theme::PRIMARY_COLOR));
                            ui.add_space(8.0);
                            
                            let mut rich_text = egui::RichText::new(text_str)
                                .size(Theme::FONT_SIZE_BODY)
                                .color(Theme::TEXT_SECONDARY);
                            
                            if in_strong {
                                rich_text = rich_text.strong();
                            }
                            if in_emphasis {
                                rich_text = rich_text.italics();
                            }
                            
                            ui.label(rich_text);
                        });
                    } else {
                        // Regular text rendering
                        let mut rich_text = egui::RichText::new(text_str);
                        
                        if in_heading {
                            let size = match heading_level {
                                1 => Theme::FONT_SIZE_HEADING,
                                2 => Theme::FONT_SIZE_SUBHEADING,
                                _ => Theme::FONT_SIZE_BODY + 2.0,
                            };
                            rich_text = rich_text.size(size).strong().color(Theme::TEXT_PRIMARY);
                        } else if in_code_block {
                            rich_text = rich_text
                                .family(egui::FontFamily::Monospace)
                                .background_color(Theme::CARD_BG)
                                .color(Theme::SUCCESS_COLOR);
                        } else {
                            rich_text = rich_text.size(Theme::FONT_SIZE_BODY).color(Theme::TEXT_SECONDARY);
                        }
                        
                        if in_strong && !in_heading {
                            rich_text = rich_text.strong();
                        }
                        if in_emphasis {
                            rich_text = rich_text.italics();
                        }
                        
                        ui.label(rich_text);
                    }
                }
                Event::Code(code) => {
                    ui.label(
                        egui::RichText::new(code.to_string())
                            .family(egui::FontFamily::Monospace)
                            .background_color(Theme::CARD_BG)
                            .color(Theme::SUCCESS_COLOR)
                    );
                }
                Event::SoftBreak => {
                    if !in_list_item {
                        ui.label(" ");
                    }
                }
                Event::HardBreak => {
                    ui.add_space(Theme::SPACING_SMALL);
                }
                Event::Rule => {
                    ui.add_space(Theme::SPACING_MEDIUM);
                    ui.separator();
                    ui.add_space(Theme::SPACING_MEDIUM);
                }
                _ => {}
            }
        }
    }
}

impl Modal for ViewNoteModal {
    fn title(&self) -> &str {
        "View Note"
    }
    
    fn render(&mut self, ui: &mut egui::Ui) -> bool {
        let mut should_close = false;
        
        // Load note data on first render
        if !self.loaded {
            match NoteService::get_note_by_id(self.note_id) {
                Ok(note) => {
                    self.title = note.title;
                    self.details = note.details;
                    self.loaded = true;
                }
                Err(e) => {
                    ui.colored_label(Theme::DANGER_COLOR, format!("Error loading note: {}", e));
                    return true; // Close modal on error
                }
            }
        }
        
        Theme::apply_body_style(ui);
        
        ui.add_space(Theme::SPACING_MEDIUM);
        
        // Display title
        ui.label(egui::RichText::new(&self.title)
            .size(Theme::FONT_SIZE_HEADING)
            .strong()
            .color(Theme::TEXT_PRIMARY));
        
        ui.add_space(Theme::SPACING_MEDIUM);
        ui.separator();
        ui.add_space(Theme::SPACING_MEDIUM);

        // Render markdown content in a scrollable area
        egui::ScrollArea::vertical()
            .max_height(ui.available_height() - 80.0)
            .show(ui, |ui| {
                self.render_markdown(ui, &self.details);
            });
        
        ui.add_space(Theme::SPACING_LARGE);
        
        ui.horizontal(|ui| {
            if ui.add(Theme::button("Close")).clicked() {
                should_close = true;
            }
        });
        
        should_close
    }
}
