use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Card {
    pub client: Client,
}

impl Card {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List cards\n\n**Parameters:**\n\n- `card_program_id: Option<uuid::Uuid>`: Filter by card program.\n- `display_name: Option<String>`: Filter by display name.\n- `entity_id: Option<uuid::Uuid>`: Filter by business entity.\n- `is_activated: Option<bool>`: Filter only for activated cards. Defaults to True if not specified\n- `is_terminated: Option<bool>`: Filter only for terminated cards. Defaults to False if not specified\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `user_id: Option<uuid::Uuid>`: Filter by card owner.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_get_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiCardResourceSchema = client\n        .card()\n        .get_list_with_pagination(\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(\"some-string\".to_string()),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(true),\n            Some(true),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        card_program_id: Option<uuid::Uuid>,
        display_name: Option<String>,
        entity_id: Option<uuid::Uuid>,
        is_activated: Option<bool>,
        is_terminated: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        user_id: Option<uuid::Uuid>,
    ) -> Result<crate::types::PaginatedResponseApiCardResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/cards"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = card_program_id {
            query_params.push(("card_program_id", format!("{}", p)));
        }

        if let Some(p) = display_name {
            query_params.push(("display_name", p));
        }

        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = is_activated {
            query_params.push(("is_activated", format!("{}", p)));
        }

