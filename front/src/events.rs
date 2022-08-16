use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Events {
    pub client: Client,
}

impl Events {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List events\n\nLists all the detailed events which occured in the inboxes of the company ordered in reverse chronological order (newest first).\n> ⚠\u{fe0f} Deprecated field may be included\n>\n> This route return the deprecated `last_message` conversation field for conversations\n> associated with individual events. Please use the conversation's\n> `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `q: Option<String>`: Search query object with optional properties `before`, `after`, or `types`. `before` and `after` should be a timestamp in seconds with up to 3 decimal places. `types` should be a list of event types.\n\n```rust,no_run\nasync fn example_events_list() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListEventsResponse = client\n        .events()\n        .list(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        q: Option<String>,
    ) -> Result<crate::types::ListEventsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "events"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        if let Some(p) = q {
            query_params.push(("q", p));
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

    #[doc = "Get event\n\nFetch an event.\n> ⚠\u{fe0f} Deprecated field may be included\n>\n> This \
             route return the deprecated `last_message` conversation field for conversations\n> \
             associated with individual events. Please use the conversation's\n> \
             `_links.related.last_message` field instead.\n\n\n**Parameters:**\n\n- `event_id: \
             &'astr`: The event ID (required)\n\n```rust,no_run\nasync fn example_events_get() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let \
             result: front_api::types::EventResponse = \
             client.events().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        event_id: &'a str,
    ) -> Result<crate::types::EventResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "events/{event_id}".replace("{event_id}", event_id)
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
