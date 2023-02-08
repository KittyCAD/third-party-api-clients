use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Benefits {
    pub client: Client,
}

impl Benefits {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get all benefits supported by Gusto\n\nReturns all benefits supported by Gusto.\n\nThe benefit object in Gusto contains high level information about a particular benefit type and its tax considerations. When companies choose to offer a benefit, they are creating a Company Benefit object associated with a particular benefit.\n\n```rust,no_run\nasync fn example_benefits_get() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::SupportedBenefit> = client.benefits().get().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::SupportedBenefit>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "v1/benefits"),
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

    #[doc = "Get a supported benefit by ID\n\nReturns a benefit supported by Gusto.\n\nThe benefit \
             object in Gusto contains high level information about a particular benefit type and \
             its tax considerations. When companies choose to offer a benefit, they are creating a \
             Company Benefit object associated with a particular benefit.\n\n**Parameters:**\n\n- \
             `benefit_id: &'astr`: The ID of the benefit (required)\n\n```rust,no_run\nasync fn \
             example_benefits_get_id() -> anyhow::Result<()> {\n    let client =\n        \
             gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::SupportedBenefit \
             = client.benefits().get_id(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id<'a>(
        &'a self,
        benefit_id: &'a str,
    ) -> Result<crate::types::SupportedBenefit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/benefits/{benefit_id}".replace("{benefit_id}", benefit_id)
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

    #[doc = "Get benefits for a company\n\nCompany benefits represent the benefits that a company \
             is offering to employees. This ties together a particular supported benefit with the \
             company-specific information for the offering of that benefit.\n\nNote that company \
             benefits can be deactivated only when no employees are \
             enrolled.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company \
             (required)\n\n```rust,no_run\nasync fn \
             example_benefits_get_companies_company_id_company() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             Vec<gusto_api::types::CompanyBenefit> = client\n        .benefits()\n        \
             .get_companies_company_id_company(\"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_company<'a>(
        &'a self,
        company_id: &'a str,
    ) -> Result<Vec<crate::types::CompanyBenefit>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/company_benefits".replace("{company_id}", company_id)
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

    #[doc = "Create a company benefit\n\nCompany benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.\n\nNote that company benefits can be deactivated only when no employees are enrolled.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n\n```rust,no_run\nasync fn example_benefits_post_companies_company_id_company() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::CompanyBenefit = client\n        .benefits()\n        .post_companies_company_id_company(\n            \"some-string\",\n            &gusto_api::types::PostCompaniesCompanyIdCompanyBenefitsRequestBody {\n                benefit_id: 3.14 as f64,\n                active: Some(true),\n                description: \"some-string\".to_string(),\n                responsible_for_employer_taxes: Some(false),\n                responsible_for_employee_w_2: Some(false),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id_company<'a>(
        &'a self,
        company_id: &'a str,
        body: &crate::types::PostCompaniesCompanyIdCompanyBenefitsRequestBody,
    ) -> Result<crate::types::CompanyBenefit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/company_benefits".replace("{company_id}", company_id)
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

