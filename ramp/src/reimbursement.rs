use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Reimbursement {
    pub client: Client,
}

impl Reimbursement {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List reimbursements\n\n**Parameters:**\n\n- `entity_id: Option<uuid::Uuid>`: Filter for reimbursements by business entity.\n- `from_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for reimbursements that happens after the given date.\n- `has_no_sync_commits: Option<bool>`: Filter for reimbursements that have not been synced to ERP systems yet.\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `sync_ready: Option<bool>`: Filter for reimbursements that are coded with accounting fields and ready to sync to ERP systems.\n- `synced_after: Option<chrono::DateTime<chrono::Utc>>`: Filter for reimbursements that have been synced after the given date.\n- `to_date: Option<chrono::DateTime<chrono::Utc>>`: Filter for reimbursements that happens before the given date.\n- `trip_id: Option<uuid::Uuid>`: Filter for reimbursements that are associated with a trip.\n- `updated_after: Option<chrono::DateTime<chrono::Utc>>`: Filter for reimbursements that have been updated after the given date.\n- `user_id: Option<uuid::Uuid>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_reimbursement_get_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiReimbursementResourceSchema = client\n        .reimbursement()\n        .get_list_with_pagination(\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(chrono::Utc::now()),\n            Some(true),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(true),\n            Some(chrono::Utc::now()),\n            Some(chrono::Utc::now()),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(chrono::Utc::now()),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        entity_id: Option<uuid::Uuid>,
        from_date: Option<chrono::DateTime<chrono::Utc>>,
        has_no_sync_commits: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        sync_ready: Option<bool>,
        synced_after: Option<chrono::DateTime<chrono::Utc>>,
        to_date: Option<chrono::DateTime<chrono::Utc>>,
        trip_id: Option<uuid::Uuid>,
        updated_after: Option<chrono::DateTime<chrono::Utc>>,
        user_id: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiReimbursementResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "developer/v1/reimbursements"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = entity_id {
            query_params.push(("entity_id", format!("{}", p)));
        }

        if let Some(p) = from_date {
            query_params.push(("from_date", format!("{}", p)));
        }

        if let Some(p) = has_no_sync_commits {
            query_params.push(("has_no_sync_commits", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
        }

        if let Some(p) = sync_ready {
            query_params.push(("sync_ready", format!("{}", p)));
        }

        if let Some(p) = synced_after {
            query_params.push(("synced_after", format!("{}", p)));
        }

        if let Some(p) = to_date {
            query_params.push(("to_date", format!("{}", p)));
        }

        if let Some(p) = trip_id {
            query_params.push(("trip_id", format!("{}", p)));
        }

        if let Some(p) = updated_after {
            query_params.push(("updated_after", format!("{}", p)));
        }

        if let Some(p) = user_id {
            query_params.push(("user_id", format!("{}", p)));
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

    #[doc = "Fetch a reimbursement\n\n**Parameters:**\n\n- `reimbursement_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_reimbursement_get_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Reimbursement =\n        client.reimbursement().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        reimbursement_id: &'a str,
    ) -> Result<crate::types::Reimbursement, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/reimbursements/{reimbursement_id}"
                    .replace("{reimbursement_id}", reimbursement_id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
