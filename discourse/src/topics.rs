use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Topics {
    pub client: Client,
}

impl Topics {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get specific posts from a topic\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_topics_get_specific_posts_from() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetSpecificPostsFromTopicResponse = client\n        .topics()\n        .get_specific_posts_from(\n            \"some-string\",\n            &discourse_api::types::GetSpecificPostsFromTopicRequestBody { post_ids: 4 as i64 },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_specific_posts_from<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::GetSpecificPostsFromTopicRequestBody,
    ) -> Result<crate::types::GetSpecificPostsFromTopicResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/posts.json".replace("{id}", id)
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

    #[doc = "Get a single topic\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_topics_get() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetTopicResponse = client.topics().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::GetTopicResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}.json".replace("{id}", id)
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

    #[doc = "Remove a topic\n\n**Parameters:**\n\n- `id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_topics_remove() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    \
             client.topics().remove(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove<'a>(&'a self, id: &'a str) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}.json".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update a topic\n\n**Parameters:**\n\n- `id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_topics_update() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::UpdateTopicResponse = client\n        .topics()\n        \
             .update(\n            \"some-string\",\n            \
             &discourse_api::types::UpdateTopicRequestBody {\n                topic: \
             Some(discourse_api::types::UpdateTopicRequestBodyTopic {\n                    title: \
             Some(\"some-string\".to_string()),\n                    category_id: Some(4 as \
             i64),\n                }),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::UpdateTopicRequestBody,
    ) -> Result<crate::types::UpdateTopicResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/-/{id}.json".replace("{id}", id)
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

    #[doc = "Invite to topic\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_topics_invite_to() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::InviteToTopicResponse = client\n        .topics()\n        .invite_to(\n            \"some-string\",\n            &discourse_api::types::InviteToTopicRequestBody {\n                user: Some(\"some-string\".to_string()),\n                email: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn invite_to<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::InviteToTopicRequestBody,
    ) -> Result<crate::types::InviteToTopicResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/invite.json".replace("{id}", id)
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

    #[doc = "Invite group to topic\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_topics_invite_group_to() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::InviteGroupToTopicResponse = client\n        .topics()\n        .invite_group_to(\n            \"some-string\",\n            &discourse_api::types::InviteGroupToTopicRequestBody {\n                group: Some(\"some-string\".to_string()),\n                should_notify: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn invite_group_to<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::InviteGroupToTopicRequestBody,
    ) -> Result<crate::types::InviteGroupToTopicResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/invite-group.json".replace("{id}", id)
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

    #[doc = "Bookmark topic\n\n**Parameters:**\n\n- `id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_topics_bookmark() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             client.topics().bookmark(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn bookmark<'a>(&'a self, id: &'a str) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/bookmark.json".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update the status of a topic\n\n**Parameters:**\n\n- `id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_topics_update_status() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::UpdateTopicStatusResponse = client\n        \
             .topics()\n        .update_status(\n            \"some-string\",\n            \
             &discourse_api::types::UpdateTopicStatusRequestBody {\n                status: \
             discourse_api::types::UpdateTopicStatusRequestBodyStatus::Archived,\n                \
             enabled: discourse_api::types::Enabled::False,\n                until: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_status<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::UpdateTopicStatusRequestBody,
    ) -> Result<crate::types::UpdateTopicStatusResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/status.json".replace("{id}", id)
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

    #[doc = "Get the latest topics\n\n**Parameters:**\n\n- `ascending: Option<String>`: Defaults \
             to `desc`, add `ascending=true` to sort asc\n- `order: Option<String>`: Enum: \
             `default`, `created`, `activity`, `views`, `posts`, `category`,\n`likes`, `op_likes`, \
             `posters`\n- `per_page: Option<i64>`: Maximum number of topics returned, between \
             1-100\n\n```rust,no_run\nasync fn example_topics_list_latest() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::ListLatestTopicsResponse = client\n        .topics()\n        \
             .list_latest(\n            Some(\"some-string\".to_string()),\n            \
             Some(\"some-string\".to_string()),\n            Some(4 as i64),\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_latest<'a>(
        &'a self,
        ascending: Option<String>,
        order: Option<String>,
        per_page: Option<i64>,
    ) -> Result<crate::types::ListLatestTopicsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "latest.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = ascending {
            query_params.push(("ascending", p));
        }

        if let Some(p) = order {
            query_params.push(("order", p));
        }

        if let Some(p) = per_page {
            query_params.push(("per_page", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get the top topics filtered by period\n\n**Parameters:**\n\n- `per_page: \
             Option<i64>`: Maximum number of topics returned, between 1-100\n- `period: \
             Option<String>`: Enum: `all`, `yearly`, `quarterly`, `monthly`, `weekly`, \
             `daily`\n\n```rust,no_run\nasync fn example_topics_list_top() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::ListTopTopicsResponse = client\n        .topics()\n        \
             .list_top(Some(4 as i64), Some(\"some-string\".to_string()))\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_top<'a>(
        &'a self,
        per_page: Option<i64>,
        period: Option<String>,
    ) -> Result<crate::types::ListTopTopicsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "top.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = per_page {
            query_params.push(("per_page", format!("{}", p)));
        }

        if let Some(p) = period {
            query_params.push(("period", p));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Set notification level\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_topics_set_notification_level() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::SetNotificationLevelResponse = client\n        .topics()\n        .set_notification_level(\n            \"some-string\",\n            &discourse_api::types::SetNotificationLevelRequestBody {\n                notification_level: discourse_api::types::NotificationLevel::Two,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn set_notification_level<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::SetNotificationLevelRequestBody,
    ) -> Result<crate::types::SetNotificationLevelResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/notifications.json".replace("{id}", id)
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

    #[doc = "Update topic timestamp\n\n**Parameters:**\n\n- `id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_topics_update_timestamp() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::UpdateTopicTimestampResponse = client\n        \
             .topics()\n        .update_timestamp(\n            \"some-string\",\n            \
             &discourse_api::types::UpdateTopicTimestampRequestBody {\n                timestamp: \
             \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_timestamp<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::UpdateTopicTimestampRequestBody,
    ) -> Result<crate::types::UpdateTopicTimestampResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/change-timestamp.json".replace("{id}", id)
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

    #[doc = "Create topic timer\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_topics_create_timer() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateTopicTimerResponse = client\n        .topics()\n        .create_timer(\n            \"some-string\",\n            &discourse_api::types::CreateTopicTimerRequestBody {\n                time: Some(\"some-string\".to_string()),\n                status_type: Some(\"some-string\".to_string()),\n                based_on_last_post: Some(true),\n                category_id: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_timer<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::CreateTopicTimerRequestBody,
    ) -> Result<crate::types::CreateTopicTimerResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/{id}/timer.json".replace("{id}", id)
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

    #[doc = "Get topic by external_id\n\n**Parameters:**\n\n- `external_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_topics_get_by_external_id() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             client.topics().get_by_external_id(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_by_external_id<'a>(
        &'a self,
        external_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "t/external_id/{external_id}.json".replace("{external_id}", external_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
