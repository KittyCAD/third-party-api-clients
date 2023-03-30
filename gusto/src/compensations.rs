use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Compensations {
    pub client: Client,
}

impl Compensations {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a compensation\n\nCompensations contain information on how much is paid out for a \
             job. Jobs may have many compensations, but only one that is active. The current \
             compensation is the one with the most recent `effective_date`.\n\nNote: Currently, \
             jobs are arbitrarily limited to a single compensation as multiple compensations per \
             job are not yet available in Gusto. The API is architected as if multiple \
             compensations may exist, so integrations should integrate under the same assumption. \
             The only exception is that creating a compensation with the same `job_id` as another \
             will fail with a relevant error.\n\n\n**Parameters:**\n\n- `compensation_id: &'astr`: \
             The ID of the compensation (required)\n\n```rust,no_run\nasync fn \
             example_compensations_get_id() -> anyhow::Result<()> {\n    let client =\n        \
             gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::Compensation = \
             client.compensations().get_id(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id<'a>(
        &'a self,
        compensation_id: &'a str,
    ) -> Result<crate::types::Compensation, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/compensations/{compensation_id}".replace("{compensation_id}", compensation_id)
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

    #[doc = "Update a compensation\n\nCompensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.\n\nNote: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error\n\n**Parameters:**\n\n- `compensation_id: &'astr`: The ID of the compensation (required)\n\n```rust,no_run\nasync fn example_compensations_put_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Compensation = client\n        .compensations()\n        .put_id(\n            \"some-string\",\n            &gusto_api::types::PutCompensationsCompensationIdRequestBody {\n                version: \"some-string\".to_string(),\n                rate: Some(\"some-string\".to_string()),\n                payment_unit: Some(gusto_api::types::PaymentUnit::Month),\n                flsa_status: Some(gusto_api::types::FlsaStatus::Owner),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_id<'a>(
        &'a self,
        compensation_id: &'a str,
        body: &crate::types::PutCompensationsCompensationIdRequestBody,
    ) -> Result<crate::types::Compensation, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/compensations/{compensation_id}".replace("{compensation_id}", compensation_id)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get compensations for a job\n\nCompensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.\n\nNote: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error.\n\nUse the `flsa_status` to determine if an employee is elibgle for overtime.\n\n**Parameters:**\n\n- `job_id: &'astr`: The ID of the job to which the compensation belongs (required)\n\n```rust,no_run\nasync fn example_compensations_get_jobs_job_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Compensation> = client\n        .compensations()\n        .get_jobs_job_id(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_jobs_job_id<'a>(
        &'a self,
        job_id: &'a str,
    ) -> Result<Vec<crate::types::Compensation>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/jobs/{job_id}/compensations".replace("{job_id}", job_id)
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
