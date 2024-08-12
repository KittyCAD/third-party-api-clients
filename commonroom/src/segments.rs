use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Segments {
    pub client: Client,
}

impl Segments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get segments in a community\n\nGets all segments in a community\n\n\n```rust,no_run\nasync fn example_segments_get() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    let result: Vec<commonroom_api::types::GetSegmentsResponse> = client.segments().get().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::GetSegmentsResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "segments"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Gets all status for a segment\n\nGets all status for a \
             segment\n\n\n```rust,no_run\nasync fn example_segments_get_statuses() -> \
             anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    \
             let result: Vec<commonroom_api::types::GetSegmentStatusesResponse> =\n        \
             client.segments().get_statuses().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_statuses<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::GetSegmentStatusesResponse>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "segments/:id/status"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Add Community Member(s) to Segment\n\nAdd existing community members to an existing segment\n\n\n```rust,no_run\nasync fn example_segments_add_members_to() -> anyhow::Result<()> {\n    let client = commonroom_api::Client::new_from_env();\n    client\n        .segments()\n        .add_members_to(&commonroom_api::types::AddMembersToSegmentRequestBody {\n            social_type: \"some-string\".to_string(),\n            value: \"some-string\".to_string(),\n            status_id: Some(3.14 as f64),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_members_to<'a>(
        &'a self,
        body: &crate::types::AddMembersToSegmentRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "segments/:id"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Add a Note to a Segment\n\nAdd a Note to a Segment\n\n\n```rust,no_run\nasync fn \
             example_segments_add_note_to() -> anyhow::Result<()> {\n    let client = \
             commonroom_api::Client::new_from_env();\n    client\n        .segments()\n        \
             .add_note_to(&commonroom_api::types::AddNoteToSegmentRequestBody {\n            \
             segment_id: Some(3.14 as f64),\n            note: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_note_to<'a>(
        &'a self,
        body: &crate::types::AddNoteToSegmentRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "segments/note"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
