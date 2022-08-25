use lambda_runtime::Error;
use serde_json::Value;
use sqlx::Connection;
use std::str::FromStr;
use uuid::Uuid;

use crate::folders::dto::dtos::FolderUpdateDto;
use crate::folders::entity::entities::{Currency, FolderSkin, FolderType};
use crate::utils::{database_utils, responses};

pub async fn put_folder(body: &str) -> Result<Value, Error> {
    println!("PUT /folder method started");
    let body: FolderUpdateDto = match serde_json::from_str(body) {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_deserialization_response(),
    };

    println!("body: {:?}", body);

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    match sqlx::query(
        "update folder set (title, folder_type, currency, skin) = ($1, $2, $3, $4) where id = $5;",
    )
    .bind(body.title())
    .bind(body.folder_type())
    .bind(body.currency())
    .bind(body.skin())
    .bind(body.id())
    .execute(&mut database_connection)
    .await
    {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response(),
    };

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
    fn test_post_folder() -> Result<(), String> {
        let uuid = String::from("218cd1f0-6e0e-4b76-9b45-ef073afff5fd");
        let body = json!({
            "id": &uuid,
            "title": "test_folder2",
            "folder_type": "CARD",
            "currency": "UAH",
            "skin": "BLUE"
        })
        .to_string();
        let actual = put_folder(&body);
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
