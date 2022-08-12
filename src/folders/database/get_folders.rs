#[path = "../../utils/responses.rs"] mod utils;
#[path = "../../utils/database.rs"] mod database;
#[path = "../../folders/entity/entities.rs"] mod entities;

use serde_json::{json, Value};
use lambda_runtime::Error;
use sqlx::{Connection};
use uuid::Uuid;

pub async fn get_folders(user_id: &Uuid) -> Result<Value, Error> {

    println!("GET /folder method started");
    let mut database_connection = database::create_connection().await;

    println!("Connected to database");

    let folders = sqlx::query_as::<_, entities::Folder>("SELECT * FROM folder WHERE owner_id = $1;")
        .bind(user_id)
        .fetch_all(&mut database_connection).await?;

    database_connection.close();
    println!("Folders from db: {:?}", &folders);

    utils::get_ok_response(json!(folders))
}