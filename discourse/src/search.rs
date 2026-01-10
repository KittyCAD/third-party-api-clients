use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Search {
    pub client: Client,
}

impl Search {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Search for a term\n\n**Parameters:**\n\n- `page: Option<i64>`\n- `q: Option<String>`: The query string needs to be url encoded and is made up of the following options:\n- Search term. This is just a string. Usually it would be the first item in the query.\n- `@<username>`: Use the `@` followed by the username to specify posts by this user.\n- `#<category>`: Use the `#` followed by the category slug to search within this category.\n- `tags:`: `api,solved` or for posts that have all the specified tags `api+solved`.\n- `before:`: `yyyy-mm-dd`\n- `after:`: `yyyy-mm-dd`\n- `order:`: `latest`, `likes`, `views`, `latest_topic`\n- `assigned:`: username (without `@`)\n- `in:`: `title`, `likes`, `personal`, `messages`, `seen`, `unseen`, `posted`, `created`, `watching`, `tracking`, `bookmarks`, `assigned`, `unassigned`, `first`, `pinned`, `wiki`\n- `with:`: `images`\n- `status:`: `open`, `closed`, `public`, `archived`, `noreplies`, `single_user`, `solved`, `unsolved`\n- `group:`: group_name or group_id\n- `group_messages:`: group_name or group_id\n- `min_posts:`: 1\n- `max_posts:`: 10\n- `min_views:`: 1\n- `max_views:`: 10\n\nIf you are using cURL you can use the `-G` and the `--data-urlencode` flags to encode the query:\n\n```text\ncurl -i -sS -X GET -G \"http://localhost:4200/search.json\" \\\n--data-urlencode 'q=wordpress @scossar #fun after:2020-01-01'\n```\n\n\n```rust,no_run\nasync fn example_search_search() -> anyhow::Result<()> {\n    let client = discourse_api::Client::new_from_env();\n    let result: discourse_api::types::SearchResponse = client\n        .search()\n        .search(Some(4 as i64), Some(\"some-string\".to_string()))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn search<'a>(
        &'a self,
        page: Option<i64>,
        q: Option<String>,
    ) -> Result<crate::types::SearchResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "search.json"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}
