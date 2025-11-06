use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct SpendProgram {
    pub client: Client,
}

impl SpendProgram {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List spend programs\n\n**Parameters:**\n\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_spend_program_get_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiSpendProgramResourceSchema = client\n        .spend_program()\n        .get_resource(\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiSpendProgramResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/spend-programs"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create a spend program\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_spend_program_post_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiSpendProgramResource = client\n        .spend_program()\n        .post_resource(&ramp_api::types::ApiSpendProgramCreateRequestBody {\n            description: \"some-string\".to_string(),\n            display_name: \"some-string\".to_string(),\n            icon: ramp_api::types::ApiSpendProgramCreateRequestBodyIcon::Software,\n            is_shareable: true,\n            issuance_rules: Some(\n                ramp_api::types::ApiSpendProgramCreateIssuanceRulesRequestBody {\n                    automatic: Some(ramp_api::types::ApiSpendProgramIssuanceRulesRequestBody {\n                        applies_to_all: true,\n                        department_ids: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                        location_ids: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                        user_custom_field_ids: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                    }),\n                    requestable: Some(ramp_api::types::ApiSpendProgramIssuanceRulesRequestBody {\n                        applies_to_all: true,\n                        department_ids: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                        location_ids: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                        user_custom_field_ids: Some(vec![uuid::Uuid::from_str(\n                            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                        )?]),\n                    }),\n                },\n            ),\n            issue_physical_card_if_needed: true,\n            permitted_spend_types: ramp_api::types::ApiPermittedSpendTypesRequestBody {\n                primary_card_enabled: true,\n                reimbursements_enabled: true,\n            },\n            spending_restrictions: ramp_api::types::ApiSpendingRestrictionsRequestBody {\n                allowed_categories: Some(vec![4 as i64]),\n                allowed_vendors: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                blocked_categories: Some(vec![4 as i64]),\n                blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                blocked_vendors: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                interval: ramp_api::types::Interval::Tertiary,\n                limit: ramp_api::types::CurrencyAmountRequestBody {\n                    amount: 4 as i64,\n                    currency_code: Some(\"some-string\".to_string()),\n                },\n                lock_date: Some(chrono::Utc::now()),\n                transaction_amount_limit: Some(ramp_api::types::CurrencyAmountRequestBody {\n                    amount: 4 as i64,\n                    currency_code: Some(\"some-string\".to_string()),\n                }),\n            },\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_resource<'a>(
        &'a self,
        body: &crate::types::ApiSpendProgramCreateRequestBody,
    ) -> Result<crate::types::ApiSpendProgramResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/spend-programs"),
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

    #[doc = "Fetch a spend program\n\n**Parameters:**\n\n- `spend_program_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_spend_program_get_single_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::ApiSpendProgramResource = client\n        .spend_program()\n        \
             .get_single_resource(\"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_single_resource<'a>(
        &'a self,
        spend_program_id: &'a str,
    ) -> Result<crate::types::ApiSpendProgramResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/spend-programs/{spend_program_id}"
                    .replace("{spend_program_id}", spend_program_id)
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
