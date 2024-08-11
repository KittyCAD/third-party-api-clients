use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Me {
    pub client: Client,
}

impl Me {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Retrieve my SSO information\n\nSSO information of the current user\n- Requires: `API \
             Tier 1`\n- Expandable fields: `company`\n\n**Parameters:**\n\n- `expand: \
             Option<String>`\n\n```rust,no_run\nasync fn example_me_list_sso() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::Ssome = client\n        .me()\n        \
             .list_sso(Some(\"some-string\".to_string()))\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sso<'a>(
        &'a self,
        expand: Option<String>,
    ) -> Result<crate::types::Ssome, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "sso-me"),
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
