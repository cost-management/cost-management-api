use lambda_runtime::Error;
use serde_json::{json, Value};
use sqlx::Connection;
use std::collections::HashMap;
use std::iter::Map;
use uuid::Uuid;

use crate::folders::dto::dtos::{FolderCustomerDto, FolderCustomerMetadata};
use crate::folders::entity::entities::Folder;
use crate::utils::{database_utils, responses};

pub async fn get_folders(user_id: &Uuid) -> Result<Value, Error> {
    println!("GET /folder method started");
    let mut database_connection = database_utils::create_connection().await;

    println!("Connected to database");

    let folders = match sqlx::query_as::<_, Folder>("select folder.id, title, folder_type, units, nanos, currency, skin, folder.created_at, customer_id, customer_role, email from folder join customer_folder on customer_folder.folder_id = folder.id join customer c on c.id = customer_folder.customer_id where folder.id in (select folder_id from customer_folder where customer_folder.customer_id = $1);")
        .bind(user_id)
        .fetch_all(&mut database_connection).await {
        Ok(val) => val,
        Err(err) => return responses::get_fail_query_response()
    };

    let response = map_to_folder_dto(folders);

    database_connection.close();
    println!("Folders from db: {:?}", &response);

    responses::get_ok_response(json!(response))
}

fn map_to_folder_dto(folders: Vec<Folder>) -> Vec<FolderCustomerDto> {
    let mut temp_map: HashMap<String, FolderCustomerDto> =
        HashMap::with_capacity(folders.capacity());

    for folder in folders {

        temp_map
            .entry(folder.id().to_string())
            .and_modify(|f| {
                f.folder_customer_metadata()
                    .push(FolderCustomerMetadata::new(
                        folder.customer_id(),
                        folder.customer_role(),
                        folder.email().to_string(),
                    ))
            })
            .or_insert_with(|| {
                FolderCustomerDto::new(
                    folder.id(),
                    folder.title().to_string(),
                    folder.folder_type(),
                    folder.currency(),
                    folder.units(),
                    folder.nanos(),
                    folder.skin(),
                    folder.created_at(),
                    vec![FolderCustomerMetadata::new(
                        folder.customer_id(),
                        folder.customer_role(),
                        folder.email().to_string(),
                    )],
                )
            });
    }

    temp_map.into_iter().map(|entry| entry.1).collect()
}
