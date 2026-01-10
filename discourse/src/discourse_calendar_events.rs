use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug, Default)]
pub struct ListEventsParams {
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    pub attending_user: Option<String>,
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    pub category_id: Option<i64>,
    pub include_details: Option<crate::types::IncludeDetails>,
    pub include_subcategories: Option<crate::types::IncludeSubcategories>,
    pub limit: Option<i64>,
    pub order: Option<crate::types::Order>,
    pub post_id: Option<i64>,
}

#[derive(Clone, Debug, Default)]
pub struct ExportEventsIcsParams {
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    pub attending_user: Option<String>,
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    pub category_id: Option<i64>,
    pub include_subcategories: Option<crate::types::IncludeSubcategories>,
    pub limit: Option<i64>,
    pub order: Option<crate::types::Order>,
}

#[derive(Clone, Debug)]
pub struct DiscourseCalendarEvents {
    pub client: Client,
}

impl DiscourseCalendarEvents {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List calendar events\n\n**Parameters:**\n\n- `after: Option<chrono::DateTime<chrono::Utc>>`: Return events starting after this date/time (ISO 8601 format)\n- `attending_user: Option<String>`: Filter to events where the specified user (username) has RSVP'd\nas going\n- `before: Option<chrono::DateTime<chrono::Utc>>`: Return events starting before this date/time (ISO 8601 format)\n- `category_id: Option<i64>`: Filter events by category ID\n- `include_details: Option<crate::types::IncludeDetails>`: Include detailed event information (creator, invitees, stats,\netc.)\n- `include_subcategories: Option<crate::types::IncludeSubcategories>`: Include events from subcategories when filtering by category\n- `limit: Option<i64>`: Maximum number of events to return (default: 200)\n- `order: Option<crate::types::Order>`: Sort order for events by start date (default: asc)\n- `post_id: Option<i64>`: Filter to events associated with a specific post ID\n\n```rust,no_run\nasync fn example_discourse_calendar_events_list_events() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::ListEventsResponse = client\n        .discourse_calendar_events()\n        .list_events(discourse_api::discourse_calendar_events::ListEventsParams {\n            after: Some(chrono::Utc::now()),\n            attending_user: Some(\"some-string\".to_string()),\n            before: Some(chrono::Utc::now()),\n            category_id: Some(4 as i64),\n            include_details: Some(discourse_api::types::IncludeDetails::False),\n            include_subcategories: Some(discourse_api::types::IncludeSubcategories::False),\n            limit: Some(4 as i64),\n            order: Some(discourse_api::types::Order::Desc),\n            post_id: Some(4 as i64),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_events<'a>(
        &'a self,
        params: ListEventsParams,
    ) -> Result<crate::types::ListEventsResponse, crate::types::error::Error> {
        let ListEventsParams {
            after,
            attending_user,
            before,
            category_id,
            include_details,
            include_subcategories,
            limit,
            order,
            post_id,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url, "discourse-post-event/events.json"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = after {
            query_params.push(("after", format!("{}", p)));
        }

        if let Some(p) = attending_user {
            query_params.push(("attending_user", p));
        }

        if let Some(p) = before {
            query_params.push(("before", format!("{}", p)));
        }

        if let Some(p) = category_id {
            query_params.push(("category_id", format!("{}", p)));
        }

        if let Some(p) = include_details {
            query_params.push(("include_details", format!("{}", p)));
        }

        if let Some(p) = include_subcategories {
            query_params.push(("include_subcategories", format!("{}", p)));
        }

        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = order {
            query_params.push(("order", format!("{}", p)));
        }

        if let Some(p) = post_id {
            query_params.push(("post_id", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Export calendar events in iCalendar format\n\n**Parameters:**\n\n- `after: Option<chrono::DateTime<chrono::Utc>>`: Return events starting after this date/time (ISO 8601 format)\n- `attending_user: Option<String>`: Filter to events where the specified user (username) has RSVP'd\nas going\n- `before: Option<chrono::DateTime<chrono::Utc>>`: Return events starting before this date/time (ISO 8601 format)\n- `category_id: Option<i64>`: Filter events by category ID\n- `include_subcategories: Option<crate::types::IncludeSubcategories>`: Include events from subcategories when filtering by category\n- `limit: Option<i64>`: Maximum number of events to return (default: 200)\n- `order: Option<crate::types::Order>`: Sort order for events by start date (default: asc)\n\n```rust,no_run\nasync fn example_discourse_calendar_events_export_events_ics() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: String = client\n        .discourse_calendar_events()\n        .export_events_ics(discourse_api::discourse_calendar_events::ExportEventsIcsParams {\n            after: Some(chrono::Utc::now()),\n            attending_user: Some(\"some-string\".to_string()),\n            before: Some(chrono::Utc::now()),\n            category_id: Some(4 as i64),\n            include_subcategories: Some(discourse_api::types::IncludeSubcategories::False),\n            limit: Some(4 as i64),\n            order: Some(discourse_api::types::Order::Desc),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn export_events_ics<'a>(
        &'a self,
        params: ExportEventsIcsParams,
    ) -> Result<String, crate::types::error::Error> {
        let ExportEventsIcsParams {
            after,
            attending_user,
            before,
            category_id,
            include_subcategories,
            limit,
            order,
        } = params;
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url, "discourse-post-event/events.ics"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = after {
            query_params.push(("after", format!("{}", p)));
        }

        if let Some(p) = attending_user {
            query_params.push(("attending_user", p));
        }

        if let Some(p) = before {
            query_params.push(("before", format!("{}", p)));
        }

        if let Some(p) = category_id {
            query_params.push(("category_id", format!("{}", p)));
        }

        if let Some(p) = include_subcategories {
            query_params.push(("include_subcategories", format!("{}", p)));
        }

        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = order {
            query_params.push(("order", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await?;
            Ok(text)
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
