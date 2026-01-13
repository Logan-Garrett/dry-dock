// src/services/bookmark_service.rs

// The goal of this service is to store, retrieve, and manage bookmarks within Dry Dock.

// These BookMarks can be URLS, file paths and maybe other things down the line.
// With url and file path we own the folder or the actual file.

use crate::dal::BookmarksRepository;
use super::log_service;

/////
/// BLL Functions for Bookmark Management
/// As things get more complex we can add more logic here.
/////
pub fn add_new_bookmark(name: &str, path: &str) -> Result<(), String> {
    BookmarksRepository::create(name, path)
}

pub fn delete_bookmark(id: i32) -> Result<(), String> {
    BookmarksRepository::delete(id)
}

pub fn fetch_all_bookmarks() -> Result<Vec<(i32, String, String, String)>, String> {
    BookmarksRepository::get_all()
}

pub fn open_bookmark_path(path: &str) {
    // Are we opening a URL or a file path? (Down the line we can add more types)
    if path.starts_with("http://") || path.starts_with("https://") {
        // Open URL in default browser
        if let Err(e) = webbrowser::open(path) {
            log_service::add_log_entry("ERROR", &format!("Failed to open URL: {}", e));
        } else {
            log_service::add_log_entry("INFO", &format!("Opening URL: {}", path));
        }
    } else if std::path::Path::new(path).exists() {
        // Open file/folder path
        if let Err(e) = opener::open(path) {
            log_service::add_log_entry("ERROR", &format!("Failed to open file path: {}", e));
        } else {
            log_service::add_log_entry("INFO", &format!("Opening file: {}", path));
        }
    } else {
        log_service::add_log_entry("ERROR", &format!("Path does not exist: {}", path));
    }
}