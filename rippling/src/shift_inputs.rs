use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ShiftInputs {
    pub client: Client,
}

impl ShiftInputs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List shift inputs\n\nA List of shift inputs\n- Requires: `API Tier 2`\n- Filterable fields: `name`\n- Expandable fields: `creator`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_shift_inputs_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut shift_inputs = client.shift_inputs();\n    let mut stream = shift_inputs.list_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        cursor: Option<String>,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListShiftInputsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "shift-inputs"),
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

    #[doc = "List shift inputs\n\nA List of shift inputs\n- Requires: `API Tier 2`\n- Filterable fields: `name`\n- Expandable fields: `creator`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `filter: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_shift_inputs_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut shift_inputs = client.shift_inputs();\n    let mut stream = shift_inputs.list_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        expand: Option<String>,
        filter: Option<String>,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::ShiftInput, crate::types::error::Error>>
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
                                    format!("{}/{}", self.client.base_url, "shift-inputs"),
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
                            .map_ok(|result: crate::types::ListShiftInputsResponse| {
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

    #[doc = "Create a new shift input\n\nCreate a new shift input\n\n```rust,no_run\nasync fn example_shift_inputs_create() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::ShiftInput = client\n        .shift_inputs()\n        .create(&rippling_api::types::ShiftInputRequest {\n            creator_id: Some(\"some-string\".to_string()),\n            name: \"some-string\".to_string(),\n            prompt: \"some-string\".to_string(),\n            type_: \"some-string\".to_string(),\n            country_code: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::ShiftInputRequest,
    ) -> Result<crate::types::ShiftInput, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "shift-inputs"),
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

    #[doc = "Retrieve a specific shift input\n\nRetrieve a specific shift input\n\n**Parameters:**\n\n- `expand: Option<String>`\n- `id: &'astr`: ID of the resource to return (required)\n\n```rust,no_run\nasync fn example_shift_inputs_get() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::GetShiftInputsResponse = client\n        .shift_inputs()\n        .get(\n            Some(\"some-string\".to_string()),\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        expand: Option<String>,
        id: &'a str,
    ) -> Result<crate::types::GetShiftInputsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "shift-inputs/{id}".replace("{id}", id)
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

    #[doc = "Delete a shift input\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to \
             delete (required)\n\n```rust,no_run\nasync fn example_shift_inputs_delete() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             client\n        .shift_inputs()\n        \
             .delete(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(&'a self, id: &'a str) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "shift-inputs/{id}".replace("{id}", id)
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

    #[doc = "Update a shift input\n\nUpdated a specific shift input\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to patch (required)\n\n```rust,no_run\nasync fn example_shift_inputs_update() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::ShiftInput = client\n        .shift_inputs()\n        .update(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            &rippling_api::types::ShiftInputRequest {\n                creator_id: Some(\"some-string\".to_string()),\n                name: \"some-string\".to_string(),\n                prompt: \"some-string\".to_string(),\n                type_: \"some-string\".to_string(),\n                country_code: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::ShiftInputRequest,
    ) -> Result<crate::types::ShiftInput, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "shift-inputs/{id}".replace("{id}", id)
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
