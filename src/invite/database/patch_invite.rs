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
    let body: InvitePatchDto = serde_json::from_str(body)?;

    println!("body: {:?}", body);

    let folder_id = Uuid::from_str(body.folder_id().to_string().replace('"', "").as_str())?;
    let invited_customer_id = Uuid::from_str(
        body.invited_customer_id()
            .to_string()
            .replace('"', "")
            .as_str(),
    )?;
    let customer_role = CustomerFolderRole::from_str(
        body.customer_role()
            .to_string()
            .to_ascii_uppercase()
            .replace('"', "")
            .as_str(),
    )
    .unwrap();

    println!("Variables from payload are set");

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let mut tx = database_connection.begin().await?;

    println!("Transaction was created");

    sqlx::query(
        "INSERT INTO customer_folder (folder_id, customer_id, customer_role) VALUES ($1, $2, $3);",
    )
    .bind(folder_id)
    .bind(invited_customer_id)
    .bind(customer_role)
    .execute(&mut tx)
    .await?;

    sqlx::query("delete from invite where folder_id = $1 and invited_customer_id = $2;")
        .bind(folder_id)
        .bind(invited_customer_id)
        .execute(&mut tx)
        .await?;

    tx.commit().await?;

    println!("Transaction was committed");

    database_connection.close();

    println!("Database connection is closed");

    responses::get_ok_response_with_id(body.folder_id())
}
