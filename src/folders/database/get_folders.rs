use std::collections::HashMap;
use std::iter::Map;
use serde_json::{json, Value};
use lambda_runtime::Error;
use sqlx::Connection;
use uuid::Uuid;

use crate::utils::{database_utils, responses};
use crate::folders::dto::dtos::{FolderCustomerDto, FolderCustomerMetadata};
use crate::folders::entity::entities::Folder;

pub async fn get_folders(user_id: &Uuid) -> Result<Value, Error> {

    println!("GET /folder method started");
    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let folders = sqlx::query_as::<_, Folder>("select * from folder join customer_folder on customer_folder.folder_id = folder.id where folder.id in (select folder_id from customer_folder where customer_folder.customer_id = $1);")
        .bind(user_id)
        .fetch_all(&mut database_connection).await?;

    let response = map_to_folder_dto(folders);

    database_connection.close();
    println!("Folders from db: {:?}", &response);

    responses::get_ok_response(json!(response))
}

fn map_to_folder_dto(folders: Vec<Folder>) -> Vec<FolderCustomerDto>{
    let mut temp_map: HashMap<String, FolderCustomerDto> = HashMap::with_capacity(folders.capacity());

    for folder in folders {
        let metadata = FolderCustomerMetadata::new(folder.customer_id(), folder.customer_role());

        temp_map
            .entry(
                folder
                    .id()
                    .to_string())
            .and_modify(
                |f| f.folder_customer_metadata().push(metadata))
            .or_insert_with(|| FolderCustomerDto::new(
                    folder.id(),
                    folder.title().to_string(),
                    folder.folder_type(),
                    folder.currency(),
                    folder.units(),
                    folder.nanos(),
                    folder.skin(),
                    folder.created_at(),
                    vec![metadata]));
    }

    temp_map.into_iter().map(|entry| entry.1).collect()
}
