use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Beta {
    pub client: Client,
}

impl Beta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "API Token status information\n\nStatus information about the API token used in the \
             request.\n\n\n```rust,no_run\nasync fn example_beta_api_token_status() -> \
             anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    \
             let result: commonroom_api::types::ApiToken = \
             client.beta().api_token_status().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn api_token_status<'a>(
        &'a self,
    ) -> Result<crate::types::ApiToken, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "api-token-status"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
