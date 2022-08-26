use crate::folders::entity::entities::CustomerFolderRole;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitePatchDto {
    folder_id: Uuid,
    invited_customer_id: Uuid,
    customer_role: CustomerFolderRole,
}

impl InvitePatchDto {
    pub fn folder_id(&self) -> Uuid {
        self.folder_id
    }
    pub fn invited_customer_id(&self) -> Uuid {
        self.invited_customer_id
    }
    pub fn customer_role(&self) -> CustomerFolderRole {
        self.customer_role
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvitePostDto {
    folder_id: Uuid,
    invited_customer_email: String,
    customer_role: CustomerFolderRole,
}

impl InvitePostDto {
    pub fn folder_id(&self) -> Uuid {
        self.folder_id
    }
    pub fn invited_customer_email(&self) -> String {
        self.invited_customer_email.to_ascii_lowercase()
    }
    pub fn customer_role(&self) -> CustomerFolderRole {
        self.customer_role
    }
}
