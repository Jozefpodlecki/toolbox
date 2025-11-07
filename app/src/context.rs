use chrono::{DateTime, Utc};
use uuid::Uuid;


pub struct AppContext {
    pub launched_on: DateTime<Utc>,
    pub session_id: Uuid
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            launched_on: Utc::now(),
            session_id: Uuid::now_v7()
        }
    }
}