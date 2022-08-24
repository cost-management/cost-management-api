use crate::folders::entity::entities::CustomerFolderRole;
use lambda_runtime::Error;
use serde_json::Value;
use sqlx::Connection;
use std::str::FromStr;
use uuid::Uuid;

use crate::invite::dto::dtos::InvitePostDto;
use crate::utils::{database_utils, responses};

pub async fn post_invite(body: &str) -> Result<Value, Error> {
    println!("POST /invites method started");
    let body: InvitePostDto = serde_json::from_str(body)?;

    println!("body: {:?}", body);

    let folder_id = Uuid::from_str(body.folder_id().to_string().replace('"', "").as_str())?;
    let invited_customer_email = body.invited_customer_email().to_string().replace('"', "");
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

    sqlx::query(
        "insert into invite (folder_id, invited_customer_id, customer_role, created_at)  values ($1, (select id from customer where email = $2), $3, 'now()');",
    )
    .bind(folder_id)
    .bind(invited_customer_email)
    .bind(customer_role)
    .execute(&mut database_connection)
    .await?;

    database_connection.close();

    println!("Database connection is closed");

    responses::get_ok_response_with_id(body.folder_id())
}
