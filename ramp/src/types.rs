#![doc = r" This module contains the generated types for the library."]
#[cfg(feature = "tabled")]
use tabled::Tabled;
pub mod base64 {
    #![doc = " Base64 data that encodes to url safe base64, but can decode from multiple"]
    #![doc = " base64 implementations to account for various clients and libraries. Compatible"]
    #![doc = " with serde and JsonSchema."]
    use std::{convert::TryFrom, fmt};

    use serde::{
        de::{Error, Unexpected, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };
    static ALLOWED_DECODING_FORMATS: &[data_encoding::Encoding] = &[
        data_encoding::BASE64,
        data_encoding::BASE64URL,
        data_encoding::BASE64URL_NOPAD,
        data_encoding::BASE64_MIME,
        data_encoding::BASE64_NOPAD,
    ];
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = " A container for binary that should be base64 encoded in serialisation. In reverse"]
    #[doc = " when deserializing, will decode from many different types of base64 possible."]
    pub struct Base64Data(pub Vec<u8>);
    impl Base64Data {
        #[doc = " Return is the data is empty."]
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

    impl fmt::Display for Base64Data {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", data_encoding::BASE64URL_NOPAD.encode(&self.0))
        }
    }

    impl From<Base64Data> for Vec<u8> {
        fn from(data: Base64Data) -> Vec<u8> {
            data.0
        }
    }

    impl From<Vec<u8>> for Base64Data {
        fn from(data: Vec<u8>) -> Base64Data {
            Base64Data(data)
        }
    }

    impl AsRef<[u8]> for Base64Data {
        fn as_ref(&self) -> &[u8] {
            &self.0
        }
    }

    impl TryFrom<&str> for Base64Data {
        type Error = anyhow::Error;
        fn try_from(v: &str) -> Result<Self, Self::Error> {
            for config in ALLOWED_DECODING_FORMATS {
                if let Ok(data) = config.decode(v.as_bytes()) {
                    return Ok(Base64Data(data));
                }
            }
            anyhow::bail!("Could not decode base64 data: {}", v);
        }
    }

    struct Base64DataVisitor;
    impl<'de> Visitor<'de> for Base64DataVisitor {
        type Value = Base64Data;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "a base64 encoded string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            for config in ALLOWED_DECODING_FORMATS {
                if let Ok(data) = config.decode(v.as_bytes()) {
                    return Ok(Base64Data(data));
                }
            }
            Err(serde::de::Error::invalid_value(Unexpected::Str(v), &self))
        }
    }

    impl<'de> Deserialize<'de> for Base64Data {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(Base64DataVisitor)
        }
    }

    impl Serialize for Base64Data {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let encoded = data_encoding::BASE64URL_NOPAD.encode(&self.0);
            serializer.serialize_str(&encoded)
        }
    }

    impl schemars::JsonSchema for Base64Data {
        fn schema_name() -> String {
            "Base64Data".to_string()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            let mut obj = gen.root_schema_for::<String>().schema;
            obj.format = Some("byte".to_string());
            schemars::schema::Schema::Object(obj)
        }

        fn is_referenceable() -> bool {
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use std::convert::TryFrom;

        use super::Base64Data;
        #[test]
        fn test_base64_try_from() {
            assert!(Base64Data::try_from("aGVsbG8=").is_ok());
            assert!(Base64Data::try_from("abcdefghij").is_err());
        }
    }
}

pub mod paginate {
    #![doc = " Utility functions used for pagination."]
    use anyhow::Result;
    #[doc = " A trait for types that allow pagination."]
    pub trait Pagination {
        #[doc = " The item that is paginated."]
        type Item: serde::de::DeserializeOwned;
        #[doc = " Returns true if the response has more pages."]
        fn has_more_pages(&self) -> bool;
        #[doc = " Modify a request to get the next page."]
        fn next_page(
            &self,
            req: reqwest::Request,
        ) -> Result<reqwest::Request, crate::types::error::Error>;
        #[doc = " Get the items from a page."]
        fn items(&self) -> Vec<Self::Item>;
    }
}

pub mod phone_number {
    #![doc = " A library to implement phone numbers for our database and JSON serialization and \
              deserialization."]
    use std::str::FromStr;

    use schemars::JsonSchema;
    #[doc = " A phone number."]
    #[derive(Debug, Default, Clone, PartialEq, Hash, Eq)]
    pub struct PhoneNumber(pub Option<phonenumber::PhoneNumber>);
    impl From<phonenumber::PhoneNumber> for PhoneNumber {
        fn from(id: phonenumber::PhoneNumber) -> PhoneNumber {
            PhoneNumber(Some(id))
        }
    }

    impl AsRef<Option<phonenumber::PhoneNumber>> for PhoneNumber {
        fn as_ref(&self) -> &Option<phonenumber::PhoneNumber> {
            &self.0
        }
    }

    impl std::ops::Deref for PhoneNumber {
        type Target = Option<phonenumber::PhoneNumber>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl serde::ser::Serialize for PhoneNumber {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::ser::Serializer,
        {
            serializer.serialize_str(&self.to_string())
        }
    }

    impl<'de> serde::de::Deserialize<'de> for PhoneNumber {
        fn deserialize<D>(deserializer: D) -> Result<PhoneNumber, D::Error>
        where
            D: serde::de::Deserializer<'de>,
        {
            let s = String::deserialize(deserializer).unwrap_or_default();
            PhoneNumber::from_str(&s).map_err(serde::de::Error::custom)
        }
    }

    impl std::str::FromStr for PhoneNumber {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.trim().is_empty() {
                return Ok(PhoneNumber(None));
            }
            let s = if !s.trim().starts_with('+') {
                format!("+1{s}")
            } else {
                s.to_string()
            }
            .replace(['-', '(', ')', ' '], "");
            Ok(PhoneNumber(Some(phonenumber::parse(None, &s).map_err(
                |e| anyhow::anyhow!("invalid phone number `{}`: {}", s, e),
            )?)))
        }
    }

    impl std::fmt::Display for PhoneNumber {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = if let Some(phone) = &self.0 {
                phone
                    .format()
                    .mode(phonenumber::Mode::International)
                    .to_string()
            } else {
                String::new()
            };
            write!(f, "{}", s)
        }
    }

    impl JsonSchema for PhoneNumber {
        fn schema_name() -> String {
            "PhoneNumber".to_string()
        }

        fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            let mut obj = gen.root_schema_for::<String>().schema;
            obj.format = Some("phone".to_string());
            schemars::schema::Schema::Object(obj)
        }

        fn is_referenceable() -> bool {
            false
        }
    }

    #[cfg(test)]
    mod test {
        use pretty_assertions::assert_eq;

        use super::PhoneNumber;
        #[test]
        fn test_parse_phone_number() {
            let mut phone = "+1-555-555-5555";
            let mut phone_parsed: PhoneNumber =
                serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            let mut expected = PhoneNumber(Some(phonenumber::parse(None, phone).unwrap()));
            assert_eq!(phone_parsed, expected);
            let mut expected_str = "+1 555-555-5555";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "555-555-5555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "+1 555-555-5555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "5555555555";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "(510) 864-1234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            expected = PhoneNumber(Some(phonenumber::parse(None, "+15108641234").unwrap()));
            assert_eq!(phone_parsed, expected);
            expected_str = "+1 510-864-1234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "(510)8641234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, expected);
            expected_str = "+1 510-864-1234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
            phone = "";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            assert_eq!(phone_parsed, PhoneNumber(None));
            assert_eq!("", serde_json::json!(phone_parsed));
            phone = "+49 30  1234 1234";
            phone_parsed = serde_json::from_str(&format!(r#""{}""#, phone)).unwrap();
            expected = PhoneNumber(Some(phonenumber::parse(None, phone).unwrap()));
            assert_eq!(phone_parsed, expected);
            expected_str = "+49 30 12341234";
            assert_eq!(expected_str, serde_json::json!(phone_parsed));
        }
    }
}

pub mod error {
    #![doc = " Error methods."]
    #[doc = " Error produced by generated client methods."]
    pub enum Error {
        #[doc = " The request did not conform to API requirements."]
        InvalidRequest(String),
        #[doc = " A server error either due to the data, or with the connection."]
        CommunicationError(reqwest_middleware::Error),
        #[doc = " A request error, caused when building the request."]
        RequestError(reqwest::Error),
        #[doc = " An expected response whose deserialization failed."]
        SerdeError {
            #[doc = " The error."]
            error: format_serde_error::SerdeError,
            #[doc = " The response status."]
            status: reqwest::StatusCode,
        },
        #[doc = " An expected error response."]
        InvalidResponsePayload {
            #[doc = " The error."]
            error: reqwest_middleware::Error,
            #[doc = " The full response."]
            response: reqwest::Response,
        },
        #[doc = " A response not listed in the API description. This may represent a"]
        #[doc = " success or failure response; check `status().is_success()`."]
        UnexpectedResponse(reqwest::Response),
    }

    impl Error {
        #[doc = " Returns the status code, if the error was generated from a response."]
        pub fn status(&self) -> Option<reqwest::StatusCode> {
            match self {
                Error::InvalidRequest(_) => None,
                Error::RequestError(e) => e.status(),
                Error::CommunicationError(reqwest_middleware::Error::Reqwest(e)) => e.status(),
                Error::CommunicationError(reqwest_middleware::Error::Middleware(_)) => None,
                Error::SerdeError { error: _, status } => Some(*status),
                Error::InvalidResponsePayload { error: _, response } => Some(response.status()),
                Error::UnexpectedResponse(r) => Some(r.status()),
            }
        }

        #[doc = " Creates a new error from a response status and a serde error."]
        pub fn from_serde_error(
            e: format_serde_error::SerdeError,
            status: reqwest::StatusCode,
        ) -> Self {
            Self::SerdeError { error: e, status }
        }
    }

    impl From<reqwest_middleware::Error> for Error {
        fn from(e: reqwest_middleware::Error) -> Self {
            Self::CommunicationError(e)
        }
    }

