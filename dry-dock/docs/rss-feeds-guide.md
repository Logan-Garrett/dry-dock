# RSS Feeds Guide

## Table of Contents
- [What is RSS?](#what-is-rss)
- [RSS Feed Structure](#rss-feed-structure)
- [How RSS Feeds Work](#how-rss-feeds-work)
- [Subscribing to RSS Feeds](#subscribing-to-rss-feeds)
- [Implementing RSS in Rust](#implementing-rss-in-rust)
- [RSS Feed Structure Design](#rss-feed-structure-design)

---

## What is RSS?

**RSS (Really Simple Syndication)** is an XML-based format used to distribute and syndicate content from websites. It allows users to stay updated with their favorite websites without having to manually visit each one.

### Key Benefits
- **Aggregation**: Subscribe to multiple sources in one place
- **No algorithms**: Content appears chronologically
- **Privacy**: No tracking, no account required
- **Portability**: Your subscriptions are yours to keep
- **Efficiency**: Check multiple sources at once

### Common Use Cases
- News websites
- Blogs
- Podcasts
- YouTube channels
- Reddit subreddits
- Forum threads

---

## RSS Feed Structure

RSS feeds are XML documents with a specific structure. There are two main versions:
- **RSS 2.0** (most common)
- **Atom 1.0** (more modern, but less widely adopted)

### RSS 2.0 Basic Structure

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<rss version="2.0">
  <channel>
    <title>Example Feed</title>
    <link>https://example.com</link>
    <description>This is an example RSS feed</description>
    <language>en-us</language>
    <pubDate>Mon, 06 Jan 2026 12:00:00 GMT</pubDate>
    <lastBuildDate>Mon, 06 Jan 2026 12:00:00 GMT</lastBuildDate>
    
    <item>
      <title>First Article Title</title>
      <link>https://example.com/article-1</link>
      <description>Summary or full content of the article</description>
      <author>author@example.com</author>
      <pubDate>Mon, 06 Jan 2026 10:00:00 GMT</pubDate>
      <guid isPermaLink="true">https://example.com/article-1</guid>
    </item>
    
    <item>
      <title>Second Article Title</title>
      <link>https://example.com/article-2</link>
      <description>Another article summary</description>
      <author>author@example.com</author>
      <pubDate>Sun, 05 Jan 2026 15:00:00 GMT</pubDate>
      <guid isPermaLink="true">https://example.com/article-2</guid>
    </item>
  </channel>
</rss>
```

### Atom Feed Structure

```xml
<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <title>Example Feed</title>
  <link href="https://example.com"/>
  <updated>2026-01-06T12:00:00Z</updated>
  <author>
    <name>John Doe</name>
  </author>
  <id>https://example.com/feed</id>

  <entry>
    <title>First Article</title>
    <link href="https://example.com/article-1"/>
    <id>https://example.com/article-1</id>
    <updated>2026-01-06T10:00:00Z</updated>
    <summary>Summary of the article</summary>
  </entry>
</feed>
```

### Essential Elements

#### Channel/Feed Level
- `title`: Name of the feed
- `link`: URL to the website
- `description`: Brief description of the feed
- `pubDate`/`updated`: When the feed was last updated
- `language`: Language code (e.g., en-us)

#### Item/Entry Level
- `title`: Article/post title
- `link`: URL to the full content
- `description`/`summary`: Article summary or full content
- `pubDate`/`updated`: Publication date
- `guid`/`id`: Unique identifier for the item
- `author`: Content creator

---

## How RSS Feeds Work

### 1. **Feed Discovery**
Users find RSS feed URLs through:
- Feed autodiscovery links in website HTML headers
- Visible RSS icons/links on websites
- Manual URL construction (e.g., `/feed`, `/rss`, `/atom.xml`)
- RSS feed directories

### 2. **Subscription Process**
```
User → Finds Feed URL → Adds to RSS Reader → Reader Fetches Feed
```

### 3. **Feed Fetching**
RSS readers periodically:
1. Send HTTP GET request to the feed URL
2. Parse the XML response
3. Extract new items (using GUID to identify duplicates)
4. Store items in local database
5. Display to user

### 4. **Update Polling**
- Readers check feeds at intervals (5 min to several hours)
- Use HTTP headers for efficient checking:
  - `Last-Modified`: Check if feed changed since last fetch
  - `ETag`: Entity tag for cache validation
  - `If-Modified-Since`: Conditional GET request

---

## Subscribing to RSS Feeds

### Finding RSS Feeds

1. **Look for RSS icons** on websites (usually in header/footer)
2. **Check common URLs**:
   - `/feed`
   - `/rss`
   - `/atom.xml`
   - `/feed.xml`
   - `/index.xml`

3. **Browser autodiscovery**: Check HTML source for:
```html
<link rel="alternate" type="application/rss+xml" 
      title="RSS Feed" href="/feed.xml" />
```

4. **Use online tools**: RSS feed finders/validators

### Connection Requirements

#### Minimal Requirements
- **HTTP client**: To fetch the XML content
- **XML parser**: To parse RSS/Atom format
- **Storage**: To track subscriptions and read/unread status
- **Scheduler**: To periodically check for updates

#### Network Considerations
- Handle HTTP redirects (301, 302)
- Respect `Cache-Control` headers
- Implement timeout handling
- Handle connection failures gracefully
- Support HTTPS/TLS

#### Etiquette
- Set a proper `User-Agent` header
- Implement conditional GET requests
- Don't poll too frequently (respect server resources)
- Follow `robots.txt` guidelines

---

## Implementing RSS in Rust

### Option 1: Using Existing Packages (Recommended)

Rust has excellent RSS parsing libraries:

#### **`rss` crate** (Most Popular)
```toml
[dependencies]
rss = "2.0"
```

```rust
use rss::Channel;
use std::fs::File;
use std::io::BufReader;

fn parse_local_feed() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("feed.xml")?;
    let channel = Channel::read_from(BufReader::new(file))?;
    
    println!("Feed: {}", channel.title());
    
    for item in channel.items() {
        println!("Title: {}", item.title().unwrap_or("No title"));
        println!("Link: {}", item.link().unwrap_or("No link"));
        println!("---");
    }
    
    Ok(())
}
```

#### **`atom_syndication` crate** (For Atom feeds)
```toml
[dependencies]
atom_syndication = "0.12"
```

```rust
use atom_syndication::Feed;
use std::fs::File;
use std::io::BufReader;

fn parse_atom_feed() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("atom.xml")?;
    let feed = Feed::read_from(BufReader::new(file))?;
    
    println!("Feed: {}", feed.title());
    
    for entry in feed.entries() {
        println!("Title: {}", entry.title());
        println!("Link: {}", entry.links()[0].href());
        println!("---");
    }
    
    Ok(())
}
```

#### **`feed-rs` crate** (Unified Parser)
Handles both RSS and Atom:
```toml
[dependencies]
feed-rs = "1.3"
```

```rust
use feed_rs::parser;

fn parse_any_feed(xml: &str) -> Result<(), Box<dyn std::error::Error>> {
    let feed = parser::parse(xml.as_bytes())?;
    
    println!("Feed: {}", feed.title.unwrap().content);
    
    for entry in feed.entries {
        println!("Title: {}", entry.title.unwrap().content);
        if let Some(link) = entry.links.first() {
            println!("Link: {}", link.href);
        }
        println!("---");
    }
    
    Ok(())
}
```

#### **Fetching Feeds from Network**
```toml
[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
rss = "2.0"
```

```rust
use reqwest;
use rss::Channel;

fn fetch_feed(url: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let content = reqwest::blocking::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed_url = "https://example.com/feed.xml";
    let channel = fetch_feed(feed_url)?;
    
    println!("Feed: {}", channel.title());
    println!("Description: {}", channel.description());
    
    for item in channel.items() {
        println!("\n{}", item.title().unwrap_or("Untitled"));
        if let Some(desc) = item.description() {
            println!("{}", &desc[..100.min(desc.len())]);
        }
    }
    
    Ok(())
}
```

### Option 2: Manual Implementation (Not Recommended)

You *could* parse RSS manually using XML parsers like `quick-xml` or `roxmltree`, but:

**Cons:**
- RSS spec has many edge cases
- Must handle RSS 2.0, RSS 1.0, Atom separately
- Need to handle malformed feeds
- Time-consuming to implement correctly
- Reinventing the wheel

**When to consider:**
- Learning exercise
- Very specific parsing requirements
- Embedded systems with size constraints
- Need absolute control over parsing

**Basic example** (simplified, not production-ready):
```rust
use roxmltree::Document;

fn parse_rss_manually(xml: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let doc = Document::parse(xml)?;
    let mut titles = Vec::new();
    
    for node in doc.descendants() {
        if node.has_tag_name("item") {
            for child in node.children() {
                if child.has_tag_name("title") {
                    if let Some(text) = child.text() {
                        titles.push(text.to_string());
                    }
                }
            }
        }
    }
    
    Ok(titles)
}
```

### Recommendation: **Use `feed-rs` or `rss` crate**

---

## RSS Feed Structure Design

### Application Architecture

For an RSS reader application, here's the recommended structure:

```
FeedManager
├── Subscriptions (List of feed URLs)
├── FeedFetcher (HTTP client)
├── FeedParser (RSS/Atom parser)
├── ItemStorage (Database/cache)
└── UpdateScheduler (Periodic checks)
```

### Data Models

```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Represents a subscribed feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub id: String,
    pub title: String,
    pub url: String,
    pub site_url: String,
    pub description: Option<String>,
    pub last_updated: Option<DateTime<Utc>>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

/// Represents a single item/entry in a feed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedItem {
    pub id: String,           // GUID
    pub feed_id: String,      // Reference to parent feed
    pub title: String,
    pub url: String,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub published: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub is_read: bool,
    pub is_starred: bool,
}

/// Configuration for feed fetching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedConfig {
    pub update_interval_minutes: u64,
    pub request_timeout_seconds: u64,
    pub max_concurrent_fetches: usize,
    pub user_agent: String,
}
```

### Core Operations

```rust
use std::error::Error;

pub trait FeedManager {
    /// Add a new feed subscription
    fn subscribe(&mut self, url: &str) -> Result<Feed, Box<dyn Error>>;
    
    /// Remove a feed subscription
    fn unsubscribe(&mut self, feed_id: &str) -> Result<(), Box<dyn Error>>;
    
    /// Fetch updates for a specific feed
    fn update_feed(&mut self, feed_id: &str) -> Result<Vec<FeedItem>, Box<dyn Error>>;
    
    /// Fetch updates for all feeds
    fn update_all_feeds(&mut self) -> Result<(), Box<dyn Error>>;
    
    /// Get all items from a feed
    fn get_feed_items(&self, feed_id: &str) -> Result<Vec<FeedItem>, Box<dyn Error>>;
    
    /// Mark item as read/unread
    fn mark_read(&mut self, item_id: &str, is_read: bool) -> Result<(), Box<dyn Error>>;
    
    /// Star/unstar an item
    fn mark_starred(&mut self, item_id: &str, is_starred: bool) -> Result<(), Box<dyn Error>>;
}
```

### Example Implementation Structure

```rust
use reqwest::blocking::Client;
use feed_rs::parser;
use std::collections::HashMap;

pub struct SimpleFeedManager {
    feeds: HashMap<String, Feed>,
    items: HashMap<String, Vec<FeedItem>>,
    client: Client,
    config: FeedConfig,
}

impl SimpleFeedManager {
    pub fn new(config: FeedConfig) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.request_timeout_seconds))
            .user_agent(&config.user_agent)
            .build()
            .unwrap();
        
        Self {
            feeds: HashMap::new(),
            items: HashMap::new(),
            client,
            config,
        }
    }
    
    pub fn fetch_and_parse(&self, url: &str) -> Result<Vec<FeedItem>, Box<dyn Error>> {
        // Fetch feed
        let response = self.client.get(url).send()?;
        let content = response.text()?;
        
        // Parse feed
        let feed = parser::parse(content.as_bytes())?;
        
        // Convert to FeedItem
        let items: Vec<FeedItem> = feed.entries
            .into_iter()
            .map(|entry| {
                FeedItem {
                    id: entry.id,
                    feed_id: url.to_string(), // Simplified
                    title: entry.title.map(|t| t.content).unwrap_or_default(),
                    url: entry.links.first().map(|l| l.href.clone()).unwrap_or_default(),
                    content: entry.content.and_then(|c| c.body),
                    summary: entry.summary.map(|s| s.content),
                    author: entry.authors.first().map(|a| a.name.clone()),
                    published: entry.published,
                    updated: entry.updated,
                    is_read: false,
                    is_starred: false,
                }
            })
            .collect();
        
        Ok(items)
    }
}
```

### Storage Options

1. **In-Memory** (for prototyping)
   - Use `HashMap` or `Vec`
   - Fast but not persistent

2. **JSON Files** (simple persistence)
   ```rust
   use serde_json;
   
   fn save_feeds(feeds: &[Feed], path: &str) -> Result<(), Box<dyn Error>> {
       let json = serde_json::to_string_pretty(feeds)?;
       std::fs::write(path, json)?;
       Ok(())
   }
   ```

3. **SQLite** (recommended for production)
   ```toml
   [dependencies]
   rusqlite = "0.30"
   ```
   
   ```sql
   CREATE TABLE feeds (
       id TEXT PRIMARY KEY,
       title TEXT NOT NULL,
       url TEXT UNIQUE NOT NULL,
       site_url TEXT,
       description TEXT,
       last_updated INTEGER,
       etag TEXT,
       last_modified TEXT
   );
   
   CREATE TABLE items (
       id TEXT PRIMARY KEY,
       feed_id TEXT NOT NULL,
       title TEXT NOT NULL,
       url TEXT NOT NULL,
       content TEXT,
       summary TEXT,
       author TEXT,
       published INTEGER,
       updated INTEGER,
       is_read INTEGER DEFAULT 0,
       is_starred INTEGER DEFAULT 0,
       FOREIGN KEY (feed_id) REFERENCES feeds(id)
   );
   
   CREATE INDEX idx_items_feed_id ON items(feed_id);
   CREATE INDEX idx_items_is_read ON items(is_read);
   ```

### Update Scheduler

```rust
use std::thread;
use std::time::Duration;

