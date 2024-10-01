use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct TransferPayment {
    pub client: Client,
}

impl TransferPayment {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List transfer payments\n\n**Parameters:**\n\n- `entity_id: Option<uuid::Uuid>`: Filter by business entity.\n- `from_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for transfers that occurred after the specified date. Input need to be presented in ISO8601 format, e.g. 2020-12-02T00:00:00\n- `has_no_sync_commits: Option<bool>`: Filter for transfers that have no sync commits.\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `statement_id: Option<uuid::Uuid>`: Filter for transfers that shows up in the specified statement\n- `status: Option<crate::types::GetTransferListWithPaginationStatus>`: Filter by transfer state.\n- `to_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for transfers that occurred before the specified date. Input need to be presented in ISO8601 format, e.g. 2020-12-02T00:00:00\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_transfer_payment_get_transfer_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiTransferResourceSchema = client\n        .transfer_payment()\n        .get_transfer_list_with_pagination(\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(chrono::Utc::now()),\n            Some(false),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(ramp_api::types::GetTransferListWithPaginationStatus::Unnecessary),\n            Some(chrono::Utc::now()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_transfer_list_with_pagination<'a>(
        &'a self,
        entity_id: Option<uuid::Uuid>,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        has_no_sync_commits: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        statement_id: Option<uuid::Uuid>,
        status: Option<crate::types::GetTransferListWithPaginationStatus>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<crate::types::PaginatedResponseApiTransferResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/transfers"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = from_date {
            query_params.push(("from_date", format!("{}", p)));
        }

        if let Some(p) = has_no_sync_commits {
            query_params.push(("has_no_sync_commits", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
        }

        if let Some(p) = statement_id {
            query_params.push(("statement_id", format!("{}", p)));
        }

        if let Some(p) = status {
            query_params.push(("status", format!("{}", p)));
        }

        if let Some(p) = to_date {
            query_params.push(("to_date", format!("{}", p)));
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

    #[doc = "Fetch a transfer payment\n\n**Parameters:**\n\n- `transfer_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn \
             example_transfer_payment_get_transfer_resource() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Transfer = \
             client\n        .transfer_payment()\n        \
             .get_transfer_resource(\"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_transfer_resource<'a>(
        &'a self,
        transfer_id: &'a str,
    ) -> Result<crate::types::Transfer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/transfers/{transfer_id}".replace("{transfer_id}", transfer_id)
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
