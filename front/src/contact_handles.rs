use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ContactHandles {
    pub client: Client,
}

impl ContactHandles {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Add contact handle\n\nAdds a new handle to a contact.\n\n**Parameters:**\n\n- `contact_id: &'astr`: The contact ID (required)\n\n```rust,no_run\nasync fn example_contact_handles_add() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .contact_handles()\n        .add(\n            \"some-string\",\n            &front_api::types::ContactHandle {\n                handle: \"some-string\".to_string(),\n                source: front_api::types::Source::Custom,\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add<'a>(
        &'a self,
        contact_id: &'a str,
        body: &crate::types::ContactHandle,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}/handles".replace("{contact_id}", contact_id)
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

    #[doc = "Delete contact handle\n\nRemove a handle from a contact.\n\n**Parameters:**\n\n- \
             `contact_id: &'astr`: The contact ID (required)\n\n```rust,no_run\nasync fn \
             example_contact_handles_delete() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    client\n        .contact_handles()\n        \
             .delete(\n            \"some-string\",\n            \
             &front_api::types::DeleteContactHandle {\n                handle: \
             \"some-string\".to_string(),\n                source: \
             front_api::types::Source::Custom,\n                force: Some(true),\n            \
             },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        contact_id: &'a str,
        body: &crate::types::DeleteContactHandle,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}/handles".replace("{contact_id}", contact_id)
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
