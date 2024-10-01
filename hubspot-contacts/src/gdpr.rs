use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Gdpr {
    pub client: Client,
}

impl Gdpr {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "GDPR DELETE\n\nPermanently delete a contact and all associated content to follow GDPR. Use optional property 'idProperty' set to 'email' to identify contact by email address. If email address is not found, the email address will be added to a blocklist and prevent it from being used in the future.\n\n```rust,no_run\nasync fn example_gdpr_post_crm_v_3_objects_contacts_delete_purge() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    client\n        .gdpr()\n        .post_crm_v_3_objects_contacts_delete_purge(&hubspot_contacts::types::PublicGdprDeleteInput {\n            id_property: Some(\"some-string\".to_string()),\n            object_id: \"some-string\".to_string(),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_contacts_delete_purge<'a>(
        &'a self,
        body: &crate::types::PublicGdprDeleteInput,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/contacts/gdpr-delete"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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
}
