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
    let body: FolderInsertDto = match serde_json::from_str(body) {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_deserialization_response(),
    };

    println!("body: {:?}", body);

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let mut tx = match database_connection.begin().await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_create_transaction(),
    };

    println!("Transaction was created");

    match sqlx::query("insert into folder (id, title, folder_type, currency, skin, units, nanos, created_at) values ($1, $2, $3, $4, $5, $6, $7, 'now()');")
        .bind(body.id())
        .bind(body.title())
        .bind(body.folder_type())
        .bind(body.currency())
        .bind(body.skin())
        .bind(body.units())
        .bind(body.nanos())
        .execute(&mut tx).await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response()
    };

    match sqlx::query("insert into customer_folder (customer_id, folder_id, customer_role) values ($1, $2, 'OWNER');")
        .bind(user_id)
        .bind(body.id())
        .execute(&mut tx).await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response()
    };

    match tx.commit().await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_commit_transaction_response(),
    };

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
            "id":"5cb570e7-fbaa-4f7c-b5fa-88d667d60b3b",
            "title":"",
            "folder_type":"CASH",
            "currency":"UAH",
            "skin":"RED",
            "units": 10,
            "nanos":12,
            "created_at":"2019-10-12T07:20:50.52Z"
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
