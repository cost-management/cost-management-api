use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeDto {
    id: Uuid,
    title: String,
    folder_id: Uuid,
    income_category: String,
    customer_id: Uuid,
    amount: rust_decimal::Decimal,
    timezone: i16,
    created_at: sqlx::types::chrono::DateTime<Utc>,
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
    pub fn income_category(&self) -> &str {
        &self.income_category
    }
    pub fn customer_id(&self) -> Uuid {
        self.customer_id
    }
    pub fn timezone(&self) -> i16 {
        self.timezone
    }
    pub fn created_at(&self) -> sqlx::types::chrono::DateTime<Utc> {
        self.created_at
    }
    pub fn amount(&self) -> rust_decimal::Decimal {
        self.amount
    }
}
