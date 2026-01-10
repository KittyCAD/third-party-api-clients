use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Backups {
    pub client: Client,
}

impl Backups {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List backups\n\n```rust,no_run\nasync fn example_backups_get() -> anyhow::Result<()> \
             {\n    let client = discourse_api::Client::new_from_env();\n    let result: \
             Vec<discourse_api::types::GetBackupsResponse> = client.backups().get().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::GetBackupsResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "admin/backups.json"),
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

    #[doc = "Create backup\n\n```rust,no_run\nasync fn example_backups_create() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateBackupResponse = client\n        .backups()\n        .create(&discourse_api::types::CreateBackupRequestBody { with_uploads: true })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateBackupRequestBody,
    ) -> Result<crate::types::CreateBackupResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "admin/backups.json"),
        );
        req = req.bearer_auth(&self.client.token);
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

    #[doc = "Download backup\n\n**Parameters:**\n\n- `filename: &'astr` (required)\n- `token: &'astr` (required)\n\n```rust,no_run\nasync fn example_backups_download() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    client\n        .backups()\n        .download(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download<'a>(
        &'a self,
        filename: &'a str,
        token: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/backups/{filename}".replace("{filename}", filename)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("token", format!("{}", token))];
        req = req.query(&query_params);
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

    #[doc = "Send download backup email\n\n**Parameters:**\n\n- `filename: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_backups_send_download_email() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             client.backups().send_download_email(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn send_download_email<'a>(
        &'a self,
        filename: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "admin/backups/{filename}".replace("{filename}", filename)
            ),
        );
        req = req.bearer_auth(&self.client.token);
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
