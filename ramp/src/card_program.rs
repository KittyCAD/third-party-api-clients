use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct CardProgram {
    pub client: Client,
}

impl CardProgram {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Retrieve the card program by ID\n\n**Parameters:**\n\n- `card_program_id: &'astr` \
             (required)\n\n```rust,no_run\nasync fn example_card_program_get_resource() -> \
             anyhow::Result<()> {\n    let client =\n        \
             ramp_api::Client::new_from_env(String::from(\"token\"), \
             String::from(\"refresh-token\"));\n    let result: \
             ramp_api::types::ApiCardProgramResource =\n        \
             client.card_program().get_resource(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_resource<'a>(
        &'a self,
        card_program_id: &'a str,
    ) -> Result<crate::types::ApiCardProgramResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "developer/v1/card-programs/{card_program_id}"
                    .replace("{card_program_id}", &card_program_id)
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List all the card programs\n\n**Parameters:**\n\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_program_get_list() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiCardProgramResourceSchema = client\n        .card_program()\n        .get_list(\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_card_program_get_list_stream() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let mut card_program = client.card_program();\n    let mut stream = card_program.get_list_stream(Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list<'a>(
        &'a self,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
    ) -> Result<
        crate::types::PaginatedResponseApiCardProgramResourceSchema,
        crate::types::error::Error,
    > {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "developer/v1/card-programs/"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = Vec::new();
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List all the card programs\n\n**Parameters:**\n\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_program_get_list() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiCardProgramResourceSchema = client\n        .card_program()\n        .get_list(\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_card_program_get_list_stream() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let mut card_program = client.card_program();\n    let mut stream = card_program.get_list_stream(Some(4 as i64));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub fn get_list_stream<'a>(
        &'a self,
        page_size: Option<i64>,
    ) -> impl futures::Stream<
        Item = Result<crate::types::ApiCardProgramResource, crate::types::error::Error>,
    > + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self . get_list (page_size , None) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold (result , move | new_result | async move { if new_result . has_more_pages () { async { let mut req = self . client . client . request (http :: Method :: GET , & format ! ("{}/{}" , self . client . base_url , "developer/v1/card-programs/") ,) ; req = req . bearer_auth (& self . client . token . read () . await . access_token) ; let mut request = req . build () ? ; request = new_result . next_page (request) ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (format_serde_error :: SerdeError :: new (text . to_string () , err) , status) . into ()) } else { Err (crate :: types :: error :: Error :: UnexpectedResponse (resp)) } } . map_ok (| result : crate :: types :: PaginatedResponseApiCardProgramResourceSchema | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , result ,)) }) . await } else { Ok (None) } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () . boxed ()
    }

    #[doc = "Create a new card program\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_card_program_post_list() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::ApiCardProgramResource = client\n        .card_program()\n        .post_list(&ramp_api::types::ApiCardProgramCreate {\n            acting_user_id: 4 as i64,\n            business_id: 4 as i64,\n            is_default: true,\n            is_physical: true,\n            spending_restrictions: ramp_api::types::ApiCardSpendingRestrictionsLoad {\n                categories: Some(vec![4 as i64]),\n                amount: 3.14 as f64,\n                policy_id: Some(\"some-string\".to_string()),\n                vendor_whitelist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                blocked_mcc_codes: Some(vec![\"some-string\".to_string()]),\n                categories_blacklist: Some(vec![4 as i64]),\n                card_accounting_rules: Some(vec![ramp_api::types::ApiCardAccountingRulesData {\n                    accounting_provider_access_uuid: Some(\"some-string\".to_string()),\n                    tracking_category_id: 4 as i64,\n                    tracking_category_option_id: 4 as i64,\n                    tracking_category_option_remote_name: \"some-string\".to_string(),\n                }]),\n                vendor_blacklist: Some(vec![uuid::Uuid::from_str(\n                    \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n                )?]),\n                lock_date: Some(chrono::Utc::now()),\n                categories_whitelist: Some(vec![4 as i64]),\n                transaction_amount_limit: Some(3.14 as f64),\n                interval: ramp_api::types::Interval::Total,\n            },\n            policy_id: 4 as i64,\n            display_name: \"some-string\".to_string(),\n            description: \"some-string\".to_string(),\n            icon: Some(ramp_api::types::Icon::TravelExpensesIcon),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn post_list<'a>(
        &'a self,
        body: &crate::types::ApiCardProgramCreate,
    ) -> Result<crate::types::ApiCardProgramResource, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "developer/v1/card-programs/"),
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}
