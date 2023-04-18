use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Countries {
    pub client: Client,
}

impl Countries {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List countries\n\nReturns a list of all countries that are supported by Remote API alphabetically ordered.\n\n```rust,no_run\nasync fn example_countries_get_supported_country() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::CountriesResponse =\n        client.countries().get_supported_country().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_supported_country<'a>(
        &'a self,
    ) -> Result<crate::types::CountriesResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "v1/countries"),
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

    #[doc = "List all holidays of a country\n\nList all holidays of a country for a specific year. \
             Optionally, it can be filtered by country subdivision.\n\n**Parameters:**\n\n- \
             `country_code: &'astr`: Country code according to ISO 3166-1 3-digit alphabetic codes \
             (required)\n- `country_subdivision_code: Option<String>`: Country subdivision code \
             according to ISO 3166-2 codes\n- `year: &'astr`: Year for the holidays \
             (required)\n\n```rust,no_run\nasync fn example_countries_get_index_holiday() -> \
             anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let \
             result: remote_api::types::HolidaysResponse = client\n        .countries()\n        \
             .get_index_holiday(\n            \"some-string\",\n            \
             Some(\"some-string\".to_string()),\n            \"some-string\",\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_index_holiday<'a>(
        &'a self,
        country_code: &'a str,
        country_subdivision_code: Option<String>,
        year: &'a str,
    ) -> Result<crate::types::HolidaysResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/countries/{country_code}/holidays/{year}"
                    .replace("{country_code}", country_code)
                    .replace("{year}", year)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = country_subdivision_code {
            query_params.push(("country_subdivision_code", p));
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

    #[doc = "Show form schema\n\nReturns the json schema of a supported form. Possible form names are:\n\n    - address_details\n    - administrative_details\n    - bank_account_details\n    - billing_address_details\n    - contract_details\n    - emergency_contact_details\n    - employment_document_details\n    - personal_details\n    - pricing_plan_details\n\n    \n\n**Parameters:**\n\n- `country_code: &'astr`: Country code according to ISO 3-digit alphabetic codes (required)\n- `form: &'astr`: Name of the desired form (required)\n\n```rust,no_run\nasync fn example_countries_get_show_form_country() -> anyhow::Result<()> {\n    let client = remote_api::Client::new_from_env();\n    let result: remote_api::types::CountryFormResponse = client\n        .countries()\n        .get_show_form_country(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_show_form_country<'a>(
        &'a self,
        country_code: &'a str,
        form: &'a str,
    ) -> Result<crate::types::CountryFormResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/countries/{country_code}/{form}"
                    .replace("{country_code}", country_code)
                    .replace("{form}", form)
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
