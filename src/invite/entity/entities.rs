use crate::folders::entity::entities::{Currency, FolderSkin, FolderType};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, sqlx::Type, Serialize, Deserialize)]
pub struct Invite {
    id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
    skin: FolderSkin,
    email: String,
    created_at: sqlx::types::chrono::DateTime<Utc>,
}
