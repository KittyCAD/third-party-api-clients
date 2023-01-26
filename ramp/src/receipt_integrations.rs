use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ReceiptIntegrations {
    pub client: Client,
}

impl ReceiptIntegrations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Remove an email from the Email-based receipt integration opt out list, opting it in\n\n**Parameters:**\n\n- `mailbox_opted_out_email_uuid: &'astr` (required)\n\n```rust,no_run\nasync fn example_receipt_integrations_delete_opted_out_emails_delete_resource() -> anyhow::Result<()>\n{\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .receipt_integrations()\n        .delete_opted_out_emails_delete_resource(\"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_opted_out_emails_delete_resource<'a>(
        &'a self,
        mailbox_opted_out_email_uuid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/receipt-integrations/opt-out/{mailbox_opted_out_email_uuid}".replace(
                    "{mailbox_opted_out_email_uuid}",
                    &mailbox_opted_out_email_uuid
                )
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List all emails that have been opted out of Email-based receipt \
             integrations\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_receipt_integrations_get_opted_out_emails_list_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::ApiReceiptIntegrationOptedOutEmailResource = client\n        \
             .receipt_integrations()\n        .get_opted_out_emails_list_resource(\n            \
             &ramp_api::types::ApiReceiptIntegrationOptedOutEmailResource {\n                id: \
             Some(uuid::Uuid::from_str(\n                    \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n                \
             email: Some(\"email@example.com\".to_string()),\n            },\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_opted_out_emails_list_resource<'a>(
        &'a self,
        body: &crate::types::ApiReceiptIntegrationOptedOutEmailResource,
    ) -> Result<crate::types::ApiReceiptIntegrationOptedOutEmailResource, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/receipt-integrations/opt-out"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Add a new email to be opted out of Email-based receipt integrations\n\n```rust,no_run\nasync fn example_receipt_integrations_post_opted_out_emails_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiReceiptIntegrationOptedOutEmailResource = client\n        .receipt_integrations()\n        .post_opted_out_emails_list_resource(\n            &ramp_api::types::ApiReceiptIntegrationOptedOutEmailCreate {\n                business_id: 4 as i64,\n                email: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_opted_out_emails_list_resource<'a>(
        &'a self,
        body: &crate::types::ApiReceiptIntegrationOptedOutEmailCreate,
    ) -> Result<crate::types::ApiReceiptIntegrationOptedOutEmailResource, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/receipt-integrations/opt-out"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
