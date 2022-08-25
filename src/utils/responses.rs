use lambda_runtime::Error;
use serde_json::{json, Value};
use uuid::Uuid;

pub fn get_ok_response(body: Value) -> Result<Value, Error> {
    let json_body = json!(body).to_string();
    Ok(json!({
        "statusCode": 200,
        "body" : json_body,
        "isBase64Encoded" : false,
        "headers" : {"content-type" : "application/json"},
    }))
}

pub fn get_ok_response_with_id(id: Uuid) -> Result<Value, Error> {
    let json_body = json!({"id":id.to_string()}).to_string();
    Ok(json!({
        "statusCode": 200,
        "body" : json_body,
        "isBase64Encoded" : false,
        "headers" : {"content-type" : "application/json"},
    }))
}

pub fn get_fail_response(status_code: i16, message: &str) -> Result<Value, Error> {
    let json_body = json!({"message":message.to_string()}).to_string();
    Ok(json!({
        "statusCode": status_code,
        "body" : json_body,
        "isBase64Encoded" : false,
        "headers" : {"content-type" : "application/json"},
    }))
}

pub fn get_fail_query_response() -> Result<Value, Error> {
    get_fail_response(500, "Database call was failed")
}

pub fn get_fail_uuid_response() -> Result<Value, Error> {
    get_fail_response(400, "Something is not ok with uuid")
}

pub fn get_id_is_absent_response() -> Result<Value, Error> {
    get_fail_response(400, "Id is absent")
}

pub fn get_fail_on_deserialization_response() -> Result<Value, Error> {
    get_fail_response(404, "Deserialization was failed")
}

pub fn get_fail_on_commit_transaction_response() -> Result<Value, Error> {
    get_fail_response(500, "Commit transaction was failed")
}

pub fn get_fail_on_create_transaction() -> Result<Value, Error> {
    get_fail_response(500, "Creation of transaction was failed")
}
