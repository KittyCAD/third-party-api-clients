use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct GetCrmV3ObjectsTicketsTicketIdGetByIdParams<'a> {
    pub archived: Option<bool>,
    pub associations: Option<Vec<String>>,
    pub id_property: Option<String>,
    pub properties: Option<Vec<String>>,
    pub properties_with_history: Option<Vec<String>>,
    pub ticket_id: &'a str,
}

impl<'a> GetCrmV3ObjectsTicketsTicketIdGetByIdParams<'a> {
    pub fn new(ticket_id: &'a str) -> Self {
        Self {
            archived: Default::default(),
            associations: Default::default(),
            id_property: Default::default(),
            properties: Default::default(),
            properties_with_history: Default::default(),
            ticket_id,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct GetCrmV3ObjectsTicketsGetPageParams {
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

    #[doc = "Retrieve a ticket\n\nRetrieve a ticket by its ID (`ticketId`) or by a unique property (`idProperty`). You can specify what is returned using the `properties` query parameter.\n\n**Parameters:**\n\n- `archived: Option<bool>`: Whether to return only results that have been archived.\n- `associations: Option<Vec<String>>`: A comma separated list of object types to retrieve associated IDs for. If any of the specified associations do not exist, they will be ignored.\n- `id_property: Option<String>`: The name of a property whose values are unique for this object\n- `properties: Option<Vec<String>>`: A comma separated list of the properties to be returned in the response. If any of the specified properties are not present on the requested object(s), they will be ignored.\n- `properties_with_history: Option<Vec<String>>`: A comma separated list of the properties to be returned along with their history of previous values. If any of the specified properties are not present on the requested object(s), they will be ignored.\n- `ticket_id: &'astr`: The ID of the ticket. (required)\n\n```rust,no_run\nasync fn example_basic_get_crm_v_3_objects_tickets_ticket_id_get_by_id() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    let result: hubspot_tickets::types::SimplePublicObjectWithAssociations = client\n        .basic()\n        .get_crm_v_3_objects_tickets_ticket_id_get_by_id(\n            hubspot_tickets::basic::GetCrmV3ObjectsTicketsTicketIdGetByIdParams {\n                archived: Some(true),\n                associations: Some(vec![\"some-string\".to_string()]),\n                id_property: Some(\"some-string\".to_string()),\n                properties: Some(vec![\"some-string\".to_string()]),\n                properties_with_history: Some(vec![\"some-string\".to_string()]),\n                ticket_id: \"some-string\",\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_crm_v_3_objects_tickets_ticket_id_get_by_id<'a>(
        &'a self,
        params: GetCrmV3ObjectsTicketsTicketIdGetByIdParams<'a>,
    ) -> Result<crate::types::SimplePublicObjectWithAssociations, crate::types::error::Error> {
        let GetCrmV3ObjectsTicketsTicketIdGetByIdParams {
            archived,
            associations,
            id_property,
            properties,
            properties_with_history,
            ticket_id,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "crm/v3/objects/tickets/{ticketId}".replace("{ticketId}", ticket_id)
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

    #[doc = "Archive a ticket\n\nArchive a ticket, sending it to the recycling bin. Deleted tickets can be restored within 90 days of deletion. Learn more about [restoring records](https://knowledge.hubspot.com/records/restore-deleted-records).\n\n**Parameters:**\n\n- `ticket_id: &'astr`: The ID of the ticket to delete. (required)\n\n```rust,no_run\nasync fn example_basic_delete_crm_v_3_objects_tickets_ticket_id_archive() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    client\n        .basic()\n        .delete_crm_v_3_objects_tickets_ticket_id_archive(\"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_crm_v_3_objects_tickets_ticket_id_archive<'a>(
        &'a self,
        ticket_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "crm/v3/objects/tickets/{ticketId}".replace("{ticketId}", ticket_id)
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

    #[doc = "Update a ticket\n\n\nUpdate a ticket by ID (`ticketId`) or unique property value (`idProperty`). Provided property values will be overwritten. Read-only and non-existent properties will result in an error. Properties values can be cleared by passing an empty string.\n\n**Parameters:**\n\n- `id_property: Option<String>`: The name of a property whose values are unique for this object\n- `ticket_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_basic_patch_crm_v_3_objects_tickets_ticket_id_update() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    let result: hubspot_tickets::types::SimplePublicObject = client\n        .basic()\n        .patch_crm_v_3_objects_tickets_ticket_id_update(\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            &hubspot_tickets::types::SimplePublicObjectInput {\n                object_write_trace_id: Some(\"some-string\".to_string()),\n                properties: std::collections::HashMap::from([(\n                    \"some-key\".to_string(),\n                    \"some-string\".to_string(),\n                )]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_crm_v_3_objects_tickets_ticket_id_update<'a>(
        &'a self,
        id_property: Option<String>,
        ticket_id: &'a str,
        body: &crate::types::SimplePublicObjectInput,
    ) -> Result<crate::types::SimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "crm/v3/objects/tickets/{ticketId}".replace("{ticketId}", ticket_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = id_property {
            query_params.push(("idProperty", p));
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

    #[doc = "Merge two tickets with same type\n\nMerge two tickets, combining them into one ticket \
             record.\n\n```rust,no_run\nasync fn \
             example_basic_post_crm_v_3_objects_tickets_merge_merge() -> anyhow::Result<()> {\n    \
             let client = hubspot_tickets::Client::new_from_env();\n    let result: \
             hubspot_tickets::types::SimplePublicObject = client\n        .basic()\n        \
             .post_crm_v_3_objects_tickets_merge_merge(&hubspot_tickets::types::PublicMergeInput \
             {\n            object_id_to_merge: \"some-string\".to_string(),\n            \
             primary_object_id: \"some-string\".to_string(),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_tickets_merge_merge<'a>(
        &'a self,
        body: &crate::types::PublicMergeInput,
    ) -> Result<crate::types::SimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/tickets/merge"
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

    #[doc = "Retrieve tickets\n\nRetrieve a ticket by its ID (`ticketId`) or by a unique property (`idProperty`). You can specify what is returned using the `properties` query parameter.\n\n**Parameters:**\n\n- `after: Option<String>`: The paging cursor token of the last successfully read resource will be returned as the `paging.next.after` JSON property of a paged response containing more results.\n- `archived: Option<bool>`: Whether to return only results that have been archived.\n- `associations: Option<Vec<String>>`: A comma separated list of object types to retrieve associated IDs for. If any of the specified associations do not exist, they will be ignored.\n- `limit: Option<i32>`: The maximum number of results to display per page.\n- `properties: Option<Vec<String>>`: A comma separated list of the properties to be returned in the response. If any of the specified properties are not present on the requested object(s), they will be ignored.\n- `properties_with_history: Option<Vec<String>>`: A comma separated list of the properties to be returned along with their history of previous values. If any of the specified properties are not present on the requested object(s), they will be ignored. Usage of this parameter will reduce the maximum number of objects that can be read by a single request.\n\n```rust,no_run\nasync fn example_basic_get_crm_v_3_objects_tickets_get_page() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    let result: hubspot_tickets::types::CollectionResponseSimplePublicObjectWithAssociationsForwardPaging =\n        client\n            .basic()\n            .get_crm_v_3_objects_tickets_get_page(\n                hubspot_tickets::basic::GetCrmV3ObjectsTicketsGetPageParams {\n                    after: Some(\"some-string\".to_string()),\n                    archived: Some(true),\n                    associations: Some(vec![\"some-string\".to_string()]),\n                    limit: Some(4 as i32),\n                    properties: Some(vec![\"some-string\".to_string()]),\n                    properties_with_history: Some(vec![\"some-string\".to_string()]),\n                },\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_crm_v_3_objects_tickets_get_page<'a>(
        &'a self,
        params: GetCrmV3ObjectsTicketsGetPageParams,
    ) -> Result<
        crate::types::CollectionResponseSimplePublicObjectWithAssociationsForwardPaging,
        crate::types::error::Error,
    > {
        let GetCrmV3ObjectsTicketsGetPageParams {
            after,
            archived,
            associations,
            limit,
            properties,
            properties_with_history,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "crm/v3/objects/tickets"),
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

    #[doc = "Create a ticket\n\nCreate a single ticket. Include a `properties` object to define [property values](https://developers.hubspot.com/docs/guides/api/crm/properties) for the ticket, along with an `associations` array to define [associations](https://developers.hubspot.com/docs/guides/api/crm/associations/associations-v4) with other CRM records.\n\n```rust,no_run\nasync fn example_basic_post_crm_v_3_objects_tickets_create() -> anyhow::Result<()> {\n    let client = hubspot_tickets::Client::new_from_env();\n    let result: hubspot_tickets::types::SimplePublicObject = client\n        .basic()\n        .post_crm_v_3_objects_tickets_create(&hubspot_tickets::types::SimplePublicObjectInputForCreate {\n            associations: vec![hubspot_tickets::types::PublicAssociationsForObject {\n                types: vec![hubspot_tickets::types::AssociationSpec {\n                    association_category: hubspot_tickets::types::AssociationCategory::UserDefined,\n                    association_type_id: 4 as i32,\n                }],\n                to: hubspot_tickets::types::PublicObjectId {\n                    id: \"some-string\".to_string(),\n                },\n            }],\n            object_write_trace_id: Some(\"some-string\".to_string()),\n            properties: std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                \"some-string\".to_string(),\n            )]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_tickets_create<'a>(
        &'a self,
        body: &crate::types::SimplePublicObjectInputForCreate,
    ) -> Result<crate::types::SimplePublicObject, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "crm/v3/objects/tickets"),
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
