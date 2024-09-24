use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Job {
    pub client: Client,
}

impl Job {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List job codes\n\nA List of job codes\n- Requires: `API Tier 2`\n- Filterable fields: `job_dimension_id`, `group_id`\n- Expandable fields: `job_dimension`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_job_list_codes_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut job = client.job();\n    let mut stream = job.list_codes_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_codes<'a>(
        &'a self,
        cursor: Option<String>,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListJobCodesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "job-codes"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = cursor {
            query_params.push(("cursor", p));
        }

        if let Some(p) = expand {
            query_params.push(("expand", p));
        }

        if let Some(p) = filter {
            query_params.push(("filter", p));
        }

        if let Some(p) = order_by {
            query_params.push(("order_by", p));
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

    #[doc = "List job codes\n\nA List of job codes\n- Requires: `API Tier 2`\n- Filterable fields: `job_dimension_id`, `group_id`\n- Expandable fields: `job_dimension`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_job_list_codes_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut job = client.job();\n    let mut stream = job.list_codes_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_codes_stream<'a>(
        &'a self,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::JobCode, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_codes(None, expand, filter, order_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "job-codes"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
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
                            .map_ok(|result: crate::types::ListJobCodesResponse| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a new job code\n\nCreate a new job code\n\n```rust,no_run\nasync fn \
             example_job_create_codes() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: rippling_api::types::JobCode = \
             client\n        .job()\n        .create_codes(&rippling_api::types::JobCodeRequest \
             {\n            name: \"some-string\".to_string(),\n            job_dimension_id: \
             \"some-string\".to_string(),\n            external_id: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_codes<'a>(
        &'a self,
        body: &crate::types::JobCodeRequest,
    ) -> Result<crate::types::JobCode, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "job-codes"),
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

    #[doc = "Retrieve a specific job code\n\nRetrieve a specific job code\n\n**Parameters:**\n\n- \
             `expand: Option<String>`\n- `id: &'astr`: ID of the resource to return \
             (required)\n\n```rust,no_run\nasync fn example_job_get_codes() -> anyhow::Result<()> \
             {\n    let client = rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::GetJobCodesResponse = client\n        .job()\n        \
             .get_codes(\n            Some(\"some-string\".to_string()),\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_codes<'a>(
        &'a self,
        expand: Option<String>,
        id: &'a str,
    ) -> Result<crate::types::GetJobCodesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "job-codes/{id}".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = expand {
            query_params.push(("expand", p));
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

    #[doc = "Delete a job code\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to delete \
             (required)\n\n```rust,no_run\nasync fn example_job_delete_codes() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             client\n        .job()\n        \
             .delete_codes(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_codes<'a>(&'a self, id: &'a str) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "job-codes/{id}".replace("{id}", id)
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

    #[doc = "Update a job code\n\nUpdated a specific job code\n\n**Parameters:**\n\n- `id: \
             &'astr`: ID of the resource to patch (required)\n\n```rust,no_run\nasync fn \
             example_job_update_codes() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: rippling_api::types::JobCode = \
             client\n        .job()\n        .update_codes(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            \
             &rippling_api::types::JobCodeRequest {\n                name: \
             \"some-string\".to_string(),\n                job_dimension_id: \
             \"some-string\".to_string(),\n                external_id: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_codes<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::JobCodeRequest,
    ) -> Result<crate::types::JobCode, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "job-codes/{id}".replace("{id}", id)
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

    #[doc = "List job dimensions\n\nA List of job dimensions\n- Requires: `API Tier 2`\n- Filterable fields: `name`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_job_list_dimensions_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut job = client.job();\n    let mut stream = job.list_dimensions_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_dimensions<'a>(
        &'a self,
        cursor: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListJobDimensionsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "job-dimensions"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = cursor {
            query_params.push(("cursor", p));
        }

        if let Some(p) = filter {
            query_params.push(("filter", p));
        }

        if let Some(p) = order_by {
            query_params.push(("order_by", p));
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

    #[doc = "List job dimensions\n\nA List of job dimensions\n- Requires: `API Tier 2`\n- Filterable fields: `name`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_job_list_dimensions_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut job = client.job();\n    let mut stream = job.list_dimensions_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_dimensions_stream<'a>(
        &'a self,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::JobDimension, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_dimensions(None, filter, order_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "job-dimensions"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
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
                            .map_ok(|result: crate::types::ListJobDimensionsResponse| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Create a new job dimension\n\nCreate a new job dimension\n\n```rust,no_run\nasync fn \
             example_job_create_dimensions() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::JobDimension = client\n        .job()\n        \
             .create_dimensions(&rippling_api::types::JobDimensionRequest {\n            name: \
             \"some-string\".to_string(),\n            external_id: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_dimensions<'a>(
        &'a self,
        body: &crate::types::JobDimensionRequest,
    ) -> Result<crate::types::JobDimension, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "job-dimensions"),
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

    #[doc = "Retrieve a specific job dimension\n\nRetrieve a specific job dimension\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to return (required)\n\n```rust,no_run\nasync fn example_job_get_dimensions() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::GetJobDimensionsResponse = client\n        .job()\n        .get_dimensions(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_dimensions<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::GetJobDimensionsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "job-dimensions/{id}".replace("{id}", id)
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

    #[doc = "Delete a job dimension\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to \
             delete (required)\n\n```rust,no_run\nasync fn example_job_delete_dimensions() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             client\n        .job()\n        \
             .delete_dimensions(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_dimensions<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "job-dimensions/{id}".replace("{id}", id)
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

    #[doc = "Update a job dimension\n\nUpdated a specific job dimension\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to patch (required)\n\n```rust,no_run\nasync fn example_job_update_dimensions() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::JobDimension = client\n        .job()\n        .update_dimensions(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            &rippling_api::types::JobDimensionRequest {\n                name: \"some-string\".to_string(),\n                external_id: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_dimensions<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::JobDimensionRequest,
    ) -> Result<crate::types::JobDimension, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "job-dimensions/{id}".replace("{id}", id)
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
