use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Ats {
    pub client: Client,
}

impl Ats {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "POST New Candidate\n\nPushes a candidate from an applicant tracking system directly into the Rippling onboarding flow. Please note, this endpoint is only available to applications integrating with OAuth2.0.\n\nNOTE: This endpoint is NOT available for use with Rippling customer API Keys.\n\n```rust,no_run\nasync fn example_ats_post_candidates_push_candidate() -> anyhow::Result<()> {\n    let client = rippling_api::Client::new_from_env();\n    let result: rippling_api::types::Candidate = client\n        .ats()\n        .post_candidates_push_candidate(&rippling_api::types::Candidate {\n            name: Some(\"some-string\".to_string()),\n            email: Some(\"some-string\".to_string()),\n            job_title: Some(\"some-string\".to_string()),\n            phone_number: Some(\"some-string\".to_string()),\n            candidate_id: Some(\"some-string\".to_string()),\n            start_date: Some(chrono::Utc::now().date_naive()),\n            salary_unit: Some(rippling_api::types::SalaryUnit::PayPeriod),\n            salary_per_unit: Some(3.14 as f64),\n            signing_bonus: Some(3.14 as f64),\n            currency: Some(\"some-string\".to_string()),\n            equity_shares: Some(4 as i64),\n            department: Some(\"some-string\".to_string()),\n            employment_type: Some(rippling_api::types::CandidateEmploymentType::Temp),\n            work_location: Some(\"some-string\".to_string()),\n            attachments: Some(vec![rippling_api::types::Attachments {\n                file_name: Some(\"some-string\".to_string()),\n                file_url: Some(\"some-string\".to_string()),\n            }]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_candidates_push_candidate<'a>(
        &'a self,
        body: &crate::types::Candidate,
    ) -> Result<crate::types::Candidate, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "platform/api/ats_candidates/push_candidate"
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
