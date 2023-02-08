use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Jobs {
    pub client: Client,
}

impl Jobs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a job\n\nGet a job.\n\n**Parameters:**\n\n- `job_id: &'astr`: The job ID \
             (required)\n\n```rust,no_run\nasync fn example_jobs_get_id() -> anyhow::Result<()> \
             {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::Job = \
             client.jobs().get_id(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_id<'a>(
        &'a self,
        job_id: &'a str,
    ) -> Result<crate::types::Job, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/jobs/{job_id}".replace("{job_id}", job_id)
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

    #[doc = "Update a job\n\nUpdate a job.\n\n**Parameters:**\n\n- `job_id: &'astr`: The job ID (required)\n\n```rust,no_run\nasync fn example_jobs_put_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Job = client\n        .jobs()\n        .put_id(\n            \"some-string\",\n            &gusto_api::types::PutJobsJobIdRequestBody {\n                version: \"some-string\".to_string(),\n                title: Some(\"some-string\".to_string()),\n                location_id: Some(3.14 as f64),\n                hire_date: Some(chrono::Utc::now().date().naive_utc()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_id<'a>(
        &'a self,
        job_id: &'a str,
        body: &crate::types::PutJobsJobIdRequestBody,
    ) -> Result<crate::types::Job, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/jobs/{job_id}".replace("{job_id}", job_id)
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

    #[doc = "Delete an individual job\n\nDeletes a specific job that an employee \
             holds.\n\n**Parameters:**\n\n- `job_id: &'astr`: The job ID \
             (required)\n\n```rust,no_run\nasync fn example_jobs_delete_id() -> anyhow::Result<()> \
             {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    \
             client.jobs().delete_id(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_id<'a>(
        &'a self,
        job_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/jobs/{job_id}".replace("{job_id}", job_id)
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

    #[doc = "Get jobs for an employee\n\nGet all of the jobs that an employee \
             holds.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The employee ID \
             (required)\n\n```rust,no_run\nasync fn example_jobs_get_employees_employee_id() -> \
             anyhow::Result<()> {\n    let client =\n        \
             gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Job> = \
             client\n        .jobs()\n        .get_employees_employee_id(\"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_employees_employee_id<'a>(
        &'a self,
        employee_id: &'a str,
    ) -> Result<Vec<crate::types::Job>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/jobs".replace("{employee_id}", employee_id)
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

    #[doc = "Create a job\n\nCreate a job.\n\n**Parameters:**\n\n- `employee_id: &'astr`: The employee ID (required)\n\n```rust,no_run\nasync fn example_jobs_post_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Job = client\n        .jobs()\n        .post_id(\n            \"some-string\",\n            &gusto_api::types::PostJobsJobIdRequestBody {\n                title: Some(\"some-string\".to_string()),\n                location_id: Some(3.14 as f64),\n                hire_date: Some(chrono::Utc::now().date().naive_utc()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_id<'a>(
        &'a self,
        employee_id: &'a str,
        body: &crate::types::PostJobsJobIdRequestBody,
    ) -> Result<crate::types::Job, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/employees/{employee_id}/jobs".replace("{employee_id}", employee_id)
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

    #[doc = "Create a compensation\n\nCompensations contain information on how much is paid out for a job. Jobs may have many compensations, but only one that is active. The current compensation is the one with the most recent `effective_date`.\n\nNote: Currently, jobs are arbitrarily limited to a single compensation as multiple compensations per job are not yet available in Gusto. The API is architected as if multiple compensations may exist, so integrations should integrate under the same assumption. The only exception is that creating a compensation with the same `job_id` as another will fail with a relevant error\n\n**Parameters:**\n\n- `job_id: &'astr`: The ID of the job to which the compensation belongs (required)\n\n```rust,no_run\nasync fn example_jobs_post_id_compensations() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Compensation = client\n        .jobs()\n        .post_id_compensations(\n            \"some-string\",\n            &gusto_api::types::PostJobsJobIdCompensationsRequestBody {\n                rate: \"some-string\".to_string(),\n                payment_unit: gusto_api::types::PaymentUnit::Month,\n                flsa_status: gusto_api::types::FlsaStatus::Nonexempt,\n                effective_date: Some(chrono::Utc::now().date().naive_utc()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_id_compensations<'a>(
        &'a self,
        job_id: &'a str,
        body: &crate::types::PostJobsJobIdCompensationsRequestBody,
    ) -> Result<crate::types::Compensation, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/jobs/{job_id}/compensations".replace("{job_id}", job_id)
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
}
