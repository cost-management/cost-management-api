use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use sqlx::types::chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, sqlx::FromRow, sqlx::Type, Serialize, Deserialize)]
pub struct Folder {
    id: Uuid,
    customer_id: Uuid,
    customer_role: CustomerFolderRole,
    title: String,
    folder_type: FolderType,
    units: i64,
    nanos: i16,
    skin: FolderSkin,
    currency: Currency,
    created_at: sqlx::types::chrono::DateTime<Utc>,
}

impl Folder {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn customer_role(&self) -> CustomerFolderRole {
        self.customer_role
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn folder_type(&self) -> FolderType {
        self.folder_type
    }
    pub fn units(&self) -> i64 {
        self.units
    }
    pub fn nanos(&self) -> i16 {
        self.nanos
    }
    pub fn currency(&self) -> Currency {
        self.currency
    }
    pub fn created_at(&self) -> sqlx::types::chrono::DateTime<Utc> {
        self.created_at
    }
    pub fn skin(&self) -> FolderSkin {
        self.skin
    }
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Copy, Clone)]
#[sqlx(type_name = "folder_type")]
pub enum FolderType {
    CARD,
    CASH,
}

impl fmt::Display for FolderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for FolderType {
    type Err = ();

    fn from_str(input: &str) -> Result<FolderType, Self::Err> {
        match input {
            "CARD" => Ok(FolderType::CARD),
            "CASH" => Ok(FolderType::CASH),
            _ => Err(()),
        }
    }
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Currency {
    UAH,
    USD,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Currency {
    type Err = ();

    fn from_str(input: &str) -> Result<Currency, Self::Err> {
        match input {
            "UAH" => Ok(Currency::UAH),
            "USD" => Ok(Currency::USD),
            _ => Err(()),
        }
    }
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Copy, Clone)]
#[sqlx(type_name = "customer_folder_role")]
pub enum CustomerFolderRole {
    OWNER,
    ADMIN,
    USER,
}

impl fmt::Display for CustomerFolderRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for CustomerFolderRole {
    type Err = ();

    fn from_str(input: &str) -> Result<CustomerFolderRole, Self::Err> {
        match input {
            "OWNER" => Ok(CustomerFolderRole::OWNER),
            "ADMIN" => Ok(CustomerFolderRole::ADMIN),
            "USER" => Ok(CustomerFolderRole::USER),
            _ => Err(()),
        }
    }
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize, Copy, Clone)]
#[sqlx(type_name = "folder_skin")]
pub enum FolderSkin {
    BLUE,
    GREEN,
    RED,
}

impl fmt::Display for FolderSkin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for FolderSkin {
    type Err = ();

    fn from_str(input: &str) -> Result<FolderSkin, Self::Err> {
        match input {
            "BLUE" => Ok(FolderSkin::BLUE),
            "GREEN" => Ok(FolderSkin::GREEN),
            "RED" => Ok(FolderSkin::RED),
            _ => Err(()),
        }
    }
}