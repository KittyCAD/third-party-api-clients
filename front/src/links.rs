use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Links {
    pub client: Client,
}

impl Links {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List link conversations\n\nList the conversations linked to a specific link. For more advanced filtering, see the [search endpoint](https://dev.frontapp.com/reference/conversations#search-conversations).\n> ⚠\u{fe0f} Deprecated field included\n>\n> This endpoint returns a deprecated `last_message` field in the top-level conversation bodies listed. Please use the\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `link_id: &'astr`: The Link ID (required)\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with a property `statuses`, whose value should be a list of conversation statuses (`assigned`, `unassigned`, `archived`, or `deleted`).\n\n```rust,no_run\nasync fn example_links_list_conversations() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListLinkConversationsResponse = client\n        .links()\n        .list_conversations(\n            Some(4 as i64),\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_conversations<'a>(
        &'a self,
        limit: Option<i64>,
        link_id: &'a str,
        page_token: Option<String>,
        q: Option<String>,
    ) -> Result<crate::types::ListLinkConversationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "links/{link_id}/conversations".replace("{link_id}", link_id)
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

    #[doc = "List links\n\nList the links of the company paginated by id. Allows filtering by link type via the q.types param.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with a property `types`, whose value should be a list of link types (examples - `web`, `jira`, `asana` ).\n\n```rust,no_run\nasync fn example_links_list() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListLinksResponse = client\n        .links()\n        .list(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
    ) -> Result<crate::types::ListLinksResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "links"),
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

    #[doc = "Create link\n\nCreate a link. If the link is resolved to an installed links \
             integration, any name retrieved from the integration will override the provided \
             name.\n\n```rust,no_run\nasync fn example_links_create() -> anyhow::Result<()> {\n    \
             let client = front_api::Client::new_from_env();\n    let result: \
             front_api::types::LinkResponse = client\n        .links()\n        \
             .create(&front_api::types::CreateLink {\n            name: \
             Some(\"some-string\".to_string()),\n            external_url: \
             \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateLink,
    ) -> Result<crate::types::LinkResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "links"),
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

    #[doc = "Get link\n\nFetch a link.\n\n**Parameters:**\n\n- `link_id: &'astr`: The link ID (required)\n\n```rust,no_run\nasync fn example_links_get() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::LinkResponse = client.links().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        link_id: &'a str,
    ) -> Result<crate::types::LinkResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "links/{link_id}".replace("{link_id}", link_id)
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

    #[doc = "Update a link\n\nUpdate a link.\n\n**Parameters:**\n\n- `link_id: &'astr`: The link \
             ID (required)\n\n```rust,no_run\nasync fn example_links_update() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .links()\n        .update(\n            \"some-string\",\n            \
             &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        link_id: &'a str,
        body: &crate::types::UpdateLink,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "links/{link_id}".replace("{link_id}", link_id)
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
