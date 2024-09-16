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

    #[doc = "GET Employees\n\nRetrieves the list of active employees currently provisioned within \
             the application. The fields retrieved depend on the employee scopes that you have \
             access to for your access token or API key. The only guarenteed fields include id, \
             personalEmail, and roleState.\n\nFor optimal performance, ensure pagination is used \
             via our limit and offset parameters. Pagination should be set to a maximum of \
             100.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Sets a limit on the returned \
             values\n- `offset: Option<i64>`: Offsets the returned values\n\n```rust,no_run\nasync \
             fn example_employees_get() -> anyhow::Result<()> {\n    let client = \
             rippling_base_api::Client::new_from_env();\n    let result: \
             Vec<rippling_base_api::types::Employee> = client\n        .employees()\n        \
             .get(Some(4 as i64), Some(4 as i64))\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::types::Employee>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/employees"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = offset {
            query_params.push(("offset", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "GET Employee\n\nRetrieves the information for a single employee based on the scopes that your API key or access token have access to.\n\n**Parameters:**\n\n- `employee_id: &'astr`: Unique identifier for the employee within Rippling. (required)\n\n```rust,no_run\nasync fn example_employees_get_id() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: rippling_base_api::types::Employee = client.employees().get_id(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id<'a>(
        &'a self,
        employee_id: &'a str,
    ) -> Result<crate::types::Employee, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/employees/{employeeId}".replace("{employeeId}", employee_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
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

    #[doc = "GET Employees (Including Terminated)\n\nThis endpoint is similar to the active \
             employees endpoint, but instead, it includes both active and terminated \
             employees.\n\nFor optimal performance, ensure pagination is used via our limit and \
             offset parameters. Pagination should be set to a maximum of \
             100.\n\n**Parameters:**\n\n- `ein: Option<i64>`: Employer identification number, also \
             known as the Federal Emplower Identification Number or the Federal Tax Identification \
             Number.\n- `limit: Option<i64>`: Sets a limit on the returned values\n- `offset: \
             Option<i64>`: Offsets the returned values\n- `send_all_roles: Option<bool>`: For \
             integrations that rely on provisioning, this parameter can be used to identify non \
             provisioned roles for compliance purposes. TRUE will return every employee from the \
             company (bypassing any access rules).\n\n```rust,no_run\nasync fn \
             example_employees_get_include_terminated() -> anyhow::Result<()> {\n    let client = \
             rippling_base_api::Client::new_from_env();\n    let result: \
             Vec<rippling_base_api::types::Employee> = client\n        .employees()\n        \
             .get_include_terminated(Some(4 as i64), Some(4 as i64), Some(4 as i64), \
             Some(false))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_include_terminated<'a>(
        &'a self,
        ein: Option<i64>,
        limit: Option<i64>,
        offset: Option<i64>,
        send_all_roles: Option<bool>,
    ) -> Result<Vec<crate::types::Employee>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/employees/include_terminated"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = ein {
            query_params.push(("EIN", format!("{}", p)));
        }

        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = offset {
            query_params.push(("offset", format!("{}", p)));
        }

        if let Some(p) = send_all_roles {
            query_params.push(("send_all_roles", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
