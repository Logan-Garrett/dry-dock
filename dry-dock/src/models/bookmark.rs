// src/models/bookmark.rs

#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub created_at: String,
}

impl Bookmark {
    pub fn new(id: i32, name: String, path: String, created_at: String) -> Self {
        Self {
            id,
            name,
            path,
            created_at,
        }
    }
}
