use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Teams {
    pub client: Client,
}

impl Teams {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List teams\n\nA List of teams\n- Requires: `API Tier 1`\n- Expandable fields: `parent`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `expand: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nasync fn example_teams_list() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::ListTeamsResponse = client\n        .teams()\n        .list(\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        expand: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListTeamsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "teams"),
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

    #[doc = "Retrieve a specific team\n\nRetrieve a specific team\n\n**Parameters:**\n\n- `expand: \
             Option<String>`\n- `id: &'astr`: ID of the resource to return \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_teams_get() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::GetTeamsResponse = client\n        .teams()\n        \
             .get(\n            Some(\"some-string\".to_string()),\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        expand: Option<String>,
        id: &'a str,
    ) -> Result<crate::types::GetTeamsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{id}".replace("{id}", &id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = expand {
            query_params.push(("expand", p));
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
