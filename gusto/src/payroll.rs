use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Payroll {
    pub client: Client,
}

impl Payroll {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get pay periods for a company\n\nPay periods are the foundation of payroll. Compensation, time & attendance, taxes, and expense reports all rely on when they happened. To begin submitting information for a given payroll, we need to agree on the time period.\n\n\nBy default, this endpoint returns every current and past pay period for a company. Since companies can process payroll as often as every week, there can be up to 53 pay periods a year. If a company has been running payroll with Gusto for five years, this endpoint could return up to 265 pay periods. Use the `start_date` and `end_date` parameters to reduce the scope of the response.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n- `end_date: Option<chrono::NaiveDate>`\n- `start_date: Option<chrono::NaiveDate>`\n\n```rust,no_run\nasync fn example_payroll_get_companies_company_id_pay_periods() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::PayPeriod> = client\n        .payroll()\n        .get_companies_company_id_pay_periods(\n            \"some-string\",\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_pay_periods<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        end_date: Option<chrono::NaiveDate>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<Vec<crate::types::PayPeriod>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/pay_periods"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get all payrolls for a company\n\nReturns all payrolls, current and past for a company.\n\nNotes:\n* Hour and dollar amounts are returned as string representations of numeric decimals.\n* Hours are represented to the thousands place; dollar amounts are represented to the cent.\n* Every eligible compensation is returned for each employee. If no data has yet be inserted for a given field, it defaults to “0.00” (for fixed amounts) or “0.000” (for hours ).\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n- `end_date: Option<chrono::NaiveDate>`: Return payrolls whose pay period is before the end date\n- `include: Option<Vec<crate::types::GetCompaniesCompanyIdPayrollsInclude>>`: Include the requested attribute in the employee_compensations attribute in the response\n- `include_off_cycle: Option<bool>`: Whether to include off cycle payrolls in the response\n- `processed: Option<bool>`: Whether to return processed or unprocessed payrolls\n- `start_date: Option<chrono::NaiveDate>`: Return payrolls whose pay period is after the start date\n\n```rust,no_run\nasync fn example_payroll_get_companies_company_id_payrolls() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: Vec<gusto_api::types::Payroll> = client\n        .payroll()\n        .get_companies_company_id_payrolls(\n            \"some-string\",\n            Some(chrono::Utc::now().date().naive_utc()),\n            Some(vec![\n                gusto_api::types::GetCompaniesCompanyIdPayrollsInclude::Taxes,\n            ]),\n            Some(false),\n            Some(false),\n            Some(chrono::Utc::now().date().naive_utc()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_payrolls<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        end_date: Option<chrono::NaiveDate>,
        include: Option<Vec<crate::types::GetCompaniesCompanyIdPayrollsInclude>>,
        include_off_cycle: Option<bool>,
        processed: Option<bool>,
        start_date: Option<chrono::NaiveDate>,
    ) -> Result<Vec<crate::types::Payroll>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/payrolls"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = end_date {
            query_params.push(("end_date", format!("{}", p)));
        }

        if let Some(p) = include {
            query_params.push(("include", itertools::join(p, ",")));
        }

        if let Some(p) = include_off_cycle {
            query_params.push(("include_off_cycle", format!("{}", p)));
        }

        if let Some(p) = processed {
            query_params.push(("processed", format!("{}", p)));
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
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create an Off-Cycle Payroll (Beta)\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nCreates a new, unprocessed, off-cycle payroll.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n\n```rust,no_run\nasync fn example_payroll_post_companies_company_id_payrolls() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Payroll = client\n        .payroll()\n        .post_companies_company_id_payrolls(\n            \"some-string\",\n            &gusto_api::types::PostCompaniesCompanyIdPayrollsRequestBody {\n                off_cycle: \"some-string\".to_string(),\n                off_cycle_reason: Some(gusto_api::types::OffCycleReason::Correction),\n                start_date: Some(chrono::Utc::now().date().naive_utc()),\n                end_date: Some(chrono::Utc::now().date().naive_utc()),\n                employee_ids: Some(vec![4 as i64]),\n                check_date: Some(\"some-string\".to_string()),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_companies_company_id_payrolls<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        body: &crate::types::PostCompaniesCompanyIdPayrollsRequestBody,
    ) -> Result<crate::types::Payroll, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/payrolls"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
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

    #[doc = "Get a single payroll\n\nReturns a payroll.\n\nNotes:\n* Hour and dollar amounts are returned as string representations of numeric decimals.\n* Hours are represented to the thousands place; dollar amounts are represented to the cent.\n* Every eligible compensation is returned for each employee. If no data has yet be inserted for a given field, it defaults to “0.00” (for fixed amounts) or “0.000” (for hours ).\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n- `include: Option<crate::types::GetCompaniesCompanyIdPayrollsIdInclude>`: Include the requested attribute in the employee_compensations attribute in the response\n- `payroll_id_or_uuid: &'astr`: The ID or UUID of the payroll  (required)\n- `show_calculation: Option<String>`: with `include`, shows the tax, and/or benefit, and/or deduction details for a calculated, unprocessed payroll. \n\n```rust,no_run\nasync fn example_payroll_get_companies_company_id_payrolls_id() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Payroll = client\n        .payroll()\n        .get_companies_company_id_payrolls_id(\n            \"some-string\",\n            Some(gusto_api::types::GetCompaniesCompanyIdPayrollsIdInclude::Taxes),\n            \"some-string\",\n            Some(\"some-string\".to_string()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_payrolls_id<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        include: Option<crate::types::GetCompaniesCompanyIdPayrollsIdInclude>,
        payroll_id_or_uuid: &'a str,
        show_calculation: Option<String>,
    ) -> Result<crate::types::Payroll, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/payrolls/{payroll_id_or_uuid}"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
                    .replace("{payroll_id_or_uuid}", payroll_id_or_uuid)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = vec![];
        if let Some(p) = include {
            query_params.push(("include", format!("{}", p)));
        }

        if let Some(p) = show_calculation {
            query_params.push(("show_calculation", p));
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

    #[doc = "Update a payroll by ID\n\nThis endpoint allows you to update information for one or more employees for a specific **unprocessed** payroll.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n- `payroll_id_or_uuid: &'astr`: The ID or UUID of the payroll  (required)\n\n```rust,no_run\nasync fn example_payroll_put_companies_company_id_payrolls() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result : gusto_api::types::Payroll = client . payroll () . put_companies_company_id_payrolls (\"some-string\" , \"some-string\" , & gusto_api::types::PutCompaniesCompanyIdPayrollsRequestBody { version : \"some-string\" . to_string () , employee_compensations : vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensations { employee_id : 4 as i64 , fixed_compensations : Some (vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsFixedCompensations { name : Some (\"some-string\" . to_string ()) , amount : Some (\"some-string\" . to_string ()) , job_id : Some (4 as i64) }]) , hourly_compensations : Some (vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsHourlyCompensations { name : Some (\"some-string\" . to_string ()) , hours : Some (\"some-string\" . to_string ()) , job_id : Some (4 as i64) }]) , paid_time_off : Some (vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsPaidTimeOff { name : Some (\"some-string\" . to_string ()) , hours : Some (\"some-string\" . to_string ()) }]) }] }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_payrolls<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        payroll_id_or_uuid: &'a str,
        body: &crate::types::PutCompaniesCompanyIdPayrollsRequestBody,
    ) -> Result<crate::types::Payroll, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/payrolls/{payroll_id_or_uuid}"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
                    .replace("{payroll_id_or_uuid}", payroll_id_or_uuid)
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

    #[doc = "Update a payroll\n\nThis endpoint allows you to update information for one or more employees for a specific **unprocessed** payroll.\n\nThe payrolls are identified by their pay periods’ start_date and end_date. Both are required and must correspond with an existing, unprocessed payroll. *If the dates do not match, the entire request will be rejected.* This was an explicit design decision to remove any assumptions around the timespan for data sent.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr`: The ID or UUID of the company (required)\n- `pay_period_end_date: chrono::NaiveDate`: The end_date of the pay period for the payroll (required)\n- `pay_period_start_date: chrono::NaiveDate`: The start_date of the pay period for the payroll (required)\n\n```rust,no_run\nasync fn example_payroll_put_companies_company_id_payrolls_pay_period_start_date_pay_period_end_date(\n) -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result : gusto_api::types::Payroll = client . payroll () . put_companies_company_id_payrolls_pay_period_start_date_pay_period_end_date (\"some-string\" , chrono :: Utc :: now () . date () . naive_utc () , chrono :: Utc :: now () . date () . naive_utc () , & gusto_api::types::PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBody { version : \"some-string\" . to_string () , employee_compensations : vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensations { employee_id : 4 as i64 , fixed_compensations : Some (vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsFixedCompensations { name : Some (\"some-string\" . to_string ()) , amount : Some (\"some-string\" . to_string ()) , job_id : Some (4 as i64) }]) , hourly_compensations : Some (vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsHourlyCompensations { name : Some (\"some-string\" . to_string ()) , hours : Some (\"some-string\" . to_string ()) , job_id : Some (4 as i64) }]) , paid_time_off : Some (vec ! [gusto_api::types::PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsPaidTimeOff { name : Some (\"some-string\" . to_string ()) , hours : Some (\"some-string\" . to_string ()) }]) }] }) . await ? ;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_payrolls_pay_period_start_date_pay_period_end_date<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
        pay_period_end_date: chrono::NaiveDate,
        pay_period_start_date: chrono::NaiveDate,
        body : & crate :: types :: PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBody,
    ) -> Result<crate::types::Payroll, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/payrolls/{pay_period_start_date}/\
                 {pay_period_end_date}"
                    .replace("{company_id_or_uuid}", company_id_or_uuid)
                    .replace("{pay_period_end_date}", &format!("{}", pay_period_end_date))
                    .replace(
                        "{pay_period_start_date}",
                        &format!("{}", pay_period_start_date)
                    )
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

    #[doc = "Calculate a Payroll (Beta)\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nPerforms calculations for taxes, benefits, and deductions for an unprocessed payroll. The calculated payroll details provide a preview of the actual values that will be used when the payroll is run.\n\nThis endpoint is asynchronous and responds with only a 202 HTTP status. To view the details of the calculated payroll, use the GET /v1/companies/{company_id}/payrolls/{payroll_id} endpoint with the *show_calculation=true* and *include=taxes,benefits,deductions* params\n\n**Parameters:**\n\n- `company_id: &'astr` (required)\n- `payroll_id: &'astr` (required)\n\n```rust,no_run\nasync fn example_payroll_put_companies_company_id_payrolls_id_calculate() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .payroll()\n        .put_companies_company_id_payrolls_id_calculate(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_payrolls_id_calculate<'a>(
        &'a self,
        company_id: &'a str,
        payroll_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/payrolls/{payroll_id}/calculate"
                    .replace("{company_id}", company_id)
                    .replace("{payroll_id}", payroll_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Submit Payroll (Beta)\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nSubmits an unprocessed payroll to be calculated and run. Upon success, transitions the payroll to the `processed` state.\n\n**Parameters:**\n\n- `company_id: &'astr`: Company ID (required)\n- `payroll_id: &'astr`: Payroll ID (required)\n\n```rust,no_run\nasync fn example_payroll_put_companies_company_id_payrolls_id_submit() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    client\n        .payroll()\n        .put_companies_company_id_payrolls_id_submit(\"some-string\", \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_companies_company_id_payrolls_id_submit<'a>(
        &'a self,
        company_id: &'a str,
        payroll_id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/payrolls/{payroll_Id}/submit"
                    .replace("{company_id}", company_id)
                    .replace("{payroll_Id}", payroll_id)
            ),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Cancel a Payroll (Beta)\n\nThis endpoint is in beta and intended for **[Gusto Embedded Payroll](https://gusto.com/embedded-payroll)** customers. Please [apply for early access](https://gusto-embedded-payroll.typeform.com/to/iomAQIj3?utm_source=docs) if you’d like to learn more and use it for production. Note, this endpoint will require you to enter a different agreement with Gusto.\n\nTransitions a `processed` payroll back to the `unprocessed` state. A payroll cannot be canceled once it has entered the `funded` state.\n\n\n**Parameters:**\n\n- `company_id: &'astr`: Company ID (required)\n- `payroll_id: &'astr`: Payroll ID (required)\n\n```rust,no_run\nasync fn example_payroll_put_api_companies_company_id_payrolls_id_cancel() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::Payroll = client\n        .payroll()\n        .put_api_companies_company_id_payrolls_id_cancel(\"some-string\", \"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_api_companies_company_id_payrolls_id_cancel<'a>(
        &'a self,
        company_id: &'a str,
        payroll_id: &'a str,
    ) -> Result<crate::types::Payroll, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id}/payrolls/{payroll_id}/cancel"
                    .replace("{company_id}", company_id)
                    .replace("{payroll_id}", payroll_id)
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

    #[doc = "Get approved Payroll Reversals\n\nReturns all approved Payroll Reversals for a Company.\n\n**Parameters:**\n\n- `company_id_or_uuid: &'astr` (required)\n\n```rust,no_run\nasync fn example_payroll_get_companies_company_id_or_uuid_reversals() -> anyhow::Result<()> {\n    let client =\n        gusto_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: gusto_api::types::GetCompaniesCompanyIdOrUuidPayrollReversalsResponse = client\n        .payroll()\n        .get_companies_company_id_or_uuid_reversals(\"some-string\")\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_companies_company_id_or_uuid_reversals<'a>(
        &'a self,
        company_id_or_uuid: &'a str,
    ) -> Result<
        crate::types::GetCompaniesCompanyIdOrUuidPayrollReversalsResponse,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "v1/companies/{company_id_or_uuid}/payroll_reversals"
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
}
