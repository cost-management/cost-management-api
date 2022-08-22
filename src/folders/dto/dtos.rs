use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::types::chrono::Utc;
use crate::folders::entity::entities::{Currency, FolderType, CustomerFolderRole, FolderSkin};

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderInsertDto {
    id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
    skin: FolderSkin,
    units: i64,
    nanos: i16,
}

impl FolderInsertDto {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn folder_type(&self) -> FolderType {
        self.folder_type
    }

    pub fn currency(&self) -> Currency {
        self.currency
    }
    pub fn skin(&self) -> FolderSkin {
        self.skin
    }
    pub fn units(&self) -> i64 {
        self.units
    }
    pub fn nanos(&self) -> i16 {
        self.nanos
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderUpdateDto {
    id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
    skin: FolderSkin
}

impl FolderUpdateDto {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn folder_type(&self) -> FolderType {
        self.folder_type
    }
    pub fn currency(&self) -> Currency {
        self.currency
    }
    pub fn skin(&self) -> FolderSkin {
        self.skin
    }
}

impl FolderInsertDto {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderCustomerDto {
    id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
    units: i64,
    nanos: i16,
    skin: FolderSkin,
    created_at: sqlx::types::chrono::DateTime<Utc>,
    folder_customer_metadata: Vec<FolderCustomerMetadata>
}

impl FolderCustomerDto {
    pub fn id(&self) -> Uuid {
        self.id
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
    pub fn units(&self) -> i64 {
        self.units
    }
    pub fn nanos(&self) -> i16 {
        self.nanos
    }
    pub fn folder_customer_metadata(&mut self) -> &mut Vec<FolderCustomerMetadata> {
        &mut self.folder_customer_metadata
    }
    pub fn created_at(&self) -> sqlx::types::chrono::DateTime<Utc> {
        self.created_at
    }
    pub fn skin(&self) -> FolderSkin {
        self.skin
    }
    pub fn new(
        id: Uuid,
        title: String,
        folder_type: FolderType,
        currency: Currency,
        units: i64,
        nanos: i16,
        skin: FolderSkin,
        created_at: sqlx::types::chrono::DateTime<Utc>,
        folder_customer_metadata: Vec<FolderCustomerMetadata>
    ) -> Self {
        Self { id, title, folder_type, currency, units, nanos, skin, created_at, folder_customer_metadata }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct FolderCustomerMetadata {
    customer_id: Uuid,
    customer_role: CustomerFolderRole
}

impl FolderCustomerMetadata {
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn customer_role(&self) -> CustomerFolderRole {
        self.customer_role
    }
    pub fn new(customer_id: Uuid, customer_role: CustomerFolderRole) -> Self {
        Self { customer_id, customer_role }
    }
}
