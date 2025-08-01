use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Search {
    pub client: Client,
}

impl Search {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Search for contacts\n\nSearch for contacts by filtering on properties, searching through associations, and sorting results. Learn more about [CRM search](https://developers.hubspot.com/docs/guides/api/crm/search#make-a-search-request).\n\n```rust,no_run\nasync fn example_search_post_crm_v_3_objects_contacts_do() -> anyhow::Result<()> {\n    let client = hubspot_contacts::Client::new_from_env();\n    let result: hubspot_contacts::types::CollectionResponseWithTotalSimplePublicObjectForwardPaging = client\n        .search()\n        .post_crm_v_3_objects_contacts_do(&hubspot_contacts::types::PublicObjectSearchRequest {\n            query: Some(\"some-string\".to_string()),\n            limit: Some(4 as i32),\n            after: Some(\"some-string\".to_string()),\n            sorts: Some(vec![\"some-string\".to_string()]),\n            properties: Some(vec![\"some-string\".to_string()]),\n            filter_groups: Some(vec![hubspot_contacts::types::FilterGroup {\n                filters: vec![hubspot_contacts::types::Filter {\n                    high_value: Some(\"some-string\".to_string()),\n                    property_name: \"some-string\".to_string(),\n                    values: Some(vec![\"some-string\".to_string()]),\n                    value: Some(\"some-string\".to_string()),\n                    operator: hubspot_contacts::types::Operator::NotIn,\n                }],\n            }]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_crm_v_3_objects_contacts_do<'a>(
        &'a self,
        body: &crate::types::PublicObjectSearchRequest,
    ) -> Result<
        crate::types::CollectionResponseWithTotalSimplePublicObjectForwardPaging,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "crm/v3/objects/contacts/search"
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
