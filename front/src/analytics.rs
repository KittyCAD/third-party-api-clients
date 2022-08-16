use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Analytics {
    pub client: Client,
}

impl Analytics {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Create a new analytics report.\n\nCreate a new analytics report for a set of metrics over a specific time span.\nThe report will be executed asynchronously. The response will include a link that can be used to retrieve the\nreport status & result.\n\n\n```rust,no_run\nasync fn example_analytics_create_report() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::AnalyticsReportResponse2 = client\n        .analytics()\n        .create_report(&serde_json::Value::String(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_report<'a>(
        &'a self,
        body: &crate::types::AnalyticsReportRequest2,
    ) -> Result<crate::types::AnalyticsReportResponse2, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "analytics/reports"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Fetch an analytics report.\n\n**Parameters:**\n\n- `report_uid: &'astr`: The report \
             UID. (required)\n\n```rust,no_run\nasync fn example_analytics_get_report() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::AnalyticsReportResponse2 =\n        \
             client.analytics().get_report(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_report<'a>(
        &'a self,
        report_uid: &'a str,
    ) -> Result<crate::types::AnalyticsReportResponse2, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "analytics/reports/{report_uid}".replace("{report_uid}", report_uid)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create a new analytics export.\n\nCreate a new analytics export of messages or activities over a specific time span.\nThe export will be executed asynchronously. The response will include a link that can be used to retrieve the\nexport status & result.\n\n\n```rust,no_run\nasync fn example_analytics_create_export() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::AnalyticsExportResponse2 = client\n        .analytics()\n        .create_export(&serde_json::Value::String(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_export<'a>(
        &'a self,
        body: &crate::types::AnalyticsExportRequest2,
    ) -> Result<crate::types::AnalyticsExportResponse2, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "analytics/exports"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Fetch an analytics export.\n\n**Parameters:**\n\n- `export_id: &'astr`: The export \
             ID. (required)\n\n```rust,no_run\nasync fn example_analytics_get_export() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::AnalyticsExportResponse2 =\n        \
             client.analytics().get_export(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_export<'a>(
        &'a self,
        export_id: &'a str,
    ) -> Result<crate::types::AnalyticsExportResponse2, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "analytics/exports/{export_id}".replace("{export_id}", export_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
