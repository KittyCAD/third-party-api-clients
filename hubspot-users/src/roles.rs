use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Roles {
    pub client: Client,
}

impl Roles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Retrieves the roles on an account\n\nRetrieves the roles on an account\n\n```rust,no_run\nasync fn example_roles_get_settings_v_3_users_get_all() -> anyhow::Result<()> {\n    let client = hubspot_users::Client::new_from_env();\n    let result: hubspot_users::types::CollectionResponsePublicPermissionSetNoPaging =\n        client.roles().get_settings_v_3_users_get_all().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_settings_v_3_users_get_all<'a>(
        &'a self,
    ) -> Result<
        crate::types::CollectionResponsePublicPermissionSetNoPaging,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "settings/v3/users/roles"),
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
