use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Accounting {
    pub client: Client,
}

impl Accounting {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List general ledger accounts\n\n**Parameters:**\n\n- `is_active: Option<bool>`\n- `is_synced: Option<bool>`\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_get_gl_account_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiAccountingGLAccountResourceSchema = client\n        .accounting()\n        .get_gl_account_list_resource(\n            Some(false),\n            Some(false),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_gl_account_list_resource<'a>(
        &'a self,
        is_active: Option<bool>,
        is_synced: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiAccountingGLAccountResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/accounts"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = is_active {
            query_params.push(("is_active", format!("{}", p)));
        }

        if let Some(p) = is_synced {
            query_params.push(("is_synced", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Upload general ledger accounts\n\nYou can upload up to 500 general ledger accounts in an all-or-nothing fashion. If a general ledger accounts within a batch is malformed or violates a database constraint, the entire batch containing that account will be disregarded.\nTo have a successful upload, please sanitize the data and ensure the general ledger accounts that you are trying to upload do not already exist on Ramp.\nIf a general ledger account is already on Ramp but you want to update its attributes, please use the PATCH developer/v1/accounting/accounts/{id} endpoint instead.\n\n```rust,no_run\nasync fn example_accounting_post_gl_account_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiAccountingTrackingCategoryUploadResponse = client\n        .accounting()\n        .post_gl_account_list_resource(&ramp_api::types::ApiAccountingGLAccountUploadRequestBody {\n            gl_accounts: vec![ramp_api::types::Glaccount {\n                classification: ramp_api::types::Classification::Unknown,\n                code: Some(\"some-string\".to_string()),\n                id: \"some-string\".to_string(),\n                name: \"some-string\".to_string(),\n            }],\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_gl_account_list_resource<'a>(
        &'a self,
        body: &crate::types::ApiAccountingGLAccountUploadRequestBody,
    ) -> Result<crate::types::ApiAccountingTrackingCategoryUploadResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/accounts"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Fetch a general ledger account\n\n**Parameters:**\n\n- `gl_account_id: uuid::Uuid` \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_accounting_get_gl_account_resource() -> anyhow::Result<()> {\n    let client \
             =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::GeneralLedgerAccount = client\n        .accounting()\n        \
             .get_gl_account_resource(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_gl_account_resource<'a>(
        &'a self,
        gl_account_id: uuid::Uuid,
    ) -> Result<crate::types::GeneralLedgerAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/accounts/{gl_account_id}"
                    .replace("{gl_account_id}", &format!("{}", gl_account_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Delete a general ledger account\n\n**Parameters:**\n\n- `gl_account_id: uuid::Uuid` \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_accounting_delete_gl_account_resource() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client\n        .accounting()\n        \
             .delete_gl_account_resource(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_gl_account_resource<'a>(
        &'a self,
        gl_account_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/accounts/{gl_account_id}"
                    .replace("{gl_account_id}", &format!("{}", gl_account_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Update a general ledger account\n\nThis endpoint can be used to update the name or code of a GL account;\nIt can also be used to associate a general ledger account with a list of subsidiaries.\n\n**Parameters:**\n\n- `gl_account_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_patch_gl_account_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::GeneralLedgerAccount = client\n        .accounting()\n        .patch_gl_account_resource(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiAccountingGLAccountUpdateRequestBody {\n                code: Some(\"some-string\".to_string()),\n                name: Some(\"some-string\".to_string()),\n                reactivate: Some(false),\n                subsidiaries: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_gl_account_resource<'a>(
        &'a self,
        gl_account_id: uuid::Uuid,
        body: &crate::types::ApiAccountingGLAccountUpdateRequestBody,
    ) -> Result<crate::types::GeneralLedgerAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/accounts/{gl_account_id}"
                    .replace("{gl_account_id}", &format!("{}", gl_account_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Fetch an accounting connection\n\n```rust,no_run\nasync fn \
             example_accounting_get_connection_resouce() -> anyhow::Result<()> {\n    let client \
             =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::AccountingProvider =\n        \
             client.accounting().get_connection_resouce().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_connection_resouce<'a>(
        &'a self,
    ) -> Result<crate::types::AccountingProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/connection"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Register a new accounting connection\n\nA connection is required in order to use our \
             accounting API functionality.\n\nThis endpoint provides the option to reactivate a \
             deleted accounting connection\ninstead of creating a brand-new connection if the user \
             passes the \"reactivate\" parameter.\nThis is useful if the user registered \
             accounting fields in a previous connection\nand doesn't want to re-register those \
             fields again.\n\n```rust,no_run\nasync fn \
             example_accounting_post_connection_resouce() -> anyhow::Result<()> {\n    let client \
             =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::AccountingProvider = client\n        .accounting()\n        \
             .post_connection_resouce(\n            \
             &ramp_api::types::ApiAccountingProviderAccessCreateRequestBody {\n                \
             reactivate: false,\n                remote_provider_name: \
             \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_connection_resouce<'a>(
        &'a self,
        body: &crate::types::ApiAccountingProviderAccessCreateRequestBody,
    ) -> Result<crate::types::AccountingProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/connection"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Delete an accounting connection\n\n```rust,no_run\nasync fn \
             example_accounting_delete_connection_resouce() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    \
             client.accounting().delete_connection_resouce().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_connection_resouce<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/connection"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "List options for a given custom accounting field\n\n**Parameters:**\n\n- `field_id: uuid::Uuid` (required)\n- `is_active: Option<bool>`\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_get_custom_field_option_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiAccountingCustomFieldOptionResourceSchema =\n        client\n            .accounting()\n            .get_custom_field_option_list_resource(\n                uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                Some(false),\n                Some(4 as i64),\n                Some(uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?),\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_field_option_list_resource<'a>(
        &'a self,
        field_id: uuid::Uuid,
        is_active: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiAccountingCustomFieldOptionResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/field-options"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![("field_id", format!("{}", field_id))];
        if let Some(p) = is_active {
            query_params.push(("is_active", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Upload new options\n\nYou can upload up to 500 new field options for a given custom accounting field in an all-or-nothing fashion. If a field option within a batch is malformed or violates a database constraint, the entire batch containing that field option will be disregarded.\nTo have a successful upload, please sanitize the data and ensure the field options that you are trying to upload do not already exist on Ramp.\nIf a field option is already on Ramp but you want to update its attributes, please use the PATCH developer/v1/accounting/field-options/{id} endpoint instead.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_post_custom_field_option_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiAccountingTrackingCategoryUploadResponse = client\n        .accounting()\n        .post_custom_field_option_list_resource(\n            &ramp_api::types::ApiAccountingCustomFieldOptionUploadRequestBody {\n                field_id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                options: vec![ramp_api::types::FieldOption {\n                    id: \"some-string\".to_string(),\n                    value: \"some-string\".to_string(),\n                }],\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_custom_field_option_list_resource<'a>(
        &'a self,
        body: &crate::types::ApiAccountingCustomFieldOptionUploadRequestBody,
    ) -> Result<crate::types::ApiAccountingTrackingCategoryUploadResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/field-options"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Fetch a custom accounting field option\n\n**Parameters:**\n\n- `field_option_id: \
             uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_accounting_get_custom_field_option_resource() -> anyhow::Result<()> {\n    \
             let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: ramp_api::types::CustomFieldOption \
             = client\n        .accounting()\n        \
             .get_custom_field_option_resource(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_field_option_resource<'a>(
        &'a self,
        field_option_id: uuid::Uuid,
    ) -> Result<crate::types::CustomFieldOption, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/field-options/{field_option_id}"
                    .replace("{field_option_id}", &format!("{}", field_option_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Delete a custom accounting field option\n\n**Parameters:**\n\n- `field_option_id: \
             uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_accounting_delete_custom_field_option_resource() -> anyhow::Result<()> {\n    \
             let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client\n        .accounting()\n        \
             .delete_custom_field_option_resource(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_custom_field_option_resource<'a>(
        &'a self,
        field_option_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/field-options/{field_option_id}"
                    .replace("{field_option_id}", &format!("{}", field_option_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Update a custom accounting field option\n\n**Parameters:**\n\n- `field_option_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_patch_custom_field_option_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::CustomFieldOption = client\n        .accounting()\n        .patch_custom_field_option_resource(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiAccountingCustomFieldOptionUpdateRequestBody {\n                reactivate: Some(false),\n                value: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_custom_field_option_resource<'a>(
        &'a self,
        field_option_id: uuid::Uuid,
        body: &crate::types::ApiAccountingCustomFieldOptionUpdateRequestBody,
    ) -> Result<crate::types::CustomFieldOption, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/field-options/{field_option_id}"
                    .replace("{field_option_id}", &format!("{}", field_option_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "List custom accounting fields\n\n**Parameters:**\n\n- `is_active: Option<bool>`\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_get_custom_field_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiAccountingCustomFieldResourceSchema = client\n        .accounting()\n        .get_custom_field_list_resource(\n            Some(false),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_field_list_resource<'a>(
        &'a self,
        is_active: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiAccountingCustomFieldResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/fields"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = is_active {
            query_params.push(("is_active", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create a new custom accounting field\n\nIf an custom field with the same id already exist on Ramp, then that existing one will be returned instead of creating a new one; If the existing custom field is inactive, it will be reactivated.\nIf you want to update the existing custom field, please use the PATCH developer/v1/accounting/fields/{id} endpoint instead.\n\n```rust,no_run\nasync fn example_accounting_post_custom_field_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiAccountingCustomFieldResource = client\n        .accounting()\n        .post_custom_field_list_resource(&ramp_api::types::ApiAccountingCustomFieldCreateRequestBody {\n            id: \"some-string\".to_string(),\n            input_type: ramp_api::types::InputType::SingleChoice,\n            is_splittable: Some(false),\n            name: \"some-string\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_custom_field_list_resource<'a>(
        &'a self,
        body: &crate::types::ApiAccountingCustomFieldCreateRequestBody,
    ) -> Result<crate::types::ApiAccountingCustomFieldResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/fields"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Fetch a custom accounting field\n\n**Parameters:**\n\n- `field_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_get_custom_field_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiAccountingCustomFieldResource = client\n        .accounting()\n        .get_custom_field_resource(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_custom_field_resource<'a>(
        &'a self,
        field_id: uuid::Uuid,
    ) -> Result<crate::types::ApiAccountingCustomFieldResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/fields/{field_id}"
                    .replace("{field_id}", &format!("{}", field_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Delete a custom accounting field\n\n**Parameters:**\n\n- `field_id: uuid::Uuid` \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_accounting_delete_custom_field_resource() -> anyhow::Result<()> {\n    let \
             client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client\n        .accounting()\n        \
             .delete_custom_field_resource(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_custom_field_resource<'a>(
        &'a self,
        field_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/fields/{field_id}"
                    .replace("{field_id}", &format!("{}", field_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Update a custom accounting field\n\n**Parameters:**\n\n- `field_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_patch_custom_field_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiAccountingCustomFieldResource = client\n        .accounting()\n        .patch_custom_field_resource(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiAccountingCustomFieldUpdateRequestBody {\n                is_splittable: Some(false),\n                name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_custom_field_resource<'a>(
        &'a self,
        field_id: uuid::Uuid,
        body: &crate::types::ApiAccountingCustomFieldUpdateRequestBody,
    ) -> Result<crate::types::ApiAccountingCustomFieldResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/fields/{field_id}"
                    .replace("{field_id}", &format!("{}", field_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Post sync status\n\nThis endpoint allows customers to notify Ramp of a list of sync results.\nAn idempotency key is required to ensure that subsequent requests are properly handled.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_post_sync_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .accounting()\n        .post_sync_list_resource(&ramp_api::types::ApiAccountingSyncCreateRequestBody {\n            failed_syncs: Some(vec![ramp_api::types::ApiAccountingFailedSyncRequestBody {\n                error: ramp_api::types::ApiAccountingSyncErrorRequestBody {\n                    message: \"some-string\".to_string(),\n                },\n                id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            }]),\n            idempotency_key: \"some-string\".to_string(),\n            successful_syncs: Some(vec![ramp_api::types::ApiAccountingSuccessfulSyncRequestBody {\n                id: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                reference_id: \"some-string\".to_string(),\n            }]),\n            sync_type: ramp_api::types::SyncType::WalletTransferSync,\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_sync_list_resource<'a>(
        &'a self,
        body: &crate::types::ApiAccountingSyncCreateRequestBody,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/syncs"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "List vendors\n\n**Parameters:**\n\n- `is_active: Option<bool>`\n- `is_synced: Option<bool>`\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_get_vendor_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiAccountingVendorResourceSchema = client\n        .accounting()\n        .get_vendor_list_resource(\n            Some(false),\n            Some(false),\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_vendor_list_resource<'a>(
        &'a self,
        is_active: Option<bool>,
        is_synced: Option<bool>,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiAccountingVendorResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/vendors"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = is_active {
            query_params.push(("is_active", format!("{}", p)));
        }

        if let Some(p) = is_synced {
            query_params.push(("is_synced", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Upload vendors\n\nYou can upload up to 500 vendors in an all-or-nothing fashion. If a vendors within a batch is malformed or violates a database constraint, the entire batch containing that vendors will be disregarded.\nTo have a successful upload, please sanitize the data and ensure the subsidiaries that you are trying to upload do not already exist on Ramp.\nIf a vendors is already on Ramp but you want to update its attributes, please use the PATCH developer/v1/accounting/vendors/{id} endpoint instead.\n\n```rust,no_run\nasync fn example_accounting_post_vendor_list_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiAccountingTrackingCategoryUploadResponse = client\n        .accounting()\n        .post_vendor_list_resource(&ramp_api::types::ApiAccountingVendorUploadRequestBody {\n            vendors: vec![ramp_api::types::Vendor {\n                id: \"some-string\".to_string(),\n                name: \"some-string\".to_string(),\n            }],\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_vendor_list_resource<'a>(
        &'a self,
        body: &crate::types::ApiAccountingVendorUploadRequestBody,
    ) -> Result<crate::types::ApiAccountingTrackingCategoryUploadResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url, "developer/v1/accounting/vendors"
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Fetch a vendor\n\n**Parameters:**\n\n- `vendor_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_get_vendor_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::VendorAccount = client\n        .accounting()\n        .get_vendor_resource(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_vendor_resource<'a>(
        &'a self,
        vendor_id: uuid::Uuid,
    ) -> Result<crate::types::VendorAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/vendors/{vendor_id}"
                    .replace("{vendor_id}", &format!("{}", vendor_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Delete a vendor\n\n**Parameters:**\n\n- `vendor_id: uuid::Uuid` \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_accounting_delete_vendor_resource() -> anyhow::Result<()> {\n    let client \
             =\n        ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    client\n        .accounting()\n        \
             .delete_vendor_resource(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_vendor_resource<'a>(
        &'a self,
        vendor_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/vendors/{vendor_id}"
                    .replace("{vendor_id}", &format!("{}", vendor_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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

    #[doc = "Update a vendor\n\n**Parameters:**\n\n- `vendor_id: uuid::Uuid` (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_accounting_patch_vendor_resource() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::VendorAccount = client\n        .accounting()\n        .patch_vendor_resource(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &ramp_api::types::ApiAccountingVendorUpdateRequestBody {\n                code: Some(\"some-string\".to_string()),\n                name: Some(\"some-string\".to_string()),\n                reactivate: Some(false),\n                subsidiaries: Some(vec![\"some-string\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_vendor_resource<'a>(
        &'a self,
        vendor_id: uuid::Uuid,
        body: &crate::types::ApiAccountingVendorUpdateRequestBody,
    ) -> Result<crate::types::VendorAccount, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/accounting/vendors/{vendor_id}"
                    .replace("{vendor_id}", &format!("{}", vendor_id))
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}
