use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ObjectCategories {
    pub client: Client,
}

impl ObjectCategories {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List object categories\n\nA List of object categories\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_object_categories_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut object_categories = client.object_categories();\n    let mut stream = object_categories.list_stream();\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        cursor: Option<String>,
    ) -> Result<crate::types::ListObjectCategoriesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "object-categories"),
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

    #[doc = "List object categories\n\nA List of object categories\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_object_categories_list_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut object_categories = client.object_categories();\n    let mut stream = object_categories.list_stream();\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
    ) -> impl futures::Stream<Item = Result<crate::types::ObjectCategory, crate::types::error::Error>>
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
                                    format!("{}/{}", self.client.base_url, "object-categories"),
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
                            .map_ok(|result: crate::types::ListObjectCategoriesResponse| {
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

    #[doc = "Create a new object category\n\nCreate a new object category\n\n```rust,no_run\nasync \
             fn example_object_categories_create() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::ObjectCategory = client\n        .object_categories()\n        \
             .create(&rippling_api::types::CreateObjectCategoriesRequestBody {\n            name: \
             Some(\"some-string\".to_string()),\n            description: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateObjectCategoriesRequestBody,
    ) -> Result<crate::types::ObjectCategory, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "object-categories"),
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

    #[doc = "Retrieve a specific object category\n\nRetrieve a specific object \
             category\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_object_categories_get() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::ObjectCategory =\n        \
             client.object_categories().get(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::ObjectCategory, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "object-categories/{id}".replace("{id}", id)
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

    #[doc = "Delete a object category\n\n**Parameters:**\n\n- `id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_object_categories_delete() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             client.object_categories().delete(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(&'a self, id: &'a str) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "object-categories/{id}".replace("{id}", id)
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

    #[doc = "Update a object category\n\nUpdated a specific object \
             category\n\n**Parameters:**\n\n- `id: &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_object_categories_update() -> anyhow::Result<()> {\n    let client = \
             rippling_api::Client::new_from_env();\n    let result: \
             rippling_api::types::ObjectCategory = client\n        .object_categories()\n        \
             .update(\n            \"some-string\",\n            \
             &rippling_api::types::UpdateObjectCategoriesRequestBody {\n                name: \
             Some(\"some-string\".to_string()),\n                description: \
             Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::UpdateObjectCategoriesRequestBody,
    ) -> Result<crate::types::ObjectCategory, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "object-categories/{id}".replace("{id}", id)
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