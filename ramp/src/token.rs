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

    #[doc = "Create a token\n\nToken is specific to app whose client credentials are passed. Please refer to our [request authorization docs](https://docs.ramp.com/developer-api/v1/authorization) for usage.\n\nExpects two headers:\n- Authorization header formed from base-64 encoded client credentials as `Authorization: Basic <base64-encoded client_id:client_secret>`\n- `Content-Type: application/x-www-form-urlencoded`\n\nRequired content body depends on authorization type method, as defined by `grant_type`.\n- Authorization Code Grant (`grant_type=authorization_code`): `grant_type`, `code`, and `redirect_uri` are required. Request must happen after requested scopes have been approved and exchanged for authorization code.\n- Refresh Token Grant (`grant_type=refresh_token`): `grant_type` and `refresh_token` are required. User must have previously obtained refresh token in authorization code flow.\n- Client Credentials Grant (`grant_type=client_credentials`): `grant_type` and `scope` are required.\n\n```rust,no_run\nasync fn example_token_post() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::TokenResponse = client\n        .token()\n        .post(&ramp_api::types::TokenRequestBody {\n            code: Some(\"some-string\".to_string()),\n            grant_type: ramp_api::types::GrantType::ClientCredentials,\n            redirect_uri: Some(\"some-string\".to_string()),\n            refresh_token: Some(\"some-string\".to_string()),\n            scope: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post<'a>(
        &'a self,
        body: &crate::types::TokenRequestBody,
    ) -> Result<crate::types::TokenResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/token"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        req = req.json(body);
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

    #[doc = "Revoke an access or refresh token\n\nExpects an authorization header formed from base-64 encoded client credentials as `Authorization: Basic <base64-encoded client_id:client_secret>`.\n\nContent body must be form-encoded. Example:\n```ignore\ncurl \\\n-X POST \\\n-H \"Authorization: Basic <base64-encoded client_id:client_secret>\" \\\n-H \"Content-Type: application/x-www-form-urlencoded\" \\\n--data-urlencode 'token=$RAMP_API_TOKEN' \\\n'https://api.ramp.com/developer/v1/token/revoke'\n```ignore\n\n```rust,no_run\nasync fn example_token_post_revoke() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .token()\n        .post_revoke(&ramp_api::types::TokenRevokeRequestBody {\n            token: \"some-string\".to_string(),\n            token_type_hint: Some(ramp_api::types::TokenTypeHint::RefreshToken),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_revoke<'a>(
        &'a self,
        body: &crate::types::TokenRevokeRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/token/revoke"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
