use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Statement {
    pub client: Client,
}

impl Statement {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List statements\n\n**Parameters:**\n\n- `from_date: \
             Option<chrono::DateTime<chrono::Utc>>`: Shows only statements with an end_date on or \
             after this date. This parameter should be provided as a datetime string that conforms \
             to ISO 8601\n- `page_size: Option<i64>`: The number of results to be returned in each \
             page. The value must be between 2 and 10,000. If not specified, the default value \
             1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the \
             previous page, used for pagination to get the next page.\n- `to_date: \
             Option<chrono::DateTime<chrono::Utc>>`: Shows only statements with an end_date on or \
             before this date. This parameter should be provided as a datetime string that \
             conforms to ISO 8601. Defaults to current datetime if not \
             specified.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_statement_get_list_with_pagination() -> anyhow::Result<()> {\n    let client \
             =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::PaginatedResponseApiStatementResourceSchema = client\n        \
             .statement()\n        .get_list_with_pagination(\n            \
             Some(chrono::Utc::now()),\n            Some(4 as i64),\n            \
             Some(uuid::Uuid::from_str(\n                \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            \
             Some(chrono::Utc::now()),\n        )\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<crate::types::PaginatedResponseApiStatementResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/statements"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
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

    #[doc = "Fetch a statement\n\n**Parameters:**\n\n- `statement_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_statement_get_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Statement = \
             client.statement().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        statement_id: &'a str,
    ) -> Result<crate::types::Statement, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/statements/{statement_id}".replace("{statement_id}", statement_id)
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
