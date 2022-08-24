use crate::folders::dto::dtos::FolderInsertDto;
use crate::folders::entity::entities::{Currency, FolderSkin, FolderType};
use crate::utils::{database_utils, responses};
use lambda_runtime::Error;
use serde_json::Value;
use sqlx::Connection;
use std::str::FromStr;
use uuid::Uuid;

pub async fn post_folder(user_id: &Uuid, body: &str) -> Result<Value, Error> {
    println!("POST /folder method started");
    let body: FolderInsertDto = serde_json::from_str(body)?;

    println!("body: {:?}", body);

    let folder_id = Uuid::from_str(body.id().to_string().replace('"', "").as_str())?;
    let title = body.title().to_string().replace('"', "");
    let folder_type = FolderType::from_str(
        body.folder_type()
            .to_string()
            .to_ascii_uppercase()
            .replace('"', "")
            .as_str(),
    )
    .unwrap();
    let currency = Currency::from_str(
        body.currency()
            .to_string()
            .to_ascii_uppercase()
            .replace('"', "")
            .as_str(),
    )
    .unwrap();
    let folder_skin = FolderSkin::from_str(
        body.skin()
            .to_string()
            .to_ascii_uppercase()
            .replace('"', "")
            .as_str(),
    )
    .unwrap();
    let units = body.units();
    let nanos = body.nanos();

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let mut tx = database_connection.begin().await?;

    println!("Transaction was created");

    sqlx::query("insert into folder (id, title, folder_type, currency, skin, units, nanos, created_at) values ($1, $2, $3, $4, $5, $6, $7, 'now()');")
        .bind(folder_id)
        .bind(title)
        .bind(folder_type)
        .bind(currency)
        .bind(folder_skin)
        .bind(units)
        .bind(nanos)
        .execute(&mut tx).await?;

    sqlx::query("insert into customer_folder (customer_id, folder_id, customer_role) values ($1, $2, 'OWNER');")
        .bind(user_id)
        .bind(folder_id)
        .execute(&mut tx).await?;

    tx.commit().await?;

    println!("Transaction was committed");

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
        let uuid = Uuid::new_v4();
        let body = json!({
            "id": &uuid,
            "title": "test_folder",
            "folder_type": "CARD",
            "currency": "UAH",
            "skin": "BLUE"
        })
        .to_string();
        let actual = post_folder(&uuid, &body);
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
