use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Members {
    pub client: Client,
}

impl Members {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Create or update Community Member\n\nCreate or Update a Community Member\n\n\n```rust,no_run\nasync fn example_members_create_or_update_community() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    client\n        .members()\n        .create_or_update_community(&commonroom_api::types::CreateOrUpdateCommunityMemberRequestBody {\n            source: \"some-string\".to_string(),\n            full_name: Some(\"some-string\".to_string()),\n            avatar_url: Some(\"some-string\".to_string()),\n            description: Some(\"some-string\".to_string()),\n            socials: Some(vec![\n                commonroom_api::types::CreateOrUpdateCommunityMemberRequestBodySocials::Email(\n                    commonroom_api::types::Email {\n                        type_: Some(commonroom_api::types::EmailType::Email),\n                        value: Some(\"some-string\".to_string()),\n                    },\n                ),\n            ]),\n            segment: Some(3.14 as f64),\n            company: Some(\"some-string\".to_string()),\n            domain: Some(\"some-string\".to_string()),\n            title: Some(\"some-string\".to_string()),\n            role: Some(\"some-string\".to_string()),\n            location: Some(commonroom_api::types::Location {\n                city: Some(\"some-string\".to_string()),\n                region: Some(\"some-string\".to_string()),\n                country: Some(\"some-string\".to_string()),\n            }),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_or_update_community<'a>(
        &'a self,
        body: &crate::types::CreateOrUpdateCommunityMemberRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "members/"),
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

    #[doc = "Add a Note to a Community Member\n\nAdd a Note to a Community \
             Member\n\n\n```rust,no_run\nasync fn example_members_add_note_to() -> \
             anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    \
             client\n        .members()\n        \
             .add_note_to(&commonroom_api::types::AddNoteToMemberRequestBody {\n            \
             social_type: Some(\"some-string\".to_string()),\n            value: \
             Some(\"some-string\".to_string()),\n            note: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_note_to<'a>(
        &'a self,
        body: &crate::types::AddNoteToMemberRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "members/note"),
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

    #[doc = "Gets all member custom fields for a community\n\nGets all member custom fields for a \
             community\n\n\n```rust,no_run\nasync fn example_members_get_custom_fields() -> \
             anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    \
             let result: Vec<commonroom_api::types::GetMemberCustomFieldsResponse> =\n        \
             client.members().get_custom_fields().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_fields<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::GetMemberCustomFieldsResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "members/customFields"),
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
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Set an existing Custom Field for a Community Member\n\nSet an existing Custom Field for a Community Member\n\n\n```rust,no_run\nasync fn example_members_set_custom_field_value() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    client\n        .members()\n        .set_custom_field_value(&commonroom_api::types::SetMemberCustomFieldValueRequestBody {\n            social_type: Some(\"some-string\".to_string()),\n            value: Some(\"some-string\".to_string()),\n            custom_field_id: Some(3.14 as f64),\n            custom_field_value: Some(commonroom_api::types::CustomFieldValue {\n                type_: Some(\"some-string\".to_string()),\n                value: Some(serde_json::Value::String(\"some-string\".to_string())),\n            }),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn set_custom_field_value<'a>(
        &'a self,
        body: &crate::types::SetMemberCustomFieldValueRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "members/customFields"),
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

    #[doc = "Adds a new or existing tag to a Community Member\n\nAdds a new or existing tag to a \
             Community Member\n\n\n```rust,no_run\nasync fn example_members_add_tags_to() -> \
             anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    \
             client\n        .members()\n        \
             .add_tags_to(&commonroom_api::types::AddTagsToMemberRequestBody {\n            \
             social_type: Some(\"some-string\".to_string()),\n            value: \
             Some(\"some-string\".to_string()),\n            tags: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_tags_to<'a>(
        &'a self,
        body: &crate::types::AddTagsToMemberRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "members/tags"),
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

    #[doc = "Get Community Member by E-Mail\n\nA Community Member's profile based on an email \
             address.\n\n**Parameters:**\n\n- `email: &'astr` (required)\n\n```rust,no_run\nasync \
             fn example_members_get_by_email() -> anyhow::Result<()> {\n    let client = \
             commonroom_api::Client::new_from_env();\n    let result: \
             Vec<commonroom_api::types::CommunityMember> =\n        \
             client.members().get_by_email(\"email@example.com\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_by_email<'a>(
        &'a self,
        email: &'a str,
    ) -> Result<Vec<crate::types::CommunityMember>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/{email}".replace("{email}", email)
            ),
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
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Anonymize Community Member\n\nRequest removal of all personally identifiable \
             information (PII) for the\nCommunity Member associated by this email address.\n\nThis \
             does not immediately anonymize the community member. The anonymization\nis queued and \
             will happen at a future time within 15 days.\n\n\n**Parameters:**\n\n- `email: \
             &'astr` (required)\n\n```rust,no_run\nasync fn example_members_anonymize() -> \
             anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    \
             let result: String = client.members().anonymize(\"email@example.com\").await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn anonymize<'a>(
        &'a self,
        email: &'a str,
    ) -> Result<String, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/{email}".replace("{email}", email)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await?;
            Ok(text)
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get Community Member By E-Mail or Socials\n\n**Parameters:**\n\n- `email: Option<String>`\n- `github: Option<String>`\n- `linkedin: Option<String>`\n- `twitter: Option<String>`\n\n```rust,no_run\nasync fn example_members_get_by_socials() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    let result: Vec<commonroom_api::types::GetMemberBySocialsResponse> = client\n        .members()\n        .get_by_socials(\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_by_socials<'a>(
        &'a self,
        email: Option<String>,
        github: Option<String>,
        linkedin: Option<String>,
        twitter: Option<String>,
    ) -> Result<Vec<crate::types::GetMemberBySocialsResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "members"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = email {
            query_params.push(("email", p));
        }

        if let Some(p) = github {
            query_params.push(("github", p));
        }

        if let Some(p) = linkedin {
            query_params.push(("linkedin", p));
        }

        if let Some(p) = twitter {
            query_params.push(("twitter", p));
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
