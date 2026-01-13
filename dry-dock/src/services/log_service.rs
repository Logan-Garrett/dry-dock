/// src/services/log_service.rs

use crate::dal::repositories::LogsRepository;

pub fn add_log_entry(level: &str, message: &str) {
    _ = LogsRepository::create(level, message);
}

pub fn get_all_logs() -> Vec<(i32, String, String, String)> {
    LogsRepository::get_all().unwrap_or_else(|_| vec![])
}

pub fn search_logs(query: &str) -> Vec<(i32, String, String, String)> {
    let all_logs = get_all_logs();
    all_logs
        .into_iter()
        .filter(|(_, _, message, _)| message.contains(query))
        .collect()
}