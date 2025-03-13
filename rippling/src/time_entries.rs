use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct TimeEntries {
    pub client: Client,
}

impl TimeEntries {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List time cards\n\nA List of time cards\n- Requires: `API Tier 2`\n- Filterable fields: `pay_period.start_date`, `worker_id`\n- Expandable fields: `worker`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_time_entries_list_time_cards_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut time_entries = client.time_entries();\n    let mut stream = time_entries.list_time_cards_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_time_cards<'a>(
        &'a self,
        cursor: Option<String>,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListTimeCardsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "time-cards"),
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

    #[doc = "List time cards\n\nA List of time cards\n- Requires: `API Tier 2`\n- Filterable fields: `pay_period.start_date`, `worker_id`\n- Expandable fields: `worker`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_time_entries_list_time_cards_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut time_entries = client.time_entries();\n    let mut stream = time_entries.list_time_cards_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_time_cards_stream<'a>(
        &'a self,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::TimeCard, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_time_cards(None, expand, filter, order_by)
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
                                    format!("{}/{}", self.client.base_url, "time-cards"),
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
                            .map_ok(|result: crate::types::ListTimeCardsResponse| {
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

    #[doc = "Retrieve a specific time card\n\nRetrieve a specific time \
             card\n\n**Parameters:**\n\n- `expand: Option<String>`\n- `id: &'astr`: ID of the \
             resource to return (required)\n\n```rust,no_run\nasync fn \
             example_time_entries_get_time_cards() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::GetTimeCardsResponse = client\n        .time_entries()\n        \
             .get_time_cards(\n            Some(\"some-string\".to_string()),\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_time_cards<'a>(
        &'a self,
        expand: Option<String>,
        id: &'a str,
    ) -> Result<crate::types::GetTimeCardsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "time-cards/{id}".replace("{id}", id)
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

    #[doc = "List time entries\n\nA List of time entries\n- Requires: `API Tier 2`\n- Filterable fields: `worker_id`, `start_time`, `pay_period.start_date`\n- Expandable fields: `worker`, `time_card`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_time_entries_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut time_entries = client.time_entries();\n    let mut stream = time_entries.list_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        cursor: Option<String>,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListTimeEntriesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "time-entries"),
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

    #[doc = "List time entries\n\nA List of time entries\n- Requires: `API Tier 2`\n- Filterable fields: `worker_id`, `start_time`, `pay_period.start_date`\n- Expandable fields: `worker`, `time_card`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_time_entries_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut time_entries = client.time_entries();\n    let mut stream = time_entries.list_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::TimeEntry, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list(None, expand, filter, order_by)
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
                                    format!("{}/{}", self.client.base_url, "time-entries"),
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
                            .map_ok(|result: crate::types::ListTimeEntriesResponse| {
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

    #[doc = "Create a new time entry\n\nCreate a new time entry\n\n```rust,no_run\nasync fn example_time_entries_create() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::TimeEntry = client\n        .time_entries()\n        .create(&rippling_api::types::TimeEntryRequest {\n            worker_id: \"some-string\".to_string(),\n            duration: Some(3.14 as f64),\n            comments: Some(vec![rippling_api::types::TimeEntryCommentRequest {\n                text: Some(\"some-string\".to_string()),\n            }]),\n            job_shifts: Some(vec![rippling_api::types::JobShiftRequest {\n                start_time: Some(\"some-string\".to_string()),\n                end_time: Some(\"some-string\".to_string()),\n                duration: Some(3.14 as f64),\n                start_date: Some(\"some-string\".to_string()),\n                original_start_time: Some(\"some-string\".to_string()),\n                original_end_time: Some(\"some-string\".to_string()),\n                job_codes_id: Some(vec![\"some-string\".to_string()]),\n                is_hours_only_input: Some(true),\n            }]),\n            breaks: Some(vec![rippling_api::types::BreakRequest {\n                start_time: Some(\"some-string\".to_string()),\n                end_time: Some(\"some-string\".to_string()),\n                break_type_id: Some(\"some-string\".to_string()),\n            }]),\n            tags: Some(vec![\"some-string\".to_string()]),\n            idempotency_key: Some(\"some-string\".to_string()),\n            create_extra_hours_run: Some(true),\n            status: Some(rippling_api::types::TimeEntryRequestStatus::Paid),\n            pay_period: Some(rippling_api::types::PayPeriodRequest {\n                start_date: Some(\"some-string\".to_string()),\n                end_date: Some(\"some-string\".to_string()),\n                pay_schedule_id: Some(\"some-string\".to_string()),\n            }),\n            shift_input_values: Some(vec![rippling_api::types::ShiftInputValueRequest {\n                shift_input_id: \"some-string\".to_string(),\n                author_id: Some(\"some-string\".to_string()),\n            }]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::TimeEntryRequest,
    ) -> Result<crate::types::TimeEntry, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "time-entries"),
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

    #[doc = "Retrieve a specific time entry\n\nRetrieve a specific time entry\n\n**Parameters:**\n\n- `expand: Option<String>`\n- `id: &'astr`: ID of the resource to return (required)\n\n```rust,no_run\nasync fn example_time_entries_get() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::GetTimeEntriesResponse = client\n        .time_entries()\n        .get(\n            Some(\"some-string\".to_string()),\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        expand: Option<String>,
        id: &'a str,
    ) -> Result<crate::types::GetTimeEntriesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "time-entries/{id}".replace("{id}", id)
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

    #[doc = "Delete a time entry\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to \
             delete (required)\n\n```rust,no_run\nasync fn example_time_entries_delete() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             client\n        .time_entries()\n        \
             .delete(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(&'a self, id: &'a str) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "time-entries/{id}".replace("{id}", id)
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

    #[doc = "Update a time entry\n\nUpdated a specific time entry\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to patch (required)\n\n```rust,no_run\nasync fn example_time_entries_update() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::TimeEntry = client\n        .time_entries()\n        .update(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            &rippling_api::types::TimeEntryRequest {\n                worker_id: \"some-string\".to_string(),\n                duration: Some(3.14 as f64),\n                comments: Some(vec![rippling_api::types::TimeEntryCommentRequest {\n                    text: Some(\"some-string\".to_string()),\n                }]),\n                job_shifts: Some(vec![rippling_api::types::JobShiftRequest {\n                    start_time: Some(\"some-string\".to_string()),\n                    end_time: Some(\"some-string\".to_string()),\n                    duration: Some(3.14 as f64),\n                    start_date: Some(\"some-string\".to_string()),\n                    original_start_time: Some(\"some-string\".to_string()),\n                    original_end_time: Some(\"some-string\".to_string()),\n                    job_codes_id: Some(vec![\"some-string\".to_string()]),\n                    is_hours_only_input: Some(true),\n                }]),\n                breaks: Some(vec![rippling_api::types::BreakRequest {\n                    start_time: Some(\"some-string\".to_string()),\n                    end_time: Some(\"some-string\".to_string()),\n                    break_type_id: Some(\"some-string\".to_string()),\n                }]),\n                tags: Some(vec![\"some-string\".to_string()]),\n                idempotency_key: Some(\"some-string\".to_string()),\n                create_extra_hours_run: Some(true),\n                status: Some(rippling_api::types::TimeEntryRequestStatus::Paid),\n                pay_period: Some(rippling_api::types::PayPeriodRequest {\n                    start_date: Some(\"some-string\".to_string()),\n                    end_date: Some(\"some-string\".to_string()),\n                    pay_schedule_id: Some(\"some-string\".to_string()),\n                }),\n                shift_input_values: Some(vec![rippling_api::types::ShiftInputValueRequest {\n                    shift_input_id: \"some-string\".to_string(),\n                    author_id: Some(\"some-string\".to_string()),\n                }]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::TimeEntryRequest,
    ) -> Result<crate::types::TimeEntry, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "time-entries/{id}".replace("{id}", id)
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
