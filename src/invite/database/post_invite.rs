use crate::folders::entity::entities::CustomerFolderRole;
use lambda_runtime::Error;
use serde_json::Value;
use sqlx::{Connection, Row};
use std::str::FromStr;
use uuid::Uuid;

use crate::invite::dto::dtos::InvitePostDto;
use crate::utils::notification::notifications;
use crate::utils::{database_utils, responses};

pub async fn post_invite(body: &str, user_id: &Uuid) -> Result<Value, Error> {
    println!("POST /invites method started");
    let body: InvitePostDto = match serde_json::from_str(body) {
        Ok(val) => val,
        Err(err) => return responses::get_fail_on_deserialization_response(),
    };

    println!("body: {:?}", body);

    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    match sqlx::query(
        "insert into invite (folder_id, invited_customer_id, customer_role, created_at)  values ($1, (select id from customer where email = $2), $3, 'now()');",
    )
        .bind(body.folder_id())
        .bind(body.invited_customer_email())
        .bind(body.customer_role())
        .execute(&mut database_connection)
        .await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response()
    };

    let owner_email: String = match sqlx::query("select email from customer where id = $1")
        .bind(user_id)
        .fetch_one(&mut database_connection)
        .await
    {
        Ok(email) => email.get(0),
        Err(err) => return responses::get_fail_query_response(),
    };

    let token: String = match sqlx::query("select token from customer where email = $1")
        .bind(body.invited_customer_email())
        .fetch_one(&mut database_connection)
        .await
    {
        Ok(token) => token.get(0),
        Err(err) => return responses::get_fail_query_response(),
    };

    database_connection.close();

    println!("Database connection is closed");

    let notification_result =
        notifications::send_invite_notification(owner_email.as_str(), token.as_str()).await?;

    println!("Notification was sent: {}", notification_result);

    responses::get_ok_response_with_id(body.folder_id())
}
