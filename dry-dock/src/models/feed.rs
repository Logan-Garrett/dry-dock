// src/models/feed.rs

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub description: String,
    pub pub_date: i64,
}

impl FeedItem {
    pub fn new(id: i32, title: String, link: String, description: String, pub_date: i64) -> Self {
        Self {
            id,
            title,
            link,
            description,
            pub_date,
        }
    }
}
