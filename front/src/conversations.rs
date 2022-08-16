use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Conversations {
    pub client: Client,
}

impl Conversations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List conversations\n\nList the conversations in the company in reverse chronological order (most recently updated first). For more advanced filtering, see the [search endpoint](https://dev.frontapp.com/reference/conversations#search-conversations).\n> ⚠\u{fe0f} Deprecated field included\n>\n> This endpoint returns a deprecated `last_message` field in the top-level conversation bodies listed. Please use the\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with a property `statuses`, whose value should be a list of conversation statuses (`assigned`, `unassigned`, `archived`, or `deleted`).\n\n```rust,no_run\nasync fn example_conversations_list() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListConversationsResponse = client\n        .conversations()\n        .list(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
    ) -> Result<crate::types::ListConversationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "conversations"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        if let Some(p) = q {
            query_params.push(("q", p));
        }

        req = req.query(&query_params);
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

    #[doc = "Create conversation\n\nCreate a conversation.\n> ⚠\u{fe0f} Currently, only discussions can be created with this endpoint.\n\n\n```rust,no_run\nasync fn example_conversations_create() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ConversationResponse = client\n        .conversations()\n        .create(&serde_json::Value::String(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateConversation,
    ) -> Result<crate::types::ConversationResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "conversations"),
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

    #[doc = "Get conversation\n\nFetch a conversation.\n> ⚠\u{fe0f} Deprecated field \
             included\n>\n> This endpoint returns a deprecated `last_message` field in the main \
             body. Please use the\n> `_links.related.last_message` field \
             instead.\n\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID \
             (required)\n\n```rust,no_run\nasync fn example_conversations_get_by_id() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ConversationResponse =\n        \
             client.conversations().get_by_id(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_by_id<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> Result<crate::types::ConversationResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}".replace("{conversation_id}", conversation_id)
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

    #[doc = "Update conversation\n\nUpdate a conversation.\n\n**Parameters:**\n\n- \
             `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn \
             example_conversations_update() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    client\n        .conversations()\n        \
             .update(\n            \"some-string\",\n            \
             &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::UpdateConversation,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}".replace("{conversation_id}", conversation_id)
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

    #[doc = "Update conversation assignee\n\nAssign or unassign a \
             conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID \
             (required)\n\n```rust,no_run\nasync fn example_conversations_update_assignee() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .conversations()\n        .update_assignee(\n            \
             \"some-string\",\n            \
             &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_assignee<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::UpdateConversationAssignee,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/assignee"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "Add conversation tag\n\nAdds one or more tags to a conversation\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn example_conversations_add_tag() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .conversations()\n        .add_tag(\n            \"some-string\",\n            &front_api::types::TagIds {\n                tag_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_tag<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::TagIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/tags"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "Remove conversation tag\n\nRemoves one or more tags to a \
             conversation\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID \
             (required)\n\n```rust,no_run\nasync fn example_conversations_remove_tag() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .conversations()\n        .remove_tag(\n            \
             \"some-string\",\n            &front_api::types::TagIds {\n                tag_ids: \
             vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove_tag<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::TagIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/tags"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "Add conversation link\n\nAdds one or more links to a conversation\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn example_conversations_add_link() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .conversations()\n        .add_link(\n            \"some-string\",\n            &front_api::types::AddConversationLinkRequestBody {\n                link_ids: Some(vec![\"some-string\".to_string()]),\n                link_external_urls: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_link<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::AddConversationLinkRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/links"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "Remove conversation links\n\nRemoves one or more links to a conversation\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn example_conversations_remove_link() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .conversations()\n        .remove_link(\n            \"some-string\",\n            &front_api::types::RemoveConversationLinkRequestBody {\n                link_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove_link<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::RemoveConversationLinkRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/links"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "List conversation inboxes\n\nList the inboxes in which a conversation is \
             listed.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID \
             (required)\n\n```rust,no_run\nasync fn example_conversations_list_inboxes() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListConversationInboxesResponse =\n        \
             client.conversations().list_inboxes(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_inboxes<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> Result<crate::types::ListConversationInboxesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/inboxes"
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

    #[doc = "List conversation followers\n\nList the teammates following a \
             conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID \
             (required)\n\n```rust,no_run\nasync fn example_conversations_list_followers() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListConversationFollowersResponse =\n        \
             client.conversations().list_followers(\"some-string\").await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_followers<'a>(
        &'a self,
        conversation_id: &'a str,
    ) -> Result<crate::types::ListConversationFollowersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/followers"
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

    #[doc = "Add conversation followers\n\nAdds teammates to the list of followers of a conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn example_conversations_add_followers() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .conversations()\n        .add_followers(\n            \"some-string\",\n            &front_api::types::AddConversationFollowersRequestBody {\n                teammate_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_followers<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::AddConversationFollowersRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/followers"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "Delete conversation followers\n\nRemoves teammates from the list of followers of a \
             conversation.\n\n**Parameters:**\n\n- `conversation_id: &'astr`: The conversation ID \
             (required)\n\n```rust,no_run\nasync fn example_conversations_delete_followers() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .conversations()\n        .delete_followers(\n            \
             \"some-string\",\n            \
             &front_api::types::DeleteConversationFollowersRequestBody {\n                \
             teammate_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_followers<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::DeleteConversationFollowersRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/followers"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "List conversation messages\n\nList the messages in a conversation in reverse \
             chronological order (newest first).\n\n**Parameters:**\n\n- `conversation_id: \
             &'astr`: The conversation ID (required)\n- `limit: Option<i64>`: Max number of \
             results per page\n- `page_token: Option<String>`: Token to use to request the next \
             page\n\n```rust,no_run\nasync fn example_conversations_list_messages() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListConversationMessagesResponse = client\n        \
             .conversations()\n        .list_messages(\n            \"some-string\",\n            \
             Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_messages<'a>(
        &'a self,
        conversation_id: &'a str,
        limit: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListConversationMessagesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/messages"
                    .replace("{conversation_id}", conversation_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        req = req.query(&query_params);
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

    #[doc = "List conversation events\n\nList the events that occured for a conversation in \
             reverse chronological order (newest first).\n\n**Parameters:**\n\n- `conversation_id: \
             &'astr`: The conversation ID (required)\n- `limit: Option<i64>`: Max number of \
             results per page\n- `page_token: Option<String>`: Token to use to request the next \
             page\n\n```rust,no_run\nasync fn example_conversations_list_events() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListConversationEventsResponse = client\n        \
             .conversations()\n        .list_events(\n            \"some-string\",\n            \
             Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_events<'a>(
        &'a self,
        conversation_id: &'a str,
        limit: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListConversationEventsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/events"
                    .replace("{conversation_id}", conversation_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        req = req.query(&query_params);
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

    #[doc = "Update conversation reminders\n\nSnooze or unsnooze a conversation for the provided \
             user.\nFor private conversations, reminders can only be created and edited through \
             the API for teammates that own the conversation.\nFor shared conversations, reminders \
             created and edited through the API are shared for all teammates within the shared \
             inbox(es) that the conversation belongs to.\n\n\n**Parameters:**\n\n- \
             `conversation_id: &'astr`: The conversation ID (required)\n\n```rust,no_run\nasync fn \
             example_conversations_update_reminders() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    client\n        .conversations()\n        \
             .update_reminders(\n            \"some-string\",\n            \
             &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_reminders<'a>(
        &'a self,
        conversation_id: &'a str,
        body: &crate::types::UpdateConversationReminders,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/{conversation_id}/reminders"
                    .replace("{conversation_id}", conversation_id)
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

    #[doc = "Search conversations\n\nSearch for conversations. Response will include a count of total matches and an array of conversations in descending order by last activity.\nSee the [search syntax documentation](https://dev.frontapp.com/docs/search-1) for usage examples.\n> ⚠\u{fe0f} Deprecated field included\n>\n> This endpoint returns a deprecated `last_message` field in the top-level conversation bodies listed. Please use the\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `query: &'astr`: Search query string. See available Conversation Search Parameters below (required)\n\n```rust,no_run\nasync fn example_conversations_search() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::SearchConversationsResponse = client\n        .conversations()\n        .search(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn search<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        query: &'a str,
    ) -> Result<crate::types::SearchConversationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "conversations/search/{query}".replace("{query}", query)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        req = req.query(&query_params);
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
