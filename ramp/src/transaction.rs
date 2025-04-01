use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Transaction {
    pub client: Client,
}

impl Transaction {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List transactions\n\nThis endpoint supports filtering and ordering. Note that setting multiple ordering parameters is unsupported.\n\n**Parameters:**\n\n- `card_id: Option<uuid::Uuid>`: Filter by physical card.\n- `department_id: Option<uuid::Uuid>`: Filter by department.\n- `entity_id: Option<uuid::Uuid>`: Filter transactions by business entity.\n- `expense_policy_interaction_has_alert: Option<serde_json::Value>`\n- `expense_policy_interaction_needs_review: Option<bool>`: Filter for transactions that require expense policy review.\n- `from_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for transactions that happens after the given date.\n- `has_no_sync_commits: Option<bool>`: Filter for transactions that have not been synced to ERP systems yet.\n- `include_merchant_data: Option<bool>`: Include all purchase data provided by the merchant.\n- `limit_id: Option<uuid::Uuid>`: Filter by limit.\n- `location_id: Option<uuid::Uuid>`: Filter by location.\n- `manager_id: Option<serde_json::Value>`\n- `max_amount: Option<f64>`: Filter for transactions that have smaller amount that the given amount. This is a U.S. Dollar denominated amount.\n- `merchant_id: Option<uuid::Uuid>`: Filter by merchant.\n- `min_amount: Option<f64>`: Filter for transactions that have larger amount that the given amount. This is a U.S. Dollar denominated amount.\n- `order_by_amount_asc: Option<bool>`: Sort transactions by amount in ascending order.\n- `order_by_amount_desc: Option<bool>`: Sort transactions by amount in descending order.\n- `order_by_date_asc: Option<bool>`: Sort transactions by date in ascending order.\n- `order_by_date_desc: Option<bool>`: Sort transactions by date in descending order.\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `requires_memo: Option<bool>`: Filters for transactions which require a memo, but do not have one. This can only be set to true.\n- `sk_category_id: Option<String>`: Filter by a Ramp category code (integer).\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `state: Option<crate::types::GetTransactionsCanonicalListWithPaginationState>`: Filter by transaction state.\n- `statement_id: Option<uuid::Uuid>`: Filter by statement.\n- `sync_ready: Option<bool>`: Filter for transactions that are coded with accounting fields and ready to sync to ERP systems.\n- `synced_after: Option<chrono::DateTime<chrono::Utc>>`: Filter for transactions that have been synced after the given date.\n- `to_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for transactions that happens before the given date.\n- `trip_id: Option<uuid::Uuid>`: Filter for trip ID.\n- `user_id: Option<uuid::Uuid>`: Filter by user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_transaction_get_transactions_canonical_list_with_pagination() -> anyhow::Result<()>\n{\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiTransactionCanonicalSchema = client\n        .transaction()\n        .get_transactions_canonical_list_with_pagination(\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(serde_json::Value::String(\"some-string\".to_string())),\n            Some(true),\n            Some(chrono::Utc::now()),\n            Some(true),\n            Some(true),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(serde_json::Value::String(\"some-string\".to_string())),\n            Some(3.14 as f64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(3.14 as f64),\n            Some(true),\n            Some(true),\n            Some(true),\n            Some(true),\n            Some(4 as i64),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(ramp_api::types::GetTransactionsCanonicalListWithPaginationState::Error),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(true),\n            Some(chrono::Utc::now()),\n            Some(chrono::Utc::now()),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_transactions_canonical_list_with_pagination<'a>(
        &'a self,
        card_id: Option<uuid::Uuid>,
        department_id: Option<uuid::Uuid>,
        entity_id: Option<uuid::Uuid>,
        expense_policy_interaction_has_alert: Option<serde_json::Value>,
        expense_policy_interaction_needs_review: Option<bool>,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        has_no_sync_commits: Option<bool>,
        include_merchant_data: Option<bool>,
        limit_id: Option<uuid::Uuid>,
        location_id: Option<uuid::Uuid>,
        manager_id: Option<serde_json::Value>,
        max_amount: Option<f64>,
        merchant_id: Option<uuid::Uuid>,
        min_amount: Option<f64>,
        order_by_amount_asc: Option<bool>,
        order_by_amount_desc: Option<bool>,
        order_by_date_asc: Option<bool>,
        order_by_date_desc: Option<bool>,
        page_size: Option<i64>,
        requires_memo: Option<bool>,
        sk_category_id: Option<String>,
        start: Option<uuid::Uuid>,
        state: Option<crate::types::GetTransactionsCanonicalListWithPaginationState>,
        statement_id: Option<uuid::Uuid>,
        sync_ready: Option<bool>,
        synced_after: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        trip_id: Option<uuid::Uuid>,
        user_id: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiTransactionCanonicalSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/transactions"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = card_id {
            query_params.push(("card_id", format!("{}", p)));
        }

        if let Some(p) = department_id {
            query_params.push(("department_id", format!("{}", p)));
        }

        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = expense_policy_interaction_has_alert {
            query_params.push(("expense_policy_interaction_has_alert", format!("{}", p)));
        }

        if let Some(p) = expense_policy_interaction_needs_review {
            query_params.push(("expense_policy_interaction_needs_review", format!("{}", p)));
        }

        if let Some(p) = from_date {
            query_params.push(("from_date", format!("{}", p)));
        }

        if let Some(p) = has_no_sync_commits {
            query_params.push(("has_no_sync_commits", format!("{}", p)));
        }

        if let Some(p) = include_merchant_data {
            query_params.push(("include_merchant_data", format!("{}", p)));
        }

        if let Some(p) = limit_id {
            query_params.push(("limit_id", format!("{}", p)));
        }

        if let Some(p) = location_id {
            query_params.push(("location_id", format!("{}", p)));
        }

        if let Some(p) = manager_id {
            query_params.push(("manager_id", format!("{}", p)));
        }

        if let Some(p) = max_amount {
            query_params.push(("max_amount", format!("{}", p)));
        }

        if let Some(p) = merchant_id {
            query_params.push(("merchant_id", format!("{}", p)));
        }

        if let Some(p) = min_amount {
            query_params.push(("min_amount", format!("{}", p)));
        }

        if let Some(p) = order_by_amount_asc {
            query_params.push(("order_by_amount_asc", format!("{}", p)));
        }

        if let Some(p) = order_by_amount_desc {
            query_params.push(("order_by_amount_desc", format!("{}", p)));
        }

        if let Some(p) = order_by_date_asc {
            query_params.push(("order_by_date_asc", format!("{}", p)));
        }

        if let Some(p) = order_by_date_desc {
            query_params.push(("order_by_date_desc", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = requires_memo {
            query_params.push(("requires_memo", format!("{}", p)));
        }

        if let Some(p) = sk_category_id {
            query_params.push(("sk_category_id", p));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
        }

        if let Some(p) = state {
            query_params.push(("state", format!("{}", p)));
        }

        if let Some(p) = statement_id {
            query_params.push(("statement_id", format!("{}", p)));
        }

        if let Some(p) = sync_ready {
            query_params.push(("sync_ready", format!("{}", p)));
        }

        if let Some(p) = synced_after {
            query_params.push(("synced_after", format!("{}", p)));
        }

        if let Some(p) = to_date {
            query_params.push(("to_date", format!("{}", p)));
        }

        if let Some(p) = trip_id {
            query_params.push(("trip_id", format!("{}", p)));
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

    #[doc = "Fetch a transaction\n\n**Parameters:**\n\n- `include_merchant_data: Option<bool>`: \
             Include all purchase data provided by the merchant\n- `transaction_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_transaction_get_canonical_resource() \
             -> anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Transaction = \
             client\n        .transaction()\n        .get_canonical_resource(Some(true), \
             \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_canonical_resource<'a>(
        &'a self,
        include_merchant_data: Option<bool>,
        transaction_id: &'a str,
    ) -> Result<crate::types::Transaction, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/transactions/{transaction_id}"
                    .replace("{transaction_id}", transaction_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = include_merchant_data {
            query_params.push(("include_merchant_data", format!("{}", p)));
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
}
