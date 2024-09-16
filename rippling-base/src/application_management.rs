use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ApplicationManagement {
    pub client: Client,
}

impl ApplicationManagement {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "GET Matching App Users\n\nReturns matching users and their app IDs based on the app handles.\n\nNote:There could be multiple instances of the same app. In this case, the API will return all instances in the format app_handle_app_owner_id.\n\n\n**Parameters:**\n\n- `app_handles: Option<String>`: CSV of app handles. See GET /app_detail/app_handles\n\n\n```rust,no_run\nasync fn example_application_management_get_app_app_matching_users() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: rippling_base_api::types::GetAppAppMatchingUsersResponse = client\n        .application_management()\n        .get_app_app_matching_users(Some(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_app_app_matching_users<'a>(
        &'a self,
        app_handles: Option<String>,
    ) -> Result<crate::types::GetAppAppMatchingUsersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/app_detail/app_matching_users"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = app_handles {
            query_params.push(("app_handles", p));
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

    #[doc = "Mark App Installed\n\nThis endpoint can be hit to mark your app as installed in \
             Rippling, if you aren't hitting Rippling's other endpoints on installation. The \
             endpoint does not require any scopes.\n\nPlease note, hitting any other endpoint \
             should mark your app as installed as well.\n\n```rust,no_run\nasync fn \
             example_application_management_post_mark_app_installed() -> anyhow::Result<()> {\n    \
             let client = rippling_base_api::Client::new_from_env();\n    let result: \
             rippling_base_api::types::PostMarkAppInstalledResponse = client\n        \
             .application_management()\n        .post_mark_app_installed()\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_mark_app_installed<'a>(
        &'a self,
    ) -> Result<crate::types::PostMarkAppInstalledResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/mark_app_installed"
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
