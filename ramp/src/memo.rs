use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Memo {
    pub client: Client,
}

impl Memo {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get the memo of a transaction\n\n**Parameters:**\n\n- `transaction_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_memo_get_single_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Memo = \
             client.memo().get_single_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_single_resource<'a>(
        &'a self,
        transaction_id: &'a str,
    ) -> Result<crate::types::Memo, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/memos/{transaction_id}".replace("{transaction_id}", transaction_id)
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

    #[doc = "Returns a list of memos that meets the criteria\n\n**Parameters:**\n\n- `card_id: Option<String>`\n- `department_id: Option<String>`\n- `from_date: Option<chrono::DateTime<chrono::Utc>>`\n- `location_id: Option<String>`\n- `manager_id: Option<String>`\n- `merchant_id: Option<String>`\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `to_date: Option<chrono::DateTime<chrono::Utc>>`\n- `user_id: Option<String>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_memo_get_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiMemoResourceSchema = client\n        .memo()\n        .get_list_with_pagination(\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(chrono::Utc::now()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        card_id: Option<String>,
        department_id: Option<String>,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        location_id: Option<String>,
        manager_id: Option<String>,
        merchant_id: Option<String>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        user_id: Option<String>,
    ) -> Result<crate::types::PaginatedResponseApiMemoResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "developer/v1/memos/"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = Vec::new();
        if let Some(p) = card_id {
            query_params.push(("card_id", p));
        }

        if let Some(p) = department_id {
            query_params.push(("department_id", p));
        }

        if let Some(p) = from_date {
            query_params.push(("from_date", format!("{}", p)));
        }

        if let Some(p) = location_id {
            query_params.push(("location_id", p));
        }

        if let Some(p) = manager_id {
            query_params.push(("manager_id", p));
        }

        if let Some(p) = merchant_id {
            query_params.push(("merchant_id", p));
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

        if let Some(p) = user_id {
            query_params.push(("user_id", p));
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
}
