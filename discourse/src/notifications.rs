use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Notifications {
    pub client: Client,
}

impl Notifications {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get the notifications that belong to the current user\n\n```rust,no_run\nasync fn \
             example_notifications_get() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::GetNotificationsResponse = \
             client.notifications().get().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
    ) -> Result<crate::types::GetNotificationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "notifications.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Mark notifications as read\n\n```rust,no_run\nasync fn \
             example_notifications_mark_as_read() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::MarkNotificationsAsReadResponse = client\n        \
             .notifications()\n        \
             .mark_as_read(&discourse_api::types::MarkNotificationsAsReadRequestBody { id: Some(4 \
             as i64) })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn mark_as_read<'a>(
        &'a self,
        body: &crate::types::MarkNotificationsAsReadRequestBody,
    ) -> Result<crate::types::MarkNotificationsAsReadResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url, "notifications/mark-read.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
