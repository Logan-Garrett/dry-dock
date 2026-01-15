// src/models/mod.rs
pub mod config;
pub mod note;
pub mod feed;
pub mod bookmark;
pub mod chat_message;

pub use config::Config;
pub use note::Note;
pub use feed::FeedItem;
pub use bookmark::Bookmark;
pub use chat_message::{ChatMessage, MessageRole};
