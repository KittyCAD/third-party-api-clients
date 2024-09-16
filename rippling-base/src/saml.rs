use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Saml {
    pub client: Client,
}

impl Saml {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "GET SAML Metadata\n\nReturns a SAML IDP metadata file for the current app \
             integration. Note that this endpoint is only accessible using a token associated with \
             an app integration that has SAML enabled; otherwise it returns a 404 \
             error.\n\nRippling's SAML Metadata is per customer app installation. It is not the \
             same across all customers. It is not the same if the customer uninstalls and \
             reinstalls your app. Any time a new app is installed, unique SAML Metadata will be \
             generated specific to that app.\n\n```rust,no_run\nasync fn \
             example_saml_get_idp_metadata() -> anyhow::Result<()> {\n    let client = \
             rippling_base_api::Client::new_from_env();\n    let result: String = \
             client.saml().get_idp_metadata().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_idp_metadata<'a>(&'a self) -> Result<String, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/saml/idp_metadata"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await?;
            Ok(text)
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
