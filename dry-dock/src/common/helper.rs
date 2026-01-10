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

pub fn load_config_file() -> Result<String, std::io::Error> {
    // Try 1: Current directory (for development)
    if let Ok(data) = std::fs::read_to_string("AppConfig.json") {
        return Ok(data);
    }

    // Try 2: macOS app bundle resources
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(bundle_resources) = exe_path
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("Resources").join("AppConfig.json"))
        {
            if let Ok(data) = std::fs::read_to_string(&bundle_resources) {
                return Ok(data);
            }
        }
    }

    // Try 3: User's config directory
    if let Some(config_dir) = dirs::config_dir() {
        let user_config = config_dir.join("DryDock").join("AppConfig.json");
        if let Ok(data) = std::fs::read_to_string(&user_config) {
            return Ok(data);
        }
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "AppConfig.json not found in any expected location",
    ))
}

pub fn load_icon_path(config_icon_path: &str) -> String {
    // Try 1: Current directory (for development)
    if std::path::Path::new(config_icon_path).exists() {
        return config_icon_path.to_string();
    }

    // Try 2: macOS app bundle resources
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(bundle_icon) = exe_path
            .parent()
            .and_then(|p| p.parent())
            .map(|p| p.join("Resources").join(config_icon_path))
        {
            if bundle_icon.exists() {
                return bundle_icon.to_string_lossy().to_string();
            }
        }
    }

    // Fallback to original path
    config_icon_path.to_string()
}