use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Shifts {
    pub client: Client,
}

impl Shifts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List Shifts\n\nList the shifts.\n\n```rust,no_run\nasync fn example_shifts_list() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListShiftsResponse = client.shifts().list().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListShiftsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "shifts"),
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

    #[doc = "Create shift\n\nCreate a shift.\n\n```rust,no_run\nasync fn example_shifts_create() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ShiftResponse = client\n        .shifts()\n        .create(&serde_json::Value::String(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateShift,
    ) -> Result<crate::types::ShiftResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "shifts"),
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

    #[doc = "List team Shifts\n\nList the shifts for a team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_shifts_list_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListTeamShiftsResponse =\n        client.shifts().list_team(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamShiftsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/shifts".replace("{team_id}", team_id)
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

    #[doc = "Create team shift\n\nCreate a shift for a team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The Team ID (required)\n\n```rust,no_run\nasync fn example_shifts_create_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ShiftResponse = client\n        .shifts()\n        .create_team(\n            \"some-string\",\n            &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateShift,
    ) -> Result<crate::types::ShiftResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/shifts".replace("{team_id}", team_id)
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

    #[doc = "List Teammate Shifts\n\nLists all the shifts for the \
             teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID \
             (required)\n\n```rust,no_run\nasync fn example_shifts_list_teammate() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListTeammateShiftsResponse =\n        \
             client.shifts().list_teammate(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateShiftsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/shifts".replace("{teammate_id}", teammate_id)
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

    #[doc = "Get shift\n\nFetch a shift.\n\n**Parameters:**\n\n- `shift_id: &'astr`: The Shift ID (required)\n\n```rust,no_run\nasync fn example_shifts_get() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ShiftResponse = client.shifts().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        shift_id: &'a str,
    ) -> Result<crate::types::ShiftResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "shift/{shift_id}".replace("{shift_id}", shift_id)
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

    #[doc = "List shift's teammates\n\nList the teammates assigned to a \
             shift.\n\n**Parameters:**\n\n- `shift_id: &'astr`: The Shift ID \
             (required)\n\n```rust,no_run\nasync fn example_shifts_list_teammates() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListShiftTeammatesResponse =\n        \
             client.shifts().list_teammates(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammates<'a>(
        &'a self,
        shift_id: &'a str,
    ) -> Result<crate::types::ListShiftTeammatesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "shift/{shift_id}/teammates".replace("{shift_id}", shift_id)
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

    #[doc = "Add teammates to shift\n\nAdd teammates to a shift. The selected teammates must be in the team that owns the shift.\n\n**Parameters:**\n\n- `shift_id: &'astr`: The Shift ID (required)\n\n```rust,no_run\nasync fn example_shifts_add_teammates_to() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .shifts()\n        .add_teammates_to(\n            \"some-string\",\n            &front_api::types::TeammateIds {\n                teammate_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_teammates_to<'a>(
        &'a self,
        shift_id: &'a str,
        body: &crate::types::TeammateIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "shift/{shift_id}/teammates".replace("{shift_id}", shift_id)
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

    #[doc = "Remove teammates from shift\n\nRemove teammates from a shift.\n\n**Parameters:**\n\n- \
             `shift_id: &'astr`: The Shift ID (required)\n\n```rust,no_run\nasync fn \
             example_shifts_remove_teammates_from() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    client\n        .shifts()\n        \
             .remove_teammates_from(\n            \"some-string\",\n            \
             &front_api::types::TeammateIds {\n                teammate_ids: \
             vec![\"some-string\".to_string()],\n            },\n        )\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove_teammates_from<'a>(
        &'a self,
        shift_id: &'a str,
        body: &crate::types::TeammateIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "shift/{shift_id}/teammates".replace("{shift_id}", shift_id)
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

    #[doc = "Update shift\n\nUpdate a shift.\n\n**Parameters:**\n\n- `shift_id: &'astr`: The Shift ID (required)\n\n```rust,no_run\nasync fn example_shifts_update() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    client\n        .shifts()\n        .update(\n            \"some-string\",\n            &front_api::types::UpdateShift {\n                name: Some(\"some-string\".to_string()),\n                color: Some(front_api::types::Color::Red),\n                timezone: Some(\"some-string\".to_string()),\n                times: Some(front_api::types::ShiftIntervals {\n                    mon: Some(front_api::types::ShiftInterval {\n                        start: \"some-string\".to_string(),\n                        end: \"some-string\".to_string(),\n                    }),\n                    tue: Some(front_api::types::ShiftInterval {\n                        start: \"some-string\".to_string(),\n                        end: \"some-string\".to_string(),\n                    }),\n                    wed: Some(front_api::types::ShiftInterval {\n                        start: \"some-string\".to_string(),\n                        end: \"some-string\".to_string(),\n                    }),\n                    thu: Some(front_api::types::ShiftInterval {\n                        start: \"some-string\".to_string(),\n                        end: \"some-string\".to_string(),\n                    }),\n                    fri: Some(front_api::types::ShiftInterval {\n                        start: \"some-string\".to_string(),\n                        end: \"some-string\".to_string(),\n                    }),\n                    sat: Some(front_api::types::ShiftInterval {\n                        start: \"some-string\".to_string(),\n                        end: \"some-string\".to_string(),\n                    }),\n                    sun: Some(front_api::types::ShiftInterval {\n                        start: \"some-string\".to_string(),\n                        end: \"some-string\".to_string(),\n                    }),\n                }),\n                teammate_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        shift_id: &'a str,
        body: &crate::types::UpdateShift,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "shifts/{shift_id}".replace("{shift_id}", shift_id)
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
