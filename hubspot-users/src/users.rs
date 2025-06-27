use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Retrieves a list of users from an account\n\nRetrieves a list of users from an \
             account\n\n**Parameters:**\n\n- `after: Option<String>`: Results will display maximum \
             100 users per page. Additional results will be on the next page.\n- `limit: \
             Option<i32>`: The number of users to retrieve\n\n```rust,no_run\nasync fn \
             example_users_get_settings_v_3_get_page() -> anyhow::Result<()> {\n    let client = \
             hubspot_users::Client::new_from_env();\n    let result: \
             hubspot_users::types::CollectionResponsePublicUserForwardPaging = client\n        \
             .users()\n        .get_settings_v_3_get_page(Some(\"some-string\".to_string()), \
             Some(4 as i32))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_settings_v_3_get_page<'a>(
        &'a self,
        after: Option<String>,
        limit: Option<i32>,
    ) -> Result<crate::types::CollectionResponsePublicUserForwardPaging, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "settings/v3/users/"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = after {
            query_params.push(("after", p));
        }

        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
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

    #[doc = "Adds a user\n\nNew users will only have minimal permissions, which is contacts-base. A welcome email will prompt them to set a password and log in to HubSpot.\n\n```rust,no_run\nasync fn example_users_post_settings_v_3_create() -> anyhow::Result<()> {\n    let client = hubspot_users::Client::new_from_env();\n    let result: hubspot_users::types::PublicUser = client\n        .users()\n        .post_settings_v_3_create(&hubspot_users::types::UserProvisionRequest {\n            first_name: Some(\"some-string\".to_string()),\n            last_name: Some(\"some-string\".to_string()),\n            primary_team_id: Some(\"some-string\".to_string()),\n            send_welcome_email: Some(true),\n            role_id: Some(\"some-string\".to_string()),\n            secondary_team_ids: Some(vec![\"some-string\".to_string()]),\n            email: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_settings_v_3_create<'a>(
        &'a self,
        body: &crate::types::UserProvisionRequest,
    ) -> Result<crate::types::PublicUser, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "settings/v3/users/"),
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

    #[doc = "Retrieves a user\n\nRetrieves a user identified by `userId`. `userId` refers to the \
             user's ID by default, or optionally email as specified by the `IdProperty` query \
             param.\n\n**Parameters:**\n\n- `id_property: Option<crate::types::IdProperty>`: The \
             name of a property with unique user values. Valid values are `USER_ID`(default) or \
             `EMAIL`\n- `user_id: &'astr`: Identifier of user to retrieve \
             (required)\n\n```rust,no_run\nasync fn example_users_get_settings_v_3_id_get_by_id() \
             -> anyhow::Result<()> {\n    let client = hubspot_users::Client::new_from_env();\n    \
             let result: hubspot_users::types::PublicUser = client\n        .users()\n        \
             .get_settings_v_3_id_get_by_id(Some(hubspot_users::types::IdProperty::Email), \
             \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_settings_v_3_id_get_by_id<'a>(
        &'a self,
        id_property: Option<crate::types::IdProperty>,
        user_id: &'a str,
    ) -> Result<crate::types::PublicUser, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "settings/v3/users/{userId}".replace("{userId}", user_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = id_property {
            query_params.push(("idProperty", format!("{}", p)));
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

    #[doc = "Modifies a user\n\nModifies a user identified by `userId`. `userId` refers to the user's ID by default, or optionally email as specified by the `IdProperty` query param.\n\n**Parameters:**\n\n- `id_property: Option<crate::types::IdProperty>`: The name of a property with unique user values. Valid values are `USER_ID`(default) or `EMAIL`\n- `user_id: &'astr`: Identifier of user to retrieve (required)\n\n```rust,no_run\nasync fn example_users_put_settings_v_3_id_replace() -> anyhow::Result<()> {\n    let client = hubspot_users::Client::new_from_env();\n    let result: hubspot_users::types::PublicUser = client\n        .users()\n        .put_settings_v_3_id_replace(\n            Some(hubspot_users::types::IdProperty::Email),\n            \"some-string\",\n            &hubspot_users::types::PublicUserUpdate {\n                first_name: Some(\"some-string\".to_string()),\n                last_name: Some(\"some-string\".to_string()),\n                primary_team_id: Some(\"some-string\".to_string()),\n                role_id: Some(\"some-string\".to_string()),\n                secondary_team_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_settings_v_3_id_replace<'a>(
        &'a self,
        id_property: Option<crate::types::IdProperty>,
        user_id: &'a str,
        body: &crate::types::PublicUserUpdate,
    ) -> Result<crate::types::PublicUser, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "settings/v3/users/{userId}".replace("{userId}", user_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = id_property {
            query_params.push(("idProperty", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "Removes a user\n\nRemoves a user identified by `userId`. `userId` refers to the \
             user's ID by default, or optionally email as specified by the `IdProperty` query \
             param.\n\n**Parameters:**\n\n- `id_property: Option<crate::types::IdProperty>`: The \
             name of a property with unique user values. Valid values are `USER_ID`(default) or \
             `EMAIL`\n- `user_id: &'astr`: Identifier of user to delete \
             (required)\n\n```rust,no_run\nasync fn example_users_delete_settings_v_3_id_archive() \
             -> anyhow::Result<()> {\n    let client = hubspot_users::Client::new_from_env();\n    \
             client\n        .users()\n        \
             .delete_settings_v_3_id_archive(Some(hubspot_users::types::IdProperty::Email), \
             \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_settings_v_3_id_archive<'a>(
        &'a self,
        id_property: Option<crate::types::IdProperty>,
        user_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "settings/v3/users/{userId}".replace("{userId}", user_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = id_property {
            query_params.push(("idProperty", format!("{}", p)));
        }

        req = req.query(&query_params);
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
