// src/app/active_screen.rs

/// Enum to represent the active screen
/// Every screen in the app should be represented here.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActiveScreen {
    None,
    Feeds,
    Notes,
    Assistant,
    Terminal,
    Bookmarks,
}
