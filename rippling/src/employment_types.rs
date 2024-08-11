use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct EmploymentTypes {
    pub client: Client,
}

impl EmploymentTypes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List employment types\n\nA List of employment types\n- Requires: `API Tier 1`\n- \
             Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `order_by: \
             Option<String>`\n\n```rust,no_run\nasync fn example_employment_types_list() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::ListEmploymentTypesResponse = client\n        \
             .employment_types()\n        .list(Some(\"some-string\".to_string()))\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        order_by: Option<String>,
    ) -> Result<crate::types::ListEmploymentTypesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "employment-types"),
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
            })
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Retrieve a specific employment type\n\nRetrieve a specific employment \
             type\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to return \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_employment_types_get() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::GetEmploymentTypesResponse = client\n        \
             .employment_types()\n        .get(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::GetEmploymentTypesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "employment-types/{id}".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
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
