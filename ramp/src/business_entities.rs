use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct BusinessEntities {
    pub client: Client,
}

impl BusinessEntities {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List business entities\n\n**Parameters:**\n\n- `currency: Option<String>`\n- `entity_name: Option<String>`\n- `is_primary: Option<bool>`\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_business_entities_get_entity_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiEntityResource = client\n        .business_entities()\n        .get_entity_list_with_pagination(\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(false),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_entity_list_with_pagination<'a>(
        &'a self,
        currency: Option<String>,
        entity_name: Option<String>,
        is_primary: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<crate::types::ApiEntityResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "developer/v1/entities"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = currency {
            query_params.push(("currency", p));
        }

        if let Some(p) = entity_name {
            query_params.push(("entity_name", p));
        }

        if let Some(p) = is_primary {
            query_params.push(("is_primary", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get a business entity\n\n**Parameters:**\n\n- `entity_id: uuid::Uuid` \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_business_entities_get_entity_resource() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiEntityResource \
             = client\n        .business_entities()\n        \
             .get_entity_resource(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_entity_resource<'a>(
        &'a self,
        entity_id: uuid::Uuid,
    ) -> Result<crate::types::ApiEntityResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/entities/{entity_id}"
                    .replace("{entity_id}", &format!("{}", entity_id))
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
