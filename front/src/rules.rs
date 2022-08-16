use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Rules {
    pub client: Client,
}

impl Rules {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List rule\n\nList the rules of the company.\n\n```rust,no_run\nasync fn \
             example_rules_list() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListRulesResponse = client.rules().list().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListRulesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "rules"),
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

    #[doc = "List team rules\n\nList the rules of a team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_rules_list_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListTeamRulesResponse =\n        client.rules().list_team(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamRulesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/rules".replace("{team_id}", team_id)
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

    #[doc = "List teammate rule\n\nList the rules of a teammate.\n\n**Parameters:**\n\n- \
             `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn \
             example_rules_list_teammate() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTeammateRulesResponse =\n        \
             client.rules().list_teammate(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateRulesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/rules".replace("{teammate_id}", teammate_id)
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

    #[doc = "Get rule\n\nFetche a rule.\n\n**Parameters:**\n\n- `rule_id: &'astr`: The Rule ID (required)\n\n```rust,no_run\nasync fn example_rules_get() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::RuleResponse = client.rules().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        rule_id: &'a str,
    ) -> Result<crate::types::RuleResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "rules/{rule_id}".replace("{rule_id}", rule_id)
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
}
