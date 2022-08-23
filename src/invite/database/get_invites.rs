use lambda_runtime::Error;
use serde_json::{json, Value};
use sqlx::Connection;
use uuid::Uuid;

use crate::invite::entity::entities::Invite;
use crate::utils::{database_utils, responses};

pub async fn get_invites(user_id: &Uuid) -> Result<Value, Error> {
    println!("GET /invites method started");
    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let response = sqlx::query_as::<_, Invite>("select folder.id, title, folder_type, currency, skin, invite.created_at, customer.email from folder join invite on folder.id = invite.folder_id join customer on customer.id = invite.invited_customer_id  where customer.id = $1;")
        .bind(user_id)
        .fetch_all(&mut database_connection)
        .await?;

    database_connection.close();
    println!("Incomes from db: {:?}", &response);

    responses::get_ok_response(json!(response))
}
