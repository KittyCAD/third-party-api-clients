use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Employments {
    pub client: Client,
}

impl Employments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List employments\n\nLists all employments, except for the deleted ones.\n\nThis endpoint requires and returns country-specific data. The exact required and returned fields will\nvary depending on which country the employment is in. To see the list of parameters for each country,\nsee the **Show form schema** endpoint under the [Countries](#tag/Countries) category.\n\nPlease note that the compliance requirements for each country are subject to change, which will result\nin required and optional parameters being added and deleted without notice.\n\nIf you are using this endpoint to build an integration, make sure you are dynamically collecting or\ndisplaying the latest parameters for each country by querying the _\"Show form schema\"_ endpoint.\n\nFor more information on JSON Schemas, see the **How JSON Schemas work** documentation.\n\nTo learn how you can dynamically generate forms to display in your UI, see the documentation for\nthe [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.\n\n\n\n**Parameters:**\n\n- `company_id: Option<String>`: Company ID\n- `page: Option<i64>`: Starts fetching records after the given page\n- `page_size: Option<i64>`: Change the amount of records returned per page, defaults to 20, limited to 100\n\n```rust,no_run\nasync fn example_employments_get_index() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::ListEmploymentsResponse = client\n        .employments()\n        .get_index(\n            Some(\"some-string\".to_string()),\n            Some(4 as i64),\n            Some(4 as i64),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_index<'a>(
        &'a self,
        company_id: Option<String>,
        page: Option<i64>,
        page_size: Option<i64>,
    ) -> Result<crate::types::ListEmploymentsResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "v1/employments"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = company_id {
            query_params.push(("company_id", p));
        }

        if let Some(p) = page {
            query_params.push(("page", format!("{}", p)));
        }

        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
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

    #[doc = "Create employment\n\nCreates an employment. We support creating employees and contractors.\n\nThis endpoint requires and returns country-specific data. The exact required and returned fields will\nvary depending on which country the employment is in. To see the list of parameters for each country,\nsee the **Show form schema** endpoint under the [Countries](#tag/Countries) category.\n\nPlease note that the compliance requirements for each country are subject to change, which will result\nin required and optional parameters being added and deleted without notice.\n\nIf you are using this endpoint to build an integration, make sure you are dynamically collecting or\ndisplaying the latest parameters for each country by querying the _\"Show form schema\"_ endpoint.\n\nFor more information on JSON Schemas, see the **How JSON Schemas work** documentation.\n\nTo learn how you can dynamically generate forms to display in your UI, see the documentation for\nthe [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.\n\n\n\n```rust,no_run\nasync fn example_employments_post_create() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::EmploymentResponse = client\n        .employments()\n        .post_create(&remote_api::types::EmploymentBasicParams {\n            company_id: \"some-string\".to_string(),\n            country_code: \"some-string\".to_string(),\n            full_name: \"some-string\".to_string(),\n            job_title: \"some-string\".to_string(),\n            personal_email: \"email@example.com\".to_string(),\n            provisional_start_date: Some(chrono::Utc::now().date().naive_utc()),\n            type_: remote_api::types::EmploymentBasicParamsType::Employee,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_create<'a>(
        &'a self,
        body: &crate::types::EmploymentBasicParams,
    ) -> Result<crate::types::EmploymentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "v1/employments"),
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

    #[doc = "Show employment\n\nShows all the information of an employment.\n\nThis endpoint requires and returns country-specific data. The exact required and returned fields will\nvary depending on which country the employment is in. To see the list of parameters for each country,\nsee the **Show form schema** endpoint under the [Countries](#tag/Countries) category.\n\nPlease note that the compliance requirements for each country are subject to change, which will result\nin required and optional parameters being added and deleted without notice.\n\nIf you are using this endpoint to build an integration, make sure you are dynamically collecting or\ndisplaying the latest parameters for each country by querying the _\"Show form schema\"_ endpoint.\n\nFor more information on JSON Schemas, see the **How JSON Schemas work** documentation.\n\nTo learn how you can dynamically generate forms to display in your UI, see the documentation for\nthe [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.\n\n\n\n**Parameters:**\n\n- `employment_id: &'astr`: Employment ID (required)\n\n```rust,no_run\nasync fn example_employments_get_show() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::EmploymentResponse =\n        client.employments().get_show(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_show<'a>(
        &'a self,
        employment_id: &'a str,
    ) -> Result<crate::types::EmploymentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/employments/{employment_id}".replace("{employment_id}", employment_id)
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

    #[doc = "Update employment\n\nUpdates an employment.\n\n**For `created` employments:** You can change all basic params and onboarding tasks or perform a per onboarding task update.\n\n**For `active` employments:** At this stage, it is only allowed to update the manager (`manager_id` field).\n\nThis endpoint requires and returns country-specific data. The exact required and returned fields will\nvary depending on which country the employment is in. To see the list of parameters for each country,\nsee the **Show form schema** endpoint under the [Countries](#tag/Countries) category.\n\nPlease note that the compliance requirements for each country are subject to change, which will result\nin required and optional parameters being added and deleted without notice.\n\nIf you are using this endpoint to build an integration, make sure you are dynamically collecting or\ndisplaying the latest parameters for each country by querying the _\"Show form schema\"_ endpoint.\n\nFor more information on JSON Schemas, see the **How JSON Schemas work** documentation.\n\nTo learn how you can dynamically generate forms to display in your UI, see the documentation for\nthe [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.\n\n\nPlease contact Remote if you need to update contractors via API since it's currently not supported.\n\n\n**Parameters:**\n\n- `employment_id: &'astr`: Employment ID (required)\n\n```rust,no_run\nasync fn example_employments_patch_update_2() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::EmploymentResponse = client\n        .employments()\n        .patch_update_2(\n            \"some-string\",\n            &remote_api::types::EmploymentFullParams {\n                address_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                administrative_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                bank_account_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                billing_address_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                company_id: \"some-string\".to_string(),\n                contract_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                country: Some(remote_api::types::Country {\n                    code: \"some-string\".to_string(),\n                    country_subdivisions: Some(vec![remote_api::types::CountrySubdivision {\n                        code: Some(\"some-string\".to_string()),\n                        name: \"some-string\".to_string(),\n                        subdivision_type: Some(\"some-string\".to_string()),\n                    }]),\n                    name: \"some-string\".to_string(),\n                }),\n                emergency_contact_details: Some(serde_json::Value::String(\n                    \"some-string\".to_string(),\n                )),\n                full_name: \"some-string\".to_string(),\n                job_title: \"some-string\".to_string(),\n                manager_id: Some(\"some-string\".to_string()),\n                personal_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                personal_email: \"some-string\".to_string(),\n                pricing_plan_details: Some(remote_api::types::PricingPlanDetails {\n                    frequency: \"some-string\".to_string(),\n                }),\n                provisional_start_date: Some(chrono::Utc::now().date().naive_utc()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_update_2<'a>(
        &'a self,
        employment_id: &'a str,
        body: &crate::types::EmploymentFullParams,
    ) -> Result<crate::types::EmploymentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/employments/{employment_id}".replace("{employment_id}", employment_id)
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

    #[doc = "Update employment\n\nUpdates an employment.\n\n**For `created` employments:** You can change all basic params and onboarding tasks or perform a per onboarding task update.\n\n**For `active` employments:** At this stage, it is only allowed to update the manager (`manager_id` field).\n\nThis endpoint requires and returns country-specific data. The exact required and returned fields will\nvary depending on which country the employment is in. To see the list of parameters for each country,\nsee the **Show form schema** endpoint under the [Countries](#tag/Countries) category.\n\nPlease note that the compliance requirements for each country are subject to change, which will result\nin required and optional parameters being added and deleted without notice.\n\nIf you are using this endpoint to build an integration, make sure you are dynamically collecting or\ndisplaying the latest parameters for each country by querying the _\"Show form schema\"_ endpoint.\n\nFor more information on JSON Schemas, see the **How JSON Schemas work** documentation.\n\nTo learn how you can dynamically generate forms to display in your UI, see the documentation for\nthe [json-schema-form](https://www.notion.so/remotecom/json-schema-form-Documentation-4f390236948b4b2e8b7350ebcd488ca6) tool.\n\n\nPlease contact Remote if you need to update contractors via API since it's currently not supported.\n\n\n**Parameters:**\n\n- `employment_id: &'astr`: Employment ID (required)\n\n```rust,no_run\nasync fn example_employments_patch_update() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::EmploymentResponse = client\n        .employments()\n        .patch_update(\n            \"some-string\",\n            &remote_api::types::EmploymentFullParams {\n                address_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                administrative_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                bank_account_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                billing_address_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                company_id: \"some-string\".to_string(),\n                contract_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                country: Some(remote_api::types::Country {\n                    code: \"some-string\".to_string(),\n                    country_subdivisions: Some(vec![remote_api::types::CountrySubdivision {\n                        code: Some(\"some-string\".to_string()),\n                        name: \"some-string\".to_string(),\n                        subdivision_type: Some(\"some-string\".to_string()),\n                    }]),\n                    name: \"some-string\".to_string(),\n                }),\n                emergency_contact_details: Some(serde_json::Value::String(\n                    \"some-string\".to_string(),\n                )),\n                full_name: \"some-string\".to_string(),\n                job_title: \"some-string\".to_string(),\n                manager_id: Some(\"some-string\".to_string()),\n                personal_details: Some(serde_json::Value::String(\"some-string\".to_string())),\n                personal_email: \"some-string\".to_string(),\n                pricing_plan_details: Some(remote_api::types::PricingPlanDetails {\n                    frequency: \"some-string\".to_string(),\n                }),\n                provisional_start_date: Some(chrono::Utc::now().date().naive_utc()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_update<'a>(
        &'a self,
        employment_id: &'a str,
        body: &crate::types::EmploymentFullParams,
    ) -> Result<crate::types::EmploymentResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/employments/{employment_id}".replace("{employment_id}", employment_id)
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
