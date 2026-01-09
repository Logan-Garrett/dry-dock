use crate::dal::db_context::*;
use rusqlite::params;
use chrono::{DateTime, Utc};

pub fn fetch_and_store_feed(feed_id: i32, feed_url: &str) -> Result<usize, String> {
    // Trim and validate URL
    let mut feed_url = feed_url.trim().to_string();
    
    if feed_url.is_empty() {
        return Err("Feed URL is empty".to_string());
    }
    
    // Auto-fix URLs missing protocol - prepend https://
    if !feed_url.starts_with("http://") && !feed_url.starts_with("https://") {
        println!("URL missing protocol, adding https:// to: {}", feed_url);
        feed_url = format!("https://{}", feed_url);
    }
    
    println!("Fetching feed from: {}", feed_url);
    
    // Create a client with proper configuration
    let client = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .redirect(reqwest::redirect::Policy::limited(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // Fetch the RSS feed
    let response = client.get(&feed_url)
        .send()
        .map_err(|e| format!("Failed to fetch feed from '{}': {}", feed_url, e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {} for URL: {}", response.status(), feed_url));
    }
    
    let content = response.text()
        .map_err(|e| format!("Failed to read feed content: {}", e))?;
    
    // Try parsing as RSS first, then Atom
    let items_added = if let Ok(items) = parse_rss(&content) {
        store_feed_items(feed_id, items)?
    } else if let Ok(items) = parse_atom(&content) {
        store_feed_items(feed_id, items)?
    } else {
        return Err("Failed to parse feed as RSS or Atom".to_string());
    };
    
    // Update last_updated timestamp for the feed
    let conn = get_connection()?;
    let now = Utc::now().timestamp();
    conn.execute(
        "UPDATE feeds SET last_updated = ?1 WHERE id = ?2",
        params![now, feed_id],
    )
    .map_err(|e| format!("Failed to update feed timestamp: {}", e))?;
    
    Ok(items_added)
}

struct FeedItem {
    title: String,
    link: String,
    description: String,
    pub_date: i64,
    guid: String,
}

fn parse_rss(content: &str) -> Result<Vec<FeedItem>, String> {
    let channel = rss::Channel::read_from(content.as_bytes())
        .map_err(|e| format!("RSS parse error: {}", e))?;
    
    let items = channel.items()
        .iter()
        .map(|item| {
            let title = item.title()
                .unwrap_or("Untitled")
                .to_string();
            
            let link = item.link()
                .unwrap_or("")
                .to_string();
            
            let description = item.description()
                .unwrap_or("")
                .to_string();
            
            let pub_date = item.pub_date()
                .and_then(|d| DateTime::parse_from_rfc2822(d).ok())
                .map(|dt| dt.timestamp())
                .unwrap_or_else(|| Utc::now().timestamp());
            
            let guid = item.guid()
                .map(|g| g.value().to_string())
                .unwrap_or_else(|| link.clone());
            
            FeedItem {
                title,
                link,
                description,
                pub_date,
                guid,
            }
        })
        .collect();
    
    Ok(items)
}

fn parse_atom(content: &str) -> Result<Vec<FeedItem>, String> {
    let feed = atom_syndication::Feed::read_from(content.as_bytes())
        .map_err(|e| format!("Atom parse error: {}", e))?;
    
    let items = feed.entries()
        .iter()
        .map(|entry| {
            let title = entry.title().to_string();
            
            let link = entry.links()
                .first()
                .map(|l| l.href().to_string())
                .unwrap_or_default();
            
            let description = entry.summary()
                .map(|s| s.to_string())
                .or_else(|| entry.content().and_then(|c| c.value().map(|v| v.to_string())))
                .unwrap_or_default();
            
            let pub_date = entry.published()
                .or_else(|| Some(entry.updated()))
                .map(|dt| dt.timestamp())
                .unwrap_or_else(|| Utc::now().timestamp());
            
            let guid = entry.id().to_string();
            
            FeedItem {
                title,
                link,
                description,
                pub_date,
                guid,
            }
        })
        .collect();
    
    Ok(items)
}

fn store_feed_items(feed_id: i32, items: Vec<FeedItem>) -> Result<usize, String> {
    let conn = get_connection()?;
    let now = Utc::now().timestamp();
    let mut items_added = 0;
    
    for item in items {
        // Insert or ignore if already exists (based on guid)
        match conn.execute(
            "INSERT OR IGNORE INTO feed_items (feed_id, title, link, description, pub_date, guid, created_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![feed_id, item.title, item.link, item.description, item.pub_date, item.guid, now],
        ) {
            Ok(1) => items_added += 1,
            Ok(_) => {}, // Already exists
            Err(e) => eprintln!("Failed to insert feed item: {}", e),
        }
    }
    
    Ok(items_added)
}

pub fn refresh_all_feeds() -> Result<String, String> {
    let conn = get_connection()?;
    
    let mut stmt = conn
        .prepare("SELECT id, url, title FROM feeds")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    
    let feeds: Vec<(i32, String, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .map_err(|e| format!("Failed to query feeds: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to collect feeds: {}", e))?;
    
    println!("Found {} feeds to refresh", feeds.len());
    
    let mut total_items = 0;
    let mut errors = Vec::new();
    
    for (feed_id, feed_url, feed_title) in feeds {
        println!("Processing feed {}: {} ({})", feed_id, feed_title, feed_url);
        match fetch_and_store_feed(feed_id, &feed_url) {
            Ok(count) => {
                println!("Feed {} ({}): Added {} items", feed_id, feed_title, count);
                total_items += count;
            },
            Err(e) => {
                eprintln!("Feed {} ({}) error: {}", feed_id, feed_title, e);
                errors.push(format!("{}: {}", feed_title, e));
            }
        }
    }
    
    if errors.is_empty() {
        Ok(format!("Successfully refreshed feeds. Added {} new items.", total_items))
    } else {
        Ok(format!(
            "Refreshed feeds with {} errors. Added {} items.\nErrors:\n{}",
            errors.len(),
            total_items,
            errors.join("\n")
        ))
    }
}