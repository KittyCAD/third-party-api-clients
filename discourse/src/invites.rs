use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Invites {
    pub client: Client,
}

impl Invites {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Create an invite\n\n```rust,no_run\nasync fn example_invites_create() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateInviteResponse = client\n        .invites()\n        .create(&discourse_api::types::CreateInviteRequestBody {\n            email: Some(\"some-string\".to_string()),\n            skip_email: true,\n            custom_message: Some(\"some-string\".to_string()),\n            max_redemptions_allowed: Some(4 as i64),\n            topic_id: Some(4 as i64),\n            group_ids: Some(\"some-string\".to_string()),\n            group_names: Some(\"some-string\".to_string()),\n            expires_at: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateInviteRequestBody,
    ) -> Result<crate::types::CreateInviteResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "invites.json"),
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

    #[doc = "Create multiple invites\n\n```rust,no_run\nasync fn example_invites_create_multiple() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateMultipleInvitesResponse = client\n        .invites()\n        .create_multiple(&discourse_api::types::CreateMultipleInvitesRequestBody {\n            email: Some(\"some-string\".to_string()),\n            skip_email: true,\n            custom_message: Some(\"some-string\".to_string()),\n            max_redemptions_allowed: Some(4 as i64),\n            topic_id: Some(4 as i64),\n            group_ids: Some(\"some-string\".to_string()),\n            group_names: Some(\"some-string\".to_string()),\n            expires_at: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_multiple<'a>(
        &'a self,
        body: &crate::types::CreateMultipleInvitesRequestBody,
    ) -> Result<crate::types::CreateMultipleInvitesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "invites/create-multiple.json"
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
