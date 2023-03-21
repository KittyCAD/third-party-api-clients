use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Default {
    pub client: Client,
}

impl Default {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts.json`.\n\nRetrieves a collection of Accounts belonging to the account used to make the request\n\n**Parameters:**\n\n- `friendly_name: Option<String>`: Only return the Account resources with friendly names that exactly match this name.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `status: Option<crate::types::AccountEnumStatus>`: Only return Account resources with the given status. Can be `closed`, `suspended` or `active`.\n\n```rust,no_run\nasync fn example_default_list_account() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAccountResponse = client\n        .default()\n        .list_account(\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::AccountEnumStatus::Closed),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_account<'a>(
        &'a self,
        friendly_name: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        status: Option<crate::types::AccountEnumStatus>,
    ) -> Result<crate::types::ListAccountResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "2010-04-01/Accounts.json"),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = status {
            query_params.push(("Status", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts.json`.\n\nCreate a new Twilio \
             Subaccount from the account making the request\n\n```rust,no_run\nasync fn \
             example_default_create_account() -> anyhow::Result<()> {\n    let client = \
             twilio_api::Client::new_from_env();\n    let result: \
             twilio_api::types::ApiV2010Account = client\n        .default()\n        \
             .create_account(&twilio_api::types::CreateAccountRequest {\n            \
             friendly_name: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_account<'a>(
        &'a self,
        body: &crate::types::CreateAccountRequest,
    ) -> Result<crate::types::ApiV2010Account, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "2010-04-01/Accounts.json"),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{Sid}.json`.\n\nFetch the account \
             specified by the provided Account Sid\n\n**Parameters:**\n\n- `sid: &'astr`: The \
             Account Sid that uniquely identifies the account to fetch \
             (required)\n\n```rust,no_run\nasync fn example_default_fetch_account() -> \
             anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let \
             result: twilio_api::types::ApiV2010Account =\n        \
             client.default().fetch_account(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_account<'a>(
        &'a self,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010Account, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{Sid}.json".replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{Sid}.json`.\n\nModify the properties of a given Account\n\n**Parameters:**\n\n- `sid: &'astr`: The Account Sid that uniquely identifies the account to update (required)\n\n```rust,no_run\nasync fn example_default_update_account() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010Account = client\n        .default()\n        .update_account(\n            \"some-string\",\n            &twilio_api::types::UpdateAccountRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n                status: Some(twilio_api::types::AccountEnumStatus::Suspended),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_account<'a>(
        &'a self,
        sid: &'a str,
        body: &crate::types::UpdateAccountRequest,
    ) -> Result<crate::types::ApiV2010Account, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{Sid}.json".replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Addresses.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to read. (required)\n- `customer_name: Option<String>`: The `customer_name` of the Address resources to read.\n- `friendly_name: Option<String>`: The string that identifies the Address resources to read.\n- `iso_country: Option<String>`: The ISO country code of the Address resources to read.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAddressResponse = client\n        .default()\n        .list_address(\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_address<'a>(
        &'a self,
        account_sid: &'a str,
        customer_name: Option<String>,
        friendly_name: Option<String>,
        iso_country: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListAddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Addresses.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = customer_name {
            query_params.push(("CustomerName", p));
        }

        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = iso_country {
            query_params.push(("IsoCountry", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Addresses.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Address resource. (required)\n\n```rust,no_run\nasync fn example_default_create_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountAddress = client\n        .default()\n        .create_address(\n            \"some-string\",\n            &twilio_api::types::CreateAddressRequest {\n                customer_name: \"some-string\".to_string(),\n                street: \"some-string\".to_string(),\n                city: \"some-string\".to_string(),\n                region: \"some-string\".to_string(),\n                postal_code: \"some-string\".to_string(),\n                iso_country: \"some-string\".to_string(),\n                friendly_name: Some(\"some-string\".to_string()),\n                emergency_enabled: Some(true),\n                auto_correct_address: Some(true),\n                street_secondary: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_address<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateAddressRequest,
    ) -> Result<crate::types::ApiV2010AccountAddress, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Addresses.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Address resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountAddress = client\n        .default()\n        .fetch_address(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_address<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountAddress, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Address resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountAddress = client\n        .default()\n        .update_address(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateAddressRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n                customer_name: Some(\"some-string\".to_string()),\n                street: Some(\"some-string\".to_string()),\n                city: Some(\"some-string\".to_string()),\n                region: Some(\"some-string\".to_string()),\n                postal_code: Some(\"some-string\".to_string()),\n                emergency_enabled: Some(false),\n                auto_correct_address: Some(false),\n                street_secondary: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_address<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateAddressRequest,
    ) -> Result<crate::types::ApiV2010AccountAddress, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that is responsible for the Address resource to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Address resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_address(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_address<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Addresses/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Applications.json`.\n\nRetrieve a list of applications representing an application within the requesting account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to read. (required)\n- `friendly_name: Option<String>`: The string that identifies the Application resources to read.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_application() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListApplicationResponse = client\n        .default()\n        .list_application(\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_application<'a>(
        &'a self,
        account_sid: &'a str,
        friendly_name: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListApplicationResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Applications.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Applications.json`.\n\nCreate a new application within your account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nasync fn example_default_create_application() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountApplication = client\n        .default()\n        .create_application(\n            \"some-string\",\n            &twilio_api::types::CreateApplicationRequest {\n                api_version: Some(\"some-string\".to_string()),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(twilio_api::types::CreateApplicationRequestVoiceMethod::Delete),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_fallback_method: Some(\n                    twilio_api::types::CreateApplicationRequestVoiceFallbackMethod::Get,\n                ),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateApplicationRequestStatusCallbackMethod::Post,\n                ),\n                voice_caller_id_lookup: Some(true),\n                sms_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_method: Some(twilio_api::types::CreateApplicationRequestSmsMethod::Post),\n                sms_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_fallback_method: Some(\n                    twilio_api::types::CreateApplicationRequestSmsFallbackMethod::Delete,\n                ),\n                sms_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                message_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n                public_application_connect_enabled: Some(false),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_application<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateApplicationRequest,
    ) -> Result<crate::types::ApiV2010AccountApplication, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Applications.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json`.\n\nFetch the application specified by the provided sid\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Application resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_application() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountApplication = client\n        .default()\n        .fetch_application(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_application<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountApplication, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json`.\n\nUpdates the application's properties\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Application resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_application() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountApplication = client\n        .default()\n        .update_application(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateApplicationRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n                api_version: Some(\"some-string\".to_string()),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(twilio_api::types::UpdateApplicationRequestVoiceMethod::Post),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_fallback_method: Some(\n                    twilio_api::types::UpdateApplicationRequestVoiceFallbackMethod::Patch,\n                ),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::UpdateApplicationRequestStatusCallbackMethod::Get,\n                ),\n                voice_caller_id_lookup: Some(false),\n                sms_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_method: Some(twilio_api::types::UpdateApplicationRequestSmsMethod::Delete),\n                sms_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_fallback_method: Some(\n                    twilio_api::types::UpdateApplicationRequestSmsFallbackMethod::Put,\n                ),\n                sms_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                message_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                public_application_connect_enabled: Some(true),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_application<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateApplicationRequest,
    ) -> Result<crate::types::ApiV2010AccountApplication, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json`.\n\nDelete the application by the specified application sid\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Application resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Application resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_application() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_application(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_application<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Applications/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps/{ConnectAppSid}.json`.\n\nFetch an instance of an authorized-connect-app\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the AuthorizedConnectApp resource to fetch. (required)\n- `connect_app_sid: &'astr`: The SID of the Connect App to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_authorized_connect_app() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountAuthorizedConnectApp = client\n        .default()\n        .fetch_authorized_connect_app(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_authorized_connect_app<'a>(
        &'a self,
        account_sid: &'a str,
        connect_app_sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountAuthorizedConnectApp, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps/{ConnectAppSid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ConnectAppSid}", connect_app_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps.json`.\n\nRetrieve a list of authorized-connect-apps belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the AuthorizedConnectApp resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_authorized_connect_app() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAuthorizedConnectAppResponse = client\n        .default()\n        .list_authorized_connect_app(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_authorized_connect_app<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListAuthorizedConnectAppResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AuthorizedConnectApps.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the available phone number Country resources. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_available_phone_number_country() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberCountryResponse = client\n        .default()\n        .list_available_phone_number_country(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_country<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListAvailablePhoneNumberCountryResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the available phone number Country resource. (required)\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country to fetch available phone number information about. (required)\n\n```rust,no_run\nasync fn example_default_fetch_available_phone_number_country() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountAvailablePhoneNumberCountry = client\n        .default()\n        .fetch_available_phone_number_country(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_available_phone_number_country<'a>(
        &'a self,
        account_sid: &'a str,
        country_code: &'a str,
    ) -> Result<crate::types::ApiV2010AccountAvailablePhoneNumberCountry, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Local.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. (required)\n- `area_code: Option<i64>`: The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.\n- `beta: Option<bool>`: Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `contains: Option<String>`: The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumberlocal-resource?code-sample=code-find-phone-numbers-by-number-pattern) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumberlocal-resource?code-sample=code-find-phone-numbers-by-character-pattern). If specified, this value must have at least two characters.\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. (required)\n- `distance: Option<i64>`: The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.\n- `exclude_all_address_required: Option<bool>`: Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_foreign_address_required: Option<bool>`: Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_local_address_required: Option<bool>`: Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `fax_enabled: Option<bool>`: Whether the phone numbers can receive faxes. Can be: `true` or `false`.\n- `in_lata: Option<String>`: Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.\n- `in_locality: Option<String>`: Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.\n- `in_postal_code: Option<String>`: Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.\n- `in_rate_center: Option<String>`: Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.\n- `in_region: Option<String>`: Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.\n- `mms_enabled: Option<bool>`: Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.\n- `near_lat_long: Option<String>`: Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.\n- `near_number: crate::types::phone_number::PhoneNumber`: Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `sms_enabled: Option<bool>`: Whether the phone numbers can receive text messages. Can be: `true` or `false`.\n- `voice_enabled: Option<bool>`: Whether the phone numbers can receive calls. Can be: `true` or `false`.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_available_phone_number_local() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberLocalResponse = client\n        .default()\n        .list_available_phone_number_local(\n            \"some-string\",\n            Some(4 as i64),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(true),\n            Some(true),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(false),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_local<'a>(
        &'a self,
        account_sid: &'a str,
        area_code: Option<i64>,
        beta: Option<bool>,
        contains: Option<String>,
        country_code: &'a str,
        distance: Option<i64>,
        exclude_all_address_required: Option<bool>,
        exclude_foreign_address_required: Option<bool>,
        exclude_local_address_required: Option<bool>,
        fax_enabled: Option<bool>,
        in_lata: Option<String>,
        in_locality: Option<String>,
        in_postal_code: Option<String>,
        in_rate_center: Option<String>,
        in_region: Option<String>,
        mms_enabled: Option<bool>,
        near_lat_long: Option<String>,
        near_number: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        sms_enabled: Option<bool>,
        voice_enabled: Option<bool>,
    ) -> Result<crate::types::ListAvailablePhoneNumberLocalResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Local.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = area_code {
            query_params.push(("AreaCode", format!("{p}")));
        }

        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = contains {
            query_params.push(("Contains", p));
        }

        if let Some(p) = distance {
            query_params.push(("Distance", format!("{p}")));
        }

        if let Some(p) = exclude_all_address_required {
            query_params.push(("ExcludeAllAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_foreign_address_required {
            query_params.push(("ExcludeForeignAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_local_address_required {
            query_params.push(("ExcludeLocalAddressRequired", format!("{p}")));
        }

        if let Some(p) = fax_enabled {
            query_params.push(("FaxEnabled", format!("{p}")));
        }

        if let Some(p) = in_lata {
            query_params.push(("InLata", p));
        }

        if let Some(p) = in_locality {
            query_params.push(("InLocality", p));
        }

        if let Some(p) = in_postal_code {
            query_params.push(("InPostalCode", p));
        }

        if let Some(p) = in_rate_center {
            query_params.push(("InRateCenter", p));
        }

        if let Some(p) = in_region {
            query_params.push(("InRegion", p));
        }

        if let Some(p) = mms_enabled {
            query_params.push(("MmsEnabled", format!("{p}")));
        }

        if let Some(p) = near_lat_long {
            query_params.push(("NearLatLong", p));
        }

        if let Some(p) = near_number.0 {
            query_params.push(("NearNumber", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = sms_enabled {
            query_params.push(("SmsEnabled", format!("{p}")));
        }

        if let Some(p) = voice_enabled {
            query_params.push(("VoiceEnabled", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/MachineToMachine.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. (required)\n- `area_code: Option<i64>`: The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.\n- `beta: Option<bool>`: Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `contains: Option<String>`: The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters.\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. (required)\n- `distance: Option<i64>`: The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.\n- `exclude_all_address_required: Option<bool>`: Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_foreign_address_required: Option<bool>`: Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_local_address_required: Option<bool>`: Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `fax_enabled: Option<bool>`: Whether the phone numbers can receive faxes. Can be: `true` or `false`.\n- `in_lata: Option<String>`: Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.\n- `in_locality: Option<String>`: Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.\n- `in_postal_code: Option<String>`: Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.\n- `in_rate_center: Option<String>`: Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.\n- `in_region: Option<String>`: Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.\n- `mms_enabled: Option<bool>`: Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.\n- `near_lat_long: Option<String>`: Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.\n- `near_number: crate::types::phone_number::PhoneNumber`: Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `sms_enabled: Option<bool>`: Whether the phone numbers can receive text messages. Can be: `true` or `false`.\n- `voice_enabled: Option<bool>`: Whether the phone numbers can receive calls. Can be: `true` or `false`.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_available_phone_number_machine_to_machine() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberMachineToMachineResponse = client\n        .default()\n        .list_available_phone_number_machine_to_machine(\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(true),\n            Some(false),\n            Some(false),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(true),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_machine_to_machine<'a>(
        &'a self,
        account_sid: &'a str,
        area_code: Option<i64>,
        beta: Option<bool>,
        contains: Option<String>,
        country_code: &'a str,
        distance: Option<i64>,
        exclude_all_address_required: Option<bool>,
        exclude_foreign_address_required: Option<bool>,
        exclude_local_address_required: Option<bool>,
        fax_enabled: Option<bool>,
        in_lata: Option<String>,
        in_locality: Option<String>,
        in_postal_code: Option<String>,
        in_rate_center: Option<String>,
        in_region: Option<String>,
        mms_enabled: Option<bool>,
        near_lat_long: Option<String>,
        near_number: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        sms_enabled: Option<bool>,
        voice_enabled: Option<bool>,
    ) -> Result<
        crate::types::ListAvailablePhoneNumberMachineToMachineResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/\
                 MachineToMachine.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = area_code {
            query_params.push(("AreaCode", format!("{p}")));
        }

        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = contains {
            query_params.push(("Contains", p));
        }

        if let Some(p) = distance {
            query_params.push(("Distance", format!("{p}")));
        }

        if let Some(p) = exclude_all_address_required {
            query_params.push(("ExcludeAllAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_foreign_address_required {
            query_params.push(("ExcludeForeignAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_local_address_required {
            query_params.push(("ExcludeLocalAddressRequired", format!("{p}")));
        }

        if let Some(p) = fax_enabled {
            query_params.push(("FaxEnabled", format!("{p}")));
        }

        if let Some(p) = in_lata {
            query_params.push(("InLata", p));
        }

        if let Some(p) = in_locality {
            query_params.push(("InLocality", p));
        }

        if let Some(p) = in_postal_code {
            query_params.push(("InPostalCode", p));
        }

        if let Some(p) = in_rate_center {
            query_params.push(("InRateCenter", p));
        }

        if let Some(p) = in_region {
            query_params.push(("InRegion", p));
        }

        if let Some(p) = mms_enabled {
            query_params.push(("MmsEnabled", format!("{p}")));
        }

        if let Some(p) = near_lat_long {
            query_params.push(("NearLatLong", p));
        }

        if let Some(p) = near_number.0 {
            query_params.push(("NearNumber", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = sms_enabled {
            query_params.push(("SmsEnabled", format!("{p}")));
        }

        if let Some(p) = voice_enabled {
            query_params.push(("VoiceEnabled", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Mobile.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. (required)\n- `area_code: Option<i64>`: The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.\n- `beta: Option<bool>`: Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `contains: Option<String>`: The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters.\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. (required)\n- `distance: Option<i64>`: The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.\n- `exclude_all_address_required: Option<bool>`: Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_foreign_address_required: Option<bool>`: Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_local_address_required: Option<bool>`: Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `fax_enabled: Option<bool>`: Whether the phone numbers can receive faxes. Can be: `true` or `false`.\n- `in_lata: Option<String>`: Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.\n- `in_locality: Option<String>`: Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.\n- `in_postal_code: Option<String>`: Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.\n- `in_rate_center: Option<String>`: Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.\n- `in_region: Option<String>`: Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.\n- `mms_enabled: Option<bool>`: Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.\n- `near_lat_long: Option<String>`: Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.\n- `near_number: crate::types::phone_number::PhoneNumber`: Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `sms_enabled: Option<bool>`: Whether the phone numbers can receive text messages. Can be: `true` or `false`.\n- `voice_enabled: Option<bool>`: Whether the phone numbers can receive calls. Can be: `true` or `false`.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_available_phone_number_mobile() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberMobileResponse = client\n        .default()\n        .list_available_phone_number_mobile(\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(true),\n            Some(true),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(false),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_mobile<'a>(
        &'a self,
        account_sid: &'a str,
        area_code: Option<i64>,
        beta: Option<bool>,
        contains: Option<String>,
        country_code: &'a str,
        distance: Option<i64>,
        exclude_all_address_required: Option<bool>,
        exclude_foreign_address_required: Option<bool>,
        exclude_local_address_required: Option<bool>,
        fax_enabled: Option<bool>,
        in_lata: Option<String>,
        in_locality: Option<String>,
        in_postal_code: Option<String>,
        in_rate_center: Option<String>,
        in_region: Option<String>,
        mms_enabled: Option<bool>,
        near_lat_long: Option<String>,
        near_number: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        sms_enabled: Option<bool>,
        voice_enabled: Option<bool>,
    ) -> Result<crate::types::ListAvailablePhoneNumberMobileResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Mobile.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = area_code {
            query_params.push(("AreaCode", format!("{p}")));
        }

        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = contains {
            query_params.push(("Contains", p));
        }

        if let Some(p) = distance {
            query_params.push(("Distance", format!("{p}")));
        }

        if let Some(p) = exclude_all_address_required {
            query_params.push(("ExcludeAllAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_foreign_address_required {
            query_params.push(("ExcludeForeignAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_local_address_required {
            query_params.push(("ExcludeLocalAddressRequired", format!("{p}")));
        }

        if let Some(p) = fax_enabled {
            query_params.push(("FaxEnabled", format!("{p}")));
        }

        if let Some(p) = in_lata {
            query_params.push(("InLata", p));
        }

        if let Some(p) = in_locality {
            query_params.push(("InLocality", p));
        }

        if let Some(p) = in_postal_code {
            query_params.push(("InPostalCode", p));
        }

        if let Some(p) = in_rate_center {
            query_params.push(("InRateCenter", p));
        }

        if let Some(p) = in_region {
            query_params.push(("InRegion", p));
        }

        if let Some(p) = mms_enabled {
            query_params.push(("MmsEnabled", format!("{p}")));
        }

        if let Some(p) = near_lat_long {
            query_params.push(("NearLatLong", p));
        }

        if let Some(p) = near_number.0 {
            query_params.push(("NearNumber", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = sms_enabled {
            query_params.push(("SmsEnabled", format!("{p}")));
        }

        if let Some(p) = voice_enabled {
            query_params.push(("VoiceEnabled", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/National.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. (required)\n- `area_code: Option<i64>`: The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.\n- `beta: Option<bool>`: Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `contains: Option<String>`: The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters.\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. (required)\n- `distance: Option<i64>`: The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.\n- `exclude_all_address_required: Option<bool>`: Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_foreign_address_required: Option<bool>`: Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_local_address_required: Option<bool>`: Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `fax_enabled: Option<bool>`: Whether the phone numbers can receive faxes. Can be: `true` or `false`.\n- `in_lata: Option<String>`: Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.\n- `in_locality: Option<String>`: Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.\n- `in_postal_code: Option<String>`: Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.\n- `in_rate_center: Option<String>`: Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.\n- `in_region: Option<String>`: Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.\n- `mms_enabled: Option<bool>`: Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.\n- `near_lat_long: Option<String>`: Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.\n- `near_number: crate::types::phone_number::PhoneNumber`: Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `sms_enabled: Option<bool>`: Whether the phone numbers can receive text messages. Can be: `true` or `false`.\n- `voice_enabled: Option<bool>`: Whether the phone numbers can receive calls. Can be: `true` or `false`.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_available_phone_number_national() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberNationalResponse = client\n        .default()\n        .list_available_phone_number_national(\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(true),\n            Some(false),\n            Some(false),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(false),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_national<'a>(
        &'a self,
        account_sid: &'a str,
        area_code: Option<i64>,
        beta: Option<bool>,
        contains: Option<String>,
        country_code: &'a str,
        distance: Option<i64>,
        exclude_all_address_required: Option<bool>,
        exclude_foreign_address_required: Option<bool>,
        exclude_local_address_required: Option<bool>,
        fax_enabled: Option<bool>,
        in_lata: Option<String>,
        in_locality: Option<String>,
        in_postal_code: Option<String>,
        in_rate_center: Option<String>,
        in_region: Option<String>,
        mms_enabled: Option<bool>,
        near_lat_long: Option<String>,
        near_number: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        sms_enabled: Option<bool>,
        voice_enabled: Option<bool>,
    ) -> Result<crate::types::ListAvailablePhoneNumberNationalResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/National.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = area_code {
            query_params.push(("AreaCode", format!("{p}")));
        }

        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = contains {
            query_params.push(("Contains", p));
        }

        if let Some(p) = distance {
            query_params.push(("Distance", format!("{p}")));
        }

        if let Some(p) = exclude_all_address_required {
            query_params.push(("ExcludeAllAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_foreign_address_required {
            query_params.push(("ExcludeForeignAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_local_address_required {
            query_params.push(("ExcludeLocalAddressRequired", format!("{p}")));
        }

        if let Some(p) = fax_enabled {
            query_params.push(("FaxEnabled", format!("{p}")));
        }

        if let Some(p) = in_lata {
            query_params.push(("InLata", p));
        }

        if let Some(p) = in_locality {
            query_params.push(("InLocality", p));
        }

        if let Some(p) = in_postal_code {
            query_params.push(("InPostalCode", p));
        }

        if let Some(p) = in_rate_center {
            query_params.push(("InRateCenter", p));
        }

        if let Some(p) = in_region {
            query_params.push(("InRegion", p));
        }

        if let Some(p) = mms_enabled {
            query_params.push(("MmsEnabled", format!("{p}")));
        }

        if let Some(p) = near_lat_long {
            query_params.push(("NearLatLong", p));
        }

        if let Some(p) = near_number.0 {
            query_params.push(("NearNumber", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = sms_enabled {
            query_params.push(("SmsEnabled", format!("{p}")));
        }

        if let Some(p) = voice_enabled {
            query_params.push(("VoiceEnabled", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/SharedCost.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. (required)\n- `area_code: Option<i64>`: The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.\n- `beta: Option<bool>`: Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `contains: Option<String>`: The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters.\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. (required)\n- `distance: Option<i64>`: The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.\n- `exclude_all_address_required: Option<bool>`: Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_foreign_address_required: Option<bool>`: Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_local_address_required: Option<bool>`: Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `fax_enabled: Option<bool>`: Whether the phone numbers can receive faxes. Can be: `true` or `false`.\n- `in_lata: Option<String>`: Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.\n- `in_locality: Option<String>`: Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.\n- `in_postal_code: Option<String>`: Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.\n- `in_rate_center: Option<String>`: Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.\n- `in_region: Option<String>`: Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.\n- `mms_enabled: Option<bool>`: Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.\n- `near_lat_long: Option<String>`: Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.\n- `near_number: crate::types::phone_number::PhoneNumber`: Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `sms_enabled: Option<bool>`: Whether the phone numbers can receive text messages. Can be: `true` or `false`.\n- `voice_enabled: Option<bool>`: Whether the phone numbers can receive calls. Can be: `true` or `false`.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_available_phone_number_shared_cost() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberSharedCostResponse = client\n        .default()\n        .list_available_phone_number_shared_cost(\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(true),\n            Some(true),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(false),\n            Some(true),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_shared_cost<'a>(
        &'a self,
        account_sid: &'a str,
        area_code: Option<i64>,
        beta: Option<bool>,
        contains: Option<String>,
        country_code: &'a str,
        distance: Option<i64>,
        exclude_all_address_required: Option<bool>,
        exclude_foreign_address_required: Option<bool>,
        exclude_local_address_required: Option<bool>,
        fax_enabled: Option<bool>,
        in_lata: Option<String>,
        in_locality: Option<String>,
        in_postal_code: Option<String>,
        in_rate_center: Option<String>,
        in_region: Option<String>,
        mms_enabled: Option<bool>,
        near_lat_long: Option<String>,
        near_number: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        sms_enabled: Option<bool>,
        voice_enabled: Option<bool>,
    ) -> Result<crate::types::ListAvailablePhoneNumberSharedCostResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/SharedCost.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = area_code {
            query_params.push(("AreaCode", format!("{p}")));
        }

        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = contains {
            query_params.push(("Contains", p));
        }

        if let Some(p) = distance {
            query_params.push(("Distance", format!("{p}")));
        }

        if let Some(p) = exclude_all_address_required {
            query_params.push(("ExcludeAllAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_foreign_address_required {
            query_params.push(("ExcludeForeignAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_local_address_required {
            query_params.push(("ExcludeLocalAddressRequired", format!("{p}")));
        }

        if let Some(p) = fax_enabled {
            query_params.push(("FaxEnabled", format!("{p}")));
        }

        if let Some(p) = in_lata {
            query_params.push(("InLata", p));
        }

        if let Some(p) = in_locality {
            query_params.push(("InLocality", p));
        }

        if let Some(p) = in_postal_code {
            query_params.push(("InPostalCode", p));
        }

        if let Some(p) = in_rate_center {
            query_params.push(("InRateCenter", p));
        }

        if let Some(p) = in_region {
            query_params.push(("InRegion", p));
        }

        if let Some(p) = mms_enabled {
            query_params.push(("MmsEnabled", format!("{p}")));
        }

        if let Some(p) = near_lat_long {
            query_params.push(("NearLatLong", p));
        }

        if let Some(p) = near_number.0 {
            query_params.push(("NearNumber", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = sms_enabled {
            query_params.push(("SmsEnabled", format!("{p}")));
        }

        if let Some(p) = voice_enabled {
            query_params.push(("VoiceEnabled", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/TollFree.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. (required)\n- `area_code: Option<i64>`: The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.\n- `beta: Option<bool>`: Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `contains: Option<String>`: The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters.\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. (required)\n- `distance: Option<i64>`: The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.\n- `exclude_all_address_required: Option<bool>`: Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_foreign_address_required: Option<bool>`: Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_local_address_required: Option<bool>`: Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `fax_enabled: Option<bool>`: Whether the phone numbers can receive faxes. Can be: `true` or `false`.\n- `in_lata: Option<String>`: Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.\n- `in_locality: Option<String>`: Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.\n- `in_postal_code: Option<String>`: Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.\n- `in_rate_center: Option<String>`: Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.\n- `in_region: Option<String>`: Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.\n- `mms_enabled: Option<bool>`: Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.\n- `near_lat_long: Option<String>`: Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.\n- `near_number: crate::types::phone_number::PhoneNumber`: Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `sms_enabled: Option<bool>`: Whether the phone numbers can receive text messages. Can be: `true` or `false`.\n- `voice_enabled: Option<bool>`: Whether the phone numbers can receive calls. Can be: `true` or `false`.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_available_phone_number_toll_free() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberTollFreeResponse = client\n        .default()\n        .list_available_phone_number_toll_free(\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(true),\n            Some(true),\n            Some(false),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(false),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_toll_free<'a>(
        &'a self,
        account_sid: &'a str,
        area_code: Option<i64>,
        beta: Option<bool>,
        contains: Option<String>,
        country_code: &'a str,
        distance: Option<i64>,
        exclude_all_address_required: Option<bool>,
        exclude_foreign_address_required: Option<bool>,
        exclude_local_address_required: Option<bool>,
        fax_enabled: Option<bool>,
        in_lata: Option<String>,
        in_locality: Option<String>,
        in_postal_code: Option<String>,
        in_rate_center: Option<String>,
        in_region: Option<String>,
        mms_enabled: Option<bool>,
        near_lat_long: Option<String>,
        near_number: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        sms_enabled: Option<bool>,
        voice_enabled: Option<bool>,
    ) -> Result<crate::types::ListAvailablePhoneNumberTollFreeResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/TollFree.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = area_code {
            query_params.push(("AreaCode", format!("{p}")));
        }

        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = contains {
            query_params.push(("Contains", p));
        }

        if let Some(p) = distance {
            query_params.push(("Distance", format!("{p}")));
        }

        if let Some(p) = exclude_all_address_required {
            query_params.push(("ExcludeAllAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_foreign_address_required {
            query_params.push(("ExcludeForeignAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_local_address_required {
            query_params.push(("ExcludeLocalAddressRequired", format!("{p}")));
        }

        if let Some(p) = fax_enabled {
            query_params.push(("FaxEnabled", format!("{p}")));
        }

        if let Some(p) = in_lata {
            query_params.push(("InLata", p));
        }

        if let Some(p) = in_locality {
            query_params.push(("InLocality", p));
        }

        if let Some(p) = in_postal_code {
            query_params.push(("InPostalCode", p));
        }

        if let Some(p) = in_rate_center {
            query_params.push(("InRateCenter", p));
        }

        if let Some(p) = in_region {
            query_params.push(("InRegion", p));
        }

        if let Some(p) = mms_enabled {
            query_params.push(("MmsEnabled", format!("{p}")));
        }

        if let Some(p) = near_lat_long {
            query_params.push(("NearLatLong", p));
        }

        if let Some(p) = near_number.0 {
            query_params.push(("NearNumber", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = sms_enabled {
            query_params.push(("SmsEnabled", format!("{p}")));
        }

        if let Some(p) = voice_enabled {
            query_params.push(("VoiceEnabled", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Voip.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) requesting the AvailablePhoneNumber resources. (required)\n- `area_code: Option<i64>`: The area code of the phone numbers to read. Applies to only phone numbers in the US and Canada.\n- `beta: Option<bool>`: Whether to read phone numbers that are new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `contains: Option<String>`: The pattern on which to match phone numbers. Valid characters are `*`, `0-9`, `a-z`, and `A-Z`. The `*` character matches any single digit. For examples, see [Example 2](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-2) and [Example 3](https://www.twilio.com/docs/phone-numbers/api/availablephonenumber-resource#local-get-basic-example-3). If specified, this value must have at least two characters.\n- `country_code: &'astr`: The [ISO-3166-1](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country code of the country from which to read phone numbers. (required)\n- `distance: Option<i64>`: The search radius, in miles, for a `near_` query.  Can be up to `500` and the default is `25`. Applies to only phone numbers in the US and Canada.\n- `exclude_all_address_required: Option<bool>`: Whether to exclude phone numbers that require an [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_foreign_address_required: Option<bool>`: Whether to exclude phone numbers that require a foreign [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `exclude_local_address_required: Option<bool>`: Whether to exclude phone numbers that require a local [Address](https://www.twilio.com/docs/usage/api/address). Can be: `true` or `false` and the default is `false`.\n- `fax_enabled: Option<bool>`: Whether the phone numbers can receive faxes. Can be: `true` or `false`.\n- `in_lata: Option<String>`: Limit results to a specific local access and transport area ([LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area)). Given a phone number, search within the same [LATA](https://en.wikipedia.org/wiki/Local_access_and_transport_area) as that number. Applies to only phone numbers in the US and Canada.\n- `in_locality: Option<String>`: Limit results to a particular locality or city. Given a phone number, search within the same Locality as that number.\n- `in_postal_code: Option<String>`: Limit results to a particular postal code. Given a phone number, search within the same postal code as that number. Applies to only phone numbers in the US and Canada.\n- `in_rate_center: Option<String>`: Limit results to a specific rate center, or given a phone number search within the same rate center as that number. Requires `in_lata` to be set as well. Applies to only phone numbers in the US and Canada.\n- `in_region: Option<String>`: Limit results to a particular region, state, or province. Given a phone number, search within the same region as that number. Applies to only phone numbers in the US and Canada.\n- `mms_enabled: Option<bool>`: Whether the phone numbers can receive MMS messages. Can be: `true` or `false`.\n- `near_lat_long: Option<String>`: Given a latitude/longitude pair `lat,long` find geographically close numbers within `distance` miles. Applies to only phone numbers in the US and Canada.\n- `near_number: crate::types::phone_number::PhoneNumber`: Given a phone number, find a geographically close number within `distance` miles. Distance defaults to 25 miles. Applies to only phone numbers in the US and Canada.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `sms_enabled: Option<bool>`: Whether the phone numbers can receive text messages. Can be: `true` or `false`.\n- `voice_enabled: Option<bool>`: Whether the phone numbers can receive calls. Can be: `true` or `false`.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_available_phone_number_voip() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListAvailablePhoneNumberVoipResponse = client\n        .default()\n        .list_available_phone_number_voip(\n            \"some-string\",\n            Some(4 as i64),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(true),\n            Some(true),\n            Some(false),\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(true),\n            Some(false),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_available_phone_number_voip<'a>(
        &'a self,
        account_sid: &'a str,
        area_code: Option<i64>,
        beta: Option<bool>,
        contains: Option<String>,
        country_code: &'a str,
        distance: Option<i64>,
        exclude_all_address_required: Option<bool>,
        exclude_foreign_address_required: Option<bool>,
        exclude_local_address_required: Option<bool>,
        fax_enabled: Option<bool>,
        in_lata: Option<String>,
        in_locality: Option<String>,
        in_postal_code: Option<String>,
        in_rate_center: Option<String>,
        in_region: Option<String>,
        mms_enabled: Option<bool>,
        near_lat_long: Option<String>,
        near_number: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        sms_enabled: Option<bool>,
        voice_enabled: Option<bool>,
    ) -> Result<crate::types::ListAvailablePhoneNumberVoipResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/AvailablePhoneNumbers/{CountryCode}/Voip.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CountryCode}", country_code)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = area_code {
            query_params.push(("AreaCode", format!("{p}")));
        }

        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = contains {
            query_params.push(("Contains", p));
        }

        if let Some(p) = distance {
            query_params.push(("Distance", format!("{p}")));
        }

        if let Some(p) = exclude_all_address_required {
            query_params.push(("ExcludeAllAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_foreign_address_required {
            query_params.push(("ExcludeForeignAddressRequired", format!("{p}")));
        }

        if let Some(p) = exclude_local_address_required {
            query_params.push(("ExcludeLocalAddressRequired", format!("{p}")));
        }

        if let Some(p) = fax_enabled {
            query_params.push(("FaxEnabled", format!("{p}")));
        }

        if let Some(p) = in_lata {
            query_params.push(("InLata", p));
        }

        if let Some(p) = in_locality {
            query_params.push(("InLocality", p));
        }

        if let Some(p) = in_postal_code {
            query_params.push(("InPostalCode", p));
        }

        if let Some(p) = in_rate_center {
            query_params.push(("InRateCenter", p));
        }

        if let Some(p) = in_region {
            query_params.push(("InRegion", p));
        }

        if let Some(p) = mms_enabled {
            query_params.push(("MmsEnabled", format!("{p}")));
        }

        if let Some(p) = near_lat_long {
            query_params.push(("NearLatLong", p));
        }

        if let Some(p) = near_number.0 {
            query_params.push(("NearNumber", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = sms_enabled {
            query_params.push(("SmsEnabled", format!("{p}")));
        }

        if let Some(p) = voice_enabled {
            query_params.push(("VoiceEnabled", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Balance.json`.\n\nFetch \
             the balance for an Account based on Account Sid. Balance changes may not be reflected \
             immediately. Child accounts do not contain balance \
             information\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique SID identifier \
             of the Account. (required)\n\n```rust,no_run\nasync fn \
             example_default_fetch_balance() -> anyhow::Result<()> {\n    let client = \
             twilio_api::Client::new_from_env();\n    let result: \
             twilio_api::types::ApiV2010AccountBalance =\n        \
             client.default().fetch_balance(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_balance<'a>(
        &'a self,
        account_sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Balance.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls.json`.\n\nRetrieves a collection of calls made to and from your account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to read. (required)\n- `end_time: Option<chrono::DateTime<chrono::Utc>>`: Only include calls that ended on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that ended on this date. You can also specify an inequality, such as `EndTime<=YYYY-MM-DD`, to read calls that ended on or before midnight of this date, and `EndTime>=YYYY-MM-DD` to read calls that ended on or after midnight of this date.\n- `from: crate::types::phone_number::PhoneNumber`: Only include calls from this phone number, SIP address, Client identifier or SIM SID.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `parent_call_sid: Option<String>`: Only include calls spawned by calls with this SID.\n- `start_time: Option<chrono::DateTime<chrono::Utc>>`: Only include calls that started on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read only calls that started on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read calls that started on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read calls that started on or after midnight of this date.\n- `status: Option<crate::types::CallEnumStatus>`: The status of the calls to include. Can be: `queued`, `ringing`, `in-progress`, `canceled`, `completed`, `failed`, `busy`, or `no-answer`.\n- `to: crate::types::phone_number::PhoneNumber`: Only show calls made to this phone number, SIP address, Client identifier or SIM SID.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_call() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListCallResponse = client\n        .default()\n        .list_call(\n            \"some-string\",\n            Some(chrono::Utc::now()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now()),\n            Some(twilio_api::types::CallEnumStatus::Canceled),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_call<'a>(
        &'a self,
        account_sid: &'a str,
        end_time: Option<chrono::DateTime<chrono::Utc>>,
        from: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        parent_call_sid: Option<String>,
        start_time: Option<chrono::DateTime<chrono::Utc>>,
        status: Option<crate::types::CallEnumStatus>,
        to: crate::types::phone_number::PhoneNumber,
    ) -> Result<crate::types::ListCallResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls.json".replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = end_time {
            query_params.push(("EndTime", format!("{p}")));
        }

        if let Some(p) = from.0 {
            query_params.push(("From", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = parent_call_sid {
            query_params.push(("ParentCallSid", p));
        }

        if let Some(p) = start_time {
            query_params.push(("StartTime", format!("{p}")));
        }

        if let Some(p) = status {
            query_params.push(("Status", format!("{p}")));
        }

        if let Some(p) = to.0 {
            query_params.push(("To", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls.json`.\n\nCreate a new outgoing call to phones, SIP-enabled endpoints or Twilio Client connections\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nasync fn example_default_create_call() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCall = client\n        .default()\n        .create_call(\n            \"some-string\",\n            &twilio_api::types::CreateCallRequest {\n                to: \"some-string\".to_string(),\n                from: \"some-string\".to_string(),\n                method: Some(twilio_api::types::Method::Head),\n                fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                fallback_method: Some(twilio_api::types::FallbackMethod::Delete),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_event: Some(vec![\"some-string\".to_string()]),\n                status_callback_method: Some(\n                    twilio_api::types::CreateCallRequestStatusCallbackMethod::Patch,\n                ),\n                send_digits: Some(\"some-string\".to_string()),\n                timeout: Some(4 as i64),\n                record: Some(false),\n                recording_channels: Some(\"some-string\".to_string()),\n                recording_status_callback: Some(\"some-string\".to_string()),\n                recording_status_callback_method: Some(\n                    twilio_api::types::RecordingStatusCallbackMethod::Patch,\n                ),\n                sip_auth_username: Some(\"some-string\".to_string()),\n                sip_auth_password: Some(\"some-string\".to_string()),\n                machine_detection: Some(\"some-string\".to_string()),\n                machine_detection_timeout: Some(4 as i64),\n                recording_status_callback_event: Some(vec![\"some-string\".to_string()]),\n                trim: Some(\"some-string\".to_string()),\n                caller_id: Some(\"some-string\".to_string()),\n                machine_detection_speech_threshold: Some(4 as i64),\n                machine_detection_speech_end_threshold: Some(4 as i64),\n                machine_detection_silence_timeout: Some(4 as i64),\n                async_amd: Some(\"some-string\".to_string()),\n                async_amd_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                async_amd_status_callback_method: Some(\n                    twilio_api::types::AsyncAmdStatusCallbackMethod::Put,\n                ),\n                byoc: Some(\"some-string\".to_string()),\n                call_reason: Some(\"some-string\".to_string()),\n                call_token: Some(\"some-string\".to_string()),\n                recording_track: Some(\"some-string\".to_string()),\n                time_limit: Some(4 as i64),\n                url: Some(\"https://example.com/foo/bar\".to_string()),\n                twiml: Some(\"some-string\".to_string()),\n                application_sid: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_call<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateCallRequest,
    ) -> Result<crate::types::ApiV2010AccountCall, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls.json".replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json`.\n\nFetch the call specified by the provided Call SID\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to fetch. (required)\n- `sid: &'astr`: The SID of the Call resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_call() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCall = client\n        .default()\n        .fetch_call(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_call<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountCall, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json`.\n\nInitiates a call redirect or terminates a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Call resource to update (required)\n\n```rust,no_run\nasync fn example_default_update_call() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCall = client\n        .default()\n        .update_call(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateCallRequest {\n                url: Some(\"https://example.com/foo/bar\".to_string()),\n                method: Some(twilio_api::types::UpdateCallRequestMethod::Patch),\n                status: Some(twilio_api::types::CallEnumUpdateStatus::Completed),\n                fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                fallback_method: Some(twilio_api::types::FallbackMethod::Post),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::UpdateCallRequestStatusCallbackMethod::Post,\n                ),\n                twiml: Some(\"some-string\".to_string()),\n                time_limit: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_call<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateCallRequest,
    ) -> Result<crate::types::ApiV2010AccountCall, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json`.\n\nDelete a Call record from your account. Once the record is deleted, it will no longer appear in the API and Account Portal logs.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call resource(s) to delete. (required)\n- `sid: &'astr`: The Twilio-provided Call SID that uniquely identifies the Call resource to delete (required)\n\n```rust,no_run\nasync fn example_default_delete_call() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_call(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_call<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Events.json`.\n\nRetrieve a list of all events for a call.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique SID identifier of the Account. (required)\n- `call_sid: &'astr`: The unique SID identifier of the Call. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_call_event() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListCallEventResponse = client\n        .default()\n        .list_call_event(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_call_event<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListCallEventResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Events.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json`.\n\nFetch a Feedback resource from a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `call_sid: &'astr`: The call sid that uniquely identifies the call (required)\n\n```rust,no_run\nasync fn example_default_fetch_call_feedback() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallFeedback = client\n        .default()\n        .fetch_call_feedback(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_call_feedback<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountCallCallFeedback, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json`.\n\nUpdate a Feedback resource for a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `call_sid: &'astr`: The call sid that uniquely identifies the call (required)\n\n```rust,no_run\nasync fn example_default_update_call_feedback() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallFeedback = client\n        .default()\n        .update_call_feedback(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateCallFeedbackRequest {\n                quality_score: Some(4 as i64),\n                issue: Some(vec![\n                    twilio_api::types::CallFeedbackEnumIssues::DigitsNotCaptured,\n                ]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_call_feedback<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        body: &crate::types::UpdateCallFeedbackRequest,
    ) -> Result<crate::types::ApiV2010AccountCallCallFeedback, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Feedback.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary.json`.\n\nCreate a FeedbackSummary resource for a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n\n```rust,no_run\nasync fn example_default_create_call_feedback_summary() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallFeedbackSummary = client\n        .default()\n        .create_call_feedback_summary(\n            \"some-string\",\n            &twilio_api::types::CreateCallFeedbackSummaryRequest {\n                start_date: chrono::Utc::now().date().naive_utc(),\n                end_date: chrono::Utc::now().date().naive_utc(),\n                include_subaccounts: Some(true),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateCallFeedbackSummaryRequestStatusCallbackMethod::Get,\n                ),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_call_feedback_summary<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateCallFeedbackSummaryRequest,
    ) -> Result<crate::types::ApiV2010AccountCallCallFeedbackSummary, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json`.\n\nFetch a FeedbackSummary resource from a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies this resource. (required)\n\n```rust,no_run\nasync fn example_default_fetch_call_feedback_summary() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallFeedbackSummary = client\n        .default()\n        .fetch_call_feedback_summary(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_call_feedback_summary<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountCallCallFeedbackSummary, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json`.\n\nDelete a FeedbackSummary resource from a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies this resource. (required)\n\n```rust,no_run\nasync fn example_default_delete_call_feedback_summary() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_call_feedback_summary(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_call_feedback_summary<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/FeedbackSummary/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call Notification resource to fetch. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the Call Notification resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Call Notification resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_call_notification() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallNotificationInstance = client\n        .default()\n        .fetch_call_notification(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_call_notification<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountCallCallNotificationInstance, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Call Notification resources to read. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the Call Notification resources to read. (required)\n- `log: Option<i64>`: Only read notifications of the specified log level. Can be:  `0` to read only ERROR notifications or `1` to read only WARNING notifications. By default, all notifications are read.\n- `message_date: Option<chrono::NaiveDate>`: Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_call_notification() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListCallNotificationResponse = client\n        .default()\n        .list_call_notification(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_call_notification<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        log: Option<i64>,
        message_date: Option<chrono::NaiveDate>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListCallNotificationResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Notifications.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = log {
            query_params.push(("Log", format!("{p}")));
        }

        if let Some(p) = message_date {
            query_params.push(("MessageDate", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json`.\n\nRetrieve a list of recordings belonging to the call used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to read. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resources to read. (required)\n- `date_created: Option<chrono::NaiveDate>`: The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_call_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListCallRecordingResponse = client\n        .default()\n        .list_call_recording(\n            \"some-string\",\n            \"some-string\",\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_call_recording<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        date_created: Option<chrono::NaiveDate>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListCallRecordingResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = date_created {
            query_params.push(("DateCreated", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json`.\n\nCreate a recording for the call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) to associate the resource with. (required)\n\n```rust,no_run\nasync fn example_default_create_call_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallRecording = client\n        .default()\n        .create_call_recording(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateCallRecordingRequest {\n                recording_status_callback_event: Some(vec![\"some-string\".to_string()]),\n                recording_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                recording_status_callback_method: Some(\n                    twilio_api::types::CreateCallRecordingRequestRecordingStatusCallbackMethod::Get,\n                ),\n                trim: Some(\"some-string\".to_string()),\n                recording_channels: Some(\"some-string\".to_string()),\n                recording_track: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_call_recording<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        body: &crate::types::CreateCallRecordingRequest,
    ) -> Result<crate::types::ApiV2010AccountCallCallRecording, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json`.\n\nFetch an instance of a recording for a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resource to fetch. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_call_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallRecording = client\n        .default()\n        .fetch_call_recording(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_call_recording<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountCallCallRecording, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json`.\n\nChanges the status of the recording to paused, stopped, or in-progress. Note: Pass `Twilio.CURRENT` instead of recording sid to reference current active recording.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resource to update. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_call_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallCallRecording = client\n        .default()\n        .update_call_recording(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateCallRecordingRequest {\n                status: twilio_api::types::CallRecordingEnumStatus::InProgress,\n                pause_behavior: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_call_recording<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateCallRecordingRequest,
    ) -> Result<crate::types::ApiV2010AccountCallCallRecording, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json`.\n\nDelete a recording from your account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to delete. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_call_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_call_recording(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_call_recording<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Recordings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json`.\n\nFetch an instance of a conference\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference resource(s) to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Conference resource to fetch (required)\n\n```rust,no_run\nasync fn example_default_fetch_conference() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConference = client\n        .default()\n        .fetch_conference(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_conference<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountConference, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference resource(s) to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Conference resource to update (required)\n\n```rust,no_run\nasync fn example_default_update_conference() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConference = client\n        .default()\n        .update_conference(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateConferenceRequest {\n                status: Some(twilio_api::types::ConferenceEnumUpdateStatus::Completed),\n                announce_url: Some(\"https://example.com/foo/bar\".to_string()),\n                announce_method: Some(twilio_api::types::AnnounceMethod::Patch),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_conference<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateConferenceRequest,
    ) -> Result<crate::types::ApiV2010AccountConference, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Conferences.json`.\n\nRetrieve a list of conferences belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference resource(s) to read. (required)\n- `date_created: Option<chrono::NaiveDate>`: The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that started on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify  conferences that started on or after midnight on a date, use `>=YYYY-MM-DD`.\n- `date_updated: Option<chrono::NaiveDate>`: The `date_updated` value, specified as `YYYY-MM-DD`, of the resources to read. To read conferences that were last updated on or before midnight on a date, use `<=YYYY-MM-DD`, and to specify conferences that were last updated on or after midnight on a given date, use  `>=YYYY-MM-DD`.\n- `friendly_name: Option<String>`: The string that identifies the Conference resources to read.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `status: Option<crate::types::ConferenceEnumStatus>`: The status of the resources to read. Can be: `init`, `in-progress`, or `completed`.\n\n```rust,no_run\nasync fn example_default_list_conference() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListConferenceResponse = client\n        .default()\n        .list_conference(\n            \"some-string\",\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::ConferenceEnumStatus::Completed),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_conference<'a>(
        &'a self,
        account_sid: &'a str,
        date_created: Option<chrono::NaiveDate>,
        date_updated: Option<chrono::NaiveDate>,
        friendly_name: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        status: Option<crate::types::ConferenceEnumStatus>,
    ) -> Result<crate::types::ListConferenceResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = date_created {
            query_params.push(("DateCreated", format!("{p}")));
        }

        if let Some(p) = date_updated {
            query_params.push(("DateUpdated", format!("{p}")));
        }

        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = status {
            query_params.push(("Status", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json`.\n\nFetch an instance of a recording for a call\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resource to fetch. (required)\n- `conference_sid: &'astr`: The Conference SID that identifies the conference associated with the recording to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Conference Recording resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_conference_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConferenceConferenceRecording = client\n        .default()\n        .fetch_conference_recording(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_conference_recording<'a>(
        &'a self,
        account_sid: &'a str,
        conference_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountConferenceConferenceRecording,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ConferenceSid}", conference_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json`.\n\nChanges the status of the recording to paused, stopped, or in-progress. Note: To use `Twilio.CURRENT`, pass it as recording sid.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resource to update. (required)\n- `conference_sid: &'astr`: The Conference SID that identifies the conference associated with the recording to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Conference Recording resource to update. Use `Twilio.CURRENT` to reference the current active recording. (required)\n\n```rust,no_run\nasync fn example_default_update_conference_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConferenceConferenceRecording = client\n        .default()\n        .update_conference_recording(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateConferenceRecordingRequest {\n                status: twilio_api::types::ConferenceRecordingEnumStatus::Completed,\n                pause_behavior: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_conference_recording<'a>(
        &'a self,
        account_sid: &'a str,
        conference_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateConferenceRecordingRequest,
    ) -> Result<
        crate::types::ApiV2010AccountConferenceConferenceRecording,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ConferenceSid}", conference_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.json`.\n\nDelete a recording from your account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resources to delete. (required)\n- `conference_sid: &'astr`: The Conference SID that identifies the conference associated with the recording to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Conference Recording resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_conference_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_conference_recording(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_conference_recording<'a>(
        &'a self,
        account_sid: &'a str,
        conference_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings/{Sid}.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ConferenceSid}", conference_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings.json`.\n\nRetrieve a list of recordings belonging to the call used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Conference Recording resources to read. (required)\n- `conference_sid: &'astr`: The Conference SID that identifies the conference associated with the recording to read. (required)\n- `date_created: Option<chrono::NaiveDate>`: The `date_created` value, specified as `YYYY-MM-DD`, of the resources to read. You can also specify inequality: `DateCreated<=YYYY-MM-DD` will return recordings generated at or before midnight on a given date, and `DateCreated>=YYYY-MM-DD` returns recordings generated at or after midnight on a date.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_conference_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListConferenceRecordingResponse = client\n        .default()\n        .list_conference_recording(\n            \"some-string\",\n            \"some-string\",\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_conference_recording<'a>(
        &'a self,
        account_sid: &'a str,
        conference_sid: &'a str,
        date_created: Option<chrono::NaiveDate>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListConferenceRecordingResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Recordings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ConferenceSid}", conference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = date_created {
            query_params.push(("DateCreated", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json`.\n\nFetch an instance of a connect-app\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the ConnectApp resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_connect_app() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConnectApp = client\n        .default()\n        .fetch_connect_app(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_connect_app<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountConnectApp, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json`.\n\nUpdate a connect-app with the specified parameters\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resources to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the ConnectApp resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_connect_app() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConnectApp = client\n        .default()\n        .update_connect_app(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateConnectAppRequest {\n                authorize_redirect_url: Some(\"https://example.com/foo/bar\".to_string()),\n                company_name: Some(\"some-string\".to_string()),\n                deauthorize_callback_method: Some(\n                    twilio_api::types::UpdateConnectAppRequestDeauthorizeCallbackMethod::Get,\n                ),\n                deauthorize_callback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n                homepage_url: Some(\"https://example.com/foo/bar\".to_string()),\n                permissions: Some(vec![twilio_api::types::ConnectAppEnumPermission::GetAll]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_connect_app<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateConnectAppRequest,
    ) -> Result<crate::types::ApiV2010AccountConnectApp, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json`.\n\nDelete an instance of a connect-app\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the ConnectApp resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_delete_connect_app() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_connect_app(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_connect_app<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/ConnectApps/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/ConnectApps.json`.\n\nRetrieve a list of connect-apps belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ConnectApp resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_connect_app() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListConnectAppResponse = client\n        .default()\n        .list_connect_app(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_connect_app<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListConnectAppResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/ConnectApps.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Addresses/{AddressSid}/DependentPhoneNumbers.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the DependentPhoneNumber resources to read. (required)\n- `address_sid: &'astr`: The SID of the Address resource associated with the phone number. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_dependent_phone_number() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListDependentPhoneNumberResponse = client\n        .default()\n        .list_dependent_phone_number(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_dependent_phone_number<'a>(
        &'a self,
        account_sid: &'a str,
        address_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListDependentPhoneNumberResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Addresses/{AddressSid}/DependentPhoneNumbers.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{AddressSid}", address_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json`.\n\nFetch an incoming-phone-number belonging to the account used to make the request.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the IncomingPhoneNumber resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_incoming_phone_number() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountIncomingPhoneNumber = client\n        .default()\n        .fetch_incoming_phone_number(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_incoming_phone_number<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountIncomingPhoneNumber, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json`.\n\nUpdate an incoming-phone-number instance.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resource to update.  For more information, see [Exchanging Numbers Between Subaccounts](https://www.twilio.com/docs/iam/api/subaccounts#exchanging-numbers). (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the IncomingPhoneNumber resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_incoming_phone_number() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountIncomingPhoneNumber = client\n        .default()\n        .update_incoming_phone_number(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateIncomingPhoneNumberRequest {\n                account_sid: Some(\"some-string\".to_string()),\n                api_version: Some(\"some-string\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n                sms_application_sid: Some(\"some-string\".to_string()),\n                sms_fallback_method: Some(\n                    twilio_api::types::UpdateIncomingPhoneNumberRequestSmsFallbackMethod::Delete,\n                ),\n                sms_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_method: Some(twilio_api::types::UpdateIncomingPhoneNumberRequestSmsMethod::Get),\n                sms_url: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::UpdateIncomingPhoneNumberRequestStatusCallbackMethod::Post,\n                ),\n                voice_application_sid: Some(\"some-string\".to_string()),\n                voice_caller_id_lookup: Some(true),\n                voice_fallback_method: Some(\n                    twilio_api::types::UpdateIncomingPhoneNumberRequestVoiceFallbackMethod::Post,\n                ),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(\n                    twilio_api::types::UpdateIncomingPhoneNumberRequestVoiceMethod::Delete,\n                ),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                emergency_status: Some(\n                    twilio_api::types::IncomingPhoneNumberEnumEmergencyStatus::Inactive,\n                ),\n                emergency_address_sid: Some(\"some-string\".to_string()),\n                trunk_sid: Some(\"some-string\".to_string()),\n                voice_receive_mode: Some(\n                    twilio_api::types::IncomingPhoneNumberEnumVoiceReceiveMode::Fax,\n                ),\n                identity_sid: Some(\"some-string\".to_string()),\n                address_sid: Some(\"some-string\".to_string()),\n                bundle_sid: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_incoming_phone_number<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateIncomingPhoneNumberRequest,
    ) -> Result<crate::types::ApiV2010AccountIncomingPhoneNumber, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json`.\n\nDelete a phone-numbers belonging to the account used to make the request.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the IncomingPhoneNumber resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_incoming_phone_number() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_incoming_phone_number(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_incoming_phone_number<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json`.\n\nRetrieve a list of incoming-phone-numbers belonging to the account used to make the request.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IncomingPhoneNumber resources to read. (required)\n- `beta: Option<bool>`: Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `friendly_name: Option<String>`: A string that identifies the IncomingPhoneNumber resources to read.\n- `origin: Option<String>`: Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `phone_number: crate::types::phone_number::PhoneNumber`: The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_incoming_phone_number() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListIncomingPhoneNumberResponse = client\n        .default()\n        .list_incoming_phone_number(\n            \"some-string\",\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_incoming_phone_number<'a>(
        &'a self,
        account_sid: &'a str,
        beta: Option<bool>,
        friendly_name: Option<String>,
        origin: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        phone_number: crate::types::phone_number::PhoneNumber,
    ) -> Result<crate::types::ListIncomingPhoneNumberResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = origin {
            query_params.push(("Origin", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = phone_number.0 {
            query_params.push(("PhoneNumber", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json`.\n\nPurchase a phone-number for the account.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_create_incoming_phone_number() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountIncomingPhoneNumber = client\n        .default()\n        .create_incoming_phone_number(\n            \"some-string\",\n            &twilio_api::types::CreateIncomingPhoneNumberRequest {\n                api_version: Some(\"some-string\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n                sms_application_sid: Some(\"some-string\".to_string()),\n                sms_fallback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberRequestSmsFallbackMethod::Put,\n                ),\n                sms_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_method: Some(twilio_api::types::CreateIncomingPhoneNumberRequestSmsMethod::Post),\n                sms_url: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberRequestStatusCallbackMethod::Post,\n                ),\n                voice_application_sid: Some(\"some-string\".to_string()),\n                voice_caller_id_lookup: Some(true),\n                voice_fallback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberRequestVoiceFallbackMethod::Get,\n                ),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(twilio_api::types::CreateIncomingPhoneNumberRequestVoiceMethod::Get),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                emergency_status: Some(\n                    twilio_api::types::IncomingPhoneNumberEnumEmergencyStatus::Active,\n                ),\n                emergency_address_sid: Some(\"some-string\".to_string()),\n                trunk_sid: Some(\"some-string\".to_string()),\n                identity_sid: Some(\"some-string\".to_string()),\n                address_sid: Some(\"some-string\".to_string()),\n                voice_receive_mode: Some(\n                    twilio_api::types::IncomingPhoneNumberEnumVoiceReceiveMode::Voice,\n                ),\n                bundle_sid: Some(\"some-string\".to_string()),\n                phone_number: twilio_api::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n                area_code: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_incoming_phone_number<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateIncomingPhoneNumberRequest,
    ) -> Result<crate::types::ApiV2010AccountIncomingPhoneNumber, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json`.\n\nFetch an instance of an Add-on installation currently assigned to this Number.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource to fetch. (required)\n- `resource_sid: &'astr`: The SID of the Phone Number to which the Add-on is assigned. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_incoming_phone_number_assigned_add_on() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOn =\n        client\n            .default()\n            .fetch_incoming_phone_number_assigned_add_on(\n                \"some-string\",\n                \"some-string\",\n                \"some-string\",\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_incoming_phone_number_assigned_add_on<'a>(
        &'a self,
        account_sid: &'a str,
        resource_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOn,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/\
                 AssignedAddOns/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ResourceSid}", resource_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{Sid}.json`.\n\nRemove the assignment of an Add-on installation from the Number specified.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to delete. (required)\n- `resource_sid: &'astr`: The SID of the Phone Number to which the Add-on is assigned. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_incoming_phone_number_assigned_add_on() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_incoming_phone_number_assigned_add_on(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_incoming_phone_number_assigned_add_on<'a>(
        &'a self,
        account_sid: &'a str,
        resource_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/\
                 AssignedAddOns/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ResourceSid}", resource_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json`.\n\nRetrieve a list of Add-on installations currently assigned to this Number.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `resource_sid: &'astr`: The SID of the Phone Number to which the Add-on is assigned. (required)\n\n```rust,no_run\nasync fn example_default_list_incoming_phone_number_assigned_add_on() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListIncomingPhoneNumberAssignedAddOnResponse = client\n        .default()\n        .list_incoming_phone_number_assigned_add_on(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_incoming_phone_number_assigned_add_on<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        resource_sid: &'a str,
    ) -> Result<
        crate::types::ListIncomingPhoneNumberAssignedAddOnResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/\
                 AssignedAddOns.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ResourceSid}", resource_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns.json`.\n\nAssign an Add-on installation to the Number specified.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `resource_sid: &'astr`: The SID of the Phone Number to assign the Add-on. (required)\n\n```rust,no_run\nasync fn example_default_create_incoming_phone_number_assigned_add_on() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOn =\n        client\n            .default()\n            .create_incoming_phone_number_assigned_add_on(\n                \"some-string\",\n                \"some-string\",\n                &twilio_api::types::CreateIncomingPhoneNumberAssignedAddOnRequest {\n                    installed_add_on_sid: \"some-string\".to_string(),\n                },\n            )\n            .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_incoming_phone_number_assigned_add_on<'a>(
        &'a self,
        account_sid: &'a str,
        resource_sid: &'a str,
        body: &crate::types::CreateIncomingPhoneNumberAssignedAddOnRequest,
    ) -> Result<
        crate::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOn,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/\
                 AssignedAddOns.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ResourceSid}", resource_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions/{Sid}.json`.\n\nFetch an instance of an Extension for the Assigned Add-on.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resource to fetch. (required)\n- `assigned_add_on_sid: &'astr`: The SID that uniquely identifies the assigned Add-on installation. (required)\n- `resource_sid: &'astr`: The SID of the Phone Number to which the Add-on is assigned. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_incoming_phone_number_assigned_add_on_extension(\n) -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOnIncomingPhoneNumberAssignedAddOnExtension = client . default () . fetch_incoming_phone_number_assigned_add_on_extension (\"some-string\" , \"some-string\" , \"some-string\" , \"some-string\" ,) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]    pub async fn fetch_incoming_phone_number_assigned_add_on_extension < 'a > (& 'a self , account_sid : & 'a str , assigned_add_on_sid : & 'a str , resource_sid : & 'a str , sid : & 'a str) -> Result < crate :: types :: ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOnIncomingPhoneNumberAssignedAddOnExtension , crate :: types :: error :: Error >{
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/\
                 AssignedAddOns/{AssignedAddOnSid}/Extensions/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{AssignedAddOnSid}", assigned_add_on_sid)
                    .replace("{ResourceSid}", resource_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/AssignedAddOns/{AssignedAddOnSid}/Extensions.json`.\n\nRetrieve a list of Extensions for the Assigned Add-on.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. (required)\n- `assigned_add_on_sid: &'astr`: The SID that uniquely identifies the assigned Add-on installation. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `resource_sid: &'astr`: The SID of the Phone Number to which the Add-on is assigned. (required)\n\n```rust,no_run\nasync fn example_default_list_incoming_phone_number_assigned_add_on_extension() -> anyhow::Result<()>\n{\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListIncomingPhoneNumberAssignedAddOnExtensionResponse = client\n        .default()\n        .list_incoming_phone_number_assigned_add_on_extension(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_incoming_phone_number_assigned_add_on_extension<'a>(
        &'a self,
        account_sid: &'a str,
        assigned_add_on_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        resource_sid: &'a str,
    ) -> Result<
        crate::types::ListIncomingPhoneNumberAssignedAddOnExtensionResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/{ResourceSid}/\
                 AssignedAddOns/{AssignedAddOnSid}/Extensions.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{AssignedAddOnSid}", assigned_add_on_sid)
                    .replace("{ResourceSid}", resource_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. (required)\n- `beta: Option<bool>`: Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `friendly_name: Option<String>`: A string that identifies the resources to read.\n- `origin: Option<String>`: Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `phone_number: crate::types::phone_number::PhoneNumber`: The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_incoming_phone_number_local() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListIncomingPhoneNumberLocalResponse = client\n        .default()\n        .list_incoming_phone_number_local(\n            \"some-string\",\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_incoming_phone_number_local<'a>(
        &'a self,
        account_sid: &'a str,
        beta: Option<bool>,
        friendly_name: Option<String>,
        origin: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        phone_number: crate::types::phone_number::PhoneNumber,
    ) -> Result<crate::types::ListIncomingPhoneNumberLocalResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = origin {
            query_params.push(("Origin", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = phone_number.0 {
            query_params.push(("PhoneNumber", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_create_incoming_phone_number_local() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberLocal = client\n        .default()\n        .create_incoming_phone_number_local(\n            \"some-string\",\n            &twilio_api::types::CreateIncomingPhoneNumberLocalRequest {\n                phone_number: twilio_api::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n                api_version: Some(\"some-string\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n                sms_application_sid: Some(\"some-string\".to_string()),\n                sms_fallback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberLocalRequestSmsFallbackMethod::Get,\n                ),\n                sms_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberLocalRequestSmsMethod::Post,\n                ),\n                sms_url: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberLocalRequestStatusCallbackMethod::Put,\n                ),\n                voice_application_sid: Some(\"some-string\".to_string()),\n                voice_caller_id_lookup: Some(true),\n                voice_fallback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberLocalRequestVoiceFallbackMethod::Post,\n                ),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberLocalRequestVoiceMethod::Put,\n                ),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                identity_sid: Some(\"some-string\".to_string()),\n                address_sid: Some(\"some-string\".to_string()),\n                emergency_status: Some(\n                    twilio_api::types::IncomingPhoneNumberLocalEnumEmergencyStatus::Inactive,\n                ),\n                emergency_address_sid: Some(\"some-string\".to_string()),\n                trunk_sid: Some(\"some-string\".to_string()),\n                voice_receive_mode: Some(\n                    twilio_api::types::IncomingPhoneNumberLocalEnumVoiceReceiveMode::Fax,\n                ),\n                bundle_sid: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_incoming_phone_number_local<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateIncomingPhoneNumberLocalRequest,
    ) -> Result<
        crate::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberLocal,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Local.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. (required)\n- `beta: Option<bool>`: Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `friendly_name: Option<String>`: A string that identifies the resources to read.\n- `origin: Option<String>`: Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `phone_number: crate::types::phone_number::PhoneNumber`: The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_incoming_phone_number_mobile() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListIncomingPhoneNumberMobileResponse = client\n        .default()\n        .list_incoming_phone_number_mobile(\n            \"some-string\",\n            Some(true),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_incoming_phone_number_mobile<'a>(
        &'a self,
        account_sid: &'a str,
        beta: Option<bool>,
        friendly_name: Option<String>,
        origin: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        phone_number: crate::types::phone_number::PhoneNumber,
    ) -> Result<crate::types::ListIncomingPhoneNumberMobileResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = origin {
            query_params.push(("Origin", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = phone_number.0 {
            query_params.push(("PhoneNumber", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_create_incoming_phone_number_mobile() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberMobile = client\n        .default()\n        .create_incoming_phone_number_mobile(\n            \"some-string\",\n            &twilio_api::types::CreateIncomingPhoneNumberMobileRequest {\n                phone_number: twilio_api::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n                api_version: Some(\"some-string\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n                sms_application_sid: Some(\"some-string\".to_string()),\n                sms_fallback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberMobileRequestSmsFallbackMethod::Post,\n                ),\n                sms_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberMobileRequestSmsMethod::Put,\n                ),\n                sms_url: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberMobileRequestStatusCallbackMethod::Put,\n                ),\n                voice_application_sid: Some(\"some-string\".to_string()),\n                voice_caller_id_lookup: Some(false),\n                voice_fallback_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberMobileRequestVoiceFallbackMethod::Patch,\n                ),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(\n                    twilio_api::types::CreateIncomingPhoneNumberMobileRequestVoiceMethod::Head,\n                ),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                identity_sid: Some(\"some-string\".to_string()),\n                address_sid: Some(\"some-string\".to_string()),\n                emergency_status: Some(\n                    twilio_api::types::IncomingPhoneNumberMobileEnumEmergencyStatus::Inactive,\n                ),\n                emergency_address_sid: Some(\"some-string\".to_string()),\n                trunk_sid: Some(\"some-string\".to_string()),\n                voice_receive_mode: Some(\n                    twilio_api::types::IncomingPhoneNumberMobileEnumVoiceReceiveMode::Voice,\n                ),\n                bundle_sid: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_incoming_phone_number_mobile<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateIncomingPhoneNumberMobileRequest,
    ) -> Result<
        crate::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberMobile,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/Mobile.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the resources to read. (required)\n- `beta: Option<bool>`: Whether to include phone numbers new to the Twilio platform. Can be: `true` or `false` and the default is `true`.\n- `friendly_name: Option<String>`: A string that identifies the resources to read.\n- `origin: Option<String>`: Whether to include phone numbers based on their origin. Can be: `twilio` or `hosted`. By default, phone numbers of all origin are included.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `phone_number: crate::types::phone_number::PhoneNumber`: The phone numbers of the IncomingPhoneNumber resources to read. You can specify partial numbers and use '*' as a wildcard for any digit.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_incoming_phone_number_toll_free() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListIncomingPhoneNumberTollFreeResponse = client\n        .default()\n        .list_incoming_phone_number_toll_free(\n            \"some-string\",\n            Some(false),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_incoming_phone_number_toll_free<'a>(
        &'a self,
        account_sid: &'a str,
        beta: Option<bool>,
        friendly_name: Option<String>,
        origin: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        phone_number: crate::types::phone_number::PhoneNumber,
    ) -> Result<crate::types::ListIncomingPhoneNumberTollFreeResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = beta {
            query_params.push(("Beta", format!("{p}")));
        }

        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = origin {
            query_params.push(("Origin", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = phone_number.0 {
            query_params.push(("PhoneNumber", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_create_incoming_phone_number_toll_free() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberTollFree = client . default () . create_incoming_phone_number_toll_free (\"some-string\" , & twilio_api::types::CreateIncomingPhoneNumberTollFreeRequest { phone_number : twilio_api::types::phone_number :: PhoneNumber :: from_str (\"+1555-555-5555\") ? , api_version : Some (\"some-string\" . to_string ()) , friendly_name : Some (\"some-string\" . to_string ()) , sms_application_sid : Some (\"some-string\" . to_string ()) , sms_fallback_method : Some (twilio_api::types::CreateIncomingPhoneNumberTollFreeRequestSmsFallbackMethod :: Head) , sms_fallback_url : Some (\"https://example.com/foo/bar\" . to_string ()) , sms_method : Some (twilio_api::types::CreateIncomingPhoneNumberTollFreeRequestSmsMethod :: Put) , sms_url : Some (\"https://example.com/foo/bar\" . to_string ()) , status_callback : Some (\"https://example.com/foo/bar\" . to_string ()) , status_callback_method : Some (twilio_api::types::CreateIncomingPhoneNumberTollFreeRequestStatusCallbackMethod :: Patch) , voice_application_sid : Some (\"some-string\" . to_string ()) , voice_caller_id_lookup : Some (true) , voice_fallback_method : Some (twilio_api::types::CreateIncomingPhoneNumberTollFreeRequestVoiceFallbackMethod :: Delete) , voice_fallback_url : Some (\"https://example.com/foo/bar\" . to_string ()) , voice_method : Some (twilio_api::types::CreateIncomingPhoneNumberTollFreeRequestVoiceMethod :: Patch) , voice_url : Some (\"https://example.com/foo/bar\" . to_string ()) , identity_sid : Some (\"some-string\" . to_string ()) , address_sid : Some (\"some-string\" . to_string ()) , emergency_status : Some (twilio_api::types::IncomingPhoneNumberTollFreeEnumEmergencyStatus :: Active) , emergency_address_sid : Some (\"some-string\" . to_string ()) , trunk_sid : Some (\"some-string\" . to_string ()) , voice_receive_mode : Some (twilio_api::types::IncomingPhoneNumberTollFreeEnumVoiceReceiveMode :: Voice) , bundle_sid : Some (\"some-string\" . to_string ()) }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_incoming_phone_number_toll_free<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateIncomingPhoneNumberTollFreeRequest,
    ) -> Result<
        crate::types::ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberTollFree,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/IncomingPhoneNumbers/TollFree.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Key resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountKey = client\n        .default()\n        .fetch_key(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_key<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountKey, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resources to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Key resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountKey = client\n        .default()\n        .update_key(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateKeyRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_key<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateKeyRequest,
    ) -> Result<crate::types::ApiV2010AccountKey, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Key resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_key(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_key<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Keys/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Keys.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Key resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListKeyResponse = client\n        .default()\n        .list_key(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_key<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListKeyResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Keys.json".replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Keys.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Key resource. (required)\n\n```rust,no_run\nasync fn example_default_create_new_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountNewKey = client\n        .default()\n        .create_new_key(\n            \"some-string\",\n            &twilio_api::types::CreateNewKeyRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_new_key<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateNewKeyRequest,
    ) -> Result<crate::types::ApiV2010AccountNewKey, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Keys.json".replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json`.\n\nFetch a single media instance belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Media resource(s) to fetch. (required)\n- `message_sid: &'astr`: The SID of the Message resource that this Media resource belongs to. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Media resource to fetch (required)\n\n```rust,no_run\nasync fn example_default_fetch_media() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountMessageMedia = client\n        .default()\n        .fetch_media(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_media<'a>(
        &'a self,
        account_sid: &'a str,
        message_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountMessageMedia, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{MessageSid}", message_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json`.\n\nDelete media from your account. Once delete, you will no longer be billed\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Media resource(s) to delete. (required)\n- `message_sid: &'astr`: The SID of the Message resource that this Media resource belongs to. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Media resource to delete (required)\n\n```rust,no_run\nasync fn example_default_delete_media() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_media(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_media<'a>(
        &'a self,
        account_sid: &'a str,
        message_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{MessageSid}", message_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media.json`.\n\nRetrieve a list of Media resources belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Media resource(s) to read. (required)\n- `date_created: Option<chrono::DateTime<chrono::Utc>>`: Only include media that was created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read media that was created on this date. You can also specify an inequality, such as `StartTime<=YYYY-MM-DD`, to read media that was created on or before midnight of this date, and `StartTime>=YYYY-MM-DD` to read media that was created on or after midnight of this date.\n- `message_sid: &'astr`: The SID of the Message resource that this Media resource belongs to. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_media() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListMediaResponse = client\n        .default()\n        .list_media(\n            \"some-string\",\n            Some(chrono::Utc::now()),\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_media<'a>(
        &'a self,
        account_sid: &'a str,
        date_created: Option<chrono::DateTime<chrono::Utc>>,
        message_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListMediaResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Media.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{MessageSid}", message_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = date_created {
            query_params.push(("DateCreated", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json`.\n\nFetch a specific member from the queue\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to fetch. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource(s) to fetch. (required)\n- `queue_sid: &'astr`: The SID of the Queue in which to find the members to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_member() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountQueueMember = client\n        .default()\n        .fetch_member(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_member<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        queue_sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountQueueMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{QueueSid}", queue_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json`.\n\nDequeue a member from a queue and have the member's call begin executing the TwiML document at that URL\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to update. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resource(s) to update. (required)\n- `queue_sid: &'astr`: The SID of the Queue in which to find the members to update. (required)\n\n```rust,no_run\nasync fn example_default_update_member() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountQueueMember = client\n        .default()\n        .update_member(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateMemberRequest {\n                url: \"https://example.com/foo/bar\".to_string(),\n                method: Some(twilio_api::types::UpdateMemberRequestMethod::Delete),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_member<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        queue_sid: &'a str,
        body: &crate::types::UpdateMemberRequest,
    ) -> Result<crate::types::ApiV2010AccountQueueMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members/{CallSid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{QueueSid}", queue_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members.json`.\n\nRetrieve the members of the queue\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Member resource(s) to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `queue_sid: &'astr`: The SID of the Queue in which to find the members (required)\n\n```rust,no_run\nasync fn example_default_list_member() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListMemberResponse = client\n        .default()\n        .list_member(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_member<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        queue_sid: &'a str,
    ) -> Result<crate::types::ListMemberResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues/{QueueSid}/Members.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{QueueSid}", queue_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Messages.json`.\n\nRetrieve a list of messages belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resources to read. (required)\n- `date_sent: Option<chrono::DateTime<chrono::Utc>>`: The date of the messages to show. Specify a date as `YYYY-MM-DD` in GMT to read only messages sent on this date. For example: `2009-07-06`. You can also specify an inequality, such as `DateSent<=YYYY-MM-DD`, to read messages sent on or before midnight on a date, and `DateSent>=YYYY-MM-DD` to read messages sent on or after midnight on a date.\n- `from: crate::types::phone_number::PhoneNumber`: Read messages sent from only this phone number or alphanumeric sender ID.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `to: crate::types::phone_number::PhoneNumber`: Read messages sent to only this phone number.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_message() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListMessageResponse = client\n        .default()\n        .list_message(\n            \"some-string\",\n            Some(chrono::Utc::now()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_message<'a>(
        &'a self,
        account_sid: &'a str,
        date_sent: Option<chrono::DateTime<chrono::Utc>>,
        from: crate::types::phone_number::PhoneNumber,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        to: crate::types::phone_number::PhoneNumber,
    ) -> Result<crate::types::ListMessageResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = date_sent {
            query_params.push(("DateSent", format!("{p}")));
        }

        if let Some(p) = from.0 {
            query_params.push(("From", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = to.0 {
            query_params.push(("To", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Messages.json`.\n\nSend a message from the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_create_message() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountMessage = client\n        .default()\n        .create_message(\n            \"some-string\",\n            &twilio_api::types::CreateMessageRequest {\n                to: twilio_api::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                application_sid: Some(\"some-string\".to_string()),\n                max_price: Some(3.14 as f64),\n                provide_feedback: Some(false),\n                attempt: Some(4 as i64),\n                validity_period: Some(4 as i64),\n                force_delivery: Some(true),\n                content_retention: Some(twilio_api::types::MessageEnumContentRetention::Retain),\n                address_retention: Some(twilio_api::types::MessageEnumAddressRetention::Retain),\n                smart_encoded: Some(false),\n                persistent_action: Some(vec![\"some-string\".to_string()]),\n                shorten_urls: Some(true),\n                schedule_type: Some(twilio_api::types::MessageEnumScheduleType::Fixed),\n                send_at: Some(chrono::Utc::now()),\n                send_as_mms: Some(false),\n                content_sid: Some(\"some-string\".to_string()),\n                content_variables: Some(\"some-string\".to_string()),\n                from: twilio_api::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n                messaging_service_sid: Some(\"some-string\".to_string()),\n                body: Some(\"some-string\".to_string()),\n                media_url: Some(vec![\"https://example.com/foo/bar\".to_string()]),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_message<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateMessageRequest,
    ) -> Result<crate::types::ApiV2010AccountMessage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json`.\n\nFetch a message belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Message resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_message() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountMessage = client\n        .default()\n        .fetch_message(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_message<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountMessage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json`.\n\nTo redact a message-body from a post-flight message record, post to the message instance resource with an empty body\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resources to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Message resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_message() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountMessage = client\n        .default()\n        .update_message(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateMessageRequest {\n                body: Some(\"some-string\".to_string()),\n                status: Some(twilio_api::types::MessageEnumUpdateStatus::Canceled),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_message<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateMessageRequest,
    ) -> Result<crate::types::ApiV2010AccountMessage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json`.\n\nDeletes a message record from your account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Message resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Message resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_message() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_message(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_message<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Feedback.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `message_sid: &'astr`: The SID of the Message resource for which the feedback was provided. (required)\n\n```rust,no_run\nasync fn example_default_create_message_feedback() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountMessageMessageFeedback = client\n        .default()\n        .create_message_feedback(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateMessageFeedbackRequest {\n                outcome: Some(twilio_api::types::MessageFeedbackEnumOutcome::Confirmed),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_message_feedback<'a>(
        &'a self,
        account_sid: &'a str,
        message_sid: &'a str,
        body: &crate::types::CreateMessageFeedbackRequest,
    ) -> Result<crate::types::ApiV2010AccountMessageMessageFeedback, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Messages/{MessageSid}/Feedback.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{MessageSid}", message_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SigningKeys.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`:  (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_signing_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSigningKeyResponse = client\n        .default()\n        .list_signing_key(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_signing_key<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSigningKeyResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SigningKeys.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SigningKeys.json`.\n\nCreate a new Signing Key for the account making the request.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will be responsible for the new Key resource. (required)\n\n```rust,no_run\nasync fn example_default_create_new_signing_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountNewSigningKey = client\n        .default()\n        .create_new_signing_key(\n            \"some-string\",\n            &twilio_api::types::CreateNewSigningKeyRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_new_signing_key<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateNewSigningKeyRequest,
    ) -> Result<crate::types::ApiV2010AccountNewSigningKey, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SigningKeys.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Notifications/{Sid}.json`.\n\nFetch a notification belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Notification resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Notification resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_notification() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountNotificationInstance = client\n        .default()\n        .fetch_notification(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_notification<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountNotificationInstance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Notifications/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Notifications.json`.\n\nRetrieve a list of notifications belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Notification resources to read. (required)\n- `log: Option<i64>`: Only read notifications of the specified log level. Can be:  `0` to read only ERROR notifications or `1` to read only WARNING notifications. By default, all notifications are read.\n- `message_date: Option<chrono::NaiveDate>`: Only show notifications for the specified date, formatted as `YYYY-MM-DD`. You can also specify an inequality, such as `<=YYYY-MM-DD` for messages logged at or before midnight on a date, or `>=YYYY-MM-DD` for messages logged at or after midnight on a date.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_notification() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListNotificationResponse = client\n        .default()\n        .list_notification(\n            \"some-string\",\n            Some(4 as i64),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_notification<'a>(
        &'a self,
        account_sid: &'a str,
        log: Option<i64>,
        message_date: Option<chrono::NaiveDate>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListNotificationResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Notifications.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = log {
            query_params.push(("Log", format!("{p}")));
        }

        if let Some(p) = message_date {
            query_params.push(("MessageDate", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json`.\n\nFetch an outgoing-caller-id belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the OutgoingCallerId resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_outgoing_caller_id() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountOutgoingCallerId = client\n        .default()\n        .fetch_outgoing_caller_id(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_outgoing_caller_id<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountOutgoingCallerId, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json`.\n\nUpdates the caller-id\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resources to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the OutgoingCallerId resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_outgoing_caller_id() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountOutgoingCallerId = client\n        .default()\n        .update_outgoing_caller_id(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateOutgoingCallerIdRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_outgoing_caller_id<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateOutgoingCallerIdRequest,
    ) -> Result<crate::types::ApiV2010AccountOutgoingCallerId, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json`.\n\nDelete the caller-id specified from the account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the OutgoingCallerId resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_outgoing_caller_id() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_outgoing_caller_id(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_outgoing_caller_id<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json`.\n\nRetrieve a list of outgoing-caller-ids belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the OutgoingCallerId resources to read. (required)\n- `friendly_name: Option<String>`: The string that identifies the OutgoingCallerId resources to read.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `phone_number: crate::types::phone_number::PhoneNumber`: The phone number of the OutgoingCallerId resources to read.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_list_outgoing_caller_id() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListOutgoingCallerIdResponse = client\n        .default()\n        .list_outgoing_caller_id(\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::phone_number::PhoneNumber::from_str(\n                \"+1555-555-5555\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_outgoing_caller_id<'a>(
        &'a self,
        account_sid: &'a str,
        friendly_name: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        phone_number: crate::types::phone_number::PhoneNumber,
    ) -> Result<crate::types::ListOutgoingCallerIdResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = phone_number.0 {
            query_params.push(("PhoneNumber", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for the new caller ID resource. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_default_create_validation_request() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountValidationRequest = client\n        .default()\n        .create_validation_request(\n            \"some-string\",\n            &twilio_api::types::CreateValidationRequestRequest {\n                phone_number: twilio_api::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n                friendly_name: Some(\"some-string\".to_string()),\n                call_delay: Some(4 as i64),\n                extension: Some(\"some-string\".to_string()),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateValidationRequestRequestStatusCallbackMethod::Patch,\n                ),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_validation_request<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateValidationRequestRequest,
    ) -> Result<crate::types::ApiV2010AccountValidationRequest, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/OutgoingCallerIds.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json`.\n\nFetch an instance of a participant\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resource to fetch. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID or label of the participant to fetch. Non URL safe characters in a label must be percent encoded, for example, a space character is represented as %20. (required)\n- `conference_sid: &'astr`: The SID of the conference with the participant to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_participant() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConferenceParticipant = client\n        .default()\n        .fetch_participant(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_participant<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        conference_sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountConferenceParticipant, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/\
                 {CallSid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{ConferenceSid}", conference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json`.\n\nUpdate the properties of the participant\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resources to update. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID or label of the participant to update. Non URL safe characters in a label must be percent encoded, for example, a space character is represented as %20. (required)\n- `conference_sid: &'astr`: The SID of the conference with the participant to update. (required)\n\n```rust,no_run\nasync fn example_default_update_participant() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConferenceParticipant = client\n        .default()\n        .update_participant(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateParticipantRequest {\n                muted: Some(false),\n                hold: Some(true),\n                hold_url: Some(\"https://example.com/foo/bar\".to_string()),\n                hold_method: Some(twilio_api::types::HoldMethod::Head),\n                announce_url: Some(\"https://example.com/foo/bar\".to_string()),\n                announce_method: Some(twilio_api::types::UpdateParticipantRequestAnnounceMethod::Put),\n                wait_url: Some(\"https://example.com/foo/bar\".to_string()),\n                wait_method: Some(twilio_api::types::WaitMethod::Patch),\n                beep_on_exit: Some(true),\n                end_conference_on_exit: Some(false),\n                coaching: Some(true),\n                call_sid_to_coach: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_participant<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        conference_sid: &'a str,
        body: &crate::types::UpdateParticipantRequest,
    ) -> Result<crate::types::ApiV2010AccountConferenceParticipant, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/\
                 {CallSid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{ConferenceSid}", conference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/{CallSid}.json`.\n\nKick a participant from a given conference\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resources to delete. (required)\n- `call_sid: &'astr`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID or label of the participant to delete. Non URL safe characters in a label must be percent encoded, for example, a space character is represented as %20. (required)\n- `conference_sid: &'astr`: The SID of the conference with the participants to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_participant() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_participant(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_participant<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        conference_sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants/\
                 {CallSid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{ConferenceSid}", conference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json`.\n\nRetrieve a list of participants belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Participant resources to read. (required)\n- `coaching: Option<bool>`: Whether to return only participants who are coaching another call. Can be: `true` or `false`.\n- `conference_sid: &'astr`: The SID of the conference with the participants to read. (required)\n- `hold: Option<bool>`: Whether to return only participants that are on hold. Can be: `true` or `false`.\n- `muted: Option<bool>`: Whether to return only participants that are muted. Can be: `true` or `false`.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_participant() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListParticipantResponse = client\n        .default()\n        .list_participant(\n            \"some-string\",\n            Some(true),\n            \"some-string\",\n            Some(false),\n            Some(true),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_participant<'a>(
        &'a self,
        account_sid: &'a str,
        coaching: Option<bool>,
        conference_sid: &'a str,
        hold: Option<bool>,
        muted: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListParticipantResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ConferenceSid}", conference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = coaching {
            query_params.push(("Coaching", format!("{p}")));
        }

        if let Some(p) = hold {
            query_params.push(("Hold", format!("{p}")));
        }

        if let Some(p) = muted {
            query_params.push(("Muted", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `conference_sid: &'astr`: The SID of the participant's conference. (required)\n\n```rust,no_run\nasync fn example_default_create_participant() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountConferenceParticipant = client\n        .default()\n        .create_participant(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateParticipantRequest {\n                from: \"some-string\".to_string(),\n                to: \"some-string\".to_string(),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateParticipantRequestStatusCallbackMethod::Get,\n                ),\n                status_callback_event: Some(vec![\"some-string\".to_string()]),\n                label: Some(\"some-string\".to_string()),\n                timeout: Some(4 as i64),\n                record: Some(false),\n                muted: Some(true),\n                beep: Some(\"some-string\".to_string()),\n                start_conference_on_enter: Some(false),\n                end_conference_on_exit: Some(false),\n                wait_url: Some(\"https://example.com/foo/bar\".to_string()),\n                wait_method: Some(twilio_api::types::WaitMethod::Put),\n                early_media: Some(false),\n                max_participants: Some(4 as i64),\n                conference_record: Some(\"some-string\".to_string()),\n                conference_trim: Some(\"some-string\".to_string()),\n                conference_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                conference_status_callback_method: Some(\n                    twilio_api::types::ConferenceStatusCallbackMethod::Delete,\n                ),\n                conference_status_callback_event: Some(vec![\"some-string\".to_string()]),\n                recording_channels: Some(\"some-string\".to_string()),\n                recording_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                recording_status_callback_method: Some(\n                    twilio_api::types::CreateParticipantRequestRecordingStatusCallbackMethod::Head,\n                ),\n                sip_auth_username: Some(\"some-string\".to_string()),\n                sip_auth_password: Some(\"some-string\".to_string()),\n                region: Some(\"some-string\".to_string()),\n                conference_recording_status_callback: Some(\n                    \"https://example.com/foo/bar\".to_string(),\n                ),\n                conference_recording_status_callback_method: Some(\n                    twilio_api::types::ConferenceRecordingStatusCallbackMethod::Patch,\n                ),\n                recording_status_callback_event: Some(vec![\"some-string\".to_string()]),\n                conference_recording_status_callback_event: Some(vec![\"some-string\".to_string()]),\n                coaching: Some(false),\n                call_sid_to_coach: Some(\"some-string\".to_string()),\n                jitter_buffer_size: Some(\"some-string\".to_string()),\n                byoc: Some(\"some-string\".to_string()),\n                caller_id: Some(\"some-string\".to_string()),\n                call_reason: Some(\"some-string\".to_string()),\n                recording_track: Some(\"some-string\".to_string()),\n                time_limit: Some(4 as i64),\n                machine_detection: Some(\"some-string\".to_string()),\n                machine_detection_timeout: Some(4 as i64),\n                machine_detection_speech_threshold: Some(4 as i64),\n                machine_detection_speech_end_threshold: Some(4 as i64),\n                machine_detection_silence_timeout: Some(4 as i64),\n                amd_status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                amd_status_callback_method: Some(twilio_api::types::AmdStatusCallbackMethod::Get),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_participant<'a>(
        &'a self,
        account_sid: &'a str,
        conference_sid: &'a str,
        body: &crate::types::CreateParticipantRequest,
    ) -> Result<crate::types::ApiV2010AccountConferenceParticipant, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Conferences/{ConferenceSid}/Participants.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ConferenceSid}", conference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments.json`.\n\ncreate an instance of payments. This will start a new payments session\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `call_sid: &'astr`: The SID of the call that will create the resource. Call leg associated with this sid is expected to provide payment information thru DTMF. (required)\n\n```rust,no_run\nasync fn example_default_create_payments() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallPayments = client\n        .default()\n        .create_payments(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreatePaymentsRequest {\n                idempotency_key: \"some-string\".to_string(),\n                status_callback: \"https://example.com/foo/bar\".to_string(),\n                bank_account_type: Some(\n                    twilio_api::types::PaymentsEnumBankAccountType::CommercialChecking,\n                ),\n                charge_amount: Some(3.14 as f64),\n                currency: Some(\"some-string\".to_string()),\n                description: Some(\"some-string\".to_string()),\n                input: Some(\"some-string\".to_string()),\n                min_postal_code_length: Some(4 as i64),\n                parameter: Some(serde_json::Value::String(\"some-string\".to_string())),\n                payment_connector: Some(\"some-string\".to_string()),\n                payment_method: Some(twilio_api::types::PaymentsEnumPaymentMethod::CreditCard),\n                postal_code: Some(true),\n                security_code: Some(false),\n                timeout: Some(4 as i64),\n                token_type: Some(twilio_api::types::PaymentsEnumTokenType::Reusable),\n                valid_card_types: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_payments<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        body: &crate::types::CreatePaymentsRequest,
    ) -> Result<crate::types::ApiV2010AccountCallPayments, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments/{Sid}.json`.\n\nupdate an instance of payments with different phases of payment flows.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will update the resource. (required)\n- `call_sid: &'astr`: The SID of the call that will update the resource. This should be the same call sid that was used to create payments resource. (required)\n- `sid: &'astr`: The SID of Payments session that needs to be updated. (required)\n\n```rust,no_run\nasync fn example_default_update_payments() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallPayments = client\n        .default()\n        .update_payments(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdatePaymentsRequest {\n                idempotency_key: \"some-string\".to_string(),\n                status_callback: \"https://example.com/foo/bar\".to_string(),\n                capture: Some(twilio_api::types::PaymentsEnumCapture::PostalCode),\n                status: Some(twilio_api::types::PaymentsEnumStatus::Complete),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_payments<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdatePaymentsRequest,
    ) -> Result<crate::types::ApiV2010AccountCallPayments, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Payments/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json`.\n\nFetch an instance of a queue identified by the QueueSid\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Queue resource to fetch (required)\n\n```rust,no_run\nasync fn example_default_fetch_queue() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountQueue = client\n        .default()\n        .fetch_queue(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_queue<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountQueue, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json`.\n\nUpdate the queue with the new parameters\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resource to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Queue resource to update (required)\n\n```rust,no_run\nasync fn example_default_update_queue() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountQueue = client\n        .default()\n        .update_queue(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateQueueRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n                max_size: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_queue<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateQueueRequest,
    ) -> Result<crate::types::ApiV2010AccountQueue, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json`.\n\nRemove an empty queue\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resource to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Queue resource to delete (required)\n\n```rust,no_run\nasync fn example_default_delete_queue() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_queue(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_queue<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Queues.json`.\n\nRetrieve a list of queues belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Queue resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_queue() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListQueueResponse = client\n        .default()\n        .list_queue(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_queue<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListQueueResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Queues.json`.\n\nCreate a queue\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nasync fn example_default_create_queue() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountQueue = client\n        .default()\n        .create_queue(\n            \"some-string\",\n            &twilio_api::types::CreateQueueRequest {\n                friendly_name: \"some-string\".to_string(),\n                max_size: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_queue<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateQueueRequest,
    ) -> Result<crate::types::ApiV2010AccountQueue, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Queues.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json`.\n\nFetch an instance of a recording\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resource to fetch. (required)\n- `include_soft_deleted: Option<bool>`: A boolean parameter indicating whether to retrieve soft deleted recordings or not. Recordings metadata are kept after deletion for a retention period of 40 days.\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountRecording = client\n        .default()\n        .fetch_recording(\"some-string\", Some(false), \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_recording<'a>(
        &'a self,
        account_sid: &'a str,
        include_soft_deleted: Option<bool>,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountRecording, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = include_soft_deleted {
            query_params.push(("IncludeSoftDeleted", format!("{p}")));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json`.\n\nDelete a recording from your account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_recording(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_recording<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings.json`.\n\nRetrieve a list of recordings belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording resources to read. (required)\n- `call_sid: Option<String>`: The [Call](https://www.twilio.com/docs/voice/api/call-resource) SID of the resources to read.\n- `conference_sid: Option<String>`: The Conference SID that identifies the conference associated with the recording to read.\n- `date_created: Option<chrono::DateTime<chrono::Utc>>`: Only include recordings that were created on this date. Specify a date as `YYYY-MM-DD` in GMT, for example: `2009-07-06`, to read recordings that were created on this date. You can also specify an inequality, such as `DateCreated<=YYYY-MM-DD`, to read recordings that were created on or before midnight of this date, and `DateCreated>=YYYY-MM-DD` to read recordings that were created on or after midnight of this date.\n- `include_soft_deleted: Option<bool>`: A boolean parameter indicating whether to retrieve soft deleted recordings or not. Recordings metadata are kept after deletion for a retention period of 40 days.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_recording() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListRecordingResponse = client\n        .default()\n        .list_recording(\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now()),\n            Some(true),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_recording<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: Option<String>,
        conference_sid: Option<String>,
        date_created: Option<chrono::DateTime<chrono::Utc>>,
        include_soft_deleted: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListRecordingResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = call_sid {
            query_params.push(("CallSid", p));
        }

        if let Some(p) = conference_sid {
            query_params.push(("ConferenceSid", p));
        }

        if let Some(p) = date_created {
            query_params.push(("DateCreated", format!("{p}")));
        }

        if let Some(p) = include_soft_deleted {
            query_params.push(("IncludeSoftDeleted", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.json`.\n\nFetch an instance of an AddOnResult\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult resource to fetch. (required)\n- `reference_sid: &'astr`: The SID of the recording to which the result to fetch belongs. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording AddOnResult resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_recording_add_on_result() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountRecordingRecordingAddOnResult = client\n        .default()\n        .fetch_recording_add_on_result(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_recording_add_on_result<'a>(
        &'a self,
        account_sid: &'a str,
        reference_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountRecordingRecordingAddOnResult,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ReferenceSid}", reference_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.json`.\n\nDelete a result and purge all associated Payloads\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult resources to delete. (required)\n- `reference_sid: &'astr`: The SID of the recording to which the result to delete belongs. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording AddOnResult resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_recording_add_on_result() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_recording_add_on_result(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_recording_add_on_result<'a>(
        &'a self,
        account_sid: &'a str,
        reference_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{Sid}.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ReferenceSid}", reference_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults.json`.\n\nRetrieve a list of results belonging to the recording\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `reference_sid: &'astr`: The SID of the recording to which the result to read belongs. (required)\n\n```rust,no_run\nasync fn example_default_list_recording_add_on_result() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListRecordingAddOnResultResponse = client\n        .default()\n        .list_recording_add_on_result(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_recording_add_on_result<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        reference_sid: &'a str,
    ) -> Result<crate::types::ListRecordingAddOnResultResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{ReferenceSid}", reference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json`.\n\nFetch an instance of a result payload\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resource to fetch. (required)\n- `add_on_result_sid: &'astr`: The SID of the AddOnResult to which the payload to fetch belongs. (required)\n- `reference_sid: &'astr`: The SID of the recording to which the AddOnResult resource that contains the payload to fetch belongs. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording AddOnResult Payload resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_recording_add_on_result_payload() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountRecordingRecordingAddOnResultRecordingAddOnResultPayload = client . default () . fetch_recording_add_on_result_payload (\"some-string\" , \"some-string\" , \"some-string\" , \"some-string\" ,) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_recording_add_on_result_payload<'a>(
        &'a self,
        account_sid: &'a str,
        add_on_result_sid: &'a str,
        reference_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountRecordingRecordingAddOnResultRecordingAddOnResultPayload,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/\
                 {AddOnResultSid}/Payloads/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{AddOnResultSid}", add_on_result_sid)
                    .replace("{ReferenceSid}", reference_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads/{Sid}.json`.\n\nDelete a payload from the result along with all associated Data\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resources to delete. (required)\n- `add_on_result_sid: &'astr`: The SID of the AddOnResult to which the payloads to delete belongs. (required)\n- `reference_sid: &'astr`: The SID of the recording to which the AddOnResult resource that contains the payloads to delete belongs. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Recording AddOnResult Payload resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_recording_add_on_result_payload() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_recording_add_on_result_payload(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_recording_add_on_result_payload<'a>(
        &'a self,
        account_sid: &'a str,
        add_on_result_sid: &'a str,
        reference_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/\
                 {AddOnResultSid}/Payloads/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{AddOnResultSid}", add_on_result_sid)
                    .replace("{ReferenceSid}", reference_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/{AddOnResultSid}/Payloads.json`.\n\nRetrieve a list of payloads belonging to the AddOnResult\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Recording AddOnResult Payload resources to read. (required)\n- `add_on_result_sid: &'astr`: The SID of the AddOnResult to which the payloads to read belongs. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `reference_sid: &'astr`: The SID of the recording to which the AddOnResult resource that contains the payloads to read belongs. (required)\n\n```rust,no_run\nasync fn example_default_list_recording_add_on_result_payload() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListRecordingAddOnResultPayloadResponse = client\n        .default()\n        .list_recording_add_on_result_payload(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_recording_add_on_result_payload<'a>(
        &'a self,
        account_sid: &'a str,
        add_on_result_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        reference_sid: &'a str,
    ) -> Result<crate::types::ListRecordingAddOnResultPayloadResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{ReferenceSid}/AddOnResults/\
                 {AddOnResultSid}/Payloads.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{AddOnResultSid}", add_on_result_sid)
                    .replace("{ReferenceSid}", reference_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch. (required)\n- `recording_sid: &'astr`: The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Transcription resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_recording_transcription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountRecordingRecordingTranscription = client\n        .default()\n        .fetch_recording_transcription(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_recording_transcription<'a>(
        &'a self,
        account_sid: &'a str,
        recording_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountRecordingRecordingTranscription,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{RecordingSid}", recording_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete. (required)\n- `recording_sid: &'astr`: The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcription to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Transcription resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_recording_transcription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_recording_transcription(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_recording_transcription<'a>(
        &'a self,
        account_sid: &'a str,
        recording_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions/{Sid}.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{RecordingSid}", recording_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `recording_sid: &'astr`: The SID of the [Recording](https://www.twilio.com/docs/voice/api/recording) that created the transcriptions to read. (required)\n\n```rust,no_run\nasync fn example_default_list_recording_transcription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListRecordingTranscriptionResponse = client\n        .default()\n        .list_recording_transcription(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_recording_transcription<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        recording_sid: &'a str,
    ) -> Result<crate::types::ListRecordingTranscriptionResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Recordings/{RecordingSid}/Transcriptions.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{RecordingSid}", recording_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json`.\n\nFetch an instance of a short code\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the ShortCode resource to fetch (required)\n\n```rust,no_run\nasync fn example_default_fetch_short_code() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountShortCode = client\n        .default()\n        .fetch_short_code(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_short_code<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountShortCode, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json`.\n\nUpdate a short code with the following parameters\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the ShortCode resource to update (required)\n\n```rust,no_run\nasync fn example_default_update_short_code() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountShortCode = client\n        .default()\n        .update_short_code(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateShortCodeRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n                api_version: Some(\"some-string\".to_string()),\n                sms_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_method: Some(twilio_api::types::UpdateShortCodeRequestSmsMethod::Delete),\n                sms_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sms_fallback_method: Some(\n                    twilio_api::types::UpdateShortCodeRequestSmsFallbackMethod::Get,\n                ),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_short_code<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateShortCodeRequest,
    ) -> Result<crate::types::ApiV2010AccountShortCode, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes.json`.\n\nRetrieve a list of short-codes belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the ShortCode resource(s) to read. (required)\n- `friendly_name: Option<String>`: The string that identifies the ShortCode resources to read.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `short_code: Option<String>`: Only show the ShortCode resources that match this pattern. You can specify partial numbers and use '*' as a wildcard for any digit.\n\n```rust,no_run\nasync fn example_default_list_short_code() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListShortCodeResponse = client\n        .default()\n        .list_short_code(\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_short_code<'a>(
        &'a self,
        account_sid: &'a str,
        friendly_name: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        short_code: Option<String>,
    ) -> Result<crate::types::ListShortCodeResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SMS/ShortCodes.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = friendly_name {
            query_params.push(("FriendlyName", p));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = short_code {
            query_params.push(("ShortCode", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`:  (required)\n- `sid: &'astr`:  (required)\n\n```rust,no_run\nasync fn example_default_fetch_signing_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSigningKey = client\n        .default()\n        .fetch_signing_key(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_signing_key<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountSigningKey, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`:  (required)\n- `sid: &'astr`:  (required)\n\n```rust,no_run\nasync fn example_default_update_signing_key() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSigningKey = client\n        .default()\n        .update_signing_key(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateSigningKeyRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_signing_key<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateSigningKeyRequest,
    ) -> Result<crate::types::ApiV2010AccountSigningKey, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to \
             `/2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json`.\n\n\n\n**Parameters:**\n\\
             n- `account_sid: &'astr`:  (required)\n- `sid: &'astr`:  \
             (required)\n\n```rust,no_run\nasync fn example_default_delete_signing_key() -> \
             anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    \
             client\n        .default()\n        .delete_signing_key(\"some-string\", \
             \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_signing_key<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SigningKeys/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings.json`.\n\nRetrieve a list of credential list mappings belonging to the domain used in the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to read. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_auth_calls_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipAuthCallsCredentialListMappingResponse = client\n        .default()\n        .list_sip_auth_calls_credential_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_auth_calls_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<
        crate::types::ListSipAuthCallsCredentialListMappingResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 CredentialListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings.json`.\n\nCreate a new credential list mapping resource\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that will contain the new resource. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_auth_calls_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsCredentialListMapping = client . default () . create_sip_auth_calls_credential_list_mapping (\"some-string\" , \"some-string\" , & twilio_api::types::CreateSipAuthCallsCredentialListMappingRequest { credential_list_sid : \"some-string\" . to_string () }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]    pub async fn create_sip_auth_calls_credential_list_mapping < 'a > (& 'a self , account_sid : & 'a str , domain_sid : & 'a str , body : & crate :: types :: CreateSipAuthCallsCredentialListMappingRequest) -> Result < crate :: types :: ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsCredentialListMapping , crate :: types :: error :: Error >{
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 CredentialListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings/{Sid}.json`.\n\nFetch a specific instance of a credential list mapping\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resource to fetch. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the CredentialListMapping resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_auth_calls_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsCredentialListMapping = client . default () . fetch_sip_auth_calls_credential_list_mapping (\"some-string\" , \"some-string\" , \"some-string\" ,) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]    pub async fn fetch_sip_auth_calls_credential_list_mapping < 'a > (& 'a self , account_sid : & 'a str , domain_sid : & 'a str , sid : & 'a str) -> Result < crate :: types :: ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsCredentialListMapping , crate :: types :: error :: Error >{
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 CredentialListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/CredentialListMappings/{Sid}.json`.\n\nDelete a credential list mapping from the requested domain\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to delete. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resource to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the CredentialListMapping resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_auth_calls_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_auth_calls_credential_list_mapping(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_auth_calls_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 CredentialListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings.json`.\n\nRetrieve a list of IP Access Control List mappings belonging to the domain used in the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IpAccessControlListMapping resources to read. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_auth_calls_ip_access_control_list_mapping() -> anyhow::Result<()>\n{\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipAuthCallsIpAccessControlListMappingResponse = client\n        .default()\n        .list_sip_auth_calls_ip_access_control_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_auth_calls_ip_access_control_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<
        crate::types::ListSipAuthCallsIpAccessControlListMappingResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 IpAccessControlListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings.json`.\n\nCreate a new IP Access Control List mapping\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that will contain the new resource. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_auth_calls_ip_access_control_list_mapping() -> anyhow::Result<()>\n{\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsIpAccessControlListMapping = client . default () . create_sip_auth_calls_ip_access_control_list_mapping (\"some-string\" , \"some-string\" , & twilio_api::types::CreateSipAuthCallsIpAccessControlListMappingRequest { ip_access_control_list_sid : \"some-string\" . to_string () }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]    pub async fn create_sip_auth_calls_ip_access_control_list_mapping < 'a > (& 'a self , account_sid : & 'a str , domain_sid : & 'a str , body : & crate :: types :: CreateSipAuthCallsIpAccessControlListMappingRequest) -> Result < crate :: types :: ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsIpAccessControlListMapping , crate :: types :: error :: Error >{
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 IpAccessControlListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings/{Sid}.json`.\n\nFetch a specific instance of an IP Access Control List mapping\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IpAccessControlListMapping resource to fetch. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the IpAccessControlListMapping resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_auth_calls_ip_access_control_list_mapping() -> anyhow::Result<()>\n{\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsIpAccessControlListMapping = client . default () . fetch_sip_auth_calls_ip_access_control_list_mapping (\"some-string\" , \"some-string\" , \"some-string\" ,) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]    pub async fn fetch_sip_auth_calls_ip_access_control_list_mapping < 'a > (& 'a self , account_sid : & 'a str , domain_sid : & 'a str , sid : & 'a str) -> Result < crate :: types :: ApiV2010AccountSipSipDomainSipAuthSipAuthCallsSipAuthCallsIpAccessControlListMapping , crate :: types :: error :: Error >{
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 IpAccessControlListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/IpAccessControlListMappings/{Sid}.json`.\n\nDelete an IP Access Control List mapping from the requested domain\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the IpAccessControlListMapping resources to delete. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the IpAccessControlListMapping resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_auth_calls_ip_access_control_list_mapping() -> anyhow::Result<()>\n{\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_auth_calls_ip_access_control_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_auth_calls_ip_access_control_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Calls/\
                 IpAccessControlListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json`.\n\nRetrieve a list of credential list mappings belonging to the domain used in the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to read. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_auth_registrations_credential_list_mapping() -> anyhow::Result<()>\n{\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipAuthRegistrationsCredentialListMappingResponse = client\n        .default()\n        .list_sip_auth_registrations_credential_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_auth_registrations_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<
        crate::types::ListSipAuthRegistrationsCredentialListMappingResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/\
                 CredentialListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings.json`.\n\nCreate a new credential list mapping resource\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that will contain the new resource. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_auth_registrations_credential_list_mapping(\n) -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountSipSipDomainSipAuthSipAuthRegistrationsSipAuthRegistrationsCredentialListMapping = client . default () . create_sip_auth_registrations_credential_list_mapping (\"some-string\" , \"some-string\" , & twilio_api::types::CreateSipAuthRegistrationsCredentialListMappingRequest { credential_list_sid : \"some-string\" . to_string () }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]    pub async fn create_sip_auth_registrations_credential_list_mapping < 'a > (& 'a self , account_sid : & 'a str , domain_sid : & 'a str , body : & crate :: types :: CreateSipAuthRegistrationsCredentialListMappingRequest) -> Result < crate :: types :: ApiV2010AccountSipSipDomainSipAuthSipAuthRegistrationsSipAuthRegistrationsCredentialListMapping , crate :: types :: error :: Error >{
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/\
                 CredentialListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json`.\n\nFetch a specific instance of a credential list mapping\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resource to fetch. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the CredentialListMapping resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_auth_registrations_credential_list_mapping() -> anyhow::Result<()>\n{\n    let client = twilio_api::Client::new_from_env();\n    let result : twilio_api::types::ApiV2010AccountSipSipDomainSipAuthSipAuthRegistrationsSipAuthRegistrationsCredentialListMapping = client . default () . fetch_sip_auth_registrations_credential_list_mapping (\"some-string\" , \"some-string\" , \"some-string\" ,) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]    pub async fn fetch_sip_auth_registrations_credential_list_mapping < 'a > (& 'a self , account_sid : & 'a str , domain_sid : & 'a str , sid : & 'a str) -> Result < crate :: types :: ApiV2010AccountSipSipDomainSipAuthSipAuthRegistrationsSipAuthRegistrationsCredentialListMapping , crate :: types :: error :: Error >{
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/\
                 CredentialListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/CredentialListMappings/{Sid}.json`.\n\nDelete a credential list mapping from the requested domain\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the CredentialListMapping resources to delete. (required)\n- `domain_sid: &'astr`: The SID of the SIP domain that contains the resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the CredentialListMapping resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_auth_registrations_credential_list_mapping(\n) -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_auth_registrations_credential_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_auth_registrations_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/Auth/Registrations/\
                 CredentialListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials.json`.\n\nRetrieve a list of credentials.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `credential_list_sid: &'astr`: The unique id that identifies the credential list that contains the desired credentials. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_credential() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipCredentialResponse = client\n        .default()\n        .list_sip_credential(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_credential<'a>(
        &'a self,
        account_sid: &'a str,
        credential_list_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSipCredentialResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
                 Credentials.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CredentialListSid}", credential_list_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to \
             `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
             Credentials.json`.\n\nCreate a new credential resource.\n\n**Parameters:**\n\n- \
             `account_sid: &'astr`: The unique id of the Account that is responsible for this \
             resource. (required)\n- `credential_list_sid: &'astr`: The unique id that identifies \
             the credential list to include the created credential. \
             (required)\n\n```rust,no_run\nasync fn example_default_create_sip_credential() -> \
             anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let \
             result: twilio_api::types::ApiV2010AccountSipSipCredentialListSipCredential = \
             client\n        .default()\n        .create_sip_credential(\n            \
             \"some-string\",\n            \"some-string\",\n            \
             &twilio_api::types::CreateSipCredentialRequest {\n                username: \
             \"some-string\".to_string(),\n                password: \
             \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_sip_credential<'a>(
        &'a self,
        account_sid: &'a str,
        credential_list_sid: &'a str,
        body: &crate::types::CreateSipCredentialRequest,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipCredentialListSipCredential,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
                 Credentials.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CredentialListSid}", credential_list_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to \
             `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
             Credentials/{Sid}.json`.\n\nFetch a single credential.\n\n**Parameters:**\n\n- \
             `account_sid: &'astr`: The unique id of the Account that is responsible for this \
             resource. (required)\n- `credential_list_sid: &'astr`: The unique id that identifies \
             the credential list that contains the desired credential. (required)\n- `sid: \
             &'astr`: The unique id that identifies the resource to fetch. \
             (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_credential() -> \
             anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let \
             result: twilio_api::types::ApiV2010AccountSipSipCredentialListSipCredential = \
             client\n        .default()\n        .fetch_sip_credential(\"some-string\", \
             \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_sip_credential<'a>(
        &'a self,
        account_sid: &'a str,
        credential_list_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipCredentialListSipCredential,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
                 Credentials/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CredentialListSid}", credential_list_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/Credentials/{Sid}.json`.\n\nUpdate a credential resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `credential_list_sid: &'astr`: The unique id that identifies the credential list that includes this credential. (required)\n- `sid: &'astr`: The unique id that identifies the resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_sip_credential() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipCredentialListSipCredential = client\n        .default()\n        .update_sip_credential(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateSipCredentialRequest {\n                password: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_sip_credential<'a>(
        &'a self,
        account_sid: &'a str,
        credential_list_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateSipCredentialRequest,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipCredentialListSipCredential,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
                 Credentials/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CredentialListSid}", credential_list_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to \
             `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
             Credentials/{Sid}.json`.\n\nDelete a credential resource.\n\n**Parameters:**\n\n- \
             `account_sid: &'astr`: The unique id of the Account that is responsible for this \
             resource. (required)\n- `credential_list_sid: &'astr`: The unique id that identifies \
             the credential list that contains the desired credentials. (required)\n- `sid: \
             &'astr`: The unique id that identifies the resource to delete. \
             (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_credential() -> \
             anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    \
             client\n        .default()\n        .delete_sip_credential(\"some-string\", \
             \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_credential<'a>(
        &'a self,
        account_sid: &'a str,
        credential_list_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{CredentialListSid}/\
                 Credentials/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CredentialListSid}", credential_list_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json`.\n\nGet All Credential Lists\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_credential_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipCredentialListResponse = client\n        .default()\n        .list_sip_credential_list(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_credential_list<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSipCredentialListResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json`.\n\nCreate a Credential List\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_credential_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipCredentialList = client\n        .default()\n        .create_sip_credential_list(\n            \"some-string\",\n            &twilio_api::types::CreateSipCredentialListRequest {\n                friendly_name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_sip_credential_list<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateSipCredentialListRequest,
    ) -> Result<crate::types::ApiV2010AccountSipSipCredentialList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json`.\n\nGet a Credential List\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `sid: &'astr`: The credential list Sid that uniquely identifies this resource (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_credential_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipCredentialList = client\n        .default()\n        .fetch_sip_credential_list(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_sip_credential_list<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountSipSipCredentialList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json`.\n\nUpdate a Credential List\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `sid: &'astr`: The credential list Sid that uniquely identifies this resource (required)\n\n```rust,no_run\nasync fn example_default_update_sip_credential_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipCredentialList = client\n        .default()\n        .update_sip_credential_list(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateSipCredentialListRequest {\n                friendly_name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_sip_credential_list<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateSipCredentialListRequest,
    ) -> Result<crate::types::ApiV2010AccountSipSipCredentialList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to \
             `/2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json`.\n\nDelete a \
             Credential List\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the \
             Account that is responsible for this resource. (required)\n- `sid: &'astr`: The \
             credential list Sid that uniquely identifies this resource \
             (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_credential_list() \
             -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    \
             client\n        .default()\n        .delete_sip_credential_list(\"some-string\", \
             \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_credential_list<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/CredentialLists/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.json`.\n\nRead multiple CredentialListMapping resources from an account.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP Domain that includes the resource to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipCredentialListMappingResponse = client\n        .default()\n        .list_sip_credential_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSipCredentialListMappingResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.json`.\n\nCreate a CredentialListMapping resource for an account.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP Domain for which the CredentialList resource will be mapped. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipDomainSipCredentialListMapping = client\n        .default()\n        .create_sip_credential_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateSipCredentialListMappingRequest {\n                credential_list_sid: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_sip_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        body: &crate::types::CreateSipCredentialListMappingRequest,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipDomainSipCredentialListMapping,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/{Sid}.json`.\n\nFetch a single CredentialListMapping resource from an account.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP Domain that includes the resource to fetch. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipDomainSipCredentialListMapping = client\n        .default()\n        .fetch_sip_credential_list_mapping(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_sip_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipDomainSipCredentialListMapping,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/\
                 {Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/{Sid}.json`.\n\nDelete a CredentialListMapping resource from an account.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP Domain that includes the resource to delete. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_credential_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_credential_list_mapping(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_credential_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/CredentialListMappings/\
                 {Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains.json`.\n\nRetrieve a list of domains belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_domain() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipDomainResponse = client\n        .default()\n        .list_sip_domain(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_domain<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSipDomainResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains.json`.\n\nCreate a new Domain\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_domain() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipDomain = client\n        .default()\n        .create_sip_domain(\n            \"some-string\",\n            &twilio_api::types::CreateSipDomainRequest {\n                domain_name: \"some-string\".to_string(),\n                friendly_name: Some(\"some-string\".to_string()),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(twilio_api::types::CreateSipDomainRequestVoiceMethod::Head),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_fallback_method: Some(\n                    twilio_api::types::CreateSipDomainRequestVoiceFallbackMethod::Delete,\n                ),\n                voice_status_callback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_status_callback_method: Some(\n                    twilio_api::types::CreateSipDomainRequestVoiceStatusCallbackMethod::Put,\n                ),\n                sip_registration: Some(true),\n                emergency_calling_enabled: Some(true),\n                secure: Some(false),\n                byoc_trunk_sid: Some(\"some-string\".to_string()),\n                emergency_caller_sid: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_sip_domain<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateSipDomainRequest,
    ) -> Result<crate::types::ApiV2010AccountSipSipDomain, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json`.\n\nFetch an instance of a Domain\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the SipDomain resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_domain() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipDomain = client\n        .default()\n        .fetch_sip_domain(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_sip_domain<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountSipSipDomain, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json`.\n\nUpdate the attributes of a domain\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resource to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the SipDomain resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_sip_domain() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipDomain = client\n        .default()\n        .update_sip_domain(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateSipDomainRequest {\n                friendly_name: Some(\"some-string\".to_string()),\n                voice_fallback_method: Some(\n                    twilio_api::types::UpdateSipDomainRequestVoiceFallbackMethod::Post,\n                ),\n                voice_fallback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_method: Some(twilio_api::types::UpdateSipDomainRequestVoiceMethod::Head),\n                voice_status_callback_method: Some(\n                    twilio_api::types::UpdateSipDomainRequestVoiceStatusCallbackMethod::Get,\n                ),\n                voice_status_callback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                voice_url: Some(\"https://example.com/foo/bar\".to_string()),\n                sip_registration: Some(false),\n                domain_name: Some(\"some-string\".to_string()),\n                emergency_calling_enabled: Some(false),\n                secure: Some(false),\n                byoc_trunk_sid: Some(\"some-string\".to_string()),\n                emergency_caller_sid: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_sip_domain<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateSipDomainRequest,
    ) -> Result<crate::types::ApiV2010AccountSipSipDomain, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json`.\n\nDelete an instance of a Domain\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the SipDomain resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the SipDomain resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_domain() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_domain(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_domain<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json`.\n\nRetrieve a list of IpAccessControlLists that belong to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_ip_access_control_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipIpAccessControlListResponse = client\n        .default()\n        .list_sip_ip_access_control_list(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_ip_access_control_list<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSipIpAccessControlListResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json`.\n\nCreate a new IpAccessControlList resource\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_ip_access_control_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipIpAccessControlList = client\n        .default()\n        .create_sip_ip_access_control_list(\n            \"some-string\",\n            &twilio_api::types::CreateSipIpAccessControlListRequest {\n                friendly_name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_sip_ip_access_control_list<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateSipIpAccessControlListRequest,
    ) -> Result<crate::types::ApiV2010AccountSipSipIpAccessControlList, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json`.\n\nFetch a specific instance of an IpAccessControlList\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_ip_access_control_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipIpAccessControlList = client\n        .default()\n        .fetch_sip_ip_access_control_list(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_sip_ip_access_control_list<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountSipSipIpAccessControlList, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json`.\n\nRename an IpAccessControlList\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to udpate. (required)\n\n```rust,no_run\nasync fn example_default_update_sip_ip_access_control_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipIpAccessControlList = client\n        .default()\n        .update_sip_ip_access_control_list(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateSipIpAccessControlListRequest {\n                friendly_name: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_sip_ip_access_control_list<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateSipIpAccessControlListRequest,
    ) -> Result<crate::types::ApiV2010AccountSipSipIpAccessControlList, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json`.\n\nDelete an IpAccessControlList from the requested account\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_ip_access_control_list() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_ip_access_control_list(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_ip_access_control_list<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings/{Sid}.json`.\n\nFetch an IpAccessControlListMapping resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP domain. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_ip_access_control_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipDomainSipIpAccessControlListMapping = client\n        .default()\n        .fetch_sip_ip_access_control_list_mapping(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_sip_ip_access_control_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipDomainSipIpAccessControlListMapping,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/\
                 IpAccessControlListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings/{Sid}.json`.\n\nDelete an IpAccessControlListMapping resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP domain. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_ip_access_control_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_ip_access_control_list_mapping(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_ip_access_control_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/\
                 IpAccessControlListMappings/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings.json`.\n\nRetrieve a list of IpAccessControlListMapping resources.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP domain. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_ip_access_control_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipIpAccessControlListMappingResponse = client\n        .default()\n        .list_sip_ip_access_control_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_ip_access_control_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSipIpAccessControlListMappingResponse, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/\
                 IpAccessControlListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/IpAccessControlListMappings.json`.\n\nCreate a new IpAccessControlListMapping resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the Account that is responsible for this resource. (required)\n- `domain_sid: &'astr`: A 34 character string that uniquely identifies the SIP domain. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_ip_access_control_list_mapping() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipDomainSipIpAccessControlListMapping = client\n        .default()\n        .create_sip_ip_access_control_list_mapping(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateSipIpAccessControlListMappingRequest {\n                ip_access_control_list_sid: \"some-string\".to_string(),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_sip_ip_access_control_list_mapping<'a>(
        &'a self,
        account_sid: &'a str,
        domain_sid: &'a str,
        body: &crate::types::CreateSipIpAccessControlListMappingRequest,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipDomainSipIpAccessControlListMapping,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/Domains/{DomainSid}/\
                 IpAccessControlListMappings.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{DomainSid}", domain_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses.json`.\n\nRead multiple IpAddress resources.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `ip_access_control_list_sid: &'astr`: The IpAccessControlList Sid that identifies the IpAddress resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_sip_ip_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListSipIpAddressResponse = client\n        .default()\n        .list_sip_ip_address(\n            \"some-string\",\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_sip_ip_address<'a>(
        &'a self,
        account_sid: &'a str,
        ip_access_control_list_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListSipIpAddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/\
                 {IpAccessControlListSid}/IpAddresses.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{IpAccessControlListSid}", ip_access_control_list_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses.json`.\n\nCreate a new IpAddress resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `ip_access_control_list_sid: &'astr`: The IpAccessControlList Sid with which to associate the created IpAddress resource. (required)\n\n```rust,no_run\nasync fn example_default_create_sip_ip_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipIpAccessControlListSipIpAddress = client\n        .default()\n        .create_sip_ip_address(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateSipIpAddressRequest {\n                friendly_name: \"some-string\".to_string(),\n                ip_address: \"some-string\".to_string(),\n                cidr_prefix_length: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_sip_ip_address<'a>(
        &'a self,
        account_sid: &'a str,
        ip_access_control_list_sid: &'a str,
        body: &crate::types::CreateSipIpAddressRequest,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipIpAccessControlListSipIpAddress,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/\
                 {IpAccessControlListSid}/IpAddresses.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{IpAccessControlListSid}", ip_access_control_list_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json`.\n\nRead one IpAddress resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `ip_access_control_list_sid: &'astr`: The IpAccessControlList Sid that identifies the IpAddress resources to fetch. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the IpAddress resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_sip_ip_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipIpAccessControlListSipIpAddress = client\n        .default()\n        .fetch_sip_ip_address(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_sip_ip_address<'a>(
        &'a self,
        account_sid: &'a str,
        ip_access_control_list_sid: &'a str,
        sid: &'a str,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipIpAccessControlListSipIpAddress,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/\
                 {IpAccessControlListSid}/IpAddresses/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{IpAccessControlListSid}", ip_access_control_list_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json`.\n\nUpdate an IpAddress resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `ip_access_control_list_sid: &'astr`: The IpAccessControlList Sid that identifies the IpAddress resources to update. (required)\n- `sid: &'astr`: A 34 character string that identifies the IpAddress resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_sip_ip_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountSipSipIpAccessControlListSipIpAddress = client\n        .default()\n        .update_sip_ip_address(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateSipIpAddressRequest {\n                ip_address: Some(\"some-string\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n                cidr_prefix_length: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_sip_ip_address<'a>(
        &'a self,
        account_sid: &'a str,
        ip_access_control_list_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateSipIpAddressRequest,
    ) -> Result<
        crate::types::ApiV2010AccountSipSipIpAccessControlListSipIpAddress,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/\
                 {IpAccessControlListSid}/IpAddresses/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{IpAccessControlListSid}", ip_access_control_list_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/{IpAccessControlListSid}/IpAddresses/{Sid}.json`.\n\nDelete an IpAddress resource.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The unique id of the [Account](https://www.twilio.com/docs/iam/api/account) responsible for this resource. (required)\n- `ip_access_control_list_sid: &'astr`: The IpAccessControlList Sid that identifies the IpAddress resources to delete. (required)\n- `sid: &'astr`: A 34 character string that uniquely identifies the resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_sip_ip_address() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_sip_ip_address(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_sip_ip_address<'a>(
        &'a self,
        account_sid: &'a str,
        ip_access_control_list_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/SIP/IpAccessControlLists/\
                 {IpAccessControlListSid}/IpAddresses/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{IpAccessControlListSid}", ip_access_control_list_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec.json`.\n\nCreate a Siprec\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Siprec resource. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Siprec resource is associated with. (required)\n\n```rust,no_run\nasync fn example_default_create_siprec() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallSiprec = client\n        .default()\n        .create_siprec(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateSiprecRequest {\n                name: Some(\"some-string\".to_string()),\n                connector_name: Some(\"some-string\".to_string()),\n                track: Some(twilio_api::types::SiprecEnumTrack::OutboundTrack),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateSiprecRequestStatusCallbackMethod::Get,\n                ),\n                parameter_1_name: Some(\"some-string\".to_string()),\n                parameter_1_value: Some(\"some-string\".to_string()),\n                parameter_2_name: Some(\"some-string\".to_string()),\n                parameter_2_value: Some(\"some-string\".to_string()),\n                parameter_3_name: Some(\"some-string\".to_string()),\n                parameter_3_value: Some(\"some-string\".to_string()),\n                parameter_4_name: Some(\"some-string\".to_string()),\n                parameter_4_value: Some(\"some-string\".to_string()),\n                parameter_5_name: Some(\"some-string\".to_string()),\n                parameter_5_value: Some(\"some-string\".to_string()),\n                parameter_6_name: Some(\"some-string\".to_string()),\n                parameter_6_value: Some(\"some-string\".to_string()),\n                parameter_7_name: Some(\"some-string\".to_string()),\n                parameter_7_value: Some(\"some-string\".to_string()),\n                parameter_8_name: Some(\"some-string\".to_string()),\n                parameter_8_value: Some(\"some-string\".to_string()),\n                parameter_9_name: Some(\"some-string\".to_string()),\n                parameter_9_value: Some(\"some-string\".to_string()),\n                parameter_10_name: Some(\"some-string\".to_string()),\n                parameter_10_value: Some(\"some-string\".to_string()),\n                parameter_11_name: Some(\"some-string\".to_string()),\n                parameter_11_value: Some(\"some-string\".to_string()),\n                parameter_12_name: Some(\"some-string\".to_string()),\n                parameter_12_value: Some(\"some-string\".to_string()),\n                parameter_13_name: Some(\"some-string\".to_string()),\n                parameter_13_value: Some(\"some-string\".to_string()),\n                parameter_14_name: Some(\"some-string\".to_string()),\n                parameter_14_value: Some(\"some-string\".to_string()),\n                parameter_15_name: Some(\"some-string\".to_string()),\n                parameter_15_value: Some(\"some-string\".to_string()),\n                parameter_16_name: Some(\"some-string\".to_string()),\n                parameter_16_value: Some(\"some-string\".to_string()),\n                parameter_17_name: Some(\"some-string\".to_string()),\n                parameter_17_value: Some(\"some-string\".to_string()),\n                parameter_18_name: Some(\"some-string\".to_string()),\n                parameter_18_value: Some(\"some-string\".to_string()),\n                parameter_19_name: Some(\"some-string\".to_string()),\n                parameter_19_value: Some(\"some-string\".to_string()),\n                parameter_20_name: Some(\"some-string\".to_string()),\n                parameter_20_value: Some(\"some-string\".to_string()),\n                parameter_21_name: Some(\"some-string\".to_string()),\n                parameter_21_value: Some(\"some-string\".to_string()),\n                parameter_22_name: Some(\"some-string\".to_string()),\n                parameter_22_value: Some(\"some-string\".to_string()),\n                parameter_23_name: Some(\"some-string\".to_string()),\n                parameter_23_value: Some(\"some-string\".to_string()),\n                parameter_24_name: Some(\"some-string\".to_string()),\n                parameter_24_value: Some(\"some-string\".to_string()),\n                parameter_25_name: Some(\"some-string\".to_string()),\n                parameter_25_value: Some(\"some-string\".to_string()),\n                parameter_26_name: Some(\"some-string\".to_string()),\n                parameter_26_value: Some(\"some-string\".to_string()),\n                parameter_27_name: Some(\"some-string\".to_string()),\n                parameter_27_value: Some(\"some-string\".to_string()),\n                parameter_28_name: Some(\"some-string\".to_string()),\n                parameter_28_value: Some(\"some-string\".to_string()),\n                parameter_29_name: Some(\"some-string\".to_string()),\n                parameter_29_value: Some(\"some-string\".to_string()),\n                parameter_30_name: Some(\"some-string\".to_string()),\n                parameter_30_value: Some(\"some-string\".to_string()),\n                parameter_31_name: Some(\"some-string\".to_string()),\n                parameter_31_value: Some(\"some-string\".to_string()),\n                parameter_32_name: Some(\"some-string\".to_string()),\n                parameter_32_value: Some(\"some-string\".to_string()),\n                parameter_33_name: Some(\"some-string\".to_string()),\n                parameter_33_value: Some(\"some-string\".to_string()),\n                parameter_34_name: Some(\"some-string\".to_string()),\n                parameter_34_value: Some(\"some-string\".to_string()),\n                parameter_35_name: Some(\"some-string\".to_string()),\n                parameter_35_value: Some(\"some-string\".to_string()),\n                parameter_36_name: Some(\"some-string\".to_string()),\n                parameter_36_value: Some(\"some-string\".to_string()),\n                parameter_37_name: Some(\"some-string\".to_string()),\n                parameter_37_value: Some(\"some-string\".to_string()),\n                parameter_38_name: Some(\"some-string\".to_string()),\n                parameter_38_value: Some(\"some-string\".to_string()),\n                parameter_39_name: Some(\"some-string\".to_string()),\n                parameter_39_value: Some(\"some-string\".to_string()),\n                parameter_40_name: Some(\"some-string\".to_string()),\n                parameter_40_value: Some(\"some-string\".to_string()),\n                parameter_41_name: Some(\"some-string\".to_string()),\n                parameter_41_value: Some(\"some-string\".to_string()),\n                parameter_42_name: Some(\"some-string\".to_string()),\n                parameter_42_value: Some(\"some-string\".to_string()),\n                parameter_43_name: Some(\"some-string\".to_string()),\n                parameter_43_value: Some(\"some-string\".to_string()),\n                parameter_44_name: Some(\"some-string\".to_string()),\n                parameter_44_value: Some(\"some-string\".to_string()),\n                parameter_45_name: Some(\"some-string\".to_string()),\n                parameter_45_value: Some(\"some-string\".to_string()),\n                parameter_46_name: Some(\"some-string\".to_string()),\n                parameter_46_value: Some(\"some-string\".to_string()),\n                parameter_47_name: Some(\"some-string\".to_string()),\n                parameter_47_value: Some(\"some-string\".to_string()),\n                parameter_48_name: Some(\"some-string\".to_string()),\n                parameter_48_value: Some(\"some-string\".to_string()),\n                parameter_49_name: Some(\"some-string\".to_string()),\n                parameter_49_value: Some(\"some-string\".to_string()),\n                parameter_50_name: Some(\"some-string\".to_string()),\n                parameter_50_value: Some(\"some-string\".to_string()),\n                parameter_51_name: Some(\"some-string\".to_string()),\n                parameter_51_value: Some(\"some-string\".to_string()),\n                parameter_52_name: Some(\"some-string\".to_string()),\n                parameter_52_value: Some(\"some-string\".to_string()),\n                parameter_53_name: Some(\"some-string\".to_string()),\n                parameter_53_value: Some(\"some-string\".to_string()),\n                parameter_54_name: Some(\"some-string\".to_string()),\n                parameter_54_value: Some(\"some-string\".to_string()),\n                parameter_55_name: Some(\"some-string\".to_string()),\n                parameter_55_value: Some(\"some-string\".to_string()),\n                parameter_56_name: Some(\"some-string\".to_string()),\n                parameter_56_value: Some(\"some-string\".to_string()),\n                parameter_57_name: Some(\"some-string\".to_string()),\n                parameter_57_value: Some(\"some-string\".to_string()),\n                parameter_58_name: Some(\"some-string\".to_string()),\n                parameter_58_value: Some(\"some-string\".to_string()),\n                parameter_59_name: Some(\"some-string\".to_string()),\n                parameter_59_value: Some(\"some-string\".to_string()),\n                parameter_60_name: Some(\"some-string\".to_string()),\n                parameter_60_value: Some(\"some-string\".to_string()),\n                parameter_61_name: Some(\"some-string\".to_string()),\n                parameter_61_value: Some(\"some-string\".to_string()),\n                parameter_62_name: Some(\"some-string\".to_string()),\n                parameter_62_value: Some(\"some-string\".to_string()),\n                parameter_63_name: Some(\"some-string\".to_string()),\n                parameter_63_value: Some(\"some-string\".to_string()),\n                parameter_64_name: Some(\"some-string\".to_string()),\n                parameter_64_value: Some(\"some-string\".to_string()),\n                parameter_65_name: Some(\"some-string\".to_string()),\n                parameter_65_value: Some(\"some-string\".to_string()),\n                parameter_66_name: Some(\"some-string\".to_string()),\n                parameter_66_value: Some(\"some-string\".to_string()),\n                parameter_67_name: Some(\"some-string\".to_string()),\n                parameter_67_value: Some(\"some-string\".to_string()),\n                parameter_68_name: Some(\"some-string\".to_string()),\n                parameter_68_value: Some(\"some-string\".to_string()),\n                parameter_69_name: Some(\"some-string\".to_string()),\n                parameter_69_value: Some(\"some-string\".to_string()),\n                parameter_70_name: Some(\"some-string\".to_string()),\n                parameter_70_value: Some(\"some-string\".to_string()),\n                parameter_71_name: Some(\"some-string\".to_string()),\n                parameter_71_value: Some(\"some-string\".to_string()),\n                parameter_72_name: Some(\"some-string\".to_string()),\n                parameter_72_value: Some(\"some-string\".to_string()),\n                parameter_73_name: Some(\"some-string\".to_string()),\n                parameter_73_value: Some(\"some-string\".to_string()),\n                parameter_74_name: Some(\"some-string\".to_string()),\n                parameter_74_value: Some(\"some-string\".to_string()),\n                parameter_75_name: Some(\"some-string\".to_string()),\n                parameter_75_value: Some(\"some-string\".to_string()),\n                parameter_76_name: Some(\"some-string\".to_string()),\n                parameter_76_value: Some(\"some-string\".to_string()),\n                parameter_77_name: Some(\"some-string\".to_string()),\n                parameter_77_value: Some(\"some-string\".to_string()),\n                parameter_78_name: Some(\"some-string\".to_string()),\n                parameter_78_value: Some(\"some-string\".to_string()),\n                parameter_79_name: Some(\"some-string\".to_string()),\n                parameter_79_value: Some(\"some-string\".to_string()),\n                parameter_80_name: Some(\"some-string\".to_string()),\n                parameter_80_value: Some(\"some-string\".to_string()),\n                parameter_81_name: Some(\"some-string\".to_string()),\n                parameter_81_value: Some(\"some-string\".to_string()),\n                parameter_82_name: Some(\"some-string\".to_string()),\n                parameter_82_value: Some(\"some-string\".to_string()),\n                parameter_83_name: Some(\"some-string\".to_string()),\n                parameter_83_value: Some(\"some-string\".to_string()),\n                parameter_84_name: Some(\"some-string\".to_string()),\n                parameter_84_value: Some(\"some-string\".to_string()),\n                parameter_85_name: Some(\"some-string\".to_string()),\n                parameter_85_value: Some(\"some-string\".to_string()),\n                parameter_86_name: Some(\"some-string\".to_string()),\n                parameter_86_value: Some(\"some-string\".to_string()),\n                parameter_87_name: Some(\"some-string\".to_string()),\n                parameter_87_value: Some(\"some-string\".to_string()),\n                parameter_88_name: Some(\"some-string\".to_string()),\n                parameter_88_value: Some(\"some-string\".to_string()),\n                parameter_89_name: Some(\"some-string\".to_string()),\n                parameter_89_value: Some(\"some-string\".to_string()),\n                parameter_90_name: Some(\"some-string\".to_string()),\n                parameter_90_value: Some(\"some-string\".to_string()),\n                parameter_91_name: Some(\"some-string\".to_string()),\n                parameter_91_value: Some(\"some-string\".to_string()),\n                parameter_92_name: Some(\"some-string\".to_string()),\n                parameter_92_value: Some(\"some-string\".to_string()),\n                parameter_93_name: Some(\"some-string\".to_string()),\n                parameter_93_value: Some(\"some-string\".to_string()),\n                parameter_94_name: Some(\"some-string\".to_string()),\n                parameter_94_value: Some(\"some-string\".to_string()),\n                parameter_95_name: Some(\"some-string\".to_string()),\n                parameter_95_value: Some(\"some-string\".to_string()),\n                parameter_96_name: Some(\"some-string\".to_string()),\n                parameter_96_value: Some(\"some-string\".to_string()),\n                parameter_97_name: Some(\"some-string\".to_string()),\n                parameter_97_value: Some(\"some-string\".to_string()),\n                parameter_98_name: Some(\"some-string\".to_string()),\n                parameter_98_value: Some(\"some-string\".to_string()),\n                parameter_99_name: Some(\"some-string\".to_string()),\n                parameter_99_value: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_siprec<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        body: &crate::types::CreateSiprecRequest,
    ) -> Result<crate::types::ApiV2010AccountCallSiprec, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec/{Sid}.json`.\n\nStop a Siprec using either the SID of the Siprec resource or the `name` used when creating the resource\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Siprec resource. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Siprec resource is associated with. (required)\n- `sid: &'astr`: The SID of the Siprec resource, or the `name` used when creating the resource (required)\n\n```rust,no_run\nasync fn example_default_update_siprec() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallSiprec = client\n        .default()\n        .update_siprec(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateSiprecRequest {\n                status: twilio_api::types::SiprecEnumUpdateStatus::Stopped,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_siprec<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateSiprecRequest,
    ) -> Result<crate::types::ApiV2010AccountCallSiprec, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Siprec/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams.json`.\n\nCreate a Stream\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Stream resource. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Stream resource is associated with. (required)\n\n```rust,no_run\nasync fn example_default_create_stream() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallStream = client\n        .default()\n        .create_stream(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateStreamRequest {\n                url: \"https://example.com/foo/bar\".to_string(),\n                name: Some(\"some-string\".to_string()),\n                track: Some(twilio_api::types::StreamEnumTrack::InboundTrack),\n                status_callback: Some(\"https://example.com/foo/bar\".to_string()),\n                status_callback_method: Some(\n                    twilio_api::types::CreateStreamRequestStatusCallbackMethod::Put,\n                ),\n                parameter_1_name: Some(\"some-string\".to_string()),\n                parameter_1_value: Some(\"some-string\".to_string()),\n                parameter_2_name: Some(\"some-string\".to_string()),\n                parameter_2_value: Some(\"some-string\".to_string()),\n                parameter_3_name: Some(\"some-string\".to_string()),\n                parameter_3_value: Some(\"some-string\".to_string()),\n                parameter_4_name: Some(\"some-string\".to_string()),\n                parameter_4_value: Some(\"some-string\".to_string()),\n                parameter_5_name: Some(\"some-string\".to_string()),\n                parameter_5_value: Some(\"some-string\".to_string()),\n                parameter_6_name: Some(\"some-string\".to_string()),\n                parameter_6_value: Some(\"some-string\".to_string()),\n                parameter_7_name: Some(\"some-string\".to_string()),\n                parameter_7_value: Some(\"some-string\".to_string()),\n                parameter_8_name: Some(\"some-string\".to_string()),\n                parameter_8_value: Some(\"some-string\".to_string()),\n                parameter_9_name: Some(\"some-string\".to_string()),\n                parameter_9_value: Some(\"some-string\".to_string()),\n                parameter_10_name: Some(\"some-string\".to_string()),\n                parameter_10_value: Some(\"some-string\".to_string()),\n                parameter_11_name: Some(\"some-string\".to_string()),\n                parameter_11_value: Some(\"some-string\".to_string()),\n                parameter_12_name: Some(\"some-string\".to_string()),\n                parameter_12_value: Some(\"some-string\".to_string()),\n                parameter_13_name: Some(\"some-string\".to_string()),\n                parameter_13_value: Some(\"some-string\".to_string()),\n                parameter_14_name: Some(\"some-string\".to_string()),\n                parameter_14_value: Some(\"some-string\".to_string()),\n                parameter_15_name: Some(\"some-string\".to_string()),\n                parameter_15_value: Some(\"some-string\".to_string()),\n                parameter_16_name: Some(\"some-string\".to_string()),\n                parameter_16_value: Some(\"some-string\".to_string()),\n                parameter_17_name: Some(\"some-string\".to_string()),\n                parameter_17_value: Some(\"some-string\".to_string()),\n                parameter_18_name: Some(\"some-string\".to_string()),\n                parameter_18_value: Some(\"some-string\".to_string()),\n                parameter_19_name: Some(\"some-string\".to_string()),\n                parameter_19_value: Some(\"some-string\".to_string()),\n                parameter_20_name: Some(\"some-string\".to_string()),\n                parameter_20_value: Some(\"some-string\".to_string()),\n                parameter_21_name: Some(\"some-string\".to_string()),\n                parameter_21_value: Some(\"some-string\".to_string()),\n                parameter_22_name: Some(\"some-string\".to_string()),\n                parameter_22_value: Some(\"some-string\".to_string()),\n                parameter_23_name: Some(\"some-string\".to_string()),\n                parameter_23_value: Some(\"some-string\".to_string()),\n                parameter_24_name: Some(\"some-string\".to_string()),\n                parameter_24_value: Some(\"some-string\".to_string()),\n                parameter_25_name: Some(\"some-string\".to_string()),\n                parameter_25_value: Some(\"some-string\".to_string()),\n                parameter_26_name: Some(\"some-string\".to_string()),\n                parameter_26_value: Some(\"some-string\".to_string()),\n                parameter_27_name: Some(\"some-string\".to_string()),\n                parameter_27_value: Some(\"some-string\".to_string()),\n                parameter_28_name: Some(\"some-string\".to_string()),\n                parameter_28_value: Some(\"some-string\".to_string()),\n                parameter_29_name: Some(\"some-string\".to_string()),\n                parameter_29_value: Some(\"some-string\".to_string()),\n                parameter_30_name: Some(\"some-string\".to_string()),\n                parameter_30_value: Some(\"some-string\".to_string()),\n                parameter_31_name: Some(\"some-string\".to_string()),\n                parameter_31_value: Some(\"some-string\".to_string()),\n                parameter_32_name: Some(\"some-string\".to_string()),\n                parameter_32_value: Some(\"some-string\".to_string()),\n                parameter_33_name: Some(\"some-string\".to_string()),\n                parameter_33_value: Some(\"some-string\".to_string()),\n                parameter_34_name: Some(\"some-string\".to_string()),\n                parameter_34_value: Some(\"some-string\".to_string()),\n                parameter_35_name: Some(\"some-string\".to_string()),\n                parameter_35_value: Some(\"some-string\".to_string()),\n                parameter_36_name: Some(\"some-string\".to_string()),\n                parameter_36_value: Some(\"some-string\".to_string()),\n                parameter_37_name: Some(\"some-string\".to_string()),\n                parameter_37_value: Some(\"some-string\".to_string()),\n                parameter_38_name: Some(\"some-string\".to_string()),\n                parameter_38_value: Some(\"some-string\".to_string()),\n                parameter_39_name: Some(\"some-string\".to_string()),\n                parameter_39_value: Some(\"some-string\".to_string()),\n                parameter_40_name: Some(\"some-string\".to_string()),\n                parameter_40_value: Some(\"some-string\".to_string()),\n                parameter_41_name: Some(\"some-string\".to_string()),\n                parameter_41_value: Some(\"some-string\".to_string()),\n                parameter_42_name: Some(\"some-string\".to_string()),\n                parameter_42_value: Some(\"some-string\".to_string()),\n                parameter_43_name: Some(\"some-string\".to_string()),\n                parameter_43_value: Some(\"some-string\".to_string()),\n                parameter_44_name: Some(\"some-string\".to_string()),\n                parameter_44_value: Some(\"some-string\".to_string()),\n                parameter_45_name: Some(\"some-string\".to_string()),\n                parameter_45_value: Some(\"some-string\".to_string()),\n                parameter_46_name: Some(\"some-string\".to_string()),\n                parameter_46_value: Some(\"some-string\".to_string()),\n                parameter_47_name: Some(\"some-string\".to_string()),\n                parameter_47_value: Some(\"some-string\".to_string()),\n                parameter_48_name: Some(\"some-string\".to_string()),\n                parameter_48_value: Some(\"some-string\".to_string()),\n                parameter_49_name: Some(\"some-string\".to_string()),\n                parameter_49_value: Some(\"some-string\".to_string()),\n                parameter_50_name: Some(\"some-string\".to_string()),\n                parameter_50_value: Some(\"some-string\".to_string()),\n                parameter_51_name: Some(\"some-string\".to_string()),\n                parameter_51_value: Some(\"some-string\".to_string()),\n                parameter_52_name: Some(\"some-string\".to_string()),\n                parameter_52_value: Some(\"some-string\".to_string()),\n                parameter_53_name: Some(\"some-string\".to_string()),\n                parameter_53_value: Some(\"some-string\".to_string()),\n                parameter_54_name: Some(\"some-string\".to_string()),\n                parameter_54_value: Some(\"some-string\".to_string()),\n                parameter_55_name: Some(\"some-string\".to_string()),\n                parameter_55_value: Some(\"some-string\".to_string()),\n                parameter_56_name: Some(\"some-string\".to_string()),\n                parameter_56_value: Some(\"some-string\".to_string()),\n                parameter_57_name: Some(\"some-string\".to_string()),\n                parameter_57_value: Some(\"some-string\".to_string()),\n                parameter_58_name: Some(\"some-string\".to_string()),\n                parameter_58_value: Some(\"some-string\".to_string()),\n                parameter_59_name: Some(\"some-string\".to_string()),\n                parameter_59_value: Some(\"some-string\".to_string()),\n                parameter_60_name: Some(\"some-string\".to_string()),\n                parameter_60_value: Some(\"some-string\".to_string()),\n                parameter_61_name: Some(\"some-string\".to_string()),\n                parameter_61_value: Some(\"some-string\".to_string()),\n                parameter_62_name: Some(\"some-string\".to_string()),\n                parameter_62_value: Some(\"some-string\".to_string()),\n                parameter_63_name: Some(\"some-string\".to_string()),\n                parameter_63_value: Some(\"some-string\".to_string()),\n                parameter_64_name: Some(\"some-string\".to_string()),\n                parameter_64_value: Some(\"some-string\".to_string()),\n                parameter_65_name: Some(\"some-string\".to_string()),\n                parameter_65_value: Some(\"some-string\".to_string()),\n                parameter_66_name: Some(\"some-string\".to_string()),\n                parameter_66_value: Some(\"some-string\".to_string()),\n                parameter_67_name: Some(\"some-string\".to_string()),\n                parameter_67_value: Some(\"some-string\".to_string()),\n                parameter_68_name: Some(\"some-string\".to_string()),\n                parameter_68_value: Some(\"some-string\".to_string()),\n                parameter_69_name: Some(\"some-string\".to_string()),\n                parameter_69_value: Some(\"some-string\".to_string()),\n                parameter_70_name: Some(\"some-string\".to_string()),\n                parameter_70_value: Some(\"some-string\".to_string()),\n                parameter_71_name: Some(\"some-string\".to_string()),\n                parameter_71_value: Some(\"some-string\".to_string()),\n                parameter_72_name: Some(\"some-string\".to_string()),\n                parameter_72_value: Some(\"some-string\".to_string()),\n                parameter_73_name: Some(\"some-string\".to_string()),\n                parameter_73_value: Some(\"some-string\".to_string()),\n                parameter_74_name: Some(\"some-string\".to_string()),\n                parameter_74_value: Some(\"some-string\".to_string()),\n                parameter_75_name: Some(\"some-string\".to_string()),\n                parameter_75_value: Some(\"some-string\".to_string()),\n                parameter_76_name: Some(\"some-string\".to_string()),\n                parameter_76_value: Some(\"some-string\".to_string()),\n                parameter_77_name: Some(\"some-string\".to_string()),\n                parameter_77_value: Some(\"some-string\".to_string()),\n                parameter_78_name: Some(\"some-string\".to_string()),\n                parameter_78_value: Some(\"some-string\".to_string()),\n                parameter_79_name: Some(\"some-string\".to_string()),\n                parameter_79_value: Some(\"some-string\".to_string()),\n                parameter_80_name: Some(\"some-string\".to_string()),\n                parameter_80_value: Some(\"some-string\".to_string()),\n                parameter_81_name: Some(\"some-string\".to_string()),\n                parameter_81_value: Some(\"some-string\".to_string()),\n                parameter_82_name: Some(\"some-string\".to_string()),\n                parameter_82_value: Some(\"some-string\".to_string()),\n                parameter_83_name: Some(\"some-string\".to_string()),\n                parameter_83_value: Some(\"some-string\".to_string()),\n                parameter_84_name: Some(\"some-string\".to_string()),\n                parameter_84_value: Some(\"some-string\".to_string()),\n                parameter_85_name: Some(\"some-string\".to_string()),\n                parameter_85_value: Some(\"some-string\".to_string()),\n                parameter_86_name: Some(\"some-string\".to_string()),\n                parameter_86_value: Some(\"some-string\".to_string()),\n                parameter_87_name: Some(\"some-string\".to_string()),\n                parameter_87_value: Some(\"some-string\".to_string()),\n                parameter_88_name: Some(\"some-string\".to_string()),\n                parameter_88_value: Some(\"some-string\".to_string()),\n                parameter_89_name: Some(\"some-string\".to_string()),\n                parameter_89_value: Some(\"some-string\".to_string()),\n                parameter_90_name: Some(\"some-string\".to_string()),\n                parameter_90_value: Some(\"some-string\".to_string()),\n                parameter_91_name: Some(\"some-string\".to_string()),\n                parameter_91_value: Some(\"some-string\".to_string()),\n                parameter_92_name: Some(\"some-string\".to_string()),\n                parameter_92_value: Some(\"some-string\".to_string()),\n                parameter_93_name: Some(\"some-string\".to_string()),\n                parameter_93_value: Some(\"some-string\".to_string()),\n                parameter_94_name: Some(\"some-string\".to_string()),\n                parameter_94_value: Some(\"some-string\".to_string()),\n                parameter_95_name: Some(\"some-string\".to_string()),\n                parameter_95_value: Some(\"some-string\".to_string()),\n                parameter_96_name: Some(\"some-string\".to_string()),\n                parameter_96_value: Some(\"some-string\".to_string()),\n                parameter_97_name: Some(\"some-string\".to_string()),\n                parameter_97_value: Some(\"some-string\".to_string()),\n                parameter_98_name: Some(\"some-string\".to_string()),\n                parameter_98_value: Some(\"some-string\".to_string()),\n                parameter_99_name: Some(\"some-string\".to_string()),\n                parameter_99_value: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_stream<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        body: &crate::types::CreateStreamRequest,
    ) -> Result<crate::types::ApiV2010AccountCallStream, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams/{Sid}.json`.\n\nStop a Stream using either the SID of the Stream resource or the `name` used when creating the resource\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created this Stream resource. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the Stream resource is associated with. (required)\n- `sid: &'astr`: The SID of the Stream resource, or the `name` used when creating the resource (required)\n\n```rust,no_run\nasync fn example_default_update_stream() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallStream = client\n        .default()\n        .update_stream(\n            \"some-string\",\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateStreamRequest {\n                status: twilio_api::types::StreamEnumUpdateStatus::Stopped,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_stream<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateStreamRequest,
    ) -> Result<crate::types::ApiV2010AccountCallStream, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/Streams/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Tokens.json`.\n\nCreate a new token for ICE servers\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nasync fn example_default_create_token() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountToken = client\n        .default()\n        .create_token(\n            \"some-string\",\n            &twilio_api::types::CreateTokenRequest {\n                ttl: Some(4 as i64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_token<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateTokenRequest,
    ) -> Result<crate::types::ApiV2010AccountToken, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Tokens.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json`.\n\nFetch an instance of a Transcription\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Transcription resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_transcription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountTranscription = client\n        .default()\n        .fetch_transcription(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_transcription<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountTranscription, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json`.\n\nDelete a transcription from the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the Transcription resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_transcription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_transcription(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_transcription<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Transcriptions/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Transcriptions.json`.\n\nRetrieve a list of transcriptions belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the Transcription resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n\n```rust,no_run\nasync fn example_default_list_transcription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListTranscriptionResponse = client\n        .default()\n        .list_transcription(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_transcription<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
    ) -> Result<crate::types::ListTranscriptionResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Transcriptions.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records.json`.\n\nRetrieve a list of usage-records belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordResponse = client\n        .default()\n        .list_usage_record(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordEnumCategory::CallsPayVerbTransactions),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(true),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/AllTime.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordAllTimeEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_all_time() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordAllTimeResponse = client\n        .default()\n        .list_usage_record_all_time(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordAllTimeEnumCategory::CallsSip),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(false),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_all_time<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordAllTimeEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordAllTimeResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/AllTime.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/Daily.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordDailyEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_daily() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordDailyResponse = client\n        .default()\n        .list_usage_record_daily(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordDailyEnumCategory::PfaxMinutesInbound),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(false),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_daily<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordDailyEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordDailyResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/Daily.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/LastMonth.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordLastMonthEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_last_month() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordLastMonthResponse = client\n        .default()\n        .list_usage_record_last_month(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordLastMonthEnumCategory::TurnmegabytesIreland),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(true),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_last_month<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordLastMonthEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordLastMonthResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/LastMonth.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/Monthly.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordMonthlyEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_monthly() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordMonthlyResponse = client\n        .default()\n        .list_usage_record_monthly(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordMonthlyEnumCategory::MarketplaceNomoroboSpamScore),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(false),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_monthly<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordMonthlyEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordMonthlyResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/Monthly.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/ThisMonth.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordThisMonthEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_this_month() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordThisMonthResponse = client\n        .default()\n        .list_usage_record_this_month(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordThisMonthEnumCategory::AmazonPolly),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(true),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_this_month<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordThisMonthEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordThisMonthResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/ThisMonth.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/Today.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordTodayEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_today() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordTodayResponse = client\n        .default()\n        .list_usage_record_today(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordTodayEnumCategory::ShortcodesRandom),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(false),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_today<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordTodayEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordTodayResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/Today.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/Yearly.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordYearlyEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_yearly() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordYearlyResponse = client\n        .default()\n        .list_usage_record_yearly(\n            \"some-string\",\n            Some(twilio_api::types::UsageRecordYearlyEnumCategory::MmsOutbound),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(true),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_yearly<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordYearlyEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordYearlyResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/Yearly.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Records/Yesterday.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageRecord resources to read. (required)\n- `category: Option<crate::types::UsageRecordYesterdayEnumCategory>`: The [usage category](https://www.twilio.com/docs/usage/api/usage-record#usage-categories) of the UsageRecord resources to read. Only UsageRecord resources in the specified category are retrieved.\n- `end_date: Option<chrono::NaiveDate>`: Only include usage that occurred on or before this date. Specify the date in GMT and format as `YYYY-MM-DD`.  You can also specify offsets from the current date, such as: `+30days`, which will set the end date to 30 days from the current date.\n- `include_subaccounts: Option<bool>`: Whether to include usage from the master account and all its subaccounts. Can be: `true` (the default) to include usage from the master account and all subaccounts or `false` to retrieve usage from only the specified account.\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `start_date: Option<chrono::NaiveDate>`: Only include usage that has occurred on or after this date. Specify the date in GMT and format as `YYYY-MM-DD`. You can also specify offsets from the current date, such as: `-30days`, which will set the start date to be 30 days before the current date.\n\n```rust,no_run\nasync fn example_default_list_usage_record_yesterday() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageRecordYesterdayResponse = client\n        .default()\n        .list_usage_record_yesterday(\n            \"some-string\",\n            Some(\n                twilio_api::types::UsageRecordYesterdayEnumCategory::MarketplaceInfogroupDataaxleBizinfo,\n            ),\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(true),\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_record_yesterday<'a>(
        &'a self,
        account_sid: &'a str,
        category: Option<crate::types::UsageRecordYesterdayEnumCategory>,
        end_date: Option<chrono::NaiveDate>,
        include_subaccounts: Option<bool>,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ListUsageRecordYesterdayResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Records/Yesterday.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = category {
            query_params.push(("Category", format!("{p}")));
        }

        if let Some(p) = end_date {
            query_params.push(("EndDate", format!("{p}")));
        }

        if let Some(p) = include_subaccounts {
            query_params.push(("IncludeSubaccounts", format!("{p}")));
        }

        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = start_date {
            query_params.push(("StartDate", format!("{p}")));
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

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json`.\n\nFetch and instance of a usage-trigger\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resource to fetch. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the UsageTrigger resource to fetch. (required)\n\n```rust,no_run\nasync fn example_default_fetch_usage_trigger() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountUsageUsageTrigger = client\n        .default()\n        .fetch_usage_trigger(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn fetch_usage_trigger<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<crate::types::ApiV2010AccountUsageUsageTrigger, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json`.\n\nUpdate an instance of a usage trigger\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resources to update. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the UsageTrigger resource to update. (required)\n\n```rust,no_run\nasync fn example_default_update_usage_trigger() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountUsageUsageTrigger = client\n        .default()\n        .update_usage_trigger(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::UpdateUsageTriggerRequest {\n                callback_method: Some(\n                    twilio_api::types::UpdateUsageTriggerRequestCallbackMethod::Delete,\n                ),\n                callback_url: Some(\"https://example.com/foo/bar\".to_string()),\n                friendly_name: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_usage_trigger<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
        body: &crate::types::UpdateUsageTriggerRequest,
    ) -> Result<crate::types::ApiV2010AccountUsageUsageTrigger, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json`.\n\n\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resources to delete. (required)\n- `sid: &'astr`: The Twilio-provided string that uniquely identifies the UsageTrigger resource to delete. (required)\n\n```rust,no_run\nasync fn example_default_delete_usage_trigger() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_usage_trigger(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_usage_trigger<'a>(
        &'a self,
        account_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Triggers/{Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Perform a `GET` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json`.\n\nRetrieve a list of usage-triggers belonging to the account used to make the request\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created the UsageTrigger resources to read. (required)\n- `page: Option<i64>`: The page index. This value is simply for client state.\n- `page_size: Option<i64>`: How many resources to return in each list page. The default is 50, and the maximum is 1000.\n- `page_token: Option<String>`: The page token. This is provided by the API.\n- `recurring: Option<crate::types::UsageTriggerEnumRecurring>`: The frequency of recurring UsageTriggers to read. Can be: `daily`, `monthly`, or `yearly` to read recurring UsageTriggers. An empty value or a value of `alltime` reads non-recurring UsageTriggers.\n- `trigger_by: Option<crate::types::UsageTriggerEnumTriggerField>`: The trigger field of the UsageTriggers to read.  Can be: `count`, `usage`, or `price` as described in the [UsageRecords documentation](https://www.twilio.com/docs/usage/api/usage-record#usage-count-price).\n- `usage_category: Option<crate::types::UsageTriggerEnumUsageCategory>`: The usage category of the UsageTriggers to read. Must be a supported [usage categories](https://www.twilio.com/docs/usage/api/usage-record#usage-categories).\n\n```rust,no_run\nasync fn example_default_list_usage_trigger() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ListUsageTriggerResponse = client\n        .default()\n        .list_usage_trigger(\n            \"some-string\",\n            Some(4 as i64),\n            Some(4 as i64),\n            Some(\"some-string\".to_string()),\n            Some(twilio_api::types::UsageTriggerEnumRecurring::Monthly),\n            Some(twilio_api::types::UsageTriggerEnumTriggerField::Price),\n            Some(twilio_api::types::UsageTriggerEnumUsageCategory::VerifyTotp),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_usage_trigger<'a>(
        &'a self,
        account_sid: &'a str,
        page: Option<i64>,
        page_size: Option<i64>,
        page_token: Option<String>,
        recurring: Option<crate::types::UsageTriggerEnumRecurring>,
        trigger_by: Option<crate::types::UsageTriggerEnumTriggerField>,
        usage_category: Option<crate::types::UsageTriggerEnumUsageCategory>,
    ) -> Result<crate::types::ListUsageTriggerResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let mut query_params = vec![];
        if let Some(p) = page {
            query_params.push(("Page", format!("{p}")));
        }

        if let Some(p) = page_size {
            query_params.push(("PageSize", format!("{p}")));
        }

        if let Some(p) = page_token {
            query_params.push(("PageToken", p));
        }

        if let Some(p) = recurring {
            query_params.push(("Recurring", format!("{p}")));
        }

        if let Some(p) = trigger_by {
            query_params.push(("TriggerBy", format!("{p}")));
        }

        if let Some(p) = usage_category {
            query_params.push(("UsageCategory", format!("{p}")));
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json`.\n\nCreate a new UsageTrigger\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that will create the resource. (required)\n\n```rust,no_run\nasync fn example_default_create_usage_trigger() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountUsageUsageTrigger = client\n        .default()\n        .create_usage_trigger(\n            \"some-string\",\n            &twilio_api::types::CreateUsageTriggerRequest {\n                callback_url: \"https://example.com/foo/bar\".to_string(),\n                trigger_value: \"some-string\".to_string(),\n                usage_category: twilio_api::types::UsageTriggerEnumUsageCategory::WirelessUsageDataAsia,\n                callback_method: Some(\n                    twilio_api::types::CreateUsageTriggerRequestCallbackMethod::Delete,\n                ),\n                friendly_name: Some(\"some-string\".to_string()),\n                recurring: Some(twilio_api::types::UsageTriggerEnumRecurring::Yearly),\n                trigger_by: Some(twilio_api::types::UsageTriggerEnumTriggerField::Count),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_usage_trigger<'a>(
        &'a self,
        account_sid: &'a str,
        body: &crate::types::CreateUsageTriggerRequest,
    ) -> Result<crate::types::ApiV2010AccountUsageUsageTrigger, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Usage/Triggers.json"
                    .replace("{AccountSid}", account_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessages.json`.\n\nCreate a new User Defined Message for the given Call SID.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that created User Defined Message. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the User Defined Message is associated with. (required)\n\n```rust,no_run\nasync fn example_default_create_user_defined_message() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallUserDefinedMessage = client\n        .default()\n        .create_user_defined_message(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateUserDefinedMessageRequest {\n                content: \"some-string\".to_string(),\n                idempotency_key: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_defined_message<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        body: &crate::types::CreateUserDefinedMessageRequest,
    ) -> Result<crate::types::ApiV2010AccountCallUserDefinedMessage, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessages.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `POST` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions.json`.\n\nSubscribe to User Defined Messages for a given Call SID.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that subscribed to the User Defined Messages. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the User Defined Messages subscription is associated with. This refers to the Call SID that is producing the user defined messages. (required)\n\n```rust,no_run\nasync fn example_default_create_user_defined_message_subscription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    let result: twilio_api::types::ApiV2010AccountCallUserDefinedMessageSubscription = client\n        .default()\n        .create_user_defined_message_subscription(\n            \"some-string\",\n            \"some-string\",\n            &twilio_api::types::CreateUserDefinedMessageSubscriptionRequest {\n                callback: \"https://example.com/foo/bar\".to_string(),\n                idempotency_key: Some(\"some-string\".to_string()),\n                method: Some(twilio_api::types::CreateUserDefinedMessageSubscriptionRequestMethod::Post),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_defined_message_subscription<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        body: &crate::types::CreateUserDefinedMessageSubscriptionRequest,
    ) -> Result<
        crate::types::ApiV2010AccountCallUserDefinedMessageSubscription,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions.\
                 json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        req = req.form(body);
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

    #[doc = "Perform a `DELETE` request to `/2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions/{Sid}.json`.\n\nDelete a specific User Defined Message Subscription.\n\n**Parameters:**\n\n- `account_sid: &'astr`: The SID of the [Account](https://www.twilio.com/docs/iam/api/account) that subscribed to the User Defined Messages. (required)\n- `call_sid: &'astr`: The SID of the [Call](https://www.twilio.com/docs/voice/api/call-resource) the User Defined Message Subscription is associated with. This refers to the Call SID that is producing the User Defined Messages. (required)\n- `sid: &'astr`: The SID that uniquely identifies this User Defined Message Subscription. (required)\n\n```rust,no_run\nasync fn example_default_delete_user_defined_message_subscription() -> anyhow::Result<()> {\n    let client = twilio_api::Client::new_from_env();\n    client\n        .default()\n        .delete_user_defined_message_subscription(\"some-string\", \"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_user_defined_message_subscription<'a>(
        &'a self,
        account_sid: &'a str,
        call_sid: &'a str,
        sid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "2010-04-01/Accounts/{AccountSid}/Calls/{CallSid}/UserDefinedMessageSubscriptions/\
                 {Sid}.json"
                    .replace("{AccountSid}", account_sid)
                    .replace("{CallSid}", call_sid)
                    .replace("{Sid}", sid)
            ),
        );
        req = req.basic_auth(&self.client.username, Some(&self.client.password));
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
