use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Companies {
    pub client: Client,
}

impl Companies {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List companies\n\nA List of companies\n- Requires: `API Tier 1`\n- Expandable fields: `parent_legal_entity`, `legal_entities`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `expand: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nasync fn example_companies_list() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::ListCompaniesResponse = client\n        .companies()\n        .list(\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        expand: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListCompaniesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "companies"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = expand {
            query_params.push(("expand", p));
        }

        if let Some(p) = order_by {
            query_params.push(("order_by", p));
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
                .into()
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
