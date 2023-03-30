use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct AdminsBeta {
    pub client: Client,
}

impl AdminsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get all the admins at a company\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nReturns a list of all the admins at a company\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n\n```rust,no_run\nasync fn example_admins_beta_get_companies_company_id_admins() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Admin> = client\n        .admins_beta()\n        .get_companies_company_id_admins(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_admins<'a>(
        &'a self,
        company_id: &'a str,
    ) -> Result<Vec<crate::types::Admin>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/admins".replace("{company_id}", company_id)
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

    #[doc = "Create an admin for the company.\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nCreates a new admin for a company. If the email matches an existing user, this will create an admin account for the current user. Otherwise, this will create a new user.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n\n```rust,no_run\nasync fn example_admins_beta_post_companies_company_id_admins() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Admin = client\n        .admins_beta()\n        .post_companies_company_id_admins(\n            \"some-string\",\n            &gusto_api::types::PostCompaniesCompanyIdAdminsRequestBody {\n                first_name: \"some-string\".to_string(),\n                last_name: \"some-string\".to_string(),\n                email: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id_admins<'a>(
        &'a self,
        company_id: &'a str,
        body: &crate::types::PostCompaniesCompanyIdAdminsRequestBody,
    ) -> Result<crate::types::Admin, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/admins".replace("{company_id}", company_id)
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
