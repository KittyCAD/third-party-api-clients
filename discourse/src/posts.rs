use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Posts {
    pub client: Client,
}

impl Posts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List latest posts across topics\n\n**Parameters:**\n\n- `before: Option<i64>`: Load \
             posts with an id lower than this value. Useful for \
             pagination.\n\n```rust,no_run\nasync fn example_posts_list() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::ListPostsResponse = client.posts().list(Some(4 as \
             i64)).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        before: Option<i64>,
    ) -> Result<crate::types::ListPostsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "posts.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = before {
            query_params.push(("before", format!("{}", p)));
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

    #[doc = "Creates a new topic, a new post, or a private message\n\n```rust,no_run\nasync fn \
             example_posts_create_topic_pm() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::CreateTopicPostPMResponse = client\n        .posts()\n        \
             .create_topic_pm(&discourse_api::types::CreateTopicPostPMRequestBody {\n            \
             title: Some(\"some-string\".to_string()),\n            raw: \
             \"some-string\".to_string(),\n            topic_id: Some(4 as i64),\n            \
             category: Some(4 as i64),\n            target_recipients: \
             Some(\"some-string\".to_string()),\n            target_usernames: \
             Some(\"some-string\".to_string()),\n            archetype: \
             Some(\"some-string\".to_string()),\n            created_at: \
             Some(\"some-string\".to_string()),\n            reply_to_post_number: Some(4 as \
             i64),\n            embed_url: Some(\"some-string\".to_string()),\n            \
             external_id: Some(\"some-string\".to_string()),\n            auto_track: \
             Some(true),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_topic_pm<'a>(
        &'a self,
        body: &crate::types::CreateTopicPostPMRequestBody,
    ) -> Result<crate::types::CreateTopicPostPMResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "posts.json"),
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

    #[doc = "Retrieve a single post\n\nThis endpoint can be used to get the number of likes on a post using the\n`actions_summary` property in the response. `actions_summary` responses\nwith the id of `2` signify a `like`. If there are no `actions_summary`\nitems with the id of `2`, that means there are 0 likes. Other ids likely\nrefer to various different flag types.\n\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_posts_get() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetPostResponse = client.posts().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::GetPostResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "posts/{id}.json".replace("{id}", id)
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

    #[doc = "Update a single post\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_posts_update() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::UpdatePostResponse = client\n        .posts()\n        .update(\n            \"some-string\",\n            &discourse_api::types::UpdatePostRequestBody {\n                post: Some(discourse_api::types::UpdatePostRequestBodyPost {\n                    raw: \"some-string\".to_string(),\n                    edit_reason: Some(\"some-string\".to_string()),\n                }),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::UpdatePostRequestBody,
    ) -> Result<crate::types::UpdatePostResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "posts/{id}.json".replace("{id}", id)
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

    #[doc = "delete a single post\n\n**Parameters:**\n\n- `id: i64` \
             (required)\n\n```rust,no_run\nasync fn example_posts_delete() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    client\n        \
             .posts()\n        .delete(\n            4 as i64,\n            \
             &discourse_api::types::DeletePostRequestBody {\n                force_destroy: \
             Some(true),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        id: i64,
        body: &crate::types::DeletePostRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "posts/{id}.json".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "List replies to a post\n\n**Parameters:**\n\n- `id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_posts_replies() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             Vec<discourse_api::types::PostRepliesResponse> =\n        \
             client.posts().replies(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn replies<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<Vec<crate::types::PostRepliesResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "posts/{id}/replies.json".replace("{id}", id)
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

    #[doc = "Lock a post from being edited\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn example_posts_lock() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::LockPostResponse = client\n        .posts()\n        .lock(\n            \"some-string\",\n            &discourse_api::types::LockPostRequestBody {\n                locked: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn lock<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::LockPostRequestBody,
    ) -> Result<crate::types::LockPostResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "posts/{id}/locked.json".replace("{id}", id)
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

    #[doc = "Like a post and other actions\n\n```rust,no_run\nasync fn \
             example_posts_perform_action() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::PerformPostActionResponse = client\n        .posts()\n        \
             .perform_action(&discourse_api::types::PerformPostActionRequestBody {\n            \
             id: 4 as i64,\n            post_action_type_id: 4 as i64,\n            flag_topic: \
             Some(true),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn perform_action<'a>(
        &'a self,
        body: &crate::types::PerformPostActionRequestBody,
    ) -> Result<crate::types::PerformPostActionResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "post_actions.json"),
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
