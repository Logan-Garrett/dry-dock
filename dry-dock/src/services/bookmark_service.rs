// src/servicces/bookmark_service.rs

// The goal of this serviuce it to store, retrieve, and manage bookmarks within Dry Dock.

// These BookMarks can be URLS, file paths and maybe other things down the line.
// With url and file path we own the folder or the actual file.

use rusqlite::params;

use crate::common::helper::current_timestamp;

/////
/// BLL Functions for Bookmark Management
/// As things get more complex we can add more logic here.
/////
pub fn add_new_bookmark(name: &str, path: &str) -> Result<(), String> {
    add_bookmark(name, path)
}

pub fn delete_bookmark(id: i32) -> Result<(), String> {
    remove_bookmark(id)
}

pub fn edit_bookmark(id: i32, name: &str, path: &str) -> Result<(), String> {
    update_bookmark(id, name, path)
}

pub fn fetch_bookmark(id: i32) -> Result<(i32, String, String, String), String> {
    get_bookmark(id)
}

pub fn fetch_all_bookmarks() -> Result<Vec<(i32, String, String, String)>, String> {
    get_bookmarks()
}

pub fn open_bookmark_path(path: &str) {
    // Are we opening a URL or a file path? (Down the line we can add more types)
    if path.starts_with("http://") || path.starts_with("https://") {
        // Open URL in default browser
        if let Err(e) = webbrowser::open(path) {
            println!("Failed to open URL: {}", e);
        } else {
            println!("Opening URL: {}", path);
        }
    } else if std::path::Path::new(path).exists() {
        // Open file path with default application
        if let Err(e) = opener::open(path) {
            println!("Failed to open file path: {}", e);
        } else {
            println!("Opening file: {}", path);
        }
    } else {
        println!("Path does not exist: {}", path);
    }
}


/////
/// DAL Functions for Bookmark Management
/////
fn add_bookmark(name: &str, path: &str) -> Result<(), String> {
    let conn = crate::dal::db_context::get_connection()?;
    let now = current_timestamp();

    conn.execute(
        "INSERT INTO bookmarks (name, location, created_at) VALUES (?1, ?2, ?3)",
        params![name, path, now],
    )
    .map_err(|e| format!("Failed to add bookmark: {}", e))?;

    Ok(())
}

fn remove_bookmark(id: i32) -> Result<(), String> {
    let conn = crate::dal::db_context::get_connection()?;

    conn.execute(
        "DELETE FROM bookmarks WHERE id = ?1",
        params![id],
    )
    .map_err(|e| format!("Failed to remove bookmark: {}", e))?;

    Ok(())
}

fn update_bookmark(id: i32, name: &str, path: &str) -> Result<(), String> {
    let conn = crate::dal::db_context::get_connection()?;

    conn.execute(
        "UPDATE bookmarks SET name = ?1, location = ?2 WHERE id = ?3",
        params![name, path, id],
    )
    .map_err(|e| format!("Failed to update bookmark: {}", e))?;

    Ok(())
}

fn get_bookmark(id: i32) -> Result<(i32, String, String, String), String> {
    let conn = crate::dal::db_context::get_connection()?;
    let mut stmt = conn
        .prepare("SELECT id, name, location, created_at FROM bookmarks WHERE id = ?1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let bookmark = stmt
        .query_row(params![id], |row| {
            Ok((
                row.get("id")?,
                row.get("name")?,
                row.get("location")?,
                row.get("created_at")?,
            ))
        })
        .map_err(|e| format!("Failed to get bookmark: {}", e))?;

    Ok(bookmark)
}

fn get_bookmarks() -> Result<Vec<(i32, String, String, String)>, String> {
    let conn = crate::dal::db_context::get_connection()?;
    let mut stmt = conn
        .prepare("SELECT id, name, location, created_at FROM bookmarks")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let bookmark_iter = stmt
        .query_map([], |row| {
            Ok((
                row.get("id")?,
                row.get("name")?,
                row.get("location")?,
                row.get("created_at")?,
            ))
        })
        .map_err(|e| format!("Failed to query bookmarks: {}", e))?;

    let mut bookmarks = Vec::new();
    for bookmark in bookmark_iter {
        bookmarks.push(bookmark.map_err(|e| format!("Failed to map bookmark: {}", e))?);
    }

    Ok(bookmarks)
}