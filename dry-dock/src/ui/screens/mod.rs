// src/ui/screens/mod.rs
pub mod screen_trait;
pub mod feeds_screen;
pub mod notes_screen;
pub mod bookmarks_screen;
pub mod screen_factory;

pub use feeds_screen::FeedsScreen;
pub use notes_screen::NotesScreen;
pub use bookmarks_screen::BookmarksScreen;
pub use screen_factory::ScreenFactory;
