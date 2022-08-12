#[path = "../../utils/responses.rs"]
mod utils;
#[path = "../../utils/database.rs"]
mod database;
#[path = "../../folders/dto/dtos.rs"]
mod dtos;
#[path = "../../folders/entity/entities.rs"]
mod entities;

use std::str::FromStr;
use serde_json::{json, Value};
use lambda_runtime::Error;
use sqlx::Connection;
use uuid::Uuid;
use entities::{FolderType, Currency};

pub async fn put_folder(body: &String) -> Result<Value, Error> {

    println!("PUT /folder method started");
    let body: dtos::FolderDto = serde_json::from_str(body)?;

    println!("body: {:?}", body);
    let mut database_connection = database::create_connection().await;

    println!("Connected to database");

    let folder_id = Uuid::from_str(body.id().to_string().replace('"', "").as_str()).unwrap();
    let title = body.title().to_string().replace('"', "");
    let folder_type = FolderType::from_str(body.folder_type().to_string().to_ascii_uppercase().replace('"', "").as_str()).unwrap();
    let currency = Currency::from_str(body.currency().to_string().to_ascii_uppercase().replace('"', "").as_str()).unwrap();

    println!("Variables from payload are set");

    sqlx::query("UPDATE folder SET (title, folder_type, currency) = ($1, $2, $3) WHERE id = $4;")
        .bind(title)
        .bind(folder_type)
        .bind(currency)
        .bind(folder_id)
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
        let uuid = String::from("218cd1f0-6e0e-4b76-9b45-ef073afff5fd");
        let body = json!({
                            "id": &uuid,
                            "owner_id": &uuid,
                            "title": "test_folder2",
                            "folder_type": "CARD",
                            "currency": "UAN"
                        }).to_string();
        let actual = put_folder(&body);
        let expected: Result<Value, ()> = Ok(json!({
                                                    "statusCode": 200,
                                                    "body" : {"id": uuid.to_string().as_str()},
                                                    "isBase64Encoded" : false,
                                                    "headers" : {"content-type" : "application/json"},
                                                }));
        assert_eq!(Runtime::new().unwrap().block_on(actual).unwrap().to_string(), expected.unwrap().to_string());
        Ok(())
    }
}