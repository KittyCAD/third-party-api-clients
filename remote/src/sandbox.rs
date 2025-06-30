use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Sandbox {
    pub client: Client,
}

impl Sandbox {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Update employment in the Sandbox Environment\n\nUpdates an employment. Use this endpoint to modify employment states for testing\nin the Sandbox environment. This endpoint will respond with a 404 outside of the\nSandbox environment.\n\nFor updating an employment's parameters outside of testing purposes, use [this\nEmployment update endpoint](#operation/patch_update_employment).\n\n\n**Parameters:**\n\n- `employment_id: &'astr`: Employment ID (required)\n\n```rust,no_run\nasync fn example_sandbox_patch_update_employment_4() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::EmploymentResponse = client\n        .sandbox()\n        .patch_update_employment_4(\n            \"some-string\",\n            &remote_api::types::EmploymentUpdateParams {\n                status: Some(remote_api::types::EmploymentStatus::Pending),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_update_employment_4<'a>(
        &'a self,
        employment_id: &'a str,
        body: &crate::types::EmploymentUpdateParams,
    ) -> Result<crate::types::EmploymentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/sandbox/employments/{employment_id}".replace("{employment_id}", employment_id)
            ),
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

    #[doc = "Update employment in the Sandbox Environment\n\nUpdates an employment. Use this endpoint to modify employment states for testing\nin the Sandbox environment. This endpoint will respond with a 404 outside of the\nSandbox environment.\n\nFor updating an employment's parameters outside of testing purposes, use [this\nEmployment update endpoint](#operation/patch_update_employment).\n\n\n**Parameters:**\n\n- `employment_id: &'astr`: Employment ID (required)\n\n```rust,no_run\nasync fn example_sandbox_patch_update_employment_3() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::EmploymentResponse = client\n        .sandbox()\n        .patch_update_employment_3(\n            \"some-string\",\n            &remote_api::types::EmploymentUpdateParams {\n                status: Some(remote_api::types::EmploymentStatus::Pending),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_update_employment_3<'a>(
        &'a self,
        employment_id: &'a str,
        body: &crate::types::EmploymentUpdateParams,
    ) -> Result<crate::types::EmploymentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/sandbox/employments/{employment_id}".replace("{employment_id}", employment_id)
            ),
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
}
