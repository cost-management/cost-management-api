use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::income::entity::entities::IncomeCategory;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeDto{
    id: Uuid,
    title: String,
    folder_id: Uuid,
    income_category: IncomeCategory,
    customer_id: Uuid,
    units: i64,
    nanos: i16,
    timezone: i16
}

impl IncomeDto {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn folder_id(&self) -> Uuid {
        self.folder_id
    }
    pub fn income_category(&self) -> IncomeCategory {
        self.income_category
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
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