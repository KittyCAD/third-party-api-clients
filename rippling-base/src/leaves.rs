use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Leaves {
    pub client: Client,
}

impl Leaves {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "GET Leave Requests\n\nRetrieves the current leave requests.The query can be filtered by a number of specific query parameters.\n\n**Parameters:**\n\n- `end_date: Option<String>`: End date of leave.\n- `from: Option<String>`: Filter to capture whether the leave request overlaps with a date range.\n- `id: Option<String>`\n- `leave_policy: Option<String>`\n- `limit: Option<String>`: Sets a limit on the returned values\n- `offset: Option<String>`: Offsets the returned values\n- `processed_by: Option<String>`\n- `requested_by: Option<String>`\n- `role: Option<String>`\n- `start_date: Option<String>`: Start date of leave.\n- `status: Option<String>`\n- `to: Option<String>`: Filter to capture whether the leave request overlaps with a date range.\n\n```rust,no_run\nasync fn example_leaves_get_requests() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: Vec<rippling_base_api::types::LeaveRequest> = client\n        .leaves()\n        .get_requests(\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_requests<'a>(
        &'a self,
        end_date: Option<String>,
        from: Option<String>,
        id: Option<String>,
        leave_policy: Option<String>,
        limit: Option<String>,
        offset: Option<String>,
        processed_by: Option<String>,
        requested_by: Option<String>,
        role: Option<String>,
        start_date: Option<String>,
        status: Option<String>,
        to: Option<String>,
    ) -> Result<Vec<crate::types::LeaveRequest>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/leave_requests"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = end_date {
            query_params.push(("endDate", p));
        }

        if let Some(p) = from {
            query_params.push(("from", p));
        }

        if let Some(p) = id {
            query_params.push(("id", p));
        }

        if let Some(p) = leave_policy {
            query_params.push(("leavePolicy", p));
        }

        if let Some(p) = limit {
            query_params.push(("limit", p));
        }

        if let Some(p) = offset {
            query_params.push(("offset", p));
        }

        if let Some(p) = processed_by {
            query_params.push(("processedBy", p));
        }

        if let Some(p) = requested_by {
            query_params.push(("requestedBy", p));
        }

        if let Some(p) = role {
            query_params.push(("role", p));
        }

        if let Some(p) = start_date {
            query_params.push(("startDate", p));
        }

        if let Some(p) = status {
            query_params.push(("status", p));
        }

        if let Some(p) = to {
            query_params.push(("to", p));
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

    #[doc = "POST Leave Request\n\nCreate a leave request. This endpoint is currently in alpha and should not be used by default. Only TILT managed requests can have a status other than PENDING.\n\n```rust,no_run\nasync fn example_leaves_post_requests() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: rippling_base_api::types::LeaveRequest = client\n        .leaves()\n        .post_requests(&rippling_base_api::types::PostLeaveRequestsRequestBody {\n            role: \"some-string\".to_string(),\n            requested_by: Some(\"some-string\".to_string()),\n            status: Some(\"some-string\".to_string()),\n            start_date: \"some-string\".to_string(),\n            end_date: \"some-string\".to_string(),\n            start_date_start_time: Some(\"some-string\".to_string()),\n            end_date_end_time: Some(\"some-string\".to_string()),\n            start_date_custom_hours: Some(\"some-string\".to_string()),\n            end_date_custom_hours: Some(\"some-string\".to_string()),\n            company_leave_type: \"some-string\".to_string(),\n            leave_policy: \"some-string\".to_string(),\n            reason_for_leave: Some(\"some-string\".to_string()),\n            managed_by: Some(\"some-string\".to_string()),\n            external_id: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_requests<'a>(
        &'a self,
        body: &crate::types::PostLeaveRequestsRequestBody,
    ) -> Result<crate::types::LeaveRequest, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "platform/api/leave_requests"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "GET Leave Balances\n\nRetrieves the leave balances for employees\n\n**Parameters:**\n\n- `limit: Option<i64>`: Sets a limit on the returned values\n- `offset: Option<i64>`: Offset the returned values\n\n```rust,no_run\nasync fn example_leaves_get_balances() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: rippling_base_api::types::GetLeaveBalancesResponse = client\n        .leaves()\n        .get_balances(Some(4 as i64), Some(4 as i64))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balances<'a>(
        &'a self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<crate::types::GetLeaveBalancesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "platform/api/leave_balances"),
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

    #[doc = "GET Leave Balance\n\nRetrieves the leave balances for a given role. A role represents \
             1 employee. An employee can work at 1 and only 1 company.\n\n**Parameters:**\n\n- \
             `role: &'astr`: This is the unique role ID of the employee. It corresponds to the IDs \
             returned in the Get/employees endpoint (required)\n\n```rust,no_run\nasync fn \
             example_leaves_get_balance() -> anyhow::Result<()> {\n    let client = \
             rippling_base_api::Client::new_from_env();\n    let result: \
             rippling_base_api::types::GetLeaveBalanceResponse =\n        \
             client.leaves().get_balance(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance<'a>(
        &'a self,
        role: &'a str,
    ) -> Result<crate::types::GetLeaveBalanceResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/leave_balances/{role}".replace("{role}", role)
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

    #[doc = "POST Cancel Leave Request\n\nCancel a leave request.\n\n**Parameters:**\n\n- `id: &'astr`: Unique identifier of the leave request being canceled. (required)\n\n```rust,no_run\nasync fn example_leaves_cancel_requests() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: rippling_base_api::types::LeaveRequest = client.leaves().cancel_requests(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn cancel_requests<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::LeaveRequest, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/leave_requests/{id}/cancel".replace("{id}", id)
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

    #[doc = "POST Process Leave Request\n\nApprove or decline a leave request. Only pending requests can be processed (approved / declined). Only an admin or manager is capable of taking action on a request.\n\n**Parameters:**\n\n- `action: crate::types::Action`: The action to be taken on the leave request. Can be either approved or declined. (required)\n- `id: &'astr`: Unique identifier of the leave request being processed. (required)\n\n```rust,no_run\nasync fn example_leaves_process_requests() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: rippling_base_api::types::LeaveRequest = client\n        .leaves()\n        .process_requests(rippling_base_api::types::Action::Decline, \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn process_requests<'a>(
        &'a self,
        action: crate::types::Action,
        id: &'a str,
    ) -> Result<crate::types::LeaveRequest, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/leave_requests/{id}/process".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("action", format!("{}", action))];
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

    #[doc = "PATCH Leave Request\n\nUpdate an existing leave request. With the exception of TILT-managed leave requests, updates are restricted based on status; APPROVED requests can not be updated with different dates, and the status itself can not be updated. Use the process endpoint to update the status of a PENDING request. For other updates, use the cancel endpoint to cancel the request, and create a new request with the updated information.\n\n**Parameters:**\n\n- `id: &'astr`: Unique identifier of the leave request being modified (required)\n\n```rust,no_run\nasync fn example_leaves_patch_requests_request_id() -> anyhow::Result<()> {\n    let client = rippling_base_api::Client::new_from_env();\n    let result: Vec<rippling_base_api::types::LeaveRequest> = client\n        .leaves()\n        .patch_requests_request_id(\n            \"some-string\",\n            &rippling_base_api::types::PatchLeaveRequestsLeaveRequestIdRequestBody {\n                requested_by: Some(\"some-string\".to_string()),\n                status: Some(\"some-string\".to_string()),\n                start_date: Some(\"some-string\".to_string()),\n                end_date: Some(\"some-string\".to_string()),\n                start_date_start_time: Some(\"some-string\".to_string()),\n                end_date_end_time: Some(\"some-string\".to_string()),\n                start_date_custom_hours: Some(\"some-string\".to_string()),\n                end_date_custom_hours: Some(\"some-string\".to_string()),\n                reason_for_leave: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_requests_request_id<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::PatchLeaveRequestsLeaveRequestIdRequestBody,
    ) -> Result<Vec<crate::types::LeaveRequest>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "platform/api/leave_requests/{id}".replace("{id}", id)
            ),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
