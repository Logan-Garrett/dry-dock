// src/dal/repositories/bookmarks_repository.rs
use crate::dal::db_context::get_connection;
use rusqlite::params;

/// Bookmarks Repository - handles all bookmark-related database operations
pub struct BookmarksRepository;

impl BookmarksRepository {
    /// Create a new bookmark
    pub fn create(name: &str, location: &str) -> Result<(), String> {
        let conn = get_connection()?;
        let now = chrono::Utc::now().timestamp();

        conn.execute(
            "INSERT INTO bookmarks (name, location, created_at) VALUES (?1, ?2, ?3)",
            params![name, location, now],
        )
        .map_err(|e| format!("Failed to create bookmark: {}", e))?;

        Ok(())
    }

    /// Get all bookmarks
    pub fn get_all() -> Result<Vec<(i32, String, String, String)>, String> {
        let conn = get_connection()?;

        let mut stmt = conn
            .prepare("SELECT id, name, location, created_at FROM bookmarks ORDER BY name ASC")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let bookmarks = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>("id")?,
                    row.get::<_, String>("name")?,
                    row.get::<_, String>("location")?,
                    row.get::<_, i64>("created_at")?,
                ))
            })
            .map_err(|e| format!("Failed to query bookmarks: {}", e))?
            .collect::<Result<Vec<(i32, String, String, i64)>, _>>()
            .map_err(|e| format!("Failed to collect bookmarks: {}", e))?;

        // Convert timestamps to formatted strings
        let formatted_bookmarks = bookmarks
            .into_iter()
            .map(|(id, name, location, timestamp)| {
                let created_at_str = chrono::DateTime::from_timestamp(timestamp, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                (id, name, location, created_at_str)
            })
            .collect();

        Ok(formatted_bookmarks)
    }

    /// Delete a bookmark
    pub fn delete(bookmark_id: i32) -> Result<(), String> {
        let conn = get_connection()?;

        conn.execute(
            "DELETE FROM bookmarks WHERE id = ?1",
            params![bookmark_id],
        )
        .map_err(|e| format!("Failed to delete bookmark: {}", e))?;

        Ok(())
    }
}
