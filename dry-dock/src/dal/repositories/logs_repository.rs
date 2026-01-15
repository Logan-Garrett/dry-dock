// // src/dal/repositories/logs_repository.rs
use crate::dal::db_context::get_connection;
use rusqlite::params;

/// Logs Repository - handles all log-related database operations
pub struct LogsRepository;

impl LogsRepository {
    /// Create a new log entry
    pub fn create(level: &str, message: &str) -> Result<(), String> {
        let conn = get_connection()?;
        let now = chrono::Utc::now().timestamp();

        // convert LogLevel enum to string
        let level_str = match level {
            "Info" => "INFO",
            "Warning" => "WARNING",
            "Error" => "ERROR",
            _ => "INFO",
        };

        conn.execute(
            "INSERT INTO logs (level, message, timestamp) VALUES (?1, ?2, ?3)",
            params![level_str, message, now],
        )
        .map_err(|e| format!("Failed to create log entry: {}", e))?;

        Ok(())
    }

    /// Get all logs (limited to most recent 1000)
    pub fn get_all() -> Result<Vec<(i32, String, String, String)>, String> {
        let conn = get_connection()?;

        let mut stmt = conn
            .prepare("SELECT id, level, message, timestamp FROM logs ORDER BY timestamp DESC LIMIT 1000")
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let logs = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>("id")?,
                    row.get::<_, String>("level")?,
                    row.get::<_, String>("message")?,
                    row.get::<_, i64>("timestamp")?,
                ))
            })
            .map_err(|e| format!("Failed to query logs: {}", e))?
            .collect::<Result<Vec<(i32, String, String, i64)>, _>>()
            .map_err(|e| format!("Failed to collect logs: {}", e))?;

        // Convert timestamps to formatted strings
        let formatted_logs = logs
            .into_iter()
            .map(|(id, level, message, timestamp)| {
                let timestamp_str = chrono::DateTime::from_timestamp(timestamp, 0)
                    .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "Unknown".to_string());
                (id, level, message, timestamp_str)
            })
            .collect();

        Ok(formatted_logs)
    }
}