use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Basic {
    pub client: Client,
}

impl Basic {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Read\n\nRead an Object identified by `{contactId}`. `{contactId}` refers to the internal object ID.  Control what is returned via the `properties` query param.\n\n**Parameters:**\n\n- `archived: Option<bool>`: Whether to return only results that have been archived.\n- `associations: Option<Vec<String>>`: A comma separated list of object types to retrieve associated IDs for. If any of the specified associations do not exist, they will be ignored.\n- `contact_id: &'astr` (required)\n- `id_property: Option<String>`: What id property to query, could be id or email\n- `properties: Option<Vec<String>>`: A comma separated list of the properties to be returned in the response. If any of the specified properties are not present on the requested object(s), they will be ignored.\n- `properties_with_history: Option<Vec<String>>`: A comma separated list of the properties to be returned along with their history of previous values. If any of the specified properties are not present on the requested object(s), they will be ignored.\n\n```rust,no_run\nasync fn example_basic_get_crm_v_3_objects_contacts_contact_id_get_by_id() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::SimplePublicObjectWithAssociations = client\n        .basic()\n        .get_crm_v_3_objects_contacts_contact_id_get_by_id(\n            Some(true),\n            Some(vec![\"some-string\".to_string()]),\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n            Some(vec![\"some-string\".to_string()]),\n            Some(vec![\"some-string\".to_string()]),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_crm_v_3_objects_contacts_contact_id_get_by_id<'a>(
        &'a self,
        archived: Option<bool>,
        associations: Option<Vec<String>>,
        contact_id: &'a str,
        id_property: Option<String>,
        properties: Option<Vec<String>>,
        properties_with_history: Option<Vec<String>>,
    ) -> Result<crate::types::SimplePublicObjectWithAssociations, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "crm/v3/objects/contacts/{contactId}".replace("{contactId}", contact_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = archived {
            query_params.push(("archived", format!("{}", p)));
        }

        if let Some(p) = associations {
            query_params.push(("associations", itertools::join(p, ",")));
        }

        if let Some(p) = id_property {
            query_params.push(("idProperty", p));
        }

        if let Some(p) = properties {
            query_params.push(("properties", itertools::join(p, ",")));
        }

        if let Some(p) = properties_with_history {
            query_params.push(("propertiesWithHistory", itertools::join(p, ",")));
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

    #[doc = "Archive\n\nMove an Object identified by `{contactId}` to the recycling \
             bin.\n\n**Parameters:**\n\n- `contact_id: &'astr` (required)\n\n```rust,no_run\nasync \
             fn example_basic_delete_crm_v_3_objects_contacts_contact_id_archive() -> \
             anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    \
             client\n        .basic()\n        \
             .delete_crm_v_3_objects_contacts_contact_id_archive(\"some-string\")\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_crm_v_3_objects_contacts_contact_id_archive<'a>(
        &'a self,
        contact_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "crm/v3/objects/contacts/{contactId}".replace("{contactId}", contact_id)
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

    #[doc = "Update\n\nPerform a partial update of an Object identified by `{contactId}`. `{contactId}` refers to the internal object ID. Provided property values will be overwritten. Read-only and non-existent properties will be ignored. Properties values can be cleared by passing an empty string.\n\n**Parameters:**\n\n- `contact_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_basic_patch_crm_v_3_objects_contacts_contact_id_update() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::SimplePublicObject = client\n        .basic()\n        .patch_crm_v_3_objects_contacts_contact_id_update(\n            \"some-string\",\n            &hubspot_contacts::types::SimplePublicObjectInput {\n                properties: std::collections::HashMap::from([(\n                    \"some-key\".to_string(),\n                    \"some-string\".to_string(),\n                )]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_crm_v_3_objects_contacts_contact_id_update<'a>(
        &'a self,
        contact_id: &'a str,
        body: &crate::types::SimplePublicObjectInput,
    ) -> Result<crate::types::SimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "crm/v3/objects/contacts/{contactId}".replace("{contactId}", contact_id)
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

    #[doc = "List\n\nRead a page of contacts. Control what is returned via the `properties` query param.\n\n**Parameters:**\n\n- `after: Option<String>`: The paging cursor token of the last successfully read resource will be returned as the `paging.next.after` JSON property of a paged response containing more results.\n- `archived: Option<bool>`: Whether to return only results that have been archived.\n- `associations: Option<Vec<String>>`: A comma separated list of object types to retrieve associated IDs for. If any of the specified associations do not exist, they will be ignored.\n- `limit: Option<i32>`: The maximum number of results to display per page.\n- `properties: Option<Vec<String>>`: A comma separated list of the properties to be returned in the response. If any of the specified properties are not present on the requested object(s), they will be ignored.\n- `properties_with_history: Option<Vec<String>>`: A comma separated list of the properties to be returned along with their history of previous values. If any of the specified properties are not present on the requested object(s), they will be ignored. Usage of this parameter will reduce the maximum number of objects that can be read by a single request.\n\n```rust,no_run\nasync fn example_basic_get_crm_v_3_objects_contacts_get_page() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::CollectionResponseSimplePublicObjectWithAssociationsForwardPaging =\n        client\n            .basic()\n            .get_crm_v_3_objects_contacts_get_page(\n                Some(\"some-string\".to_string()),\n                Some(true),\n                Some(vec![\"some-string\".to_string()]),\n                Some(4 as i32),\n                Some(vec![\"some-string\".to_string()]),\n                Some(vec![\"some-string\".to_string()]),\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_crm_v_3_objects_contacts_get_page<'a>(
        &'a self,
        after: Option<String>,
        archived: Option<bool>,
        associations: Option<Vec<String>>,
        limit: Option<i32>,
        properties: Option<Vec<String>>,
        properties_with_history: Option<Vec<String>>,
    ) -> Result<
        crate::types::CollectionResponseSimplePublicObjectWithAssociationsForwardPaging,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "crm/v3/objects/contacts"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = after {
            query_params.push(("after", p));
        }

        if let Some(p) = archived {
            query_params.push(("archived", format!("{}", p)));
        }

        if let Some(p) = associations {
            query_params.push(("associations", itertools::join(p, ",")));
        }

        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = properties {
            query_params.push(("properties", itertools::join(p, ",")));
        }

        if let Some(p) = properties_with_history {
            query_params.push(("propertiesWithHistory", itertools::join(p, ",")));
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

    #[doc = "Create\n\nCreate a contact with the given properties and return a copy of the object, including the ID. Documentation and examples for creating standard contacts is provided.\n\n```rust,no_run\nasync fn example_basic_post_crm_v_3_objects_contacts_create() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::SimplePublicObject = client\n        .basic()\n        .post_crm_v_3_objects_contacts_create(&hubspot_contacts::types::SimplePublicObjectInputForCreate {\n            associations: vec![hubspot_contacts::types::PublicAssociationsForObject {\n                types: vec![hubspot_contacts::types::AssociationSpec {\n                    association_category: hubspot_contacts::types::AssociationCategory::UserDefined,\n                    association_type_id: 4 as i32,\n                }],\n                to: hubspot_contacts::types::PublicObjectId {\n                    id: \"some-string\".to_string(),\n                },\n            }],\n            properties: std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                \"some-string\".to_string(),\n            )]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_contacts_create<'a>(
        &'a self,
        body: &crate::types::SimplePublicObjectInputForCreate,
    ) -> Result<crate::types::SimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "crm/v3/objects/contacts"),
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
