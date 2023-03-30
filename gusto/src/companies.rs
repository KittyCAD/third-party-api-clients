use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Companies {
    pub client: Client,
}

impl Companies {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get a company\n\nGet a company.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: \
             The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn \
             example_companies_get() -> anyhow::Result<()> {\n    let client =\n        \
             gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: gusto_api::types::Company = \
             client.companies().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
    ) -> Result<crate::types::Company, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create a partner managed company (Beta)\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\n### Overview\n\nThe partner managed company API provides a way to create a Gusto company that you can manage. This endpoint behaves similarly to [creating a company](../~1v1~1provision/post) in that it does the following:\n\n* Creates a new company in Gusto.\n* Creates a new user in Gusto.\n* Makes the new user the primary payroll administrator of the new company.\n* Sends a welcome email to the new user.\n\nAdditionally, on successful creation of the company, this API will do the following:\n* Creates a link between the partner and the company.\n* Creates access tokens and refresh tokens that can be used immediately.\n\nIn the response, you will receive the access token, the refresh token, and the uuid of the created company.\n\n### Authentication\n\nDue to the nature of this endpoint, Gusto will provide partners with an API token and will permit partners to use API Token Authentication instead of OAuth to provision Gusto accounts. The API token is included in the authorization HTTP header with the Token scheme, e.g.:\n\n```ignore\nContent-Type: application/json\nAuthorization: Token bbb286ff1a4fe6b84742b0d49b8d0d65bd0208d27d3d50333591df71\n```ignore\n\n```rust,no_run\nasync fn example_companies_post_partner_managed() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::PostPartnerManagedCompaniesResponse = client\n        .companies()\n        .post_partner_managed(&gusto_api::types::PostPartnerManagedCompaniesRequestBody {\n            user: gusto_api::types::User {\n                first_name: \"some-string\".to_string(),\n                last_name: \"some-string\".to_string(),\n                email: \"some-string\".to_string(),\n                phone: Some(\"some-string\".to_string()),\n            },\n            company: gusto_api::types::PostPartnerManagedCompaniesRequestBodyCompany {\n                name: \"some-string\".to_string(),\n                trade_name: Some(\"some-string\".to_string()),\n                ein: Some(\"some-string\".to_string()),\n            },\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_partner_managed<'a>(
        &'a self,
        body: &crate::types::PostPartnerManagedCompaniesRequestBody,
    ) -> Result<crate::types::PostPartnerManagedCompaniesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "v1/partner_managed_companies"
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create a company\n\n### Overview\n\nThe company provisioning API provides a way to create a Gusto company as part of your integration. When you successfully call the API, the API does the following:\n\n* Creates a new company in Gusto.\n* Creates a new user in Gusto.\n* Makes the new user the primary payroll administrator of the new company.\n* Sends a welcome email to the new user.\n\nIn the response, you will receive an account claim URL. Redirect the user to this URL to complete their account setup inside of Gusto\n\n### Authentication\n\nDue to the nature of this endpoint, Gusto will provide partners with an API token and will permit partners to use API Token Authentication instead of OAuth to provision Gusto accounts. The API token is included in the authorization HTTP header with the Token scheme, e.g.:\n\n```ignore\nContent-Type: application/json\nAuthorization: Token bbb286ff1a4fe6b84742b0d49b8d0d65bd0208d27d3d50333591df71\n```ignore\n\n```rust,no_run\nasync fn example_companies_post_provision() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::PostProvisionResponse = client\n        .companies()\n        .post_provision(&gusto_api::types::PostProvisionRequestBody {\n            user: gusto_api::types::User {\n                first_name: \"some-string\".to_string(),\n                last_name: \"some-string\".to_string(),\n                email: \"some-string\".to_string(),\n                phone: Some(\"some-string\".to_string()),\n            },\n            company: gusto_api::types::PostProvisionRequestBodyCompany {\n                name: \"some-string\".to_string(),\n                trade_name: Some(\"some-string\".to_string()),\n                ein: Some(\"some-string\".to_string()),\n                states: Some(vec![\"some-string\".to_string()]),\n                number_employees: Some(3.14 as f64),\n                addresses: Some(vec![\n                    gusto_api::types::PostProvisionRequestBodyCompanyAddresses {\n                        street_1: Some(\"some-string\".to_string()),\n                        street_2: Some(\"some-string\".to_string()),\n                        city: Some(\"some-string\".to_string()),\n                        zip: Some(\"some-string\".to_string()),\n                        state: Some(\"some-string\".to_string()),\n                        phone: Some(\"some-string\".to_string()),\n                        is_primary: Some(\"some-string\".to_string()),\n                    },\n                ]),\n            },\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_provision<'a>(
        &'a self,
        body: &crate::types::PostProvisionRequestBody,
    ) -> Result<crate::types::PostProvisionResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "v1/provision"),
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
