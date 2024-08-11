use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct User {
    pub client: Client,
}

impl User {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List users\n\n**Parameters:**\n\n- `department_id: Option<uuid::Uuid>`: filter by department\n- `email: Option<String>`: filter by email\n- `entity_id: Option<uuid::Uuid>`: filter by business entity\n- `location_id: Option<uuid::Uuid>`: filter by location\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `role: Option<crate::types::GetListWithPaginationRole>`: Filter by user role\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_user_get_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiUserResourceSchema = client\n        .user()\n        .get_list_with_pagination(\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(\"email@example.com\".to_string()),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(4 as i64),\n            Some(ramp_api::types::GetListWithPaginationRole::ItAdmin),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        department_id: Option<uuid::Uuid>,
        email: Option<String>,
        entity_id: Option<uuid::Uuid>,
        location_id: Option<uuid::Uuid>,
        page_size: Option<i64>,
        role: Option<crate::types::GetListWithPaginationRole>,
        start: Option<uuid::Uuid>,
    ) -> Result<crate::types::PaginatedResponseApiUserResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "developer/v1/users"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = department_id {
            query_params.push(("department_id", format!("{}", p)));
        }

        if let Some(p) = email {
            query_params.push(("email", p));
        }

        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = location_id {
            query_params.push(("location_id", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = role {
            query_params.push(("role", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create a user invite\n\nCall this endpoint to trigger an async task to send out a user invite.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_user_post_creation_deferred_task() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID = client\n        .user()\n        .post_creation_deferred_task(&ramp_api::types::ApiUserCreateRequestBody {\n            department_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            direct_manager_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            email: \"email@example.com\".to_string(),\n            employee_id: Some(\"some-string\".to_string()),\n            first_name: \"some-string\".to_string(),\n            idempotency_key: Some(\"some-string\".to_string()),\n            last_name: \"some-string\".to_string(),\n            location_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            role: ramp_api::types::Role::ItAdmin,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_creation_deferred_task<'a>(
        &'a self,
        body: &crate::types::ApiUserCreateRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "developer/v1/users/deferred"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Fetch deferred task status\n\n**Parameters:**\n\n- `task_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_user_get_deferred_task_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::UserDeferredTask = client\n        .user()\n        .get_deferred_task_resource(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_deferred_task_resource<'a>(
        &'a self,
        task_id: uuid::Uuid,
    ) -> Result<crate::types::UserDeferredTask, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/users/deferred/status/{task_id}"
                    .replace("{task_id}", &format!("{}", task_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Fetch a user\n\n**Parameters:**\n\n- `user_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_user_get_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::User = client\n        .user()\n        .get_resource(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        user_id: uuid::Uuid,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/users/{user_id}".replace("{user_id}", &format!("{}", user_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Delete a user\n\nThis action terminates the user's cards. This action is not reversible; if you prefer to transfer the user's cards, you must do this before making the DELETE request.\nIf the user you are terminating is currently in an approval chain, then you could choose to either replace the user with his/her direct manager in the approval chain or specify a replacement.  You could also choose to do nothing but it may lead to errors.\n\n**Parameters:**\n\n- `user_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_user_delete_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .user()\n        .delete_resource(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiUserDeleteOptionRequestBody {\n                reassign_approvals_behavior: Some(\n                    ramp_api::types::ReassignApprovalsBehavior::ReplaceWithUser,\n                ),\n                replacement_approver_user_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_resource<'a>(
        &'a self,
        user_id: uuid::Uuid,
        body: &crate::types::ApiUserDeleteOptionRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/users/{user_id}".replace("{user_id}", &format!("{}", user_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        req = req.json(body);
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

    #[doc = "Update a user\n\n**Parameters:**\n\n- `user_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_user_patch_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .user()\n        .patch_resource(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiUserUpdateRequestBody {\n                auto_promote: Some(false),\n                department_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                direct_manager_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                employee_id: Some(\"some-string\".to_string()),\n                first_name: Some(\"some-string\".to_string()),\n                is_manager: Some(false),\n                last_name: Some(\"some-string\".to_string()),\n                location_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                role: Some(ramp_api::types::ApiUserUpdateRequestBodyRole::ItAdmin),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_resource<'a>(
        &'a self,
        user_id: uuid::Uuid,
        body: &crate::types::ApiUserUpdateRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/users/{user_id}".replace("{user_id}", &format!("{}", user_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        req = req.json(body);
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
}
