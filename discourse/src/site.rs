use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Site {
    pub client: Client,
}

impl Site {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get site info\n\nCan be used to fetch all categories and \
             subcategories\n\n```rust,no_run\nasync fn example_site_get() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::GetSiteResponse = client.site().get().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
    ) -> Result<crate::types::GetSiteResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "site.json"),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get site basic info\n\nCan be used to fetch basic info about a \
             site\n\n```rust,no_run\nasync fn example_site_get_basic_info() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             discourse_api::types::GetSiteBasicInfoResponse = \
             client.site().get_basic_info().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_basic_info<'a>(
        &'a self,
    ) -> Result<crate::types::GetSiteBasicInfoResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "site/basic-info.json"),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
