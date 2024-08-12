use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct TimeOffRequests {
    pub client: Client,
}

impl TimeOffRequests {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get time off requests for a company\n\nGet all time off requests, past and present, \
             for a company.\n\nIn order to reduce the number of time off requests returned in a \
             single response, or to retrieve time off requests from a time period of interest, you \
             may use the `start_date` and `end_date` parameters.\n\nYou may provide both or either \
             parameters to scope the returned data. For \
             example:\n\n`?start_date='2019-01-01'`\n\nReturns all time off requests where the \
             request start date is equal to or after January 1, \
             2019.\n\n`?end_date='2019-01-01'`\n\nReturns all time off requests where the request \
             end date is equal to or before January 1, \
             2019.\n\n`?start_date='2019-05-01'&end_date='2019-08-31'`\n\nReturns all time off \
             requests where the request start date is equal to or after May 1, 2019 and the \
             request end date is equal to or before August 31, 2019.\n\n\n**Parameters:**\n\n- \
             `company_id: &'astr`: The ID of the company (required)\n- `end_date: \
             Option<chrono::NaiveDate>`: Filter time off requests where the request end date is \
             equal to or after this parameter\n- `start_date: Option<chrono::NaiveDate>`: Filter \
             time off requests where the request start date is equal to or after this \
             parameter\n\n```rust,no_run\nasync fn \
             example_time_off_requests_get_companies_company_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             Vec<gusto_api::types::TimeOffRequest> = client\n        .time_off_requests()\n        \
             .get_companies_company_id(\n            \"some-string\",\n            \
             Some(chrono::Utc::now().date_naive()),\n            \
             Some(chrono::Utc::now().date_naive()),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id<'a>(
        &'a self,
        company_id: &'a str,
        end_date: Option<chrono::NaiveDate>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<Vec<crate::types::TimeOffRequest>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/time_off_requests".replace("{company_id}", company_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = end_date {
            query_params.push(("end_date", format!("{}", p)));
        }

        if let Some(p) = start_date {
            query_params.push(("start_date", format!("{}", p)));
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

    #[doc = "Get a specific time off request\n\nDetails of a single time off \
             request\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company \
             (required)\n- `time_off_request_id: &'astr`: The ID of the time off request \
             (required)\n\n```rust,no_run\nasync fn \
             example_time_off_requests_get_companies_company_id_id() -> anyhow::Result<()> {\n    \
             let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::TimeOffRequest = \
             client\n        .time_off_requests()\n        \
             .get_companies_company_id_id(\"some-string\", \"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_id<'a>(
        &'a self,
        company_id: &'a str,
        time_off_request_id: &'a str,
    ) -> Result<crate::types::TimeOffRequest, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/time_off_requests/{time_off_request_id}"
                    .replace("{company_id}", company_id)
                    .replace("{time_off_request_id}", time_off_request_id)
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
}
