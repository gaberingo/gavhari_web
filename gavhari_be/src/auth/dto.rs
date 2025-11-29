use actix_web::cookie::time::UtcDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct SessionData {
    pub session_id: String,
    pub created_at: i64,
    pub last_accesses: i64,
    pub visit_count: u32,
    pub is_guest: bool,
}

impl SessionData {
    pub fn new() -> Self {
        let now = UtcDateTime::now().unix_timestamp();
        SessionData {
            session_id: Uuid::new_v4().to_string(),
            created_at: now,
            last_accesses: now,
            visit_count: 0,
            is_guest: true,
        }
    }

    pub fn update_last_access(&mut self, timestamp: i64) {
        self.last_accesses = timestamp;
    }

    pub fn with_updated_access(mut self, timestamp: i64) -> Self {
        self.last_accesses = timestamp;
        self
    }

    pub fn authenticate(&mut self) {
        self.is_guest = false;
    }
}