pub fn start_feed_updater(manager: SimpleFeedManager, interval_minutes: u64) {
    thread::spawn(move || {
        loop {
            println!("Updating all feeds...");
            // Update all feeds
            // manager.update_all_feeds().ok();
            
            thread::sleep(Duration::from_secs(interval_minutes * 60));
        }
    });
}
```

---

## Complete Example: Mini RSS Reader

```rust
use feed_rs::parser;
use reqwest::blocking::Client;
use std::error::Error;

struct RssReader {
    client: Client,
}

impl RssReader {
    fn new() -> Self {
        Self {
            client: Client::builder()
                .user_agent("DryDock RSS Reader/1.0")
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap(),
        }
    }
    
    fn fetch_feed(&self, url: &str) -> Result<(), Box<dyn Error>> {
        println!("Fetching: {}", url);
        
        let response = self.client.get(url).send()?;
        let content = response.text()?;
        let feed = parser::parse(content.as_bytes())?;
        
        println!("\n=== {} ===", feed.title.unwrap().content);
        if let Some(desc) = feed.description {
            println!("{}", desc.content);
        }
        println!();
        
        for entry in feed.entries.iter().take(5) {
            println!("• {}", entry.title.as_ref().unwrap().content);
            if let Some(link) = entry.links.first() {
                println!("  {}", link.href);
            }
            if let Some(published) = entry.published {
                println!("  Published: {}", published);
            }
            println!();
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = RssReader::new();
    
    // Example feeds
    reader.fetch_feed("https://hnrss.org/frontpage")?;
    
    Ok(())
}
```

---

## Summary

### Use a Package? **YES**

**Recommended packages:**
- `feed-rs` - Universal parser (RSS + Atom)
- `rss` - RSS-specific, well-maintained
- `atom_syndication` - For Atom feeds

**Additional tools:**
- `reqwest` - HTTP client for fetching feeds
- `rusqlite` - Storage
- `serde` - Serialization
- `chrono` - Date handling

### Essential Components
1. HTTP client with proper headers
2. XML parser (via RSS library)
3. Storage for feeds and items
4. Periodic update mechanism
5. Duplicate detection (via GUID)

### Best Practices
- Respect server resources (don't poll too often)
- Use conditional GET requests (ETag, Last-Modified)
- Handle network failures gracefully
- Store items locally for offline access
- Implement proper error handling
- Set meaningful User-Agent headers

---

## Additional Resources

- [RSS 2.0 Specification](https://www.rssboard.org/rss-specification)
- [Atom Specification (RFC 4287)](https://datatracker.ietf.org/doc/html/rfc4287)
- [feed-rs crate](https://crates.io/crates/feed-rs)
- [rss crate](https://crates.io/crates/rss)
