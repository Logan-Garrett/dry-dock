// src/dal/db_context.rs
use once_cell::sync::OnceCell;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

// Global connection pool
static DB_POOL: OnceCell<Pool<SqliteConnectionManager>> = OnceCell::new();

pub fn initialize_database(db_path: &str) -> Result<(), String> {
    // Check if DB exists, if not create it
    if !does_database_exist(db_path) {
        create_database(db_path)?;
    }

    // Create connection pool
    create_connection_pool(db_path)?;

    // Run migrations to ensure DB is up to date
    run_migrations()?;

    Ok(())
}

fn create_connection_pool(db_path: &str) -> Result<(), String> {
    let manager = SqliteConnectionManager::file(db_path)
        .with_init(|conn| {
            // Confirm WAL and FK settings each time a connection is established
            // Enable foreign keys
            conn.execute_batch("PRAGMA foreign_keys = ON;")?;
            // Enable WAL mode for better concurrency
            conn.execute_batch("PRAGMA journal_mode = WAL;")?;
            Ok(())
        });
    
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .map_err(|e| format!("Failed to create connection pool: {}", e))?;
    
    DB_POOL
        .set(pool)
        .map_err(|_| "Database pool already initialized".to_string())?;

    Ok(())
}

pub fn get_connection() -> Result<r2d2::PooledConnection<SqliteConnectionManager>, String> {
    DB_POOL
        .get()
        .ok_or("Database not initialized. Call initialize_database first.".to_string())?
        .get()
        .map_err(|e| format!("Failed to get connection from pool: {}", e))
}

fn does_database_exist(db_path: &str) -> bool {
    std::path::Path::new(db_path).exists()
}

fn create_database(db_path: &str) -> Result<(), String> {
    // For SQLite, opening a connection to a non-existing file creates it
    let connection = Connection::open(db_path)
        .map_err(|e| format!("Failed to create database: {}", e))?;

    // Set FK constraints on
    connection
        .execute_batch("PRAGMA foreign_keys = ON;")
        .map_err(|e| format!("Failed to set foreign key constraints: {}", e))?;

    Ok(())
}

fn run_migrations() -> Result<(), String> {
    let conn = get_connection()?;
    
    // Tables and Indexes
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS feeds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            url TEXT UNIQUE NOT NULL,
            last_updated INTEGER,
            created_at INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            details TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER
        );
        "
    )
    .map_err(|e| format!("Failed to run migrations: {}", e))?;

    Ok(())
}