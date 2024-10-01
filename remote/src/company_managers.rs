use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CompanyManagers {
    pub client: Client,
}

impl CompanyManagers {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List Company Managers\n\nList all company managers of an integration. If filtered by \
             the company_id param,\nit lists only company managers belonging to the specified \
             company.\n\n\n**Parameters:**\n\n- `company_id: Option<String>`: A Company ID to \
             filter the results (only applicable for Integration Partners).\n- `page: \
             Option<i64>`: Starts fetching records after the given page\n- `page_size: \
             Option<i64>`: Change the amount of records returned per page, defaults to 20, limited \
             to 100\n\n```rust,no_run\nasync fn example_company_managers_get_index() -> \
             anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let \
             result: remote_api::types::CompanyManagersResponse = client\n        \
             .company_managers()\n        .get_index(\n            \
             Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 \
             as i64),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_index<'a>(
        &'a self,
        company_id: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::CompanyManagersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "v1/company-managers"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = company_id {
            query_params.push(("company_id", p));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create and invite a Company Manager\n\nCreate a Company Manager and sends the invitation email for signing in to the Remote Platform.\n\n```rust,no_run\nasync fn example_company_managers_post_create() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::CompanyManagerCreatedResponse = client\n        .company_managers()\n        .post_create(&remote_api::types::CompanyManagerParams {\n            company_id: Some(\"some-string\".to_string()),\n            email: \"email@example.com\".to_string(),\n            name: \"some-string\".to_string(),\n            role: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_create<'a>(
        &'a self,
        body: &crate::types::CompanyManagerParams,
    ) -> Result<crate::types::CompanyManagerCreatedResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "v1/company-managers"),
        );
        req = req.bearer_auth(&self.client.token);
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
