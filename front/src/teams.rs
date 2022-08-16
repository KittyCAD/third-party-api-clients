use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Teams {
    pub client: Client,
}

impl Teams {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List teams\n\nList the teams in the company.\n\n```rust,no_run\nasync fn \
             example_teams_list() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTeamsResponse = client.teams().list().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListTeamsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "teams"),
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

    #[doc = "Get team\n\nFetch a team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The Team ID (required)\n\n```rust,no_run\nasync fn example_teams_get() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::TeamResponse = client.teams().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::TeamResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}".replace("{team_id}", team_id)
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

    #[doc = "Add teammates to team\n\nAdd one or more teammates to a team.\n\n**Parameters:**\n\n- \
             `team_id: &'astr`: The Team ID (required)\n\n```rust,no_run\nasync fn \
             example_teams_add_teammates_to() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    client\n        .teams()\n        \
             .add_teammates_to(\n            \"some-string\",\n            \
             &front_api::types::TeammateIds {\n                teammate_ids: \
             vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_teammates_to<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::TeammateIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/teammates".replace("{team_id}", team_id)
            ),
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

    #[doc = "Remove teammates from team\n\nRemove one or more teammates from a \
             team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The Team ID \
             (required)\n\n```rust,no_run\nasync fn example_teams_remove_teammates_from() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .teams()\n        .remove_teammates_from(\n            \
             \"some-string\",\n            &front_api::types::TeammateIds {\n                \
             teammate_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove_teammates_from<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::TeammateIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/teammates".replace("{team_id}", team_id)
            ),
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
}
