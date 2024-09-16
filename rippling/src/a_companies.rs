use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ACompanies {
    pub client: Client,
}

impl ACompanies {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "GET Current Company\n\nReturns the currently accessible company for the given token. \
             Please note, the returned fields depend on the scopes that are enabled for your \
             access token or API key.\n\n```rust,no_run\nasync fn \
             example_a_companies_get_companies() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: rippling_api::types::Company = \
             client.a_companies().get_companies().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies<'a>(
        &'a self,
    ) -> Result<crate::types::Company, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/companies/current"
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

    #[doc = "GET Departments\n\nReturns a list of departments for the given company.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Sets a limit on the returned values\n- `offset: Option<i64>`: Offsets the returned values\n\n```rust,no_run\nasync fn example_a_companies_get_departments() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: Vec<Option<rippling_api::types::Department>> = client\n        .a_companies()\n        .get_departments(Some(4 as i64), Some(4 as i64))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_departments<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Option<crate::types::Department>>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/departments"),
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

    #[doc = "GET Work Locations\n\nReturns the list of work locations for a given \
             company.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Sets a limit on the returned \
             values\n- `offset: Option<i64>`: Offsets the returned values\n\n```rust,no_run\nasync \
             fn example_a_companies_get_work_locations() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             Vec<rippling_api::types::WorkLocation> = client\n        .a_companies()\n        \
             .get_work_locations(Some(4 as i64), Some(4 as i64))\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_work_locations<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::types::WorkLocation>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/work_locations"),
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

    #[doc = "GET Custom Fields\n\nReturns the custom fields for the given \
             company.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Sets a limit on the returned \
             values\n- `offset: Option<i64>`: Offsets the returned values\n\n```rust,no_run\nasync \
             fn example_a_companies_get_custom_fields() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             Vec<rippling_api::types::CustomFields> = client\n        .a_companies()\n        \
             .get_custom_fields(Some(4 as i64), Some(4 as i64))\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_fields<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::types::CustomFields>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/custom_fields"),
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

    #[doc = "GET Teams\n\nRetrieves the list of teams for the company.\n\n**Parameters:**\n\n- \
             `limit: Option<i64>`: Sets a limit on the returned values\n- `offset: Option<i64>`: \
             Offsets the returned values\n\n```rust,no_run\nasync fn \
             example_a_companies_get_teams() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: Vec<rippling_api::types::Team> \
             = client\n        .a_companies()\n        .get_teams(Some(4 as i64), Some(4 as \
             i64))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_teams<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::types::Team>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/teams"),
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

    #[doc = "GET Levels\n\nRetrieves the levels for the company. Levels are set positions for an \
             organization, such as Manager, or Executive.\n\n**Parameters:**\n\n- `limit: \
             Option<i64>`: Sets a limit on the returned values\n- `offset: Option<i64>`: Offsets \
             the returned values\n\n```rust,no_run\nasync fn example_a_companies_get_levels() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: Vec<rippling_api::types::Level> = client\n        .a_companies()\n        \
             .get_levels(Some(4 as i64), Some(4 as i64))\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_levels<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<crate::types::Level>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/levels"),
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

    #[doc = "GET Company Leave Types\n\nRetrieves the current company leave types. The query can \
             be filtered by managedBy field.\n\n**Parameters:**\n\n- `managed_by: \
             Option<String>`\n\n```rust,no_run\nasync fn \
             example_a_companies_get_company_leave_types() -> anyhow::Result<()> {\n    let client \
             = rippling_api::Client::new_from_env();\n    let result: \
             Vec<rippling_api::types::CompanyLeaveType> = client\n        .a_companies()\n        \
             .get_company_leave_types(Some(\"some-string\".to_string()))\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_company_leave_types<'a>(
        &'a self,
        managed_by: Option<String>,
    ) -> Result<Vec<crate::types::CompanyLeaveType>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/company_leave_types"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = managed_by {
            query_params.push(("managedBy", p));
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

    #[doc = "GET Company Activity\n\nRetrieves the activity for a given company.\n\nThe most reliable method to ingest all activity from Rippling is to use a pagination cursor via the 'next' parameter. This will ensure that events are not skipped or duplicated due to the lack of timestamp precision.\n\nThe general sequence of steps to leverage the next parameter:\n\n1. Issue an initial request using startDate with a value set to some date in the last 90 days\n2. Retrieve the next page of events through the next value from the response data.\n3. Issue the paginated request\n4. Retrieve the next page of events through the next value from the response data\n5. Pause and repeat the previous step\n\n\n**Parameters:**\n\n- `end_date: Option<chrono::NaiveDate>`: Timestamp to list activity before (inclusive).\n- `limit: Option<String>`: Specifies the number of results to page (maximum: 1000) (default: 1000)\n- `next: Option<String>`: Specifies the pagination cursor to the next page\n- `start_date: Option<chrono::NaiveDate>`: Timestamp to list activity after (inclusive). This should be less than 90 days from now. Defaults to 90 days.\n\n```rust,no_run\nasync fn example_a_companies_get_company_activity() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::GetCompanyActivityResponse = client\n        .a_companies()\n        .get_company_activity(\n            Some(chrono::Utc::now().date_naive()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date_naive()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_company_activity<'a>(
        &'a self,
        end_date: Option<chrono::NaiveDate>,
        limit: Option<String>,
        next: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::GetCompanyActivityResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/company_activity"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = end_date {
            query_params.push(("endDate", format!("{}", p)));
        }

        if let Some(p) = limit {
            query_params.push(("limit", p));
        }

        if let Some(p) = next {
            query_params.push(("next", p));
        }

        if let Some(p) = start_date {
            query_params.push(("startDate", format!("{}", p)));
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
