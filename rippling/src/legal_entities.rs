use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct LegalEntities {
    pub client: Client,
}

impl LegalEntities {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List legal entities\n\nA List of legal entities\n- Requires: `API Tier 2`\n- \
             Expandable fields: `parent`, `company`\n- Sortable fields: `id`, `created_at`, \
             `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: \
             Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse \
             futures_util::TryStreamExt;\nasync fn example_legal_entities_list_stream() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             mut legal_entities = client.legal_entities();\n    let mut stream = \
             legal_entities.list_stream(\n        Some(\"some-string\".to_string()),\n        \
             Some(\"some-string\".to_string()),\n    );\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        cursor: Option<String>,
        expand: Option<String>,
        order_by: Option<String>,
    ) -> Result<crate::types::ListLegalEntitiesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "legal-entities"),
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

    #[doc = "List legal entities\n\nA List of legal entities\n- Requires: `API Tier 2`\n- \
             Expandable fields: `parent`, `company`\n- Sortable fields: `id`, `created_at`, \
             `updated_at`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `expand: \
             Option<String>`\n- `order_by: Option<String>`\n\n```rust,no_run\nuse \
             futures_util::TryStreamExt;\nasync fn example_legal_entities_list_stream() -> \
             anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let \
             mut legal_entities = client.legal_entities();\n    let mut stream = \
             legal_entities.list_stream(\n        Some(\"some-string\".to_string()),\n        \
             Some(\"some-string\".to_string()),\n    );\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        expand: Option<String>,
        order_by: Option<String>,
    ) -> impl futures::Stream<Item = Result<crate::types::LegalEntity, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list(None, expand, order_by)
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
                                    format!("{}/{}", self.client.base_url, "legal-entities"),
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
                            .map_ok(|result: crate::types::ListLegalEntitiesResponse| {
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

    #[doc = "Retrieve a specific legal entity\n\nRetrieve a specific legal entity\n\n**Parameters:**\n\n- `expand: Option<String>`\n- `id: &'astr`: ID of the resource to return (required)\n\n```rust,no_run\nasync fn example_legal_entities_get() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::GetLegalEntitiesResponse = client\n        .legal_entities()\n        .get(\n            Some(\"some-string\".to_string()),\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        expand: Option<String>,
        id: &'a str,
    ) -> Result<crate::types::GetLegalEntitiesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "legal-entities/{id}".replace("{id}", id)
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
}