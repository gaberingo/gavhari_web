use actix_web::cookie::time::UtcDateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserSessionData {
    id: i32,
    role: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserSession {
    pub created_at: i64,
    pub last_accesses: i64,
    pub visit_count: u32,
    pub anonim_user: bool,
    pub data: Option<UserSessionData>,
}

impl UserSession {
    pub fn new() -> Self {
        let now = UtcDateTime::now().unix_timestamp();
        UserSession {
            created_at: now,
            last_accesses: now,
            visit_count: 0,
            anonim_user: true,
            data: None,
        }
    }

    pub fn update_last_access(&mut self) {
        let now = Utc::now().timestamp();
        self.last_accesses = now;
        self.visit_count += 1;
    }

    pub fn authenticate(&mut self) {
        self.anonim_user = false;
    }
}
