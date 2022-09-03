use crate::folders::entity::entities::{Currency, CustomerFolderRole, FolderSkin, FolderType};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderInsertDto {
    id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
    skin: FolderSkin,
    units: i64,
    nanos: i16,
    created_at: sqlx::types::chrono::DateTime<Utc>,
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
    pub fn created_at(&self) -> sqlx::types::chrono::DateTime<Utc> {
        self.created_at
    }
    pub fn new(
        id: Uuid,
        title: String,
        folder_type: FolderType,
        currency: Currency,
        skin: FolderSkin,
        units: i64,
        nanos: i16,
        created_at: sqlx::types::chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            folder_type,
            currency,
            skin,
            units,
            nanos,
            created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FolderUpdateDto {
    id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
    skin: FolderSkin,
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

impl FolderInsertDto {}

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
    folder_customer_metadata: Vec<FolderCustomerMetadata>,
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
        folder_customer_metadata: Vec<FolderCustomerMetadata>,
    ) -> Self {
        Self {
            id,
            title,
            folder_type,
            currency,
            units,
            nanos,
            skin,
            created_at,
            folder_customer_metadata,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderCustomerMetadata {
    customer_id: Uuid,
    customer_role: CustomerFolderRole,
    customer_email: String,
}

impl FolderCustomerMetadata {
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn customer_role(&self) -> CustomerFolderRole {
        self.customer_role
    }
    pub fn new(
        customer_id: Uuid,
        customer_role: CustomerFolderRole,
        customer_email: String,
    ) -> Self {
        Self {
            customer_id,
            customer_role,
            customer_email,
        }
    }
}
