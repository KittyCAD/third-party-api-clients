use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Scim {
    pub client: Client,
}

impl Scim {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Active community members by role\n\nReturns the list of active owners, filtered based on the query parameters\n\n**Parameters:**\n\n- `count: Option<i64>`\n- `filter: Option<String>`\n- `start_index: Option<i64>`\n\n```rust,no_run\nasync fn example_scim_active_community_members_by_role() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    let result: commonroom_api::types::ActiveCommunityMembersByRoleResponse = client\n        .scim()\n        .active_community_members_by_role(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn active_community_members_by_role<'a>(
        &'a self,
        count: Option<i64>,
        filter: Option<String>,
        start_index: Option<i64>,
    ) -> Result<crate::types::ActiveCommunityMembersByRoleResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "Users"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = count {
            query_params.push(("count", format!("{}", p)));
        }

        if let Some(p) = filter {
            query_params.push(("filter", p));
        }

        if let Some(p) = start_index {
            query_params.push(("startIndex", format!("{}", p)));
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

    #[doc = "Creates the community member with specific role\n\nCreates the community member with owner role\n\n**Parameters:**\n\n- `count: Option<i64>`\n- `filter: Option<String>`\n- `start_index: Option<i64>`\n\n```rust,no_run\nasync fn example_scim_create_community_member_with_specific_role() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    let result: commonroom_api::types::User = client\n        .scim()\n        .create_community_member_with_specific_role(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            &commonroom_api::types::CreateCommunityMemberWithSpecificRoleRequestBody {\n                schemas: Some(vec![\"some-string\".to_string()]),\n                user_name: Some(\"some-string\".to_string()),\n                name: Some(commonroom_api::types::Name {\n                    given_name: Some(\"some-string\".to_string()),\n                    family_name: Some(\"some-string\".to_string()),\n                }),\n                emails: Some(vec![commonroom_api::types::Emails {\n                    value: Some(\"some-string\".to_string()),\n                    primary: Some(false),\n                    type_: Some(\"some-string\".to_string()),\n                }]),\n                active: Some(false),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_community_member_with_specific_role<'a>(
        &'a self,
        count: Option<i64>,
        filter: Option<String>,
        start_index: Option<i64>,
        body: &crate::types::CreateCommunityMemberWithSpecificRoleRequestBody,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "Users"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = count {
            query_params.push(("count", format!("{}", p)));
        }

        if let Some(p) = filter {
            query_params.push(("filter", p));
        }

        if let Some(p) = start_index {
            query_params.push(("startIndex", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "Retrieves a specific user account\n\n**Parameters:**\n\n- `email: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_scim_get_specific_user_account() -> \
             anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    \
             let result: commonroom_api::types::User = client\n        .scim()\n        \
             .get_specific_user_account(\"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_specific_user_account<'a>(
        &'a self,
        email: &'a str,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "Users/:email".replace("{email}", email)
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

    #[doc = "Updates an user account\n\nUpdates the user account specified in the \
             query\n\n**Parameters:**\n\n- `email: &'astr` (required)\n\n```rust,no_run\nasync fn \
             example_scim_update_user_account() -> anyhow::Result<()> {\n    let client = \
             commonroom_api::Client::new_from_env();\n    let result: commonroom_api::types::User \
             = client\n        .scim()\n        .update_user_account(\n            \
             \"some-string\",\n            &commonroom_api::types::UpdateUserAccountRequestBody \
             {\n                schemas: Some(vec![\"some-string\".to_string()]),\n                \
             operations: Some(vec![commonroom_api::types::Operations {\n                    op: \
             Some(\"some-string\".to_string()),\n                    value: \
             Some(commonroom_api::types::OperationsValue {\n                        active: \
             Some(false),\n                    }),\n                }]),\n            },\n        \
             )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_user_account<'a>(
        &'a self,
        email: &'a str,
        body: &crate::types::UpdateUserAccountRequestBody,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "Users/:email".replace("{email}", email)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
