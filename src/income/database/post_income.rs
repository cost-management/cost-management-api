/*#[path = "../../utils/responses.rs"]
mod utils;
#[path = "../../utils/database.rs"]
mod database;
#[path = "../../income/entity/entities.rs"] mod entities;

use std::str::FromStr;
use serde_json::{json, Value};
use lambda_runtime::Error;
use sqlx::Connection;
use uuid::Uuid;
use entities::{Currency, FolderType};

pub async fn post_income(body: &String) -> Result<Value, Error> {

    println!("POST /income method started");
    let body: dtos::FolderDto = serde_json::from_str(body)?;

    println!("body: {:?}", body);
    let mut database_connection = database::create_connection().await;

    println!("Connected to database");

    let folder_id = Uuid::from_str(body.id().to_string().replace('"', "").as_str()).unwrap();
    let owner_id = Uuid::from_str(body.owner_id().to_string().replace('"', "").as_str()).unwrap();
    let title = body.title().to_string().replace('"', "");
    let folder_type = FolderType::from_str(body.folder_type().to_string().to_ascii_uppercase().replace('"', "").as_str()).unwrap();
    let currency = Currency::from_str(body.currency().to_string().to_ascii_uppercase().replace('"', "").as_str()).unwrap();

    println!("Variables from payload are set");

    sqlx::query("INSERT INTO folder (id, owner_id, title, folder_type, currency, created_at) VALUES ($1, $2, $3, $4, $5, 'now()');")
        .bind(folder_id)
        .bind(owner_id)
        .bind(title)
        .bind(folder_type)
        .bind(currency)
        .execute(&mut database_connection).await?;


    database_connection.close();

    println!("Database connection is closed");

    utils::get_ok_response_with_id(body.id())
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;
    use super::*;

    #[test]
    fn test_post_folder() -> Result<(), String> {
        let uuid = Uuid::new_v4();
        let body = json!({
                            "id": &uuid,
                            "owner_id": &uuid,
                            "title": "test_folder",
                            "folder_type": "CARD",
                            "currency": "UAN"
                        }).to_string();
        let actual = post_folder(&body);
        let expected: Result<Value, ()> = Ok(json!({
                                                    "statusCode": 200,
                                                    "body" : {"id": uuid.to_string().as_str()},
                                                    "isBase64Encoded" : false,
                                                    "headers" : {"content-type" : "application/json"},
                                                }));
        assert_eq!(Runtime::new().unwrap().block_on(actual).unwrap().to_string(), expected.unwrap().to_string());
        Ok(())
    }
}*/