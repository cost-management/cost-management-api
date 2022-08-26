use crate::utils::{database_utils, responses};
use lambda_runtime::Error;
use serde_json::{json, Value};
use sqlx::Connection;
use std::str::FromStr;
use uuid::Uuid;

pub async fn delete_folder(body: &str) -> Result<Value, Error> {
    println!("DELETE /incomes method started");
    let json_id: Value = serde_json::from_str(body)?;
    let income_id = match Uuid::from_str(json_id["id"].as_str().unwrap()) {
        Ok(val) => val,
        Err(err) => return responses::get_fail_uuid_response(),
    };

    println!("uuid: {:?}", &income_id);

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    match sqlx::query("DELETE FROM income WHERE id = $1;")
        .bind(&income_id)
        .execute(&mut database_connection)
        .await
    {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response(),
    };

    database_connection.close();
    println!("Database connection is closed");

    responses::get_ok_response_with_id(income_id)
}
