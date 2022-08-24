mod folders;
mod income;
mod invite;
mod utils;

use aws_lambda_events::apigw::ApiGatewayProxyRequest;
use lambda_http::http::Method;
use lambda_http::{lambda_runtime, lambda_runtime::Error, service_fn};
use lambda_runtime::LambdaEvent;
use serde_json::{json, Value};
use std::str::FromStr;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(service_fn(start_lambda)).await?;

    Ok(())
}

async fn start_lambda(event: LambdaEvent<ApiGatewayProxyRequest>) -> Result<Value, Error> {
    let user_id = Uuid::from_str(event.payload.headers.get("id").unwrap().to_str()?)?;
    //let user_id = Uuid::from_str("f89de55c-3ce0-4151-b835-b834ccbc32a1")?;
    let path = &event.payload.path.unwrap();
    let http_method = event.payload.http_method;
    let body = event
        .payload
        .body
        .unwrap_or_else(|| String::from("Empty body!"));

    let root_path = String::from("/");
    let folders_path = String::from("/folders");
    let incomes_path = String::from("/incomes");
    let invites_path = String::from("/invites");

    println!("Lambda started up with path: {:?}", path);
    println!("Lambda started up with method: {:?}", &http_method);
    println!("Lambda started up with user id: {:?}", &user_id);
    println!("Lambda started up with body: {:?}", &body);

    if !(http_method == Method::GET) && body == "Empty body!" {
        return Ok(json!({"statusCode": 400}));
    }

    if &folders_path == path {
        match http_method {
            Method::GET => folders::database::get_folders::get_folders(&user_id).await,
            Method::POST => folders::database::post_folder::post_folder(&user_id, &body).await,
            Method::DELETE => folders::database::delete_folder::delete_folder(&body).await,
            Method::PUT => folders::database::put_folder::put_folder(&body).await,
            _ => Ok(json!({"statusCode": 400})),
        }
    } else if &incomes_path == path {
        match http_method {
            Method::GET => {
                income::database::get_incomes::get_folders(&Uuid::from_str(
                    event.payload.headers.get("folder_id").unwrap().to_str()?,
                )?)
                .await
            }
            Method::POST => income::database::post_income::post_income(&body).await,
            Method::DELETE => income::database::delete_income::delete_folder(&body).await,
            _ => Ok(json!({"statusCode": 400})),
        }
    } else if &invites_path == path {
        match http_method {
            Method::GET => invite::database::get_invites::get_invites(&user_id).await,
            Method::POST => invite::database::post_invite::post_invite(&body).await,
            Method::PATCH => invite::database::patch_invite::patch_invite(&body).await,
            _ => Ok(json!({"statusCode": 400})),
        }
    } else {
        Ok(json!({"statusCode": 400}))
    }
}
