use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct ContractorPayments {
    pub client: Client,
}

impl ContractorPayments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get contractor payments for a company\n\nReturns an object containing individual contractor payments, within a given time period, including totals.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n- `end_date: Option<chrono::NaiveDate>`: The time period for which to retrieve contractor payments\n- `start_date: Option<chrono::NaiveDate>`: The time period for which to retrieve contractor payments\n\n```rust,no_run\nasync fn example_contractor_payments_get_companies_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::ContractorPaymentSummary = client\n        .contractor_payments()\n        .get_companies_company_id(\n            \"some-string\",\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id<'a>(
        &'a self,
        company_id: &'a str,
        end_date: Option<chrono::NaiveDate>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<crate::types::ContractorPaymentSummary, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/contractor_payments"
                    .replace("{company_id}", company_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = end_date {
            query_params.push(("end_date", format!("{}", p)));
        }

        if let Some(p) = start_date {
            query_params.push(("start_date", format!("{}", p)));
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

    #[doc = "Create a contractor payment (Beta)\n\nReturns an object containing individual contractor payments, within a given time period, including totals.\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\n**Parameters:**\n\n- `bonus: Option<f64>`: If the contractor is on an hourly wage, this is the bonus the contractor earned.\n- `company_id: &'astr`: The ID of the company (required)\n- `contractor_id: f64`: The contractor receiving the payment (required)\n- `date: &'astr`: The payment date (required)\n- `hours: Option<f64>`: If the contractor is on an hourly wage, this is the number of hours that the contractor worked for the payment.\n- `reimbursement: Option<f64>`: Reimbursed wages for the contractor .\n- `wage: Option<f64>`: If the contractor is on a fixed wage, this is the fixed wage payment for the contractor, regardless of hours worked.\n\n```rust,no_run\nasync fn example_contractor_payments_post_companies_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::ContractorPayment = client\n        .contractor_payments()\n        .post_companies_company_id(\n            Some(3.14 as f64),\n            \"some-string\",\n            3.14 as f64,\n            \"some-string\",\n            Some(3.14 as f64),\n            Some(3.14 as f64),\n            Some(3.14 as f64),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id<'a>(
        &'a self,
        bonus: Option<f64>,
        company_id: &'a str,
        contractor_id: f64,
        date: &'a str,
        hours: Option<f64>,
        reimbursement: Option<f64>,
        wage: Option<f64>,
    ) -> Result<crate::types::ContractorPayment, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/contractor_payments"
                    .replace("{company_id}", company_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![
            ("contractor_id", format!("{}", contractor_id)),
            ("date", date.to_string()),
        ];
        if let Some(p) = bonus {
            query_params.push(("bonus", format!("{}", p)));
        }

        if let Some(p) = hours {
            query_params.push(("hours", format!("{}", p)));
        }

        if let Some(p) = reimbursement {
            query_params.push(("reimbursement", format!("{}", p)));
        }

        if let Some(p) = wage {
            query_params.push(("wage", format!("{}", p)));
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

    #[doc = "Get a single contractor payment\n\nReturns a single contractor \
             payments\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company \
             (required)\n- `contractor_payment_id_or_uuid: &'astr`: The ID or UUID of the \
             contractor payment (required)\n\n```rust,no_run\nasync fn \
             example_contractor_payments_get_single_companies_company_id() -> anyhow::Result<()> \
             {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             gusto_api::types::ContractorPayment = client\n        .contractor_payments()\n        \
             .get_single_companies_company_id(\"some-string\", \"some-string\")\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_single_companies_company_id<'a>(
        &'a self,
        company_id: &'a str,
        contractor_payment_id_or_uuid: &'a str,
    ) -> Result<crate::types::ContractorPayment, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/contractor_payments/{contractor_payment_id_or_uuid}"
                    .replace("{company_id}", company_id)
                    .replace(
                        "{contractor_payment_id_or_uuid}",
                        contractor_payment_id_or_uuid
                    )
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

    #[doc = "Cancel a contractor payment (Beta)\n\nCancels and deletes a contractor payment. If the contractor payment has already started processing, the payment cannot be cancelled.\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\n**Parameters:**\n\n- `company_id: &'astr`: The ID of the company (required)\n- `contractor_payment_id_or_uuid: &'astr`: The ID or UUID of the contractor payment (required)\n\n```rust,no_run\nasync fn example_contractor_payments_delete_companies_company_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .contractor_payments()\n        .delete_companies_company_id(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_companies_company_id<'a>(
        &'a self,
        company_id: &'a str,
        contractor_payment_id_or_uuid: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/contractor_payments/{contractor_payment_id_or_uuid}"
                    .replace("{company_id}", company_id)
                    .replace(
                        "{contractor_payment_id_or_uuid}",
                        contractor_payment_id_or_uuid
                    )
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
}
