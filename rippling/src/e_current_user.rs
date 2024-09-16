use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ECurrentUser {
    pub client: Client,
}

impl ECurrentUser {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "GET Current User\n\nRetrieves basic information about the Rippling user whose access token you're using. This is generally used for the SSO flow.\n\n```rust,no_run\nasync fn example_e_current_user_get_me() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::AuthenticatedUserMe = client.e_current_user().get_me().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_me<'a>(
        &'a self,
    ) -> Result<crate::types::AuthenticatedUserMe, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/me"),
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
