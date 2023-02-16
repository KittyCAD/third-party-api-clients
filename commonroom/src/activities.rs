use crate::Client;
use anyhow::Result;
#[derive(Clone, Debug)]
pub struct Activities {
    pub client: Client,
}

impl Activities {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Add an activity to existing community member(s)\n\nAdd an activity to existing community member(s) with matching socials\n\n\n```rust,no_run\nasync fn example_activities_create_user_input_activity() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    client\n        .activities()\n        .create_user_input_activity(&commonroom_api::types::CreateUserInputActivityRequestBody {\n            source: \"some-string\".to_string(),\n            social_type: \"some-string\".to_string(),\n            value: \"some-string\".to_string(),\n            activity_type: commonroom_api::types::ActivityType {},\n            activity_body: \"some-string\".to_string(),\n            occurred_at: Some(\"some-string\".to_string()),\n            tags: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_input_activity<'a>(
        &'a self,
        body: &crate::types::CreateUserInputActivityRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "activities"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get Activity Types\n\nGets all activity types\n\n\n```rust,no_run\nasync fn example_activities_get_activity_types() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    let result: Vec<commonroom_api::types::GetActivityTypesResponse> =\n        client.activities().get_activity_types().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_activity_types<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::GetActivityTypesResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "activityTypes"),
        );
        req = req.bearer_auth(&self.client.token);
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
