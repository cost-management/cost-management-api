use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, sqlx::Type, Serialize, Deserialize)]
pub struct Income {
    id: Uuid,
    title: String,
    folder_id: Uuid,
    income_category: String,
    customer_id: Uuid,
    created_at: sqlx::types::chrono::DateTime<Utc>,
    units: i64,
    nanos: i16,
    timezone: i16,
}

impl Income {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn folder_id(&self) -> Uuid {
        self.folder_id
    }
    pub fn income_category(&self) -> &str {
        &self.income_category
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn created_at(&self) -> sqlx::types::chrono::DateTime<Utc> {
        self.created_at
    }
    pub fn units(&self) -> i64 {
        self.units
    }
    pub fn nanos(&self) -> i16 {
        self.nanos
    }
    pub fn timezone(&self) -> i16 {
        self.timezone
    }
}
