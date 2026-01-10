// src/ui/screens/screen_trait.rs

/// Trait that all screens must implement
pub trait Screen {
    fn title(&self) -> &str;
}
