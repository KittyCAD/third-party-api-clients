use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct EarningType {
    pub client: Client,
}

impl EarningType {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get all earning types for a company\n\nA payroll item in Gusto is associated to an earning type to name the type of earning described by the payroll item.\n\n#### Default Earning Type\nCertain earning types are special because they have tax considerations. Those earning types are mostly the same for every company depending on its legal structure (LLC, Corporation, etc.)\n\n#### Custom Earning Type\nCustom earning types are all the other earning types added specifically for a company.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n\n```rust,no_run\nasync fn example_earning_type_get_companies_company_id_earning_types() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::GetCompaniesCompanyIdEarningTypesResponse = client\n        .earning_type()\n        .get_companies_company_id_earning_types(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_earning_types<'a>(
        &'a self,
        company_id: &'a str,
    ) -> Result<crate::types::GetCompaniesCompanyIdEarningTypesResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/earning_types".replace("{company_id}", company_id)
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create a custom earning type\n\nCreate a custom earning type.\n\nIf an inactive earning type exists with the same name, this will reactivate it instead of creating a new one.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n\n```rust,no_run\nasync fn example_earning_type_post_companies_company_id_earning_types() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::EarningType = client\n        .earning_type()\n        .post_companies_company_id_earning_types(\n            \"some-string\",\n            &gusto_api::types::PostCompaniesCompanyIdEarningTypesRequestBody {\n                name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id_earning_types<'a>(
        &'a self,
        company_id: &'a str,
        body: &crate::types::PostCompaniesCompanyIdEarningTypesRequestBody,
    ) -> Result<crate::types::EarningType, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/earning_types".replace("{company_id}", company_id)
            ),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Update an earning type\n\nUpdate an earning type.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n- `earning_type_uuid: &'astr`: The UUID of the earning type (required)\n\n```rust,no_run\nasync fn example_earning_type_put_companies_company_id_earning_types_uuid() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::EarningType = client\n        .earning_type()\n        .put_companies_company_id_earning_types_uuid(\n            \"some-string\",\n            \"some-string\",\n            &gusto_api::types::PutCompaniesCompanyIdEarningTypesEarningTypeUuidRequestBody {\n                name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_earning_types_uuid<'a>(
        &'a self,
        company_id: &'a str,
        earning_type_uuid: &'a str,
        body: &crate::types::PutCompaniesCompanyIdEarningTypesEarningTypeUuidRequestBody,
    ) -> Result<crate::types::EarningType, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/earning_types/{earning_type_uuid}"
                    .replace("{company_id}", company_id)
                    .replace("{earning_type_uuid}", earning_type_uuid)
            ),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Deactivate an earning type\n\nDeactivate an earning type.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n- `earning_type_uuid: &'astr`: The UUID of the earning type (required)\n\n```rust,no_run\nasync fn example_earning_type_delete_companies_company_id_earning_types_uuid() -> anyhow::Result<()>\n{\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .earning_type()\n        .delete_companies_company_id_earning_types_uuid(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_companies_company_id_earning_types_uuid<'a>(
        &'a self,
        company_id: &'a str,
        earning_type_uuid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/earning_types/{earning_type_uuid}"
                    .replace("{company_id}", company_id)
                    .replace("{earning_type_uuid}", earning_type_uuid)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
