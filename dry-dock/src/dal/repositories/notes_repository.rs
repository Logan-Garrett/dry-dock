// src/dal/repositories/notes_repository.rs
use crate::dal::db_context::get_connection;
use rusqlite::params;

/// Notes Repository - handles all note-related database operations
pub struct NotesRepository;

impl NotesRepository {
    /// Create a new note
    pub fn create(title: &str, details: &str) -> Result<(), String> {
        let conn = get_connection()?;
        let now = chrono::Utc::now().timestamp();

        conn.execute(
            "INSERT INTO notes (title, details, created_at) VALUES (?1, ?2, ?3)",
            params![title, details, now],
        )
        .map_err(|e| format!("Failed to create note: {}", e))?;

        Ok(())
    }

    /// Delete a note by ID
    pub fn delete(note_id: i32) -> Result<(), String> {
        let conn = get_connection()?;

        conn.execute(
            "DELETE FROM notes WHERE id = ?1",
            params![note_id],
        )
        .map_err(|e| format!("Failed to delete note: {}", e))?;

        Ok(())
    }

    /// Get all notes
    pub fn get_all() -> Result<Vec<(i32, String, String, i64, Option<i64>)>, String> {
        let conn = get_connection()?;

        let mut stmt = conn
            .prepare("SELECT id, title, details, created_at, updated_at FROM notes ORDER BY created_at DESC")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let notes = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>("id")?,
                    row.get::<_, String>("title")?,
                    row.get::<_, String>("details")?,
                    row.get::<_, i64>("created_at")?,
                    row.get::<_, Option<i64>>("updated_at")?,
                ))
            })
            .map_err(|e| format!("Failed to query notes: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect notes: {}", e))?;

        Ok(notes)
    }
}
