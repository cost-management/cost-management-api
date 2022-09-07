use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum NotificationType {
    INVITE,
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for NotificationType {
    type Err = ();

    fn from_str(input: &str) -> Result<NotificationType, Self::Err> {
        match input {
            "INVITE" => Ok(NotificationType::INVITE),
            _ => Err(()),
        }
    }
}
