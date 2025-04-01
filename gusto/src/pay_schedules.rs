use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct PaySchedules {
    pub client: Client,
}

impl PaySchedules {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get the pay schedules for a company\n\nThe pay schedule object in Gusto captures the \
             details of when employees work and when they should be paid. A company can have \
             multiple pay schedules.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the \
             company (required)\n\n```rust,no_run\nasync fn \
             example_pay_schedules_get_companies_company_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::PaySchedule> \
             = client\n        .pay_schedules()\n        \
             .get_companies_company_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id<'a>(
        &'a self,
        company_id: &'a str,
    ) -> Result<Vec<crate::types::PaySchedule>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/pay_schedules".replace("{company_id}", company_id)
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get a pay schedule\n\nThe pay schedule object in Gusto captures the details of when \
             employees work and when they should be paid. A company can have multiple pay \
             schedules.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of \
             the company (required)\n- `pay_schedule_id_or_uuid: &'astr`: The ID or UUID of the \
             pay schedule (required)\n\n```rust,no_run\nasync fn \
             example_pay_schedules_get_companies_company_id_id() -> anyhow::Result<()> {\n    let \
             client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::PaySchedule = \
             client\n        .pay_schedules()\n        \
             .get_companies_company_id_id(\"some-string\", \"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        pay_schedule_id_or_uuid: &'a str,
    ) -> Result<crate::types::PaySchedule, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/pay_schedules/{pay_schedule_id_or_uuid}"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
                    .replace("{pay_schedule_id_or_uuid}", pay_schedule_id_or_uuid)
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update a pay schedule\n\nUpdates a pay schedule.\n\nThis endpoint is in beta. Please contact developer-gws@gusto.com if you’d like to have more information and use it for production. Note, this may require you to enter a different agreement with Gusto\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n- `pay_schedule_id_or_uuid: &'astr`: The ID or UUID of the pay schedule (required)\n\n```rust,no_run\nasync fn example_pay_schedules_put_companies_company_id_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::PaySchedule = client\n        .pay_schedules()\n        .put_companies_company_id_id(\n            \"some-string\",\n            \"some-string\",\n            &gusto_api::types::PutCompaniesCompanyIdPaySchedulesPayScheduleIdRequestBody {\n                version: \"some-string\".to_string(),\n                auto_pilot: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        pay_schedule_id_or_uuid: &'a str,
        body: &crate::types::PutCompaniesCompanyIdPaySchedulesPayScheduleIdRequestBody,
    ) -> Result<crate::types::PaySchedule, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/pay_schedules/{pay_schedule_id_or_uuid}"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
                    .replace("{pay_schedule_id_or_uuid}", pay_schedule_id_or_uuid)
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
