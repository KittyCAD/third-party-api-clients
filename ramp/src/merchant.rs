use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Merchant {
    pub client: Client,
}

impl Merchant {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List all the merchants\n\n**Parameters:**\n\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `transaction_from_date: Option<chrono::DateTime<chrono::Utc>>`\n- `transaction_to_date: Option<chrono::DateTime<chrono::Utc>>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_merchant_get_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiMerchantResourceSchema = client\n        .merchant()\n        .get_list_with_pagination(\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(chrono::Utc::now()),\n            Some(chrono::Utc::now()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_merchant_get_list_with_pagination_stream() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let mut merchant = client.merchant();\n    let mut stream = merchant.get_list_with_pagination_stream(\n        Some(4 as i64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_list_with_pagination<'a>(
        &'a self,
        page_size: Option<i64>,
        start: Option<uuid::Uuid>,
        transaction_from_date: Option<chrono::DateTime<chrono::Utc>>,
        transaction_to_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<crate::types::PaginatedResponseApiMerchantResourceSchema, crate::types::error::Error>
    {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "developer/v1/merchants/"),
        );
        req = req.bearer_auth(&self.client.token.read().await.access_token);
        let mut query_params = Vec::new();
        if let Some(p) = page_size {
            query_params.push(("page_size", format!("{}", p)));
        }

        if let Some(p) = start {
            query_params.push(("start", format!("{}", p)));
        }

        if let Some(p) = transaction_from_date {
            query_params.push(("transaction_from_date", format!("{}", p)));
        }

        if let Some(p) = transaction_to_date {
            query_params.push(("transaction_to_date", format!("{}", p)));
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

    #[doc = "List all the merchants\n\n**Parameters:**\n\n- `page_size: Option<i64>`: The number of results to be returned in each page. The value must be between 2 and 10,000. If not specified, the default value 1,000 will be used.\n- `start: Option<uuid::Uuid>`: The ID of the last entity of the previous page, used for pagination to get the next page.\n- `transaction_from_date: Option<chrono::DateTime<chrono::Utc>>`\n- `transaction_to_date: Option<chrono::DateTime<chrono::Utc>>`\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_merchant_get_list_with_pagination() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let result: ramp_api::types::PaginatedResponseApiMerchantResourceSchema = client\n        .merchant()\n        .get_list_with_pagination(\n            Some(4 as i64),\n            Some(uuid::Uuid::from_str(\n                \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n            )?),\n            Some(chrono::Utc::now()),\n            Some(chrono::Utc::now()),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nuse futures_util::TryStreamExt;\nasync fn example_merchant_get_list_with_pagination_stream() -> anyhow::Result<()> {\n    let client =\n        ramp_api::Client::new_from_env(String::from(\"token\"), String::from(\"refresh-token\"));\n    let mut merchant = client.merchant();\n    let mut stream = merchant.get_list_with_pagination_stream(\n        Some(4 as i64),\n        Some(chrono::Utc::now()),\n        Some(chrono::Utc::now()),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub fn get_list_with_pagination_stream<'a>(
        &'a self,
        page_size: Option<i64>,
        transaction_from_date: Option<chrono::DateTime<chrono::Utc>>,
        transaction_to_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> impl futures::Stream<Item = Result<crate::types::Merchant, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self . get_list_with_pagination (page_size , None , transaction_from_date , transaction_to_date) . map_ok (move | result | { let items = futures :: stream :: iter (result . items () . into_iter () . map (Ok)) ; let next_pages = futures :: stream :: try_unfold (result , move | new_result | async move { if new_result . has_more_pages () { async { let mut req = self . client . client . request (http :: Method :: GET , & format ! ("{}/{}" , self . client . base_url , "developer/v1/merchants/") ,) ; req = req . bearer_auth (& self . client . token . read () . await . access_token) ; let mut request = req . build () ? ; request = new_result . next_page (request) ? ; let resp = self . client . client . execute (request) . await ? ; let status = resp . status () ; if status . is_success () { let text = resp . text () . await . unwrap_or_default () ; serde_json :: from_str (& text) . map_err (| err | crate :: types :: error :: Error :: from_serde_error (format_serde_error :: SerdeError :: new (text . to_string () , err) , status) . into ()) } else { Err (crate :: types :: error :: Error :: UnexpectedResponse (resp)) } } . map_ok (| result : crate :: types :: PaginatedResponseApiMerchantResourceSchema | { Some ((futures :: stream :: iter (result . items () . into_iter () . map (Ok) ,) , result ,)) }) . await } else { Ok (None) } }) . try_flatten () ; items . chain (next_pages) }) . try_flatten_stream () . boxed ()
    }
}
