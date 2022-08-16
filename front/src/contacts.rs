use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Contacts {
    pub client: Client,
}

impl Contacts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List contacts\n\nList the contacts of the company.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with the optional properties `updated_after` and `updated_before`, whose value should be a timestamp in seconds with up to 3 decimal places.\n- `sort_by: Option<String>`: Field used to sort the records\n- `sort_order: Option<crate::types::SortOrder>`: Order by which results should be sorted\n\n```rust,no_run\nasync fn example_contacts_list() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListContactsResponse = client\n        .contacts()\n        .list(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(front_api::types::SortOrder::Asc),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<crate::types::SortOrder>,
    ) -> Result<crate::types::ListContactsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "contacts"),
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

        if let Some(p) = sort_by {
            query_params.push(("sort_by", p));
        }

        if let Some(p) = sort_order {
            query_params.push(("sort_order", format!("{}", p)));
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

    #[doc = "Create contact\n\nCreate a new contact.\n\n```rust,no_run\nasync fn example_contacts_create() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ContactResponse = client\n        .contacts()\n        .create(&front_api::types::CreateContact {\n            name: Some(\"some-string\".to_string()),\n            description: Some(\"some-string\".to_string()),\n            avatar: Some(bytes::Bytes::from(\"some-string\")),\n            is_spammer: Some(true),\n            links: Some(vec![\"some-string\".to_string()]),\n            group_names: Some(vec![\"some-string\".to_string()]),\n            custom_fields: Some(std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                \"some-string\".to_string(),\n            )])),\n            handles: Some(vec![front_api::types::ContactHandle {\n                handle: \"some-string\".to_string(),\n                source: front_api::types::Source::Intercom,\n            }]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateContact,
    ) -> Result<crate::types::ContactResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "contacts"),
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

    #[doc = "List team contacts\n\nList the contacts of a team.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with the optional properties `updated_after` and `updated_before`, whose value should be a timestamp in seconds with up to 3 decimal places.\n- `sort_by: Option<String>`: Field used to sort the records\n- `sort_order: Option<crate::types::SortOrder>`: Order by which results should be sorted\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_contacts_list_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListTeamContactsResponse = client\n        .contacts()\n        .list_team(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(front_api::types::SortOrder::Asc),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<crate::types::SortOrder>,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamContactsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/contacts".replace("{team_id}", team_id)
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

        if let Some(p) = sort_by {
            query_params.push(("sort_by", p));
        }

        if let Some(p) = sort_order {
            query_params.push(("sort_order", format!("{}", p)));
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

    #[doc = "Create team contact\n\nCreate a contact for a team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_contacts_create_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ContactResponse = client\n        .contacts()\n        .create_team(\n            \"some-string\",\n            &front_api::types::CreateContact {\n                name: Some(\"some-string\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                avatar: Some(bytes::Bytes::from(\"some-string\")),\n                is_spammer: Some(false),\n                links: Some(vec![\"some-string\".to_string()]),\n                group_names: Some(vec![\"some-string\".to_string()]),\n                custom_fields: Some(std::collections::HashMap::from([(\n                    \"some-key\".to_string(),\n                    \"some-string\".to_string(),\n                )])),\n                handles: Some(vec![front_api::types::ContactHandle {\n                    handle: \"some-string\".to_string(),\n                    source: front_api::types::Source::Custom,\n                }]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateContact,
    ) -> Result<crate::types::ContactResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/contacts".replace("{team_id}", team_id)
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

    #[doc = "List teammate contacts\n\nList the contacts of a teammate.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with the optional properties `updated_after` and `updated_before`, whose value should be a timestamp in seconds with up to 3 decimal places.\n- `sort_by: Option<String>`: Field used to sort the records\n- `sort_order: Option<crate::types::SortOrder>`: Order by which results should be sorted\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_contacts_list_teammate() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListTeammateContactsResponse = client\n        .contacts()\n        .list_teammate(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(front_api::types::SortOrder::Asc),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<crate::types::SortOrder>,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateContactsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/contacts".replace("{teammate_id}", teammate_id)
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

        if let Some(p) = sort_by {
            query_params.push(("sort_by", p));
        }

        if let Some(p) = sort_order {
            query_params.push(("sort_order", format!("{}", p)));
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

    #[doc = "Create teammate contact\n\nCreate a contact for a teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_contacts_create_teammate() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ContactResponse = client\n        .contacts()\n        .create_teammate(\n            \"some-string\",\n            &front_api::types::CreateContact {\n                name: Some(\"some-string\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                avatar: Some(bytes::Bytes::from(\"some-string\")),\n                is_spammer: Some(false),\n                links: Some(vec![\"some-string\".to_string()]),\n                group_names: Some(vec![\"some-string\".to_string()]),\n                custom_fields: Some(std::collections::HashMap::from([(\n                    \"some-key\".to_string(),\n                    \"some-string\".to_string(),\n                )])),\n                handles: Some(vec![front_api::types::ContactHandle {\n                    handle: \"some-string\".to_string(),\n                    source: front_api::types::Source::Twitter,\n                }]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
        body: &crate::types::CreateContact,
    ) -> Result<crate::types::ContactResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/contacts".replace("{teammate_id}", teammate_id)
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

    #[doc = "Get contact\n\nFetch a contact.\n\n**Parameters:**\n\n- `contact_id: &'astr`: The \
             contact ID (required)\n\n```rust,no_run\nasync fn example_contacts_get() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ContactResponse = \
             client.contacts().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        contact_id: &'a str,
    ) -> Result<crate::types::ContactResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}".replace("{contact_id}", contact_id)
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

    #[doc = "Delete a contact\n\nDelete a contact.\n\n**Parameters:**\n\n- `contact_id: &'astr`: \
             The contact ID (required)\n\n```rust,no_run\nasync fn example_contacts_delete() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client.contacts().delete(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        contact_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}".replace("{contact_id}", contact_id)
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

    #[doc = "Update a contact\n\nUpdates a contact.\n\n**Parameters:**\n\n- `contact_id: &'astr`: The contact ID (required)\n\n```rust,no_run\nasync fn example_contacts_update() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .contacts()\n        .update(\n            \"some-string\",\n            &front_api::types::Contact {\n                name: Some(\"some-string\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                avatar: Some(bytes::Bytes::from(\"some-string\")),\n                is_spammer: Some(true),\n                links: Some(vec![\"some-string\".to_string()]),\n                group_names: Some(vec![\"some-string\".to_string()]),\n                custom_fields: Some(std::collections::HashMap::from([(\n                    \"some-key\".to_string(),\n                    \"some-string\".to_string(),\n                )])),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        contact_id: &'a str,
        body: &crate::types::Contact,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}".replace("{contact_id}", contact_id)
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

    #[doc = "Merge contacts\n\nMerges the contacts specified into a single contact, deleting the merged-in contacts.\nIf a target contact ID is supplied, the other contacts will be merged into that one.\nOtherwise, some contact in the contact ID list will be treated as the target contact.\nMerge conflicts will be resolved in the following ways:\n  * name will prioritize manually-updated and non-private contact names\n  * descriptions will be concatenated and separated by newlines in order from\n    oldest to newest with the (optional) target contact's description first\n  * all handles, groups, links, and notes will be preserved\n  * other conflicts will use the most recently updated contact's value\n\n\n```rust,no_run\nasync fn example_contacts_merge() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ContactResponse = client\n        .contacts()\n        .merge(&serde_json::Value::String(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn merge<'a>(
        &'a self,
        body: &crate::types::MergeContacts,
    ) -> Result<crate::types::ContactResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "contacts/merge"),
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

    #[doc = "List contact conversations\n\nList the conversations for a contact in reverse chronological order (newest first). For more advanced filtering, see the [search endpoint](https://dev.frontapp.com/reference/conversations#search-conversations).\n> ⚠\u{fe0f} Deprecated field included\n>\n> This endpoint returns a deprecated `last_message` field in the top-level conversation bodies listed. Please use the\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `contact_id: &'astr`: The Contact ID (required)\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with a property `statuses`, whose value should be a list of conversation statuses (`assigned`, `unassigned`, `archived`, or `deleted`).\n\n```rust,no_run\nasync fn example_contacts_list_conversations() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListContactConversationsResponse = client\n        .contacts()\n        .list_conversations(\n            \"some-string\",\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_conversations<'a>(
        &'a self,
        contact_id: &'a str,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
    ) -> Result<crate::types::ListContactConversationsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}/conversations".replace("{contact_id}", contact_id)
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
