// src/dal/repositories/mod.rs
pub mod notes_repository;
pub mod feeds_repository;
pub mod bookmarks_repository;

pub use notes_repository::NotesRepository;
pub use feeds_repository::{FeedsRepository, FeedItemsRepository};
pub use bookmarks_repository::BookmarksRepository;
