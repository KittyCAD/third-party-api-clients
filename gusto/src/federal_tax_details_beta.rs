use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct FederalTaxDetailsBeta {
    pub client: Client,
}

impl FederalTaxDetailsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get Federal Tax Details\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nFetches attributes relevant for a company's federal taxes.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The company id or uuid (required)\n\n```rust,no_run\nasync fn example_federal_tax_details_beta_get_companies_company_id_or_uuid_federal_tax_details(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::GetCompaniesCompanyIdOrUuidFederalTaxDetailsResponse = client\n        .federal_tax_details_beta()\n        .get_companies_company_id_or_uuid_federal_tax_details(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_or_uuid_federal_tax_details<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
    ) -> Result<
        crate::types::GetCompaniesCompanyIdOrUuidFederalTaxDetailsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/federal_tax_details"
                    .replace("{company_id_or_uuid}", &company_id_or_uuid)
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Update Federal Tax Details\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nUpdates attributes relevant for a company's federal taxes. This information is required is to onboard a company for use with Gusto Embedded Payroll.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The company id or uuid (required)\n\n```rust,no_run\nasync fn example_federal_tax_details_beta_put_companies_company_id_or_uuid_federal_tax_details(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::PutCompaniesCompanyIdOrUuidFederalTaxDetailsResponse = client\n        .federal_tax_details_beta()\n        .put_companies_company_id_or_uuid_federal_tax_details(\n            \"some-string\",\n            &gusto_api::types::PutCompaniesCompanyIdOrUuidFederalTaxDetailsRequestBody {\n                legal_name: Some(\"some-string\".to_string()),\n                ein: Some(\"some-string\".to_string()),\n                tax_payer_type: Some(\"some-string\".to_string()),\n                filing_form: Some(\"some-string\".to_string()),\n                taxable_as_scorp: Some(true),\n                version: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_or_uuid_federal_tax_details<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        body: &crate::types::PutCompaniesCompanyIdOrUuidFederalTaxDetailsRequestBody,
    ) -> Result<
        crate::types::PutCompaniesCompanyIdOrUuidFederalTaxDetailsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/federal_tax_details"
                    .replace("{company_id_or_uuid}", &company_id_or_uuid)
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
