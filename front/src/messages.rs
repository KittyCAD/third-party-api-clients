use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Messages {
    pub client: Client,
}

impl Messages {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Reply to conversation\n\nReply to a conversation by sending a message and appending it to the conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn example_messages_reply_to_conversation() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageResponse = client\n        .messages()\n        .reply_to_conversation(\n            \"some-string\",\n            &front_api::types::OutboundReplyMessage {\n                to: Some(vec![\"some-string\".to_string()]),\n                cc: Some(vec![\"some-string\".to_string()]),\n                bcc: Some(vec![\"some-string\".to_string()]),\n                sender_name: Some(\"some-string\".to_string()),\n                subject: Some(\"some-string\".to_string()),\n                author_id: Some(\"some-string\".to_string()),\n                channel_id: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                text: Some(\"some-string\".to_string()),\n                options: Some(front_api::types::OutboundReplyMessageOptions {\n                    tag_ids: Some(vec![\"some-string\".to_string()]),\n                    archive: Some(false),\n                }),\n                attachments: Some(vec![bytes::Bytes::from(\"some-string\")]),\n                signature_id: Some(\"some-string\".to_string()),\n                should_add_default_signature: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn reply_to_conversation<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::OutboundReplyMessage,
    ) -> Result<crate::types::MessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/messages"
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

    #[doc = "Get message\n\nFetch a message.\n\n**Parameters:**\n\n- `message_id: &'astr`: The \
             message ID (required)\n\n```rust,no_run\nasync fn example_messages_get() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::MessageResponse = \
             client.messages().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        message_id: &'a str,
    ) -> Result<crate::types::MessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "messages/{message_id}".replace("{message_id}", message_id)
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

    #[doc = "Get message seen status\n\nGet the seen receipts for the given message. If no seen-by \
             information is available, there will be a single entry for the first time the message \
             was seen by any recipient. If seen-by information is available, there will be an \
             entry for each recipient who has seen the message.\n\n**Parameters:**\n\n- \
             `message_id: &'astr`: The message ID (required)\n\n```rust,no_run\nasync fn \
             example_messages_get_seen_status() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::GetMessageSeenStatusResponse =\n        \
             client.messages().get_seen_status(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_seen_status<'a>(
        &'a self,
        message_id: &'a str,
    ) -> Result<crate::types::GetMessageSeenStatusResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "messages/{message_id}/seen".replace("{message_id}", message_id)
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

    #[doc = "Mark message seen\n\nMark an outbound message from Front as seen. Note, the message \
             seen route should only be called in response to an actual end-user's message-seen \
             action. In accordance with this behavior, the route is rate limited to 10 requests \
             per hour.\n\n**Parameters:**\n\n- `message_id: &'astr`: The message ID \
             (required)\n\n```rust,no_run\nasync fn example_messages_mark_seen() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .messages()\n        .mark_seen(\"some-string\", \
             &front_api::types::MarkMessageSeenRequestBody {})\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn mark_seen<'a>(
        &'a self,
        message_id: &'a str,
        body: &crate::types::MarkMessageSeenRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "messages/{message_id}/seen".replace("{message_id}", message_id)
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

    #[doc = "Create conversation\n\nSend a new message from a channel.\n\n**Parameters:**\n\n- `channel_id: &'astr`: The sending channel ID (required)\n\n```rust,no_run\nasync fn example_messages_create_conversation() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageResponse = client\n        .messages()\n        .create_conversation(\n            \"some-string\",\n            &front_api::types::OutboundMessage {\n                to: vec![\"some-string\".to_string()],\n                cc: Some(vec![\"some-string\".to_string()]),\n                bcc: Some(vec![\"some-string\".to_string()]),\n                sender_name: Some(\"some-string\".to_string()),\n                subject: Some(\"some-string\".to_string()),\n                author_id: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                text: Some(\"some-string\".to_string()),\n                options: Some(front_api::types::Options {\n                    tag_ids: Some(vec![\"some-string\".to_string()]),\n                    archive: Some(true),\n                }),\n                attachments: Some(vec![bytes::Bytes::from(\"some-string\")]),\n                signature_id: Some(\"some-string\".to_string()),\n                should_add_default_signature: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_conversation<'a>(
        &'a self,
        channel_id: &'a str,
        body: &crate::types::OutboundMessage,
    ) -> Result<crate::types::MessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "channels/{channel_id}/messages".replace("{channel_id}", channel_id)
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

    #[doc = "Receive custom messages\n\nReceive a custom message in Front. This endpoint is available for custom channels **ONLY**.\n\n**Parameters:**\n\n- `channel_id: &'astr`: The channel ID (required)\n\n```rust,no_run\nasync fn example_messages_receive_custom() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ReceiveCustomMessageResponse = client\n        .messages()\n        .receive_custom(\n            \"some-string\",\n            &front_api::types::CustomMessage {\n                sender: front_api::types::Sender {\n                    contact_id: Some(\"some-string\".to_string()),\n                    name: Some(\"some-string\".to_string()),\n                    handle: \"some-string\".to_string(),\n                },\n                subject: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                body_format: Some(front_api::types::BodyFormat::Markdown),\n                metadata: Some(front_api::types::Metadata {\n                    thread_ref: Some(\"some-string\".to_string()),\n                    headers: Some(std::collections::HashMap::from([(\n                        \"some-key\".to_string(),\n                        \"some-string\".to_string(),\n                    )])),\n                }),\n                attachments: Some(vec![bytes::Bytes::from(\"some-string\")]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn receive_custom<'a>(
        &'a self,
        channel_id: &'a str,
        body: &crate::types::CustomMessage,
    ) -> Result<crate::types::ReceiveCustomMessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "channels/{channel_id}/incoming_messages".replace("{channel_id}", channel_id)
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

    #[doc = "Import message\n\nImport a new message in an inbox.\n\n**Parameters:**\n\n- `inbox_id: &'astr`: The Inbox ID (required)\n\n```rust,no_run\nasync fn example_messages_import_inbox() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ImportInboxMessageResponse = client\n        .messages()\n        .import_inbox(\n            \"some-string\",\n            &front_api::types::ImportMessage {\n                sender: front_api::types::ImportMessageSender {\n                    author_id: Some(\"some-string\".to_string()),\n                    name: Some(\"some-string\".to_string()),\n                    handle: \"some-string\".to_string(),\n                },\n                to: vec![\"some-string\".to_string()],\n                cc: Some(vec![\"some-string\".to_string()]),\n                bcc: Some(vec![\"some-string\".to_string()]),\n                subject: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                body_format: Some(front_api::types::ImportMessageBodyFormat::Html),\n                external_id: \"some-string\".to_string(),\n                created_at: 4 as i64,\n                type_: Some(front_api::types::ImportMessageType::Email),\n                assignee_id: Some(\"some-string\".to_string()),\n                tags: Some(vec![\"some-string\".to_string()]),\n                metadata: front_api::types::ImportMessageMetadata {\n                    thread_ref: Some(\"some-string\".to_string()),\n                    is_inbound: true,\n                    is_archived: Some(true),\n                    should_skip_rules: Some(false),\n                },\n                attachments: Some(vec![bytes::Bytes::from(\"some-string\")]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn import_inbox<'a>(
        &'a self,
        inbox_id: &'a str,
        body: &crate::types::ImportMessage,
    ) -> Result<crate::types::ImportInboxMessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "inboxes/{inbox_id}/imported_messages".replace("{inbox_id}", inbox_id)
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
}
