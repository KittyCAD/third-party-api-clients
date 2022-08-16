use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Comments {
    pub client: Client,
}

impl Comments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List conversation comments\n\nList the comments in a conversation in reverse \
             chronological order (newest first).\n\n**Parameters:**\n\n- `conversation_id: \
             &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn \
             example_comments_list_conversation() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListConversationCommentsResponse =\n        \
             client.comments().list_conversation(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_conversation<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> Result<crate::types::ListConversationCommentsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/comments"
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

    #[doc = "Create comment\n\nAdd a comment to a conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn example_comments_create() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::CommentResponse = client\n        .comments()\n        .create(\n            \"some-string\",\n            &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::CreateComment,
    ) -> Result<crate::types::CommentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/comments"
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

    #[doc = "Get comment\n\nFetches a comment.\n\n**Parameters:**\n\n- `comment_id: &'astr`: The \
             Comment ID (required)\n\n```rust,no_run\nasync fn example_comments_get() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::CommentResponse = \
             client.comments().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        comment_id: &'a str,
    ) -> Result<crate::types::CommentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "comments/{comment_id}".replace("{comment_id}", comment_id)
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

    #[doc = "List comment mentions\n\nList the teammates mentioned in a \
             comment.\n\n**Parameters:**\n\n- `comment_id: &'astr`: The Comment ID \
             (required)\n\n```rust,no_run\nasync fn example_comments_list_mentions() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListCommentMentionsResponse =\n        \
             client.comments().list_mentions(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_mentions<'a>(
        &'a self,
        comment_id: &'a str,
    ) -> Result<crate::types::ListCommentMentionsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "comments/{comment_id}/mentions".replace("{comment_id}", comment_id)
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
}
