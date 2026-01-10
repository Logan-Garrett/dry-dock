// src/models/note.rs

#[derive(Debug, Clone)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub details: String,
    pub created_at: i64,
    pub updated_at: Option<i64>,
}

impl Note {
    pub fn new(id: i32, title: String, details: String, created_at: i64, updated_at: Option<i64>) -> Self {
        Self {
            id,
            title,
            details,
            created_at,
            updated_at,
        }
    }
}
