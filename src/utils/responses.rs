use serde_json::{json, Value};
use lambda_runtime::Error;
use aws_lambda_events::encodings::Body;
use uuid::Uuid;

pub(crate) fn get_ok_response(body: Value) -> Result<Value, Error> {
    let json_body = json!(body).to_string();
    Ok(json!({
                "statusCode": 200,
                "body" : json_body,
                "isBase64Encoded" : false,
                "headers" : {"content-type" : "application/json"},
            }))
}

pub(crate) fn get_ok_response_with_id(id: Uuid) -> Result<Value, Error> {
    let json_body = json!({"id":id.to_string()}).to_string();
    Ok(json!({
                "statusCode": 200,
                "body" : json_body,
                "isBase64Encoded" : false,
                "headers" : {"content-type" : "application/json"},
            }))
}
