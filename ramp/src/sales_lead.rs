use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct SalesLead {
    pub client: Client,
}

impl SalesLead {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Fetch a sales lead by ID\n\n**Parameters:**\n\n- `sales_lead_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_sales_lead_get_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::Lead = \
             client.sales_lead().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        sales_lead_id: &'a str,
    ) -> Result<crate::types::Lead, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/leads/{sales_lead_id}".replace("{sales_lead_id}", sales_lead_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Create a sales lead\n\n```rust,no_run\nasync fn example_sales_lead_post_creation() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .sales_lead()\n        .post_creation(&ramp_api::types::ApiSalesLeadCreate {\n            phone: Some(\"some-string\".to_string()),\n            state: \"some-string\".to_string(),\n            source: ramp_api::types::ApiSalesLeadCreateSource::AngelList,\n            last_name: \"some-string\".to_string(),\n            redirect_uri: \"some-string\".to_string(),\n            email: \"some-string\".to_string(),\n            first_name: \"some-string\".to_string(),\n            external_id: \"some-string\".to_string(),\n            business_info: Some(ramp_api::types::ApiSalesLeadBusinessLoad {\n                office_address: Some(ramp_api::types::ApiSalesLeadOfficeAddress {\n                    office_postal_code: Some(\"some-string\".to_string()),\n                    office_state: Some(\"some-string\".to_string()),\n                    office_city: Some(\"some-string\".to_string()),\n                    office_street_address: Some(\"some-string\".to_string()),\n                    office_apt_suite: Some(\"some-string\".to_string()),\n                    office_country: Some(\"some-string\".to_string()),\n                }),\n                date_of_incorporation: Some(chrono::Utc::now().date().naive_utc()),\n                state_of_incorporation: Some(\"some-string\".to_string()),\n                business_description: Some(\"some-string\".to_string()),\n                ein_number: Some(\"some-string\".to_string()),\n                sector: Some(\"some-string\".to_string()),\n                business_name_website: Some(\"some-string\".to_string()),\n                estimated_monthly_spend: Some(\"some-string\".to_string()),\n                business_name_legal: \"some-string\".to_string(),\n                industry: Some(\"some-string\".to_string()),\n                business_name_dba: \"some-string\".to_string(),\n                industry_group: Some(\"some-string\".to_string()),\n                office_phone_number: Some(\"some-string\".to_string()),\n                entity_type: Some(ramp_api::types::EntityType::Other),\n                sub_industry: Some(\"some-string\".to_string()),\n            }),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_creation<'a>(
        &'a self,
        body: &crate::types::ApiSalesLeadCreate,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "developer/v1/leads/"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Upload documents required by financing application process\n\n**Parameters:**\n\n- `sales_lead_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_sales_lead_post_document_upload() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::Upload = client\n        .sales_lead()\n        .post_document_upload(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_document_upload<'a>(
        &'a self,
        sales_lead_id: &'a str,
    ) -> Result<crate::types::Upload, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/leads/{sales_lead_id}/upload_document"
                    .replace("{sales_lead_id}", sales_lead_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
}
