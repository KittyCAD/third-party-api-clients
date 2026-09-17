use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug, Default)]
pub struct GetListWithPaginationParams {
    pub entity_id: Option<uuid::Uuid>,
    pub from_due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub from_issued_date: Option<chrono::DateTime<chrono::Utc>>,
    pub page_size: Option<i64>,
    pub payment_method: Option<crate::types::PaymentMethod>,
    pub payment_status: Option<crate::types::PaymentStatus>,
    pub start: Option<uuid::Uuid>,
    pub sync_ready: Option<bool>,
    pub to_due_date: Option<chrono::DateTime<chrono::Utc>>,
    pub to_issued_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Clone, Debug)]
pub struct Bill {
    pub client: Client,
}

impl Bill {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List bills\n\n**Parameters:**\n\n- `entity_id: Option<uuid::Uuid>`: Filter bills by entity.\n- `from_due_date: Option<chrono::DateTime<chrono::Utc>>`: Shows only bills with a due_at on or after this date. This parameter should be provided as a datetime string that conforms to ISO 8601\n- `from_issued_date: Option<chrono::DateTime<chrono::Utc>>`: Shows only bills with a issued_at on or after this date. This parameter should be provided as a datetime string that conforms to ISO 8601\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `payment_method: Option<crate::types::PaymentMethod>`: List bills of the provided payment method.\n- `payment_status: Option<crate::types::PaymentStatus>`: List bills of the provided payment status.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `sync_ready: Option<bool>`: Only show bills that are ready to sync to ERP, if set to True\n- `to_due_date: Option<chrono::DateTime<chrono::Utc>>`: Shows only bills with a due_at on or beofre this date. This parameter should be provided as a datetime string that conforms to ISO 8601\n- `to_issued_date: Option<chrono::DateTime<chrono::Utc>>`: Shows only bills with a issued_at on or beofre this date. This parameter should be provided as a datetime string that conforms to ISO 8601\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_bill_get_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiBillResourceSchema = client\n        .bill()\n        .get_list_with_pagination(ramp_api::bill::GetListWithPaginationParams {\n            entity_id: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            from_due_date: Some(chrono::Utc::now()),\n            from_issued_date: Some(chrono::Utc::now()),\n            page_size: Some(4 as i64),\n            payment_method: Some(ramp_api::types::PaymentMethod::PaidManually),\n            payment_status: Some(ramp_api::types::PaymentStatus::Paid),\n            start: Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            sync_ready: Some(true),\n            to_due_date: Some(chrono::Utc::now()),\n            to_issued_date: Some(chrono::Utc::now()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        params: GetListWithPaginationParams,
    ) -> Result<crate::types::PaginatedResponseApiBillResourceSchema, crate::types::error::Error>
    {
        let GetListWithPaginationParams {
            entity_id,
            from_due_date,
            from_issued_date,
            page_size,
            payment_method,
            payment_status,
            start,
            sync_ready,
            to_due_date,
            to_issued_date,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/bills"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = from_due_date {
            query_params.push(("from_due_date", format!("{}", p)));
        }

        if let Some(p) = from_issued_date {
            query_params.push(("from_issued_date", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = payment_method {
            query_params.push(("payment_method", format!("{}", p)));
        }

        if let Some(p) = payment_status {
            query_params.push(("payment_status", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
        }

        if let Some(p) = sync_ready {
            query_params.push(("sync_ready", format!("{}", p)));
        }

        if let Some(p) = to_due_date {
            query_params.push(("to_due_date", format!("{}", p)));
        }

        if let Some(p) = to_issued_date {
            query_params.push(("to_issued_date", format!("{}", p)));
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

    #[doc = "Fetch a bill\n\n**Parameters:**\n\n- `bill_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_bill_get_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Bill = \
             client.bill().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        bill_id: &'a str,
    ) -> Result<crate::types::Bill, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/bills/{bill_id}".replace("{bill_id}", bill_id)
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
