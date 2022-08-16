use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CustomFields {
    pub client: Client,
}

impl CustomFields {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List Contact's custom fields\n\nLists the custom fields in your company that are \
             application to a Contact.\n\n```rust,no_run\nasync fn \
             example_custom_fields_list_contact() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListContactCustomFieldsResponse =\n        \
             client.custom_fields().list_contact().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_contact<'a>(
        &'a self,
    ) -> Result<crate::types::ListContactCustomFieldsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "contacts/custom_fields"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List Contact's custom fields\n\nLists the custom fields in your company that are \
             application to a Contact.\n> ⚠\u{fe0f} Deprecated endpoint\n>\n> This endpoint has \
             been deprecated. Please use the fully compatible `GET /contacts/custom_fields` \
             endpoint instead.\n\n\n**NOTE:** This operation is marked as \
             deprecated.\n\n```rust,no_run\nasync fn example_custom_fields_list() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListCustomFieldsResponse = \
             client.custom_fields().list().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListCustomFieldsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "custom_fields"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Update custom field\n\nUpdate a custom field. (⚠\u{fe0f} Deprecated)\n\n**Parameters:**\n\n- `custom_field_id: &'astr`: Custom Field ID (required)\n\n**NOTE:** This operation is marked as deprecated.\n\n```rust,no_run\nasync fn example_custom_fields_update() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .custom_fields()\n        .update(\n            \"some-string\",\n            &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        custom_field_id: &'a str,
        body: &crate::types::UpdateCustomField,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "custom_fields/{custom_field_id}".replace("{custom_field_id}", custom_field_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
