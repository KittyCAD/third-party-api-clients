use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct PublicObject {
    pub client: Client,
}

impl PublicObject {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Merge two contacts with same type\n\n```rust,no_run\nasync fn \
             example_public_object_post_crm_v_3_objects_contacts_merge_merge() -> \
             anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    \
             let result: hubspot_contacts::types::SimplePublicObject = client\n        \
             .public_object()\n        \
             .post_crm_v_3_objects_contacts_merge_merge(&hubspot_contacts::types::PublicMergeInput \
             {\n            object_id_to_merge: \"some-string\".to_string(),\n            \
             primary_object_id: \"some-string\".to_string(),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_contacts_merge_merge<'a>(
        &'a self,
        body: &crate::types::PublicMergeInput,
    ) -> Result<crate::types::SimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/contacts/merge"
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
