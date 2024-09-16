use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct FGroups {
    pub client: Client,
}

impl FGroups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "GET Groups\n\nPlease note, the Groups endpoint requires an OAuth application (i.e. \
             approved 3rd party partners), as the end point is intended for mapping third-party \
             application “Groups” within Rippling organizations.\n\nLists the current third-party \
             groups for an organization.\n\n```rust,no_run\nasync fn example_f_groups_get_groups() \
             -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             let result: Vec<rippling_api::types::Group> = \
             client.f_groups().get_groups().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_groups<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::Group>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/groups"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "POST Groups\n\nCreates a generic group, that can be associated within the third-party \
             application.\n\n```rust,no_run\nasync fn example_f_groups_post_groups() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::Group = client\n        .f_groups()\n        \
             .post_groups(&rippling_api::types::PostGroupsRequestBody {\n            name: \
             Some(\"some-string\".to_string()),\n            spoke_id: \
             Some(\"some-string\".to_string()),\n            users: \
             Some(vec![\"some-string\".to_string()]),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_groups<'a>(
        &'a self,
        body: &crate::types::PostGroupsRequestBody,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "platform/api/groups"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "GET Group\n\nPlease note, the Groups endpoint requires an OAuth application (i.e. \
             approved 3rd party partners), as the end point is intended for mapping third-party \
             application “Groups” within Rippling organizations.\n\n**Parameters:**\n\n- \
             `group_id: i64`: Unique identifier for the group within Rippling. \
             (required)\n\n```rust,no_run\nasync fn example_f_groups_get_groups_group_id() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::Group = client\n        .f_groups()\n        \
             .get_groups_group_id(\n            4 as i64,\n            \
             &rippling_api::types::GroupUpdatePayload {\n                name: \
             Some(\"some-string\".to_string()),\n                spoke_id: \
             Some(\"some-string\".to_string()),\n                users: \
             Some(vec![serde_json::Value::String(\"some-string\".to_string())]),\n                \
             version: Some(\"some-string\".to_string()),\n            },\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_groups_group_id<'a>(
        &'a self,
        group_id: i64,
        body: &crate::types::GroupUpdatePayload,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/groups/{groupId}".replace("{groupId}", &format!("{}", group_id))
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "PUT Group\n\nPlease note, the Groups endpoint requires an OAuth application (i.e. approved 3rd party partners), as the end point is intended for mapping third-party application “Groups” within Rippling organizations.\n\nUsing the PUT method, all of the group fields will be updated, even if the corresponding parameter is missing. If the PATCH method is used, and a param is missing, its value won’t be changed.\n\n**Parameters:**\n\n- `group_id: i64`: Unique identifier for the group within Rippling. (required)\n\n```rust,no_run\nasync fn example_f_groups_put_groups_group_id() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::Group = client\n        .f_groups()\n        .put_groups_group_id(\n            4 as i64,\n            &rippling_api::types::GroupUpdatePayload {\n                name: Some(\"some-string\".to_string()),\n                spoke_id: Some(\"some-string\".to_string()),\n                users: Some(vec![serde_json::Value::String(\"some-string\".to_string())]),\n                version: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_groups_group_id<'a>(
        &'a self,
        group_id: i64,
        body: &crate::types::GroupUpdatePayload,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/groups/{groupId}".replace("{groupId}", &format!("{}", group_id))
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "DELETE Group\n\nPlease note, the Groups endpoint requires an OAuth application (i.e. \
             approved 3rd party partners), as the end point is intended for mapping third-party \
             application “Groups” within Rippling organizations.\n\nDeletes the specified \
             group.\n\n**Parameters:**\n\n- `group_id: i64`: Unique identifier for the group \
             within Rippling. (required)\n\n```rust,no_run\nasync fn \
             example_f_groups_delete_groups_group_id() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    client.f_groups().delete_groups_group_id(4 \
             as i64).await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_groups_group_id<'a>(
        &'a self,
        group_id: i64,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/groups/{groupId}".replace("{groupId}", &format!("{}", group_id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "PATCH Group\n\nPlease note, the Groups endpoint requires an OAuth application (i.e. approved 3rd party partners), as the end point is intended for mapping third-party application “Groups” within Rippling organizations.\n\nUsing the PUT method, all of the group fields will be updated, even if the corresponding parameter is missing. If the PATCH method is used, and a param is missing, its value won’t be changed.\n\n**Parameters:**\n\n- `group_id: i64`: Unique identifier for the group within Rippling. (required)\n\n```rust,no_run\nasync fn example_f_groups_patch_groups_group_id() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::Group = client\n        .f_groups()\n        .patch_groups_group_id(\n            4 as i64,\n            &rippling_api::types::GroupUpdatePayload {\n                name: Some(\"some-string\".to_string()),\n                spoke_id: Some(\"some-string\".to_string()),\n                users: Some(vec![serde_json::Value::String(\"some-string\".to_string())]),\n                version: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_groups_group_id<'a>(
        &'a self,
        group_id: i64,
        body: &crate::types::GroupUpdatePayload,
    ) -> Result<crate::types::Group, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/groups/{groupId}".replace("{groupId}", &format!("{}", group_id))
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
