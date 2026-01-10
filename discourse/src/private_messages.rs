use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct PrivateMessages {
    pub client: Client,
}

impl PrivateMessages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a list of private messages for a user\n\n**Parameters:**\n\n- `username: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_private_messages_list_user() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::ListUserPrivateMessagesResponse =\n        \
             client.private_messages().list_user(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_user<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<crate::types::ListUserPrivateMessagesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "topics/private-messages/{username}.json".replace("{username}", username)
            ),
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

    #[doc = "Get a list of private messages sent for a user\n\n**Parameters:**\n\n- `username: \
             &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_private_messages_get_user_sent() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::GetUserSentPrivateMessagesResponse = client\n        \
             .private_messages()\n        .get_user_sent(\"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_sent<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<crate::types::GetUserSentPrivateMessagesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "topics/private-messages-sent/{username}.json".replace("{username}", username)
            ),
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
}
