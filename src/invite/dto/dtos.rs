use crate::folders::entity::entities::CustomerFolderRole;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct InviteDto {
    folder_id: Uuid,
    invited_customer_id: Uuid,
    customer_role: CustomerFolderRole,
}

impl InviteDto {
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
