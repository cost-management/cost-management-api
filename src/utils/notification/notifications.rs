use crate::utils::notification::entities::NotificationType;
use crate::utils::{database_utils, responses};
use fcm::Priority;
use lambda_runtime::Error;
use serde_json::{json, Value};
use sqlx::Connection;
use std::env;
use std::str::FromStr;
use uuid::Uuid;

pub async fn send_invite_notification(owner_email: &str, token: &str) -> Result<Value, Error> {
    println!(
        "Notification function started with email: {} and token: {}",
        owner_email, token
    );

    let client = fcm::Client::new();

    let mut notification_builder = fcm::NotificationBuilder::new();
    let body = json!({"owner":owner_email,"type":NotificationType::INVITE.to_string()}).to_string();

    println!("Notification body: {}", body.as_str());
    notification_builder.title("");
    notification_builder.body(body.as_str());

    println!("Notification was built");

    let api_key = env::var("GOOGLE_API_KEY").unwrap().to_string();
    let notification = notification_builder.finalize();
    let mut message_builder = fcm::MessageBuilder::new(api_key.as_str(), token);
    message_builder.notification(notification);
    message_builder.priority(Priority::High);
    message_builder.content_available(true);

    let response = client.send(message_builder.finalize()).await?;
    println!("Sent: {:?}", response);

    responses::get_ok_response(json!({}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tokio::runtime::Runtime;

    #[test]
    fn test_post_folder() -> Result<(), String> {
        let actual = send_invite_notification("email@gmail.com", "token");
        let expected: Result<Value, ()> = Ok(
            json!({"body":"{}","headers":{"content-type":"application/json"},"isBase64Encoded":false,"statusCode":200}),
        );
        assert_eq!(
            Runtime::new()
                .unwrap()
                .block_on(actual)
                .unwrap()
                .to_string(),
            expected.unwrap().to_string()
        );
        Ok(())
    }
}
