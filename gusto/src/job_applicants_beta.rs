use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct JobApplicantsBeta {
    pub client: Client,
}

impl JobApplicantsBeta {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get all job applicants for a company\n\n*This endpoint is in beta - we will be making \
             breaking changes based on developer feedback.\n\nReturns all job applicants for a \
             company.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company \
             (required)\n\n```rust,no_run\nasync fn \
             example_job_applicants_beta_get_companies_company_id_job_applicants() -> \
             anyhow::Result<()>\n{\n    let client =\n        \
             gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             Vec<gusto_api::types::JobApplicant> = client\n        .job_applicants_beta()\n        \
             .get_companies_company_id_job_applicants(\"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_job_applicants<'a>(
        &'a self,
        company_id: &'a str,
    ) -> Result<Vec<crate::types::JobApplicant>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/job_applicants".replace("{company_id}", company_id)
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

    #[doc = "Create a job applicant\n\n*This endpoint is in beta - we will be making breaking changes based on developer feedback.\n\nCreate a job applicant.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n\n```rust,no_run\nasync fn example_job_applicants_beta_post_companies_company_id_job_applicants() -> anyhow::Result<()>\n{\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::JobApplicant = client\n        .job_applicants_beta()\n        .post_companies_company_id_job_applicants(\n            \"some-string\",\n            &gusto_api::types::PostCompaniesCompanyIdJobApplicantsRequestBody {\n                first_name: \"some-string\".to_string(),\n                last_name: \"some-string\".to_string(),\n                email: \"some-string\".to_string(),\n                phone: Some(\"some-string\".to_string()),\n                onboarding_person_type: Some(gusto_api::types::OnboardingPersonType::Employee),\n                send_offer: Some(true),\n                job_title: Some(\"some-string\".to_string()),\n                date_of_birth: Some(chrono::Utc::now().date().naive_utc()),\n                start_date: Some(chrono::Utc::now().date().naive_utc()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id_job_applicants<'a>(
        &'a self,
        company_id: &'a str,
        body: &crate::types::PostCompaniesCompanyIdJobApplicantsRequestBody,
    ) -> Result<crate::types::JobApplicant, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/job_applicants".replace("{company_id}", company_id)
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

    #[doc = "Get a job applicant\n\n*This endpoint is in beta - we will be making breaking changes based on developer feedback.\n\nReturns a single job applicant.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n- `job_applicant_uuid: &'astr`: The UUID of the job applicant (required)\n\n```rust,no_run\nasync fn example_job_applicants_beta_get_companies_company_id_job_applicants_job_applicant_uuid(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::JobApplicant = client\n        .job_applicants_beta()\n        .get_companies_company_id_job_applicants_job_applicant_uuid(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_job_applicants_job_applicant_uuid<'a>(
        &'a self,
        company_id: &'a str,
        job_applicant_uuid: &'a str,
    ) -> Result<crate::types::JobApplicant, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/job_applicants/{job_applicant_uuid}"
                    .replace("{company_id}", company_id)
                    .replace("{job_applicant_uuid}", job_applicant_uuid)
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

    #[doc = "Update a job applicant\n\n*This endpoint is in beta - we will be making breaking changes based on developer feedback.\n\nUpdate an existing job applicant (only allowed when the job applicant has not been imported).\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n- `job_applicant_uuid: &'astr`: The UUID of the job applicant (required)\n\n```rust,no_run\nasync fn example_job_applicants_beta_put_companies_company_id_job_applicants_job_applicant_uuid(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::JobApplicant = client\n        .job_applicants_beta()\n        .put_companies_company_id_job_applicants_job_applicant_uuid(\n            \"some-string\",\n            \"some-string\",\n            &gusto_api::types::PutCompaniesCompanyIdJobApplicantsJobApplicantUuidRequestBody {\n                first_name: Some(\"some-string\".to_string()),\n                last_name: Some(\"some-string\".to_string()),\n                email: Some(\"some-string\".to_string()),\n                phone: Some(\"some-string\".to_string()),\n                onboarding_person_type: Some(gusto_api::types::OnboardingPersonType::Employee),\n                send_offer: Some(false),\n                job_title: Some(\"some-string\".to_string()),\n                date_of_birth: Some(chrono::Utc::now().date().naive_utc()),\n                start_date: Some(chrono::Utc::now().date().naive_utc()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_job_applicants_job_applicant_uuid<'a>(
        &'a self,
        company_id: &'a str,
        job_applicant_uuid: &'a str,
        body: &crate::types::PutCompaniesCompanyIdJobApplicantsJobApplicantUuidRequestBody,
    ) -> Result<crate::types::JobApplicant, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/job_applicants/{job_applicant_uuid}"
                    .replace("{company_id}", company_id)
                    .replace("{job_applicant_uuid}", job_applicant_uuid)
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

    #[doc = "Delete a job applicant\n\n*This endpoint is in beta - we will be making breaking changes based on developer feedback.\n\nPermanently remove a job applicant by uuid (only allowed when the job applicant has not been imported).\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n- `job_applicant_uuid: &'astr`: The UUID of the job applicant (required)\n\n```rust,no_run\nasync fn example_job_applicants_beta_delete_companies_company_id_job_applicants_job_applicant_uuid(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .job_applicants_beta()\n        .delete_companies_company_id_job_applicants_job_applicant_uuid(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_companies_company_id_job_applicants_job_applicant_uuid<'a>(
        &'a self,
        company_id: &'a str,
        job_applicant_uuid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/job_applicants/{job_applicant_uuid}"
                    .replace("{company_id}", company_id)
                    .replace("{job_applicant_uuid}", job_applicant_uuid)
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
}
