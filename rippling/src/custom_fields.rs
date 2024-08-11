use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CustomFields {
    pub client: Client,
}

impl CustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List custom fields\n\nA List of custom fields\n- Requires: `API Tier 1`\n- Sortable \
             fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `order_by: \
             Option<String>`\n\n```rust,no_run\nasync fn example_custom_fields_list() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::ListCustomFieldsResponse = client\n        \
             .custom_fields()\n        .list(Some(\"some-string\".to_string()))\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        order_by: Option<String>,
    ) -> Result<crate::types::ListCustomFieldsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "custom-fields"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
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
