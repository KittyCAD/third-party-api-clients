use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct TimeOff {
    pub client: Client,
}

impl TimeOff {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List Time Off\n\nLists all Time Off records.\n\n**Parameters:**\n\n- `employment_id: \
             Option<String>`: Only show time off for a specific employment\n- `order_by: \
             Option<crate::types::OrderBy>`: Sort order\n- `page: Option<i64>`: Starts fetching \
             records after the given page\n- `page_size: Option<i64>`: Change the amount of \
             records returned per page, defaults to 20, limited to 100\n- `sort_by: \
             Option<crate::types::SortBy>`: Field to sort by\n- `status: \
             Option<crate::types::GetIndexTimeoffStatus>`: Filter time off by its status\n- \
             `timeoff_type: Option<crate::types::TimeoffType>`: Filter time off by its \
             type\n\n```rust,no_run\nasync fn example_time_off_get_index_timeoff() -> \
             anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let \
             result: remote_api::types::ListTimeoffResponse = client\n        .time_off()\n        \
             .get_index_timeoff(\n            Some(\"some-string\".to_string()),\n            \
             Some(remote_api::types::OrderBy::Asc),\n            Some(4 as i64),\n            \
             Some(4 as i64),\n            Some(remote_api::types::SortBy::Status),\n            \
             Some(remote_api::types::GetIndexTimeoffStatus::CancelRequested),\n            \
             Some(remote_api::types::TimeoffType::Other),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_index_timeoff<'a>(
        &'a self,
        employment_id: Option<String>,
        order_by: Option<crate::types::OrderBy>,
        page: Option<i64>,
        page_size: Option<i64>,
        sort_by: Option<crate::types::SortBy>,
        status: Option<crate::types::GetIndexTimeoffStatus>,
        timeoff_type: Option<crate::types::TimeoffType>,
    ) -> Result<crate::types::ListTimeoffResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "v1/timeoff"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = employment_id {
            query_params.push(("employment_id", p));
        }

        if let Some(p) = order_by {
            query_params.push(("order_by", format!("{}", p)));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = sort_by {
            query_params.push(("sort_by", format!("{}", p)));
        }

        if let Some(p) = status {
            query_params.push(("status", format!("{}", p)));
        }

        if let Some(p) = timeoff_type {
            query_params.push(("timeoff_type", format!("{}", p)));
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

    #[doc = "Create Time Off\n\nCreates a Time Off record\n\n```rust,no_run\nasync fn \
             example_time_off_post_create_timeoff() -> anyhow::Result<()> {\n    let client = \
             remote_api::Client::new_from_env();\n    let result: \
             remote_api::types::TimeoffResponse = client\n        .time_off()\n        \
             .post_create_timeoff(&remote_api::types::CreateApprovedTimeoffParams {})\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_create_timeoff<'a>(
        &'a self,
        body: &crate::types::CreateApprovedTimeoffParams,
    ) -> Result<crate::types::TimeoffResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "v1/timeoff"),
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

    #[doc = "List Time Off Types\n\nLists all time off types that can be used for the \
             `timeoff_type` parameter\n\n```rust,no_run\nasync fn \
             example_time_off_get_timeoff_types_timeoff() -> anyhow::Result<()> {\n    let client \
             = remote_api::Client::new_from_env();\n    let result: \
             remote_api::types::ListTimeoffTypesResponse =\n        \
             client.time_off().get_timeoff_types_timeoff().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_timeoff_types_timeoff<'a>(
        &'a self,
    ) -> Result<crate::types::ListTimeoffTypesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "v1/timeoff/types"),
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

    #[doc = "Show Time Off\n\nShows a single Time Off record\n\n**Parameters:**\n\n- `timeoff_id: \
             &'astr`: Timeoff ID (required)\n\n```rust,no_run\nasync fn \
             example_time_off_get_show_timeoff() -> anyhow::Result<()> {\n    let client = \
             remote_api::Client::new_from_env();\n    let result: \
             remote_api::types::TimeoffResponse =\n        \
             client.time_off().get_show_timeoff(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_show_timeoff<'a>(
        &'a self,
        timeoff_id: &'a str,
    ) -> Result<crate::types::TimeoffResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/timeoff/{id}".replace("{timeoff_id}", timeoff_id)
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

    #[doc = "Update Time Off\n\nUpdates a Time Off record. This endpoint can also be used for cancelling a time off.\n\n```rust,no_run\nasync fn example_time_off_patch_update_timeoff_2() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::TimeoffResponse = client\n        .time_off()\n        .patch_update_timeoff_2(&remote_api::types::UpdateApprovedTimeoffParams {\n            approved_at: Some(serde_json::Value::String(\"some-string\".to_string())),\n            approver_id: Some(\"some-string\".to_string()),\n            cancel_reason: \"some-string\".to_string(),\n            document: Some(remote_api::types::TimeoffDocumentParams {\n                content: \"some-string\".to_string(),\n                name: \"some-string\".to_string(),\n            }),\n            edit_reason: \"some-string\".to_string(),\n            end_date: Some(chrono::Utc::now().date().naive_utc()),\n            notes: Some(\"some-string\".to_string()),\n            start_date: Some(chrono::Utc::now().date().naive_utc()),\n            status: Some(remote_api::types::UpdateApprovedTimeoffParamsStatus::Approved),\n            timeoff_days: Some(vec![remote_api::types::TimeoffDaysParams {\n                day: Some(chrono::Utc::now().date().naive_utc()),\n                hours: Some(4 as i64),\n            }]),\n            timeoff_type: Some(remote_api::types::TimeoffType::PaternityLeave),\n            timezone: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_update_timeoff_2<'a>(
        &'a self,
        body: &crate::types::UpdateApprovedTimeoffParams,
    ) -> Result<crate::types::TimeoffResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "v1/timeoff/{id}"),
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

    #[doc = "Update Time Off\n\nUpdates a Time Off record. This endpoint can also be used for cancelling a time off.\n\n```rust,no_run\nasync fn example_time_off_patch_update_timeoff() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::TimeoffResponse = client\n        .time_off()\n        .patch_update_timeoff(&remote_api::types::UpdateApprovedTimeoffParams {\n            approved_at: Some(serde_json::Value::String(\"some-string\".to_string())),\n            approver_id: Some(\"some-string\".to_string()),\n            cancel_reason: \"some-string\".to_string(),\n            document: Some(remote_api::types::TimeoffDocumentParams {\n                content: \"some-string\".to_string(),\n                name: \"some-string\".to_string(),\n            }),\n            edit_reason: \"some-string\".to_string(),\n            end_date: Some(chrono::Utc::now().date().naive_utc()),\n            notes: Some(\"some-string\".to_string()),\n            start_date: Some(chrono::Utc::now().date().naive_utc()),\n            status: Some(remote_api::types::UpdateApprovedTimeoffParamsStatus::Cancelled),\n            timeoff_days: Some(vec![remote_api::types::TimeoffDaysParams {\n                day: Some(chrono::Utc::now().date().naive_utc()),\n                hours: Some(4 as i64),\n            }]),\n            timeoff_type: Some(remote_api::types::TimeoffType::ExtendedLeave),\n            timezone: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_update_timeoff<'a>(
        &'a self,
        body: &crate::types::UpdateApprovedTimeoffParams,
    ) -> Result<crate::types::TimeoffResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!("{}/{}", self.client.base_url, "v1/timeoff/{id}"),
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
}
