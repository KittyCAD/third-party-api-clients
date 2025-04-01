use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Limit {
    pub client: Client,
}

impl Limit {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List limits\n\n**Parameters:**\n\n- `display_name: Option<String>`: Filter by display name.\n- `entity_id: Option<uuid::Uuid>`: Filter for limits by associated business entity.\n- `is_terminated: Option<bool>`: Filter only for terminated spend limits.\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `spend_program_id: Option<uuid::Uuid>`: Filter for limits that are associated with the specified spend program\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `user_id: Option<uuid::Uuid>`: Filter for limits that are owned by the user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_limit_get_spend_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiSpendLimitResourceSchema = client\n        .limit()\n        .get_spend_list_with_pagination(\n            Some(\"some-string\".to_string()),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(true),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_spend_list_with_pagination<'a>(
        &'a self,
        display_name: Option<String>,
        entity_id: Option<uuid::Uuid>,
        is_terminated: Option<bool>,
        page_size: Option<i64>,
        spend_program_id: Option<uuid::Uuid>,
        start: Option<uuid::Uuid>,
        user_id: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiSpendLimitResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/limits"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = display_name {
            query_params.push(("display_name", p));
        }

        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = is_terminated {
            query_params.push(("is_terminated", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = spend_program_id {
            query_params.push(("spend_program_id", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
        }

        if let Some(p) = user_id {
            query_params.push(("user_id", format!("{}", p)));
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

    #[doc = "Create a limit\n\nLimit may either be created with spend program id (can provide display name and spending restrictions, cannot provide payment restrictions) or without (must provide display name, spending restrictions, and payment restrictions).\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_limit_post_spend_creation() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID = client\n        .limit()\n        .post_spend_creation(&ramp_api::types::ApiSpendLimitCreateRequestBody {\n            display_name: Some(\"some-string\".to_string()),\n            fulfillment: Some(ramp_api::types::CardFulfillmentRequestBody {\n                card_personalization: Some(ramp_api::types::CardPersonalizationRequestBody {\n                    text: Some(ramp_api::types::CardPersonalizationTextRequestBody {\n                        name_line_1: Some(ramp_api::types::CardPersonalizationNameLineRequestBody {\n                            value: Some(\"some-string\".to_string()),\n                        }),\n                        name_line_2: Some(ramp_api::types::CardPersonalizationNameLineRequestBody {\n                            value: Some(\"some-string\".to_string()),\n                        }),\n                    }),\n                }),\n                cardholder_uuid: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                shipping: Some(ramp_api::types::CardShippingRequestBody {\n                    method: Some(\"some-string\".to_string()),\n                    recipient_address: Some(ramp_api::types::CardShippingAddressRequestBody {\n                        address_1: \"some-string\".to_string(),\n                        address_2: Some(\"some-string\".to_string()),\n                        city: \"some-string\".to_string(),\n                        country: \"some-string\".to_string(),\n                        first_name: \"some-string\".to_string(),\n                        last_name: \"some-string\".to_string(),\n                        phone: Some(\"some-string\".to_string()),\n                        postal_code: \"some-string\".to_string(),\n                        state: Some(\"some-string\".to_string()),\n                    }),\n                    recipient_address_verification_state: Some(\n                        ramp_api::types::RecipientAddressVerificationState::Overriden,\n                    ),\n                    return_address: Some(ramp_api::types::CardShippingAddressRequestBody {\n                        address_1: \"some-string\".to_string(),\n                        address_2: Some(\"some-string\".to_string()),\n                        city: \"some-string\".to_string(),\n                        country: \"some-string\".to_string(),\n                        first_name: \"some-string\".to_string(),\n                        last_name: \"some-string\".to_string(),\n                        phone: Some(\"some-string\".to_string()),\n                        postal_code: \"some-string\".to_string(),\n                        state: Some(\"some-string\".to_string()),\n                    }),\n                }),\n            }),\n            idempotency_key: \"some-string\".to_string(),\n            is_shareable: Some(true),\n            permitted_spend_types: Some(ramp_api::types::ApiPermittedSpendTypesRequestBody {\n                primary_card_enabled: true,\n                reimbursements_enabled: true,\n            }),\n            spend_program_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            spending_restrictions: Some(ramp_api::types::ApiSpendingRestrictionsRequestBody {\n                allowed_categories: Some(vec![4 as i64]),\n                allowed_vendors: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                blocked_categories: Some(vec![4 as i64]),\n                blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                blocked_vendors: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                interval: ramp_api::types::Interval::Tertiary,\n                limit: ramp_api::types::CurrencyAmountRequestBody {\n                    amount: 4 as i64,\n                    currency_code: Some(\"some-string\".to_string()),\n                },\n                lock_date: Some(chrono::Utc::now()),\n                transaction_amount_limit: Some(ramp_api::types::CurrencyAmountRequestBody {\n                    amount: 4 as i64,\n                    currency_code: Some(\"some-string\".to_string()),\n                }),\n            }),\n            user_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_spend_creation<'a>(
        &'a self,
        body: &crate::types::ApiSpendLimitCreateRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "developer/v1/limits/deferred"
            ),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Fetch deferred task status\n\n**Parameters:**\n\n- `task_id: uuid::Uuid` \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_limit_get_spend_deferred_task_status() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::SpendLimitDeferredTask = client\n        .limit()\n        \
             .get_spend_deferred_task_status(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_spend_deferred_task_status<'a>(
        &'a self,
        task_id: uuid::Uuid,
    ) -> Result<crate::types::SpendLimitDeferredTask, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/deferred/status/{task_id}"
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Fetch a limit\n\n**Parameters:**\n\n- `spend_limit_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_limit_get_spend_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Limit = \
             client.limit().get_spend_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_spend_resource<'a>(
        &'a self,
        spend_limit_id: &'a str,
    ) -> Result<crate::types::Limit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/{spend_limit_id}".replace("{spend_limit_id}", spend_limit_id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update a limit\n\n**Parameters:**\n\n- `spend_limit_id: &'astr` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_limit_patch_spend_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Limit = client\n        .limit()\n        .patch_spend_resource(\n            \"some-string\",\n            &ramp_api::types::ApiSpendLimitUpdateRequestBody {\n                display_name: Some(\"some-string\".to_string()),\n                is_shareable: Some(true),\n                new_user_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                permitted_spend_types: Some(ramp_api::types::ApiPermittedSpendTypesRequestBody {\n                    primary_card_enabled: true,\n                    reimbursements_enabled: true,\n                }),\n                spend_program_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                spending_restrictions: Some(ramp_api::types::ApiSpendingRestrictionsRequestBody {\n                    allowed_categories: Some(vec![4 as i64]),\n                    allowed_vendors: Some(vec![uuid::Uuid::from_str(\n                        \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                    )?]),\n                    blocked_categories: Some(vec![4 as i64]),\n                    blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                    blocked_vendors: Some(vec![uuid::Uuid::from_str(\n                        \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                    )?]),\n                    interval: ramp_api::types::Interval::Tertiary,\n                    limit: ramp_api::types::CurrencyAmountRequestBody {\n                        amount: 4 as i64,\n                        currency_code: Some(\"some-string\".to_string()),\n                    },\n                    lock_date: Some(chrono::Utc::now()),\n                    transaction_amount_limit: Some(ramp_api::types::CurrencyAmountRequestBody {\n                        amount: 4 as i64,\n                        currency_code: Some(\"some-string\".to_string()),\n                    }),\n                }),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_spend_resource<'a>(
        &'a self,
        spend_limit_id: &'a str,
        body: &crate::types::ApiSpendLimitUpdateRequestBody,
    ) -> Result<crate::types::Limit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/{spend_limit_id}".replace("{spend_limit_id}", spend_limit_id)
            ),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Perform a `PUT` request to `/developer/v1/limits/{spend_limit_id}/add-users`.\n\n**Parameters:**\n\n- `spend_limit_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_limit_put_spend_allocation_add_users() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Limit = client\n        .limit()\n        .put_spend_allocation_add_users(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiSpendLimitModifyUserAccessRequestBody {\n                user_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_spend_allocation_add_users<'a>(
        &'a self,
        spend_limit_id: uuid::Uuid,
        body: &crate::types::ApiSpendLimitModifyUserAccessRequestBody,
    ) -> Result<crate::types::Limit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/{spend_limit_id}/add-users"
                    .replace("{spend_limit_id}", &format!("{}", spend_limit_id))
            ),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Terminate a limit\n\nThis endpoint creates an async task to terminate a limit \
             permanently.\n\n**Parameters:**\n\n- `spend_limit_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn \
             example_limit_post_spend_termination_resource() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID \
             = client\n        .limit()\n        .post_spend_termination_resource(\n            \
             \"some-string\",\n            \
             &ramp_api::types::ApiSpendLimitDeferredUpdateRequestBody {\n                \
             idempotency_key: \"some-string\".to_string(),\n            },\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_spend_termination_resource<'a>(
        &'a self,
        spend_limit_id: &'a str,
        body: &crate::types::ApiSpendLimitDeferredUpdateRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/{spend_limit_id}/deferred/termination"
                    .replace("{spend_limit_id}", spend_limit_id)
            ),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Perform a `DELETE` request to `/developer/v1/limits/{spend_limit_id}/delete-users`.\n\n**Parameters:**\n\n- `spend_limit_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_limit_delete_spend_allocation_delete_users() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Limit = client\n        .limit()\n        .delete_spend_allocation_delete_users(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiSpendLimitModifyUserAccessRequestBody {\n                user_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_spend_allocation_delete_users<'a>(
        &'a self,
        spend_limit_id: uuid::Uuid,
        body: &crate::types::ApiSpendLimitModifyUserAccessRequestBody,
    ) -> Result<crate::types::Limit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/{spend_limit_id}/delete-users"
                    .replace("{spend_limit_id}", &format!("{}", spend_limit_id))
            ),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Suspend a limit\n\n**Parameters:**\n\n- `spend_limit_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_limit_post_spend_suspension_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Limit = client\n        .limit()\n        .post_spend_suspension_resource(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_spend_suspension_resource<'a>(
        &'a self,
        spend_limit_id: &'a str,
    ) -> Result<crate::types::Limit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/{spend_limit_id}/suspension"
                    .replace("{spend_limit_id}", spend_limit_id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Unsuspend a limit\n\n**Parameters:**\n\n- `spend_limit_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_limit_post_spend_unsuspension_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Limit = client\n        .limit()\n        .post_spend_unsuspension_resource(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_spend_unsuspension_resource<'a>(
        &'a self,
        spend_limit_id: &'a str,
    ) -> Result<crate::types::Limit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/limits/{spend_limit_id}/unsuspension"
                    .replace("{spend_limit_id}", spend_limit_id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
