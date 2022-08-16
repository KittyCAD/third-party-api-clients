use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Signatures {
    pub client: Client,
}

impl Signatures {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List teammate signatures\n\nList the signatures belonging to the given \
             teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID \
             (required)\n\n```rust,no_run\nasync fn example_signatures_list_teammate() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListTeammateSignaturesResponse =\n        \
             client.signatures().list_teammate(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateSignaturesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/signatures".replace("{teammate_id}", teammate_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create teammate signature\n\nCreate a new signature for the given teammate\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_signatures_create_teammate() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::SignatureResponse = client\n        .signatures()\n        .create_teammate(\n            \"some-string\",\n            &front_api::types::CreatePrivateSignature {\n                name: \"some-string\".to_string(),\n                sender_info: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                is_default: Some(true),\n                channel_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
        body: &crate::types::CreatePrivateSignature,
    ) -> Result<crate::types::SignatureResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/signatures".replace("{teammate_id}", teammate_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List team signatures\n\nList the signatures belonging to the given \
             team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID \
             (required)\n\n```rust,no_run\nasync fn example_signatures_list_team() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListTeamSignaturesResponse =\n        \
             client.signatures().list_team(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamSignaturesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/signatures".replace("{team_id}", team_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create team signature\n\nCreate a new signature for the given \
             team\n\n**Parameters:**\n\n- `team_id: &'astr`: The teammate ID \
             (required)\n\n```rust,no_run\nasync fn example_signatures_create_team() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::SignatureResponse = client\n        .signatures()\n        \
             .create_team(\n            \"some-string\",\n            \
             &front_api::types::CreateSharedSignature {\n                name: \
             \"some-string\".to_string(),\n                sender_info: \
             Some(\"some-string\".to_string()),\n                body: \
             \"some-string\".to_string(),\n                is_visible_for_all_teammate_channels: \
             Some(true),\n                is_default: Some(true),\n                channel_ids: \
             Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateSharedSignature,
    ) -> Result<crate::types::SignatureResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/signatures".replace("{team_id}", team_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get signatures\n\nGet the given signature.\n\n**Parameters:**\n\n- `signature_id: &'astr`: The signature ID (required)\n\n```rust,no_run\nasync fn example_signatures_get() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::SignatureResponse = client.signatures().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        signature_id: &'a str,
    ) -> Result<crate::types::SignatureResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "signatures/{signature_id}".replace("{signature_id}", signature_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Delete signature\n\nDelete signature\n\n**Parameters:**\n\n- `signature_id: &'astr`: \
             The signature ID (required)\n\n```rust,no_run\nasync fn example_signatures_delete() \
             -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client.signatures().delete(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        signature_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "signatures/{signature_id}".replace("{signature_id}", signature_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Update signature\n\nUpdate signature\n\n**Parameters:**\n\n- `signature_id: &'astr`: The signature ID (required)\n\n```rust,no_run\nasync fn example_signatures_update() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::SignatureResponse = client\n        .signatures()\n        .update(\n            \"some-string\",\n            &front_api::types::UpdateSignature {\n                name: Some(\"some-string\".to_string()),\n                sender_info: Some(\"some-string\".to_string()),\n                body: Some(\"some-string\".to_string()),\n                is_visible_for_all_teammate_channels: Some(false),\n                is_default: Some(true),\n                channel_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        signature_id: &'a str,
        body: &crate::types::UpdateSignature,
    ) -> Result<crate::types::SignatureResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "signatures/{signature_id}".replace("{signature_id}", signature_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
