use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Inboxes {
    pub client: Client,
}

impl Inboxes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List inbox channels\n\nList the channels in an inbox.\n\n**Parameters:**\n\n- \
             `inbox_id: &'astr`: The Inbox ID (required)\n\n```rust,no_run\nasync fn \
             example_inboxes_list_inbox_channels() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListInboxChannelsResponse =\n        \
             client.inboxes().list_inbox_channels(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_inbox_channels<'a>(
        &'a self,
        inbox_id: &'a str,
    ) -> Result<crate::types::ListInboxChannelsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "inboxes/{inbox_id}/channels".replace("{inbox_id}", inbox_id)
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

    #[doc = "List inboxes\n\nList the inboxes of the company.\n\n```rust,no_run\nasync fn \
             example_inboxes_list() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListInboxesResponse = client.inboxes().list().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListInboxesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "inboxes"),
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

    #[doc = "Create inbox\n\nCreate an inbox in the default team.\n\n```rust,no_run\nasync fn \
             example_inboxes_create_inbox() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    client\n        .inboxes()\n        \
             .create_inbox(&front_api::types::CreateInbox {\n            name: \
             \"some-string\".to_string(),\n            teammate_ids: \
             Some(vec![\"some-string\".to_string()]),\n        })\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_inbox<'a>(
        &'a self,
        body: &crate::types::CreateInbox,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "inboxes"),
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

    #[doc = "List team inboxes\n\nList the inboxes belonging to a team.\n\n**Parameters:**\n\n- \
             `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn \
             example_inboxes_list_team() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTeamInboxesResponse =\n        \
             client.inboxes().list_team(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamInboxesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/inboxes".replace("{team_id}", team_id)
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

    #[doc = "Create team inbox\n\nCreate an inbox for a team.\n\n**Parameters:**\n\n- `team_id: \
             &'astr`: The tag ID (required)\n\n```rust,no_run\nasync fn \
             example_inboxes_create_team_inbox() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    client\n        .inboxes()\n        \
             .create_team_inbox(\n            \"some-string\",\n            \
             &front_api::types::CreateInbox {\n                name: \
             \"some-string\".to_string(),\n                teammate_ids: \
             Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team_inbox<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateInbox,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/inboxes".replace("{team_id}", team_id)
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

    #[doc = "Get inbox\n\nFetch an inbox.\n\n**Parameters:**\n\n- `inbox_id: &'astr`: The Inbox ID \
             (required)\n\n```rust,no_run\nasync fn example_inboxes_get_inbox() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::InboxResponse = \
             client.inboxes().get_inbox(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_inbox<'a>(
        &'a self,
        inbox_id: &'a str,
    ) -> Result<crate::types::InboxResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "inboxes/{inbox_id}".replace("{inbox_id}", inbox_id)
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

    #[doc = "List inbox conversations\n\nList the conversations in an inbox. For more advanced filtering, see the [search endpoint](https://dev.frontapp.com/reference/conversations#search-conversations).\n> ⚠\u{fe0f} Deprecated field included\n>\n> This endpoint returns a deprecated `last_message` field in the top-level conversation bodies listed. Please use the\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `inbox_id: &'astr`: The Inbox ID (required)\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with a property `statuses`, whose value should be a list of conversation statuses (`assigned`, `unassigned`, `archived`, or `deleted`).\n\n```rust,no_run\nasync fn example_inboxes_list_inbox_conversations() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListInboxConversationsResponse = client\n        .inboxes()\n        .list_inbox_conversations(\n            \"some-string\",\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_inbox_conversations<'a>(
        &'a self,
        inbox_id: &'a str,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
    ) -> Result<crate::types::ListInboxConversationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "inboxes/{inbox_id}/conversations".replace("{inbox_id}", inbox_id)
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

    #[doc = "List inbox access\n\nList the teammates with access to an \
             inbox.\n\n**Parameters:**\n\n- `inbox_id: &'astr`: The Inbox ID \
             (required)\n\n```rust,no_run\nasync fn example_inboxes_list_inbox_teammates() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListInboxTeammatesResponse =\n        \
             client.inboxes().list_inbox_teammates(\"some-string\").await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_inbox_teammates<'a>(
        &'a self,
        inbox_id: &'a str,
    ) -> Result<crate::types::ListInboxTeammatesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "inboxes/{inbox_id}/teammates".replace("{inbox_id}", inbox_id)
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

    #[doc = "Add inbox access\n\nGive access to one or more teammates to an \
             inbox.\n\n**Parameters:**\n\n- `inbox_id: &'astr`: The Inbox ID \
             (required)\n\n```rust,no_run\nasync fn example_inboxes_add_inbox_teammates() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .inboxes()\n        .add_inbox_teammates(\n            \
             \"some-string\",\n            &front_api::types::TeammateIds {\n                \
             teammate_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_inbox_teammates<'a>(
        &'a self,
        inbox_id: &'a str,
        body: &crate::types::TeammateIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "inboxes/{inbox_id}/teammates".replace("{inbox_id}", inbox_id)
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

    #[doc = "Removes inbox access\n\nRemove access of one or more teammates from an \
             inbox.\n\n**Parameters:**\n\n- `inbox_id: &'astr`: The Inbox ID \
             (required)\n\n```rust,no_run\nasync fn example_inboxes_remove_inbox_teammates() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .inboxes()\n        .remove_inbox_teammates(\n            \
             \"some-string\",\n            &front_api::types::TeammateIds {\n                \
             teammate_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove_inbox_teammates<'a>(
        &'a self,
        inbox_id: &'a str,
        body: &crate::types::TeammateIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "inboxes/{inbox_id}/teammates".replace("{inbox_id}", inbox_id)
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
