#[path = "../../folders/entity/entities.rs"]
mod entities;

use uuid::Uuid;
use serde::{Serialize, Deserialize};
use entities::{Currency, FolderType};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FolderDto {
    id: Uuid,
    owner_id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
}

impl FolderDto {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn owner_id(&self) -> Uuid {
        self.owner_id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn folder_type(&self) -> &FolderType {
        &self.folder_type
    }

    pub fn currency(&self) -> &Currency {
        &self.currency
    }
}