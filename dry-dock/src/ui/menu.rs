// src/ui/menu.rs
use eframe::egui;
use crate::models::Config;
use crate::ui::modals::ActiveModal;
use crate::app::ActiveScreen;
use crate::ui::styles::Theme;
use crate::services::log_service;

/// Renders the application menu bar
pub fn render_menu(ui: &mut egui::Ui, config: &Config, state: &mut crate::app::AppState) {
    Theme::apply_menu_style(ui);

    ui.menu_button(&config.app_name, |ui| {
        let settings_button = Theme::button("Settings");
        let logs_button = Theme::button("View Logs");
        let exit_button = Theme::button("Exit");
        
        if ui.add(settings_button).clicked() {
            log_service::add_log_entry("INFO", "Loading Settings...");
            state.open_modal(ActiveModal::Settings);
        }

        if ui.add(logs_button).clicked() {
            log_service::add_log_entry("INFO", "Opening Logs Directory...");
            // Need to create a modal and either store in sqllite or a file.
            // Need to add a Log service to call instead of println as well as
            // Need a verbose flaf in the config or settings whatever it may be.
            state.open_modal(ActiveModal::LogModal);
        }
        
        if ui.add(exit_button).clicked() {
            std::process::exit(0);
        }
    });

    ui.separator();

    // RSS Button (no dropdown)
    let rss_button = ui.button("RSS");
    if rss_button.clicked() {
        log_service::add_log_entry("INFO", "Loading RSS Feeds Screen...");
        state.close_modal();
        state.set_active_screen(ActiveScreen::Feeds);
    }

    if state.get_active_screen() == ActiveScreen::Feeds {
        rss_button.highlight();
    }

    ui.separator();

    // Notes Button (no dropdown)
    let notes_button = ui.button("Notes");
    if notes_button.clicked() {
        log_service::add_log_entry("INFO", "Loading Notes Screen...");
        state.close_modal();
        state.set_active_screen(ActiveScreen::Notes);
    }

    if state.get_active_screen() == ActiveScreen::Notes {
        notes_button.highlight();
    }

    ui.separator();

    // For now Commmenting out Assitant and Terminal until implemented

    // Assistant Button
    let assistant_button = ui.button("Assistant");
    if assistant_button.clicked() {
        log_service::add_log_entry("INFO", "Loading Assistant Screen...");
        state.close_modal();
        state.set_active_screen(ActiveScreen::Assistant);
    }

    if state.get_active_screen() == ActiveScreen::Assistant {
        assistant_button.highlight();
    }

    ui.separator();

    /*
    // Terminal Button
    let terminal_button = ui.button("Terminal");
    if terminal_button.clicked() {
        println!("Loading Terminal Screen...");
        state.close_modal();
        state.set_active_screen(ActiveScreen::Terminal);
    }

    if state.get_active_screen() == ActiveScreen::Terminal {
        terminal_button.highlight();
    }

    ui.separator();
    */

    // Bookmarks Button
    let bookmarks_button = ui.button("Bookmarks");
    if bookmarks_button.clicked() {
        log_service::add_log_entry("INFO", "Loading Bookmarks Manager...");
        state.set_active_screen(ActiveScreen::Bookmarks);
    }

    if state.get_active_screen() == ActiveScreen::Bookmarks {
        bookmarks_button.highlight();
    }
}
