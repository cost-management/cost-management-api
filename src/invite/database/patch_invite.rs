use crate::folders::entity::entities::CustomerFolderRole;
use lambda_runtime::Error;
use serde_json::Value;
use sqlx::Connection;
use std::str::FromStr;
use uuid::Uuid;

use crate::invite::dto::dtos::InvitePatchDto;
use crate::utils::{database_utils, responses};

pub async fn patch_invite(body: &str) -> Result<Value, Error> {
    println!("PATCH /invites method started");
    let body: InvitePatchDto = match serde_json::from_str(body) {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_deserialization_response(),
    };

    println!("body: {:?}", body);

    println!("Variables from payload are set");

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let mut tx = match database_connection.begin().await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_create_transaction(),
    };

    println!("Transaction was created");

    match sqlx::query(
        "INSERT INTO customer_folder (folder_id, customer_id, customer_role) VALUES ($1, $2, $3);",
    )
    .bind(body.folder_id())
    .bind(body.invited_customer_id())
    .bind(body.customer_role())
    .execute(&mut tx)
    .await
    {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response(),
    };

    match sqlx::query("delete from invite where folder_id = $1 and invited_customer_id = $2;")
        .bind(body.folder_id())
        .bind(body.invited_customer_id())
        .execute(&mut tx)
        .await
    {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response(),
    };

    match tx.commit().await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_commit_transaction_response(),
    };

    println!("Transaction was committed");

    database_connection.close();

    println!("Database connection is closed");

    responses::get_ok_response_with_id(body.folder_id())
}