        if let Some(p) = is_terminated {
            query_params.push(("is_terminated", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
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

    #[doc = "Create a physical card\n\nCall this endpoint to create an async task to request for new physical card.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_post_physical() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID = client\n        .card()\n        .post_physical(&ramp_api::types::ApiPhysicalRequestBody {\n            card_program_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            display_name: Some(\"some-string\".to_string()),\n            entity_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            fulfillment: Some(ramp_api::types::CardFulfillmentRequestBody {\n                card_personalization: Some(ramp_api::types::CardPersonalizationRequestBody {\n                    text: Some(ramp_api::types::CardPersonalizationTextRequestBody {\n                        name_line_1: Some(ramp_api::types::CardPersonalizationNameLineRequestBody {\n                            value: Some(\"some-string\".to_string()),\n                        }),\n                        name_line_2: Some(ramp_api::types::CardPersonalizationNameLineRequestBody {\n                            value: Some(\"some-string\".to_string()),\n                        }),\n                    }),\n                }),\n                cardholder_uuid: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                shipping: Some(ramp_api::types::CardShippingRequestBody {\n                    method: Some(\"some-string\".to_string()),\n                    recipient_address: Some(ramp_api::types::CardShippingAddressRequestBody {\n                        address_1: \"some-string\".to_string(),\n                        address_2: Some(\"some-string\".to_string()),\n                        city: \"some-string\".to_string(),\n                        country: \"some-string\".to_string(),\n                        first_name: \"some-string\".to_string(),\n                        last_name: \"some-string\".to_string(),\n                        phone: Some(\"some-string\".to_string()),\n                        postal_code: \"some-string\".to_string(),\n                        state: Some(\"some-string\".to_string()),\n                    }),\n                    recipient_address_verification_state: Some(\n                        ramp_api::types::RecipientAddressVerificationState::Overriden,\n                    ),\n                    return_address: Some(ramp_api::types::CardShippingAddressRequestBody {\n                        address_1: \"some-string\".to_string(),\n                        address_2: Some(\"some-string\".to_string()),\n                        city: \"some-string\".to_string(),\n                        country: \"some-string\".to_string(),\n                        first_name: \"some-string\".to_string(),\n                        last_name: \"some-string\".to_string(),\n                        phone: Some(\"some-string\".to_string()),\n                        postal_code: \"some-string\".to_string(),\n                        state: Some(\"some-string\".to_string()),\n                    }),\n                }),\n            }),\n            idempotency_key: \"some-string\".to_string(),\n            is_physical: true,\n            is_temporary: true,\n            spending_restrictions: Some(ramp_api::types::ApiCardSpendingRestrictionsRequestBody {\n                amount: 3.14 as f64,\n                blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                card_accounting_rules: Some(vec![\n                    ramp_api::types::ApiCardAccountingRulesDataRequestBody {\n                        tracking_category_id: uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?,\n                        tracking_category_option_id: uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?,\n                        tracking_category_option_remote_name: \"some-string\".to_string(),\n                    },\n                ]),\n                categories: Some(vec![4 as i64]),\n                categories_blacklist: Some(vec![4 as i64]),\n                categories_whitelist: Some(vec![4 as i64]),\n                currency: Some(\"some-string\".to_string()),\n                interval: ramp_api::types::Interval::Tertiary,\n                lock_date: Some(chrono::Utc::now()),\n                policy_id: Some(\"some-string\".to_string()),\n                transaction_amount_limit: Some(3.14 as f64),\n                vendor_blacklist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                vendor_whitelist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n            }),\n            user_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_physical<'a>(
        &'a self,
        body: &crate::types::ApiPhysicalRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "developer/v1/cards/deferred/physical"
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

    #[doc = "Fetch deferred task status\n\n**Parameters:**\n\n- `task_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_get_deferred_task_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::CardDeferredTask = client\n        .card()\n        .get_deferred_task_resource(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_deferred_task_resource<'a>(
        &'a self,
        task_id: uuid::Uuid,
    ) -> Result<crate::types::CardDeferredTask, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/cards/deferred/status/{task_id}"
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

    #[doc = "Create a virtual card\n\nCall this endpoint to create an async task to request for new virtual card.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_post_virtual() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID = client\n        .card()\n        .post_virtual(&ramp_api::types::ApiCardRequestBody {\n            card_program_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            display_name: Some(\"some-string\".to_string()),\n            entity_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            idempotency_key: \"some-string\".to_string(),\n            is_physical: true,\n            is_temporary: true,\n            spending_restrictions: Some(ramp_api::types::ApiCardSpendingRestrictionsRequestBody {\n                amount: 3.14 as f64,\n                blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                card_accounting_rules: Some(vec![\n                    ramp_api::types::ApiCardAccountingRulesDataRequestBody {\n                        tracking_category_id: uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?,\n                        tracking_category_option_id: uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?,\n                        tracking_category_option_remote_name: \"some-string\".to_string(),\n                    },\n                ]),\n                categories: Some(vec![4 as i64]),\n                categories_blacklist: Some(vec![4 as i64]),\n                categories_whitelist: Some(vec![4 as i64]),\n                currency: Some(\"some-string\".to_string()),\n                interval: ramp_api::types::Interval::Tertiary,\n                lock_date: Some(chrono::Utc::now()),\n                policy_id: Some(\"some-string\".to_string()),\n                transaction_amount_limit: Some(3.14 as f64),\n                vendor_blacklist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                vendor_whitelist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n            }),\n            user_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_virtual<'a>(
        &'a self,
        body: &crate::types::ApiCardRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "developer/v1/cards/deferred/virtual"
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

    #[doc = "Fetch a card\n\n**Parameters:**\n\n- `card_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_card_get_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Card = \
             client.card().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        card_id: &'a str,
    ) -> Result<crate::types::Card, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/cards/{card_id}".replace("{card_id}", card_id)
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

    #[doc = "Update a card\n\nThis endpoint allow you update the owner, display name, and spend restrictions of a card.\n\n**Parameters:**\n\n- `card_id: &'astr` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_patch_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .card()\n        .patch_resource(\n            \"some-string\",\n            &ramp_api::types::ApiCardUpdateRequestBody {\n                card_program_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                display_name: Some(\"some-string\".to_string()),\n                entity_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                has_notifications_enabled: Some(true),\n                new_user_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                spending_restrictions: Some(\n                    ramp_api::types::PartialApiCardSpendingRestrictionsUpdateRequestBody {\n                        amount: Some(3.14 as f64),\n                        blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                        categories: Some(vec![4 as i64]),\n                        categories_blacklist: Some(vec![4 as i64]),\n                        categories_whitelist: Some(vec![4 as i64]),\n                        currency: Some(\"some-string\".to_string()),\n                        interval: Some(ramp_api::types::Interval::Tertiary),\n                        lock_date: Some(chrono::Utc::now()),\n                        policy_id: Some(\"some-string\".to_string()),\n                        transaction_amount_limit: Some(3.14 as f64),\n                        vendor_blacklist: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                        vendor_whitelist: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                    },\n                ),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_resource<'a>(
        &'a self,
        card_id: &'a str,
        body: &crate::types::ApiCardUpdateRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/cards/{card_id}".replace("{card_id}", card_id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Suspend a card\n\nCall this endpoint to create an async task to suspend a card so that it is locked from use. The suspension is revertable.\n\n**Parameters:**\n\n- `card_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_card_post_suspension_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID = client\n        .card()\n        .post_suspension_resource(\n            \"some-string\",\n            &ramp_api::types::ApiCardDeferredUpdateRequestBody {\n                idempotency_key: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_suspension_resource<'a>(
        &'a self,
        card_id: &'a str,
        body: &crate::types::ApiCardDeferredUpdateRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/cards/{card_id}/deferred/suspension".replace("{card_id}", card_id)
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

    #[doc = "Terminate a card\n\nCall this endpoint to create an async task to terminate a card permanently. Please note that this action is irreversible.\n\n**Parameters:**\n\n- `card_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_card_post_termination_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID = client\n        .card()\n        .post_termination_resource(\n            \"some-string\",\n            &ramp_api::types::ApiCardDeferredUpdateRequestBody {\n                idempotency_key: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_termination_resource<'a>(
        &'a self,
        card_id: &'a str,
        body: &crate::types::ApiCardDeferredUpdateRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/cards/{card_id}/deferred/termination".replace("{card_id}", card_id)
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

    #[doc = "Unlock a card\n\nCall this endpoint to create an async task to remove a card's suspension so that it may be used again.\n\n**Parameters:**\n\n- `card_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_card_post_unsuspension_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::DeferredTaskUUID = client\n        .card()\n        .post_unsuspension_resource(\n            \"some-string\",\n            &ramp_api::types::ApiCardDeferredUpdateRequestBody {\n                idempotency_key: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_unsuspension_resource<'a>(
        &'a self,
        card_id: &'a str,
        body: &crate::types::ApiCardDeferredUpdateRequestBody,
    ) -> Result<crate::types::DeferredTaskUUID, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/cards/{card_id}/deferred/unsuspension".replace("{card_id}", card_id)
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
}
