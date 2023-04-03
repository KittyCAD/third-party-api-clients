use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CardProgram {
    pub client: Client,
}

impl CardProgram {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Retrieve the card program by ID\n\n**Parameters:**\n\n- `card_program_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_card_program_get_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::ApiCardProgramResource =\n        \
             client.card_program().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        card_program_id: &'a str,
    ) -> Result<crate::types::ApiCardProgramResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/card-programs/{card_program_id}"
                    .replace("{card_program_id}", card_program_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List all the card programs\n\n**Parameters:**\n\n- `page_size: Option<i64>`: The \
             number of results to be returned in each page. The value must be between 2 and \
             10,000. If not specified, the default value 1,000 will be used.\n- `start: \
             Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for \
             pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_card_program_get_list() -> anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::PaginatedResponseApiCardProgramResourceSchema = client\n        \
             .card_program()\n        .get_list(\n            Some(4 as i64),\n            \
             Some(uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list<'a>(
        &'a self,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiCardProgramResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/card-programs/"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create a new card program\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_program_post_list() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiCardProgramResource = client\n        .card_program()\n        .post_list(&ramp_api::types::ApiCardProgramCreate {\n            acting_user_id: 4 as i64,\n            business_id: 4 as i64,\n            is_default: false,\n            is_physical: true,\n            spending_restrictions: ramp_api::types::ApiCardSpendingRestrictionsLoad {\n                categories: Some(vec![4 as i64]),\n                amount: 3.14 as f64,\n                policy_id: Some(\"some-string\".to_string()),\n                vendor_whitelist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                categories_blacklist: Some(vec![4 as i64]),\n                card_accounting_rules: Some(vec![ramp_api::types::ApiCardAccountingRulesData {\n                    accounting_provider_access_uuid: Some(\"some-string\".to_string()),\n                    tracking_category_id: 4 as i64,\n                    tracking_category_option_id: 4 as i64,\n                    tracking_category_option_remote_name: \"some-string\".to_string(),\n                }]),\n                vendor_blacklist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                lock_date: Some(chrono::Utc::now()),\n                categories_whitelist: Some(vec![4 as i64]),\n                transaction_amount_limit: Some(3.14 as f64),\n                interval: ramp_api::types::Interval::Yearly,\n            },\n            policy_id: 4 as i64,\n            display_name: \"some-string\".to_string(),\n            description: \"some-string\".to_string(),\n            icon: Some(ramp_api::types::Icon::SoftwareTrialIcon),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_list<'a>(
        &'a self,
        body: &crate::types::ApiCardProgramCreate,
    ) -> Result<crate::types::ApiCardProgramResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/card-programs/"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
