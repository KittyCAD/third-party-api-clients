use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug, Default)]
pub struct AdminListParams {
    pub asc: Option<crate::types::Asc>,
    pub email: Option<String>,
    pub ip: Option<String>,
    pub order: Option<crate::types::AdminListOrder>,
    pub page: Option<i64>,
    pub show_emails: Option<bool>,
    pub stats: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct AdminListFlagParams {
    pub asc: Option<crate::types::Asc>,
    pub email: Option<String>,
    pub flag: crate::types::Flag,
    pub ip: Option<String>,
    pub order: Option<crate::types::AdminListFlagOrder>,
    pub page: Option<i64>,
    pub show_emails: Option<bool>,
    pub stats: Option<bool>,
}

impl AdminListFlagParams {
    pub fn new(flag: crate::types::Flag) -> Self {
        Self {
            asc: Default::default(),
            email: Default::default(),
            flag,
            ip: Default::default(),
            order: Default::default(),
            page: Default::default(),
            show_emails: Default::default(),
            stats: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Creates a user\n\n```rust,no_run\nasync fn example_users_create() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateUserResponse = client\n        .users()\n        .create(&discourse_api::types::CreateUserRequestBody {\n            name: \"some-string\".to_string(),\n            email: \"some-string\".to_string(),\n            password: \"some-string\".to_string(),\n            username: \"some-string\".to_string(),\n            active: Some(true),\n            approved: Some(true),\n            user_fields: Some(discourse_api::types::UserFields {\n                field_1: Some(true),\n            }),\n            external_ids: Some(discourse_api::types::ExternalIds {}),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateUserRequestBody,
    ) -> Result<crate::types::CreateUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "users.json"),
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

    #[doc = "Get a single user by username\n\n**Parameters:**\n\n- `username: &'astr` (required)\n\n```rust,no_run\nasync fn example_users_get() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetUserResponse = client.users().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<crate::types::GetUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/{username}.json".replace("{username}", username)
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

    #[doc = "Update a user\n\n**Parameters:**\n\n- `username: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_users_update() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::UpdateUserResponse = client\n        .users()\n        \
             .update(\n            \"some-string\",\n            \
             &discourse_api::types::UpdateUserRequestBody {\n                name: \
             Some(\"some-string\".to_string()),\n                external_ids: \
             Some(discourse_api::types::ExternalIds {}),\n            },\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        username: &'a str,
        body: &crate::types::UpdateUserRequestBody,
    ) -> Result<crate::types::UpdateUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/{username}.json".replace("{username}", username)
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

    #[doc = "Get a user by external_id\n\n**Parameters:**\n\n- `external_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_users_get_external_id() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::GetUserExternalIdResponse =\n        \
             client.users().get_external_id(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_external_id<'a>(
        &'a self,
        external_id: &'a str,
    ) -> Result<crate::types::GetUserExternalIdResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/by-external/{external_id}.json".replace("{external_id}", external_id)
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

    #[doc = "Get a user by identity provider external ID\n\n**Parameters:**\n\n- `external_id: \
             &'astr` (required)\n- `provider: &'astr`: Authentication provider name. Can be found \
             in the provider callback\nURL: `/auth/{provider}/callback` \
             (required)\n\n```rust,no_run\nasync fn \
             example_users_get_identiy_provider_external_id() -> anyhow::Result<()> {\n    let \
             client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::GetUserIdentiyProviderExternalIdResponse = client\n        \
             .users()\n        .get_identiy_provider_external_id(\"some-string\", \
             \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_identiy_provider_external_id<'a>(
        &'a self,
        external_id: &'a str,
        provider: &'a str,
    ) -> Result<crate::types::GetUserIdentiyProviderExternalIdResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/by-external/{provider}/{external_id}.json"
                    .replace("{external_id}", external_id)
                    .replace("{provider}", provider)
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

    #[doc = "Update avatar\n\n**Parameters:**\n\n- `username: &'astr` (required)\n\n```rust,no_run\nasync fn example_users_update_avatar() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::UpdateAvatarResponse = client\n        .users()\n        .update_avatar(\n            \"some-string\",\n            &discourse_api::types::UpdateAvatarRequestBody {\n                upload_id: 4 as i64,\n                type_: discourse_api::types::UpdateAvatarRequestBodyType::Gravatar,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_avatar<'a>(
        &'a self,
        username: &'a str,
        body: &crate::types::UpdateAvatarRequestBody,
    ) -> Result<crate::types::UpdateAvatarResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/{username}/preferences/avatar/pick.json".replace("{username}", username)
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

    #[doc = "Update email\n\n**Parameters:**\n\n- `username: &'astr` (required)\n\n```rust,no_run\nasync fn example_users_update_email() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    client\n        .users()\n        .update_email(\n            \"some-string\",\n            &discourse_api::types::UpdateEmailRequestBody {\n                email: \"email@example.com\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_email<'a>(
        &'a self,
        username: &'a str,
        body: &crate::types::UpdateEmailRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/{username}/preferences/email.json".replace("{username}", username)
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

    #[doc = "Update username\n\n**Parameters:**\n\n- `username: &'astr` (required)\n\n```rust,no_run\nasync fn example_users_update_username() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    client\n        .users()\n        .update_username(\n            \"some-string\",\n            &discourse_api::types::UpdateUsernameRequestBody {\n                new_username: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_username<'a>(
        &'a self,
        username: &'a str,
        body: &crate::types::UpdateUsernameRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/{username}/preferences/username.json".replace("{username}", username)
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

    #[doc = "Get a public list of users\n\n**Parameters:**\n\n- `asc: Option<crate::types::Asc>`\n- `order: crate::types::ListPublicOrder` (required)\n- `page: Option<i64>`\n- `period: crate::types::ListPublicPeriod` (required)\n\n```rust,no_run\nasync fn example_users_list_public() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::ListUsersPublicResponse = client\n        .users()\n        .list_public(\n            Some(discourse_api::types::Asc::True),\n            discourse_api::types::ListPublicOrder::TopicsEntered,\n            Some(4 as i64),\n            discourse_api::types::ListPublicPeriod::Quarterly,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_public<'a>(
        &'a self,
        asc: Option<crate::types::Asc>,
        order: crate::types::ListPublicOrder,
        page: Option<i64>,
        period: crate::types::ListPublicPeriod,
    ) -> Result<crate::types::ListUsersPublicResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "directory_items.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![
            ("order", format!("{}", order)),
            ("period", format!("{}", period)),
        ];
        if let Some(p) = asc {
            query_params.push(("asc", format!("{}", p)));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
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

    #[doc = "Get a user by id\n\n**Parameters:**\n\n- `id: i64` \
             (required)\n\n```rust,no_run\nasync fn example_users_admin_get() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::AdminGetUserResponse = client.users().admin_get(4 \
             as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn admin_get<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::AdminGetUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Delete a user\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync fn example_users_delete() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::DeleteUserResponse = client\n        .users()\n        .delete(\n            4 as i64,\n            &discourse_api::types::DeleteUserRequestBody {\n                delete_posts: Some(true),\n                block_email: Some(true),\n                block_urls: Some(true),\n                block_ip: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        id: i64,
        body: &crate::types::DeleteUserRequestBody,
    ) -> Result<crate::types::DeleteUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Activate a user\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync \
             fn example_users_activate() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::ActivateUserResponse = client.users().activate(4 as \
             i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn activate<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::ActivateUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}/activate.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Deactivate a user\n\n**Parameters:**\n\n- `id: i64` \
             (required)\n\n```rust,no_run\nasync fn example_users_deactivate() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::DeactivateUserResponse = \
             client.users().deactivate(4 as i64).await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn deactivate<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::DeactivateUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}/deactivate.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Suspend a user\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync fn example_users_suspend() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::SuspendUserResponse = client\n        .users()\n        .suspend(\n            4 as i64,\n            &discourse_api::types::SuspendUserRequestBody {\n                suspend_until: \"some-string\".to_string(),\n                reason: \"some-string\".to_string(),\n                message: Some(\"some-string\".to_string()),\n                post_action: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn suspend<'a>(
        &'a self,
        id: i64,
        body: &crate::types::SuspendUserRequestBody,
    ) -> Result<crate::types::SuspendUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}/suspend.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Silence a user\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync fn example_users_silence() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::SilenceUserResponse = client\n        .users()\n        .silence(\n            4 as i64,\n            &discourse_api::types::SilenceUserRequestBody {\n                silenced_till: \"some-string\".to_string(),\n                reason: \"some-string\".to_string(),\n                message: Some(\"some-string\".to_string()),\n                post_action: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn silence<'a>(
        &'a self,
        id: i64,
        body: &crate::types::SilenceUserRequestBody,
    ) -> Result<crate::types::SilenceUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}/silence.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Anonymize a user\n\n**Parameters:**\n\n- `id: i64` \
             (required)\n\n```rust,no_run\nasync fn example_users_anonymize() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::AnonymizeUserResponse = client.users().anonymize(4 \
             as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn anonymize<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::AnonymizeUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}/anonymize.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Log a user out\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync fn example_users_log_out() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::LogOutUserResponse = client.users().log_out(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn log_out<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::LogOutUserResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/{id}/log_out.json".replace("{id}", &format!("{}", id))
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

    #[doc = "Refresh gravatar\n\n**Parameters:**\n\n- `username: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_users_refresh_gravatar() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::RefreshGravatarResponse =\n        \
             client.users().refresh_gravatar(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn refresh_gravatar<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<crate::types::RefreshGravatarResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user_avatar/{username}/refresh_gravatar.json".replace("{username}", username)
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

    #[doc = "List users\n\n**Parameters:**\n\n- `asc: Option<crate::types::Asc>`\n- `email: \
             Option<String>`: Filter to the user with this email address\n- `ip: Option<String>`: \
             Filter to users with this IP address\n- `order: \
             Option<crate::types::AdminListOrder>`\n- `page: Option<i64>`\n- `show_emails: \
             Option<bool>`: Include user email addresses in response. These requests will\nbe \
             logged in the staff action logs.\n- `stats: Option<bool>`: Include user stats \
             information\n\n```rust,no_run\nasync fn example_users_admin_list() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: Vec<discourse_api::types::AdminListUsersResponse> = client\n        \
             .users()\n        .admin_list(discourse_api::users::AdminListParams {\n            \
             asc: Some(discourse_api::types::Asc::True),\n            email: \
             Some(\"some-string\".to_string()),\n            ip: \
             Some(\"some-string\".to_string()),\n            order: \
             Some(discourse_api::types::AdminListOrder::DaysVisited),\n            page: Some(4 as \
             i64),\n            show_emails: Some(true),\n            stats: Some(true),\n        \
             })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn admin_list<'a>(
        &'a self,
        params: AdminListParams,
    ) -> Result<Vec<crate::types::AdminListUsersResponse>, crate::types::error::Error> {
        let AdminListParams {
            asc,
            email,
            ip,
            order,
            page,
            show_emails,
            stats,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "admin/users.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = asc {
            query_params.push(("asc", format!("{}", p)));
        }

        if let Some(p) = email {
            query_params.push(("email", p));
        }

        if let Some(p) = ip {
            query_params.push(("ip", p));
        }

        if let Some(p) = order {
            query_params.push(("order", format!("{}", p)));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
        }

        if let Some(p) = show_emails {
            query_params.push(("show_emails", format!("{}", p)));
        }

        if let Some(p) = stats {
            query_params.push(("stats", format!("{}", p)));
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

    #[doc = "List users by flag\n\n**Parameters:**\n\n- `asc: Option<crate::types::Asc>`\n- `email: Option<String>`: Filter to the user with this email address\n- `flag: crate::types::Flag` (required)\n- `ip: Option<String>`: Filter to users with this IP address\n- `order: Option<crate::types::AdminListFlagOrder>`\n- `page: Option<i64>`\n- `show_emails: Option<bool>`: Include user email addresses in response. These requests will\nbe logged in the staff action logs.\n- `stats: Option<bool>`: Include user stats information\n\n```rust,no_run\nasync fn example_users_admin_list_flag() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: Vec<discourse_api::types::AdminListUsersFlagResponse> = client\n        .users()\n        .admin_list_flag(discourse_api::users::AdminListFlagParams {\n            asc: Some(discourse_api::types::Asc::True),\n            email: Some(\"some-string\".to_string()),\n            flag: discourse_api::types::Flag::Suspended,\n            ip: Some(\"some-string\".to_string()),\n            order: Some(discourse_api::types::AdminListFlagOrder::DaysVisited),\n            page: Some(4 as i64),\n            show_emails: Some(true),\n            stats: Some(true),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn admin_list_flag<'a>(
        &'a self,
        params: AdminListFlagParams,
    ) -> Result<Vec<crate::types::AdminListUsersFlagResponse>, crate::types::error::Error> {
        let AdminListFlagParams {
            asc,
            email,
            flag,
            ip,
            order,
            page,
            show_emails,
            stats,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/users/list/{flag}.json".replace("{flag}", &format!("{}", flag))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = asc {
            query_params.push(("asc", format!("{}", p)));
        }

        if let Some(p) = email {
            query_params.push(("email", p));
        }

        if let Some(p) = ip {
            query_params.push(("ip", p));
        }

        if let Some(p) = order {
            query_params.push(("order", format!("{}", p)));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
        }

        if let Some(p) = show_emails {
            query_params.push(("show_emails", format!("{}", p)));
        }

        if let Some(p) = stats {
            query_params.push(("stats", format!("{}", p)));
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

    #[doc = "Get a list of user actions\n\n**Parameters:**\n\n- `filter: &'astr` (required)\n- \
             `offset: i64` (required)\n- `username: &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_users_list_actions() -> anyhow::Result<()> {\n    let client = \
             discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::ListUserActionsResponse = client\n        .users()\n        \
             .list_actions(\"some-string\", 4 as i64, \"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_actions<'a>(
        &'a self,
        filter: &'a str,
        offset: i64,
        username: &'a str,
    ) -> Result<crate::types::ListUserActionsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user_actions.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![
            ("filter", format!("{}", filter)),
            ("offset", format!("{}", offset)),
            ("username", format!("{}", username)),
        ];
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

    #[doc = "Send password reset email\n\n```rust,no_run\nasync fn example_users_send_password_reset_email() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::SendPasswordResetEmailResponse = client\n        .users()\n        .send_password_reset_email(&discourse_api::types::SendPasswordResetEmailRequestBody {\n            login: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn send_password_reset_email<'a>(
        &'a self,
        body: &crate::types::SendPasswordResetEmailRequestBody,
    ) -> Result<crate::types::SendPasswordResetEmailResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "session/forgot_password.json"
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

    #[doc = "Change password\n\n**Parameters:**\n\n- `token: &'astr` (required)\n\n```rust,no_run\nasync fn example_users_change_password() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    client\n        .users()\n        .change_password(\n            \"some-string\",\n            &discourse_api::types::ChangePasswordRequestBody {\n                username: \"some-string\".to_string(),\n                password: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn change_password<'a>(
        &'a self,
        token: &'a str,
        body: &crate::types::ChangePasswordRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/password-reset/{token}.json".replace("{token}", token)
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

    #[doc = "Get email addresses belonging to a user\n\n**Parameters:**\n\n- `username: &'astr` (required)\n\n```rust,no_run\nasync fn example_users_get_emails() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetUserEmailsResponse =\n        client.users().get_emails(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_emails<'a>(
        &'a self,
        username: &'a str,
    ) -> Result<crate::types::GetUserEmailsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "u/{username}/emails.json".replace("{username}", username)
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
}
