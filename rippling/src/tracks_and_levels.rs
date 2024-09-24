use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct TracksAndLevels {
    pub client: Client,
}

impl TracksAndLevels {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List levels\n\nA List of levels\n- Requires: `API Tier 2`\n- Expandable fields: `parent`, `track`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_tracks_and_levels_list_levels_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut tracks_and_levels = client.tracks_and_levels();\n    let mut stream = tracks_and_levels.list_levels_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_levels<'a>(
        &'a self,
        cursor: Option<String>,
        expand: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListLevelsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "levels"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = cursor {
            query_params.push(("cursor", p));
        }

        if let Some(p) = expand {
            query_params.push(("expand", p));
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

    #[doc = "List levels\n\nA List of levels\n- Requires: `API Tier 2`\n- Expandable fields: `parent`, `track`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_tracks_and_levels_list_levels_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut tracks_and_levels = client.tracks_and_levels();\n    let mut stream = tracks_and_levels.list_levels_stream(\n        Some(\"some-string\".to_string()),\n        Some(\"some-string\".to_string()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_levels_stream<'a>(
        &'a self,
        expand: Option<String>,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::Level, crate::types::error::Error>> + Unpin + '_
    {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_levels(None, expand, order_by)
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
                                    format!("{}/{}", self.client.base_url, "levels"),
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
                            .map_ok(|result: crate::types::ListLevelsResponse| {
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

    #[doc = "Retrieve a specific level\n\nRetrieve a specific level\n\n**Parameters:**\n\n- \
             `expand: Option<String>`\n- `id: &'astr`: ID of the resource to return \
             (required)\n\n```rust,no_run\nasync fn example_tracks_and_levels_get_levels() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             result: rippling_api::types::GetLevelsResponse = client\n        \
             .tracks_and_levels()\n        .get_levels(\n            \
             Some(\"some-string\".to_string()),\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_levels<'a>(
        &'a self,
        expand: Option<String>,
        id: &'a str,
    ) -> Result<crate::types::GetLevelsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "levels/{id}".replace("{id}", id)
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

    #[doc = "List tracks\n\nA List of tracks\n- Requires: `API Tier 2`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_tracks_and_levels_list_tracks_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut tracks_and_levels = client.tracks_and_levels();\n    let mut stream = tracks_and_levels.list_tracks_stream(Some(\"some-string\".to_string()));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_tracks<'a>(
        &'a self,
        cursor: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListTracksResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "tracks"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = cursor {
            query_params.push(("cursor", p));
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

    #[doc = "List tracks\n\nA List of tracks\n- Requires: `API Tier 2`\n- Sortable fields: `id`, `created_at`, `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_tracks_and_levels_list_tracks_stream() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut tracks_and_levels = client.tracks_and_levels();\n    let mut stream = tracks_and_levels.list_tracks_stream(Some(\"some-string\".to_string()));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_tracks_stream<'a>(
        &'a self,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::Track, crate::types::error::Error>> + Unpin + '_
    {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_tracks(None, order_by)
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
                                    format!("{}/{}", self.client.base_url, "tracks"),
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
                            .map_ok(|result: crate::types::ListTracksResponse| {
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

    #[doc = "Retrieve a specific track\n\nRetrieve a specific track\n\n**Parameters:**\n\n- `id: &'astr`: ID of the resource to return (required)\n\n```rust,no_run\nasync fn example_tracks_and_levels_get_tracks() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::GetTracksResponse = client\n        .tracks_and_levels()\n        .get_tracks(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_tracks<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::GetTracksResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "tracks/{id}".replace("{id}", id)
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
}
