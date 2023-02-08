use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Employees {
    pub client: Client,
}

impl Employees {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get an employee\n\nGet an employee.\n\n**Parameters:**\n\n- `employee_id_or_uuid: &'astr`: The ID or UUID of the employee (required)\n- `include: Option<Vec<crate::types::Include>>`: Include the requested attribute(s) in each employee response\n\n```rust,no_run\nasync fn example_employees_get() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Employee = client\n        .employees()\n        .get(\n            \"some-string\",\n            Some(vec![gusto_api::types::GetInclude::CustomFields]),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        employee_id_or_uuid: &'a str,
        include: Option<Vec<crate::types::GetInclude>>,
    ) -> Result<crate::types::Employee, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id_or_uuid}"
                    .replace("{employee_id_or_uuid}", employee_id_or_uuid)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = Vec::new();
        if let Some(p) = include {
            query_params.push(("include", itertools::join(p, ",")));
        }

        req = req.query(&query_params);
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

    #[doc = "Update an employee\n\nUpdate an employee.\n\n**Parameters:**\n\n- `employee_id_or_uuid: &'astr`: The ID or UUID of the employee (required)\n\n```rust,no_run\nasync fn example_employees_put() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Employee = client\n        .employees()\n        .put(\n            \"some-string\",\n            &gusto_api::types::PutEmployeesRequestBody {\n                version: \"some-string\".to_string(),\n                first_name: Some(\"some-string\".to_string()),\n                middle_initial: Some(\"some-string\".to_string()),\n                last_name: Some(\"some-string\".to_string()),\n                date_of_birth: Some(chrono::Utc::now().date().naive_utc()),\n                email: Some(\"some-string\".to_string()),\n                ssn: Some(\"some-string\".to_string()),\n                two_percent_shareholder: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put<'a>(
        &'a self,
        employee_id_or_uuid: &'a str,
        body: &crate::types::PutEmployeesRequestBody,
    ) -> Result<crate::types::Employee, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id_or_uuid}"
                    .replace("{employee_id_or_uuid}", employee_id_or_uuid)
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

    #[doc = "Get employees of a company\n\nGet all of the employees, onboarding, active and terminated, for a given company.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n- `include: Option<Vec<crate::types::GetCompaniesCompanyIdInclude>>`: Include the requested attribute(s) in each employee response\n- `page: Option<f64>`: The page that is requested. When unspecified, will load all employees.\n- `per: Option<f64>`: Number of employees per page. When unspecified, will default to 25\n- `terminated: Option<bool>`: Filters employees by the provided boolean\n\n```rust,no_run\nasync fn example_employees_get_companies_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Employee> = client\n        .employees()\n        .get_companies_company_id(\n            \"some-string\",\n            Some(vec![\n                gusto_api::types::GetCompaniesCompanyIdInclude::CustomFields,\n            ]),\n            Some(3.14 as f64),\n            Some(3.14 as f64),\n            Some(false),\n            &gusto_api::types::GetCompaniesCompanyIdEmployeesRequestBody {},\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        include: Option<Vec<crate::types::GetCompaniesCompanyIdInclude>>,
        page: Option<f64>,
        per: Option<f64>,
        terminated: Option<bool>,
        body: &crate::types::GetCompaniesCompanyIdEmployeesRequestBody,
    ) -> Result<Vec<crate::types::Employee>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/employees"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = Vec::new();
        if let Some(p) = include {
            query_params.push(("include", itertools::join(p, ",")));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
        }

        if let Some(p) = per {
            query_params.push(("per", format!("{}", p)));
        }

        if let Some(p) = terminated {
            query_params.push(("terminated", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "Create an employee\n\nCreate an employee.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn example_employees_post() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Employee = client\n        .employees()\n        .post(\n            \"some-string\",\n            &gusto_api::types::PostEmployeesRequestBody {\n                first_name: Some(\"some-string\".to_string()),\n                middle_initial: Some(\"some-string\".to_string()),\n                last_name: Some(\"some-string\".to_string()),\n                date_of_birth: Some(chrono::Utc::now().date().naive_utc()),\n                email: Some(\"some-string\".to_string()),\n                ssn: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        body: &crate::types::PostEmployeesRequestBody,
    ) -> Result<crate::types::Employee, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/employees"
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

    #[doc = "Get an employee's home address\n\nThe home address of an employee is used to determine certain tax information about them. Addresses are geocoded on create and update to ensure validity.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee (required)\n\n```rust,no_run\nasync fn example_employees_get_id_home_address() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Location = client\n        .employees()\n        .get_id_home_address(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id_home_address<'a>(
        &'a self,
        employee_id: &'a str,
    ) -> Result<crate::types::Location, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/home_address".replace("{employee_id}", employee_id)
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

    #[doc = "Update an employee's home address\n\nThe home address of an employee is used to determine certain tax information about them. Addresses are geocoded on create and update to ensure validity.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee (required)\n\n```rust,no_run\nasync fn example_employees_put_id_home_address() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Location = client\n        .employees()\n        .put_id_home_address(\n            \"some-string\",\n            &gusto_api::types::PutEmployeesEmployeeIdHomeAddressRequestBody {\n                version: \"some-string\".to_string(),\n                street_1: Some(\"some-string\".to_string()),\n                street_2: Some(\"some-string\".to_string()),\n                city: Some(\"some-string\".to_string()),\n                state: Some(\"some-string\".to_string()),\n                zip: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_id_home_address<'a>(
        &'a self,
        employee_id: &'a str,
        body: &crate::types::PutEmployeesEmployeeIdHomeAddressRequestBody,
    ) -> Result<crate::types::Location, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/home_address".replace("{employee_id}", employee_id)
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
