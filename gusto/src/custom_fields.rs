use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CustomFields {
    pub client: Client,
}

impl CustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get an employee's custom fields\n\nReturns a list of the employee's custom \
             fields.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee \
             (required)\n\n```rust,no_run\nasync fn \
             example_custom_fields_get_employees_employee_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             gusto_api::types::GetEmployeesEmployeeIdCustomFieldsResponse = client\n        \
             .custom_fields()\n        .get_employees_employee_id(\"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_employees_employee_id<'a>(
        &'a self,
        employee_id: &'a str,
    ) -> Result<crate::types::GetEmployeesEmployeeIdCustomFieldsResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/custom_fields".replace("{employee_id}", employee_id)
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

    #[doc = "Get the custom fields of a company\n\nReturns a list of the custom fields of the \
             company. Useful when you need to know the schema of custom fields for an entire \
             company\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company \
             (required)\n\n```rust,no_run\nasync fn \
             example_custom_fields_get_companies_company_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             gusto_api::types::GetCompaniesCompanyIdCustomFieldsResponse = client\n        \
             .custom_fields()\n        .get_companies_company_id(\"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id<'a>(
        &'a self,
        company_id: &'a str,
    ) -> Result<crate::types::GetCompaniesCompanyIdCustomFieldsResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/custom_fields".replace("{company_id}", company_id)
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
}
