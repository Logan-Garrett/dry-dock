// src/dal/repositories/feeds_repository.rs
use crate::dal::db_context::get_connection;
use rusqlite::params;

/// Feeds Repository - handles all RSS feed-related database operations
pub struct FeedsRepository;

impl FeedsRepository {
    /// Add a new feed
    pub fn create(url: &str, title: &str) -> Result<(), String> {
        let conn = get_connection()?;
        let now = chrono::Utc::now().timestamp();

        conn.execute(
            "INSERT INTO feeds (title, url, created_at) VALUES (?1, ?2, ?3)",
            params![title, url, now],
        )
        .map_err(|e| format!("Failed to add feed: {}", e))?;

        Ok(())
    }

    /// Get all feeds
    pub fn get_all() -> Result<Vec<(i32, String, String)>, String> {
        let conn = get_connection()?;
        
        let mut stmt = conn
            .prepare("SELECT id, url, title FROM feeds")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let feeds = stmt
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .map_err(|e| format!("Failed to query feeds: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect feeds: {}", e))?;
        
        Ok(feeds)
    }

    /// Update feed's last_updated timestamp
    pub fn update_last_updated(feed_id: i32, timestamp: i64) -> Result<(), String> {
        let conn = get_connection()?;
        
        conn.execute(
            "UPDATE feeds SET last_updated = ?1 WHERE id = ?2",
            params![timestamp, feed_id],
        )
        .map_err(|e| format!("Failed to update feed timestamp: {}", e))?;
        
        Ok(())
    }

    /// Delete a feed by ID
    pub fn delete(feed_id: i32) -> Result<(), String> {
        let conn = get_connection()?;

        conn.execute(
            "DELETE FROM feeds WHERE id = ?1",
            params![feed_id],
        )
        .map_err(|e| format!("Failed to delete feed: {}", e))?;

        Ok(())
    }
}

/// Feed Items Repository - handles feed item operations
pub struct FeedItemsRepository;

impl FeedItemsRepository {
    /// Get feed items with optional limit
    pub fn get_latest(limit: i32) -> Result<Vec<(i32, String, String, String, i64)>, String> {
        let conn = get_connection()?;

        let mut stmt = conn
            .prepare("SELECT id, title, link, description, pub_date FROM feed_items ORDER BY pub_date DESC LIMIT ?1")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let mut items = stmt
            .query_map(params![limit], |row| {
                Ok((
                    row.get::<_, i32>("id")?,
                    row.get::<_, String>("title")?,
                    row.get::<_, String>("link")?,
                    row.get::<_, String>("description")?,
                    row.get::<_, i64>("pub_date")?,
                ))
            })
            .map_err(|e| format!("Failed to query feed items: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect feed items: {}", e))?;

        // Sort by pub_date DESC
        items.sort_by(|a, b| b.4.cmp(&a.4));
        
        Ok(items)
    }

    /// Insert or ignore feed item (based on guid uniqueness)
    pub fn insert_or_ignore(
        feed_id: i32,
        title: &str,
        link: &str,
        description: &str,
        pub_date: i64,
        guid: &str,
        created_at: i64,
    ) -> Result<bool, String> {
        let conn = get_connection()?;
        
        let rows_affected = conn.execute(
            "INSERT OR IGNORE INTO feed_items (feed_id, title, link, description, pub_date, guid, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![feed_id, title, link, description, pub_date, guid, created_at],
        )
        .map_err(|e| format!("Failed to insert feed item: {}", e))?;
        
        Ok(rows_affected == 1) // true if inserted, false if already existed
    }
}
