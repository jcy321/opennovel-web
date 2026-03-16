use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub book_id: Option<String>,
    pub chapter_id: Option<String>,
    pub user_preferences: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub last_active: DateTime<Utc>,
}

impl SessionContext {
    pub fn new(session_id: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            session_id: session_id.into(),
            book_id: None,
            chapter_id: None,
            user_preferences: HashMap::new(),
            created_at: now,
            last_active: now,
        }
    }

    pub fn with_book(mut self, book_id: impl Into<String>) -> Self {
        self.book_id = Some(book_id.into());
        self
    }

    pub fn with_chapter(mut self, chapter_id: impl Into<String>) -> Self {
        self.chapter_id = Some(chapter_id.into());
        self
    }
}
