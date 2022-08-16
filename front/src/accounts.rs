use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Accounts {
    pub client: Client,
}

impl Accounts {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List Accounts\n\nList the accounts of the company.\n\n**Parameters:**\n\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n- `sort_by: Option<String>`: Field used to sort the records\n- `sort_order: Option<crate::types::SortOrder>`: Order by which results should be sorted\n\n```rust,no_run\nasync fn example_accounts_list() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListAccountsResponse = client\n        .accounts()\n        .list(\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(front_api::types::SortOrder::Asc),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<i64>,
        page_token: Option<String>,
        sort_by: Option<String>,
        sort_order: Option<crate::types::SortOrder>,
    ) -> Result<crate::types::ListAccountsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "accounts"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        if let Some(p) = sort_by {
            query_params.push(("sort_by", p));
        }

        if let Some(p) = sort_order {
            query_params.push(("sort_order", format!("{}", p)));
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

    #[doc = "Create account\n\nCreate a new account.\n\n```rust,no_run\nasync fn example_accounts_create() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::AccountResponse = client\n        .accounts()\n        .create(&front_api::types::Account {\n            name: Some(\"some-string\".to_string()),\n            description: Some(\"some-string\".to_string()),\n            domains: Some(vec![\"some-string\".to_string()]),\n            external_id: Some(\"some-string\".to_string()),\n            custom_fields: Some(std::collections::HashMap::from([(\n                \"some-key\".to_string(),\n                \"some-string\".to_string(),\n            )])),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::Account,
    ) -> Result<crate::types::AccountResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "accounts"),
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

    #[doc = "Fetch an account\n\nFetches an account\n\n**Parameters:**\n\n- `account_id: &'astr`: The Account ID (required)\n\n```rust,no_run\nasync fn example_accounts_fetch() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::AccountResponse = client.accounts().fetch(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch<'a>(
        &'a self,
        account_id: &'a str,
    ) -> Result<crate::types::AccountResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "accounts/{account_id}".replace("{account_id}", account_id)
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

    #[doc = "Delete an account\n\nDeletes an account\n\n**Parameters:**\n\n- `account_id: &'astr`: \
             The Account ID (required)\n\n```rust,no_run\nasync fn example_accounts_delete() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client.accounts().delete(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(
        &'a self,
        account_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "accounts/{account_id}".replace("{account_id}", account_id)
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

    #[doc = "Update account\n\nUpdates an account.\n\n**Parameters:**\n\n- `account_id: &'astr`: The Account ID (required)\n\n```rust,no_run\nasync fn example_accounts_update() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::AccountResponse = client\n        .accounts()\n        .update(\n            \"some-string\",\n            &front_api::types::Account {\n                name: Some(\"some-string\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                domains: Some(vec![\"some-string\".to_string()]),\n                external_id: Some(\"some-string\".to_string()),\n                custom_fields: Some(std::collections::HashMap::from([(\n                    \"some-key\".to_string(),\n                    \"some-string\".to_string(),\n                )])),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        account_id: &'a str,
        body: &crate::types::Account,
    ) -> Result<crate::types::AccountResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "accounts/{account_id}".replace("{account_id}", account_id)
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

    #[doc = "List account contacts\n\nLists the contacts associated to an Account\n\n**Parameters:**\n\n- `account_id: &'astr`: The Account ID (required)\n- `limit: Option<i64>`: Max number of results per page\n- `page_token: Option<String>`: Token to use to request the next page\n\n```rust,no_run\nasync fn example_accounts_list_contacts() -> anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    let result: front_api::types::ListAccountContactsResponse = client\n        .accounts()\n        .list_contacts(\n            \"some-string\",\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_contacts<'a>(
        &'a self,
        account_id: &'a str,
        limit: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListAccountContactsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "accounts/{account_id}/contacts".replace("{account_id}", account_id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
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

    #[doc = "Add contact to Account\n\nAdds a list of contacts to an \
             Account\n\n**Parameters:**\n\n- `account_id: &'astr`: The Account ID \
             (required)\n\n```rust,no_run\nasync fn example_accounts_add_contacts_to() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .accounts()\n        .add_contacts_to(\n            \
             \"some-string\",\n            &front_api::types::ContactIds {\n                \
             contact_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn add_contacts_to<'a>(
        &'a self,
        account_id: &'a str,
        body: &crate::types::ContactIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "accounts/{account_id}/contacts".replace("{account_id}", account_id)
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

    #[doc = "Remove contact from Account\n\nRemoves a list of contacts from an \
             Account\n\n**Parameters:**\n\n- `account_id: &'astr`: The Account ID \
             (required)\n\n```rust,no_run\nasync fn example_accounts_remove_contacts_from() -> \
             anyhow::Result<()> {\n    let client = front_api::Client::new_from_env();\n    \
             client\n        .accounts()\n        .remove_contacts_from(\n            \
             \"some-string\",\n            &front_api::types::ContactIds {\n                \
             contact_ids: vec![\"some-string\".to_string()],\n            },\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn remove_contacts_from<'a>(
        &'a self,
        account_id: &'a str,
        body: &crate::types::ContactIds,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "accounts/{account_id}/contacts".replace("{account_id}", account_id)
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
