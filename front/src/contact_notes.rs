use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ContactNotes {
    pub client: Client,
}

impl ContactNotes {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List notes\n\nList the notes added to a contact.\n\n**Parameters:**\n\n- `contact_id: \
             &'astr`: The contact ID (required)\n\n```rust,no_run\nasync fn \
             example_contact_notes_list_notes() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: \
             front_api::types::ListNotesResponse =\n        \
             client.contact_notes().list_notes(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_notes<'a>(
        &'a self,
        contact_id: &'a str,
    ) -> Result<crate::types::ListNotesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}/notes".replace("{contact_id}", contact_id)
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

    #[doc = "Add note\n\nCreate a new note on a contact.\n\n**Parameters:**\n\n- `contact_id: &'astr`: The contact ID (required)\n\n```rust,no_run\nasync fn example_contact_notes_add_note() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ContactNoteResponses = client\n        .contact_notes()\n        .add_note(\n            \"some-string\",\n            &front_api::types::CreateContactNote {\n                author_id: \"some-string\".to_string(),\n                body: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_note<'a>(
        &'a self,
        contact_id: &'a str,
        body: &crate::types::CreateContactNote,
    ) -> Result<crate::types::ContactNoteResponses, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "contacts/{contact_id}/notes".replace("{contact_id}", contact_id)
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
