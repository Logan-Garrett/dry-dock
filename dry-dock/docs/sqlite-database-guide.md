# SQLite Database Guide for Desktop Applications

## Table of Contents
- [Why SQLite for Desktop Apps?](#why-sqlite-for-desktop-apps)
- [SQLite vs Other Databases](#sqlite-vs-other-databases)
- [Database File Location](#database-file-location)
- [Setting Up SQLite in Rust](#setting-up-sqlite-in-rust)
- [Database Initialization & Migrations](#database-initialization--migrations)
- [Connection Management](#connection-management)
- [CRUD Operations](#crud-operations)
- [Performance Optimization](#performance-optimization)
- [Transaction Management](#transaction-management)
- [Error Handling](#error-handling)
- [Testing Strategies](#testing-strategies)
- [Database Abstraction for Future Migration](#database-abstraction-for-future-migration)
- [Complete Example Application](#complete-example-application)
- [Best Practices](#best-practices)

---

## Why SQLite for Desktop Apps?

SQLite is **ideal** for desktop applications because:

### Advantages
✅ **Zero Configuration** - No server setup, no daemon processes  
✅ **Single File** - Entire database in one file, easy backup/restore  
✅ **Cross-Platform** - Works identically on Windows, macOS, Linux  
✅ **Fast** - Very fast for local operations, no network overhead  
✅ **Reliable** - ACID compliant, battle-tested in billions of devices  
✅ **Small Footprint** - Minimal memory usage, perfect for desktop apps  
✅ **No Dependencies** - Embedded database, ships with your app  
✅ **Concurrent Reads** - Multiple readers simultaneously  
✅ **Self-Contained** - No installation required for users  

### When SQLite is Perfect
- Desktop applications (like yours!)
- Mobile apps
- Embedded systems
- Local caching
- Development/testing
- Small to medium data (< 1TB)
- Read-heavy workloads
- Single-user applications

### When to Consider Alternatives
❌ High concurrent writes (100+ writers simultaneously)  
❌ Network/client-server architecture  
❌ Very large datasets (> 1TB)  
❌ Complex distributed systems  
❌ Need for fine-grained user permissions  

---

## SQLite vs Other Databases

| Feature | SQLite | PostgreSQL | MySQL |
|---------|---------|------------|-------|
| **Setup** | None | Server required | Server required |
| **File** | Single file | Data directory | Multiple files |
| **Network** | No | Yes | Yes |
| **Concurrent Writes** | Limited | Excellent | Good |
| **Data Size** | Up to ~281 TB | Unlimited | Very large |
| **Use Case** | Local/embedded | Multi-user server | Web applications |
| **Deployment** | Ship with app | Separate install | Separate install |
| **Transactions** | ACID | ACID | ACID |
| **Performance** | Excellent (local) | Good (network) | Good (network) |

**For Desktop Apps:** SQLite wins hands down due to simplicity and performance.

---

## Database File Location

Place your SQLite database in the appropriate user directory for your OS:

### Platform-Specific Locations

```rust
use std::path::PathBuf;

/// Get the appropriate database path for the current platform
pub fn get_database_path(app_name: &str) -> PathBuf {
    let mut path = get_data_directory(app_name);
    path.push("database.db");
    path
}

/// Get platform-specific data directory
pub fn get_data_directory(app_name: &str) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        // C:\Users\<User>\AppData\Roaming\<AppName>
        let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(app_name);
        path
    }
    
    #[cfg(target_os = "macos")]
    {
        // ~/Library/Application Support/<AppName>
        let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(app_name);
        path
    }
    
    #[cfg(target_os = "linux")]
    {
        // ~/.local/share/<AppName>
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(app_name);
        path
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from(app_name)
    }
}
```

### Directory Examples

- **macOS**: `~/Library/Application Support/DryDock/database.db`
- **Linux**: `~/.local/share/DryDock/database.db`
- **Windows**: `C:\Users\<User>\AppData\Roaming\DryDock\database.db`

### Create Directory if Needed

```rust
use std::fs;

pub fn ensure_data_directory(app_name: &str) -> std::io::Result<PathBuf> {
    let path = get_data_directory(app_name);
    fs::create_dir_all(&path)?;
    Ok(path)
}
```

---

## Setting Up SQLite in Rust

### Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
# SQLite with bundled library (no system dependency required)
rusqlite = { version = "0.31", features = ["bundled"] }

# Optional but recommended: connection pooling
r2d2 = "0.8"
r2d2_sqlite = "0.24"

# Directory paths
dirs = "5.0"

# Migrations (optional)
rusqlite_migration = "1.2"

# Serialization for structs
serde = { version = "1.0", features = ["derive"] }
```

**Important:** Use the `"bundled"` feature to include SQLite with your app (no user installation needed).

---

## Database Initialization & Migrations

### Basic Initialization

```rust
use rusqlite::{Connection, Result};
use std::path::Path;

pub fn initialize_database(db_path: &Path) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    
    // Enable foreign keys (disabled by default in SQLite)
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    
    // Create tables
    create_tables(&conn)?;
    
    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS feeds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            url TEXT UNIQUE NOT NULL,
            description TEXT,
            last_updated INTEGER,
            created_at INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            feed_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            content TEXT,
            published INTEGER,
            is_read INTEGER DEFAULT 0,
            is_starred INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
        );
        
        CREATE INDEX IF NOT EXISTS idx_items_feed_id ON items(feed_id);
        CREATE INDEX IF NOT EXISTS idx_items_is_read ON items(is_read);
        CREATE INDEX IF NOT EXISTS idx_items_published ON items(published);
        "
    )?;
    
    Ok(())
}
```

### Schema Migrations (Recommended)

Use migrations for evolving your schema over time:

```rust
use rusqlite::{Connection, Result};
use rusqlite_migration::{Migrations, M};

pub fn run_migrations(conn: &mut Connection) -> Result<()> {
    // Define migrations
    let migrations = Migrations::new(vec![
        // Migration 1: Initial schema
        M::up(
            "
            CREATE TABLE feeds (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT UNIQUE NOT NULL,
                created_at INTEGER NOT NULL
            );
            
            CREATE TABLE items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                feed_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                is_read INTEGER DEFAULT 0,
                FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
            );
            "
        ),
        
        // Migration 2: Add description to feeds
        M::up(
            "
            ALTER TABLE feeds ADD COLUMN description TEXT;
            ALTER TABLE feeds ADD COLUMN last_updated INTEGER;
            "
        ),
        
        // Migration 3: Add starred feature to items
        M::up(
            "
            ALTER TABLE items ADD COLUMN is_starred INTEGER DEFAULT 0;
            CREATE INDEX idx_items_feed_id ON items(feed_id);
            "
        ),
        
        // Migration 4: Add content to items
        M::up(
            "
            ALTER TABLE items ADD COLUMN content TEXT;
            ALTER TABLE items ADD COLUMN published INTEGER;
            ALTER TABLE items ADD COLUMN created_at INTEGER NOT NULL DEFAULT 0;
            CREATE INDEX idx_items_is_read ON items(is_read);
            CREATE INDEX idx_items_published ON items(published);
            "
        ),
    ]);
    
    // Run migrations
    migrations.to_latest(conn)?;
    
    Ok(())
}

// Initialize database with migrations
pub fn initialize_with_migrations(db_path: &Path) -> Result<Connection> {
    let mut conn = Connection::open(db_path)?;
    
    // Enable foreign keys
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    
    // Run migrations
    run_migrations(&mut conn)?;
    
    Ok(conn)
}
```

### Version Tracking

```rust
pub fn get_schema_version(conn: &Connection) -> Result<i32> {
    let version: i32 = conn.query_row(
        "SELECT MAX(version) FROM __migrations",
        [],
        |row| row.get(0),
    )?;
    Ok(version)
}
```

---

## Connection Management

### Single-Threaded Application (Simple)

```rust
use rusqlite::Connection;
use std::path::PathBuf;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self, rusqlite::Error> {
        let conn = initialize_with_migrations(&path)?;
        Ok(Self { conn })
    }
    
    pub fn connection(&self) -> &Connection {
        &self.conn
    }
}
```

### Multi-Threaded Application (Connection Pool)

For apps with background tasks or async operations:

```rust
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use std::path::Path;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn create_pool(db_path: &Path) -> Result<DbPool, r2d2::Error> {
    let manager = SqliteConnectionManager::file(db_path)
        .with_init(|conn| {
            // Initialize each connection in the pool
            conn.execute_batch("PRAGMA foreign_keys = ON;")?;
            conn.execute_batch("PRAGMA journal_mode = WAL;")?;  // Write-Ahead Logging
            Ok(())
        });
    
    let pool = Pool::builder()
        .max_size(5)  // Maximum 5 connections
        .build(manager)?;
    
    // Run migrations on one connection
    let conn = pool.get().unwrap();
    run_migrations(&mut *conn)?;
    
    Ok(pool)
}

// Usage
pub struct Database {
    pool: DbPool,
}

impl Database {
    pub fn new(db_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = create_pool(db_path)?;
        Ok(Self { pool })
    }
    
    pub fn get_connection(&self) -> Result<r2d2::PooledConnection<SqliteConnectionManager>, r2d2::Error> {
        self.pool.get()
    }
}
```

### In Your App Struct

```rust
use eframe::egui;

struct MyApp {
    db: Database,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let db_path = get_database_path("DryDock");
        ensure_data_directory("DryDock").expect("Failed to create data directory");
        
        let db = Database::new(db_path)
            .expect("Failed to initialize database");
        
        Self { db }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Access database
        let conn = self.db.get_connection().unwrap();
        // Use conn for queries
    }
}
```

---

## CRUD Operations

### Create (Insert)

```rust
use rusqlite::{Connection, Result, params};

pub fn insert_feed(conn: &Connection, title: &str, url: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO feeds (title, url, created_at) VALUES (?1, ?2, ?3)",
        params![title, url, chrono::Utc::now().timestamp()],
    )?;
    
    // Return the ID of the inserted row
    Ok(conn.last_insert_rowid())
}

// Using structs
#[derive(Debug)]
pub struct Feed {
    pub id: Option<i64>,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub created_at: i64,
}

impl Feed {
    pub fn insert(&self, conn: &Connection) -> Result<i64> {
        conn.execute(
            "INSERT INTO feeds (title, url, description, created_at) 
             VALUES (?1, ?2, ?3, ?4)",
            params![
                self.title,
                self.url,
                self.description,
                chrono::Utc::now().timestamp()
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }
}
```

### Read (Select)

```rust
use rusqlite::{Connection, Result};

// Get single record
pub fn get_feed(conn: &Connection, id: i64) -> Result<Feed> {
    conn.query_row(
        "SELECT id, title, url, description, created_at FROM feeds WHERE id = ?1",
        params![id],
        |row| {
            Ok(Feed {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                url: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
            })
        },
    )
}

// Get multiple records
pub fn get_all_feeds(conn: &Connection) -> Result<Vec<Feed>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, url, description, created_at FROM feeds ORDER BY created_at DESC"
    )?;
    
    let feeds = stmt.query_map([], |row| {
        Ok(Feed {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            url: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;
    
    // Collect into Vec, handling errors
    feeds.collect()
}

// With filtering
pub fn search_feeds(conn: &Connection, query: &str) -> Result<Vec<Feed>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, url, description, created_at 
         FROM feeds 
         WHERE title LIKE ?1 OR description LIKE ?1
         ORDER BY created_at DESC"
    )?;
    
    let search_term = format!("%{}%", query);
    let feeds = stmt.query_map([&search_term], |row| {
        Ok(Feed {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            url: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
        })
    })?;
    
    feeds.collect()
}
```

### Update

```rust
pub fn update_feed(conn: &Connection, id: i64, title: &str, description: &str) -> Result<usize> {
    conn.execute(
        "UPDATE feeds SET title = ?1, description = ?2 WHERE id = ?3",
        params![title, description, id],
    )
}

impl Feed {
    pub fn update(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "UPDATE feeds 
             SET title = ?1, url = ?2, description = ?3 
             WHERE id = ?4",
            params![self.title, self.url, self.description, self.id],
        )
    }
}
```

### Delete

```rust
pub fn delete_feed(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute("DELETE FROM feeds WHERE id = ?1", params![id])
}

impl Feed {
    pub fn delete(&self, conn: &Connection) -> Result<usize> {
        if let Some(id) = self.id {
            conn.execute("DELETE FROM feeds WHERE id = ?1", params![id])
        } else {
            Ok(0)
        }
    }
}
```

---

## Performance Optimization

### 1. Use Write-Ahead Logging (WAL)

WAL mode significantly improves concurrency:

```rust
pub fn optimize_connection(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;       -- Write-Ahead Logging
        PRAGMA synchronous = NORMAL;     -- Faster writes, still safe
        PRAGMA cache_size = -64000;      -- 64MB cache
        PRAGMA temp_store = MEMORY;      -- Temp tables in memory
        PRAGMA mmap_size = 268435456;    -- 256MB memory-mapped I/O
        "
    )?;
    Ok(())
}
```

### 2. Create Indexes

```sql
-- Index frequently queried columns
CREATE INDEX idx_items_feed_id ON items(feed_id);
CREATE INDEX idx_items_is_read ON items(is_read);
CREATE INDEX idx_items_published ON items(published DESC);

-- Composite index for common queries
CREATE INDEX idx_items_feed_read ON items(feed_id, is_read);

-- Full-text search (if needed)
CREATE VIRTUAL TABLE items_fts USING fts5(title, content);
```

### 3. Use Transactions for Bulk Operations

```rust
pub fn insert_many_items(conn: &Connection, items: &[Item]) -> Result<()> {
    let tx = conn.transaction()?;
    
    {
        let mut stmt = tx.prepare(
            "INSERT INTO items (feed_id, title, url, content, published, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        )?;
        
        for item in items {
            stmt.execute(params![
                item.feed_id,
                item.title,
                item.url,
                item.content,
                item.published,
                chrono::Utc::now().timestamp()
            ])?;
        }
    }
    
    tx.commit()?;
    Ok(())
}
```

### 4. Prepare Statements for Repeated Queries

```rust
pub struct Database {
    pool: DbPool,
}

impl Database {
    // Cache prepared statements if using same connection
    pub fn bulk_mark_read(&self, item_ids: &[i64]) -> Result<()> {
        let conn = self.pool.get().unwrap();
        let tx = conn.transaction()?;
        
        {
            let mut stmt = tx.prepare("UPDATE items SET is_read = 1 WHERE id = ?1")?;
            
            for &id in item_ids {
                stmt.execute(params![id])?;
            }
        }
        
        tx.commit()?;
        Ok(())
    }
}
```

### 5. Limit Query Results

```rust
pub fn get_recent_items(conn: &Connection, limit: i32) -> Result<Vec<Item>> {
    let mut stmt = conn.prepare(
        "SELECT * FROM items 
         ORDER BY published DESC 
         LIMIT ?1"
    )?;
    
    let items = stmt.query_map([limit], |row| {
        // Map row to Item
        Ok(Item {
            id: row.get(0)?,
            feed_id: row.get(1)?,
            title: row.get(2)?,
            // ...
        })
    })?;
    
    items.collect()
}
```

### 6. Use Connection Pooling for Concurrent Access

Already covered in [Connection Management](#connection-management).

### 7. ANALYZE for Query Optimization

```rust
pub fn analyze_database(conn: &Connection) -> Result<()> {
    // Update statistics for query planner
    conn.execute_batch("ANALYZE;")?;
    Ok(())
}
```

---

## Transaction Management

### Basic Transaction

```rust
use rusqlite::{Connection, Transaction, Result};

pub fn transfer_item(conn: &Connection, item_id: i64, new_feed_id: i64) -> Result<()> {
    let tx = conn.transaction()?;
    
    tx.execute(
        "UPDATE items SET feed_id = ?1 WHERE id = ?2",
        params![new_feed_id, item_id],
    )?;
    
    tx.execute(
        "UPDATE feeds SET last_updated = ?1 WHERE id = ?2",
        params![chrono::Utc::now().timestamp(), new_feed_id],
    )?;
    
    tx.commit()?;
    Ok(())
}
```

### Transaction with Rollback

```rust
pub fn complex_operation(conn: &Connection) -> Result<()> {
    let tx = conn.transaction()?;
    
    match perform_operations(&tx) {
        Ok(_) => {
            tx.commit()?;
            Ok(())
        }
        Err(e) => {
            // Automatic rollback on drop, but explicit is clearer
            drop(tx);
            Err(e)
        }
    }
}

fn perform_operations(tx: &Transaction) -> Result<()> {
    // Multiple operations
    tx.execute("INSERT INTO ...", params![])?;
    tx.execute("UPDATE ...", params![])?;
    tx.execute("DELETE FROM ...", params![])?;
    Ok(())
}
```

### Savepoints

```rust
pub fn nested_transaction(conn: &Connection) -> Result<()> {
    let mut tx = conn.transaction()?;
    
    // Outer transaction work
    tx.execute("INSERT INTO feeds ...", params![])?;
    
    // Create savepoint
    {
        let sp = tx.savepoint()?;
        
        // Try something that might fail
        if let Err(_) = sp.execute("INSERT INTO items ...", params![]) {
            // Rollback to savepoint (not entire transaction)
            drop(sp);
        } else {
            sp.commit()?;
        }
    }
    
    // Continue with outer transaction
    tx.execute("UPDATE feeds ...", params![])?;
    
    tx.commit()?;
    Ok(())
}
```

---

## Error Handling

### Custom Error Type

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    Connection(#[from] rusqlite::Error),
    
    #[error("Pool error: {0}")]
    Pool(#[from] r2d2::Error),
    
    #[error("Item not found: {0}")]
    NotFound(i64),
    
    #[error("Duplicate entry: {0}")]
    Duplicate(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

pub type DbResult<T> = Result<T, DatabaseError>;
```

### Usage

```rust
pub fn get_feed_or_error(conn: &Connection, id: i64) -> DbResult<Feed> {
    get_feed(conn, id).map_err(|_| DatabaseError::NotFound(id))
}

pub fn insert_feed_checked(conn: &Connection, feed: &Feed) -> DbResult<i64> {
    // Validate
    if feed.url.is_empty() {
        return Err(DatabaseError::Validation("URL cannot be empty".to_string()));
    }
    
    // Try insert
    match feed.insert(conn) {
        Ok(id) => Ok(id),
        Err(rusqlite::Error::SqliteFailure(err, _)) => {
            if err.code == rusqlite::ErrorCode::ConstraintViolation {
                Err(DatabaseError::Duplicate(feed.url.clone()))
            } else {
                Err(DatabaseError::Connection(rusqlite::Error::SqliteFailure(err, None)))
            }
        }
        Err(e) => Err(DatabaseError::Connection(e)),
    }
}
```

---

## Testing Strategies

### In-Memory Database for Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    
    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        create_tables(&conn).unwrap();
        conn
    }
    
    #[test]
    fn test_insert_feed() {
        let conn = setup_test_db();
        
        let feed = Feed {
            id: None,
            title: "Test Feed".to_string(),
            url: "https://example.com/feed".to_string(),
            description: Some("Test".to_string()),
            created_at: 0,
        };
        
        let id = feed.insert(&conn).unwrap();
        assert!(id > 0);
        
        let retrieved = get_feed(&conn, id).unwrap();
        assert_eq!(retrieved.title, "Test Feed");
    }
    
    #[test]
    fn test_delete_feed() {
        let conn = setup_test_db();
        
        let feed = Feed {
            id: None,
            title: "Test".to_string(),
            url: "https://test.com".to_string(),
            description: None,
            created_at: 0,
        };
        
        let id = feed.insert(&conn).unwrap();
        
        delete_feed(&conn, id).unwrap();
        
        assert!(get_feed(&conn, id).is_err());
    }
}
```

### Temporary File Database for Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_database_persistence() {
        let temp_file = NamedTempFile::new().unwrap();
        let db_path = temp_file.path();
        
        // Create and populate database
        {
            let conn = initialize_with_migrations(db_path).unwrap();
            let feed = Feed {
                id: None,
                title: "Persistent Feed".to_string(),
                url: "https://example.com".to_string(),
                description: None,
                created_at: 0,
            };
            feed.insert(&conn).unwrap();
        }
        
        // Reopen and verify
        {
            let conn = Connection::open(db_path).unwrap();
            let feeds = get_all_feeds(&conn).unwrap();
            assert_eq!(feeds.len(), 1);
            assert_eq!(feeds[0].title, "Persistent Feed");
        }
    }
}
```

---

## Database Abstraction for Future Migration

To easily switch from SQLite to PostgreSQL later, use a trait-based abstraction:

### Define Trait

```rust
use async_trait::async_trait;

#[async_trait]
pub trait FeedRepository: Send + Sync {
    async fn insert(&self, feed: &Feed) -> DbResult<i64>;
    async fn get(&self, id: i64) -> DbResult<Feed>;
    async fn get_all(&self) -> DbResult<Vec<Feed>>;
    async fn update(&self, feed: &Feed) -> DbResult<()>;
    async fn delete(&self, id: i64) -> DbResult<()>;
    async fn search(&self, query: &str) -> DbResult<Vec<Feed>>;
}

#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn insert(&self, item: &Item) -> DbResult<i64>;
    async fn get_by_feed(&self, feed_id: i64) -> DbResult<Vec<Item>>;
    async fn mark_read(&self, id: i64, is_read: bool) -> DbResult<()>;
    async fn delete(&self, id: i64) -> DbResult<()>;
}
```

### SQLite Implementation

```rust
pub struct SqliteFeedRepository {
    pool: DbPool,
}

#[async_trait]
impl FeedRepository for SqliteFeedRepository {
    async fn insert(&self, feed: &Feed) -> DbResult<i64> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get()?;
            feed.insert(&conn).map_err(DatabaseError::from)
        }).await.unwrap()
    }
    
    async fn get(&self, id: i64) -> DbResult<Feed> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get()?;
            get_feed(&conn, id).map_err(DatabaseError::from)
        }).await.unwrap()
    }
    
    async fn get_all(&self) -> DbResult<Vec<Feed>> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let conn = pool.get()?;
            get_all_feeds(&conn).map_err(DatabaseError::from)
        }).await.unwrap()
    }
    
    // Implement other methods...
}
```

### PostgreSQL Implementation (Future)

```rust
pub struct PostgresFeedRepository {
    pool: sqlx::PgPool,
}

#[async_trait]
impl FeedRepository for PostgresFeedRepository {
    async fn insert(&self, feed: &Feed) -> DbResult<i64> {
        let id = sqlx::query!(
            "INSERT INTO feeds (title, url, description, created_at) 
             VALUES ($1, $2, $3, $4) 
             RETURNING id",
            feed.title,
            feed.url,
            feed.description,
            chrono::Utc::now()
        )
        .fetch_one(&self.pool)
        .await?
        .id;
        
        Ok(id)
    }
    
    async fn get(&self, id: i64) -> DbResult<Feed> {
        let feed = sqlx::query_as!(
            Feed,
            "SELECT id, title, url, description, created_at FROM feeds WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(feed)
    }
    
    // Implement other methods...
}
```

### Usage in App

```rust
pub struct Database {
    feed_repo: Box<dyn FeedRepository>,
    item_repo: Box<dyn ItemRepository>,
}

impl Database {
    pub fn new_sqlite(path: &Path) -> DbResult<Self> {
        let pool = create_pool(path)?;
        Ok(Self {
            feed_repo: Box::new(SqliteFeedRepository { pool: pool.clone() }),
            item_repo: Box::new(SqliteItemRepository { pool }),
        })
    }
    
    // Future: Switch to PostgreSQL
    pub fn new_postgres(connection_string: &str) -> DbResult<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await?;
        
        Ok(Self {
            feed_repo: Box::new(PostgresFeedRepository { pool: pool.clone() }),
            item_repo: Box::new(PostgresItemRepository { pool }),
        })
    }
    
    pub fn feeds(&self) -> &dyn FeedRepository {
        &*self.feed_repo
    }
    
    pub fn items(&self) -> &dyn ItemRepository {
        &*self.item_repo
    }
}

// Usage
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let db_path = get_database_path("DryDock");
        let db = Database::new_sqlite(&db_path).unwrap();
        
        Self { db }
    }
}
```

---

## Complete Example Application

Here's a complete RSS feed manager with SQLite:

```rust
use rusqlite::{Connection, Result, params};
use std::path::PathBuf;
use chrono::Utc;

// ===== Models =====

#[derive(Debug, Clone)]
pub struct Feed {
    pub id: Option<i64>,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub last_updated: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub id: Option<i64>,
    pub feed_id: i64,
    pub title: String,
    pub url: String,
    pub content: Option<String>,
    pub published: Option<i64>,
    pub is_read: bool,
    pub is_starred: bool,
    pub created_at: i64,
}

// ===== Database Setup =====

pub fn get_database_path(app_name: &str) -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push(app_name);
    std::fs::create_dir_all(&path).ok();
    path.push("database.db");
    path
}

pub fn initialize_database(path: &PathBuf) -> Result<Connection> {
    let conn = Connection::open(path)?;
    
    conn.execute_batch(
        "
        PRAGMA foreign_keys = ON;
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        
        CREATE TABLE IF NOT EXISTS feeds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            url TEXT UNIQUE NOT NULL,
            description TEXT,
            last_updated INTEGER,
            created_at INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            feed_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            url TEXT NOT NULL,
            content TEXT,
            published INTEGER,
            is_read INTEGER DEFAULT 0,
            is_starred INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (feed_id) REFERENCES feeds(id) ON DELETE CASCADE
        );
        
        CREATE INDEX IF NOT EXISTS idx_items_feed_id ON items(feed_id);
        CREATE INDEX IF NOT EXISTS idx_items_is_read ON items(is_read);
        CREATE INDEX IF NOT EXISTS idx_items_published ON items(published DESC);
        "
    )?;
    
    Ok(conn)
}

// ===== Feed Operations =====

pub fn insert_feed(conn: &Connection, feed: &Feed) -> Result<i64> {
    conn.execute(
        "INSERT INTO feeds (title, url, description, created_at) VALUES (?1, ?2, ?3, ?4)",
        params![feed.title, feed.url, feed.description, Utc::now().timestamp()],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_all_feeds(conn: &Connection) -> Result<Vec<Feed>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, url, description, last_updated, created_at 
         FROM feeds 
         ORDER BY created_at DESC"
    )?;
    
    let feeds = stmt.query_map([], |row| {
        Ok(Feed {
            id: Some(row.get(0)?),
            title: row.get(1)?,
            url: row.get(2)?,
            description: row.get(3)?,
            last_updated: row.get(4)?,
            created_at: row.get(5)?,
        })
    })?;
    
    feeds.collect()
}

pub fn delete_feed(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM feeds WHERE id = ?1", params![id])?;
    Ok(())
}

// ===== Item Operations =====

pub fn insert_item(conn: &Connection, item: &FeedItem) -> Result<i64> {
    conn.execute(
        "INSERT INTO items (feed_id, title, url, content, published, is_read, is_starred, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            item.feed_id,
            item.title,
            item.url,
            item.content,
            item.published,
            item.is_read as i32,
            item.is_starred as i32,
            Utc::now().timestamp()
        ],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_items_by_feed(conn: &Connection, feed_id: i64) -> Result<Vec<FeedItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, feed_id, title, url, content, published, is_read, is_starred, created_at
         FROM items
         WHERE feed_id = ?1
         ORDER BY published DESC"
    )?;
    
    let items = stmt.query_map([feed_id], |row| {
        Ok(FeedItem {
            id: Some(row.get(0)?),
            feed_id: row.get(1)?,
            title: row.get(2)?,
            url: row.get(3)?,
            content: row.get(4)?,
            published: row.get(5)?,
            is_read: row.get::<_, i32>(6)? != 0,
            is_starred: row.get::<_, i32>(7)? != 0,
            created_at: row.get(8)?,
        })
    })?;
    
    items.collect()
}

pub fn mark_item_read(conn: &Connection, id: i64, is_read: bool) -> Result<()> {
    conn.execute(
        "UPDATE items SET is_read = ?1 WHERE id = ?2",
        params![is_read as i32, id],
    )?;
    Ok(())
}

pub fn mark_item_starred(conn: &Connection, id: i64, is_starred: bool) -> Result<()> {
    conn.execute(
        "UPDATE items SET is_starred = ?1 WHERE id = ?2",
        params![is_starred as i32, id],
    )?;
    Ok(())
}

// ===== Database Manager =====

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(app_name: &str) -> Result<Self> {
        let path = get_database_path(app_name);
        let conn = initialize_database(&path)?;
        Ok(Self { conn })
    }
    
    pub fn add_feed(&self, title: &str, url: &str, description: Option<&str>) -> Result<i64> {
        let feed = Feed {
            id: None,
            title: title.to_string(),
            url: url.to_string(),
            description: description.map(|s| s.to_string()),
            last_updated: None,
            created_at: 0,
        };
        insert_feed(&self.conn, &feed)
    }
    
    pub fn get_feeds(&self) -> Result<Vec<Feed>> {
        get_all_feeds(&self.conn)
    }
    
    pub fn delete_feed(&self, id: i64) -> Result<()> {
        delete_feed(&self.conn, id)
    }
    
    pub fn add_item(&self, feed_id: i64, title: &str, url: &str, content: Option<&str>) -> Result<i64> {
        let item = FeedItem {
            id: None,
            feed_id,
            title: title.to_string(),
            url: url.to_string(),
            content: content.map(|s| s.to_string()),
            published: Some(Utc::now().timestamp()),
            is_read: false,
            is_starred: false,
            created_at: 0,
        };
        insert_item(&self.conn, &item)
    }
    
    pub fn get_feed_items(&self, feed_id: i64) -> Result<Vec<FeedItem>> {
        get_items_by_feed(&self.conn, feed_id)
    }
    
    pub fn mark_read(&self, item_id: i64, is_read: bool) -> Result<()> {
        mark_item_read(&self.conn, item_id, is_read)
    }
    
    pub fn mark_starred(&self, item_id: i64, is_starred: bool) -> Result<()> {
        mark_item_starred(&self.conn, item_id, is_starred)
    }
}

// ===== Usage in eframe App =====

struct MyApp {
    db: Database,
    feeds: Vec<Feed>,
}

impl MyApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let db = Database::new("DryDock").expect("Failed to initialize database");
        let feeds = db.get_feeds().unwrap_or_default();
        
        Self { db, feeds }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("RSS Feeds");
            
            for feed in &self.feeds {
                ui.horizontal(|ui| {
                    ui.label(&feed.title);
                    if ui.button("Delete").clicked() {
                        if let Some(id) = feed.id {
                            self.db.delete_feed(id).ok();
                            self.feeds = self.db.get_feeds().unwrap_or_default();
                        }
                    }
                });
            }
        });
    }
}
```

---

## Best Practices

### ✅ DO

1. **Use bundled SQLite** - Include SQLite with your app (no user installation)
2. **Enable foreign keys** - `PRAGMA foreign_keys = ON;`
3. **Use WAL mode** - Better concurrency: `PRAGMA journal_mode = WAL;`
4. **Create indexes** - On frequently queried columns
5. **Use transactions** - For bulk operations
6. **Use prepared statements** - For repeated queries
7. **Handle errors** - Proper error types and handling
8. **Use migrations** - Version your schema changes
9. **Store in user data directory** - Platform-appropriate location
10. **Use connection pooling** - For multi-threaded apps
11. **Abstract database layer** - Trait-based for future flexibility
12. **Write tests** - In-memory databases for unit tests

### ❌ DON'T

1. **Don't store in app directory** - Use user data directory
2. **Don't hardcode paths** - Use platform-specific directories
3. **Don't skip migrations** - Always version schema changes
4. **Don't ignore errors** - Handle database errors properly
5. **Don't use string concatenation** - Use parameterized queries (SQL injection!)
6. **Don't open multiple connections** - Use a pool instead
7. **Don't skip transactions** - For related operations
8. **Don't over-normalize** - SQLite prefers slightly denormalized
9. **Don't use SQLite for** - Network applications or high concurrent writes
10. **Don't forget to backup** - Provide export functionality

---

## Migration Path: SQLite → PostgreSQL

When your app grows and needs PostgreSQL:

### 1. Use Abstraction Layer (Already shown above)

### 2. Export/Import Data

```rust
// Export from SQLite
pub fn export_to_json(conn: &Connection) -> Result<String> {
    let feeds = get_all_feeds(conn)?;
    let json = serde_json::to_string_pretty(&feeds)?;
    Ok(json)
}

// Import to PostgreSQL
pub async fn import_from_json(pool: &sqlx::PgPool, json: &str) -> Result<()> {
    let feeds: Vec<Feed> = serde_json::from_str(json)?;
    
    for feed in feeds {
        sqlx::query!(
            "INSERT INTO feeds (title, url, description, created_at) VALUES ($1, $2, $3, $4)",
            feed.title,
            feed.url,
            feed.description,
            feed.created_at
        )
        .execute(pool)
        .await?;
    }
    
    Ok(())
}
```

### 3. Keep SQL Portable

Write SQL that works on both databases:
- Avoid SQLite-specific functions
- Use standard SQL types
- Use `RETURNING` clause carefully (not in older MySQL)

### 4. Configuration-Based Database Selection

```rust
pub enum DatabaseType {
    SQLite(PathBuf),
    Postgres(String),
}

impl Database {
    pub fn new(config: DatabaseType) -> DbResult<Self> {
        match config {
            DatabaseType::SQLite(path) => Self::new_sqlite(&path),
            DatabaseType::Postgres(conn_str) => Self::new_postgres(&conn_str),
        }
    }
}
```

---

## Summary

### Key Takeaways

1. **SQLite is perfect for desktop apps** - Zero config, single file, cross-platform
2. **Store in user data directory** - Use `dirs` crate for platform-specific paths
3. **Use migrations** - Version your schema with `rusqlite_migration`
4. **Enable WAL mode** - Better performance and concurrency
5. **Use connection pooling** - For multi-threaded applications
6. **Create indexes** - On frequently queried columns
7. **Use transactions** - For bulk operations
8. **Abstract the database layer** - Traits for easy migration later
9. **Test with in-memory DB** - Fast unit tests
10. **Handle errors properly** - Custom error types

### Recommended Stack

```toml
[dependencies]
rusqlite = { version = "0.31", features = ["bundled"] }
r2d2 = "0.8"
r2d2_sqlite = "0.24"
rusqlite_migration = "1.2"
dirs = "5.0"
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
```

### Next Steps

1. Implement basic database setup
2. Create migration system
3. Build repository layer
4. Add error handling
5. Write unit tests
6. Optimize with indexes and WAL
7. Consider abstraction for future PostgreSQL migration

---

## Additional Resources

- [SQLite Documentation](https://www.sqlite.org/docs.html)
- [rusqlite Documentation](https://docs.rs/rusqlite/latest/rusqlite/)
- [SQLite Performance Tuning](https://www.sqlite.org/optoverview.html)
- [SQLite When To Use](https://www.sqlite.org/whentouse.html)
- [Database Abstraction Patterns](https://rust-unofficial.github.io/patterns/patterns/behavioural/strategy.html)

Happy coding with SQLite! 🗄️
