use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug, Default)]
pub struct GetCrmV3ObjectsContactsGetPageParams {
    pub after: Option<String>,
    pub archived: Option<bool>,
    pub associations: Option<Vec<String>>,
    pub limit: Option<i32>,
    pub properties: Option<Vec<String>>,
    pub properties_with_history: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct Basic {
    pub client: Client,
}

impl Basic {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Retrieve a contact\n\nRetrieve a contact by its ID (`contactId`) or by a unique property (`idProperty`). You can specify what is returned using the `properties` query parameter.\n\n**Parameters:**\n\n- `archived: Option<bool>`: Whether to return only results that have been archived.\n- `associations: Option<Vec<String>>`: A comma separated list of object types to retrieve associated IDs for. If any of the specified associations do not exist, they will be ignored.\n- `contact_id: &'astr`: The ID of the contact to retrieve. (required)\n- `properties: Option<Vec<String>>`: A comma separated list of the properties to be returned in the response. If any of the specified properties are not present on the requested object(s), they will be ignored.\n- `properties_with_history: Option<Vec<String>>`: A comma separated list of the properties to be returned along with their history of previous values. If any of the specified properties are not present on the requested object(s), they will be ignored.\n\n```rust,no_run\nasync fn example_basic_get_crm_v_3_objects_contacts_contact_id_get_by_id() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::SimplePublicObjectWithAssociations = client\n        .basic()\n        .get_crm_v_3_objects_contacts_contact_id_get_by_id(\n            Some(true),\n            Some(vec![\"some-string\".to_string()]),\n            \"some-string\",\n            Some(vec![\"some-string\".to_string()]),\n            Some(vec![\"some-string\".to_string()]),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_crm_v_3_objects_contacts_contact_id_get_by_id<'a>(
        &'a self,
        archived: Option<bool>,
        associations: Option<Vec<String>>,
        contact_id: &'a str,
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

    #[doc = "Archive a contact\n\nDelete a contact by ID. Deleted contacts can be restored within 90 days of deletion. Learn more about the [data impacted by contact deletions](https://knowledge.hubspot.com/privacy-and-consent/understand-restorable-and-permanent-contact-deletions) and how to [restore archived records](https://knowledge.hubspot.com/records/restore-deleted-records).\n\n**Parameters:**\n\n- `contact_id: &'astr`: The ID of the contact to delete. (required)\n\n```rust,no_run\nasync fn example_basic_delete_crm_v_3_objects_contacts_contact_id_archive() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    client\n        .basic()\n        .delete_crm_v_3_objects_contacts_contact_id_archive(\"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
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

    #[doc = "Update a contact\n\nUpdate a contact by ID (`contactId`) or unique property value (`idProperty`). Provided property values will be overwritten. Read-only and non-existent properties will result in an error. Properties values can be cleared by passing an empty string.\n\n**Parameters:**\n\n- `contact_id: &'astr`: The ID of the contact to update. (required)\n\n```rust,no_run\nasync fn example_basic_patch_crm_v_3_objects_contacts_contact_id_update() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::SimplePublicObject = client\n        .basic()\n        .patch_crm_v_3_objects_contacts_contact_id_update(\n            \"some-string\",\n            &hubspot_contacts::types::SimplePublicObjectInput {\n                properties: std::collections::HashMap::from([(\n                    \"some-key\".to_string(),\n                    \"some-string\".to_string(),\n                )]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Merge two contacts\n\nMerge two contact records. Learn more about [merging records](https://knowledge.hubspot.com/records/merge-records). \n\n```rust,no_run\nasync fn example_basic_post_crm_v_3_objects_contacts_merge_merge() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::SimplePublicObject = client\n        .basic()\n        .post_crm_v_3_objects_contacts_merge_merge(&hubspot_contacts::types::PublicMergeInput {\n            object_id_to_merge: \"some-string\".to_string(),\n            primary_object_id: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Permanently delete a contact (GDPR-compliant)\n\nPermanently delete a contact and all associated content to follow GDPR. Use optional property `idProperty` set to `email` to identify contact by email address. If email address is not found, the email address will be added to a blocklist and prevent it from being used in the future. Learn more about [permanently deleting contacts](https://knowledge.hubspot.com/privacy-and-consent/how-do-i-perform-a-gdpr-delete-in-hubspot).\n\n```rust,no_run\nasync fn example_basic_post_crm_v_3_objects_contacts_gdpr_delete_purge() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    client\n        .basic()\n        .post_crm_v_3_objects_contacts_gdpr_delete_purge(&hubspot_contacts::types::PublicGdprDeleteInput {\n            id_property: Some(\"some-string\".to_string()),\n            object_id: \"some-string\".to_string(),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_contacts_gdpr_delete_purge<'a>(
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

