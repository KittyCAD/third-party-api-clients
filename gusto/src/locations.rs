use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Locations {
    pub client: Client,
}

impl Locations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get company locations\n\nCompany locations represent all addresses associated with a company. These can be filing addesses, mailing addresses, and/or work locations; one address may serve multiple, or all, purposes.\n\nSince all company locations are subsets of locations, retrieving or updating an individual record should be done via the locations endpoints.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn example_locations_get_companies_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Location> = client\n        .locations()\n        .get_companies_company_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
    ) -> Result<Vec<crate::types::Location>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/locations"
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

    #[doc = "Create a company location\n\nCompany locations represent all addresses associated with a company. These can be filing addesses, mailing addresses, and/or work locations; one address may serve multiple, or all, purposes.\n\nSince all company locations are subsets of locations, retrieving or updating an individual record should be done via the locations endpoints.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn example_locations_post_companies_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Location = client\n        .locations()\n        .post_companies_company_id(\n            \"some-string\",\n            &gusto_api::types::PostCompaniesCompanyIdLocationsRequestBody {\n                phone_number: \"some-string\".to_string(),\n                street_1: \"some-string\".to_string(),\n                street_2: Some(\"some-string\".to_string()),\n                city: \"some-string\".to_string(),\n                state: \"some-string\".to_string(),\n                zip: \"some-string\".to_string(),\n                country: Some(\"some-string\".to_string()),\n                mailing_address: Some(false),\n                filing_address: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        body: &crate::types::PostCompaniesCompanyIdLocationsRequestBody,
    ) -> Result<crate::types::Location, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/locations"
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

    #[doc = "Get a location\n\nGet a location.\n\n**Parameters:**\n\n- `location_id: &'astr`: The ID of the location (required)\n\n```rust,no_run\nasync fn example_locations_get_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Location = client.locations().get_id(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id<'a>(
        &'a self,
        location_id: &'a str,
    ) -> Result<crate::types::Location, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/locations/{location_id}".replace("{location_id}", location_id)
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

    #[doc = "Update a location\n\nUpdate a location.\n\n**Parameters:**\n\n- `location_id: &'astr`: The ID of the location (required)\n\n```rust,no_run\nasync fn example_locations_put_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Location = client\n        .locations()\n        .put_id(\n            \"some-string\",\n            &gusto_api::types::PutLocationsLocationIdRequestBody {\n                phone_number: Some(\"some-string\".to_string()),\n                street_1: Some(\"some-string\".to_string()),\n                street_2: Some(\"some-string\".to_string()),\n                city: Some(\"some-string\".to_string()),\n                state: Some(\"some-string\".to_string()),\n                zip: Some(\"some-string\".to_string()),\n                country: Some(\"some-string\".to_string()),\n                mailing_address: Some(false),\n                filing_address: Some(false),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_id<'a>(
        &'a self,
        location_id: &'a str,
        body: &crate::types::PutLocationsLocationIdRequestBody,
    ) -> Result<crate::types::Location, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/locations/{location_id}".replace("{location_id}", location_id)
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