    #[doc = "Get a company benefit\n\nCompany benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.\n\nNote that company benefits can be deactivated only when no employees are enrolled.\n\n**Parameters:**\n\n- `company_benefit_id: &'astr`: The ID of the company benefit (required)\n\n```rust,no_run\nasync fn example_benefits_get_company_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::CompanyBenefit = client\n        .benefits()\n        .get_company_company_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_company_company_id<'a>(
        &'a self,
        company_benefit_id: &'a str,
    ) -> Result<crate::types::CompanyBenefit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/company_benefits/{company_benefit_id}"
                    .replace("{company_benefit_id}", company_benefit_id)
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

    #[doc = "Update a company benefit\n\nCompany benefits represent the benefits that a company is offering to employees. This ties together a particular supported benefit with the company-specific information for the offering of that benefit.\n\nNote that company benefits can be deactivated only when no employees are enrolled.\n\n**Parameters:**\n\n- `company_benefit_id: &'astr`: The ID of the company benefit (required)\n\n```rust,no_run\nasync fn example_benefits_put_company_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::CompanyBenefit = client\n        .benefits()\n        .put_company_company_id(\n            \"some-string\",\n            &gusto_api::types::PutCompanyBenefitsCompanyBenefitIdRequestBody {\n                version: \"some-string\".to_string(),\n                active: Some(true),\n                description: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_company_company_id<'a>(
        &'a self,
        company_benefit_id: &'a str,
        body: &crate::types::PutCompanyBenefitsCompanyBenefitIdRequestBody,
    ) -> Result<crate::types::CompanyBenefit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/company_benefits/{company_benefit_id}"
                    .replace("{company_benefit_id}", company_benefit_id)
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

    #[doc = "Get an employee's benefits\n\nEmployee benefits represent an employee enrolled in a \
             particular company benefit. It includes information specific to that employee’s \
             enrollment.\n\nReturns an array of all employee benefits for this \
             employee\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee \
             (required)\n\n```rust,no_run\nasync fn \
             example_benefits_get_employees_employee_id_employee() -> anyhow::Result<()> {\n    \
             let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             Vec<gusto_api::types::EmployeeBenefit> = client\n        .benefits()\n        \
             .get_employees_employee_id_employee(\"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_employees_employee_id_employee<'a>(
        &'a self,
        employee_id: &'a str,
    ) -> Result<Vec<crate::types::EmployeeBenefit>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/employee_benefits"
                    .replace("{employee_id}", employee_id)
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

    #[doc = "Create an employee benefit\n\nEmployee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee (required)\n\n```rust,no_run\nasync fn example_benefits_post_employees_employee_id_employee() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result : gusto_api::types::EmployeeBenefit = client . benefits () . post_employees_employee_id_employee (\"some-string\" , & gusto_api::types::PostEmployeesEmployeeIdEmployeeBenefitsRequestBody { company_benefit_id : 3.14 as f64 , active : Some (true) , employee_deduction : Some (\"some-string\" . to_string ()) , company_contribution : Some (\"some-string\" . to_string ()) , employee_deduction_annual_maximum : Some (\"some-string\" . to_string ()) , company_contribution_annual_maximum : Some (\"some-string\" . to_string ()) , limit_option : Some (\"some-string\" . to_string ()) , deduct_as_percentage : Some (false) , contribute_as_percentage : Some (true) , catch_up : Some (false) , coverage_amount : Some (\"some-string\" . to_string ()) , deduction_reduces_taxable_income : Some (gusto_api::types::PostEmployeesEmployeeIdEmployeeBenefitsRequestBodyDeductionReducesTaxableIncome :: DoesNotReduceTaxableIncome) , coverage_salary_multiplier : Some (\"some-string\" . to_string ()) }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_employees_employee_id_employee<'a>(
        &'a self,
        employee_id: &'a str,
        body: &crate::types::PostEmployeesEmployeeIdEmployeeBenefitsRequestBody,
    ) -> Result<crate::types::EmployeeBenefit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/employee_benefits"
                    .replace("{employee_id}", employee_id)
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

    #[doc = "Year-to-date Benefit Amounts from Different Company\n\nYear-to-date benefit amounts from a different company represents the amount of money added to an employees plan during a current year, made outside of the current contribution when they were employed at a different company.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee (required)\n\n```rust,no_run\nasync fn example_benefits_post_employee_ytd_amounts_from_different_company() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .benefits()\n        .post_employee_ytd_amounts_from_different_company(\n            \"some-string\",\n            &gusto_api::types::PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequestBody {\n                benefit_id: 3.14 as f64,\n                tax_year: 3.14 as f64,\n                ytd_employee_deduction_amount: \"some-string\".to_string(),\n                ytd_company_contribution_amount: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_employee_ytd_amounts_from_different_company<'a>(
        &'a self,
        employee_id: &'a str,
        body: &crate::types::PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/ytd_benefit_amounts_from_different_company"
                    .replace("{employee_id}", employee_id)
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

    #[doc = "Get an employee benefit\n\nEmployee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.\n\n**Parameters:**\n\n- `employee_benefit_id: &'astr`: The ID of the employee benefit (required)\n\n```rust,no_run\nasync fn example_benefits_get_employee_employee_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::EmployeeBenefit = client\n        .benefits()\n        .get_employee_employee_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_employee_employee_id<'a>(
        &'a self,
        employee_benefit_id: &'a str,
    ) -> Result<crate::types::EmployeeBenefit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employee_benefits/{employee_benefit_id}"
                    .replace("{employee_benefit_id}", employee_benefit_id)
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

    #[doc = "Update an employee benefit\n\nEmployee benefits represent an employee enrolled in a particular company benefit. It includes information specific to that employee’s enrollment.\n\n**Parameters:**\n\n- `employee_benefit_id: &'astr`: The ID of the employee benefit (required)\n\n```rust,no_run\nasync fn example_benefits_put_employee_employee_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::EmployeeBenefit = client\n        .benefits()\n        .put_employee_employee_id(\n            \"some-string\",\n            &gusto_api::types::PutEmployeeBenefitsEmployeeBenefitIdRequestBody {\n                version: \"some-string\".to_string(),\n                active: Some(false),\n                employee_deduction: Some(\"some-string\".to_string()),\n                company_contribution: Some(\"some-string\".to_string()),\n                employee_deduction_annual_maximum: Some(\"some-string\".to_string()),\n                company_contribution_annual_maximum: Some(\"some-string\".to_string()),\n                limit_option: Some(\"some-string\".to_string()),\n                deduct_as_percentage: Some(false),\n                contribute_as_percentage: Some(false),\n                catch_up: Some(true),\n                coverage_amount: Some(\"some-string\".to_string()),\n                deduction_reduces_taxable_income: Some(\n                    gusto_api::types::DeductionReducesTaxableIncome::DoesNotReduceTaxableIncome,\n                ),\n                coverage_salary_multiplier: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_employee_employee_id<'a>(
        &'a self,
        employee_benefit_id: &'a str,
        body: &crate::types::PutEmployeeBenefitsEmployeeBenefitIdRequestBody,
    ) -> Result<crate::types::EmployeeBenefit, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employee_benefits/{employee_benefit_id}"
                    .replace("{employee_benefit_id}", employee_benefit_id)
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

    #[doc = "Delete an employee benefit\n\nEmployee benefits represent an employee enrolled in a \
             particular company benefit. It includes information specific to that employee’s \
             enrollment.\n\n**Parameters:**\n\n- `employee_benefit_id: &'astr`: The ID of the \
             employee benefit (required)\n\n```rust,no_run\nasync fn \
             example_benefits_delete_employee_employee_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client\n        .benefits()\n        \
             .delete_employee_employee_id(\"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_employee_employee_id<'a>(
        &'a self,
        employee_benefit_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employee_benefits/{employee_benefit_id}"
                    .replace("{employee_benefit_id}", employee_benefit_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
