use std::fmt;
use std::str::FromStr;
use uuid::Uuid;
use sqlx::types::chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, sqlx::FromRow, sqlx::Type, Serialize, Deserialize)]
pub(crate) struct Folder {
    id: Uuid,
    owner_id: Uuid,
    title: String,
    folder_type: FolderType,
    currency: Currency,
    created_at: sqlx::types::chrono::DateTime<Utc>,
}

impl Folder {
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

    pub fn created_at(&self) -> sqlx::types::chrono::DateTime<Utc> {
        self.created_at
    }
}

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "folder_type")]
pub(crate) enum FolderType {
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

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
pub(crate) enum Currency {
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