use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Drafts {
    pub client: Client,
}

impl Drafts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List conversation drafts\n\nList the drafts in a \
             conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID \
             (required)\n\n```rust,no_run\nasync fn example_drafts_list_conversation() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListConversationDraftsResponse =\n        \
             client.drafts().list_conversation(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_conversation<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> Result<crate::types::ListConversationDraftsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/drafts"
                    .replace("{conversation_id}", conversation_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create draft reply\n\nCreate a new draft as a reply to the last message in the conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn example_drafts_create_reply() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageResponse = client\n        .drafts()\n        .create_reply(\n            \"some-string\",\n            &front_api::types::ReplyDraft {\n                author_id: \"some-string\".to_string(),\n                to: Some(vec![\"some-string\".to_string()]),\n                cc: Some(vec![\"some-string\".to_string()]),\n                bcc: Some(vec![\"some-string\".to_string()]),\n                subject: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                attachments: Some(vec![bytes::Bytes::from(\"some-string\")]),\n                mode: Some(front_api::types::Mode::Private),\n                signature_id: Some(\"some-string\".to_string()),\n                should_add_default_signature: Some(true),\n                channel_id: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_reply<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::ReplyDraft,
    ) -> Result<crate::types::MessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/drafts"
                    .replace("{conversation_id}", conversation_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create draft\n\nCreate a draft message which is the first message of a new conversation.\n\n**Parameters:**\n\n- `channel_id: &'astr`: The channel ID (required)\n\n```rust,no_run\nasync fn example_drafts_create() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageResponse = client\n        .drafts()\n        .create(\n            \"some-string\",\n            &front_api::types::CreateDraft {\n                author_id: \"some-string\".to_string(),\n                to: Some(vec![\"some-string\".to_string()]),\n                cc: Some(vec![\"some-string\".to_string()]),\n                bcc: Some(vec![\"some-string\".to_string()]),\n                subject: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                attachments: Some(vec![bytes::Bytes::from(\"some-string\")]),\n                mode: Some(front_api::types::Mode::Shared),\n                signature_id: Some(\"some-string\".to_string()),\n                should_add_default_signature: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        channel_id: &'a str,
        body: &crate::types::CreateDraft,
    ) -> Result<crate::types::MessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "channels/{channel_id}/drafts".replace("{channel_id}", channel_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Edit draft\n\nEdit a draft message.\n\n**Parameters:**\n\n- `message_id: &'astr`: The \
             draft ID (required)\n\n```rust,no_run\nasync fn example_drafts_edit() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::MessageResponse = client\n        .drafts()\n        \
             .edit(\n            \"some-string\",\n            \
             &front_api::types::EditDraft::Shared {\n                version: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn edit<'a>(
        &'a self,
        message_id: &'a str,
        body: &crate::types::EditDraft,
    ) -> Result<crate::types::MessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "drafts/{message_id}/".replace("{message_id}", message_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Delete draft\n\nDelete a draft message.\n\n**Parameters:**\n\n- `draft_id: &'astr`: The draft ID (required)\n\n```rust,no_run\nasync fn example_drafts_delete() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .drafts()\n        .delete(\n            \"some-string\",\n            &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        draft_id: &'a str,
        body: &crate::types::DeleteDraft,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "drafts/{draft_id}".replace("{draft_id}", draft_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
