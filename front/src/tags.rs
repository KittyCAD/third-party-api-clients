use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Tags {
    pub client: Client,
}

impl Tags {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List tags\n\nList the tags of the company.\n\n```rust,no_run\nasync fn \
             example_tags_list() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTagsResponse = client.tags().list().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListTagsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "tags"),
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

    #[doc = "Create tag\n\nCreate a tag.\n\n```rust,no_run\nasync fn example_tags_create() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::TagResponse = client\n        .tags()\n        \
             .create(&front_api::types::CreateTag {\n            name: \
             \"some-string\".to_string(),\n            highlight: \
             Some(front_api::types::Highlight::Orange),\n            \
             is_visible_in_conversation_lists: Some(false),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateTag,
    ) -> Result<crate::types::TagResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "tags"),
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

    #[doc = "List team tags\n\nList the tags for a team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_tags_list_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListTeamTagsResponse = client.tags().list_team(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamTagsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/tags".replace("{team_id}", team_id)
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

    #[doc = "Create team tag\n\nCreate a tag for a team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_tags_create_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::TagResponse = client\n        .tags()\n        .create_team(\n            \"some-string\",\n            &front_api::types::CreateTag {\n                name: \"some-string\".to_string(),\n                highlight: Some(front_api::types::Highlight::LightBlue),\n                is_visible_in_conversation_lists: Some(false),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateTag,
    ) -> Result<crate::types::TagResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/tags".replace("{team_id}", team_id)
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

    #[doc = "List teammate tags\n\nList the tags for a teammate.\n\n**Parameters:**\n\n- \
             `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn \
             example_tags_list_teammate() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTeammateTagsResponse =\n        \
             client.tags().list_teammate(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateTagsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/tags".replace("{teammate_id}", teammate_id)
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

    #[doc = "Create teammate tag\n\nCreate a tag for a teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_tags_create_teammate() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::TagResponse = client\n        .tags()\n        .create_teammate(\n            \"some-string\",\n            &front_api::types::CreateTag {\n                name: \"some-string\".to_string(),\n                highlight: Some(front_api::types::Highlight::Orange),\n                is_visible_in_conversation_lists: Some(false),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
        body: &crate::types::CreateTag,
    ) -> Result<crate::types::TagResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/tags".replace("{teammate_id}", teammate_id)
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

    #[doc = "List tag children\n\nList the children of a specific tag.\n\n**Parameters:**\n\n- \
             `tag_id: &'astr`: The tag ID (required)\n\n```rust,no_run\nasync fn \
             example_tags_list_children() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTagChildrenResponse =\n        \
             client.tags().list_children(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_children<'a>(
        &'a self,
        tag_id: &'a str,
    ) -> Result<crate::types::ListTagChildrenResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "tags/{tag_id}/children".replace("{tag_id}", tag_id)
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

    #[doc = "Create child tag\n\nCreates a child tag.\n\n**Parameters:**\n\n- `tag_id: &'astr`: The tag ID (required)\n\n```rust,no_run\nasync fn example_tags_create_child() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::TagResponse = client\n        .tags()\n        .create_child(\n            \"some-string\",\n            &front_api::types::CreateTag {\n                name: \"some-string\".to_string(),\n                highlight: Some(front_api::types::Highlight::Grey),\n                is_visible_in_conversation_lists: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_child<'a>(
        &'a self,
        tag_id: &'a str,
        body: &crate::types::CreateTag,
    ) -> Result<crate::types::TagResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "tags/{tag_id}/children".replace("{tag_id}", tag_id)
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

    #[doc = "Get tag\n\nFetche a tag.\n\n**Parameters:**\n\n- `tag_id: &'astr`: The tag ID (required)\n\n```rust,no_run\nasync fn example_tags_get() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::TagResponse = client.tags().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        tag_id: &'a str,
    ) -> Result<crate::types::TagResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "tags/{tag_id}".replace("{tag_id}", tag_id)
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

    #[doc = "Delete tag\n\nDelete a tag.\n\n**Parameters:**\n\n- `tag_id: &'astr`: The ID of the \
             tag to delete (required)\n\n```rust,no_run\nasync fn example_tags_delete() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client.tags().delete(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(&'a self, tag_id: &'a str) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "tags/{tag_id}".replace("{tag_id}", tag_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Update a tag\n\nUpdate a tag.\n\n**Parameters:**\n\n- `tag_id: &'astr`: The tag ID (required)\n\n```rust,no_run\nasync fn example_tags_update() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .tags()\n        .update(\n            \"some-string\",\n            &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        tag_id: &'a str,
        body: &crate::types::UpdateTag,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "tags/{tag_id}".replace("{tag_id}", tag_id)
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

    #[doc = "List tagged conversations\n\nList the conversations tagged with a tag. For more advanced filtering, see the [search endpoint](https://dev.frontapp.com/reference/conversations#search-conversations).\n> ⚠\u{fe0f} Deprecated field included\n>\n> This endpoint returns a deprecated `last_message` field in the top-level conversation bodies listed. Please use the\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with a property `statuses`, whose value should be a list of conversation statuses (`assigned`, `unassigned`, `archived`, or `deleted`).\n- `tag_id: &'astr`: The ID of the tag (required)\n\n```rust,no_run\nasync fn example_tags_list_tagged_conversations() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListTaggedConversationsResponse = client\n        .tags()\n        .list_tagged_conversations(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_tagged_conversations<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
        tag_id: &'a str,
    ) -> Result<crate::types::ListTaggedConversationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "tags/{tag_id}/conversations".replace("{tag_id}", tag_id)
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
}
