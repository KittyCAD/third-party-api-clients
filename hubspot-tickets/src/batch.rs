use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Batch {
    pub client: Client,
}

impl Batch {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Read a batch of tickets by internal ID, or unique property values\n\n**Parameters:**\n\n- `archived: Option<bool>`: Whether to return only results that have been archived.\n\n```rust,no_run\nasync fn example_batch_post_crm_v_3_objects_tickets_read_read() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    let result: hubspot_tickets::types::BatchResponseSimplePublicObject = client\n        .batch()\n        .post_crm_v_3_objects_tickets_read_read(\n            Some(true),\n            &hubspot_tickets::types::BatchReadInputSimplePublicObjectId {\n                properties_with_history: vec![\"some-string\".to_string()],\n                id_property: Some(\"some-string\".to_string()),\n                inputs: vec![hubspot_tickets::types::SimplePublicObjectId {\n                    id: \"some-string\".to_string(),\n                }],\n                properties: vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_tickets_read_read<'a>(
        &'a self,
        archived: Option<bool>,
        body: &crate::types::BatchReadInputSimplePublicObjectId,
    ) -> Result<crate::types::BatchResponseSimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/tickets/batch/read"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = archived {
            query_params.push(("archived", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "Archive a batch of tickets by ID\n\n```rust,no_run\nasync fn \
             example_batch_post_crm_v_3_objects_tickets_archive_archive() -> anyhow::Result<()> \
             {\n    let client = hubspot_tickets::Client::new_from_env();\n    client\n        \
             .batch()\n        .post_crm_v_3_objects_tickets_archive_archive(\n            \
             &hubspot_tickets::types::BatchInputSimplePublicObjectId {\n                inputs: \
             vec![hubspot_tickets::types::SimplePublicObjectId {\n                    id: \
             \"some-string\".to_string(),\n                }],\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_tickets_archive_archive<'a>(
        &'a self,
        body: &crate::types::BatchInputSimplePublicObjectId,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/tickets/batch/archive"
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

    #[doc = "Create a batch of tickets\n\n```rust,no_run\nasync fn example_batch_post_crm_v_3_objects_tickets_create_create() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    let result: hubspot_tickets::types::BatchResponseSimplePublicObject = client\n        .batch()\n        .post_crm_v_3_objects_tickets_create_create(\n            &hubspot_tickets::types::BatchInputSimplePublicObjectInputForCreate {\n                inputs: vec![hubspot_tickets::types::SimplePublicObjectInputForCreate {\n                    associations: vec![hubspot_tickets::types::PublicAssociationsForObject {\n                        types: vec![hubspot_tickets::types::AssociationSpec {\n                            association_category: hubspot_tickets::types::AssociationCategory::UserDefined,\n                            association_type_id: 4 as i32,\n                        }],\n                        to: hubspot_tickets::types::PublicObjectId {\n                            id: \"some-string\".to_string(),\n                        },\n                    }],\n                    properties: std::collections::HashMap::from([(\n                        \"some-key\".to_string(),\n                        \"some-string\".to_string(),\n                    )]),\n                }],\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_tickets_create_create<'a>(
        &'a self,
        body: &crate::types::BatchInputSimplePublicObjectInputForCreate,
    ) -> Result<crate::types::BatchResponseSimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/tickets/batch/create"
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

    #[doc = "Update a batch of tickets\n\n```rust,no_run\nasync fn example_batch_post_crm_v_3_objects_tickets_update_update() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    let result: hubspot_tickets::types::BatchResponseSimplePublicObject = client\n        .batch()\n        .post_crm_v_3_objects_tickets_update_update(\n            &hubspot_tickets::types::BatchInputSimplePublicObjectBatchInput {\n                inputs: vec![hubspot_tickets::types::SimplePublicObjectBatchInput {\n                    id_property: Some(\"some-string\".to_string()),\n                    id: \"some-string\".to_string(),\n                    properties: std::collections::HashMap::from([(\n                        \"some-key\".to_string(),\n                        \"some-string\".to_string(),\n                    )]),\n                }],\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_tickets_update_update<'a>(
        &'a self,
        body: &crate::types::BatchInputSimplePublicObjectBatchInput,
    ) -> Result<crate::types::BatchResponseSimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/tickets/batch/update"
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
