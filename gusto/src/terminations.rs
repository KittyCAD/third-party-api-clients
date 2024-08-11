use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Terminations {
    pub client: Client,
}

impl Terminations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get terminations for an employee\n\nTerminations are created whenever an employee is \
             scheduled to leave the company. The only things required are an effective date (their \
             last day of work) and whether they should receive their wages in a one-off \
             termination payroll or with the rest of the company.\n\nNote that some states require \
             employees to receive their final wages within 24 hours (unless they consent \
             otherwise,) in which case running a one-off payroll may be the only \
             option.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee \
             (required)\n\n```rust,no_run\nasync fn \
             example_terminations_get_employees_employee_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Termination> \
             = client\n        .terminations()\n        \
             .get_employees_employee_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_employees_employee_id<'a>(
        &'a self,
        employee_id: &'a str,
    ) -> Result<Vec<crate::types::Termination>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/terminations".replace("{employee_id}", employee_id)
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

    #[doc = "Create an employee termination\n\nTerminations are created whenever an employee is scheduled to leave the company. The only things required are an effective date (their last day of work) and whether they should receive their wages in a one-off termination payroll or with the rest of the company.\n\nNote that some states require employees to receive their final wages within 24 hours (unless they consent otherwise,) in which case running a one-off payroll may be the only option.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The ID of the employee (required)\n\n```rust,no_run\nasync fn example_terminations_post_employees_employee_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Termination = client\n        .terminations()\n        .post_employees_employee_id(\n            \"some-string\",\n            &gusto_api::types::PostEmployeesEmployeeIdTerminationsRequestBody {\n                effective_date: Some(chrono::Utc::now().date().naive_utc()),\n                run_termination_payroll: Some(false),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_employees_employee_id<'a>(
        &'a self,
        employee_id: &'a str,
        body: &crate::types::PostEmployeesEmployeeIdTerminationsRequestBody,
    ) -> Result<crate::types::Termination, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/terminations".replace("{employee_id}", employee_id)
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
}
