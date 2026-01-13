// src/ui/modals/active_modal.rs

/// Enum to represent the active modal state
/// Every modal in the app should be represented here.
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveModal {
    None,
    AddFeed,
    CreateNote,
    AddBookmark,
    UpdateNote(i32),        // Note ID
    UpdateBookmark(i32),    // Bookmark ID
    ViewNote(i32),          // Note ID for viewing
    LogModal,
    ManageFeeds,
    Settings,
}
