use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CustomObjectRecords {
    pub client: Client,
}

impl CustomObjectRecords {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List custom object records\n\nA List of custom object records\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_custom_object_records_list_custom_objects_custom_object_api_name_records_stream(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut custom_object_records = client.custom_object_records();\n    let mut stream = custom_object_records\n        .list_custom_objects_custom_object_api_name_records_stream(\"some-string\");\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        cursor: Option<String>,
        custom_object_api_name: &'a str,
    ) -> Result<
        crate::types::ListCustomObjectsCustomObjectApiNameRecordsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records"
                    .replace("{custom_object_api_name}", custom_object_api_name)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = cursor {
            query_params.push(("cursor", p));
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

    #[doc = "List custom object records\n\nA List of custom object records\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_custom_object_records_list_custom_objects_custom_object_api_name_records_stream(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut custom_object_records = client.custom_object_records();\n    let mut stream = custom_object_records\n        .list_custom_objects_custom_object_api_name_records_stream(\"some-string\");\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_custom_objects_custom_object_api_name_records_stream<'a>(
        &'a self,
        custom_object_api_name: &'a str,
    ) -> impl futures::Stream<Item = Result<crate::types::Results, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self . list_custom_objects_custom_object_api_name_records (None , custom_object_api_name) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , "custom-objects/{custom_object_api_name}/records" . replace ("{custom_object_api_name}" , custom_object_api_name)) ,) ; req = req . bearer_auth (& self . client . token) ; let mut request = req . build () ? ; request = new_result . next_page (request) ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (format_serde_error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: ListCustomObjectsCustomObjectApiNameRecordsResponse | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () . boxed ()
    }

    #[doc = "Create a new custom object record\n\nCreate a new custom object record\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_create_custom_objects_custom_object_api_name_records(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::CreateCustomObjectsCustomObjectApiNameRecordsResponse = client\n        .custom_object_records()\n        .create_custom_objects_custom_object_api_name_records(\n            \"some-string\",\n            &rippling_api::types::CreateCustomObjectsCustomObjectApiNameRecordsRequestBody {\n                name: Some(\"some-string\".to_string()),\n                field_api_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        body: &crate::types::CreateCustomObjectsCustomObjectApiNameRecordsRequestBody,
    ) -> Result<
        crate::types::CreateCustomObjectsCustomObjectApiNameRecordsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records"
                    .replace("{custom_object_api_name}", custom_object_api_name)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List custom object records by query\n\nA List of custom object records filtered by querying\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_list_by_query_custom_objects_custom_object_api_name_records(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::ListByQueryCustomObjectsCustomObjectApiNameRecordsResponse = client\n        .custom_object_records()\n        .list_by_query_custom_objects_custom_object_api_name_records(\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            &rippling_api::types::ListByQueryCustomObjectsCustomObjectApiNameRecordsRequestBody {\n                query: Some(\"some-string\".to_string()),\n                limit: Some(4 as i64),\n                cursor: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_by_query_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        cursor: Option<String>,
        custom_object_api_name: &'a str,
        body: &crate::types::ListByQueryCustomObjectsCustomObjectApiNameRecordsRequestBody,
    ) -> Result<
        crate::types::ListByQueryCustomObjectsCustomObjectApiNameRecordsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/query"
                    .replace("{custom_object_api_name}", custom_object_api_name)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = cursor {
            query_params.push(("cursor", p));
        }

        req = req.query(&query_params);
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

    #[doc = "Retrieve a specific custom object record\n\nRetrieve a specific custom object \
             record\n\n**Parameters:**\n\n- `codr_id: &'astr` (required)\n- \
             `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_custom_object_records_get_custom_objects_custom_object_api_name_records(\n) \
             -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             let result: rippling_api::types::GetCustomObjectsCustomObjectApiNameRecordsResponse = \
             client\n        .custom_object_records()\n        \
             .get_custom_objects_custom_object_api_name_records(\"some-string\", \
             \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        codr_id: &'a str,
        custom_object_api_name: &'a str,
    ) -> Result<
        crate::types::GetCustomObjectsCustomObjectApiNameRecordsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/{codr_id}"
                    .replace("{codr_id}", codr_id)
                    .replace("{custom_object_api_name}", custom_object_api_name)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Delete a custom object record\n\n**Parameters:**\n\n- `codr_id: &'astr` (required)\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_delete_custom_objects_custom_object_api_name_records(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    client\n        .custom_object_records()\n        .delete_custom_objects_custom_object_api_name_records(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        codr_id: &'a str,
        custom_object_api_name: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/{codr_id}"
                    .replace("{codr_id}", codr_id)
                    .replace("{custom_object_api_name}", custom_object_api_name)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update a custom object record\n\nUpdated a specific custom object record\n\n**Parameters:**\n\n- `codr_id: &'astr` (required)\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_update_custom_objects_custom_object_api_name_records(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::UpdateCustomObjectsCustomObjectApiNameRecordsResponse = client\n        .custom_object_records()\n        .update_custom_objects_custom_object_api_name_records(\n            \"some-string\",\n            \"some-string\",\n            &rippling_api::types::UpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {\n                name: Some(\"some-string\".to_string()),\n                field_api_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        codr_id: &'a str,
        custom_object_api_name: &'a str,
        body: &crate::types::UpdateCustomObjectsCustomObjectApiNameRecordsRequestBody,
    ) -> Result<
        crate::types::UpdateCustomObjectsCustomObjectApiNameRecordsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/{codr_id}"
                    .replace("{codr_id}", codr_id)
                    .replace("{custom_object_api_name}", custom_object_api_name)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Retrieve a specific custom object record by its external_id\n\nRetrieve a specific custom object record by its external_id\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n- `external_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_get_custom_objects_custom_object_api_name_records_by_external_id(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::GetCustomObjectsCustomObjectApiNameRecordsByExternalIdResponse =\n        client\n            .custom_object_records()\n            .get_custom_objects_custom_object_api_name_records_by_external_id(\n                \"some-string\",\n                \"some-string\",\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_objects_custom_object_api_name_records_by_external_id<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        external_id: &'a str,
    ) -> Result<
        crate::types::GetCustomObjectsCustomObjectApiNameRecordsByExternalIdResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/external_id/{external_id}"
                    .replace("{custom_object_api_name}", custom_object_api_name)
                    .replace("{external_id}", external_id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Bulk Create custom object records\n\nbulk create new custom object records\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_bulk_create_custom_objects_custom_object_api_name_records(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: Vec<rippling_api::types::BulkCreateCustomObjectsCustomObjectApiNameRecordsResponse> =\n        client\n            .custom_object_records()\n            .bulk_create_custom_objects_custom_object_api_name_records(\n                \"some-string\",\n                &rippling_api::types::BulkCreateCustomObjectsCustomObjectApiNameRecordsRequestBody {\n                    rows_to_write: Some(vec![rippling_api::types::RowsToWrite {\n                        name: Some(\"some-string\".to_string()),\n                        field_api_name: Some(\"some-string\".to_string()),\n                    }]),\n                    all_or_nothing: Some(false),\n                },\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn bulk_create_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        body: &crate::types::BulkCreateCustomObjectsCustomObjectApiNameRecordsRequestBody,
    ) -> Result<
        Vec<crate::types::BulkCreateCustomObjectsCustomObjectApiNameRecordsResponse>,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/bulk"
                    .replace("{custom_object_api_name}", custom_object_api_name)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "bulk delete custom object records\n\nBulk Delete custom object records\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_bulk_delete_custom_objects_custom_object_api_name_records(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    client\n        .custom_object_records()\n        .bulk_delete_custom_objects_custom_object_api_name_records(\n            \"some-string\",\n            &rippling_api::types::BulkDeleteCustomObjectsCustomObjectApiNameRecordsRequestBody {\n                rows_to_delete: Some(\"some-string\".to_string()),\n                all_or_nothing: Some(false),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn bulk_delete_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        body: &crate::types::BulkDeleteCustomObjectsCustomObjectApiNameRecordsRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/bulk"
                    .replace("{custom_object_api_name}", custom_object_api_name)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Bulk Update custom object records\n\nBulk Updated a specific custom object records\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_records_bulk_update_custom_objects_custom_object_api_name_records(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: Vec<rippling_api::types::BulkUpdateCustomObjectsCustomObjectApiNameRecordsResponse> =\n        client\n            .custom_object_records()\n            .bulk_update_custom_objects_custom_object_api_name_records(\n                \"some-string\",\n                &rippling_api::types::BulkUpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {\n                    rows_to_update: Some(vec![rippling_api::types::RowsToUpdate {\n                        name: Some(\"some-string\".to_string()),\n                        field_api_name: Some(\"some-string\".to_string()),\n                    }]),\n                    all_or_nothing: Some(false),\n                },\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn bulk_update_custom_objects_custom_object_api_name_records<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        body: &crate::types::BulkUpdateCustomObjectsCustomObjectApiNameRecordsRequestBody,
    ) -> Result<
        Vec<crate::types::BulkUpdateCustomObjectsCustomObjectApiNameRecordsResponse>,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/records/bulk"
                    .replace("{custom_object_api_name}", custom_object_api_name)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
