use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Contractors {
    pub client: Client,
}

impl Contractors {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a contractor\n\nGet a contractor.\n\n**Parameters:**\n\n- `contractor_id_or_uuid: \
             &'astr`: The ID or UUID of the contractor (required)\n\n```rust,no_run\nasync fn \
             example_contractors_get_id() -> anyhow::Result<()> {\n    let client =\n        \
             gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::Contractor = \
             client.contractors().get_id(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id<'a>(
        &'a self,
        contractor_id_or_uuid: &'a str,
    ) -> Result<crate::types::Contractor, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/contractors/{contractor_id_or_uuid}"
                    .replace("{contractor_id_or_uuid}", contractor_id_or_uuid)
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

    #[doc = "Update a contractor\n\nUpdate a contractor.\n\n**Parameters:**\n\n- `contractor_id_or_uuid: &'astr`: The ID or UUID of the contractor (required)\n\n```rust,no_run\nasync fn example_contractors_put_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Contractor = client\n        .contractors()\n        .put_id(\n            \"some-string\",\n            &gusto_api::types::PutContractorsContractorIdRequestBody {\n                version: Some(\"some-string\".to_string()),\n                start_date: Some(chrono::Utc::now().date().naive_utc()),\n                first_name: Some(\"some-string\".to_string()),\n                last_name: Some(\"some-string\".to_string()),\n                middle_initial: Some(\"some-string\".to_string()),\n                wage_type: Some(gusto_api::types::PutContractorsContractorIdRequestBodyWageType::Fixed),\n                hourly_rate: Some(\"some-string\".to_string()),\n                business_name: Some(\"some-string\".to_string()),\n                ein: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_id<'a>(
        &'a self,
        contractor_id_or_uuid: &'a str,
        body: &crate::types::PutContractorsContractorIdRequestBody,
    ) -> Result<crate::types::Contractor, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/contractors/{contractor_id_or_uuid}"
                    .replace("{contractor_id_or_uuid}", contractor_id_or_uuid)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get contractors of a company\n\nGet all contractors, active and inactive, individual \
             and business, for a company.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: \
             The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn \
             example_contractors_get_companies_company_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Contractor> \
             = client\n        .contractors()\n        \
             .get_companies_company_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
    ) -> Result<Vec<crate::types::Contractor>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/contractors"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
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

    #[doc = "Create a contractor\n\nCreate an individual or business contractor.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn example_contractors_post_companies_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Contractor = client\n        .contractors()\n        .post_companies_company_id(\n            \"some-string\",\n            &gusto_api::types::PostCompaniesCompanyIdContractorsRequestBody {\n                type_: gusto_api::types::PostCompaniesCompanyIdContractorsRequestBodyType::Individual,\n                wage_type:\n                    gusto_api::types::PostCompaniesCompanyIdContractorsRequestBodyWageType::Hourly,\n                first_name: Some(\"some-string\".to_string()),\n                last_name: Some(\"some-string\".to_string()),\n                start_date: chrono::Utc::now().date().naive_utc(),\n                self_onboarding: Some(false),\n                email: Some(\"some-string\".to_string()),\n                middle_initial: Some(\"some-string\".to_string()),\n                business_name: Some(\"some-string\".to_string()),\n                ein: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        body: &crate::types::PostCompaniesCompanyIdContractorsRequestBody,
    ) -> Result<crate::types::Contractor, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/contractors"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
