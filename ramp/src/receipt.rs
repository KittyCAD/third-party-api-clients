use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Receipt {
    pub client: Client,
}

impl Receipt {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List receipts\n\n**Parameters:**\n\n- `created_after: \
             Option<chrono::DateTime<chrono::Utc>>`: Filter for receipts that were created after \
             the specified date. Input need to be presented in ISO8601 format, e.g. \
             2020-12-02T00:00:00\n- `created_before: Option<chrono::DateTime<chrono::Utc>>`: \
             Filter for receipts that were created before the specified date. Input need to be \
             presented in ISO8601 format, e.g. 2020-12-02T00:00:00\n- `from_date: \
             Option<chrono::DateTime<chrono::Utc>>`: Filter for receipts related to transactions \
             which occurred after the specified date. Input need to be presented in ISO8601 \
             format, e.g. 2020-12-02T00:00:00\n- `page_size: Option<i64>`: The number of results \
             to be returned in each page. The value must be between 2 and 10,000. If not \
             specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The \
             ID of the last entity of the previous page, used for pagination to get the next \
             page.\n- `to_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for receipts \
             related to transactions which occurred before the specified date. Input need to be \
             presented in ISO8601 format, e.g. 2020-12-02T00:00:00\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_receipt_get_list() -> anyhow::Result<()> {\n    \
             let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::PaginatedResponseApiReceiptResourceSchema = client\n        \
             .receipt()\n        .get_list(\n            Some(chrono::Utc::now()),\n            \
             Some(chrono::Utc::now()),\n            Some(chrono::Utc::now()),\n            Some(4 \
             as i64),\n            Some(uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            \
             Some(chrono::Utc::now()),\n        )\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list<'a>(
        &'a self,
        created_after: Option<chrono::DateTime<chrono::Utc>>,
        created_before: Option<chrono::DateTime<chrono::Utc>>,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<crate::types::PaginatedResponseApiReceiptResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/receipts"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = created_after {
            query_params.push(("created_after", format!("{}", p)));
        }

        if let Some(p) = created_before {
            query_params.push(("created_before", format!("{}", p)));
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

    #[doc = "Upload a receipt\n\nMust specify a transaction to which the uploaded receipt will be attached.\n\nThis endpoint accepts the\n[multipart/form-data](https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/POST)\ninput format.\n\nThe receipt image raw data should be included in a section with\n\"Content-Disposition: attachment\", and the form data should be included\nin sections with \"Content-Disposition: form-data\".\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_receipt_post_upload() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::LightReceipt = client\n        .receipt()\n        .post_upload(&ramp_api::types::ApiReceiptUploadRequestBody {\n            idempotency_key: \"some-string\".to_string(),\n            transaction_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            user_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_upload<'a>(
        &'a self,
        body: &crate::types::ApiReceiptUploadRequestBody,
    ) -> Result<crate::types::LightReceipt, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/receipts"),
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

    #[doc = "Fetch a receipt\n\n**Parameters:**\n\n- `receipt_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_receipt_get_single_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Receipt = client\n        .receipt()\n        .get_single_resource(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_single_resource<'a>(
        &'a self,
        receipt_id: uuid::Uuid,
    ) -> Result<crate::types::Receipt, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/receipts/{receipt_id}"
                    .replace("{receipt_id}", &format!("{}", receipt_id))
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
