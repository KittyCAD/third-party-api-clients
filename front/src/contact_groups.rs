use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ContactGroups {
    pub client: Client,
}

impl ContactGroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List groups\n\nList the contact groups.\n\n```rust,no_run\nasync fn \
             example_contact_groups_list_groups() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListGroupsResponse = \
             client.contact_groups().list_groups().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_groups<'a>(
        &'a self,
    ) -> Result<crate::types::ListGroupsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "contact_groups"),
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

    #[doc = "Create group\n\nCreate a new contact group in the default \
             team.\n\n```rust,no_run\nasync fn example_contact_groups_create_group() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .contact_groups()\n        \
             .create_group(&front_api::types::CreateContactGroup {\n            name: \
             \"some-string\".to_string(),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_group<'a>(
        &'a self,
        body: &crate::types::CreateContactGroup,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "contact_groups"),
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

    #[doc = "List team groups\n\nList contact groups belonging to the requested \
             team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID \
             (required)\n\n```rust,no_run\nasync fn example_contact_groups_list_team_groups() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListTeamGroupsResponse = client\n        \
             .contact_groups()\n        .list_team_groups(\"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team_groups<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamGroupsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/contact_groups".replace("{team_id}", team_id)
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

    #[doc = "Create team group\n\nCreate a new contact group for the requested team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_contact_groups_create_team_group() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .contact_groups()\n        .create_team_group(\n            \"some-string\",\n            &front_api::types::CreateContactGroup {\n                name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team_group<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateContactGroup,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/contact_groups".replace("{team_id}", team_id)
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

    #[doc = "List teammate groups\n\nList the contact groups belonging to the requested teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_contact_groups_list_teammate_groups() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListTeammateGroupsResponse = client\n        .contact_groups()\n        .list_teammate_groups(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate_groups<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateGroupsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/contact_groups".replace("{teammate_id}", teammate_id)
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

    #[doc = "Create teammate group\n\nCreate a new contact group for the requested teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_contact_groups_create_teammate_group() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .contact_groups()\n        .create_teammate_group(\n            \"some-string\",\n            &front_api::types::CreateContactGroup {\n                name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_teammate_group<'a>(
        &'a self,
        teammate_id: &'a str,
        body: &crate::types::CreateContactGroup,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/contact_groups".replace("{teammate_id}", teammate_id)
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

    #[doc = "Delete group\n\nDelete a contact group.\n\n**Parameters:**\n\n- `contact_group_id: \
             &'astr`: The contact group ID (required)\n\n```rust,no_run\nasync fn \
             example_contact_groups_delete_group() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    \
             client.contact_groups().delete_group(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_group<'a>(
        &'a self,
        contact_group_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contact_groups/{contact_group_id}"
                    .replace("{contact_group_id}", contact_group_id)
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

    #[doc = "List contacts in group\n\nList the contacts belonging to the requested group.\n\n**Parameters:**\n\n- `contact_group_id: &'astr`: The contact group ID (required)\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n\n```rust,no_run\nasync fn example_contact_groups_list_group_contacts() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListGroupContactsResponse = client\n        .contact_groups()\n        .list_group_contacts(\n            \"some-string\",\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_group_contacts<'a>(
        &'a self,
        contact_group_id: &'a str,
        limit: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListGroupContactsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contact_groups/{contact_group_id}/contacts"
                    .replace("{contact_group_id}", contact_group_id)
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

    #[doc = "Add contacts to group\n\nAdd contacts to the requested group.\n\n**Parameters:**\n\n- `contact_group_id: &'astr`: The contact group ID (required)\n\n```rust,no_run\nasync fn example_contact_groups_add_contacts_to_group() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .contact_groups()\n        .add_contacts_to_group(\n            \"some-string\",\n            &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_contacts_to_group<'a>(
        &'a self,
        contact_group_id: &'a str,
        body: &crate::types::AddContactsToGroup,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contact_groups/{contact_group_id}/contacts"
                    .replace("{contact_group_id}", contact_group_id)
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
