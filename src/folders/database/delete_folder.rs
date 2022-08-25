use crate::utils::{database_utils, responses};
use lambda_runtime::Error;
use serde_json::Value;
use sqlx::Connection;
use std::str::FromStr;
use uuid::Uuid;

pub async fn delete_folder(body: &str) -> Result<Value, Error> {
    println!("DELETE /folder method started");
    let json_id: Value = serde_json::from_str(body)?;
    let folder_id = match Uuid::from_str(json_id["id"].as_str().unwrap()) {
        Ok(val) => val,
        Err(err) => return responses::get_fail_uuid_response(),
    };

    println!("uuid: {:?}", &folder_id);

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    match sqlx::query("DELETE FROM folder WHERE id = $1;")
        .bind(&folder_id)
        .execute(&mut database_connection)
        .await
    {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response(),
    };

    database_connection.close();
    println!("Database connection is closed");

    responses::get_ok_response_with_id(folder_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::runtime::Runtime;

    #[test]
    fn test_post_folder() -> Result<(), String> {
        let uuid = String::from("0c653dc4-a632-4215-adf2-79316450bd52");

        let body = json!(&uuid).to_string();

        println!("{}", body);
        let actual = delete_folder(&body);
        let expected: Result<Value, ()> = Ok(json!({
            "statusCode": 200,
            "body" : {"id":&uuid},
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
