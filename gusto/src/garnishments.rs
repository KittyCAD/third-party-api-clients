use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Garnishments {
    pub client: Client,
}

impl Garnishments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get garnishments for an employee\n\nGarnishments, or employee deductions, are fixed \
             amounts or percentages deducted from an employee’s pay. They can be deducted a \
             specific number of times or on a recurring basis. Garnishments can also have maximum \
             deductions on a yearly or per-pay-period bases. Common uses for garnishments are \
             court-ordered payments for child support or back taxes. Some companies provide loans \
             to their employees that are repaid via garnishments.\n\n**Parameters:**\n\n- \
             `employee_id: &'astr`: The ID of the employee to which the garnishment belongs \
             (required)\n\n```rust,no_run\nasync fn \
             example_garnishments_get_employees_employee_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Garnishment> \
             = client\n        .garnishments()\n        \
             .get_employees_employee_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_employees_employee_id<'a>(
        &'a self,
        employee_id: &'a str,
    ) -> Result<Vec<crate::types::Garnishment>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/garnishments".replace("{employee_id}", employee_id)
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

    #[doc = "Create a garnishment\n\nGarnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee to which the garnishment belongs (required)\n\n```rust,no_run\nasync fn example_garnishments_post_employees_employee_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Garnishment = client\n        .garnishments()\n        .post_employees_employee_id(\n            \"some-string\",\n            &gusto_api::types::PostEmployeesEmployeeIdGarnishmentsRequestBody {\n                active: Some(false),\n                amount: 3.14 as f64,\n                description: \"some-string\".to_string(),\n                court_ordered: false,\n                times: Some(4 as i64),\n                recurring: Some(false),\n                annual_maximum: Some(3.14 as f64),\n                pay_period_maximum: Some(3.14 as f64),\n                deduct_as_percentage: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_employees_employee_id<'a>(
        &'a self,
        employee_id: &'a str,
        body: &crate::types::PostEmployeesEmployeeIdGarnishmentsRequestBody,
    ) -> Result<crate::types::Garnishment, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/garnishments".replace("{employee_id}", employee_id)
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

    #[doc = "Get a garnishment\n\nGarnishments, or employee deductions, are fixed amounts or \
             percentages deducted from an employee’s pay. They can be deducted a specific number \
             of times or on a recurring basis. Garnishments can also have maximum deductions on a \
             yearly or per-pay-period bases. Common uses for garnishments are court-ordered \
             payments for child support or back taxes. Some companies provide loans to their \
             employees that are repaid via garnishments.\n\n**Parameters:**\n\n- `garnishment_id: \
             &'astr`: The ID of the garnishment (required)\n\n```rust,no_run\nasync fn \
             example_garnishments_get_id() -> anyhow::Result<()> {\n    let client =\n        \
             gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::Garnishment = \
             client.garnishments().get_id(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id<'a>(
        &'a self,
        garnishment_id: &'a str,
    ) -> Result<crate::types::Garnishment, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/garnishments/{garnishment_id}".replace("{garnishment_id}", garnishment_id)
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

    #[doc = "Update a garnishment\n\nGarnishments, or employee deductions, are fixed amounts or percentages deducted from an employee’s pay. They can be deducted a specific number of times or on a recurring basis. Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common uses for garnishments are court-ordered payments for child support or back taxes. Some companies provide loans to their employees that are repaid via garnishments.\n\n**Parameters:**\n\n- `garnishment_id: &'astr`: The ID of the garnishment (required)\n\n```rust,no_run\nasync fn example_garnishments_put_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Garnishment = client\n        .garnishments()\n        .put_id(\n            \"some-string\",\n            &gusto_api::types::PutGarnishmentsGarnishmentIdRequestBody {\n                active: Some(false),\n                amount: Some(3.14 as f64),\n                description: Some(\"some-string\".to_string()),\n                court_ordered: Some(false),\n                times: Some(4 as i64),\n                recurring: Some(false),\n                annual_maximum: Some(3.14 as f64),\n                pay_period_maximum: Some(3.14 as f64),\n                deduct_as_percentage: Some(true),\n                version: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_id<'a>(
        &'a self,
        garnishment_id: &'a str,
        body: &crate::types::PutGarnishmentsGarnishmentIdRequestBody,
    ) -> Result<crate::types::Garnishment, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/garnishments/{garnishment_id}".replace("{garnishment_id}", garnishment_id)
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
