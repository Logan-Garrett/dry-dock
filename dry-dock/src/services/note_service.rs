// src/services/note_service.rs
use crate::dal::NotesRepository;
use crate::models::Note;

/// Note Service - Business Logic Layer for Notes
/// This layer sits between the UI and the Data Access Layer (Repository)
/// All business logic and data transformations should be handled here
pub struct NoteService;

impl NoteService {
    /// Create a new note
    /// Validates input and delegates to repository
    pub fn create_note(title: &str, details: &str) -> Result<(), String> {
        // BLL: Validate inputs
        if title.trim().is_empty() {
            return Err("Note title cannot be empty".to_string());
        }
        
        if details.trim().is_empty() {
            return Err("Note details cannot be empty".to_string());
        }
        
        // Delegate to repository
        NotesRepository::create(title, details)
    }
    
    /// Delete a note by ID
    pub fn delete_note(note_id: i32) -> Result<(), String> {
        // BLL: Could add authorization checks, logging, etc. here
        NotesRepository::delete(note_id)
    }
    
    /// Get all notes as a list of Note models
    /// Transforms repository data into domain models
    pub fn get_all_notes() -> Result<Vec<Note>, String> {
        // Get raw data from repository
        let raw_notes = NotesRepository::get_all()?;
        
        // BLL: Transform tuples into Note models
        let notes = raw_notes
            .into_iter()
            .map(|(id, title, details, created_at, updated_at)| {
                Note::new(id, title, details, created_at, updated_at)
            })
            .collect();
        
        Ok(notes)
    }
    
    /// Search notes by title or content
    /// BLL: Implements search logic
    pub fn search_notes(query: &str) -> Result<Vec<Note>, String> {
        let all_notes = Self::get_all_notes()?;
        
        let query_lower = query.to_lowercase();
        let filtered = all_notes
            .into_iter()
            .filter(|note| {
                note.title.to_lowercase().contains(&query_lower) 
                || note.details.to_lowercase().contains(&query_lower)
            })
            .collect();
        
        Ok(filtered)
    }

    /// Get a note by ID
    pub fn get_note_by_id(note_id: i32) -> Result<Note, String> {
        let (id, title, details, created_at, updated_at) = NotesRepository::get_by_id(note_id)?;
        Ok(Note::new(id, title, details, created_at, updated_at))
    }

    /// Update a note
    pub fn update_note(note_id: i32, title: &str, details: &str) -> Result<(), String> {
        // BLL: Validate inputs
        if title.trim().is_empty() {
            return Err("Note title cannot be empty".to_string());
        }
        
        if details.trim().is_empty() {
            return Err("Note details cannot be empty".to_string());
        }
        
        // Delegate to repository
        NotesRepository::update(note_id, title, details)
    }
}
