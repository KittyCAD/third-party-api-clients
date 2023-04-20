use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CustomIdProvider {
    pub client: Client,
}

impl CustomIdProvider {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get provider id associated with access token\n\n```rust,no_run\nasync fn example_custom_id_provider_get_endpoint() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::CustomIdProvider = client.custom_id_provider().get_endpoint().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_endpoint<'a>(
        &'a self,
    ) -> Result<crate::types::CustomIdProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url, "developer/v1/custom-id-provider/"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Create custom id provider\n\n```rust,no_run\nasync fn \
             example_custom_id_provider_post_endpoint() -> anyhow::Result<()> {\n    let client \
             =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::CustomIdProvider \
             =\n        client.custom_id_provider().post_endpoint().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_endpoint<'a>(
        &'a self,
    ) -> Result<crate::types::CustomIdProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "developer/v1/custom-id-provider/"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Register an access token with custom id provider\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_custom_id_provider_post_token_endpoint() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client\n        .custom_id_provider()\n        \
             .post_token_endpoint(&ramp_api::types::ApiCustomIdProviderTokenCreate {\n            \
             custom_id_provider: \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        })\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_token_endpoint<'a>(
        &'a self,
        body: &crate::types::ApiCustomIdProviderTokenCreate,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "developer/v1/custom-id-provider/application-link"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get ramp id from corresponding custom id\n\n**Parameters:**\n\n- `custom_id: &'astr` (required)\n- `entity_type: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_id_provider_get_ramp_id_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::RampId = client\n        .custom_id_provider()\n        .get_ramp_id_resource(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_ramp_id_resource<'a>(
        &'a self,
        custom_id: &'a str,
        entity_type: &'a str,
    ) -> Result<crate::types::RampId, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/custom-id-provider/{entity_type}/{custom_id}/ramp-id"
                    .replace("{custom_id}", custom_id)
                    .replace("{entity_type}", entity_type)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Get custom id from corresponding ramp id\n\n**Parameters:**\n\n- `entity_type: \
             &'astr` (required)\n- `ramp_id: &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_custom_id_provider_get_ramp_to_custom_id() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::CustomId = \
             client\n        .custom_id_provider()\n        \
             .get_ramp_to_custom_id(\"some-string\", \"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_ramp_to_custom_id<'a>(
        &'a self,
        entity_type: &'a str,
        ramp_id: &'a str,
    ) -> Result<crate::types::CustomId, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/custom-id-provider/{entity_type}/{ramp_id}/custom-id"
                    .replace("{entity_type}", entity_type)
                    .replace("{ramp_id}", ramp_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Add custom id <-> ramp id mapping\n\n**Parameters:**\n\n- `entity_type: &'astr` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_custom_id_provider_post_create_custom_id_mapping() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .custom_id_provider()\n        .post_create_custom_id_mapping(\n            \"some-string\",\n            &ramp_api::types::ApiCustomIdMapping {\n                custom_id: ramp_api::types::ApiCustomIdMappingCustomId::L,\n                ramp_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_create_custom_id_mapping<'a>(
        &'a self,
        entity_type: &'a str,
        body: &crate::types::ApiCustomIdMapping,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/custom-id-provider/{entity_type}/custom-id-link"
                    .replace("{entity_type}", entity_type)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
