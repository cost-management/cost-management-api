use std::str::FromStr;
use serde_json::Value;
use lambda_runtime::Error;
use sqlx::Connection;
use uuid::Uuid;

use crate::folders::dto::dtos::FolderUpdateDto;
use crate::folders::entity::entities::{Currency, FolderSkin, FolderType};
use crate::utils::{database_utils, responses};

pub async fn put_folder(body: &str) -> Result<Value, Error> {

    println!("PUT /folder method started");
    let body: FolderUpdateDto = serde_json::from_str(body)?;

    println!("body: {:?}", body);

    let folder_id = Uuid::from_str(body.id().to_string().replace('"', "").as_str())?;
    let title = body.title().to_string().replace('"', "");
    let folder_type = FolderType::from_str(body.folder_type().to_string().to_ascii_uppercase().replace('"', "").as_str()).unwrap();
    let currency = Currency::from_str(body.currency().to_string().to_ascii_uppercase().replace('"', "").as_str()).unwrap();
    let folder_skin = FolderSkin::from_str(body.skin().to_string().to_ascii_uppercase().replace('"', "").as_str()).unwrap();

    println!("Variables from payload are set");

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    sqlx::query("update folder set (title, folder_type, currency, skin) = ($1, $2, $3, $4) where id = $5;")
        .bind(title)
        .bind(folder_type)
        .bind(currency)
        .bind(folder_skin)
        .bind(folder_id)
        .execute(&mut database_connection).await?;


    database_connection.close();

    println!("Database connection is closed");

    responses::get_ok_response_with_id(body.id())
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tokio::runtime::Runtime;
    use super::*;

    #[test]
    fn test_post_folder() -> Result<(), String> {
        let uuid = String::from("218cd1f0-6e0e-4b76-9b45-ef073afff5fd");
        let body = json!({
                            "id": &uuid,
                            "title": "test_folder2",
                            "folder_type": "CARD",
                            "currency": "UAH",
                            "skin": "BLUE"
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