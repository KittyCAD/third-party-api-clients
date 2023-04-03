use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CompanyBankAccountsBeta {
    pub client: Client,
}

impl CompanyBankAccountsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get all company bank accounts\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nReturns all company bank accounts\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn example_company_bank_accounts_beta_get_companies_company_id_bank_accounts(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::CompanyBankAccount> = client\n        .company_bank_accounts_beta()\n        .get_companies_company_id_bank_accounts(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_bank_accounts<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
    ) -> Result<Vec<crate::types::CompanyBankAccount>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/bank_accounts"
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

    #[doc = "Create a company bank account\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nCreate a company bank account. The new bank account will replace an existing bank account as the default company funding method.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn example_company_bank_accounts_beta_post_companies_company_id_bank_accounts(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result : gusto_api::types::CompanyBankAccount = client . company_bank_accounts_beta () . post_companies_company_id_bank_accounts (\"some-string\" , & gusto_api::types::PostCompaniesCompanyIdBankAccountsRequestBody { routing_number : Some (\"some-string\" . to_string ()) , account_number : Some (\"some-string\" . to_string ()) , account_type : Some (gusto_api::types::PostCompaniesCompanyIdBankAccountsRequestBodyAccountType :: Checking) }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id_bank_accounts<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        body: &crate::types::PostCompaniesCompanyIdBankAccountsRequestBody,
    ) -> Result<crate::types::CompanyBankAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/bank_accounts"
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

    #[doc = "Verify a company bank account\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nVerify a company bank account by confirming the two micro-deposits sent to the bank account. Note that the order of the two deposits specified in request parameters does not matter.\n\n**Parameters:**\n\n- `bank_account_uuid: &'astr`: Bank account UUID (required)\n- `company_id_or_uuid: &'astr`: The ID or UUID of the bank account (required)\n\n```rust,no_run\nasync fn example_company_bank_accounts_beta_put_companies_company_id_bank_accounts_verify(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::CompanyBankAccount = client\n        .company_bank_accounts_beta()\n        .put_companies_company_id_bank_accounts_verify(\n            \"some-string\",\n            \"some-string\",\n            &gusto_api::types::PutCompaniesCompanyIdBankAccountsVerifyRequestBody {\n                deposit_1: Some(3.14 as f64),\n                deposit_2: Some(3.14 as f64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_bank_accounts_verify<'a>(
        &'a self,
        bank_account_uuid: &'a str,
        company_id_or_uuid: &'a str,
        body: &crate::types::PutCompaniesCompanyIdBankAccountsVerifyRequestBody,
    ) -> Result<crate::types::CompanyBankAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/bank_accounts/{bank_account_uuid}/verify"
                    .replace("{bank_account_uuid}", bank_account_uuid)
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
