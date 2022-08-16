use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct MessageTemplateFolders {
    pub client: Client,
}

impl MessageTemplateFolders {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List folders\n\nList the message template folders.\n\n```rust,no_run\nasync fn \
             example_message_template_folders_list_folders() -> anyhow::Result<()> {\n    let \
             client = front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListFoldersResponse =\n        \
             client.message_template_folders().list_folders().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_folders<'a>(
        &'a self,
    ) -> Result<crate::types::ListFoldersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "message_template_folders"),
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

    #[doc = "Create folder\n\nCreate a new message template folder.\n\n```rust,no_run\nasync fn \
             example_message_template_folders_create_folder() -> anyhow::Result<()> {\n    let \
             client = front_api::Client::new_from_env();\n    let result: \
             front_api::types::MessageTemplateFolderResponse = client\n        \
             .message_template_folders()\n        \
             .create_folder(&front_api::types::CreateMessageTemplateFolder {\n            name: \
             \"some-string\".to_string(),\n            parent_folder_id: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_folder<'a>(
        &'a self,
        body: &crate::types::CreateMessageTemplateFolder,
    ) -> Result<crate::types::MessageTemplateFolderResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "message_template_folders"),
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

    #[doc = "List team folders\n\nList the message template folders belonging to the requested \
             team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID \
             (required)\n\n```rust,no_run\nasync fn \
             example_message_template_folders_list_team_folders() -> anyhow::Result<()> {\n    let \
             client = front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTeamFoldersResponse = client\n        \
             .message_template_folders()\n        .list_team_folders(\"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_team_folders<'a>(
        &'a self,
        team_id: &'a str,
    ) -> Result<crate::types::ListTeamFoldersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/message_template_folders".replace("{team_id}", team_id)
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

    #[doc = "Create team folder\n\nCreate a new message template folder belonging to the requested team.\n\n**Parameters:**\n\n- `team_id: &'astr`: The team ID (required)\n\n```rust,no_run\nasync fn example_message_template_folders_create_team_folder() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateFolderResponse = client\n        .message_template_folders()\n        .create_team_folder(\n            \"some-string\",\n            &front_api::types::CreateMessageTemplateFolder {\n                name: \"some-string\".to_string(),\n                parent_folder_id: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_team_folder<'a>(
        &'a self,
        team_id: &'a str,
        body: &crate::types::CreateMessageTemplateFolder,
    ) -> Result<crate::types::MessageTemplateFolderResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teams/{team_id}/message_template_folders".replace("{team_id}", team_id)
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

    #[doc = "List teammate folders\n\nList the message template folders belonging to the requested \
             teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID \
             (required)\n\n```rust,no_run\nasync fn \
             example_message_template_folders_list_teammate_folders() -> anyhow::Result<()> {\n    \
             let client = front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListTeammateFoldersResponse = client\n        \
             .message_template_folders()\n        .list_teammate_folders(\"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_teammate_folders<'a>(
        &'a self,
        teammate_id: &'a str,
    ) -> Result<crate::types::ListTeammateFoldersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/message_template_folders"
                    .replace("{teammate_id}", teammate_id)
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

    #[doc = "Create teammate folder\n\nCreate a new message template folder belonging to the requested teammate.\n\n**Parameters:**\n\n- `teammate_id: &'astr`: The teammate ID (required)\n\n```rust,no_run\nasync fn example_message_template_folders_create_teammate_folder() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateFolderResponse = client\n        .message_template_folders()\n        .create_teammate_folder(\n            \"some-string\",\n            &front_api::types::CreateMessageTemplateFolder {\n                name: \"some-string\".to_string(),\n                parent_folder_id: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_teammate_folder<'a>(
        &'a self,
        teammate_id: &'a str,
        body: &crate::types::CreateMessageTemplateFolder,
    ) -> Result<crate::types::MessageTemplateFolderResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "teammates/{teammate_id}/message_template_folders"
                    .replace("{teammate_id}", teammate_id)
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

    #[doc = "Get child folders\n\nFetch the child message templates folders of a message template \
             folder.\n\n**Parameters:**\n\n- `message_template_folder_id: &'astr`: The message \
             template folder ID (required)\n\n```rust,no_run\nasync fn \
             example_message_template_folders_get_child_folders() -> anyhow::Result<()> {\n    let \
             client = front_api::Client::new_from_env();\n    let result: \
             front_api::types::GetChildFoldersResponse = client\n        \
             .message_template_folders()\n        .get_child_folders(\"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_child_folders<'a>(
        &'a self,
        message_template_folder_id: &'a str,
    ) -> Result<crate::types::GetChildFoldersResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_template_folders/{message_template_folder_id}/message_template_folders"
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

    #[doc = "Create child folder\n\nCreate a new message template folder as a child of the given folder\n\n**Parameters:**\n\n- `message_template_folder_id: &'astr`: The parent message template folder ID (required)\n\n```rust,no_run\nasync fn example_message_template_folders_create_child_folder() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateFolderResponse = client\n        .message_template_folders()\n        .create_child_folder(\n            \"some-string\",\n            &front_api::types::CreateMessageTemplateFolderAsChild {\n                name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_child_folder<'a>(
        &'a self,
        message_template_folder_id: &'a str,
        body: &crate::types::CreateMessageTemplateFolderAsChild,
    ) -> Result<crate::types::MessageTemplateFolderResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_template_folders/{message_template_folder_id}/message_template_folders"
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

    #[doc = "Get folder\n\nFetch a message template folder.\n\n**Parameters:**\n\n- `message_template_folder_id: &'astr`: The message template folder ID (required)\n\n```rust,no_run\nasync fn example_message_template_folders_get_folder() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateFolderResponse = client\n        .message_template_folders()\n        .get_folder(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_folder<'a>(
        &'a self,
        message_template_folder_id: &'a str,
    ) -> Result<crate::types::MessageTemplateFolderResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_template_folders/{message_template_folder_id}"
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

    #[doc = "Delete folder\n\nDelete a message template folder and child \
             folders/templates\n\n**Parameters:**\n\n- `message_template_folder_id: &'astr`: The \
             message template folder id (required)\n\n```rust,no_run\nasync fn \
             example_message_template_folders_delete_folder() -> anyhow::Result<()> {\n    let \
             client = front_api::Client::new_from_env();\n    let result: \
             front_api::types::DeleteFolderResponse = client\n        \
             .message_template_folders()\n        .delete_folder(\"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_folder<'a>(
        &'a self,
        message_template_folder_id: &'a str,
    ) -> Result<crate::types::DeleteFolderResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_template_folders/{message_template_folder_id}"
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

    #[doc = "Update folder\n\nUpdate message template folder\n\n**Parameters:**\n\n- `message_template_folder_id: &'astr`: The message template folder ID (required)\n\n```rust,no_run\nasync fn example_message_template_folders_update_folder() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::MessageTemplateFolderResponse = client\n        .message_template_folders()\n        .update_folder(\n            \"some-string\",\n            &front_api::types::UpdateMessageTemplateFolder {\n                name: Some(\"some-string\".to_string()),\n                parent_folder_id: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_folder<'a>(
        &'a self,
        message_template_folder_id: &'a str,
        body: &crate::types::UpdateMessageTemplateFolder,
    ) -> Result<crate::types::MessageTemplateFolderResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "message_template_folders/{message_template_folder_id}"
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
}
