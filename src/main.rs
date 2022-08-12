#[path = "folders/database/get_folders.rs"]
mod get_folders;
#[path = "folders/database/post_folder.rs"]
mod post_folder;
#[path = "folders/database/delete_folder.rs"]
mod delete_folder;
#[path = "folders/database/put_folder.rs"]
mod put_folder;
#[path = "income/database/post_income.rs"]
mod post_income;

use std::str::FromStr;
use aws_lambda_events::apigw::{ApiGatewayProxyRequest};
use lambda_http::{lambda_runtime,
                  lambda_runtime::Error, service_fn};
use lambda_http::http::{Method};
use lambda_runtime::LambdaEvent;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(start_lambda)).await?;

    Ok(())
}

async fn start_lambda(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<Value, Error> {
    let user_id = Uuid::from_str(event.payload.headers.get("id").unwrap().to_str().unwrap()).unwrap();
    //let user_id = Uuid::from_str("f89de55c-3ce0-4151-b835-b834ccbc32a1").unwrap();
    let path = &event.payload.path.unwrap();
    let http_method = event.payload.http_method;
    let body = event.payload.body.unwrap_or_else(|| String::from("Empty body!"));

    let root_path = String::from("/");
    let folders_path = String::from("/folders");

    println!("Lambda started up with path: {:?}", path);
    println!("Lambda started up with method: {:?}", &http_method);
    println!("Lambda started up with user id: {:?}", &user_id);
    println!("Lambda started up with body: {:?}", &body);

    match path {
        folders_path => match http_method {
            Method::GET => get_folders::get_folders(&user_id).await,
            Method::POST => post_folder::post_folder(&body).await,
            Method::DELETE => delete_folder::delete_folder(&body).await,
            Method::PUT => put_folder::put_folder(&body).await,
            _ => Ok(json!({"statusCode": 400}))
        }
        _ => Ok(json!({"statusCode": 400}))
    }
}
