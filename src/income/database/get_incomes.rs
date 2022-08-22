use serde_json::{json, Value};
use lambda_runtime::Error;
use sqlx::Connection;
use uuid::Uuid;

use crate::utils::{database_utils, responses};
use crate::income::entity::entities::Income;

pub async fn get_folders(folder_id: &Uuid) -> Result<Value, Error> {

    println!("GET /incomes method started");
    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let response = sqlx::query_as::<_, Income>("select * from income where folder_id = $1;")
        .bind(folder_id)
        .fetch_all(&mut database_connection).await?;

    database_connection.close();
    println!("Incomes from db: {:?}", &response);

    responses::get_ok_response(json!(response))
}