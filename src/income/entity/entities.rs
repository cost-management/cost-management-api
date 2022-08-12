/*use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use sqlx::types::chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, sqlx::FromRow, sqlx::Type, Serialize, Deserialize)]
pub(crate) struct Folder {
    id: Uuid,
    title: String,
    folder_id: Uuid,
    folder_type: FolderType,
    currency: Currency,
    created_at: sqlx::types::chrono::DateTime<Utc>,
}*/