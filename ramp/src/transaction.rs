use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug, Default)]
pub struct GetTransactionsCanonicalListWithPaginationParams {
    pub card_id: Option<uuid::Uuid>,
    pub department_id: Option<uuid::Uuid>,
    pub entity_id: Option<uuid::Uuid>,
    pub expense_policy_interaction_has_alert: Option<serde_json::Value>,
    pub expense_policy_interaction_needs_review: Option<bool>,
    pub from_date: Option<chrono::DateTime<chrono::Utc>>,
    pub has_no_sync_commits: Option<bool>,
    pub include_merchant_data: Option<bool>,
    pub limit_id: Option<uuid::Uuid>,
    pub location_id: Option<uuid::Uuid>,
    pub manager_id: Option<serde_json::Value>,
    pub max_amount: Option<f64>,
    pub merchant_id: Option<uuid::Uuid>,
    pub min_amount: Option<f64>,
    pub order_by_amount_asc: Option<bool>,
    pub order_by_amount_desc: Option<bool>,
    pub order_by_date_asc: Option<bool>,
    pub order_by_date_desc: Option<bool>,
    pub page_size: Option<i64>,
    pub requires_memo: Option<bool>,
    pub sk_category_id: Option<String>,
    pub start: Option<uuid::Uuid>,
    pub state: Option<crate::types::GetTransactionsCanonicalListWithPaginationState>,
    pub statement_id: Option<uuid::Uuid>,
    pub sync_ready: Option<bool>,
    pub synced_after: Option<chrono::DateTime<chrono::Utc>>,
    pub to_date: Option<chrono::DateTime<chrono::Utc>>,
    pub trip_id: Option<uuid::Uuid>,
    pub user_id: Option<uuid::Uuid>,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub client: Client,
}

impl Transaction {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List transactions\n\nThis endpoint supports filtering and ordering. Note that setting multiple ordering parameters is unsupported.\n\n**Parameters:**\n\n- `card_id: Option<uuid::Uuid>`: Filter by physical card.\n- `department_id: Option<uuid::Uuid>`: Filter by department.\n- `entity_id: Option<uuid::Uuid>`: Filter transactions by business entity.\n- `expense_policy_interaction_has_alert: Option<serde_json::Value>`\n- `expense_policy_interaction_needs_review: Option<bool>`: Filter for transactions that require expense policy review.\n- `from_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for transactions that happens after the given date.\n- `has_no_sync_commits: Option<bool>`: Filter for transactions that have not been synced to ERP systems yet.\n- `include_merchant_data: Option<bool>`: Include all purchase data provided by the merchant.\n- `limit_id: Option<uuid::Uuid>`: Filter by limit.\n- `location_id: Option<uuid::Uuid>`: Filter by location.\n- `manager_id: Option<serde_json::Value>`\n- `max_amount: Option<f64>`: Filter for transactions that have smaller amount that the given amount. This is a U.S. Dollar denominated amount.\n- `merchant_id: Option<uuid::Uuid>`: Filter by merchant.\n- `min_amount: Option<f64>`: Filter for transactions that have larger amount that the given amount. This is a U.S. Dollar denominated amount.\n- `order_by_amount_asc: Option<bool>`: Sort transactions by amount in ascending order.\n- `order_by_amount_desc: Option<bool>`: Sort transactions by amount in descending order.\n- `order_by_date_asc: Option<bool>`: Sort transactions by date in ascending order.\n- `order_by_date_desc: Option<bool>`: Sort transactions by date in descending order.\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `requires_memo: Option<bool>`: Filters for transactions which require a memo, but do not have one. This can only be set to true.\n- `sk_category_id: Option<String>`: Filter by a Ramp category code (integer).\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `state: Option<crate::types::GetTransactionsCanonicalListWithPaginationState>`: Filter by transaction state.\n- `statement_id: Option<uuid::Uuid>`: Filter by statement.\n- `sync_ready: Option<bool>`: Filter for transactions that are coded with accounting fields and ready to sync to ERP systems.\n- `synced_after: Option<chrono::DateTime<chrono::Utc>>`: Filter for transactions that have been synced after the given date.\n- `to_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for transactions that happens before the given date.\n- `trip_id: Option<uuid::Uuid>`: Filter for trip ID.\n- `user_id: Option<uuid::Uuid>`: Filter by user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_transaction_get_transactions_canonical_list_with_pagination() -> anyhow::Result<()>\n{\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiTransactionCanonicalSchema = client\n        .transaction()\n        .get_transactions_canonical_list_with_pagination(\n            ramp_api::transaction::GetTransactionsCanonicalListWithPaginationParams {\n                card_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                department_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                entity_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                expense_policy_interaction_has_alert: Some(serde_json::Value::String(\n                    \"some-string\".to_string(),\n                )),\n                expense_policy_interaction_needs_review: Some(true),\n                from_date: Some(chrono::Utc::now()),\n                has_no_sync_commits: Some(true),\n                include_merchant_data: Some(true),\n                limit_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                location_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                manager_id: Some(serde_json::Value::String(\"some-string\".to_string())),\n                max_amount: Some(3.14 as f64),\n                merchant_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                min_amount: Some(3.14 as f64),\n                order_by_amount_asc: Some(true),\n                order_by_amount_desc: Some(true),\n                order_by_date_asc: Some(true),\n                order_by_date_desc: Some(true),\n                page_size: Some(4 as i64),\n                requires_memo: Some(true),\n                sk_category_id: Some(\"some-string\".to_string()),\n                start: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                state: Some(ramp_api::types::GetTransactionsCanonicalListWithPaginationState::Error),\n                statement_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                sync_ready: Some(true),\n                synced_after: Some(chrono::Utc::now()),\n                to_date: Some(chrono::Utc::now()),\n                trip_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                user_id: Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_transactions_canonical_list_with_pagination<'a>(
        &'a self,
        params: GetTransactionsCanonicalListWithPaginationParams,
    ) -> Result<
        crate::types::PaginatedResponseApiTransactionCanonicalSchema,
        crate::types::error::Error,
    > {
        let GetTransactionsCanonicalListWithPaginationParams {
            card_id,
            department_id,
            entity_id,
            expense_policy_interaction_has_alert,
            expense_policy_interaction_needs_review,
            from_date,
            has_no_sync_commits,
            include_merchant_data,
            limit_id,
            location_id,
            manager_id,
            max_amount,
            merchant_id,
            min_amount,
            order_by_amount_asc,
            order_by_amount_desc,
            order_by_date_asc,
            order_by_date_desc,
            page_size,
            requires_memo,
            sk_category_id,
            start,
            state,
            statement_id,
            sync_ready,
            synced_after,
            to_date,
            trip_id,
            user_id,
        } = params;
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
