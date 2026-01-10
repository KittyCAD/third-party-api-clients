use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Groups {
    pub client: Client,
}

impl Groups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Create a group\n\n```rust,no_run\nasync fn example_groups_create() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateGroupResponse = client\n        .groups()\n        .create(&discourse_api::types::CreateGroupRequestBody {\n            group: discourse_api::types::Group {\n                name: \"some-string\".to_string(),\n                full_name: Some(\"some-string\".to_string()),\n                bio_raw: Some(\"some-string\".to_string()),\n                usernames: Some(\"some-string\".to_string()),\n                owner_usernames: Some(\"some-string\".to_string()),\n                automatic_membership_email_domains: Some(\"some-string\".to_string()),\n                visibility_level: Some(4 as i64),\n                primary_group: Some(true),\n                flair_icon: Some(\"some-string\".to_string()),\n                flair_upload_id: Some(4 as i64),\n                flair_bg_color: Some(\"some-string\".to_string()),\n                public_admission: Some(true),\n                public_exit: Some(true),\n                default_notification_level: Some(4 as i64),\n                muted_category_ids: Some(vec![4 as i64]),\n                regular_category_ids: Some(vec![4 as i64]),\n                watching_category_ids: Some(vec![4 as i64]),\n                tracking_category_ids: Some(vec![4 as i64]),\n                watching_first_post_category_ids: Some(vec![4 as i64]),\n            },\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateGroupRequestBody,
    ) -> Result<crate::types::CreateGroupResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "admin/groups.json"),
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

    #[doc = "Delete a group\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync \
             fn example_groups_delete() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::DeleteGroupResponse = client.groups().delete(4 as \
             i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::DeleteGroupResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/groups/{id}.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Get a group\n\n**Parameters:**\n\n- `name: &'astr`: Use group name instead of id (required)\n\n```rust,no_run\nasync fn example_groups_get() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetGroupResponse = client.groups().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<crate::types::GetGroupResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "groups/{name}.json".replace("{name}", name)
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

    #[doc = "Update a group\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync fn example_groups_update() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::UpdateGroupResponse = client\n        .groups()\n        .update(\n            4 as i64,\n            &discourse_api::types::UpdateGroupRequestBody {\n                group: discourse_api::types::Group {\n                    name: \"some-string\".to_string(),\n                    full_name: Some(\"some-string\".to_string()),\n                    bio_raw: Some(\"some-string\".to_string()),\n                    usernames: Some(\"some-string\".to_string()),\n                    owner_usernames: Some(\"some-string\".to_string()),\n                    automatic_membership_email_domains: Some(\"some-string\".to_string()),\n                    visibility_level: Some(4 as i64),\n                    primary_group: Some(true),\n                    flair_icon: Some(\"some-string\".to_string()),\n                    flair_upload_id: Some(4 as i64),\n                    flair_bg_color: Some(\"some-string\".to_string()),\n                    public_admission: Some(true),\n                    public_exit: Some(true),\n                    default_notification_level: Some(4 as i64),\n                    muted_category_ids: Some(vec![4 as i64]),\n                    regular_category_ids: Some(vec![4 as i64]),\n                    watching_category_ids: Some(vec![4 as i64]),\n                    tracking_category_ids: Some(vec![4 as i64]),\n                    watching_first_post_category_ids: Some(vec![4 as i64]),\n                },\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        id: i64,
        body: &crate::types::UpdateGroupRequestBody,
    ) -> Result<crate::types::UpdateGroupResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "groups/{id}.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Get a group by id\n\n**Parameters:**\n\n- `id: &'astr`: Use group name instead of id (required)\n\n```rust,no_run\nasync fn example_groups_get_by_id() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetGroupByIdResponse =\n        client.groups().get_by_id(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_by_id<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::GetGroupByIdResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "groups/by-id/{id}.json".replace("{id}", id)
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

    #[doc = "List group members\n\n**Parameters:**\n\n- `name: &'astr`: Use group name instead of \
             id (required)\n\n```rust,no_run\nasync fn example_groups_list_members() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::ListGroupMembersResponse =\n        \
             client.groups().list_members(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_members<'a>(
        &'a self,
        name: &'a str,
    ) -> Result<crate::types::ListGroupMembersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "groups/{name}/members.json".replace("{name}", name)
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

    #[doc = "Add group members\n\n**Parameters:**\n\n- `id: i64` \
             (required)\n\n```rust,no_run\nasync fn example_groups_add_members() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::AddGroupMembersResponse = client\n        \
             .groups()\n        .add_members(\n            4 as i64,\n            \
             &discourse_api::types::AddGroupMembersRequestBody {\n                usernames: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_members<'a>(
        &'a self,
        id: i64,
        body: &crate::types::AddGroupMembersRequestBody,
    ) -> Result<crate::types::AddGroupMembersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "groups/{id}/members.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Remove group members\n\n**Parameters:**\n\n- `id: i64` \
             (required)\n\n```rust,no_run\nasync fn example_groups_remove_members() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::RemoveGroupMembersResponse = client\n        \
             .groups()\n        .remove_members(\n            4 as i64,\n            \
             &discourse_api::types::RemoveGroupMembersRequestBody {\n                usernames: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove_members<'a>(
        &'a self,
        id: i64,
        body: &crate::types::RemoveGroupMembersRequestBody,
    ) -> Result<crate::types::RemoveGroupMembersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "groups/{id}/members.json".replace("{id}", &format!("{}", id))
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

    #[doc = "List groups\n\n```rust,no_run\nasync fn example_groups_list() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::ListGroupsResponse = client.groups().list().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListGroupsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "groups.json"),
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
}
