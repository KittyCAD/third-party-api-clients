use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CustomObjectFields {
    pub client: Client,
}

impl CustomObjectFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List custom object fields\n\nA List of custom object fields\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_custom_object_fields_list_custom_objects_custom_object_api_name_fields_stream(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut custom_object_fields = client.custom_object_fields();\n    let mut stream = custom_object_fields\n        .list_custom_objects_custom_object_api_name_fields_stream(\"some-string\");\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_custom_objects_custom_object_api_name_fields<'a>(
        &'a self,
        cursor: Option<String>,
        custom_object_api_name: &'a str,
    ) -> Result<
        crate::types::ListCustomObjectsCustomObjectApiNameFieldsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/fields"
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

    #[doc = "List custom object fields\n\nA List of custom object fields\n- Requires: `API Tier 1`\n\n**Parameters:**\n\n- `cursor: Option<String>`\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_custom_object_fields_list_custom_objects_custom_object_api_name_fields_stream(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let mut custom_object_fields = client.custom_object_fields();\n    let mut stream = custom_object_fields\n        .list_custom_objects_custom_object_api_name_fields_stream(\"some-string\");\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_custom_objects_custom_object_api_name_fields_stream<'a>(
        &'a self,
        custom_object_api_name: &'a str,
    ) -> impl futures::Stream<
        Item = Result<crate::types::CustomObjectField, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self . list_custom_objects_custom_object_api_name_fields (None , custom_object_api_name) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold ((None , result) , move | (prev_page_token , new_result) | async move { if new_result . has_more_pages () && ! new_result . items () . is_empty () && prev_page_token != new_result . next_page_token () { async { let mut req = self . client . client . request (http :: Method :: GET , format ! ("{}/{}" , self . client . base_url , "custom-objects/{custom_object_api_name}/fields" . replace ("{custom_object_api_name}" , custom_object_api_name)) ,) ; req = req . bearer_auth (& self . client . token) ; let mut request = req . build () ? ; request = new_result . next_page (request) ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (format_serde_error :: SerdeError :: new (text . to_string () , err) , status)) } else { let text = resp . text () . await . unwrap_or_default () ; Err (crate :: types :: error :: Error :: Server { body : text . to_string () , status }) } } . map_ok (| result : crate :: types :: ListCustomObjectsCustomObjectApiNameFieldsResponse | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , (new_result . next_page_token () , result) ,)) }) . await } else { Ok (None) } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () . boxed ()
    }

    #[doc = "Create a new custom object field\n\nCreate a new custom object field\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_fields_create_custom_objects_custom_object_api_name_fields(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::CustomObjectField = client\n        .custom_object_fields()\n        .create_custom_objects_custom_object_api_name_fields(\n            \"some-string\",\n            &rippling_api::types::CreateCustomObjectsCustomObjectApiNameFieldsRequestBody {\n                name: Some(\"some-string\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                required: Some(false),\n                is_unique: Some(false),\n                enable_history: Some(false),\n                derived_field_formula: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_custom_objects_custom_object_api_name_fields<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        body: &crate::types::CreateCustomObjectsCustomObjectApiNameFieldsRequestBody,
    ) -> Result<crate::types::CustomObjectField, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/fields"
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

    #[doc = "Retrieve a specific custom object field\n\nRetrieve a specific custom object field\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n- `field_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_fields_get_custom_objects_custom_object_api_name_fields(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::CustomObjectField = client\n        .custom_object_fields()\n        .get_custom_objects_custom_object_api_name_fields(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_objects_custom_object_api_name_fields<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        field_api_name: &'a str,
    ) -> Result<crate::types::CustomObjectField, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/fields/{field_api_name}"
                    .replace("{custom_object_api_name}", custom_object_api_name)
                    .replace("{field_api_name}", field_api_name)
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

    #[doc = "Delete a custom object field\n\nDelete a custom object field\n\n**Parameters:**\n\n- \
             `custom_object_api_name: &'astr` (required)\n- `field_api_name: &'astr` \
             (required)\n\n```rust,no_run\nasync fn \
             example_custom_object_fields_delete_custom_objects_custom_object_api_name_fields(\n) \
             -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    \
             client\n        .custom_object_fields()\n        \
             .delete_custom_objects_custom_object_api_name_fields(\"some-string\", \
             \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_custom_objects_custom_object_api_name_fields<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        field_api_name: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/fields/{field_api_name}"
                    .replace("{custom_object_api_name}", custom_object_api_name)
                    .replace("{field_api_name}", field_api_name)
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

    #[doc = "Update a custom object field\n\nUpdated a specific custom object field\n\n**Parameters:**\n\n- `custom_object_api_name: &'astr` (required)\n- `field_api_name: &'astr` (required)\n\n```rust,no_run\nasync fn example_custom_object_fields_update_custom_objects_custom_object_api_name_fields(\n) -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::CustomObjectField = client\n        .custom_object_fields()\n        .update_custom_objects_custom_object_api_name_fields(\n            \"some-string\",\n            \"some-string\",\n            &rippling_api::types::UpdateCustomObjectsCustomObjectApiNameFieldsRequestBody {\n                name: Some(\"some-string\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                required: Some(false),\n                is_unique: Some(false),\n                enable_history: Some(false),\n                derived_field_formula: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_custom_objects_custom_object_api_name_fields<'a>(
        &'a self,
        custom_object_api_name: &'a str,
        field_api_name: &'a str,
        body: &crate::types::UpdateCustomObjectsCustomObjectApiNameFieldsRequestBody,
    ) -> Result<crate::types::CustomObjectField, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "custom-objects/{custom_object_api_name}/fields/{field_api_name}"
                    .replace("{custom_object_api_name}", custom_object_api_name)
                    .replace("{field_api_name}", field_api_name)
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
