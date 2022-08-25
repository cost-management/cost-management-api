use lambda_runtime::Error;
use serde_json::Value;
use sqlx::Connection;
use std::str::FromStr;
use uuid::Uuid;

use crate::income::dto::dtos::IncomeDto;
use crate::utils::{database_utils, responses};

pub async fn post_income(body: &str) -> Result<Value, Error> {
    println!("POST /incomes method started");
    let body: IncomeDto = serde_json::from_str(body)?;

    println!("body: {:?}", body);
    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let income_id = Uuid::from_str(body.id().to_string().replace('"', "").as_str())?;
    let customer_id = Uuid::from_str(body.customer_id().to_string().replace('"', "").as_str())?;
    let title = body.title().to_string().replace('"', "");
    let income_category = body.income_category();
    let folder_id = Uuid::from_str(body.folder_id().to_string().replace('"', "").as_str())?;
    let units = body.units();
    let nanos = body.nanos();
    let timezone = body.timezone();

    println!("Variables from payload are set");

    sqlx::query("INSERT INTO income (id, customer_id, title, income_category, folder_id, units, nanos, timezone, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'now()');")
        .bind(income_id)
        .bind(customer_id)
        .bind(title)
        .bind(income_category)
        .bind(folder_id)
        .bind(units)
        .bind(nanos)
        .bind(timezone)
        .execute(&mut database_connection).await?;

    database_connection.close();

    println!("Database connection is closed");

    responses::get_ok_response_with_id(body.id())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::runtime::Runtime;

    #[test]
    fn test_post_income() -> Result<(), String> {
        let uuid = Uuid::new_v4();
        let body = json!({
            "id":"24153ec5-5ec6-4866-b93a-e07ee5e37da6",
            "title":"test",
            "folder_id":"e23eb1b3-58c2-40a2-ba17-31bcbf261107",
            "income_category":"FOOD",
            "customer_id":"e23eb1b3-58c2-40a2-ba17-31bcbf261107",
            "units":-1999,
            "nanos":12,
            "timezone":3
        })
        .to_string();
        let actual = post_income(&body);
        let expected: Result<Value, ()> = Ok(json!({
            "statusCode": 200,
            "body" : {"id": uuid.to_string().as_str()},
            "isBase64Encoded" : false,
            "headers" : {"content-type" : "application/json"},
        }));
        assert_eq!(
            Runtime::new()
                .unwrap()
                .block_on(actual)
                .unwrap()
                .to_string(),
            expected.unwrap().to_string()
        );
        Ok(())
    }
}
