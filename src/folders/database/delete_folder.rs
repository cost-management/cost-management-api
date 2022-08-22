use std::str::FromStr;
use serde_json::Value;
use lambda_runtime::Error;
use sqlx::{Connection};
use uuid::Uuid;
use crate::utils::{database_utils, responses};

pub async fn delete_folder(body: &str) -> Result<Value, Error> {

    println!("DELETE /folder method started");
    let json_id: Value = serde_json::from_str(body)?;
    let folder_id = Uuid::from_str(json_id["id"].as_str().unwrap())?;

    println!("uuid: {:?}", &folder_id);

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    sqlx::query("DELETE FROM folder WHERE id = $1;")
        .bind(&folder_id)
        .execute(&mut database_connection).await?;

    database_connection.close();
    println!("Database connection is closed");

    responses::get_ok_response_with_id(folder_id)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tokio::runtime::Runtime;
    use super::*;

    #[test]
    fn test_post_folder() -> Result<(), String> {
        let uuid = String::from("0c653dc4-a632-4215-adf2-79316450bd52");

        let body = json!(
                            &uuid
                        ).to_string();

        println!("{}", body);
        let actual = delete_folder(&body);
        let expected: Result<Value, ()> = Ok(json!({
                                                    "statusCode": 200,
                                                    "body" : {"id":&uuid},
                                                    "isBase64Encoded" : false,
                                                    "headers" : {"content-type" : "application/json"},
                                                }));
        assert_eq!(Runtime::new().unwrap().block_on(actual).unwrap().to_string(), expected.unwrap().to_string());
        Ok(())
    }
}