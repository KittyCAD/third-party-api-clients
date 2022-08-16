use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Attachments {
    pub client: Client,
}

impl Attachments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Download attachment\n\nDownload an attachment file.\n\n**Parameters:**\n\n- \
             `attachment_link_id: &'astr`: The Attachment ID (required)\n\n```rust,no_run\nasync \
             fn example_attachments_download() -> anyhow::Result<()> {\n    let client = \
             front_api::Client::new_from_env();\n    let result: front_api::types::Attachment = \
             client.attachments().download(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn download<'a>(
        &'a self,
        attachment_link_id: &'a str,
    ) -> Result<crate::types::Attachment, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "download/{attachment_link_id}"
                    .replace("{attachment_link_id}", attachment_link_id)
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
