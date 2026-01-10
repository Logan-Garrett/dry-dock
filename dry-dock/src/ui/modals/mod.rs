// src/ui/modals/mod.rs
pub mod modal_trait;
pub mod active_modal;
pub mod add_feed_modal;
pub mod create_note_modal;
pub mod add_bookmark_modal;
pub mod settings_modal;
pub mod modal_factory;

pub use modal_trait::Modal;
pub use active_modal::ActiveModal;
pub use add_feed_modal::AddFeedModal;
pub use create_note_modal::CreateNoteModal;
pub use add_bookmark_modal::AddBookmarkModal;
pub use settings_modal::SettingsModal;
pub use modal_factory::ModalFactory;
