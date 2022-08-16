use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct MessageTemplates {
    pub client: Client,
}

impl MessageTemplates {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get child templates\n\nFetch the child message templates of a message template folder.\n\n**Parameters:**\n\n- `message_template_folder_id: &'astr`: The message template folder ID (required)\n\n```rust,no_run\nasync fn example_message_templates_get_child_templates() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::GetChildTemplatesResponse = client\n        .message_templates()\n        .get_child_templates(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_child_templates<'a>(
        &'a self,
        message_template_folder_id: &'a str,
    ) -> Result<crate::types::GetChildTemplatesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_template_folders/{message_template_folder_id}/message_templates"
                    .replace("{message_template_folder_id}", message_template_folder_id)
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

    #[doc = "Create child template\n\nCreate a new message template as a child of the given folder\n\n**Parameters:**\n\n- `message_template_folder_id: &'astr`: The parent message template folder ID (required)\n\n```rust,no_run\nasync fn example_message_templates_create_child_template() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateResponse = client\n        .message_templates()\n        .create_child_template(\n            \"some-string\",\n            &front_api::types::CreateMessageTemplateAsChild {\n                name: \"some-string\".to_string(),\n                subject: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                inbox_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_child_template<'a>(
        &'a self,
        message_template_folder_id: &'a str,
        body: &crate::types::CreateMessageTemplateAsChild,
    ) -> Result<crate::types::MessageTemplateResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_template_folders/{message_template_folder_id}/message_templates"
                    .replace("{message_template_folder_id}", message_template_folder_id)
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

    #[doc = "List message templates\n\nList the message templates.\n\n```rust,no_run\nasync fn \
             example_message_templates_list() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListMessageTemplatesResponse =\n        \
             client.message_templates().list().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
    ) -> Result<crate::types::ListMessageTemplatesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "message_templates"),
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

    #[doc = "Create message template\n\nCreate a new message template.\n\n```rust,no_run\nasync fn example_message_templates_create() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateResponse = client\n        .message_templates()\n        .create(&front_api::types::CreateSharedMessageTemplate {\n            name: \"some-string\".to_string(),\n            subject: Some(\"some-string\".to_string()),\n            body: \"some-string\".to_string(),\n            folder_id: Some(\"some-string\".to_string()),\n            inbox_ids: Some(vec![\"some-string\".to_string()]),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::CreateSharedMessageTemplate,
    ) -> Result<crate::types::MessageTemplateResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "message_templates"),
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

    #[doc = "List team message templates\n\nList the message templates belonging to the requested \
             team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID \
             (required)\n\n```rust,no_run\nasync fn example_message_templates_list_team() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListTeamMessageTemplatesResponse =\n        \
             client.message_templates().list_team(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamMessageTemplatesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/message_templates".replace("{team_id}", team_id)
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

    #[doc = "Create team message template\n\nCreate a new message template for the given team\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_message_templates_create_team() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateResponse = client\n        .message_templates()\n        .create_team(\n            \"some-string\",\n            &front_api::types::CreateSharedMessageTemplate {\n                name: \"some-string\".to_string(),\n                subject: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                folder_id: Some(\"some-string\".to_string()),\n                inbox_ids: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateSharedMessageTemplate,
    ) -> Result<crate::types::MessageTemplateResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/message_templates".replace("{team_id}", team_id)
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

    #[doc = "List teammate message templates\n\nList the message templates belonging to the \
             requested teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID \
             (required)\n\n```rust,no_run\nasync fn example_message_templates_list_teammate() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::ListTeammateMessageTemplatesResponse = client\n        \
             .message_templates()\n        .list_teammate(\"some-string\")\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateMessageTemplatesResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/message_templates".replace("{teammate_id}", teammate_id)
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

    #[doc = "Create teammate message template\n\nCreate a new message template for the given teammate\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_message_templates_create_teammate() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateResponse = client\n        .message_templates()\n        .create_teammate(\n            \"some-string\",\n            &front_api::types::CreatePrivateMessageTemplate {\n                name: \"some-string\".to_string(),\n                subject: Some(\"some-string\".to_string()),\n                body: \"some-string\".to_string(),\n                folder_id: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_teammate<'a>(
        &'a self,
        teammate_id: &'a str,
        body: &crate::types::CreatePrivateMessageTemplate,
    ) -> Result<crate::types::MessageTemplateResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/message_templates".replace("{teammate_id}", teammate_id)
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

    #[doc = "Get message template\n\nFetch a message template.\n\n**Parameters:**\n\n- \
             `message_template_id: &'astr`: The message template ID \
             (required)\n\n```rust,no_run\nasync fn example_message_templates_get() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::MessageTemplateResponse =\n        \
             client.message_templates().get(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        message_template_id: &'a str,
    ) -> Result<crate::types::MessageTemplateResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_templates/{message_template_id}"
                    .replace("{message_template_id}", message_template_id)
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

    #[doc = "Delete message template\n\nDelete a message template\n\n**Parameters:**\n\n- \
             `message_template_id: &'astr`: The message template ID \
             (required)\n\n```rust,no_run\nasync fn example_message_templates_delete() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client.message_templates().delete(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        message_template_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_templates/{message_template_id}"
                    .replace("{message_template_id}", message_template_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Update message template\n\nUpdate message template\n\n**Parameters:**\n\n- \
             `message_template_id: &'astr`: The message template ID \
             (required)\n\n```rust,no_run\nasync fn example_message_templates_update() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::MessageTemplateResponse = client\n        \
             .message_templates()\n        .update(\n            \"some-string\",\n            \
             &serde_json::Value::String(\"some-string\".to_string()),\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        message_template_id: &'a str,
        body: &crate::types::UpdateMessageTemplate,
    ) -> Result<crate::types::MessageTemplateResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_templates/{message_template_id}"
                    .replace("{message_template_id}", message_template_id)
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
