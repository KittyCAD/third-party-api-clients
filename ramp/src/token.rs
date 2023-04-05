use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Token {
    pub client: Client,
}

impl Token {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `POST` request to `/developer/v1/token/`.\n\n```rust,no_run\nasync fn \
             example_token_post() -> anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client.token().post().await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/token/"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `POST` request to `/developer/v1/token/pkce`.\n\n```rust,no_run\nasync fn \
             example_token_post_pkce() -> anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client.token().post_pkce().await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_pkce<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/token/pkce"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `POST` request to `/developer/v1/token/revoke`.\n\n```rust,no_run\nasync fn \
             example_token_post_revoke() -> anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client.token().post_revoke().await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_revoke<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/token/revoke"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