    impl From<reqwest::Error> for Error {
        fn from(e: reqwest::Error) -> Self {
            Self::RequestError(e)
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::InvalidRequest(s) => {
                    write!(f, "Invalid Request: {}", s)
                }
                Error::CommunicationError(e) => {
                    write!(f, "Communication Error: {}", e)
                }
                Error::RequestError(e) => {
                    write!(f, "Request Error: {}", e)
                }
                Error::SerdeError { error, status: _ } => {
                    write!(f, "Serde Error: {}", error)
                }
                Error::InvalidResponsePayload { error, response: _ } => {
                    write!(f, "Invalid Response Payload: {}", error)
                }
                Error::UnexpectedResponse(r) => {
                    write!(f, "Unexpected Response: {:?}", r)
                }
            }
        }
    }

    trait ErrorFormat {
        fn fmt_info(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(self, f)
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                Error::CommunicationError(e) => Some(e),
                Error::SerdeError { error, status: _ } => Some(error),
                Error::InvalidResponsePayload { error, response: _ } => Some(error),
                _ => None,
            }
        }
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct NestedPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<uuid::Uuid>,
}

impl std::fmt::Display for NestedPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NestedPage {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.next).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["next".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Business {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_on_card: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_integrated_with_slack: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_reimbursements_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_approved_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_legal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_sso: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CardShippingAddress>,
}

impl std::fmt::Display for Business {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Business {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(website) = &self.website {
                format!("{:?}", website).into()
            } else {
                String::new().into()
            },
            if let Some(created_time) = &self.created_time {
                format!("{:?}", created_time).into()
            } else {
                String::new().into()
            },
            if let Some(limit_locked) = &self.limit_locked {
                format!("{:?}", limit_locked).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_on_card) = &self.business_name_on_card {
                format!("{:?}", business_name_on_card).into()
            } else {
                String::new().into()
            },
            if let Some(is_integrated_with_slack) = &self.is_integrated_with_slack {
                format!("{:?}", is_integrated_with_slack).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(is_reimbursements_enabled) = &self.is_reimbursements_enabled {
                format!("{:?}", is_reimbursements_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(initial_approved_limit) = &self.initial_approved_limit {
                format!("{:?}", initial_approved_limit).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_legal) = &self.business_name_legal {
                format!("{:?}", business_name_legal).into()
            } else {
                String::new().into()
            },
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
            } else {
                String::new().into()
            },
            if let Some(enforce_sso) = &self.enforce_sso {
                format!("{:?}", enforce_sso).into()
            } else {
                String::new().into()
            },
            if let Some(billing_address) = &self.billing_address {
                format!("{:?}", billing_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "website".into(),
            "created_time".into(),
            "limit_locked".into(),
            "phone".into(),
            "business_name_on_card".into(),
            "is_integrated_with_slack".into(),
            "id".into(),
            "is_reimbursements_enabled".into(),
            "initial_approved_limit".into(),
            "business_name_legal".into(),
            "active".into(),
            "enforce_sso".into(),
            "billing_address".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BusinessBalance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_balance_including_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_including_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_balance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_flex_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_balance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_balance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_card_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub float_balance_excluding_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_balance_excluding_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev_billing_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_limit: Option<f64>,
}

impl std::fmt::Display for BusinessBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BusinessBalance {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(card_balance_including_pending) = &self.card_balance_including_pending {
                format!("{:?}", card_balance_including_pending).into()
            } else {
                String::new().into()
            },
            if let Some(balance_including_pending) = &self.balance_including_pending {
                format!("{:?}", balance_including_pending).into()
            } else {
                String::new().into()
            },
            if let Some(statement_balance) = &self.statement_balance {
                format!("{:?}", statement_balance).into()
            } else {
                String::new().into()
            },
            if let Some(available_flex_limit) = &self.available_flex_limit {
                format!("{:?}", available_flex_limit).into()
            } else {
                String::new().into()
            },
            if let Some(flex_balance) = &self.flex_balance {
                format!("{:?}", flex_balance).into()
            } else {
                String::new().into()
            },
            if let Some(max_balance) = &self.max_balance {
                format!("{:?}", max_balance).into()
            } else {
                String::new().into()
            },
            if let Some(flex_limit) = &self.flex_limit {
                format!("{:?}", flex_limit).into()
            } else {
                String::new().into()
            },
            if let Some(card_limit) = &self.card_limit {
                format!("{:?}", card_limit).into()
            } else {
                String::new().into()
            },
            if let Some(available_card_limit) = &self.available_card_limit {
                format!("{:?}", available_card_limit).into()
            } else {
                String::new().into()
            },
            if let Some(next_billing_date) = &self.next_billing_date {
                format!("{:?}", next_billing_date).into()
            } else {
                String::new().into()
            },
            if let Some(float_balance_excluding_pending) = &self.float_balance_excluding_pending {
                format!("{:?}", float_balance_excluding_pending).into()
            } else {
                String::new().into()
            },
            if let Some(card_balance_excluding_pending) = &self.card_balance_excluding_pending {
                format!("{:?}", card_balance_excluding_pending).into()
            } else {
                String::new().into()
            },
            if let Some(prev_billing_date) = &self.prev_billing_date {
                format!("{:?}", prev_billing_date).into()
            } else {
                String::new().into()
            },
            if let Some(global_limit) = &self.global_limit {
                format!("{:?}", global_limit).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "card_balance_including_pending".into(),
            "balance_including_pending".into(),
            "statement_balance".into(),
            "available_flex_limit".into(),
            "flex_balance".into(),
            "max_balance".into(),
            "flex_limit".into(),
            "card_limit".into(),
            "available_card_limit".into(),
            "next_billing_date".into(),
            "float_balance_excluding_pending".into(),
            "card_balance_excluding_pending".into(),
            "prev_billing_date".into(),
            "global_limit".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardShippingAddress {
    pub city: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "address2", default, skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub last_name: String,
    #[serde(rename = "address1")]
    pub address_1: String,
    pub first_name: String,
}

impl std::fmt::Display for CardShippingAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardShippingAddress {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.city.clone().into(),
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(address_2) = &self.address_2 {
                format!("{:?}", address_2).into()
            } else {
                String::new().into()
            },
            self.postal_code.clone().into(),
            self.country.clone().into(),
            self.last_name.clone().into(),
            self.address_1.clone().into(),
            self.first_name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "city".into(),
            "state".into(),
            "phone".into(),
            "address_2".into(),
            "postal_code".into(),
            "country".into(),
            "last_name".into(),
            "address_1".into(),
            "first_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum RecipientAddressVerificationState {
    #[serde(rename = "NOT_VERIFIED")]
    #[display("NOT_VERIFIED")]
    NotVerified,
    #[serde(rename = "OVERRIDEN")]
    #[display("OVERRIDEN")]
    Overriden,
    #[serde(rename = "VERIFIED")]
    #[display("VERIFIED")]
    Verified,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardShipping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<CardShippingAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_address: Option<CardShippingAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_address_verification_state: Option<RecipientAddressVerificationState>,
}

impl std::fmt::Display for CardShipping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardShipping {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(return_address) = &self.return_address {
                format!("{:?}", return_address).into()
            } else {
                String::new().into()
            },
            if let Some(method) = &self.method {
                format!("{:?}", method).into()
            } else {
                String::new().into()
            },
            if let Some(recipient_address) = &self.recipient_address {
                format!("{:?}", recipient_address).into()
            } else {
                String::new().into()
            },
            if let Some(recipient_address_verification_state) =
                &self.recipient_address_verification_state
            {
                format!("{:?}", recipient_address_verification_state).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "return_address".into(),
            "method".into(),
            "recipient_address".into(),
            "recipient_address_verification_state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardPersonalizationNameLine {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for CardPersonalizationNameLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardPersonalizationNameLine {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(value) = &self.value {
            format!("{:?}", value).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["value".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardPersonalizationText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_line_2: Option<CardPersonalizationNameLine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_line_1: Option<CardPersonalizationNameLine>,
}

impl std::fmt::Display for CardPersonalizationText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardPersonalizationText {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name_line_2) = &self.name_line_2 {
                format!("{:?}", name_line_2).into()
            } else {
                String::new().into()
            },
            if let Some(name_line_1) = &self.name_line_1 {
                format!("{:?}", name_line_1).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name_line_2".into(), "name_line_1".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardPersonalization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<CardPersonalizationText>,
}

impl std::fmt::Display for CardPersonalization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardPersonalization {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(text) = &self.text {
            format!("{:?}", text).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["text".into()]
    }
}

#[doc = "Fulfillment status of the card"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FulfillmentStatus {
    #[serde(rename = "DELIVERED")]
    #[display("DELIVERED")]
    Delivered,
    #[serde(rename = "DIGITALLY_PRESENTED")]
    #[display("DIGITALLY_PRESENTED")]
    DigitallyPresented,
    #[serde(rename = "ISSUED")]
    #[display("ISSUED")]
    Issued,
    #[serde(rename = "ORDERED")]
    #[display("ORDERED")]
    Ordered,
    #[serde(rename = "REJECTED")]
    #[display("REJECTED")]
    Rejected,
    #[serde(rename = "SHIPPED")]
    #[display("SHIPPED")]
    Shipped,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardFulfillment {
    #[doc = "Tracking url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_url: Option<String>,
    #[doc = "Estimated arrival time, presented in ISO8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_eta: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Fulfillment status of the card"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<FulfillmentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CardShipping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_personalization: Option<CardPersonalization>,
    #[doc = "Date on which the card is shipped out, presented in ISO8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for ApiCardFulfillment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardFulfillment {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(shipping_tracking_url) = &self.shipping_tracking_url {
                format!("{:?}", shipping_tracking_url).into()
            } else {
                String::new().into()
            },
            if let Some(shipping_eta) = &self.shipping_eta {
                format!("{:?}", shipping_eta).into()
            } else {
                String::new().into()
            },
            if let Some(fulfillment_status) = &self.fulfillment_status {
                format!("{:?}", fulfillment_status).into()
            } else {
                String::new().into()
            },
            if let Some(shipping) = &self.shipping {
                format!("{:?}", shipping).into()
            } else {
                String::new().into()
            },
            if let Some(card_personalization) = &self.card_personalization {
                format!("{:?}", card_personalization).into()
            } else {
                String::new().into()
            },
            if let Some(shipping_date) = &self.shipping_date {
                format!("{:?}", shipping_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "shipping_tracking_url".into(),
            "shipping_eta".into(),
            "fulfillment_status".into(),
            "shipping".into(),
            "card_personalization".into(),
            "shipping_date".into(),
        ]
    }
}

#[doc = "Time interval to apply limit to."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Interval {
    #[serde(rename = "ANNUAL")]
    #[display("ANNUAL")]
    Annual,
    #[serde(rename = "DAILY")]
    #[display("DAILY")]
    Daily,
    #[serde(rename = "MONTHLY")]
    #[display("MONTHLY")]
    Monthly,
    #[serde(rename = "QUARTERLY")]
    #[display("QUARTERLY")]
    Quarterly,
    #[serde(rename = "TERTIARY")]
    #[display("TERTIARY")]
    Tertiary,
    #[serde(rename = "TOTAL")]
    #[display("TOTAL")]
    Total,
    #[serde(rename = "WEEKLY")]
    #[display("WEEKLY")]
    Weekly,
    #[serde(rename = "YEARLY")]
    #[display("YEARLY")]
    Yearly,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardSpendingRestrictionsDump {
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[doc = "Date to automatically lock the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Amount limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
    #[doc = "Time interval to apply limit to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    #[doc = "List of Ramp Category Codes blocked for this card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<i64>>,
}

impl std::fmt::Display for ApiCardSpendingRestrictionsDump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardSpendingRestrictionsDump {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            if let Some(auto_lock_date) = &self.auto_lock_date {
                format!("{:?}", auto_lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(suspended) = &self.suspended {
                format!("{:?}", suspended).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_categories) = &self.blocked_categories {
                format!("{:?}", blocked_categories).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "categories".into(),
            "auto_lock_date".into(),
            "amount".into(),
            "suspended".into(),
            "transaction_amount_limit".into(),
            "interval".into(),
            "blocked_categories".into(),
        ]
    }
}

#[doc = "State of the card"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum State {
    #[serde(rename = "ACTIVE")]
    #[display("ACTIVE")]
    Active,
    #[serde(rename = "SUSPENDED")]
    #[display("SUSPENDED")]
    Suspended,
    #[serde(rename = "TERMINATED")]
    #[display("TERMINATED")]
    Terminated,
    #[serde(rename = "UNACTIVATED")]
    #[display("UNACTIVATED")]
    Unactivated,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Card {
    #[doc = "Fulfillment details of a physical Ramp card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<ApiCardFulfillment>,
    #[doc = "State of the card"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    #[doc = "Card holder's full name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_program_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the card holder."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_id: Option<uuid::Uuid>,
    #[doc = "Whether the card has overridden the default settings from its card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_program_overridden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_physical: Option<bool>,
    #[doc = "Specifies the spend restrictions on a Ramp card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiCardSpendingRestrictionsDump>,
    pub last_four: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Card {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(fulfillment) = &self.fulfillment {
                format!("{:?}", fulfillment).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(cardholder_name) = &self.cardholder_name {
                format!("{:?}", cardholder_name).into()
            } else {
                String::new().into()
            },
            if let Some(card_program_id) = &self.card_program_id {
                format!("{:?}", card_program_id).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(cardholder_id) = &self.cardholder_id {
                format!("{:?}", cardholder_id).into()
            } else {
                String::new().into()
            },
            if let Some(has_program_overridden) = &self.has_program_overridden {
                format!("{:?}", has_program_overridden).into()
            } else {
                String::new().into()
            },
            if let Some(is_physical) = &self.is_physical {
                format!("{:?}", is_physical).into()
            } else {
                String::new().into()
            },
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
            } else {
                String::new().into()
            },
            self.last_four.clone().into(),
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "fulfillment".into(),
            "state".into(),
            "cardholder_name".into(),
            "card_program_id".into(),
            "id".into(),
            "cardholder_id".into(),
            "has_program_overridden".into(),
            "is_physical".into(),
            "spending_restrictions".into(),
            "last_four".into(),
            "display_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardAccountingRulesData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_provider_access_uuid: Option<String>,
    pub tracking_category_id: i64,
    pub tracking_category_option_id: i64,
    pub tracking_category_option_remote_name: String,
}

impl std::fmt::Display for ApiCardAccountingRulesData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardAccountingRulesData {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_provider_access_uuid) = &self.accounting_provider_access_uuid {
                format!("{:?}", accounting_provider_access_uuid).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.tracking_category_id).into(),
            format!("{:?}", self.tracking_category_option_id).into(),
            self.tracking_category_option_remote_name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_provider_access_uuid".into(),
            "tracking_category_id".into(),
            "tracking_category_option_id".into(),
            "tracking_category_option_remote_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardSpendingRestrictionsUpdate {
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[doc = "Amount limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_whitelist: Option<Vec<uuid::Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_mcc_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_blacklist: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_accounting_rules: Option<Vec<ApiCardAccountingRulesData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_blacklist: Option<Vec<uuid::Uuid>>,
    #[doc = "Date to automatically lock the card. If lock date has passed, set to a future date \
             or to null to unlock the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_whitelist: Option<Vec<i64>>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
    #[doc = "Time interval to apply limit to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
}

impl std::fmt::Display for ApiCardSpendingRestrictionsUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardSpendingRestrictionsUpdate {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(policy_id) = &self.policy_id {
                format!("{:?}", policy_id).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_whitelist) = &self.vendor_whitelist {
                format!("{:?}", vendor_whitelist).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_mcc_codes) = &self.blocked_mcc_codes {
                format!("{:?}", blocked_mcc_codes).into()
            } else {
                String::new().into()
            },
            if let Some(categories_blacklist) = &self.categories_blacklist {
                format!("{:?}", categories_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(card_accounting_rules) = &self.card_accounting_rules {
                format!("{:?}", card_accounting_rules).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_blacklist) = &self.vendor_blacklist {
                format!("{:?}", vendor_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(lock_date) = &self.lock_date {
                format!("{:?}", lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(categories_whitelist) = &self.categories_whitelist {
                format!("{:?}", categories_whitelist).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "categories".into(),
            "amount".into(),
            "policy_id".into(),
            "vendor_whitelist".into(),
            "blocked_mcc_codes".into(),
            "categories_blacklist".into(),
            "card_accounting_rules".into(),
            "vendor_blacklist".into(),
            "lock_date".into(),
            "categories_whitelist".into(),
            "transaction_amount_limit".into(),
            "interval".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardUpdate {
    #[doc = "Specify a card program to link with.\nThis will override the card's spending \
             restrictions with those of the card program.\nPass card_program_id = None to detach \
             the card's current card program.\n\nIf the card_program_id field is specified, then \
             the card program's changes will override any other changes.\nFor example, if both \
             spending_restrictions and card_program_id are passed, then the new spending \
             restrictions\nwill match those of the card program (not the passed spending \
             restrictions).\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_program_id: Option<uuid::Uuid>,
    #[doc = "Cosmetic display name of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Flag to set to enable or disable notifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_notifications_enabled: Option<bool>,
    #[doc = "Modify spending restrictions. Only the fields to be modified need to be passed (so \
             fields that will stay the same do not have to be passed)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiCardSpendingRestrictionsUpdate>,
}

impl std::fmt::Display for ApiCardUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardUpdate {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(card_program_id) = &self.card_program_id {
                format!("{:?}", card_program_id).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(has_notifications_enabled) = &self.has_notifications_enabled {
                format!("{:?}", has_notifications_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "card_program_id".into(),
            "display_name".into(),
            "has_notifications_enabled".into(),
            "spending_restrictions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeferredTaskUUID {
    #[doc = "ID of the deferred task."]
    pub id: uuid::Uuid,
}

impl std::fmt::Display for DeferredTaskUUID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeferredTaskUUID {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardDeferredUpdate {
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
}

impl std::fmt::Display for ApiCardDeferredUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardDeferredUpdate {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.idempotency_key.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["idempotency_key".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeveloperAPINestedPage {
    #[doc = "the query to get to the next page; it is in the format of <BASE_URL>?<new_params>"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl std::fmt::Display for DeveloperAPINestedPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeveloperAPINestedPage {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.next).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["next".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiCardResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Card>,
}

impl std::fmt::Display for PaginatedResponseApiCardResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiCardResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardFulfillment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CardShipping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_personalization: Option<CardPersonalization>,
}

impl std::fmt::Display for CardFulfillment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardFulfillment {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(shipping) = &self.shipping {
                format!("{:?}", shipping).into()
            } else {
                String::new().into()
            },
            if let Some(card_personalization) = &self.card_personalization {
                format!("{:?}", card_personalization).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["shipping".into(), "card_personalization".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardSpendingRestrictionsLoad {
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[doc = "Amount limit total per interval."]
    pub amount: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_whitelist: Option<Vec<uuid::Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_mcc_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_blacklist: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_accounting_rules: Option<Vec<ApiCardAccountingRulesData>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_blacklist: Option<Vec<uuid::Uuid>>,
    #[doc = "Date to automatically lock the card. If lock date has passed, set to a future date \
             or to null to unlock the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_whitelist: Option<Vec<i64>>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
    #[doc = "Time interval to apply limit to."]
    pub interval: Interval,
}

impl std::fmt::Display for ApiCardSpendingRestrictionsLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardSpendingRestrictionsLoad {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.amount).into(),
            if let Some(policy_id) = &self.policy_id {
                format!("{:?}", policy_id).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_whitelist) = &self.vendor_whitelist {
                format!("{:?}", vendor_whitelist).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_mcc_codes) = &self.blocked_mcc_codes {
                format!("{:?}", blocked_mcc_codes).into()
            } else {
                String::new().into()
            },
            if let Some(categories_blacklist) = &self.categories_blacklist {
                format!("{:?}", categories_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(card_accounting_rules) = &self.card_accounting_rules {
                format!("{:?}", card_accounting_rules).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_blacklist) = &self.vendor_blacklist {
                format!("{:?}", vendor_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(lock_date) = &self.lock_date {
                format!("{:?}", lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(categories_whitelist) = &self.categories_whitelist {
                format!("{:?}", categories_whitelist).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.interval).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "categories".into(),
            "amount".into(),
            "policy_id".into(),
            "vendor_whitelist".into(),
            "blocked_mcc_codes".into(),
            "categories_blacklist".into(),
            "card_accounting_rules".into(),
            "vendor_blacklist".into(),
            "lock_date".into(),
            "categories_whitelist".into(),
            "transaction_amount_limit".into(),
            "interval".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardRequest {
    #[doc = "Details for shipping physical cards."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<CardFulfillment>,
    #[doc = "Unique identifier of the card owner."]
    pub user_id: uuid::Uuid,
    #[doc = "Alternative method to create a card using a card program. If this value is given, no \
             other attributes (other than idempotency_key) may be given."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_program_id: Option<uuid::Uuid>,
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
    #[doc = "Set to true to create a physical card"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_physical: Option<bool>,
    #[doc = "Specifies the spend restrictions on a Ramp card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiCardSpendingRestrictionsLoad>,
    #[doc = "Set to true to create a temporary card"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_temporary: Option<bool>,
    #[doc = "Cosmetic display name of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

impl std::fmt::Display for ApiCardRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardRequest {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(fulfillment) = &self.fulfillment {
                format!("{:?}", fulfillment).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_id).into(),
            if let Some(card_program_id) = &self.card_program_id {
                format!("{:?}", card_program_id).into()
            } else {
                String::new().into()
            },
            self.idempotency_key.clone().into(),
            if let Some(is_physical) = &self.is_physical {
                format!("{:?}", is_physical).into()
            } else {
                String::new().into()
            },
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
            } else {
                String::new().into()
            },
            if let Some(is_temporary) = &self.is_temporary {
                format!("{:?}", is_temporary).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "fulfillment".into(),
            "user_id".into(),
            "card_program_id".into(),
            "idempotency_key".into(),
            "is_physical".into(),
            "spending_restrictions".into(),
            "is_temporary".into(),
            "display_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardDeferredTaskData {
    #[doc = "Unique identifier of the subject card in the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<uuid::Uuid>,
    #[doc = "An error message if the deferred task fails."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl std::fmt::Display for ApiCardDeferredTaskData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardDeferredTaskData {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(card_id) = &self.card_id {
                format!("{:?}", card_id).into()
            } else {
                String::new().into()
            },
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["card_id".into(), "error".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardDeferredTask {
    #[doc = "Status of the deferred task. It could be one of the following values: STARTED, \
             IN_PROGRESS, ERROR, SUCCESS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Unique identifier of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Detailed data of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ApiCardDeferredTaskData>,
}

impl std::fmt::Display for CardDeferredTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardDeferredTask {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(data) = &self.data {
                format!("{:?}", data).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["status".into(), "id".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardProgramSpendingRestrictions {
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[doc = "Amount limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "Date to automatically lock the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
    #[doc = "Time interval to apply limit to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
}

impl std::fmt::Display for ApiCardProgramSpendingRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardProgramSpendingRestrictions {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(lock_date) = &self.lock_date {
                format!("{:?}", lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "categories".into(),
            "amount".into(),
            "lock_date".into(),
            "transaction_amount_limit".into(),
            "interval".into(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Icon {
    AdvertisingIcon,
    CardIcon,
    EducationStipendIcon,
    LunchOrderingIcon,
    OnboardingIcon,
    PerDiemCardIcon,
    SaasSubscriptionIcon,
    SoftwareTrialIcon,
    TravelExpensesIcon,
    WellnessIcon,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardProgramResource {
    #[doc = "Unique identifer of the card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Whether this card program is used as default card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[doc = "Whether this card program is used for physical cards."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_physical: Option<bool>,
    #[doc = "Spending restrictions associated with the card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiCardProgramSpendingRestrictions>,
    #[doc = "Display name of the card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Card program description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

impl std::fmt::Display for ApiCardProgramResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardProgramResource {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(is_default) = &self.is_default {
                format!("{:?}", is_default).into()
            } else {
                String::new().into()
            },
            if let Some(is_physical) = &self.is_physical {
                format!("{:?}", is_physical).into()
            } else {
                String::new().into()
            },
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "is_default".into(),
            "is_physical".into(),
            "spending_restrictions".into(),
            "display_name".into(),
            "description".into(),
            "icon".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiCardProgramResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<ApiCardProgramResource>,
}

impl std::fmt::Display for PaginatedResponseApiCardProgramResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiCardProgramResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardProgramCreate {
    pub acting_user_id: i64,
    pub business_id: i64,
    #[doc = "Whether this card program is used as default card program."]
    pub is_default: bool,
    #[doc = "Whether this card program is used for physical cards."]
    pub is_physical: bool,
    #[doc = "Spending restrictions associated with the card program."]
    pub spending_restrictions: ApiCardSpendingRestrictionsLoad,
    pub policy_id: i64,
    #[doc = "Display name of the card program."]
    pub display_name: String,
    #[doc = "Description of the card program."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
}

impl std::fmt::Display for ApiCardProgramCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardProgramCreate {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.acting_user_id).into(),
            format!("{:?}", self.business_id).into(),
            format!("{:?}", self.is_default).into(),
            format!("{:?}", self.is_physical).into(),
            format!("{:?}", self.spending_restrictions).into(),
            format!("{:?}", self.policy_id).into(),
            self.display_name.clone().into(),
            self.description.clone().into(),
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "acting_user_id".into(),
            "business_id".into(),
            "is_default".into(),
            "is_physical".into(),
            "spending_restrictions".into(),
            "policy_id".into(),
            "display_name".into(),
            "description".into(),
            "icon".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomIdProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_id_provider: Option<uuid::Uuid>,
}

impl std::fmt::Display for CustomIdProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomIdProvider {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(custom_id_provider) = &self.custom_id_provider {
            format!("{:?}", custom_id_provider).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["custom_id_provider".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCustomIdProviderTokenCreate {
    pub custom_id_provider: uuid::Uuid,
}

impl std::fmt::Display for ApiCustomIdProviderTokenCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCustomIdProviderTokenCreate {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.custom_id_provider).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["custom_id_provider".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RampId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ramp_id: Option<String>,
}

impl std::fmt::Display for RampId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RampId {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(ramp_id) = &self.ramp_id {
            format!("{:?}", ramp_id).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["ramp_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
}

impl std::fmt::Display for CustomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomId {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(custom_id) = &self.custom_id {
            format!("{:?}", custom_id).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["custom_id".into()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ApiCustomIdMappingCustomId {
    #[serde(rename = "a")]
    #[display("a")]
    A,
    #[serde(rename = "b")]
    #[display("b")]
    B,
    #[serde(rename = "c")]
    #[display("c")]
    C,
    #[serde(rename = "d")]
    #[display("d")]
    D,
    #[serde(rename = "e")]
    #[display("e")]
    E,
    #[serde(rename = "f")]
    #[display("f")]
    F,
    #[serde(rename = "g")]
    #[display("g")]
    G,
    #[serde(rename = "h")]
    #[display("h")]
    H,
    #[serde(rename = "i")]
    #[display("i")]
    I,
    #[serde(rename = "j")]
    #[display("j")]
    J,
    #[serde(rename = "k")]
    #[display("k")]
    K,
    #[serde(rename = "l")]
    #[display("l")]
    L,
    #[serde(rename = "m")]
    #[display("m")]
    M,
    #[serde(rename = "n")]
    #[display("n")]
    N,
    #[serde(rename = "o")]
    #[display("o")]
    O,
    #[serde(rename = "p")]
    #[display("p")]
    P,
    #[serde(rename = "q")]
    #[display("q")]
    Q,
    #[serde(rename = "r")]
    #[display("r")]
    R,
    #[serde(rename = "s")]
    #[display("s")]
    S,
    #[serde(rename = "t")]
    #[display("t")]
    T,
    #[serde(rename = "u")]
    #[display("u")]
    U,
    #[serde(rename = "v")]
    #[display("v")]
    V,
    #[serde(rename = "w")]
    #[display("w")]
    W,
    #[serde(rename = "x")]
    #[display("x")]
    X,
    #[serde(rename = "y")]
    #[display("y")]
    Y,
    #[serde(rename = "z")]
    #[display("z")]
    Z,
    #[serde(rename = "0")]
    #[display("0")]
    Zero,
    #[serde(rename = "1")]
    #[display("1")]
    One,
    #[serde(rename = "2")]
    #[display("2")]
    Two,
    #[serde(rename = "3")]
    #[display("3")]
    Three,
    #[serde(rename = "4")]
    #[display("4")]
    Four,
    #[serde(rename = "5")]
    #[display("5")]
    Five,
    #[serde(rename = "6")]
    #[display("6")]
    Six,
    #[serde(rename = "7")]
    #[display("7")]
    Seven,
    #[serde(rename = "8")]
    #[display("8")]
    Eight,
    #[serde(rename = "9")]
    #[display("9")]
    Nine,
    #[serde(rename = "-")]
    #[display("-")]
    Dash,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCustomIdMapping {
    pub custom_id: ApiCustomIdMappingCustomId,
    pub ramp_id: uuid::Uuid,
}

impl std::fmt::Display for ApiCustomIdMapping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCustomIdMapping {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.custom_id).into(),
            format!("{:?}", self.ramp_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["custom_id".into(), "ramp_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Department {
    pub name: String,
    pub id: uuid::Uuid,
}

impl std::fmt::Display for Department {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Department {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.name.clone().into(), format!("{:?}", self.id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiDepartmentUpdate {
    pub name: String,
    pub id: uuid::Uuid,
}

impl std::fmt::Display for ApiDepartmentUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiDepartmentUpdate {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.name.clone().into(), format!("{:?}", self.id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiDepartmentResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Department>,
}

impl std::fmt::Display for PaginatedResponseApiDepartmentResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiDepartmentResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiDepartmentCreate {
    pub business_id: i64,
    pub name: String,
}

impl std::fmt::Display for ApiDepartmentCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiDepartmentCreate {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.business_id).into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["business_id".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Location {
    pub name: String,
    pub id: uuid::Uuid,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Location {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.name.clone().into(), format!("{:?}", self.id).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiLocationUpdate {
    pub name: String,
}

impl std::fmt::Display for ApiLocationUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiLocationUpdate {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.name.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiLocationResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Location>,
}

impl std::fmt::Display for PaginatedResponseApiLocationResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiLocationResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiLocationCreate {
    pub name: String,
    pub business_id: i64,
}

impl std::fmt::Display for ApiLocationCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiLocationCreate {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.business_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "business_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Receipt {
    #[doc = "Unique identifier of the person who made the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the associated transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the receipt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Pre-signed url to download receipt image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for Receipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Receipt {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_id) = &self.transaction_id {
                format!("{:?}", transaction_id).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(receipt_url) = &self.receipt_url {
                format!("{:?}", receipt_url).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "user_id".into(),
            "transaction_id".into(),
            "id".into(),
            "receipt_url".into(),
            "created_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiReceiptResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Receipt>,
}

impl std::fmt::Display for PaginatedResponseApiReceiptResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiReceiptResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[doc = "Accounting field type"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Type {
    #[serde(rename = "AMORTIZATION_TEMPLATE")]
    #[display("AMORTIZATION_TEMPLATE")]
    AmortizationTemplate,
    #[serde(rename = "BILLABLE")]
    #[display("BILLABLE")]
    Billable,
    #[serde(rename = "CUSTOMERS_JOBS")]
    #[display("CUSTOMERS_JOBS")]
    CustomersJobs,
    #[serde(rename = "GL_ACCOUNT")]
    #[display("GL_ACCOUNT")]
    GlAccount,
    #[serde(rename = "INVENTORY_ITEM")]
    #[display("INVENTORY_ITEM")]
    InventoryItem,
    #[serde(rename = "MERCHANT")]
    #[display("MERCHANT")]
    Merchant,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
    #[serde(rename = "SUBSIDIARY")]
    #[display("SUBSIDIARY")]
    Subsidiary,
    #[serde(rename = "TAX_CODE")]
    #[display("TAX_CODE")]
    TaxCode,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReimbursementAccountingFieldSelection {
    #[doc = "ID that uniquely identifies an accounting field option within Ramp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of accounting field option"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "external id of accounting field option; It should uniquely identify an accounting \
             field option on the client end."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "Accounting field type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

impl std::fmt::Display for ApiReimbursementAccountingFieldSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReimbursementAccountingFieldSelection {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "external_id".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CurrencyAmount {
    #[doc = "The type of currency, in ISO 4217 format. e.g. USD for US dollars"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "the amount of money represented in the smallest denomination of the currency. For \
             example, when the currency is USD, then the amount is expressed in cents."]
    pub amount: i64,
}

impl std::fmt::Display for CurrencyAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CurrencyAmount {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(currency_code) = &self.currency_code {
                format!("{:?}", currency_code).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.amount).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["currency_code".into(), "amount".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReimbursementLineItem {
    #[doc = "List of accounting field options related to the line item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiReimbursementAccountingFieldSelection>>,
    #[doc = "Amount of the line item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<CurrencyAmount>,
}

impl std::fmt::Display for ApiReimbursementLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReimbursementLineItem {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["accounting_field_selections".into(), "amount".into()]
    }
}

#[doc = "The direction of the reimbursement. It could be either BUSINESS_TO_USER or \
         USER_TO_BUSINESS."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Direction {
    #[serde(rename = "BUSINESS_TO_USER")]
    #[display("BUSINESS_TO_USER")]
    BusinessToUser,
    #[serde(rename = "USER_TO_BUSINESS")]
    #[display("USER_TO_BUSINESS")]
    UserToBusiness,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Reimbursement {
    #[doc = "List of accounting fields related to the reimbursement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiReimbursementAccountingFieldSelection>>,
    #[doc = "Unique identifier of the person who made the reimbursement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "The amount that the payor pays."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "List of line items related to the reimbursement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<ApiReimbursementLineItem>>,
    #[doc = "Unique identifier of the reimbursement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<uuid::Uuid>>,
    #[doc = "The currency that the payor pays with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,
    #[doc = "Original reimbursement amount before the currency conversion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_reimbursement_amount: Option<CurrencyAmount>,
    #[doc = "The direction of the reimbursement. It could be either BUSINESS_TO_USER or \
             USER_TO_BUSINESS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[doc = "Time at which the reimbursement is created. Presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for Reimbursement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Reimbursement {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(line_items) = &self.line_items {
                format!("{:?}", line_items).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(receipts) = &self.receipts {
                format!("{:?}", receipts).into()
            } else {
                String::new().into()
            },
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_date) = &self.transaction_date {
                format!("{:?}", transaction_date).into()
            } else {
                String::new().into()
            },
            if let Some(merchant) = &self.merchant {
                format!("{:?}", merchant).into()
            } else {
                String::new().into()
            },
            if let Some(original_reimbursement_amount) = &self.original_reimbursement_amount {
                format!("{:?}", original_reimbursement_amount).into()
            } else {
                String::new().into()
            },
            if let Some(direction) = &self.direction {
                format!("{:?}", direction).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_field_selections".into(),
            "user_id".into(),
            "amount".into(),
            "line_items".into(),
            "id".into(),
            "receipts".into(),
            "currency".into(),
            "transaction_date".into(),
            "merchant".into(),
            "original_reimbursement_amount".into(),
            "direction".into(),
            "created_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiReimbursementResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Reimbursement>,
}

impl std::fmt::Display for PaginatedResponseApiReimbursementResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiReimbursementResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionAccountingFieldSelection {
    #[doc = "ID that uniquely identifies an accounting field option within Ramp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of accounting field option"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "external id of accounting field option; It should uniquely identify an accounting \
             field option on the client end."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "Accounting field type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

impl std::fmt::Display for ApiTransactionAccountingFieldSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionAccountingFieldSelection {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "external_id".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionLineItem {
    #[doc = "List of accounting field options related to the line item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiTransactionAccountingFieldSelection>>,
    #[doc = "Amount of the line item, denominated in the currency that the transaction was \
             settled in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<CurrencyAmount>,
}

impl std::fmt::Display for ApiTransactionLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionLineItem {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["accounting_field_selections".into(), "amount".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPolicyViolation {
    #[doc = "Type of the policy violation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Free form text regarding the policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "Uniquely identifies a policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Time at which the policy violation is created, presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for ApiTransactionPolicyViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPolicyViolation {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "memo".into(),
            "id".into(),
            "created_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionDispute {
    #[doc = "The dispute type; It could be one of the following values: RESOLVED_BY_RAMP, \
             CANCELLED_BY_CUSTOMER, CREATED_MERCHANT_ERROR and CREATED_UNRECOGNIZED_CHARGE."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Free form text regarding the dispute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "Uniquely identifies a transaction dispute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Time at which the dispute is created, presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for ApiTransactionDispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionDispute {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "memo".into(),
            "id".into(),
            "created_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionCardHolder {
    #[doc = "Card holder's ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "ID of the card holder's department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[doc = "ID of the card holder's location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[doc = "Card holder's last name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Name of the card holder's deparment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[doc = "Name of the card holder's location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[doc = "Card holder's first name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

impl std::fmt::Display for ApiTransactionCardHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionCardHolder {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(department_name) = &self.department_name {
                format!("{:?}", department_name).into()
            } else {
                String::new().into()
            },
            if let Some(location_name) = &self.location_name {
                format!("{:?}", location_name).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "user_id".into(),
            "department_id".into(),
            "location_id".into(),
            "last_name".into(),
            "department_name".into(),
            "location_name".into(),
            "first_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Reason {
    #[serde(rename = "AUTHORIZER")]
    #[display("AUTHORIZER")]
    Authorizer,
    #[serde(rename = "AUTHORIZER_AP_CARD_VELOCITY_LIMIT")]
    #[display("AUTHORIZER_AP_CARD_VELOCITY_LIMIT")]
    AuthorizerApCardVelocityLimit,
    #[serde(rename = "AUTHORIZER_BUSINESS_LIMIT")]
    #[display("AUTHORIZER_BUSINESS_LIMIT")]
    AuthorizerBusinessLimit,
    #[serde(rename = "AUTHORIZER_BUSINESS_VENDOR_BLACKLIST")]
    #[display("AUTHORIZER_BUSINESS_VENDOR_BLACKLIST")]
    AuthorizerBusinessVendorBlacklist,
    #[serde(rename = "AUTHORIZER_CARD_AUTO_LOCK_DATE")]
    #[display("AUTHORIZER_CARD_AUTO_LOCK_DATE")]
    AuthorizerCardAutoLockDate,
    #[serde(rename = "AUTHORIZER_CARD_CATEGORY_BLACKLIST")]
    #[display("AUTHORIZER_CARD_CATEGORY_BLACKLIST")]
    AuthorizerCardCategoryBlacklist,
    #[serde(rename = "AUTHORIZER_CARD_CATEGORY_WHITELIST")]
    #[display("AUTHORIZER_CARD_CATEGORY_WHITELIST")]
    AuthorizerCardCategoryWhitelist,
    #[serde(rename = "AUTHORIZER_CARD_LIMIT")]
    #[display("AUTHORIZER_CARD_LIMIT")]
    AuthorizerCardLimit,
    #[serde(rename = "AUTHORIZER_CARD_MCC_BLACKLIST")]
    #[display("AUTHORIZER_CARD_MCC_BLACKLIST")]
    AuthorizerCardMccBlacklist,
    #[serde(rename = "AUTHORIZER_CARD_MISSING_POLICY_ITEMS")]
    #[display("AUTHORIZER_CARD_MISSING_POLICY_ITEMS")]
    AuthorizerCardMissingPolicyItems,
    #[serde(rename = "AUTHORIZER_CARD_NOT_ACTIVATED")]
    #[display("AUTHORIZER_CARD_NOT_ACTIVATED")]
    AuthorizerCardNotActivated,
    #[serde(rename = "AUTHORIZER_CARD_SUSPENDED")]
    #[display("AUTHORIZER_CARD_SUSPENDED")]
    AuthorizerCardSuspended,
    #[serde(rename = "AUTHORIZER_CARD_VENDOR_BLACKLIST")]
    #[display("AUTHORIZER_CARD_VENDOR_BLACKLIST")]
    AuthorizerCardVendorBlacklist,
    #[serde(rename = "AUTHORIZER_CARD_VENDOR_WHITELIST")]
    #[display("AUTHORIZER_CARD_VENDOR_WHITELIST")]
    AuthorizerCardVendorWhitelist,
    #[serde(rename = "AUTHORIZER_COMMANDO_MODE")]
    #[display("AUTHORIZER_COMMANDO_MODE")]
    AuthorizerCommandoMode,
    #[serde(rename = "AUTHORIZER_FRAUD")]
    #[display("AUTHORIZER_FRAUD")]
    AuthorizerFraud,
    #[serde(rename = "AUTHORIZER_GLOBAL_MCC_BLACKLIST")]
    #[display("AUTHORIZER_GLOBAL_MCC_BLACKLIST")]
    AuthorizerGlobalMccBlacklist,
    #[serde(rename = "AUTHORIZER_NON_AP_CARD_VELOCITY_LIMIT")]
    #[display("AUTHORIZER_NON_AP_CARD_VELOCITY_LIMIT")]
    AuthorizerNonApCardVelocityLimit,
    #[serde(rename = "AUTHORIZER_TRANSACTION_AMOUNT_LIMIT")]
    #[display("AUTHORIZER_TRANSACTION_AMOUNT_LIMIT")]
    AuthorizerTransactionAmountLimit,
    #[serde(rename = "AUTHORIZER_USER_LIMIT")]
    #[display("AUTHORIZER_USER_LIMIT")]
    AuthorizerUserLimit,
    #[serde(rename = "BLOCKED_COUNTRY")]
    #[display("BLOCKED_COUNTRY")]
    BlockedCountry,
    #[serde(rename = "CARD_TERMINATED")]
    #[display("CARD_TERMINATED")]
    CardTerminated,
    #[serde(rename = "CHIP_FAILLURE")]
    #[display("CHIP_FAILLURE")]
    ChipFaillure,
    #[serde(rename = "FORBIDDEN_CATEGORY")]
    #[display("FORBIDDEN_CATEGORY")]
    ForbiddenCategory,
    #[serde(rename = "MOBILE_WALLET_FAILURE")]
    #[display("MOBILE_WALLET_FAILURE")]
    MobileWalletFailure,
    #[serde(rename = "NOT_ACTIVE")]
    #[display("NOT_ACTIVE")]
    NotActive,
    #[serde(rename = "NOT_ALLOWED")]
    #[display("NOT_ALLOWED")]
    NotAllowed,
    #[serde(rename = "OFAC_VERIFICATION_NEEDED")]
    #[display("OFAC_VERIFICATION_NEEDED")]
    OfacVerificationNeeded,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
    #[serde(rename = "PROCESSOR_CAP")]
    #[display("PROCESSOR_CAP")]
    ProcessorCap,
    #[serde(rename = "QUASI_CASH")]
    #[display("QUASI_CASH")]
    QuasiCash,
    #[serde(rename = "STRIPE_WEBHOOK_TIMEOUT")]
    #[display("STRIPE_WEBHOOK_TIMEOUT")]
    StripeWebhookTimeout,
    #[serde(rename = "SUSPECTED_BIN_ATTACK")]
    #[display("SUSPECTED_BIN_ATTACK")]
    SuspectedBinAttack,
    #[serde(rename = "SUSPECTED_FRAUD")]
    #[display("SUSPECTED_FRAUD")]
    SuspectedFraud,
    #[serde(rename = "USER_TERMINATED")]
    #[display("USER_TERMINATED")]
    UserTerminated,
    #[serde(rename = "WRONG_ADDRESS")]
    #[display("WRONG_ADDRESS")]
    WrongAddress,
    #[serde(rename = "WRONG_CVV")]
    #[display("WRONG_CVV")]
    WrongCvv,
    #[serde(rename = "WRONG_EXPIRATION")]
    #[display("WRONG_EXPIRATION")]
    WrongExpiration,
    #[serde(rename = "WRONG_POSTAL_CODE")]
    #[display("WRONG_POSTAL_CODE")]
    WrongPostalCode,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionDeclineDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
}

impl std::fmt::Display for ApiTransactionDeclineDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionDeclineDetails {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(reason) = &self.reason {
                format!("{:?}", reason).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["reason".into(), "amount".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingCategory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_category_remote_name: Option<String>,
    #[doc = "User-selected category name for transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[doc = "User-selected category id for transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_category_remote_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_category_remote_id: Option<String>,
}

impl std::fmt::Display for ApiAccountingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingCategory {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tracking_category_remote_name) = &self.tracking_category_remote_name {
                format!("{:?}", tracking_category_remote_name).into()
            } else {
                String::new().into()
            },
            if let Some(category_name) = &self.category_name {
                format!("{:?}", category_name).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_category_remote_type) = &self.tracking_category_remote_type {
                format!("{:?}", tracking_category_remote_type).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_category_remote_id) = &self.tracking_category_remote_id {
                format!("{:?}", tracking_category_remote_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tracking_category_remote_name".into(),
            "category_name".into(),
            "category_id".into(),
            "tracking_category_remote_type".into(),
            "tracking_category_remote_id".into(),
        ]
    }
}

#[doc = "transaction state."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum TransactionState {
    #[serde(rename = "ALL")]
    #[display("ALL")]
    All,
    #[serde(rename = "CLEARED")]
    #[display("CLEARED")]
    Cleared,
    #[serde(rename = "COMPLETION")]
    #[display("COMPLETION")]
    Completion,
    #[serde(rename = "DECLINED")]
    #[display("DECLINED")]
    Declined,
    #[serde(rename = "ERROR")]
    #[display("ERROR")]
    Error,
    #[serde(rename = "PENDING")]
    #[display("PENDING")]
    Pending,
    #[serde(rename = "PENDING_INITIATION")]
    #[display("PENDING_INITIATION")]
    PendingInitiation,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Transaction {
    #[doc = "List of accounting fields related to the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiTransactionAccountingFieldSelection>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[doc = "List of line items related to the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<ApiTransactionLineItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "A list of policy violations sorted in descending order by their creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_violations: Option<Vec<ApiTransactionPolicyViolation>>,
    #[doc = "Description about the merchant category code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_category_code_description: Option<String>,
    #[doc = "A list of disputes sorted in descending order by their creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disputes: Option<Vec<ApiTransactionDispute>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "Ramp-internal category id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sk_category_id: Option<i64>,
    #[doc = "Receipts listed in ascending order by their creation time, related to the \
             transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<uuid::Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>,
    #[doc = "Information about the card holder."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<ApiTransactionCardHolder>,
    #[doc = "A merchant descriptor is the name that appears on a customer's bank statement when \
             they make a purchase from that merchant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_descriptor: Option<String>,
    #[doc = "the original transaction amount before the currency conversion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_transaction_amount: Option<CurrencyAmount>,
    #[doc = "transaction state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TransactionState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_transaction_time: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Ramp-internal category name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sk_category_name: Option<String>,
    #[doc = "Settled amount of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "Details about a transaction decline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decline_details: Option<ApiTransactionDeclineDetails>,
    #[doc = "Accounting categories related to the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_categories: Option<Vec<ApiAccountingCategory>>,
    #[doc = "Merchant category code is a four-digit number in ISP 18245 used to classify a \
             business by the types of goods and services it provides."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<uuid::Uuid>,
    #[doc = "Currency that the transaction is settled in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Transaction {
    const LENGTH: usize = 23;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_name) = &self.merchant_name {
                format!("{:?}", merchant_name).into()
            } else {
                String::new().into()
            },
            if let Some(line_items) = &self.line_items {
                format!("{:?}", line_items).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(policy_violations) = &self.policy_violations {
                format!("{:?}", policy_violations).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_category_code_description) =
                &self.merchant_category_code_description
            {
                format!("{:?}", merchant_category_code_description).into()
            } else {
                String::new().into()
            },
            if let Some(disputes) = &self.disputes {
                format!("{:?}", disputes).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
            if let Some(sk_category_id) = &self.sk_category_id {
                format!("{:?}", sk_category_id).into()
            } else {
                String::new().into()
            },
            if let Some(receipts) = &self.receipts {
                format!("{:?}", receipts).into()
            } else {
                String::new().into()
            },
            if let Some(card_id) = &self.card_id {
                format!("{:?}", card_id).into()
            } else {
                String::new().into()
            },
            if let Some(card_holder) = &self.card_holder {
                format!("{:?}", card_holder).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_descriptor) = &self.merchant_descriptor {
                format!("{:?}", merchant_descriptor).into()
            } else {
                String::new().into()
            },
            if let Some(original_transaction_amount) = &self.original_transaction_amount {
                format!("{:?}", original_transaction_amount).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(user_transaction_time) = &self.user_transaction_time {
                format!("{:?}", user_transaction_time).into()
            } else {
                String::new().into()
            },
            if let Some(sk_category_name) = &self.sk_category_name {
                format!("{:?}", sk_category_name).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(decline_details) = &self.decline_details {
                format!("{:?}", decline_details).into()
            } else {
                String::new().into()
            },
            if let Some(accounting_categories) = &self.accounting_categories {
                format!("{:?}", accounting_categories).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_category_code) = &self.merchant_category_code {
                format!("{:?}", merchant_category_code).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_id) = &self.merchant_id {
                format!("{:?}", merchant_id).into()
            } else {
                String::new().into()
            },
            if let Some(currency_code) = &self.currency_code {
                format!("{:?}", currency_code).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_field_selections".into(),
            "merchant_name".into(),
            "line_items".into(),
            "id".into(),
            "policy_violations".into(),
            "merchant_category_code_description".into(),
            "disputes".into(),
            "memo".into(),
            "sk_category_id".into(),
            "receipts".into(),
            "card_id".into(),
            "card_holder".into(),
            "merchant_descriptor".into(),
            "original_transaction_amount".into(),
            "state".into(),
            "user_transaction_time".into(),
            "sk_category_name".into(),
            "amount".into(),
            "decline_details".into(),
            "accounting_categories".into(),
            "merchant_category_code".into(),
            "merchant_id".into(),
            "currency_code".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiTransactionCanonicalSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Transaction>,
}

impl std::fmt::Display for PaginatedResponseApiTransactionCanonicalSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiTransactionCanonicalSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[doc = "The employee's status; It could be one of the following values: INVITE_PENDING, \
         INVITE_DELETED, INVITE_EXPIRED, USER_ONBOARDING, USER_ACTIVE and USER_SUSPENDED"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Status {
    #[serde(rename = "INVITE_DELETED")]
    #[display("INVITE_DELETED")]
    InviteDeleted,
    #[serde(rename = "INVITE_EXPIRED")]
    #[display("INVITE_EXPIRED")]
    InviteExpired,
    #[serde(rename = "INVITE_PENDING")]
    #[display("INVITE_PENDING")]
    InvitePending,
    #[serde(rename = "USER_ACTIVE")]
    #[display("USER_ACTIVE")]
    UserActive,
    #[serde(rename = "USER_ONBOARDING")]
    #[display("USER_ONBOARDING")]
    UserOnboarding,
    #[serde(rename = "USER_SUSPENDED")]
    #[display("USER_SUSPENDED")]
    UserSuspended,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct User {
    #[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
             Owner, Bookkeeper"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "The employee's phone number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Unique identifier of the employee's department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<uuid::Uuid>,
    #[doc = "Unique employee identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the company that the employee's working for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the employee's manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<uuid::Uuid>,
    #[doc = "Whether the employee is a manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_manager: Option<bool>,
    #[doc = "Unique identifier of the employee's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<uuid::Uuid>,
    #[doc = "Last name of the employee"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The employee's status; It could be one of the following values: INVITE_PENDING, \
             INVITE_DELETED, INVITE_EXPIRED, USER_ONBOARDING, USER_ACTIVE and USER_SUSPENDED"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "The employee's email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "First name of the employee"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for User {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(business_id) = &self.business_id {
                format!("{:?}", business_id).into()
            } else {
                String::new().into()
            },
            if let Some(manager_id) = &self.manager_id {
                format!("{:?}", manager_id).into()
            } else {
                String::new().into()
            },
            if let Some(is_manager) = &self.is_manager {
                format!("{:?}", is_manager).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "role".into(),
            "phone".into(),
            "department_id".into(),
            "id".into(),
            "business_id".into(),
            "manager_id".into(),
            "is_manager".into(),
            "location_id".into(),
            "last_name".into(),
            "status".into(),
            "email".into(),
            "first_name".into(),
        ]
    }
}

#[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
         Bookkeeper; Note that Owner is not a permissible value."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Role {
    #[serde(rename = "BUSINESS_ADMIN")]
    #[display("BUSINESS_ADMIN")]
    BusinessAdmin,
    #[serde(rename = "BUSINESS_USER")]
    #[display("BUSINESS_USER")]
    BusinessUser,
    #[serde(rename = "BUSINESS_BOOKKEEPER")]
    #[display("BUSINESS_BOOKKEEPER")]
    BusinessBookkeeper,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserUpdate {
    #[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
             Bookkeeper; Note that Owner is not a permissible value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[doc = "Unique identifier of the employee's direct manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the employee's department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the employee's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiUserUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserUpdate {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(direct_manager_id) = &self.direct_manager_id {
                format!("{:?}", direct_manager_id).into()
            } else {
                String::new().into()
            },
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "role".into(),
            "direct_manager_id".into(),
            "department_id".into(),
            "location_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiUserResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<User>,
}

impl std::fmt::Display for PaginatedResponseApiUserResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiUserResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
         Bookkeeper; Note that Owner is not a invitable role."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ApiUserCreateRole {
    #[serde(rename = "BUSINESS_ADMIN")]
    #[display("BUSINESS_ADMIN")]
    BusinessAdmin,
    #[serde(rename = "BUSINESS_USER")]
    #[display("BUSINESS_USER")]
    BusinessUser,
    #[serde(rename = "BUSINESS_BOOKKEEPER")]
    #[display("BUSINESS_BOOKKEEPER")]
    BusinessBookkeeper,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserCreate {
    #[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
             Bookkeeper; Note that Owner is not a invitable role."]
    pub role: ApiUserCreateRole,
    #[doc = "The employee's phone number"]
    pub phone: String,
    #[doc = "Unique identifier of the employee's department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the employee's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<uuid::Uuid>,
    #[doc = "an idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[doc = "Last name of the employee"]
    pub last_name: String,
    #[doc = "The employee's email address"]
    pub email: String,
    #[doc = "First name of the employee"]
    pub first_name: String,
    #[doc = "Unique identifier of the employee's direct manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiUserCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserCreate {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.role).into(),
            self.phone.clone().into(),
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(idempotency_key) = &self.idempotency_key {
                format!("{:?}", idempotency_key).into()
            } else {
                String::new().into()
            },
            self.last_name.clone().into(),
            self.email.clone().into(),
            self.first_name.clone().into(),
            if let Some(direct_manager_id) = &self.direct_manager_id {
                format!("{:?}", direct_manager_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "role".into(),
            "phone".into(),
            "department_id".into(),
            "location_id".into(),
            "idempotency_key".into(),
            "last_name".into(),
            "email".into(),
            "first_name".into(),
            "direct_manager_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserDeferredTaskData {
    #[doc = "The subject employee's ID of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
    #[doc = "An error message if the deferred task fails"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl std::fmt::Display for ApiUserDeferredTaskData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserDeferredTaskData {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_id".into(), "error".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserDeferredTask {
    #[doc = "Status of the deferred task. It could be one of the following values: STARTED, \
             IN_PROGRESS, ERROR, SUCCESS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Unique identifier of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Detailed data of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ApiUserDeferredTaskData>,
}

impl std::fmt::Display for UserDeferredTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserDeferredTask {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(data) = &self.data {
                format!("{:?}", data).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["status".into(), "id".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Memo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

impl std::fmt::Display for Memo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Memo {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["memo".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiMemoResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Memo>,
}

impl std::fmt::Display for PaginatedResponseApiMemoResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiMemoResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Merchant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sk_category_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
}

impl std::fmt::Display for Merchant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Merchant {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(sk_category_name) = &self.sk_category_name {
                format!("{:?}", sk_category_name).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_name) = &self.merchant_name {
                format!("{:?}", merchant_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "sk_category_name".into(),
            "id".into(),
            "merchant_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiMerchantResourceSchema {
    pub page: DeveloperAPINestedPage,
    pub data: Vec<Merchant>,
}

impl std::fmt::Display for PaginatedResponseApiMerchantResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiMerchantResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.page).into(),
            format!("{:?}", self.data).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["page".into(), "data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadOfficeAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_street_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_apt_suite: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_country: Option<String>,
}

impl std::fmt::Display for ApiSalesLeadOfficeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSalesLeadOfficeAddress {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(office_postal_code) = &self.office_postal_code {
                format!("{:?}", office_postal_code).into()
            } else {
                String::new().into()
            },
            if let Some(office_state) = &self.office_state {
                format!("{:?}", office_state).into()
            } else {
                String::new().into()
            },
            if let Some(office_city) = &self.office_city {
                format!("{:?}", office_city).into()
            } else {
                String::new().into()
            },
            if let Some(office_street_address) = &self.office_street_address {
                format!("{:?}", office_street_address).into()
            } else {
                String::new().into()
            },
            if let Some(office_apt_suite) = &self.office_apt_suite {
                format!("{:?}", office_apt_suite).into()
            } else {
                String::new().into()
            },
            if let Some(office_country) = &self.office_country {
                format!("{:?}", office_country).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "office_postal_code".into(),
            "office_state".into(),
            "office_city".into(),
            "office_street_address".into(),
            "office_apt_suite".into(),
            "office_country".into(),
        ]
    }
}

#[doc = "Type of incorporation."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum EntityType {
    #[serde(rename = "COOPERATIVE")]
    #[display("COOPERATIVE")]
    Cooperative,
    #[serde(rename = "CORPORATION")]
    #[display("CORPORATION")]
    Corporation,
    #[serde(rename = "LLC")]
    #[display("LLC")]
    Llc,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
    #[serde(rename = "PARTNERSHIP")]
    #[display("PARTNERSHIP")]
    Partnership,
    #[serde(rename = "SOLE_PROPRIETORSHIP")]
    #[display("SOLE_PROPRIETORSHIP")]
    SoleProprietorship,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadBusinessDump {
    #[doc = "Office's address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<ApiSalesLeadOfficeAddress>,
    #[doc = "Business's incorporation date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_incorporation: Option<chrono::NaiveDate>,
    #[doc = "The state in which the business is incorporated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_of_incorporation: Option<String>,
    #[doc = "A short description of the business."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_description: Option<String>,
    #[doc = "Employer Identification Number (EIN)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein_number: Option<String>,
    #[doc = "Business's sector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[doc = "Business's website."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_website: Option<String>,
    #[doc = "Estimated monthly spend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_spend: Option<String>,
    #[doc = "Legal name of the business."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_legal: Option<String>,
    #[doc = "Business's industry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[doc = "Doing business as (DBA)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_dba: Option<String>,
    #[doc = "Business's industry group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_group: Option<String>,
    #[doc = "Office phone number. Must include country code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_phone_number: Option<String>,
    #[doc = "Type of incorporation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_industry: Option<String>,
}

impl std::fmt::Display for ApiSalesLeadBusinessDump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSalesLeadBusinessDump {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(office_address) = &self.office_address {
                format!("{:?}", office_address).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_incorporation) = &self.date_of_incorporation {
                format!("{:?}", date_of_incorporation).into()
            } else {
                String::new().into()
            },
            if let Some(state_of_incorporation) = &self.state_of_incorporation {
                format!("{:?}", state_of_incorporation).into()
            } else {
                String::new().into()
            },
            if let Some(business_description) = &self.business_description {
                format!("{:?}", business_description).into()
            } else {
                String::new().into()
            },
            if let Some(ein_number) = &self.ein_number {
                format!("{:?}", ein_number).into()
            } else {
                String::new().into()
            },
            if let Some(sector) = &self.sector {
                format!("{:?}", sector).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_website) = &self.business_name_website {
                format!("{:?}", business_name_website).into()
            } else {
                String::new().into()
            },
            if let Some(estimated_monthly_spend) = &self.estimated_monthly_spend {
                format!("{:?}", estimated_monthly_spend).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_legal) = &self.business_name_legal {
                format!("{:?}", business_name_legal).into()
            } else {
                String::new().into()
            },
            if let Some(industry) = &self.industry {
                format!("{:?}", industry).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_dba) = &self.business_name_dba {
                format!("{:?}", business_name_dba).into()
            } else {
                String::new().into()
            },
            if let Some(industry_group) = &self.industry_group {
                format!("{:?}", industry_group).into()
            } else {
                String::new().into()
            },
            if let Some(office_phone_number) = &self.office_phone_number {
                format!("{:?}", office_phone_number).into()
            } else {
                String::new().into()
            },
            if let Some(entity_type) = &self.entity_type {
                format!("{:?}", entity_type).into()
            } else {
                String::new().into()
            },
            if let Some(sub_industry) = &self.sub_industry {
                format!("{:?}", sub_industry).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "office_address".into(),
            "date_of_incorporation".into(),
            "state_of_incorporation".into(),
            "business_description".into(),
            "ein_number".into(),
            "sector".into(),
            "business_name_website".into(),
            "estimated_monthly_spend".into(),
            "business_name_legal".into(),
            "industry".into(),
            "business_name_dba".into(),
            "industry_group".into(),
            "office_phone_number".into(),
            "entity_type".into(),
            "sub_industry".into(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[derive(Default)]
pub enum Source {
    #[default]
    AngelList,
}



#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FinancingApplicationStatus {
    #[serde(rename = "ALLOY_COMPLETE")]
    #[display("ALLOY_COMPLETE")]
    AlloyComplete,
    #[serde(rename = "Admin Approved")]
    #[display("Admin Approved")]
    AdminApproved,
    #[serde(rename = "DOCUMENTS_REQUIRED")]
    #[display("DOCUMENTS_REQUIRED")]
    DocumentsRequired,
    #[serde(rename = "DOCUMENTS_SUBMITTED")]
    #[display("DOCUMENTS_SUBMITTED")]
    DocumentsSubmitted,
    #[serde(rename = "KYC Approved")]
    #[display("KYC Approved")]
    KycApproved,
    #[serde(rename = "OPS_REVIEW")]
    #[display("OPS_REVIEW")]
    OpsReview,
    Pending,
    Rejected,
    Submitted,
    Withdrawn,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Lead {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    pub source: Source,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub financing_application_status: Option<FinancingApplicationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    pub external_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_info: Option<ApiSalesLeadBusinessDump>,
}

impl std::fmt::Display for Lead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Lead {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.source).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(financing_application_status) = &self.financing_application_status {
                format!("{:?}", financing_application_status).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            self.external_id.clone().into(),
            if let Some(business_info) = &self.business_info {
                format!("{:?}", business_info).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "phone".into(),
            "updated_at".into(),
            "id".into(),
            "source".into(),
            "created_at".into(),
            "financing_application_status".into(),
            "last_name".into(),
            "email".into(),
            "first_name".into(),
            "external_id".into(),
            "business_info".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadBusinessLoad {
    #[doc = "Office's address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<ApiSalesLeadOfficeAddress>,
    #[doc = "Business's incorporation date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_incorporation: Option<chrono::NaiveDate>,
    #[doc = "The state in which the business is incorporated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_of_incorporation: Option<String>,
    #[doc = "A short description of the business."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_description: Option<String>,
    #[doc = "Employer Identification Number (EIN)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein_number: Option<String>,
    #[doc = "Business's sector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[doc = "Business's website."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_website: Option<String>,
    #[doc = "Estimated monthly spend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_spend: Option<String>,
    #[doc = "Legal name of the business."]
    pub business_name_legal: String,
    #[doc = "Business's industry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[doc = "Doing business as (DBA)"]
    pub business_name_dba: String,
    #[doc = "Business's industry group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_group: Option<String>,
    #[doc = "Office phone number. Must include country code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_phone_number: Option<String>,
    #[doc = "Type of incorporation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[doc = "Business's subindustry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_industry: Option<String>,
}

impl std::fmt::Display for ApiSalesLeadBusinessLoad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSalesLeadBusinessLoad {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(office_address) = &self.office_address {
                format!("{:?}", office_address).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_incorporation) = &self.date_of_incorporation {
                format!("{:?}", date_of_incorporation).into()
            } else {
                String::new().into()
            },
            if let Some(state_of_incorporation) = &self.state_of_incorporation {
                format!("{:?}", state_of_incorporation).into()
            } else {
                String::new().into()
            },
            if let Some(business_description) = &self.business_description {
                format!("{:?}", business_description).into()
            } else {
                String::new().into()
            },
            if let Some(ein_number) = &self.ein_number {
                format!("{:?}", ein_number).into()
            } else {
                String::new().into()
            },
            if let Some(sector) = &self.sector {
                format!("{:?}", sector).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_website) = &self.business_name_website {
                format!("{:?}", business_name_website).into()
            } else {
                String::new().into()
            },
            if let Some(estimated_monthly_spend) = &self.estimated_monthly_spend {
                format!("{:?}", estimated_monthly_spend).into()
            } else {
                String::new().into()
            },
            self.business_name_legal.clone().into(),
            if let Some(industry) = &self.industry {
                format!("{:?}", industry).into()
            } else {
                String::new().into()
            },
            self.business_name_dba.clone().into(),
            if let Some(industry_group) = &self.industry_group {
                format!("{:?}", industry_group).into()
            } else {
                String::new().into()
            },
            if let Some(office_phone_number) = &self.office_phone_number {
                format!("{:?}", office_phone_number).into()
            } else {
                String::new().into()
            },
            if let Some(entity_type) = &self.entity_type {
                format!("{:?}", entity_type).into()
            } else {
                String::new().into()
            },
            if let Some(sub_industry) = &self.sub_industry {
                format!("{:?}", sub_industry).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "office_address".into(),
            "date_of_incorporation".into(),
            "state_of_incorporation".into(),
            "business_description".into(),
            "ein_number".into(),
            "sector".into(),
            "business_name_website".into(),
            "estimated_monthly_spend".into(),
            "business_name_legal".into(),
            "industry".into(),
            "business_name_dba".into(),
            "industry_group".into(),
            "office_phone_number".into(),
            "entity_type".into(),
            "sub_industry".into(),
        ]
    }
}

#[doc = ""]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[derive(Default)]
pub enum ApiSalesLeadCreateSource {
    #[default]
    AngelList,
}



#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadCreate {
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub state: String,
    #[doc = ""]
    pub source: ApiSalesLeadCreateSource,
    #[doc = ""]
    pub last_name: String,
    #[doc = ""]
    pub redirect_uri: String,
    #[doc = ""]
    pub email: String,
    #[doc = ""]
    pub first_name: String,
    #[doc = ""]
    pub external_id: String,
    #[doc = ""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_info: Option<ApiSalesLeadBusinessLoad>,
}

impl std::fmt::Display for ApiSalesLeadCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSalesLeadCreate {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            self.state.clone().into(),
            format!("{:?}", self.source).into(),
            self.last_name.clone().into(),
            self.redirect_uri.clone().into(),
            self.email.clone().into(),
            self.first_name.clone().into(),
            self.external_id.clone().into(),
            if let Some(business_info) = &self.business_info {
                format!("{:?}", business_info).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "phone".into(),
            "state".into(),
            "source".into(),
            "last_name".into(),
            "redirect_uri".into(),
            "email".into(),
            "first_name".into(),
            "external_id".into(),
            "business_info".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Upload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sales_lead_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for Upload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Upload {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(document_type) = &self.document_type {
                format!("{:?}", document_type).into()
            } else {
                String::new().into()
            },
            if let Some(sales_lead_id) = &self.sales_lead_id {
                format!("{:?}", sales_lead_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "updated_at".into(),
            "document_type".into(),
            "sales_lead_id".into(),
            "created_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReceiptIntegrationOptedOutEmailResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl std::fmt::Display for ApiReceiptIntegrationOptedOutEmailResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReceiptIntegrationOptedOutEmailResource {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "email".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReceiptIntegrationOptedOutEmailCreate {
    pub business_id: i64,
    pub email: String,
}

impl std::fmt::Display for ApiReceiptIntegrationOptedOutEmailCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReceiptIntegrationOptedOutEmailCreate {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.business_id).into(),
            self.email.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["business_id".into(), "email".into()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    parse_display :: FromStr,
    parse_display :: Display,
)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum GetTransactionsCanonicalListWithPaginationState {
    #[serde(rename = "ALL")]
    #[display("ALL")]
    All,
    #[serde(rename = "CLEARED")]
    #[display("CLEARED")]
    Cleared,
    #[serde(rename = "COMPLETION")]
    #[display("COMPLETION")]
    Completion,
    #[serde(rename = "DECLINED")]
    #[display("DECLINED")]
    Declined,
    #[serde(rename = "ERROR")]
    #[display("ERROR")]
    Error,
    #[serde(rename = "PENDING")]
    #[display("PENDING")]
    Pending,
    #[serde(rename = "PENDING_INITIATION")]
    #[display("PENDING_INITIATION")]
    PendingInitiation,
}
