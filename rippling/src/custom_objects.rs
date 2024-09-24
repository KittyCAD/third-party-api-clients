use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CustomObjects {
    pub client: Client,
}

impl CustomObjects {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List custom objects\n\nA List of custom objects\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_custom_objects_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut custom_objects = client.custom_objects();\n    let mut stream = custom_objects.list_stream();\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        cursor: Option<String>,
    ) -> Result<crate::types::ListCustomObjectsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "custom-objects"),
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

    #[doc = "List custom objects\n\nA List of custom objects\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_custom_objects_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut custom_objects = client.custom_objects();\n    let mut stream = custom_objects.list_stream();\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
    ) -> impl futures::Stream<Item = Result<crate::types::CustomObject, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list(None)
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
                                    format!("{}/{}", self.client.base_url, "custom-objects"),
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
                            .map_ok(|result: crate::types::ListCustomObjectsResponse| {
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

    #[doc = "Create a new custom object\n\nCreate a new custom object\n\n```rust,no_run\nasync fn \
             example_custom_objects_create() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::CustomObject = client\n        .custom_objects()\n        \
             .create(&rippling_api::types::CreateCustomObjectsRequestBody {\n            name: \
             Some(\"some-string\".to_string()),\n            description: \
             Some(\"some-string\".to_string()),\n            category: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateCustomObjectsRequestBody,
    ) -> Result<crate::types::CustomObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "custom-objects"),
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

    #[doc = "Retrieve a specific custom object\n\nRetrieve a specific custom \
             object\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_custom_objects_get() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::CustomObject = \
             client.custom_objects().get(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        custom_object_api_name: &'a str,
    ) -> Result<crate::types::CustomObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}"
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

    #[doc = "Delete a custom object\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_custom_objects_delete() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             client.custom_objects().delete(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        custom_object_api_name: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}"
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

    #[doc = "Update a custom object\n\nUpdated a specific custom object\n\n**Parameters:**\n\n- \
             `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_custom_objects_update() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::CustomObject = client\n        .custom_objects()\n        \
             .update(\n            \"some-string\",\n            \
             &rippling_api::types::UpdateCustomObjectsRequestBody {\n                name: \
             Some(\"some-string\".to_string()),\n                description: \
             Some(\"some-string\".to_string()),\n                category: \
             Some(\"some-string\".to_string()),\n                plural_label: \
             Some(\"some-string\".to_string()),\n                owner_role: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        body: &crate::types::UpdateCustomObjectsRequestBody,
    ) -> Result<crate::types::CustomObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}"
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
