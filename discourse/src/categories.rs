use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Categories {
    pub client: Client,
}

impl Categories {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Retrieves a list of categories\n\n**Parameters:**\n\n- `include_subcategories: \
             Option<bool>`\n\n```rust,no_run\nasync fn example_categories_list() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::ListCategoriesResponse = \
             client.categories().list(Some(true)).await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        include_subcategories: Option<bool>,
    ) -> Result<crate::types::ListCategoriesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "categories.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = include_subcategories {
            query_params.push(("include_subcategories", format!("{}", p)));
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

    #[doc = "Creates a category\n\n```rust,no_run\nasync fn example_categories_create_category() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::CreateCategoryResponse = client\n        .categories()\n        .create_category(&discourse_api::types::CreateCategoryRequestBody {\n            name: \"some-string\".to_string(),\n            color: Some(\"some-string\".to_string()),\n            text_color: Some(\"some-string\".to_string()),\n            style_type: Some(\"some-string\".to_string()),\n            emoji: Some(\"some-string\".to_string()),\n            icon: Some(\"some-string\".to_string()),\n            parent_category_id: Some(4 as i64),\n            allow_badges: Some(true),\n            slug: Some(\"some-string\".to_string()),\n            topic_featured_links_allowed: Some(true),\n            permissions: Some(discourse_api::types::Permissions {\n                everyone: Some(4 as i64),\n                staff: Some(4 as i64),\n            }),\n            search_priority: Some(4 as i64),\n            form_template_ids: Some(vec![serde_json::Value::String(\"some-string\".to_string())]),\n            category_localizations: Some(vec![discourse_api::types::CategoryLocalizations {\n                id: Some(4 as i64),\n                locale: \"some-string\".to_string(),\n                name: \"some-string\".to_string(),\n                description: Some(\"some-string\".to_string()),\n            }]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_category<'a>(
        &'a self,
        body: &crate::types::CreateCategoryRequestBody,
    ) -> Result<crate::types::CreateCategoryResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "categories.json"),
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

    #[doc = "Updates a category\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync fn example_categories_update_category() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::UpdateCategoryResponse = client\n        .categories()\n        .update_category(\n            4 as i64,\n            &discourse_api::types::UpdateCategoryRequestBody {\n                name: \"some-string\".to_string(),\n                color: Some(\"some-string\".to_string()),\n                text_color: Some(\"some-string\".to_string()),\n                style_type: Some(\"some-string\".to_string()),\n                emoji: Some(\"some-string\".to_string()),\n                icon: Some(\"some-string\".to_string()),\n                parent_category_id: Some(4 as i64),\n                allow_badges: Some(true),\n                slug: Some(\"some-string\".to_string()),\n                topic_featured_links_allowed: Some(true),\n                permissions: Some(discourse_api::types::Permissions {\n                    everyone: Some(4 as i64),\n                    staff: Some(4 as i64),\n                }),\n                search_priority: Some(4 as i64),\n                form_template_ids: Some(vec![serde_json::Value::String(\"some-string\".to_string())]),\n                category_localizations: Some(vec![discourse_api::types::CategoryLocalizations {\n                    id: Some(4 as i64),\n                    locale: \"some-string\".to_string(),\n                    name: \"some-string\".to_string(),\n                    description: Some(\"some-string\".to_string()),\n                }]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_category<'a>(
        &'a self,
        id: i64,
        body: &crate::types::UpdateCategoryRequestBody,
    ) -> Result<crate::types::UpdateCategoryResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "categories/{id}.json".replace("{id}", &format!("{}", id))
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

    #[doc = "List topics\n\n**Parameters:**\n\n- `id: i64` (required)\n- `slug: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_categories_list_category_topics() -> \
             anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    \
             let result: discourse_api::types::ListCategoryTopicsResponse = client\n        \
             .categories()\n        .list_category_topics(4 as i64, \"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_category_topics<'a>(
        &'a self,
        id: i64,
        slug: &'a str,
    ) -> Result<crate::types::ListCategoryTopicsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "c/{slug}/{id}.json"
                    .replace("{id}", &format!("{}", id))
                    .replace("{slug}", slug)
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

    #[doc = "Show category\n\n**Parameters:**\n\n- `id: i64` (required)\n\n```rust,no_run\nasync fn example_categories_get_category() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::GetCategoryResponse =\n        client.categories().get_category(4 as i64).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_category<'a>(
        &'a self,
        id: i64,
    ) -> Result<crate::types::GetCategoryResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "c/{id}/show.json".replace("{id}", &format!("{}", id))
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
