// src/ui/modals/active_modal.rs

/// Enum to represent the active modal state
/// Every modal in the app should be represented here.
#[derive(Debug, Clone, PartialEq)]
pub enum ActiveModal {
    None,
    AddFeed,
    CreateNote,
    AddBookmark,
    Settings,
}
