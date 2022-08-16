use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Teammates {
    pub client: Client,
}

impl Teammates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List teammates\n\nList the teammates in the company.\n\n```rust,no_run\nasync fn \
             example_teammates_list() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTeammatesResponse = client.teammates().list().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListTeammatesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "teammates"),
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

    #[doc = "Get teammate\n\nFetch a teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The \
             teammate ID (required)\n\n```rust,no_run\nasync fn example_teammates_get() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::TeammateResponse = \
             client.teammates().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::TeammateResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}".replace("{teammate_id}", teammate_id)
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

    #[doc = "Update teammate\n\nUpdate a teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_teammates_update() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .teammates()\n        .update(\n            \"some-string\",\n            &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        teammate_id: &'a str,
        body: &crate::types::UpdateTeammate,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}".replace("{teammate_id}", teammate_id)
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

    #[doc = "List assigned conversations\n\nList the conversations assigned to a teammate in reverse chronological order (most recently updated first). For more advanced filtering, see the [search endpoint](https://dev.frontapp.com/reference/conversations#search-conversations).\n> ⚠\u{fe0f} Deprecated field included\n>\n> This endpoint returns a deprecated `last_message` field in the top-level conversation bodies listed. Please use the\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with a property `statuses`, whose value should be a list of conversation statuses (`assigned`, `unassigned`, `archived`, or `deleted`).\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_teammates_list_assigned_conversations() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListAssignedConversationsResponse = client\n        .teammates()\n        .list_assigned_conversations(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_assigned_conversations<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListAssignedConversationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/conversations".replace("{teammate_id}", teammate_id)
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

    #[doc = "List teammate inboxes\n\nReturns list of inboxes the teammate has access \
             to.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID \
             (required)\n\n**NOTE:** This operation is marked as \
             deprecated.\n\n```rust,no_run\nasync fn example_teammates_list_inboxes() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListTeammateInboxesResponse =\n        \
             client.teammates().list_inboxes(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_inboxes<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateInboxesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/inboxes".replace("{teammate_id}", teammate_id)
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
