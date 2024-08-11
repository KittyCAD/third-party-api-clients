use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Cashback {
    pub client: Client,
}

impl Cashback {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List cashback payments\n\n**Parameters:**\n\n- `entity_id: Option<uuid::Uuid>`: \
             Filter by business entity.\n- `from_date: Option<chrono::DateTime<chrono::Utc>>`: \
             Filter for cashback payments that occurred after the specified date. Input need to be \
             presented in ISO8601 format, e.g. 2020-12-02T00:00:00\n- `page_size: Option<i64>`: \
             The number of results to be returned in each page. The value must be between 2 and \
             10,000. If not specified, the default value 1,000 will be used.\n- `start: \
             Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for \
             pagination to get the next page.\n- `statement_id: Option<uuid::Uuid>`: Filter by \
             statement.\n- `sync_ready: Option<bool>`: Filter for cashback payments that are ready \
             to be synced. These have no pending syncs and are completed\n- `to_date: \
             Option<chrono::DateTime<chrono::Utc>>`: Filter for cashback payments that occurred \
             before the specified date. Input need to be presented in ISO8601 format, e.g. \
             2020-12-02T00:00:00\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_cashback_get_list_with_pagination() -> anyhow::Result<()> {\n    let client \
             =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::PaginatedResponseApiCashbackResourceSchema = client\n        \
             .cashback()\n        .get_list_with_pagination(\n            \
             Some(uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            \
             Some(chrono::Utc::now()),\n            Some(4 as i64),\n            \
             Some(uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            \
             Some(uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            \
             Some(false),\n            Some(chrono::Utc::now()),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        entity_id: Option<uuid::Uuid>,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        statement_id: Option<uuid::Uuid>,
        sync_ready: Option<bool>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<crate::types::PaginatedResponseApiCashbackResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "developer/v1/cashbacks"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = from_date {
            query_params.push(("from_date", format!("{}", p)));
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

        if let Some(p) = sync_ready {
            query_params.push(("sync_ready", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Fetch a cashback payment\n\n**Parameters:**\n\n- `cashback_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_cashback_get_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Cashback = \
             client.cashback().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        cashback_id: &'a str,
    ) -> Result<crate::types::Cashback, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/cashbacks/{cashback_id}".replace("{cashback_id}", cashback_id)
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
}
