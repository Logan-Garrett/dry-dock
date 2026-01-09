// src/common/helper.rs
use chrono::Datelike;

// Helper to load the current year as a string
pub fn load_current_year() -> String {
    let current_year = chrono::Utc::now().year();
    current_year.to_string()
}

// Helper to get platform-specific database path
pub fn get_database_path(app_name: &str) -> String {
    let mut path = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."));

    path.push(app_name);
    std::fs::create_dir_all(&path).ok();
    
    path.push("database.db");
    path.to_string_lossy().to_string()
}