    #[doc = "Retrieve contacts\n\nRetrieve all contacts, using query parameters to specify the information that gets returned.\n\n**Parameters:**\n\n- `after: Option<String>`: The paging cursor token of the last successfully read resource will be returned as the `paging.next.after` JSON property of a paged response containing more results.\n- `archived: Option<bool>`: Whether to return only results that have been archived.\n- `associations: Option<Vec<String>>`: A comma separated list of object types to retrieve associated IDs for. If any of the specified associations do not exist, they will be ignored.\n- `limit: Option<i32>`: The maximum number of results to display per page.\n- `properties: Option<Vec<String>>`: A comma separated list of the properties to be returned in the response. If any of the specified properties are not present on the requested object(s), they will be ignored.\n- `properties_with_history: Option<Vec<String>>`: A comma separated list of the properties to be returned along with their history of previous values. If any of the specified properties are not present on the requested object(s), they will be ignored. Usage of this parameter will reduce the maximum number of objects that can be read by a single request.\n\n```rust,no_run\nasync fn example_basic_get_crm_v_3_objects_contacts_get_page() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::CollectionResponseSimplePublicObjectWithAssociationsForwardPaging =\n        client\n            .basic()\n            .get_crm_v_3_objects_contacts_get_page(\n                hubspot_contacts::basic::GetCrmV3ObjectsContactsGetPageParams {\n                    after: Some(\"some-string\".to_string()),\n                    archived: Some(true),\n                    associations: Some(vec![\"some-string\".to_string()]),\n                    limit: Some(4 as i32),\n                    properties: Some(vec![\"some-string\".to_string()]),\n                    properties_with_history: Some(vec![\"some-string\".to_string()]),\n                },\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_crm_v_3_objects_contacts_get_page<'a>(
        &'a self,
        params: GetCrmV3ObjectsContactsGetPageParams,
    ) -> Result<
        crate::types::CollectionResponseSimplePublicObjectWithAssociationsForwardPaging,
        crate::types::error::Error,
    > {
        let GetCrmV3ObjectsContactsGetPageParams {
            after,
            archived,
            associations,
            limit,
            properties,
            properties_with_history,
        } = params;
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

    #[doc = "Create a contact\n\nCreate a single contact. Include a `properties` object to define [property values](https://developers.hubspot.com/docs/guides/api/crm/properties) for the contact, along with an `associations` array to define [associations](https://developers.hubspot.com/docs/guides/api/crm/associations/associations-v4) with other CRM records.\n\n```rust,no_run\nasync fn example_basic_post_crm_v_3_objects_contacts_create() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::SimplePublicObject = client\n        .basic()\n        .post_crm_v_3_objects_contacts_create(&hubspot_contacts::types::SimplePublicObjectInputForCreate {\n            associations: Some(vec![hubspot_contacts::types::PublicAssociationsForObject {\n                types: vec![hubspot_contacts::types::AssociationSpec {\n                    association_category: hubspot_contacts::types::AssociationCategory::UserDefined,\n                    association_type_id: 4 as i32,\n                }],\n                to: hubspot_contacts::types::PublicObjectId {\n                    id: \"some-string\".to_string(),\n                },\n            }]),\n            properties: std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                \"some-string\".to_string(),\n            )]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
