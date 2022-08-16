#![doc = r" This module contains the generated types for the library."]
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
                format!("+1{}", s)
                    .replace('-', "")
                    .replace('(', "")
                    .replace(')', "")
                    .replace(' ', "")
            } else {
                s.replace('-', "")
                    .replace('(', "")
                    .replace(')', "")
                    .replace(' ', "")
            };
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
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftInterval {
    #[doc = "Start of shift"]
    pub start: String,
    #[doc = "End of shift"]
    pub end: String,
}

impl std::fmt::Display for ShiftInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ShiftInterval {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.start.clone(), self.end.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["start".to_string(), "end".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftIntervals {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mon: Option<ShiftInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tue: Option<ShiftInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wed: Option<ShiftInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thu: Option<ShiftInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fri: Option<ShiftInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sat: Option<ShiftInterval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sun: Option<ShiftInterval>,
}

impl std::fmt::Display for ShiftIntervals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ShiftIntervals {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(mon) = &self.mon {
                format!("{:?}", mon)
            } else {
                String::new()
            },
            if let Some(tue) = &self.tue {
                format!("{:?}", tue)
            } else {
                String::new()
            },
            if let Some(wed) = &self.wed {
                format!("{:?}", wed)
            } else {
                String::new()
            },
            if let Some(thu) = &self.thu {
                format!("{:?}", thu)
            } else {
                String::new()
            },
            if let Some(fri) = &self.fri {
                format!("{:?}", fri)
            } else {
                String::new()
            },
            if let Some(sat) = &self.sat {
                format!("{:?}", sat)
            } else {
                String::new()
            },
            if let Some(sun) = &self.sun {
                format!("{:?}", sun)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "mon".to_string(),
            "tue".to_string(),
            "wed".to_string(),
            "thu".to_string(),
            "fri".to_string(),
            "sat".to_string(),
            "sun".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TagIds {
    pub tag_ids: Vec<String>,
}

impl std::fmt::Display for TagIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TagIds {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.tag_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["tag_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeammateIds {
    pub teammate_ids: Vec<String>,
}

impl std::fmt::Display for TeammateIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TeammateIds {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.teammate_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["teammate_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ChannelIds {
    pub channel_ids: Vec<String>,
}

impl std::fmt::Display for ChannelIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ChannelIds {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.channel_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["channel_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InboxIds {
    pub inbox_ids: Vec<String>,
}

impl std::fmt::Display for InboxIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for InboxIds {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.inbox_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["inbox_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeamIds {
    pub team_ids: Vec<String>,
}

impl std::fmt::Display for TeamIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TeamIds {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.team_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["team_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactIds {
    pub contact_ids: Vec<String>,
}

impl std::fmt::Display for ContactIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactIds {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.contact_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["contact_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AccountIds {
    pub account_ids: Vec<String>,
}

impl std::fmt::Display for AccountIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AccountIds {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.account_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["account_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Account {
    #[doc = "Name of the Account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Account description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[doc = "ID of the Account in an external system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "Custom attributes for this account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<std::collections::HashMap<String, String>>,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Account {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(domains) = &self.domains {
                format!("{:?}", domains)
            } else {
                String::new()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id)
            } else {
                String::new()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "description".to_string(),
            "domains".to_string(),
            "external_id".to_string(),
            "custom_fields".to_string(),
        ]
    }
}

#[doc = "Resources to compute the analytics for. Defaults to all."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsFilters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teammate_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub team_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_ids: Option<Vec<String>>,
}

impl std::fmt::Display for AnalyticsFilters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsFilters {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tag_ids) = &self.tag_ids {
                format!("{:?}", tag_ids)
            } else {
                String::new()
            },
            if let Some(teammate_ids) = &self.teammate_ids {
                format!("{:?}", teammate_ids)
            } else {
                String::new()
            },
            if let Some(channel_ids) = &self.channel_ids {
                format!("{:?}", channel_ids)
            } else {
                String::new()
            },
            if let Some(inbox_ids) = &self.inbox_ids {
                format!("{:?}", inbox_ids)
            } else {
                String::new()
            },
            if let Some(team_ids) = &self.team_ids {
                format!("{:?}", team_ids)
            } else {
                String::new()
            },
            if let Some(account_ids) = &self.account_ids {
                format!("{:?}", account_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tag_ids".to_string(),
            "teammate_ids".to_string(),
            "channel_ids".to_string(),
            "inbox_ids".to_string(),
            "team_ids".to_string(),
            "account_ids".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsReportRequest2 {
    #[doc = "Start time of the data to include in the export (seconds since \
             1970-01-01T00:00:00+00). Will be rounded down to the start of the day."]
    pub start: f64,
    #[doc = "End time of the data to include in the export (seconds since \
             1970-01-01T00:00:00+00). Will be rounded up to the end of the day."]
    pub end: f64,
    #[doc = "[IANA name](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) of the \
             timezone to format the dates with. If omitted, the export will use Etc/UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[doc = "Resources to compute the analytics for. Defaults to all."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsFilters>,
    #[doc = "List of the metrics required."]
    pub metrics: Vec<AnalyticsMetricId>,
}

impl std::fmt::Display for AnalyticsReportRequest2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsReportRequest2 {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.start),
            format!("{:?}", self.end),
            if let Some(timezone) = &self.timezone {
                format!("{:?}", timezone)
            } else {
                String::new()
            },
            if let Some(filters) = &self.filters {
                format!("{:?}", filters)
            } else {
                String::new()
            },
            format!("{:?}", self.metrics),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "start".to_string(),
            "end".to_string(),
            "timezone".to_string(),
            "filters".to_string(),
            "metrics".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Type {
    #[serde(rename = "events")]
    #[display("events")]
    Events,
    #[serde(rename = "messages")]
    #[display("messages")]
    Messages,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsExportRequest2 {
    #[doc = "Start time of the data to include in the export (seconds since \
             1970-01-01T00:00:00+00). Will be rounded down to the start of the day."]
    pub start: f64,
    #[doc = "End time of the data to include in the export (seconds since \
             1970-01-01T00:00:00+00). Will be rounded up to the end of the day."]
    pub end: f64,
    #[doc = "[IANA name](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones) of the \
             timezone to format the dates with. If omitted, the export will use Etc/UTC."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[doc = "Resources to compute the analytics for. Defaults to all."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsFilters>,
    #[serde(rename = "type")]
    pub type_: Type,
}

impl std::fmt::Display for AnalyticsExportRequest2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsExportRequest2 {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.start),
            format!("{:?}", self.end),
            if let Some(timezone) = &self.timezone {
                format!("{:?}", timezone)
            } else {
                String::new()
            },
            if let Some(filters) = &self.filters {
                format!("{:?}", filters)
            } else {
                String::new()
            },
            format!("{:?}", self.type_),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "start".to_string(),
            "end".to_string(),
            "timezone".to_string(),
            "filters".to_string(),
            "type_".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum AnalyticsMetricId {
    #[serde(rename = "avg_csat_survey_response")]
    #[display("avg_csat_survey_response")]
    AvgCsatSurveyResponse,
    #[serde(rename = "avg_first_response_time")]
    #[display("avg_first_response_time")]
    AvgFirstResponseTime,
    #[serde(rename = "avg_handle_time")]
    #[display("avg_handle_time")]
    AvgHandleTime,
    #[serde(rename = "avg_response_time")]
    #[display("avg_response_time")]
    AvgResponseTime,
    #[serde(rename = "avg_sla_breach_time")]
    #[display("avg_sla_breach_time")]
    AvgSlaBreachTime,
    #[serde(rename = "avg_total_reply_time")]
    #[display("avg_total_reply_time")]
    AvgTotalReplyTime,
    #[serde(rename = "new_segments_count")]
    #[display("new_segments_count")]
    NewSegmentsCount,
    #[serde(rename = "num_active_segments_full")]
    #[display("num_active_segments_full")]
    NumActiveSegmentsFull,
    #[serde(rename = "num_archived_segments")]
    #[display("num_archived_segments")]
    NumArchivedSegments,
    #[serde(rename = "num_archived_segments_with_reply")]
    #[display("num_archived_segments_with_reply")]
    NumArchivedSegmentsWithReply,
    #[serde(rename = "num_csat_survey_response")]
    #[display("num_csat_survey_response")]
    NumCsatSurveyResponse,
    #[serde(rename = "num_messages_received")]
    #[display("num_messages_received")]
    NumMessagesReceived,
    #[serde(rename = "num_messages_sent")]
    #[display("num_messages_sent")]
    NumMessagesSent,
    #[serde(rename = "num_sla_breach")]
    #[display("num_sla_breach")]
    NumSlaBreach,
    #[serde(rename = "pct_csat_survey_satisfaction")]
    #[display("pct_csat_survey_satisfaction")]
    PctCsatSurveySatisfaction,
    #[serde(rename = "pct_tagged_conversations")]
    #[display("pct_tagged_conversations")]
    PctTaggedConversations,
    #[serde(rename = "num_open_segments_start")]
    #[display("num_open_segments_start")]
    NumOpenSegmentsStart,
    #[serde(rename = "num_closed_segments")]
    #[display("num_closed_segments")]
    NumClosedSegments,
    #[serde(rename = "num_open_segments_end")]
    #[display("num_open_segments_end")]
    NumOpenSegmentsEnd,
    #[serde(rename = "num_workload_segments")]
    #[display("num_workload_segments")]
    NumWorkloadSegments,
}

#[doc = "A message template folder that is used to store message templates or other folders."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateMessageTemplateFolderAsChild {
    #[doc = "Name of the message template folder"]
    pub name: String,
}

impl std::fmt::Display for CreateMessageTemplateFolderAsChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateMessageTemplateFolderAsChild {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![self.name.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string()]
    }
}

#[doc = "A message template folder that is used to store message templates or other folders."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateMessageTemplateFolder {
    #[doc = "Name of the message template folder"]
    pub name: String,
    #[doc = "ID of the parent folder to be placed into. Goes into the root folder if unspecified \
             or if null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

impl std::fmt::Display for CreateMessageTemplateFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateMessageTemplateFolder {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(parent_folder_id) = &self.parent_folder_id {
                format!("{:?}", parent_folder_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "parent_folder_id".to_string()]
    }
}

#[doc = "A message template folder that is used to store message templates or other folders."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateMessageTemplateFolder {
    #[doc = "Name of the message template folder"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "ID of the parent folder to be placed into. Goes into the root folder if unspecified \
             or if null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
}

impl std::fmt::Display for UpdateMessageTemplateFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateMessageTemplateFolder {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(parent_folder_id) = &self.parent_folder_id {
                format!("{:?}", parent_folder_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "parent_folder_id".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateMessageTemplate {
    #[doc = "Name of the message template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Subject of the message template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the message template"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "ID of the parent folder to be placed into. Goes into the root folder if unspecified \
             or if null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<Vec<String>>,
}

impl std::fmt::Display for UpdateMessageTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateMessageTemplate {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            if let Some(body) = &self.body {
                format!("{:?}", body)
            } else {
                String::new()
            },
            if let Some(folder_id) = &self.folder_id {
                format!("{:?}", folder_id)
            } else {
                String::new()
            },
            if let Some(inbox_ids) = &self.inbox_ids {
                format!("{:?}", inbox_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "folder_id".to_string(),
            "inbox_ids".to_string(),
        ]
    }
}

#[doc = "A message template that is used for pre-written responses"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateMessageTemplateAsChild {
    #[doc = "Name of the message template"]
    pub name: String,
    #[doc = "Subject of the message template. If not set, the name will be used to populate the \
             subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the message template"]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<Vec<String>>,
}

impl std::fmt::Display for CreateMessageTemplateAsChild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateMessageTemplateAsChild {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(inbox_ids) = &self.inbox_ids {
                format!("{:?}", inbox_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "inbox_ids".to_string(),
        ]
    }
}

#[doc = "A message template that is used for pre-written responses"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreatePrivateMessageTemplate {
    #[doc = "Name of the message template"]
    pub name: String,
    #[doc = "Subject of the message template. If not set, the name will be used to populate the \
             subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the message template"]
    pub body: String,
    #[doc = "ID of the message template folder to place this message template in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
}

impl std::fmt::Display for CreatePrivateMessageTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreatePrivateMessageTemplate {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(folder_id) = &self.folder_id {
                format!("{:?}", folder_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "folder_id".to_string(),
        ]
    }
}

#[doc = "A message template that is used for pre-written responses"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateSharedMessageTemplate {
    #[doc = "Name of the message template"]
    pub name: String,
    #[doc = "Subject of the message template. If not set, the name will be used to populate the \
             subject."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the message template"]
    pub body: String,
    #[doc = "ID of the message template folder to place this message template in"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<Vec<String>>,
}

impl std::fmt::Display for CreateSharedMessageTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateSharedMessageTemplate {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(folder_id) = &self.folder_id {
                format!("{:?}", folder_id)
            } else {
                String::new()
            },
            if let Some(inbox_ids) = &self.inbox_ids {
                format!("{:?}", inbox_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "folder_id".to_string(),
            "inbox_ids".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateContactGroup {
    #[doc = "Name of the group"]
    pub name: String,
}

impl std::fmt::Display for CreateContactGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateContactGroup {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![self.name.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddContactsToGroup {
    #[doc = "List of IDs of the contacts to add in the requested group"]
    pub contact_ids: Vec<String>,
}

impl std::fmt::Display for AddContactsToGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AddContactsToGroup {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.contact_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["contact_ids".to_string()]
    }
}

#[doc = "Source of the handle. Can be `email`, `phone`, `twitter`, `facebook`, `intercom`, \
         `front_chat`, or `custom`."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Source {
    #[serde(rename = "twitter")]
    #[display("twitter")]
    Twitter,
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "phone")]
    #[display("phone")]
    Phone,
    #[serde(rename = "facebook")]
    #[display("facebook")]
    Facebook,
    #[serde(rename = "intercom")]
    #[display("intercom")]
    Intercom,
    #[serde(rename = "front_chat")]
    #[display("front_chat")]
    FrontChat,
    #[serde(rename = "custom")]
    #[display("custom")]
    Custom,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactHandle {
    #[doc = "Handle used to reach the contact."]
    pub handle: String,
    #[doc = "Source of the handle. Can be `email`, `phone`, `twitter`, `facebook`, `intercom`, \
             `front_chat`, or `custom`."]
    pub source: Source,
}

impl std::fmt::Display for ContactHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactHandle {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.handle.clone(), format!("{:?}", self.source)]
    }

    fn headers() -> Vec<String> {
        vec!["handle".to_string(), "source".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateContact {
    #[doc = "Contact name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contact description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Binary data of avatar. Must use `Content-Type: multipart/form-data` if specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<bytes::Bytes>,
    #[doc = "Whether or not the contact is marked as a spammer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_spammer: Option<bool>,
    #[doc = "List of all the links of the contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[doc = "List of all the group names the contact belongs to. It will automatically create \
             missing groups"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_names: Option<Vec<String>>,
    #[doc = "Custom field attributes for this contact. Leave empty if you do not wish to update \
             the attributes. Not sending existing attributes will automatically remove them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<std::collections::HashMap<String, String>>,
    #[doc = "List of the handles for this contact. Each handle object should include `handle` and \
             `source` fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handles: Option<Vec<ContactHandle>>,
}

impl std::fmt::Display for CreateContact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateContact {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(avatar) = &self.avatar {
                format!("{:?}", avatar)
            } else {
                String::new()
            },
            if let Some(is_spammer) = &self.is_spammer {
                format!("{:?}", is_spammer)
            } else {
                String::new()
            },
            if let Some(links) = &self.links {
                format!("{:?}", links)
            } else {
                String::new()
            },
            if let Some(group_names) = &self.group_names {
                format!("{:?}", group_names)
            } else {
                String::new()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields)
            } else {
                String::new()
            },
            if let Some(handles) = &self.handles {
                format!("{:?}", handles)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "description".to_string(),
            "avatar".to_string(),
            "is_spammer".to_string(),
            "links".to_string(),
            "group_names".to_string(),
            "custom_fields".to_string(),
            "handles".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Contact {
    #[doc = "Contact name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contact description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Binary data of avatar. Must use `Content-Type: multipart/form-data` if specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<bytes::Bytes>,
    #[doc = "Whether or not the contact is marked as a spammer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_spammer: Option<bool>,
    #[doc = "List of all the links of the contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[doc = "List of all the group names the contact belongs to. It will automatically create \
             missing groups"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_names: Option<Vec<String>>,
    #[doc = "Custom field attributes for this contact. Leave empty if you do not wish to update \
             the attributes. Not sending existing attributes will automatically remove them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<std::collections::HashMap<String, String>>,
}

impl std::fmt::Display for Contact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Contact {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(avatar) = &self.avatar {
                format!("{:?}", avatar)
            } else {
                String::new()
            },
            if let Some(is_spammer) = &self.is_spammer {
                format!("{:?}", is_spammer)
            } else {
                String::new()
            },
            if let Some(links) = &self.links {
                format!("{:?}", links)
            } else {
                String::new()
            },
            if let Some(group_names) = &self.group_names {
                format!("{:?}", group_names)
            } else {
                String::new()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "description".to_string(),
            "avatar".to_string(),
            "is_spammer".to_string(),
            "links".to_string(),
            "group_names".to_string(),
            "custom_fields".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MergeContacts {
    #[doc = "Optional contact ID to merge the other contacts into."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_contact_id: Option<String>,
    #[doc = "Array of all the contact IDs of the contacts to be merged.  If a target contact id \
             is provided and that contact id is not in this array, the length of this array must \
             be between 1 and 49.  If no target contact id is provided or it is contained in this \
             array, the length must be between 2 and 50."]
    pub contact_ids: Vec<String>,
}

impl std::fmt::Display for MergeContacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MergeContacts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(target_contact_id) = &self.target_contact_id {
                format!("{:?}", target_contact_id)
            } else {
                String::new()
            },
            format!("{:?}", self.contact_ids),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["target_contact_id".to_string(), "contact_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeleteContactHandle {
    #[doc = "Handle used to reach the contact."]
    pub handle: String,
    #[doc = "Source of the handle. Can be `email`, `phone`, `twitter`, `facebook`, `intercom`, \
             `front_chat`, or `custom`."]
    pub source: Source,
    #[doc = "Force the deletetion of the contact if the handle is the last one"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
}

impl std::fmt::Display for DeleteContactHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for DeleteContactHandle {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            self.handle.clone(),
            format!("{:?}", self.source),
            if let Some(force) = &self.force {
                format!("{:?}", force)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "handle".to_string(),
            "source".to_string(),
            "force".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateContactNote {
    #[doc = "ID of teammate creating the note"]
    pub author_id: String,
    #[doc = "Content of the note"]
    pub body: String,
}

impl std::fmt::Display for CreateContactNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateContactNote {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.author_id.clone(), self.body.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["author_id".to_string(), "body".to_string()]
    }
}

#[doc = "Settings to replace.\nFor custom channels, all settings may be replaced.\nFor all other \
         channels, only `undo_send_time` and `all_teammates_can_reply` may be replaced.\n"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Settings {
    #[doc = "The time (measured in seconds) that users have to undo a send operation in the \
             channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_send_time: Option<i64>,
    #[doc = "Whether teammates without inbox access can reply on this channel. Only allowed for \
             shared channels."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_teammates_can_reply: Option<bool>,
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Settings {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(undo_send_time) = &self.undo_send_time {
                format!("{:?}", undo_send_time)
            } else {
                String::new()
            },
            if let Some(all_teammates_can_reply) = &self.all_teammates_can_reply {
                format!("{:?}", all_teammates_can_reply)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "undo_send_time".to_string(),
            "all_teammates_can_reply".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateChannel {
    #[doc = "Name of the channel"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Settings to replace.\nFor custom channels, all settings may be replaced.\nFor all \
             other channels, only `undo_send_time` and `all_teammates_can_reply` may be \
             replaced.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<Settings>,
}

impl std::fmt::Display for UpdateChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateChannel {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(settings) = &self.settings {
                format!("{:?}", settings)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "settings".to_string()]
    }
}

#[doc = "Settings of the channel"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateChannelSettings {
    #[doc = "The time (measured in seconds) that users have to undo a send operation in the \
             channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_send_time: Option<i64>,
    #[doc = "Whether teammates without inbox access can reply on this channel. Only allowed for \
             shared channels."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_teammates_can_reply: Option<bool>,
}

impl std::fmt::Display for CreateChannelSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateChannelSettings {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(undo_send_time) = &self.undo_send_time {
                format!("{:?}", undo_send_time)
            } else {
                String::new()
            },
            if let Some(all_teammates_can_reply) = &self.all_teammates_can_reply {
                format!("{:?}", all_teammates_can_reply)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "undo_send_time".to_string(),
            "all_teammates_can_reply".to_string(),
        ]
    }
}

#[doc = "Type of the channel"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum CreateChannelType {
    #[serde(rename = "custom")]
    #[display("custom")]
    Custom,
    #[serde(rename = "smtp")]
    #[display("smtp")]
    Smtp,
    #[serde(rename = "twilio")]
    #[display("twilio")]
    Twilio,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateChannel {
    #[doc = "Name of the channel"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Settings of the channel"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<CreateChannelSettings>,
    #[doc = "Type of the channel"]
    #[serde(rename = "type")]
    pub type_: CreateChannelType,
    #[doc = "Sending address of your channel"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_as: Option<String>,
}

impl std::fmt::Display for CreateChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateChannel {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(settings) = &self.settings {
                format!("{:?}", settings)
            } else {
                String::new()
            },
            format!("{:?}", self.type_),
            if let Some(send_as) = &self.send_as {
                format!("{:?}", send_as)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "settings".to_string(),
            "type_".to_string(),
            "send_as".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateComment {
    #[doc = "ID of the teammate creating the comment. If omitted, will post as the API Token or \
             OAuth client of the requester."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[doc = "Content of the comment"]
    pub body: String,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
}

impl std::fmt::Display for CreateComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateComment {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author_id".to_string(),
            "body".to_string(),
            "attachments".to_string(),
        ]
    }
}

#[doc = "Conversation type"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum CreateConversationType {
    #[serde(rename = "discussion")]
    #[display("discussion")]
    Discussion,
}

impl std::default::Default for CreateConversationType {
    fn default() -> Self {
        CreateConversationType::Discussion
    }
}

#[doc = "Details for the starter comment"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Comment {
    #[doc = "ID of the teammate creating the comment. If omitted, will post as the API Token or \
             OAuth client of the requester."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[doc = "Content of the comment"]
    pub body: String,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
}

impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Comment {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author_id".to_string(),
            "body".to_string(),
            "attachments".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateConversation {
    #[doc = "Conversation type"]
    #[serde(rename = "type")]
    pub type_: CreateConversationType,
    #[doc = "Inbox ID for the conversation. Either `inbox_id` OR `teammate_ids` must be provided \
             (not both)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    #[doc = "Teammates to add to the conversation. Either `inbox_id` OR `teammate_ids` must be \
             provided (not both)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teammate_ids: Option<Vec<String>>,
    #[doc = "Subject of the conversation"]
    pub subject: String,
    #[doc = "Details for the starter comment"]
    pub comment: Comment,
}

impl std::fmt::Display for CreateConversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateConversation {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.type_),
            if let Some(inbox_id) = &self.inbox_id {
                format!("{:?}", inbox_id)
            } else {
                String::new()
            },
            if let Some(teammate_ids) = &self.teammate_ids {
                format!("{:?}", teammate_ids)
            } else {
                String::new()
            },
            self.subject.clone(),
            format!("{:?}", self.comment),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "type_".to_string(),
            "inbox_id".to_string(),
            "teammate_ids".to_string(),
            "subject".to_string(),
            "comment".to_string(),
        ]
    }
}

#[doc = "New status of the conversation"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Status {
    #[serde(rename = "archived")]
    #[display("archived")]
    Archived,
    #[serde(rename = "open")]
    #[display("open")]
    Open,
    #[serde(rename = "deleted")]
    #[display("deleted")]
    Deleted,
    #[serde(rename = "spam")]
    #[display("spam")]
    Spam,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateConversation {
    #[doc = "ID of the teammate to assign the conversation to. Set it to null to unassign."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[doc = "ID of the inbox to move the conversation to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox_id: Option<String>,
    #[doc = "New status of the conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "List of all the tag IDs replacing the old conversation tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
}

impl std::fmt::Display for UpdateConversation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateConversation {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(assignee_id) = &self.assignee_id {
                format!("{:?}", assignee_id)
            } else {
                String::new()
            },
            if let Some(inbox_id) = &self.inbox_id {
                format!("{:?}", inbox_id)
            } else {
                String::new()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(tag_ids) = &self.tag_ids {
                format!("{:?}", tag_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "assignee_id".to_string(),
            "inbox_id".to_string(),
            "status".to_string(),
            "tag_ids".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateConversationAssignee {
    #[doc = "ID of the teammate to assign the conversation to. Set it to null to unassign."]
    pub assignee_id: String,
}

impl std::fmt::Display for UpdateConversationAssignee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateConversationAssignee {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![self.assignee_id.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["assignee_id".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateConversationReminders {
    #[doc = "ID of the teammate to create a reminder for. For a private conversation, specify the \
             id of the teammate that owns the conversation. For a shared conversation, use the id \
             of any teammate that has access to the conversation's shared inbox."]
    pub teammate_id: String,
    #[doc = "Timestamp to schedule the reminder for. Set to null to cancel."]
    pub scheduled_at: String,
}

impl std::fmt::Display for UpdateConversationReminders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateConversationReminders {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.teammate_id.clone(), self.scheduled_at.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["teammate_id".to_string(), "scheduled_at".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCustomField {
    #[doc = "Name of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl std::fmt::Display for UpdateCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateCustomField {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "description".to_string()]
    }
}

#[doc = "Mode of the draft to create. Can be 'private' (draft is visible to the author only) or \
         'shared' (draft is visible to all teammates with access to the conversation)."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Mode {
    #[serde(rename = "private")]
    #[display("private")]
    Private,
    #[serde(rename = "shared")]
    #[display("shared")]
    Shared,
}

impl std::default::Default for Mode {
    fn default() -> Self {
        Mode::Private
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateDraft {
    #[doc = "ID of the teammate on behalf of whom the draft will be created"]
    pub author_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[doc = "Subject of the draft."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the draft"]
    pub body: String,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
    #[doc = "Mode of the draft to create. Can be 'private' (draft is visible to the author only) \
             or 'shared' (draft is visible to all teammates with access to the conversation)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[doc = "ID of the signature to attach to this draft. If null, no signature is attached."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<String>,
    #[doc = "Whether or not Front should try to resolve a signature for the message. Is ignored \
             if signature_id is included. Default false;"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_add_default_signature: Option<bool>,
}

impl std::fmt::Display for CreateDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateDraft {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<String> {
        vec![
            self.author_id.clone(),
            if let Some(to) = &self.to {
                format!("{:?}", to)
            } else {
                String::new()
            },
            if let Some(cc) = &self.cc {
                format!("{:?}", cc)
            } else {
                String::new()
            },
            if let Some(bcc) = &self.bcc {
                format!("{:?}", bcc)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
            if let Some(mode) = &self.mode {
                format!("{:?}", mode)
            } else {
                String::new()
            },
            if let Some(signature_id) = &self.signature_id {
                format!("{:?}", signature_id)
            } else {
                String::new()
            },
            if let Some(should_add_default_signature) = &self.should_add_default_signature {
                format!("{:?}", should_add_default_signature)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author_id".to_string(),
            "to".to_string(),
            "cc".to_string(),
            "bcc".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "attachments".to_string(),
            "mode".to_string(),
            "signature_id".to_string(),
            "should_add_default_signature".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ReplyDraft {
    #[doc = "ID of the teammate on behalf of whom the draft will be created"]
    pub author_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[doc = "Subject of the draft."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the draft"]
    pub body: String,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
    #[doc = "Mode of the draft to create. Can be 'private' (draft is visible to the author only) \
             or 'shared' (draft is visible to all teammates with access to the conversation)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    #[doc = "ID of the signature to attach to this draft. If null, no signature is attached."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<String>,
    #[doc = "Whether or not Front should try to resolve a signature for the message. Is ignored \
             if signature_id is included. Default false;"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_add_default_signature: Option<bool>,
    #[doc = "ID of the channel from which the draft will be sent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

impl std::fmt::Display for ReplyDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ReplyDraft {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<String> {
        vec![
            self.author_id.clone(),
            if let Some(to) = &self.to {
                format!("{:?}", to)
            } else {
                String::new()
            },
            if let Some(cc) = &self.cc {
                format!("{:?}", cc)
            } else {
                String::new()
            },
            if let Some(bcc) = &self.bcc {
                format!("{:?}", bcc)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
            if let Some(mode) = &self.mode {
                format!("{:?}", mode)
            } else {
                String::new()
            },
            if let Some(signature_id) = &self.signature_id {
                format!("{:?}", signature_id)
            } else {
                String::new()
            },
            if let Some(should_add_default_signature) = &self.should_add_default_signature {
                format!("{:?}", should_add_default_signature)
            } else {
                String::new()
            },
            if let Some(channel_id) = &self.channel_id {
                format!("{:?}", channel_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author_id".to_string(),
            "to".to_string(),
            "cc".to_string(),
            "bcc".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "attachments".to_string(),
            "mode".to_string(),
            "signature_id".to_string(),
            "should_add_default_signature".to_string(),
            "channel_id".to_string(),
        ]
    }
}

#[doc = "Mode of the draft to update. Can only be 'shared' (draft is visible to all teammates with \
         access to the conversation)."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum EditDraftMode {
    #[serde(rename = "shared")]
    #[display("shared")]
    Shared,
}

impl std::default::Default for EditDraftMode {
    fn default() -> Self {
        EditDraftMode::Shared
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EditDraft {
    #[doc = "ID of the teammate on behalf of whom the draft will be created"]
    pub author_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[doc = "Subject of the draft."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the draft"]
    pub body: String,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
    #[doc = "Mode of the draft to update. Can only be 'shared' (draft is visible to all teammates \
             with access to the conversation)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<EditDraftMode>,
    #[doc = "ID of the signature to attach to this draft. If null, no signature is attached."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<String>,
    #[doc = "Whether or not Front should try to resolve a signature for the message. Is ignored \
             if signature_id is included. Default false;"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_add_default_signature: Option<bool>,
    #[doc = "ID of the channel from which the draft will be sent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[doc = "Version of the draft"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl std::fmt::Display for EditDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for EditDraft {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<String> {
        vec![
            self.author_id.clone(),
            if let Some(to) = &self.to {
                format!("{:?}", to)
            } else {
                String::new()
            },
            if let Some(cc) = &self.cc {
                format!("{:?}", cc)
            } else {
                String::new()
            },
            if let Some(bcc) = &self.bcc {
                format!("{:?}", bcc)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
            if let Some(mode) = &self.mode {
                format!("{:?}", mode)
            } else {
                String::new()
            },
            if let Some(signature_id) = &self.signature_id {
                format!("{:?}", signature_id)
            } else {
                String::new()
            },
            if let Some(should_add_default_signature) = &self.should_add_default_signature {
                format!("{:?}", should_add_default_signature)
            } else {
                String::new()
            },
            if let Some(channel_id) = &self.channel_id {
                format!("{:?}", channel_id)
            } else {
                String::new()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author_id".to_string(),
            "to".to_string(),
            "cc".to_string(),
            "bcc".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "attachments".to_string(),
            "mode".to_string(),
            "signature_id".to_string(),
            "should_add_default_signature".to_string(),
            "channel_id".to_string(),
            "version".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeleteDraft {
    #[doc = "Version of the draft"]
    pub version: String,
}

impl std::fmt::Display for DeleteDraft {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for DeleteDraft {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![self.version.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["version".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateInbox {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teammate_ids: Option<Vec<String>>,
}

impl std::fmt::Display for CreateInbox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateInbox {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(teammate_ids) = &self.teammate_ids {
                format!("{:?}", teammate_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "teammate_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Options {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    #[doc = "Archive the conversation right when sending the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<bool>,
}

impl std::fmt::Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Options {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tag_ids) = &self.tag_ids {
                format!("{:?}", tag_ids)
            } else {
                String::new()
            },
            if let Some(archive) = &self.archive {
                format!("{:?}", archive)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["tag_ids".to_string(), "archive".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct OutboundMessage {
    pub to: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[doc = "Name used for the sender info of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Subject of the message for email message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "ID of the teammate on behalf of whom the answer is sent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[doc = "Body of the message"]
    pub body: String,
    #[doc = "Text version of the body for email messages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Options>,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
    #[doc = "ID of the signature to attach to this draft. If null, no signature is attached."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<String>,
    #[doc = "Whether or not Front should try to resolve a signature for the message. Is ignored \
             if signature_id is included. Default false;"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_add_default_signature: Option<bool>,
}

impl std::fmt::Display for OutboundMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for OutboundMessage {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.to),
            if let Some(cc) = &self.cc {
                format!("{:?}", cc)
            } else {
                String::new()
            },
            if let Some(bcc) = &self.bcc {
                format!("{:?}", bcc)
            } else {
                String::new()
            },
            if let Some(sender_name) = &self.sender_name {
                format!("{:?}", sender_name)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(text) = &self.text {
                format!("{:?}", text)
            } else {
                String::new()
            },
            if let Some(options) = &self.options {
                format!("{:?}", options)
            } else {
                String::new()
            },
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
            if let Some(signature_id) = &self.signature_id {
                format!("{:?}", signature_id)
            } else {
                String::new()
            },
            if let Some(should_add_default_signature) = &self.should_add_default_signature {
                format!("{:?}", should_add_default_signature)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "to".to_string(),
            "cc".to_string(),
            "bcc".to_string(),
            "sender_name".to_string(),
            "subject".to_string(),
            "author_id".to_string(),
            "body".to_string(),
            "text".to_string(),
            "options".to_string(),
            "attachments".to_string(),
            "signature_id".to_string(),
            "should_add_default_signature".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct OutboundReplyMessageOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_ids: Option<Vec<String>>,
    #[doc = "Archive the conversation right when sending the message. `true` by default"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive: Option<bool>,
}

impl std::fmt::Display for OutboundReplyMessageOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for OutboundReplyMessageOptions {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tag_ids) = &self.tag_ids {
                format!("{:?}", tag_ids)
            } else {
                String::new()
            },
            if let Some(archive) = &self.archive {
                format!("{:?}", archive)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["tag_ids".to_string(), "archive".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct OutboundReplyMessage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[doc = "Name used for the sender info of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    #[doc = "Subject of the message for email message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "ID of the teammate on behalf of whom the answer is sent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[doc = "Channel ID the message is sent from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[doc = "Body of the message"]
    pub body: String,
    #[doc = "Text version of the body for email messages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<OutboundReplyMessageOptions>,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
    #[doc = "ID of the signature to attach to this draft. If null, no signature is attached."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_id: Option<String>,
    #[doc = "Whether or not Front should try to resolve a signature for the message. Is ignored \
             if signature_id is included. Default false;"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_add_default_signature: Option<bool>,
}

impl std::fmt::Display for OutboundReplyMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for OutboundReplyMessage {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(to) = &self.to {
                format!("{:?}", to)
            } else {
                String::new()
            },
            if let Some(cc) = &self.cc {
                format!("{:?}", cc)
            } else {
                String::new()
            },
            if let Some(bcc) = &self.bcc {
                format!("{:?}", bcc)
            } else {
                String::new()
            },
            if let Some(sender_name) = &self.sender_name {
                format!("{:?}", sender_name)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id)
            } else {
                String::new()
            },
            if let Some(channel_id) = &self.channel_id {
                format!("{:?}", channel_id)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(text) = &self.text {
                format!("{:?}", text)
            } else {
                String::new()
            },
            if let Some(options) = &self.options {
                format!("{:?}", options)
            } else {
                String::new()
            },
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
            if let Some(signature_id) = &self.signature_id {
                format!("{:?}", signature_id)
            } else {
                String::new()
            },
            if let Some(should_add_default_signature) = &self.should_add_default_signature {
                format!("{:?}", should_add_default_signature)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "to".to_string(),
            "cc".to_string(),
            "bcc".to_string(),
            "sender_name".to_string(),
            "subject".to_string(),
            "author_id".to_string(),
            "channel_id".to_string(),
            "body".to_string(),
            "text".to_string(),
            "options".to_string(),
            "attachments".to_string(),
            "signature_id".to_string(),
            "should_add_default_signature".to_string(),
        ]
    }
}

#[doc = "Data of the sender"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Sender {
    #[doc = "ID of the contact in Front corresponding to the sender"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[doc = "Name of the sender"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Handle of the sender. It can be any string used to uniquely identify the sender"]
    pub handle: String,
}

impl std::fmt::Display for Sender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Sender {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(contact_id) = &self.contact_id {
                format!("{:?}", contact_id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            self.handle.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "contact_id".to_string(),
            "name".to_string(),
            "handle".to_string(),
        ]
    }
}

#[doc = "Format of the message body. Can be `markdown` (default) or `html`."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum BodyFormat {
    #[serde(rename = "html")]
    #[display("html")]
    Html,
    #[serde(rename = "markdown")]
    #[display("markdown")]
    Markdown,
}

impl std::default::Default for BodyFormat {
    fn default() -> Self {
        BodyFormat::Markdown
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Metadata {
    #[doc = "Reference which will be used to thread messages. If  omitted, Front threads by \
             sender instead"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_ref: Option<String>,
    #[doc = "Custom object where any internal information can be stored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

impl std::fmt::Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Metadata {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(thread_ref) = &self.thread_ref {
                format!("{:?}", thread_ref)
            } else {
                String::new()
            },
            if let Some(headers) = &self.headers {
                format!("{:?}", headers)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["thread_ref".to_string(), "headers".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomMessage {
    #[doc = "Data of the sender"]
    pub sender: Sender,
    #[doc = "Subject of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the message"]
    pub body: String,
    #[doc = "Format of the message body. Can be `markdown` (default) or `html`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_format: Option<BodyFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
}

impl std::fmt::Display for CustomMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CustomMessage {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.sender),
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(body_format) = &self.body_format {
                format!("{:?}", body_format)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "sender".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "body_format".to_string(),
            "metadata".to_string(),
            "attachments".to_string(),
        ]
    }
}

#[doc = "Data of the sender"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ImportMessageSender {
    #[doc = "ID of the teammate who is the author of the message. Ignored if the message is \
             inbound."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[doc = "Name of the sender"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Handle of the sender. It can be any string used to uniquely identify the sender"]
    pub handle: String,
}

impl std::fmt::Display for ImportMessageSender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ImportMessageSender {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            self.handle.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author_id".to_string(),
            "name".to_string(),
            "handle".to_string(),
        ]
    }
}

#[doc = "Format of the message body. Can be `markdown` (default) or `html`, and can only be \
         specified for `email` type."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum ImportMessageBodyFormat {
    #[serde(rename = "html")]
    #[display("html")]
    Html,
    #[serde(rename = "markdown")]
    #[display("markdown")]
    Markdown,
}

impl std::default::Default for ImportMessageBodyFormat {
    fn default() -> Self {
        ImportMessageBodyFormat::Markdown
    }
}

#[doc = "Type of the message to import. Default is `email`."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum ImportMessageType {
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "sms")]
    #[display("sms")]
    Sms,
    #[serde(rename = "intercom")]
    #[display("intercom")]
    Intercom,
    #[serde(rename = "custom")]
    #[display("custom")]
    Custom,
}

impl std::default::Default for ImportMessageType {
    fn default() -> Self {
        ImportMessageType::Email
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ImportMessageMetadata {
    #[doc = "Reference which will be used to thread messages. If  omitted, Front threads by \
             sender instead"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_ref: Option<String>,
    #[doc = "Determines if message is received (inbound) or sent (outbound) by you."]
    pub is_inbound: bool,
    #[doc = "Determines if message is archived after import."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc = "Determines if rules should be skipped. `true` by default."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_skip_rules: Option<bool>,
}

impl std::fmt::Display for ImportMessageMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ImportMessageMetadata {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(thread_ref) = &self.thread_ref {
                format!("{:?}", thread_ref)
            } else {
                String::new()
            },
            format!("{:?}", self.is_inbound),
            if let Some(is_archived) = &self.is_archived {
                format!("{:?}", is_archived)
            } else {
                String::new()
            },
            if let Some(should_skip_rules) = &self.should_skip_rules {
                format!("{:?}", should_skip_rules)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "thread_ref".to_string(),
            "is_inbound".to_string(),
            "is_archived".to_string(),
            "should_skip_rules".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ImportMessage {
    #[doc = "Data of the sender"]
    pub sender: ImportMessageSender,
    pub to: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<String>>,
    #[doc = "Subject of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the message"]
    pub body: String,
    #[doc = "Format of the message body. Can be `markdown` (default) or `html`, and can only be \
             specified for `email` type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_format: Option<ImportMessageBodyFormat>,
    #[doc = "External identifier of the message. Front won't import two messages with the same \
             external ID."]
    pub external_id: String,
    #[doc = "Date at which the message as been sent or received."]
    pub created_at: i64,
    #[doc = "Type of the message to import. Default is `email`."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ImportMessageType>,
    #[doc = "ID of the teammate who will be assigned to the conversation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<String>,
    #[doc = "List of tag names to add to the conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    pub metadata: ImportMessageMetadata,
    #[doc = "Binary data of attached files. Must use `Content-Type: multipart/form-data` if specified. See [example](https://gist.github.com/hdornier/e04d04921032e98271f46ff8a539a4cb)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<bytes::Bytes>>,
}

impl std::fmt::Display for ImportMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ImportMessage {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.sender),
            format!("{:?}", self.to),
            if let Some(cc) = &self.cc {
                format!("{:?}", cc)
            } else {
                String::new()
            },
            if let Some(bcc) = &self.bcc {
                format!("{:?}", bcc)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(body_format) = &self.body_format {
                format!("{:?}", body_format)
            } else {
                String::new()
            },
            self.external_id.clone(),
            format!("{:?}", self.created_at),
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
            if let Some(assignee_id) = &self.assignee_id {
                format!("{:?}", assignee_id)
            } else {
                String::new()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags)
            } else {
                String::new()
            },
            format!("{:?}", self.metadata),
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "sender".to_string(),
            "to".to_string(),
            "cc".to_string(),
            "bcc".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "body_format".to_string(),
            "external_id".to_string(),
            "created_at".to_string(),
            "type_".to_string(),
            "assignee_id".to_string(),
            "tags".to_string(),
            "metadata".to_string(),
            "attachments".to_string(),
        ]
    }
}

#[doc = "Color of the shift"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Color {
    #[serde(rename = "black")]
    #[display("black")]
    Black,
    #[serde(rename = "grey")]
    #[display("grey")]
    Grey,
    #[serde(rename = "pink")]
    #[display("pink")]
    Pink,
    #[serde(rename = "purple")]
    #[display("purple")]
    Purple,
    #[serde(rename = "blue")]
    #[display("blue")]
    Blue,
    #[serde(rename = "teal")]
    #[display("teal")]
    Teal,
    #[serde(rename = "green")]
    #[display("green")]
    Green,
    #[serde(rename = "yellow")]
    #[display("yellow")]
    Yellow,
    #[serde(rename = "orange")]
    #[display("orange")]
    Orange,
    #[serde(rename = "red")]
    #[display("red")]
    Red,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateShift {
    #[doc = "Name of the shift"]
    pub name: String,
    #[doc = "Color of the shift"]
    pub color: Color,
    #[doc = "A timezone name as defined in the IANA tz database"]
    pub timezone: String,
    pub times: ShiftIntervals,
    pub teammate_ids: Vec<String>,
}

impl std::fmt::Display for CreateShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateShift {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            format!("{:?}", self.color),
            self.timezone.clone(),
            format!("{:?}", self.times),
            format!("{:?}", self.teammate_ids),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "color".to_string(),
            "timezone".to_string(),
            "times".to_string(),
            "teammate_ids".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateShift {
    #[doc = "Name of the shift"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Color of the shift"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    #[doc = "A timezone name as defined in the IANA tz database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub times: Option<ShiftIntervals>,
    #[doc = "List of all the teammate ids who will be part of this shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teammate_ids: Option<Vec<String>>,
}

impl std::fmt::Display for UpdateShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateShift {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(color) = &self.color {
                format!("{:?}", color)
            } else {
                String::new()
            },
            if let Some(timezone) = &self.timezone {
                format!("{:?}", timezone)
            } else {
                String::new()
            },
            if let Some(times) = &self.times {
                format!("{:?}", times)
            } else {
                String::new()
            },
            if let Some(teammate_ids) = &self.teammate_ids {
                format!("{:?}", teammate_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "color".to_string(),
            "timezone".to_string(),
            "times".to_string(),
            "teammate_ids".to_string(),
        ]
    }
}

#[doc = "A signature that can be used to sign messages."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreatePrivateSignature {
    #[doc = "Name of the signature"]
    pub name: String,
    #[doc = "Sender info of the signature that will appear in the From line of emails sent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_info: Option<String>,
    #[doc = "Body of the signature"]
    pub body: String,
    #[doc = "If true, the signature will be set as the default signature for the teammate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl std::fmt::Display for CreatePrivateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreatePrivateSignature {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(sender_info) = &self.sender_info {
                format!("{:?}", sender_info)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(is_default) = &self.is_default {
                format!("{:?}", is_default)
            } else {
                String::new()
            },
            if let Some(channel_ids) = &self.channel_ids {
                format!("{:?}", channel_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "sender_info".to_string(),
            "body".to_string(),
            "is_default".to_string(),
            "channel_ids".to_string(),
        ]
    }
}

#[doc = "A signature that can be used to sign messages."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateSharedSignature {
    #[doc = "Name of the signature"]
    pub name: String,
    #[doc = "Sender info of the signature that will appear in the From line of emails sent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_info: Option<String>,
    #[doc = "Body of the signature"]
    pub body: String,
    #[doc = "Whether or not the signature is visible in all individual channels for teammates in \
             the given team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible_for_all_teammate_channels: Option<bool>,
    #[doc = "If true, the signature will be set as the default signature for the team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl std::fmt::Display for CreateSharedSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateSharedSignature {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(sender_info) = &self.sender_info {
                format!("{:?}", sender_info)
            } else {
                String::new()
            },
            self.body.clone(),
            if let Some(is_visible_for_all_teammate_channels) =
                &self.is_visible_for_all_teammate_channels
            {
                format!("{:?}", is_visible_for_all_teammate_channels)
            } else {
                String::new()
            },
            if let Some(is_default) = &self.is_default {
                format!("{:?}", is_default)
            } else {
                String::new()
            },
            if let Some(channel_ids) = &self.channel_ids {
                format!("{:?}", channel_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "sender_info".to_string(),
            "body".to_string(),
            "is_visible_for_all_teammate_channels".to_string(),
            "is_default".to_string(),
            "channel_ids".to_string(),
        ]
    }
}

#[doc = "A signature that can be used to sign messages."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateSignature {
    #[doc = "Name of the signature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Sender info of the signature that will appear in the From line of emails sent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_info: Option<String>,
    #[doc = "Body of the signature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Whether or not the signature is visible in all individual channels for teammates in \
             the given team. Can only be set for shared signatures."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible_for_all_teammate_channels: Option<bool>,
    #[doc = "If true, the signature will be set as the default signature for the team or teammate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl std::fmt::Display for UpdateSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateSignature {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(sender_info) = &self.sender_info {
                format!("{:?}", sender_info)
            } else {
                String::new()
            },
            if let Some(body) = &self.body {
                format!("{:?}", body)
            } else {
                String::new()
            },
            if let Some(is_visible_for_all_teammate_channels) =
                &self.is_visible_for_all_teammate_channels
            {
                format!("{:?}", is_visible_for_all_teammate_channels)
            } else {
                String::new()
            },
            if let Some(is_default) = &self.is_default {
                format!("{:?}", is_default)
            } else {
                String::new()
            },
            if let Some(channel_ids) = &self.channel_ids {
                format!("{:?}", channel_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "sender_info".to_string(),
            "body".to_string(),
            "is_visible_for_all_teammate_channels".to_string(),
            "is_default".to_string(),
            "channel_ids".to_string(),
        ]
    }
}

#[doc = "Highlight color of the tag."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Highlight {
    #[serde(rename = "grey")]
    #[display("grey")]
    Grey,
    #[serde(rename = "pink")]
    #[display("pink")]
    Pink,
    #[serde(rename = "red")]
    #[display("red")]
    Red,
    #[serde(rename = "orange")]
    #[display("orange")]
    Orange,
    #[serde(rename = "yellow")]
    #[display("yellow")]
    Yellow,
    #[serde(rename = "green")]
    #[display("green")]
    Green,
    #[serde(rename = "light-blue")]
    #[display("light-blue")]
    LightBlue,
    #[serde(rename = "blue")]
    #[display("blue")]
    Blue,
    #[serde(rename = "purple")]
    #[display("purple")]
    Purple,
}

#[doc = "A tag is a label that can be used to classify conversations."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTag {
    #[doc = "Name of the tag"]
    pub name: String,
    #[doc = "Highlight color of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Highlight>,
    #[doc = "Whether the tag is visible in conversation lists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible_in_conversation_lists: Option<bool>,
}

impl std::fmt::Display for CreateTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateTag {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            if let Some(highlight) = &self.highlight {
                format!("{:?}", highlight)
            } else {
                String::new()
            },
            if let Some(is_visible_in_conversation_lists) = &self.is_visible_in_conversation_lists {
                format!("{:?}", is_visible_in_conversation_lists)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "highlight".to_string(),
            "is_visible_in_conversation_lists".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTag {
    #[doc = "Name of the tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Highlight color of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Highlight>,
    #[doc = "ID of the parent of this tag. Set to `null` to remove  the parent tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_tag_id: Option<String>,
    #[doc = "Whether the tag is visible in conversation lists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible_in_conversation_lists: Option<bool>,
}

impl std::fmt::Display for UpdateTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateTag {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(highlight) = &self.highlight {
                format!("{:?}", highlight)
            } else {
                String::new()
            },
            if let Some(parent_tag_id) = &self.parent_tag_id {
                format!("{:?}", parent_tag_id)
            } else {
                String::new()
            },
            if let Some(is_visible_in_conversation_lists) = &self.is_visible_in_conversation_lists {
                format!("{:?}", is_visible_in_conversation_lists)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "highlight".to_string(),
            "parent_tag_id".to_string(),
            "is_visible_in_conversation_lists".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTeammate {
    #[doc = "New username. It must be unique and can only contains lowercase letters, numbers and \
             underscores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "New first name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "New last name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "New availability status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
}

impl std::fmt::Display for UpdateTeammate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateTeammate {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(username) = &self.username {
                format!("{:?}", username)
            } else {
                String::new()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name)
            } else {
                String::new()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name)
            } else {
                String::new()
            },
            if let Some(is_available) = &self.is_available {
                format!("{:?}", is_available)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "username".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
            "is_available".to_string(),
        ]
    }
}

#[doc = "A link is used to connect a Front conversation to an external resource."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateLink {
    #[doc = "Name of the link. If none is specified, the external_url is used as a default"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Underlying identifying url of the link"]
    pub external_url: String,
}

impl std::fmt::Display for CreateLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateLink {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            self.external_url.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "external_url".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateLink {
    #[doc = "Name of the link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for UpdateLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateLink {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(name) = &self.name {
            format!("{:?}", name)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Related {
    #[doc = "Link to contacts associated to the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<String>,
}

impl std::fmt::Display for Related {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Related {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(contacts) = &self.contacts {
            format!("{:?}", contacts)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["contacts".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<Related>,
}

impl std::fmt::Display for UnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AccountResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<UnderscoreLinks>,
    #[doc = "Unique identifier of the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Account name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "URL of the Account's logo"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[doc = "Account Description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    #[doc = "ID of the Account in an External system, such as your backoffice system or CRM"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "Custom Attributes for this account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<std::collections::HashMap<String, String>>,
    #[doc = "Timestamp when the account was created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[doc = "Timestamp when the account was updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl std::fmt::Display for AccountResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AccountResponse {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(logo_url) = &self.logo_url {
                format!("{:?}", logo_url)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(domains) = &self.domains {
                format!("{:?}", domains)
            } else {
                String::new()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id)
            } else {
                String::new()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "logo_url".to_string(),
            "description".to_string(),
            "domains".to_string(),
            "external_id".to_string(),
            "custom_fields".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EventResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for EventResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for EventResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[doc = "Type of event"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum EventResponseType {
    #[serde(rename = "assign")]
    #[display("assign")]
    Assign,
    #[serde(rename = "unassign")]
    #[display("unassign")]
    Unassign,
    #[serde(rename = "archive")]
    #[display("archive")]
    Archive,
    #[serde(rename = "reopen")]
    #[display("reopen")]
    Reopen,
    #[serde(rename = "trash")]
    #[display("trash")]
    Trash,
    #[serde(rename = "restore")]
    #[display("restore")]
    Restore,
    #[serde(rename = "comment")]
    #[display("comment")]
    Comment,
    #[serde(rename = "inbound")]
    #[display("inbound")]
    Inbound,
    #[serde(rename = "outbound")]
    #[display("outbound")]
    Outbound,
    #[serde(rename = "move")]
    #[display("move")]
    Move,
    #[serde(rename = "forward")]
    #[display("forward")]
    Forward,
    #[serde(rename = "tag")]
    #[display("tag")]
    Tag,
    #[serde(rename = "untag")]
    #[display("untag")]
    Untag,
    #[serde(rename = "sending_error")]
    #[display("sending_error")]
    SendingError,
}

#[doc = "Type of resource"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum MetaType {
    #[serde(rename = "teamate")]
    #[display("teamate")]
    Teamate,
    #[serde(rename = "inbox")]
    #[display("inbox")]
    Inbox,
    #[serde(rename = "tag")]
    #[display("tag")]
    Tag,
    #[serde(rename = "comment")]
    #[display("comment")]
    Comment,
    #[serde(rename = "message")]
    #[display("message")]
    Message,
}

#[doc = "Metadata about the resource"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Meta {
    #[doc = "Type of resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<MetaType>,
}

impl std::fmt::Display for Meta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Meta {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(type_) = &self.type_ {
            format!("{:?}", type_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["type_".to_string()]
    }
}

#[doc = "The resource which triggered the event"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
pub enum Data {
    RuleResponse(RuleResponse),
    TeammateResponse(TeammateResponse),
    InboxResponse(Vec<InboxResponse>),
}

#[doc = "Event source"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EventResponseSource {
    #[doc = "Metadata about the resource"]
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "The resource which triggered the event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl std::fmt::Display for EventResponseSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for EventResponseSource {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta)
            } else {
                String::new()
            },
            if let Some(data) = &self.data {
                format!("{:?}", data)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["meta".to_string(), "data".to_string()]
    }
}

#[doc = "Type of resource"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum TargetMetaType {
    #[serde(rename = "teamate")]
    #[display("teamate")]
    Teamate,
    #[serde(rename = "inbox")]
    #[display("inbox")]
    Inbox,
    #[serde(rename = "tag")]
    #[display("tag")]
    Tag,
    #[serde(rename = "comment")]
    #[display("comment")]
    Comment,
    #[serde(rename = "message")]
    #[display("message")]
    Message,
    #[serde(rename = "link")]
    #[display("link")]
    Link,
}

#[doc = "The resource which received the event"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
pub enum TargetMetaData {
    TeammateResponse(TeammateResponse),
    InboxResponse(InboxResponse),
    TagResponse(TagResponse),
    CommentResponse(CommentResponse),
    MessageResponse(MessageResponse),
    LinkResponse(LinkResponse),
}

#[doc = "Metadata about the resource"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TargetMeta {
    #[doc = "Type of resource"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<TargetMetaType>,
    #[doc = "The resource which received the event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TargetMetaData>,
}

impl std::fmt::Display for TargetMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TargetMeta {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
            if let Some(data) = &self.data {
                format!("{:?}", data)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["type_".to_string(), "data".to_string()]
    }
}

#[doc = "Partial representation (type & id) of the event's target"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Target {
    #[doc = "Metadata about the resource"]
    #[serde(rename = "_meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<TargetMeta>,
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Target {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(meta) = &self.meta {
            format!("{:?}", meta)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["meta".to_string()]
    }
}

#[doc = "An event is created everytime something interesting is happening in Front."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EventResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<EventResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the event"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Type of event"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<EventResponseType>,
    #[doc = "Date at which the event has been emitted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emitted_at: Option<String>,
    #[doc = "Event source"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<EventResponseSource>,
    #[doc = "Partial representation (type & id) of the event's target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<Target>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ConversationResponse>,
}

impl std::fmt::Display for EventResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for EventResponse {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
            if let Some(emitted_at) = &self.emitted_at {
                format!("{:?}", emitted_at)
            } else {
                String::new()
            },
            if let Some(source) = &self.source {
                format!("{:?}", source)
            } else {
                String::new()
            },
            if let Some(target) = &self.target {
                format!("{:?}", target)
            } else {
                String::new()
            },
            if let Some(conversation) = &self.conversation {
                format!("{:?}", conversation)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "type_".to_string(),
            "emitted_at".to_string(),
            "source".to_string(),
            "target".to_string(),
            "conversation".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct IdentityResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for IdentityResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for IdentityResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct IdentityResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<IdentityResponseUnderscoreLinks>,
    #[doc = "Unique ID of company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Name of company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for IdentityResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for IdentityResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsExportResponse2UnderscoreLinks {
    #[doc = "Link to analytics export"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for AnalyticsExportResponse2UnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsExportResponse2UnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[doc = "Status of the analytics"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum AnalyticsExportResponse2Status {
    #[serde(rename = "running")]
    #[display("running")]
    Running,
    #[serde(rename = "done")]
    #[display("done")]
    Done,
    #[serde(rename = "failed")]
    #[display("failed")]
    Failed,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsExportResponse2 {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<AnalyticsExportResponse2UnderscoreLinks>,
    #[doc = "Status of the analytics"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AnalyticsExportResponse2Status>,
    #[doc = "Number ranging from 0 to 100 corresponding to the percentage of the analytics \
             processed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[doc = "The URL from which the export data can be downloaded."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Size (in bytes) of the export data."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[doc = "Timestamp (in seconds) at which the export was requested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[doc = "Resources to compute the analytics for. Defaults to all."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<AnalyticsFilters>,
}

impl std::fmt::Display for AnalyticsExportResponse2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsExportResponse2 {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(progress) = &self.progress {
                format!("{:?}", progress)
            } else {
                String::new()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url)
            } else {
                String::new()
            },
            if let Some(size) = &self.size {
                format!("{:?}", size)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(filters) = &self.filters {
                format!("{:?}", filters)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "status".to_string(),
            "progress".to_string(),
            "url".to_string(),
            "size".to_string(),
            "created_at".to_string(),
            "filters".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsReportResponse2UnderscoreLinks {
    #[doc = "Link to analytics job."]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for AnalyticsReportResponse2UnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsReportResponse2UnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[doc = "Status of the report."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum AnalyticsReportResponse2Status {
    #[serde(rename = "running")]
    #[display("running")]
    Running,
    #[serde(rename = "done")]
    #[display("done")]
    Done,
    #[serde(rename = "failed")]
    #[display("failed")]
    Failed,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsReportResponse2 {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<AnalyticsReportResponse2UnderscoreLinks>,
    #[doc = "Status of the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<AnalyticsReportResponse2Status>,
    #[doc = "Number ranging from 0 to 100 corresponding to the percentage of the analytics \
             processed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    #[doc = "The metrics computed for the report."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<AnalyticsScalar2>>,
}

impl std::fmt::Display for AnalyticsReportResponse2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsReportResponse2 {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(progress) = &self.progress {
                format!("{:?}", progress)
            } else {
                String::new()
            },
            if let Some(metrics) = &self.metrics {
                format!("{:?}", metrics)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "status".to_string(),
            "progress".to_string(),
            "metrics".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsScalar2 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<AnalyticsMetricId>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AnalyticsScalarType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<AnalyticsScalarValue>,
}

impl std::fmt::Display for AnalyticsScalar2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsScalar2 {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["id".to_string(), "type_".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum AnalyticsScalarType {
    #[serde(rename = "number")]
    #[display("number")]
    Number,
    #[serde(rename = "percentage")]
    #[display("percentage")]
    Percentage,
    #[serde(rename = "string")]
    #[display("string")]
    String,
    #[serde(rename = "duration")]
    #[display("duration")]
    Duration,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ResourceUnderscoreLinks {
    #[doc = "Link to a resource."]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ResourceUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ResourceUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Resource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ResourceUnderscoreLinks>,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Resource {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["id".to_string(), "underscore_links".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ValueOneOf {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<Resource>,
}

impl std::fmt::Display for ValueOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ValueOneOf {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(label) = &self.label {
                format!("{:?}", label)
            } else {
                String::new()
            },
            if let Some(resource) = &self.resource {
                format!("{:?}", resource)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["label".to_string(), "resource".to_string()]
    }
}

#[doc = "The value of a scalar metric."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
pub enum Value {
    I64(i64),
    String(String),
    ValueOneOf(ValueOneOf),
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsScalarValue {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<AnalyticsMetricId>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<AnalyticsScalarType>,
    #[doc = "The value of a scalar metric."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl std::fmt::Display for AnalyticsScalarValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AnalyticsScalarValue {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["id".to_string(), "type_".to_string(), "value".to_string()]
    }
}

#[doc = "Attachment metadata"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AttachmentMetadata {
    #[doc = "Whether or not the attachment is part of the message body"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inline: Option<bool>,
    #[doc = "Unique identifier used to link an attachment to where it is used in the message body"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cid: Option<String>,
}

impl std::fmt::Display for AttachmentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AttachmentMetadata {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(is_inline) = &self.is_inline {
                format!("{:?}", is_inline)
            } else {
                String::new()
            },
            if let Some(cid) = &self.cid {
                format!("{:?}", cid)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["is_inline".to_string(), "cid".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Attachment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the attached file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[doc = "URL to download the attached file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "Content type of the attached file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[doc = "Size (in byte) of the attached file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "Attachment metadata"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<AttachmentMetadata>,
}

impl std::fmt::Display for Attachment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Attachment {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(filename) = &self.filename {
                format!("{:?}", filename)
            } else {
                String::new()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url)
            } else {
                String::new()
            },
            if let Some(content_type) = &self.content_type {
                format!("{:?}", content_type)
            } else {
                String::new()
            },
            if let Some(size) = &self.size {
                format!("{:?}", size)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "filename".to_string(),
            "url".to_string(),
            "content_type".to_string(),
            "size".to_string(),
            "metadata".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageTemplateFolderResponseUnderscoreLinksRelated {
    #[doc = "Link to resource's owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for MessageTemplateFolderResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageTemplateFolderResponseUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(owner) = &self.owner {
            format!("{:?}", owner)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageTemplateFolderResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<MessageTemplateFolderResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for MessageTemplateFolderResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageTemplateFolderResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageTemplateFolderResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<MessageTemplateFolderResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the message template folder"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the message template folder"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for MessageTemplateFolderResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageTemplateFolderResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageTemplateResponseUnderscoreLinksRelated {
    #[doc = "Link to resource's owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for MessageTemplateResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageTemplateResponseUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(owner) = &self.owner {
            format!("{:?}", owner)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageTemplateResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<MessageTemplateResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for MessageTemplateResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageTemplateResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageTemplateResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<MessageTemplateResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Subject of the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Body of the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "List of files attached to the response"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[doc = "Whether or not the template is available in all inboxes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_available_for_all_inboxes: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox_ids: Option<Vec<String>>,
}

impl std::fmt::Display for MessageTemplateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageTemplateResponse {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            if let Some(body) = &self.body {
                format!("{:?}", body)
            } else {
                String::new()
            },
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
            if let Some(is_available_for_all_inboxes) = &self.is_available_for_all_inboxes {
                format!("{:?}", is_available_for_all_inboxes)
            } else {
                String::new()
            },
            if let Some(inbox_ids) = &self.inbox_ids {
                format!("{:?}", inbox_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "subject".to_string(),
            "body".to_string(),
            "attachments".to_string(),
            "is_available_for_all_inboxes".to_string(),
            "inbox_ids".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactGroupResponsesUnderscoreLinksRelated {
    #[doc = "Link to group contacts"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contacts: Option<String>,
    #[doc = "Link to group owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for ContactGroupResponsesUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactGroupResponsesUnderscoreLinksRelated {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(contacts) = &self.contacts {
                format!("{:?}", contacts)
            } else {
                String::new()
            },
            if let Some(owner) = &self.owner {
                format!("{:?}", owner)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["contacts".to_string(), "owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactGroupResponsesUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ContactGroupResponsesUnderscoreLinksRelated>,
}

impl std::fmt::Display for ContactGroupResponsesUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactGroupResponsesUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactGroupResponses {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ContactGroupResponsesUnderscoreLinks>,
    #[doc = "Unique identifier of the group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether or not the contact is individual"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

impl std::fmt::Display for ContactGroupResponses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactGroupResponses {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(is_private) = &self.is_private {
                format!("{:?}", is_private)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "is_private".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactNoteResponses {
    #[doc = "A teammate is a user in Front."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<TeammateResponse>,
    #[doc = "Content of the note"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Date at which the note have been created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
}

impl std::fmt::Display for ContactNoteResponses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactNoteResponses {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(author) = &self.author {
                format!("{:?}", author)
            } else {
                String::new()
            },
            if let Some(body) = &self.body {
                format!("{:?}", body)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "author".to_string(),
            "body".to_string(),
            "created_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ChannelResponseUnderscoreLinksRelated {
    #[doc = "Link to channel inbox"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inbox: Option<String>,
    #[doc = "Link to channel owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for ChannelResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ChannelResponseUnderscoreLinksRelated {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(inbox) = &self.inbox {
                format!("{:?}", inbox)
            } else {
                String::new()
            },
            if let Some(owner) = &self.owner {
                format!("{:?}", owner)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["inbox".to_string(), "owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ChannelResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ChannelResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for ChannelResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ChannelResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[doc = "Type of the channel"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Types {
    #[serde(rename = "smtp")]
    #[display("smtp")]
    Smtp,
    #[serde(rename = "imap")]
    #[display("imap")]
    Imap,
    #[serde(rename = "twilio")]
    #[display("twilio")]
    Twilio,
    #[serde(rename = "twitter")]
    #[display("twitter")]
    Twitter,
    #[serde(rename = "facebook")]
    #[display("facebook")]
    Facebook,
    #[serde(rename = "smooch")]
    #[display("smooch")]
    Smooch,
    #[serde(rename = "intercom")]
    #[display("intercom")]
    Intercom,
    #[serde(rename = "truly")]
    #[display("truly")]
    Truly,
    #[serde(rename = "custom")]
    #[display("custom")]
    Custom,
}

#[doc = "Channel settings"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ChannelResponseSettings {
    #[doc = "The time (measured in seconds) that users have to undo a send operation in the \
             channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undo_send_time: Option<i64>,
    #[doc = "Whether teammates without inbox access can reply on this channel. Only present for \
             shared channels; omitted for private channels."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_teammates_can_reply: Option<bool>,
}

impl std::fmt::Display for ChannelResponseSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ChannelResponseSettings {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(undo_send_time) = &self.undo_send_time {
                format!("{:?}", undo_send_time)
            } else {
                String::new()
            },
            if let Some(all_teammates_can_reply) = &self.all_teammates_can_reply {
                format!("{:?}", all_teammates_can_reply)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "undo_send_time".to_string(),
            "all_teammates_can_reply".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ChannelResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ChannelResponseUnderscoreLinks>,
    #[doc = "Unique identifier for the channel"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Address receiving the messages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "Type of the channel"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Types>,
    #[doc = "Address which appears as the sender for messages sent from Front"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_as: Option<String>,
    #[doc = "Channel settings"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<ChannelResponseSettings>,
    #[doc = "Whether or not the channel is individual"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[doc = "Whether or not the channel configuration is valid"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_valid: Option<bool>,
}

impl std::fmt::Display for ChannelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ChannelResponse {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(address) = &self.address {
                format!("{:?}", address)
            } else {
                String::new()
            },
            if let Some(types) = &self.types {
                format!("{:?}", types)
            } else {
                String::new()
            },
            if let Some(send_as) = &self.send_as {
                format!("{:?}", send_as)
            } else {
                String::new()
            },
            if let Some(settings) = &self.settings {
                format!("{:?}", settings)
            } else {
                String::new()
            },
            if let Some(is_private) = &self.is_private {
                format!("{:?}", is_private)
            } else {
                String::new()
            },
            if let Some(is_valid) = &self.is_valid {
                format!("{:?}", is_valid)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "address".to_string(),
            "types".to_string(),
            "send_as".to_string(),
            "settings".to_string(),
            "is_private".to_string(),
            "is_valid".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CommentResponseUnderscoreLinksRelated {
    #[doc = "Link to comment's conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversations: Option<String>,
    #[doc = "Link to comment mentions"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentions: Option<String>,
}

impl std::fmt::Display for CommentResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CommentResponseUnderscoreLinksRelated {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(conversations) = &self.conversations {
                format!("{:?}", conversations)
            } else {
                String::new()
            },
            if let Some(mentions) = &self.mentions {
                format!("{:?}", mentions)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["conversations".to_string(), "mentions".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CommentResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<CommentResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for CommentResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CommentResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CommentResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<CommentResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A teammate is a user in Front."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<TeammateResponse>,
    #[doc = "Content of the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Date at which the comment was posted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<i64>,
    #[doc = "List of files attached to the comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
}

impl std::fmt::Display for CommentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CommentResponse {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(author) = &self.author {
                format!("{:?}", author)
            } else {
                String::new()
            },
            if let Some(body) = &self.body {
                format!("{:?}", body)
            } else {
                String::new()
            },
            if let Some(posted_at) = &self.posted_at {
                format!("{:?}", posted_at)
            } else {
                String::new()
            },
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "author".to_string(),
            "body".to_string(),
            "posted_at".to_string(),
            "attachments".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactResponseUnderscoreLinksRelated {
    #[doc = "Link to contact notes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "Link to contact conversations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversations: Option<String>,
    #[doc = "Link to contact owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for ContactResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactResponseUnderscoreLinksRelated {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(notes) = &self.notes {
                format!("{:?}", notes)
            } else {
                String::new()
            },
            if let Some(conversations) = &self.conversations {
                format!("{:?}", conversations)
            } else {
                String::new()
            },
            if let Some(owner) = &self.owner {
                format!("{:?}", owner)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "notes".to_string(),
            "conversations".to_string(),
            "owner".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ContactResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for ContactResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContactResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ContactResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Contact name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Contact description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "URL of the contact's avatar"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[doc = "Whether or not the contact is marked as a spammer"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_spammer: Option<bool>,
    #[doc = "List of all the links of the contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<String>>,
    #[doc = "List of the groups the contact belongs to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ContactGroupResponses>>,
    #[doc = "List of the handles and sources with which the contact is reachable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handles: Option<Vec<ContactHandle>>,
    #[doc = "Custom field attributes for this contact."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<std::collections::HashMap<String, String>>,
    #[doc = "Whether or not the contact is individual"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

impl std::fmt::Display for ContactResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ContactResponse {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(avatar_url) = &self.avatar_url {
                format!("{:?}", avatar_url)
            } else {
                String::new()
            },
            if let Some(is_spammer) = &self.is_spammer {
                format!("{:?}", is_spammer)
            } else {
                String::new()
            },
            if let Some(links) = &self.links {
                format!("{:?}", links)
            } else {
                String::new()
            },
            if let Some(groups) = &self.groups {
                format!("{:?}", groups)
            } else {
                String::new()
            },
            if let Some(handles) = &self.handles {
                format!("{:?}", handles)
            } else {
                String::new()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields)
            } else {
                String::new()
            },
            if let Some(is_private) = &self.is_private {
                format!("{:?}", is_private)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "description".to_string(),
            "avatar_url".to_string(),
            "is_spammer".to_string(),
            "links".to_string(),
            "groups".to_string(),
            "handles".to_string(),
            "custom_fields".to_string(),
            "is_private".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ConversationResponseUnderscoreLinksRelated {
    #[doc = "Link to conversation events"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<String>,
    #[doc = "Link to conversation followers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub followers: Option<String>,
    #[doc = "Link to conversation messages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub messages: Option<String>,
    #[doc = "Link to conversation comment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "Link to conversation inboxes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inboxes: Option<String>,
    #[doc = "Link to last message of the conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_message: Option<String>,
}

impl std::fmt::Display for ConversationResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ConversationResponseUnderscoreLinksRelated {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(events) = &self.events {
                format!("{:?}", events)
            } else {
                String::new()
            },
            if let Some(followers) = &self.followers {
                format!("{:?}", followers)
            } else {
                String::new()
            },
            if let Some(messages) = &self.messages {
                format!("{:?}", messages)
            } else {
                String::new()
            },
            if let Some(comments) = &self.comments {
                format!("{:?}", comments)
            } else {
                String::new()
            },
            if let Some(inboxes) = &self.inboxes {
                format!("{:?}", inboxes)
            } else {
                String::new()
            },
            if let Some(last_message) = &self.last_message {
                format!("{:?}", last_message)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "events".to_string(),
            "followers".to_string(),
            "messages".to_string(),
            "comments".to_string(),
            "inboxes".to_string(),
            "last_message".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ConversationResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ConversationResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for ConversationResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ConversationResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[doc = "Status of the conversation"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum ConversationResponseStatus {
    #[serde(rename = "archived")]
    #[display("archived")]
    Archived,
    #[serde(rename = "unassigned")]
    #[display("unassigned")]
    Unassigned,
    #[serde(rename = "deleted")]
    #[display("deleted")]
    Deleted,
    #[serde(rename = "assigned")]
    #[display("assigned")]
    Assigned,
}

#[doc = "Optional metadata about the conversation"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ConversationResponseMetadata {
    #[doc = "List of external_ids for partner channel associated with the conversation. Only \
             present for partner channel token authenticated requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_conversation_ids: Option<Vec<String>>,
}

impl std::fmt::Display for ConversationResponseMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ConversationResponseMetadata {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(external_conversation_ids) = &self.external_conversation_ids {
                format!("{:?}", external_conversation_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["external_conversation_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ConversationResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ConversationResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Subject of the message for email message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Status of the conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<ConversationResponseStatus>,
    #[doc = "A teammate is a user in Front."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<TeammateResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<RecipientResponse>,
    #[doc = "List of the tags for this conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagResponse>>,
    #[doc = "List of the links for this conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<Vec<LinkResponse>>,
    #[doc = "Timestamp at which the conversation have been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[doc = "Whether or not the conversation is private"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[doc = "List of scheduled (non-expired and non-canceled) reminders for this conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_reminders: Option<Vec<Reminder>>,
    #[doc = "Optional metadata about the conversation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ConversationResponseMetadata>,
}

impl std::fmt::Display for ConversationResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ConversationResponse {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(assignee) = &self.assignee {
                format!("{:?}", assignee)
            } else {
                String::new()
            },
            if let Some(recipient) = &self.recipient {
                format!("{:?}", recipient)
            } else {
                String::new()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags)
            } else {
                String::new()
            },
            if let Some(links) = &self.links {
                format!("{:?}", links)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(is_private) = &self.is_private {
                format!("{:?}", is_private)
            } else {
                String::new()
            },
            if let Some(scheduled_reminders) = &self.scheduled_reminders {
                format!("{:?}", scheduled_reminders)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "subject".to_string(),
            "status".to_string(),
            "assignee".to_string(),
            "recipient".to_string(),
            "tags".to_string(),
            "links".to_string(),
            "created_at".to_string(),
            "is_private".to_string(),
            "scheduled_reminders".to_string(),
            "metadata".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomFieldResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for CustomFieldResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CustomFieldResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[doc = "Type of the custom field"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum CustomFieldResponseType {
    #[serde(rename = "string")]
    #[display("string")]
    String,
    #[serde(rename = "boolean")]
    #[display("boolean")]
    Boolean,
    #[serde(rename = "datetime")]
    #[display("datetime")]
    Datetime,
    #[serde(rename = "number")]
    #[display("number")]
    Number,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomFieldResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<CustomFieldResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Description of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Type of the custom field"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<CustomFieldResponseType>,
}

impl std::fmt::Display for CustomFieldResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CustomFieldResponse {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "description".to_string(),
            "type_".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InboxResponseUnderscoreLinksRelated {
    #[doc = "Link to inbox teammates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teammates: Option<String>,
    #[doc = "Link to inbox conversations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversations: Option<String>,
    #[doc = "Link to inbox channels"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channels: Option<String>,
    #[doc = "Link to inbox owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for InboxResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for InboxResponseUnderscoreLinksRelated {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(teammates) = &self.teammates {
                format!("{:?}", teammates)
            } else {
                String::new()
            },
            if let Some(conversations) = &self.conversations {
                format!("{:?}", conversations)
            } else {
                String::new()
            },
            if let Some(channels) = &self.channels {
                format!("{:?}", channels)
            } else {
                String::new()
            },
            if let Some(owner) = &self.owner {
                format!("{:?}", owner)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "teammates".to_string(),
            "conversations".to_string(),
            "channels".to_string(),
            "owner".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InboxResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<InboxResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for InboxResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for InboxResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InboxResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<InboxResponseUnderscoreLinks>,
    #[doc = "Unique identifier for the inbox"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the inbox"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Whether or not the inbox is individual"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

impl std::fmt::Display for InboxResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for InboxResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(is_private) = &self.is_private {
                format!("{:?}", is_private)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "is_private".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageResponseUnderscoreLinksRelated {
    #[doc = "Link to message converation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,
    #[doc = "Link to message this message replied to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_replied_to: Option<String>,
    #[doc = "Link to message seen informatiion"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_seen: Option<String>,
}

impl std::fmt::Display for MessageResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageResponseUnderscoreLinksRelated {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(conversation) = &self.conversation {
                format!("{:?}", conversation)
            } else {
                String::new()
            },
            if let Some(message_replied_to) = &self.message_replied_to {
                format!("{:?}", message_replied_to)
            } else {
                String::new()
            },
            if let Some(message_seen) = &self.message_seen {
                format!("{:?}", message_seen)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "conversation".to_string(),
            "message_replied_to".to_string(),
            "message_seen".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<MessageResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for MessageResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[doc = "Type of the message"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum MessageResponseType {
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "tweet")]
    #[display("tweet")]
    Tweet,
    #[serde(rename = "sms")]
    #[display("sms")]
    Sms,
    #[serde(rename = "smooch")]
    #[display("smooch")]
    Smooch,
    #[serde(rename = "facebook")]
    #[display("facebook")]
    Facebook,
    #[serde(rename = "intercom")]
    #[display("intercom")]
    Intercom,
    #[serde(rename = "truly-call")]
    #[display("truly-call")]
    TrulyCall,
    #[serde(rename = "custom")]
    #[display("custom")]
    Custom,
}

#[doc = "If the message is a draft, describes the draft mode. Can be 'private' (draft is visible \
         to the author only) or 'shared' (draft is visible to all teammates with access to the \
         conversation)."]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum DraftMode {
    #[serde(rename = "shared")]
    #[display("shared")]
    Shared,
    #[serde(rename = "private")]
    #[display("private")]
    Private,
}

#[doc = "Optional metadata about the message"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageResponseMetadata {
    #[doc = "For `intercom` messages only. URL of the Intercom conversation the message is \
             comming from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intercom_url: Option<String>,
    #[doc = "For `truly-call` messages only. Length of the call in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[doc = "For `truly-call` messages only. Whether or not the call have been answered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub have_been_answered: Option<bool>,
    #[doc = "For `tweet` or 'custom' (partner channel token authenticated) messages only. Unique \
             message identifier in the underlying provider (Twitter or Partner). For custom \
             messages, only present for partner channel token authenticated requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "For `tweet` messages only. URL of the tweet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_url: Option<String>,
    #[doc = "For `tweet` messages only. Whether or not the tweet is a retweet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_retweet: Option<bool>,
    #[doc = "For `tweet` messages only. Whether or not the tweet have been retweeted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub have_been_retweeted: Option<bool>,
    #[doc = "For `tweet` messages only. Whether or not the tweet have been favorited."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub have_been_favorited: Option<bool>,
    #[doc = "For `custom` messages only. Custom reference which is used to thread messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_ref: Option<String>,
    #[doc = "For `custom` messages only. Custom object holding internal information."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
}

impl std::fmt::Display for MessageResponseMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageResponseMetadata {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(intercom_url) = &self.intercom_url {
                format!("{:?}", intercom_url)
            } else {
                String::new()
            },
            if let Some(duration) = &self.duration {
                format!("{:?}", duration)
            } else {
                String::new()
            },
            if let Some(have_been_answered) = &self.have_been_answered {
                format!("{:?}", have_been_answered)
            } else {
                String::new()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id)
            } else {
                String::new()
            },
            if let Some(twitter_url) = &self.twitter_url {
                format!("{:?}", twitter_url)
            } else {
                String::new()
            },
            if let Some(is_retweet) = &self.is_retweet {
                format!("{:?}", is_retweet)
            } else {
                String::new()
            },
            if let Some(have_been_retweeted) = &self.have_been_retweeted {
                format!("{:?}", have_been_retweeted)
            } else {
                String::new()
            },
            if let Some(have_been_favorited) = &self.have_been_favorited {
                format!("{:?}", have_been_favorited)
            } else {
                String::new()
            },
            if let Some(thread_ref) = &self.thread_ref {
                format!("{:?}", thread_ref)
            } else {
                String::new()
            },
            if let Some(headers) = &self.headers {
                format!("{:?}", headers)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "intercom_url".to_string(),
            "duration".to_string(),
            "have_been_answered".to_string(),
            "external_id".to_string(),
            "twitter_url".to_string(),
            "is_retweet".to_string(),
            "have_been_retweeted".to_string(),
            "have_been_favorited".to_string(),
            "thread_ref".to_string(),
            "headers".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MessageResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<MessageResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Type of the message"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<MessageResponseType>,
    #[doc = "Whether or not the message has been received or sent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inbound: Option<bool>,
    #[doc = "If the message is a draft, describes the draft mode. Can be 'private' (draft is \
             visible to the author only) or 'shared' (draft is visible to all teammates with \
             access to the conversation)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_mode: Option<DraftMode>,
    #[doc = "Type of the error when the draft failed to be sent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[doc = "The current version of the message in Front"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Date at which the message as been sent or received"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[doc = "Subject of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Preview of the message body"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blurb: Option<String>,
    #[doc = "A teammate is a user in Front."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<TeammateResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipients: Option<Vec<RecipientResponse>>,
    #[doc = "Body of the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Text version of the body for email messages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[doc = "List of files attached to the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature: Option<SignatureResponse>,
    #[doc = "Optional metadata about the message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<MessageResponseMetadata>,
}

impl std::fmt::Display for MessageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MessageResponse {
    const LENGTH: usize = 17;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
            if let Some(is_inbound) = &self.is_inbound {
                format!("{:?}", is_inbound)
            } else {
                String::new()
            },
            if let Some(draft_mode) = &self.draft_mode {
                format!("{:?}", draft_mode)
            } else {
                String::new()
            },
            if let Some(error_type) = &self.error_type {
                format!("{:?}", error_type)
            } else {
                String::new()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(subject) = &self.subject {
                format!("{:?}", subject)
            } else {
                String::new()
            },
            if let Some(blurb) = &self.blurb {
                format!("{:?}", blurb)
            } else {
                String::new()
            },
            if let Some(author) = &self.author {
                format!("{:?}", author)
            } else {
                String::new()
            },
            if let Some(recipients) = &self.recipients {
                format!("{:?}", recipients)
            } else {
                String::new()
            },
            if let Some(body) = &self.body {
                format!("{:?}", body)
            } else {
                String::new()
            },
            if let Some(text) = &self.text {
                format!("{:?}", text)
            } else {
                String::new()
            },
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments)
            } else {
                String::new()
            },
            if let Some(signature) = &self.signature {
                format!("{:?}", signature)
            } else {
                String::new()
            },
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "type_".to_string(),
            "is_inbound".to_string(),
            "draft_mode".to_string(),
            "error_type".to_string(),
            "version".to_string(),
            "created_at".to_string(),
            "subject".to_string(),
            "blurb".to_string(),
            "author".to_string(),
            "recipients".to_string(),
            "body".to_string(),
            "text".to_string(),
            "attachments".to_string(),
            "signature".to_string(),
            "metadata".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RecipientResponseUnderscoreLinksRelated {
    #[doc = "Link to recipient contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

impl std::fmt::Display for RecipientResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RecipientResponseUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(contact) = &self.contact {
            format!("{:?}", contact)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["contact".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RecipientResponseUnderscoreLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<RecipientResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for RecipientResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RecipientResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(related) = &self.related {
            format!("{:?}", related)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["related".to_string()]
    }
}

#[doc = "Role of the recipient"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Role {
    #[serde(rename = "from")]
    #[display("from")]
    From,
    #[serde(rename = "to")]
    #[display("to")]
    To,
    #[serde(rename = "cc")]
    #[display("cc")]
    Cc,
    #[serde(rename = "bcc")]
    #[display("bcc")]
    Bcc,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RecipientResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<RecipientResponseUnderscoreLinks>,
    #[doc = "Name of the recipient."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Handle of the contact. Can be any string used to uniquely identify the contact"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
    #[doc = "Role of the recipient"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

impl std::fmt::Display for RecipientResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RecipientResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(handle) = &self.handle {
                format!("{:?}", handle)
            } else {
                String::new()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "name".to_string(),
            "handle".to_string(),
            "role".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ReminderUnderscoreLinksRelated {
    #[doc = "Link to conversation owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for ReminderUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ReminderUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(owner) = &self.owner {
            format!("{:?}", owner)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ReminderUnderscoreLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ReminderUnderscoreLinksRelated>,
}

impl std::fmt::Display for ReminderUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ReminderUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(related) = &self.related {
            format!("{:?}", related)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Reminder {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ReminderUnderscoreLinks>,
    #[doc = "Timestamp at which the conversation reminder has been created"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[doc = "Timestamp that the conversation reminder has been scheduled for"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<i64>,
    #[doc = "Timestamp at which the conversation reminder has been updated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl std::fmt::Display for Reminder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Reminder {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(scheduled_at) = &self.scheduled_at {
                format!("{:?}", scheduled_at)
            } else {
                String::new()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "created_at".to_string(),
            "scheduled_at".to_string(),
            "updated_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RoleResponseUnderscoreLinksRelated {
    #[doc = "Link to role owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for RoleResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RoleResponseUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(owner) = &self.owner {
            format!("{:?}", owner)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RoleResponseUnderscoreLinks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<RoleResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for RoleResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RoleResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(related) = &self.related {
            format!("{:?}", related)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RoleResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<RoleResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the role"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for RoleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RoleResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RuleResponseUnderscoreLinksRelated {
    #[doc = "Link to rule owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for RuleResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RuleResponseUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(owner) = &self.owner {
            format!("{:?}", owner)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RuleResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<RuleResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for RuleResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RuleResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RuleResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<RuleResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the rule"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of the rule's actions description"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,
    #[doc = "Whether or not the rule is individual"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

impl std::fmt::Display for RuleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RuleResponse {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(actions) = &self.actions {
                format!("{:?}", actions)
            } else {
                String::new()
            },
            if let Some(is_private) = &self.is_private {
                format!("{:?}", is_private)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "actions".to_string(),
            "is_private".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SeenReceiptResponseUnderscoreLinksRelated {
    #[doc = "Link to message associated with the seen record"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl std::fmt::Display for SeenReceiptResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SeenReceiptResponseUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(message) = &self.message {
            format!("{:?}", message)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["message".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SeenReceiptResponseUnderscoreLinks {
    #[doc = "Link to self"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<SeenReceiptResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for SeenReceiptResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SeenReceiptResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SeenReceiptResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<SeenReceiptResponseUnderscoreLinks>,
    #[doc = "Timestamp when message was seen"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_seen_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seen_by: Option<ContactHandle>,
}

impl std::fmt::Display for SeenReceiptResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SeenReceiptResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(first_seen_at) = &self.first_seen_at {
                format!("{:?}", first_seen_at)
            } else {
                String::new()
            },
            if let Some(seen_by) = &self.seen_by {
                format!("{:?}", seen_by)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "first_seen_at".to_string(),
            "seen_by".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftResponseUnderscoreLinksRelated {
    #[doc = "Link to shift teammates"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teammates: Option<String>,
    #[doc = "Link to shift owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for ShiftResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ShiftResponseUnderscoreLinksRelated {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(teammates) = &self.teammates {
                format!("{:?}", teammates)
            } else {
                String::new()
            },
            if let Some(owner) = &self.owner {
                format!("{:?}", owner)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["teammates".to_string(), "owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<ShiftResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for ShiftResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ShiftResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[doc = "Color of the shift"]
#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum ShiftResponseColor {
    #[serde(rename = "black")]
    #[display("black")]
    Black,
}

impl std::default::Default for ShiftResponseColor {
    fn default() -> Self {
        ShiftResponseColor::Black
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ShiftResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the shift"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the shift"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Color of the shift"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<ShiftResponseColor>,
    #[doc = "A timezone name as defined in the IANA tz database"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub times: Option<ShiftIntervals>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl std::fmt::Display for ShiftResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ShiftResponse {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(color) = &self.color {
                format!("{:?}", color)
            } else {
                String::new()
            },
            if let Some(timezone) = &self.timezone {
                format!("{:?}", timezone)
            } else {
                String::new()
            },
            if let Some(times) = &self.times {
                format!("{:?}", times)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "color".to_string(),
            "timezone".to_string(),
            "times".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SignatureResponseUnderscoreLinksRelated {
    #[doc = "Link to signature's owner (either a team or teammate)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

impl std::fmt::Display for SignatureResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SignatureResponseUnderscoreLinksRelated {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(owner) = &self.owner {
            format!("{:?}", owner)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["owner".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SignatureResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<SignatureResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for SignatureResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SignatureResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SignatureResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<SignatureResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the signature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the signature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Body of the signature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[doc = "Sender info of the signature"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_info: Option<String>,
    #[doc = "Whether or not the signature is available in teammate channels."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible_for_all_teammate_channels: Option<bool>,
    #[doc = "Whether the signature is the default signature for the team or teammate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
}

impl std::fmt::Display for SignatureResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SignatureResponse {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(body) = &self.body {
                format!("{:?}", body)
            } else {
                String::new()
            },
            if let Some(sender_info) = &self.sender_info {
                format!("{:?}", sender_info)
            } else {
                String::new()
            },
            if let Some(is_visible_for_all_teammate_channels) =
                &self.is_visible_for_all_teammate_channels
            {
                format!("{:?}", is_visible_for_all_teammate_channels)
            } else {
                String::new()
            },
            if let Some(is_default) = &self.is_default {
                format!("{:?}", is_default)
            } else {
                String::new()
            },
            if let Some(channel_ids) = &self.channel_ids {
                format!("{:?}", channel_ids)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "body".to_string(),
            "sender_info".to_string(),
            "is_visible_for_all_teammate_channels".to_string(),
            "is_default".to_string(),
            "channel_ids".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TagResponseUnderscoreLinksRelated {
    #[doc = "Link to tag conversations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversations: Option<String>,
    #[doc = "Link to tag owner"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Link to tag children"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<String>,
}

impl std::fmt::Display for TagResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TagResponseUnderscoreLinksRelated {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(conversations) = &self.conversations {
                format!("{:?}", conversations)
            } else {
                String::new()
            },
            if let Some(owner) = &self.owner {
                format!("{:?}", owner)
            } else {
                String::new()
            },
            if let Some(children) = &self.children {
                format!("{:?}", children)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "conversations".to_string(),
            "owner".to_string(),
            "children".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TagResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<TagResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for TagResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TagResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[doc = "A tag is a label that can be used to classify conversations."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TagResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<TagResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the tag"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Highlight color of the tag."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Highlight>,
    #[doc = "Whether or not the tag is individual"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    #[doc = "Whether the tag is visible in conversation lists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_visible_in_conversation_lists: Option<bool>,
    #[doc = "Timestamp of tag create creation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[doc = "Timestamp of the last tag update"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
}

impl std::fmt::Display for TagResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TagResponse {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(highlight) = &self.highlight {
                format!("{:?}", highlight)
            } else {
                String::new()
            },
            if let Some(is_private) = &self.is_private {
                format!("{:?}", is_private)
            } else {
                String::new()
            },
            if let Some(is_visible_in_conversation_lists) = &self.is_visible_in_conversation_lists {
                format!("{:?}", is_visible_in_conversation_lists)
            } else {
                String::new()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "highlight".to_string(),
            "is_private".to_string(),
            "is_visible_in_conversation_lists".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeamResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for TeamResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TeamResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeamResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<TeamResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "List of the inboxes in the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inboxes: Option<Vec<InboxResponse>>,
    #[doc = "List of the teammates that have access to the team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TeammateResponse>>,
}

impl std::fmt::Display for TeamResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TeamResponse {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(inboxes) = &self.inboxes {
                format!("{:?}", inboxes)
            } else {
                String::new()
            },
            if let Some(members) = &self.members {
                format!("{:?}", members)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "inboxes".to_string(),
            "members".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeammateResponseUnderscoreLinksRelated {
    #[doc = "Link to teammate's inboxes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inboxes: Option<String>,
    #[doc = "Link to teammate's conversations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversations: Option<String>,
}

impl std::fmt::Display for TeammateResponseUnderscoreLinksRelated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TeammateResponseUnderscoreLinksRelated {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(inboxes) = &self.inboxes {
                format!("{:?}", inboxes)
            } else {
                String::new()
            },
            if let Some(conversations) = &self.conversations {
                format!("{:?}", conversations)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["inboxes".to_string(), "conversations".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeammateResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related: Option<TeammateResponseUnderscoreLinksRelated>,
}

impl std::fmt::Display for TeammateResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TeammateResponseUnderscoreLinks {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(self_) = &self.self_ {
                format!("{:?}", self_)
            } else {
                String::new()
            },
            if let Some(related) = &self.related {
                format!("{:?}", related)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string(), "related".to_string()]
    }
}

#[doc = "A teammate is a user in Front."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeammateResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<TeammateResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the teammate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Email address of the teammate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Username of the teammate (used for \"@\" mentions)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "First name of the teammate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name of the teammate"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Whether or not the teammate is an admin in your company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[doc = "Whether or not the teammate is available"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    #[doc = "Whether or not the teammate account has been blocked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_blocked: Option<bool>,
}

impl std::fmt::Display for TeammateResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for TeammateResponse {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email)
            } else {
                String::new()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username)
            } else {
                String::new()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name)
            } else {
                String::new()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name)
            } else {
                String::new()
            },
            if let Some(is_admin) = &self.is_admin {
                format!("{:?}", is_admin)
            } else {
                String::new()
            },
            if let Some(is_available) = &self.is_available {
                format!("{:?}", is_available)
            } else {
                String::new()
            },
            if let Some(is_blocked) = &self.is_blocked {
                format!("{:?}", is_blocked)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "email".to_string(),
            "username".to_string(),
            "first_name".to_string(),
            "last_name".to_string(),
            "is_admin".to_string(),
            "is_available".to_string(),
            "is_blocked".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LinkResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for LinkResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for LinkResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[doc = "A link used to connect a Front conversation to an external resource."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LinkResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<LinkResponseUnderscoreLinks>,
    #[doc = "Unique identifier of the link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Display name of the link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Type of the link. Typically associated with the underlying link provider (if known)"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Underlying identifying external URL of the link"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
}

impl std::fmt::Display for LinkResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for LinkResponse {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_)
            } else {
                String::new()
            },
            if let Some(external_url) = &self.external_url {
                format!("{:?}", external_url)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "underscore_links".to_string(),
            "id".to_string(),
            "name".to_string(),
            "type_".to_string(),
            "external_url".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    #[display("asc")]
    Asc,
    #[serde(rename = "desc")]
    #[display("desc")]
    Desc,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Pagination {
    #[doc = "Link to next page of results"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Pagination {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(next) = &self.next {
            format!("{:?}", next)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["next".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCannedAnswersApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfCannedAnswersApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCannedAnswersApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCannedAnswersApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfCannedAnswersApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListOfCannedAnswersApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCannedAnswersApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCannedAnswerFoldersApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfCannedAnswerFoldersApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCannedAnswerFoldersApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCannedAnswerFoldersApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfCannedAnswerFoldersApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListOfCannedAnswerFoldersApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCannedAnswerFoldersApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfSignaturesApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfSignaturesApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfSignaturesApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfSignaturesApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfSignaturesApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SignatureResponse>>,
}

impl std::fmt::Display for ListOfSignaturesApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfSignaturesApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfInboxesApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfInboxesApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfInboxesApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfInboxesApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfInboxesApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<InboxResponse>>,
}

impl std::fmt::Display for ListOfInboxesApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfInboxesApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCommentsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfCommentsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCommentsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCommentsApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfCommentsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<CommentResponse>>,
}

impl std::fmt::Display for ListOfCommentsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCommentsApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfTeamsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfTeamsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfTeamsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfTeamsApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfTeamsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeamResponse>>,
}

impl std::fmt::Display for ListOfTeamsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfTeamsApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfTeammatesApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfTeammatesApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfTeammatesApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfTeammatesApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfTeammatesApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeammateResponse>>,
}

impl std::fmt::Display for ListOfTeammatesApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfTeammatesApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfShiftsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfShiftsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfShiftsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfShiftsApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfShiftsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ShiftResponse>>,
}

impl std::fmt::Display for ListOfShiftsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfShiftsApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfContactsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfContactsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfContactsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfContactsApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfContactsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactResponse>>,
}

impl std::fmt::Display for ListOfContactsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfContactsApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfAccountsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfAccountsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfAccountsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfAccountsApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfAccountsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AccountResponse>>,
}

impl std::fmt::Display for ListOfAccountsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfAccountsApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfContactGroupsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfContactGroupsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfContactGroupsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfContactGroupsApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfContactGroupsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactGroupResponses>>,
}

impl std::fmt::Display for ListOfContactGroupsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfContactGroupsApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfContactNotesApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfContactNotesApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfContactNotesApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfContactNotesApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfContactNotesApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactNoteResponses>>,
}

impl std::fmt::Display for ListOfContactNotesApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfContactNotesApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfMessagesApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfMessagesApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfMessagesApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfMessagesApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfMessagesApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageResponse>>,
}

impl std::fmt::Display for ListOfMessagesApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfMessagesApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfSeenReceiptsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfSeenReceiptsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfSeenReceiptsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfSeenReceiptsApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfSeenReceiptsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SeenReceiptResponse>>,
}

impl std::fmt::Display for ListOfSeenReceiptsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfSeenReceiptsApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfConversationsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfConversationsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfConversationsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfConversationsApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfConversationsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListOfConversationsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfConversationsApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfConversationSearchResultsApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[doc = "Total number of matching conversations"]
    #[serde(rename = "_total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListOfConversationSearchResultsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfConversationSearchResultsApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(total) = &self.total {
                format!("{:?}", total)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "total".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfEventsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfEventsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfEventsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfEventsApplicationJson {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfEventsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<EventResponse>>,
}

impl std::fmt::Display for ListOfEventsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfEventsApplicationJson {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfRolesApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfRolesApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfRolesApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfRolesApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfRolesApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RoleResponse>>,
}

impl std::fmt::Display for ListOfRolesApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfRolesApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfRulesApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfRulesApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfRulesApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfRulesApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfRulesApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RuleResponse>>,
}

impl std::fmt::Display for ListOfRulesApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfRulesApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfTagsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfTagsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfTagsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfTagsApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfTagsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TagResponse>>,
}

impl std::fmt::Display for ListOfTagsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfTagsApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfLinksApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfLinksApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfLinksApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfLinksApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfLinksApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<LinkResponse>>,
}

impl std::fmt::Display for ListOfLinksApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfLinksApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfChannelsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfChannelsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfChannelsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfChannelsApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfChannelsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChannelResponse>>,
}

impl std::fmt::Display for ListOfChannelsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfChannelsApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCustomFieldsApplicationJsonUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListOfCustomFieldsApplicationJsonUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCustomFieldsApplicationJsonUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListOfCustomFieldsApplicationJson {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListOfCustomFieldsApplicationJsonUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<CustomFieldResponse>>,
}

impl std::fmt::Display for ListOfCustomFieldsApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListOfCustomFieldsApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AcceptedMessageApplicationJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Message unique identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_uid: Option<String>,
}

impl std::fmt::Display for AcceptedMessageApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AcceptedMessageApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(message_uid) = &self.message_uid {
                format!("{:?}", message_uid)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["status".to_string(), "message_uid".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AcceptedCannedAnswerFolderDeletionApplicationJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "id of the message template to be deleted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template_folder_id: Option<String>,
}

impl std::fmt::Display for AcceptedCannedAnswerFolderDeletionApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AcceptedCannedAnswerFolderDeletionApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(message_template_folder_id) = &self.message_template_folder_id {
                format!("{:?}", message_template_folder_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "status".to_string(),
            "message_template_folder_id".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AcceptedApplicationJson {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl std::fmt::Display for AcceptedApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AcceptedApplicationJson {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(status) = &self.status {
            format!("{:?}", status)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["status".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListAccountsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListAccountsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListAccountsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListAccountsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListAccountsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<AccountResponse>>,
}

impl std::fmt::Display for ListAccountsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListAccountsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListAccountContactsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListAccountContactsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListAccountContactsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListAccountContactsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListAccountContactsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactResponse>>,
}

impl std::fmt::Display for ListAccountContactsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListAccountContactsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListEventsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListEventsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListEventsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListEventsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListEventsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<EventResponse>>,
}

impl std::fmt::Display for ListEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListEventsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListFoldersResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListFoldersResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListFoldersResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListFoldersResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListFoldersResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListFoldersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListFoldersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamFoldersResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamFoldersResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamFoldersResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamFoldersResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamFoldersResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListTeamFoldersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamFoldersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateFoldersResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateFoldersResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateFoldersResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateFoldersResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateFoldersResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListTeammateFoldersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateFoldersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetChildFoldersResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for GetChildFoldersResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetChildFoldersResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetChildFoldersResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<GetChildFoldersResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for GetChildFoldersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetChildFoldersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetChildTemplatesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for GetChildTemplatesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetChildTemplatesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetChildTemplatesResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<GetChildTemplatesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for GetChildTemplatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetChildTemplatesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeleteFolderResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "id of the message template to be deleted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template_folder_id: Option<String>,
}

impl std::fmt::Display for DeleteFolderResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for DeleteFolderResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(message_template_folder_id) = &self.message_template_folder_id {
                format!("{:?}", message_template_folder_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "status".to_string(),
            "message_template_folder_id".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListMessageTemplatesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListMessageTemplatesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListMessageTemplatesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListMessageTemplatesResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListMessageTemplatesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListMessageTemplatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListMessageTemplatesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamMessageTemplatesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamMessageTemplatesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamMessageTemplatesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamMessageTemplatesResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamMessageTemplatesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListTeamMessageTemplatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamMessageTemplatesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateMessageTemplatesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateMessageTemplatesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateMessageTemplatesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateMessageTemplatesResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateMessageTemplatesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageTemplateResponse>>,
}

impl std::fmt::Display for ListTeammateMessageTemplatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateMessageTemplatesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListGroupsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListGroupsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListGroupsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListGroupsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListGroupsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactGroupResponses>>,
}

impl std::fmt::Display for ListGroupsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListGroupsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamGroupsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamGroupsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamGroupsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamGroupsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamGroupsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactGroupResponses>>,
}

impl std::fmt::Display for ListTeamGroupsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamGroupsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateGroupsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateGroupsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateGroupsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateGroupsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateGroupsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactGroupResponses>>,
}

impl std::fmt::Display for ListTeammateGroupsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateGroupsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListGroupContactsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListGroupContactsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListGroupContactsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListGroupContactsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListGroupContactsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactResponse>>,
}

impl std::fmt::Display for ListGroupContactsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListGroupContactsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListContactsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListContactsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListContactsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListContactsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListContactsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactResponse>>,
}

impl std::fmt::Display for ListContactsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListContactsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamContactsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamContactsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamContactsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamContactsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamContactsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactResponse>>,
}

impl std::fmt::Display for ListTeamContactsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamContactsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateContactsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateContactsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateContactsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateContactsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateContactsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactResponse>>,
}

impl std::fmt::Display for ListTeammateContactsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateContactsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListContactConversationsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListContactConversationsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListContactConversationsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListContactConversationsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListContactConversationsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListContactConversationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListContactConversationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListNotesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListNotesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListNotesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListNotesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListNotesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ContactNoteResponses>>,
}

impl std::fmt::Display for ListNotesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListNotesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListChannelsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListChannelsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListChannelsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListChannelsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListChannelsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChannelResponse>>,
}

impl std::fmt::Display for ListChannelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListChannelsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamChannelsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamChannelsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamChannelsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamChannelsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamChannelsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChannelResponse>>,
}

impl std::fmt::Display for ListTeamChannelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamChannelsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateChannelsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateChannelsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateChannelsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateChannelsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateChannelsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChannelResponse>>,
}

impl std::fmt::Display for ListTeammateChannelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateChannelsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ValidateChannelResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl std::fmt::Display for ValidateChannelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ValidateChannelResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(status) = &self.status {
            format!("{:?}", status)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["status".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxChannelsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListInboxChannelsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxChannelsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxChannelsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListInboxChannelsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ChannelResponse>>,
}

impl std::fmt::Display for ListInboxChannelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxChannelsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationCommentsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListConversationCommentsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationCommentsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationCommentsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListConversationCommentsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<CommentResponse>>,
}

impl std::fmt::Display for ListConversationCommentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationCommentsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCommentMentionsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListCommentMentionsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListCommentMentionsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCommentMentionsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListCommentMentionsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeammateResponse>>,
}

impl std::fmt::Display for ListCommentMentionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListCommentMentionsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListConversationsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListConversationsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListConversationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddConversationLinkRequestBody {
    #[doc = "Link IDs to add. Either link_ids or link_external_urls must be specified but not both"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_ids: Option<Vec<String>>,
    #[doc = "Link external URLs to add. Creates links if necessary. Either link_ids or \
             link_external_urls must be specified but not both"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_external_urls: Option<Vec<String>>,
}

impl std::fmt::Display for AddConversationLinkRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AddConversationLinkRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(link_ids) = &self.link_ids {
                format!("{:?}", link_ids)
            } else {
                String::new()
            },
            if let Some(link_external_urls) = &self.link_external_urls {
                format!("{:?}", link_external_urls)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["link_ids".to_string(), "link_external_urls".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RemoveConversationLinkRequestBody {
    #[doc = "Link IDs to remove."]
    pub link_ids: Vec<String>,
}

impl std::fmt::Display for RemoveConversationLinkRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for RemoveConversationLinkRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.link_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["link_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationInboxesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListConversationInboxesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationInboxesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationInboxesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListConversationInboxesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<InboxResponse>>,
}

impl std::fmt::Display for ListConversationInboxesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationInboxesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationFollowersResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListConversationFollowersResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationFollowersResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationFollowersResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListConversationFollowersResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeammateResponse>>,
}

impl std::fmt::Display for ListConversationFollowersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationFollowersResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddConversationFollowersRequestBody {
    #[doc = "IDs of the teammate to add to the followers list."]
    pub teammate_ids: Vec<String>,
}

impl std::fmt::Display for AddConversationFollowersRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AddConversationFollowersRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.teammate_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["teammate_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeleteConversationFollowersRequestBody {
    #[doc = "IDs of the teammate to remove from the followers list."]
    pub teammate_ids: Vec<String>,
}

impl std::fmt::Display for DeleteConversationFollowersRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for DeleteConversationFollowersRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.teammate_ids)]
    }

    fn headers() -> Vec<String> {
        vec!["teammate_ids".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationMessagesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListConversationMessagesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationMessagesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationMessagesResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListConversationMessagesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageResponse>>,
}

impl std::fmt::Display for ListConversationMessagesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationMessagesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationEventsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListConversationEventsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationEventsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationEventsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListConversationEventsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<EventResponse>>,
}

impl std::fmt::Display for ListConversationEventsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationEventsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListContactCustomFieldsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListContactCustomFieldsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListContactCustomFieldsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListContactCustomFieldsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListContactCustomFieldsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<CustomFieldResponse>>,
}

impl std::fmt::Display for ListContactCustomFieldsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListContactCustomFieldsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCustomFieldsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListCustomFieldsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListCustomFieldsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCustomFieldsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListCustomFieldsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<CustomFieldResponse>>,
}

impl std::fmt::Display for ListCustomFieldsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListCustomFieldsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationDraftsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListConversationDraftsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationDraftsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListConversationDraftsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListConversationDraftsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<MessageResponse>>,
}

impl std::fmt::Display for ListConversationDraftsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListConversationDraftsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListInboxesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListInboxesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<InboxResponse>>,
}

impl std::fmt::Display for ListInboxesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamInboxesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamInboxesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamInboxesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamInboxesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamInboxesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<InboxResponse>>,
}

impl std::fmt::Display for ListTeamInboxesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamInboxesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxConversationsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListInboxConversationsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxConversationsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxConversationsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListInboxConversationsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListInboxConversationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxConversationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxTeammatesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListInboxTeammatesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxTeammatesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListInboxTeammatesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListInboxTeammatesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeammateResponse>>,
}

impl std::fmt::Display for ListInboxTeammatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListInboxTeammatesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetMessageSeenStatusResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for GetMessageSeenStatusResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetMessageSeenStatusResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetMessageSeenStatusResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<GetMessageSeenStatusResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SeenReceiptResponse>>,
}

impl std::fmt::Display for GetMessageSeenStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetMessageSeenStatusResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MarkMessageSeenRequestBody {}

impl std::fmt::Display for MarkMessageSeenRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MarkMessageSeenRequestBody {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<String> {
        vec![]
    }

    fn headers() -> Vec<String> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ReceiveCustomMessageResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Message unique identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_uid: Option<String>,
}

impl std::fmt::Display for ReceiveCustomMessageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ReceiveCustomMessageResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(message_uid) = &self.message_uid {
                format!("{:?}", message_uid)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["status".to_string(), "message_uid".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ImportInboxMessageResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Message unique identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_uid: Option<String>,
}

impl std::fmt::Display for ImportInboxMessageResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ImportInboxMessageResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(message_uid) = &self.message_uid {
                format!("{:?}", message_uid)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["status".to_string(), "message_uid".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListRulesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListRulesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListRulesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListRulesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListRulesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RuleResponse>>,
}

impl std::fmt::Display for ListRulesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListRulesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamRulesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamRulesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamRulesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamRulesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamRulesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RuleResponse>>,
}

impl std::fmt::Display for ListTeamRulesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamRulesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateRulesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateRulesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateRulesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateRulesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateRulesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RuleResponse>>,
}

impl std::fmt::Display for ListTeammateRulesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateRulesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SearchConversationsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[doc = "Total number of matching conversations"]
    #[serde(rename = "_total", default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for SearchConversationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SearchConversationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(total) = &self.total {
                format!("{:?}", total)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "total".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListShiftsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListShiftsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListShiftsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListShiftsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListShiftsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ShiftResponse>>,
}

impl std::fmt::Display for ListShiftsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListShiftsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamShiftsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamShiftsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamShiftsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamShiftsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamShiftsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ShiftResponse>>,
}

impl std::fmt::Display for ListTeamShiftsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamShiftsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateShiftsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateShiftsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateShiftsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateShiftsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateShiftsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ShiftResponse>>,
}

impl std::fmt::Display for ListTeammateShiftsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateShiftsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListShiftTeammatesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListShiftTeammatesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListShiftTeammatesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListShiftTeammatesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListShiftTeammatesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeammateResponse>>,
}

impl std::fmt::Display for ListShiftTeammatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListShiftTeammatesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateSignaturesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateSignaturesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateSignaturesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateSignaturesResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateSignaturesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SignatureResponse>>,
}

impl std::fmt::Display for ListTeammateSignaturesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateSignaturesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamSignaturesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamSignaturesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamSignaturesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamSignaturesResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamSignaturesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SignatureResponse>>,
}

impl std::fmt::Display for ListTeamSignaturesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamSignaturesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTagsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTagsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTagsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTagsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTagsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TagResponse>>,
}

impl std::fmt::Display for ListTagsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTagsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamTagsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamTagsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamTagsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamTagsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamTagsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TagResponse>>,
}

impl std::fmt::Display for ListTeamTagsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamTagsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateTagsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateTagsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateTagsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateTagsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateTagsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TagResponse>>,
}

impl std::fmt::Display for ListTeammateTagsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateTagsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTagChildrenResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTagChildrenResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTagChildrenResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTagChildrenResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTagChildrenResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TagResponse>>,
}

impl std::fmt::Display for ListTagChildrenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTagChildrenResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTaggedConversationsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTaggedConversationsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTaggedConversationsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTaggedConversationsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTaggedConversationsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListTaggedConversationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTaggedConversationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeamsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamsResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeamsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeamResponse>>,
}

impl std::fmt::Display for ListTeamsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeamsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammatesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammatesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammatesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammatesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammatesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<TeammateResponse>>,
}

impl std::fmt::Display for ListTeammatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammatesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListAssignedConversationsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListAssignedConversationsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListAssignedConversationsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListAssignedConversationsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListAssignedConversationsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListAssignedConversationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListAssignedConversationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateInboxesResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListTeammateInboxesResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateInboxesResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeammateInboxesResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListTeammateInboxesResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<InboxResponse>>,
}

impl std::fmt::Display for ListTeammateInboxesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListTeammateInboxesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLinkConversationsResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListLinkConversationsResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListLinkConversationsResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLinkConversationsResponse {
    #[serde(
        rename = "_pagination",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pagination: Option<Pagination>,
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListLinkConversationsResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<ConversationResponse>>,
}

impl std::fmt::Display for ListLinkConversationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListLinkConversationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pagination) = &self.pagination {
                format!("{:?}", pagination)
            } else {
                String::new()
            },
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pagination".to_string(),
            "underscore_links".to_string(),
            "results".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Eq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLinksResponseUnderscoreLinks {
    #[doc = "Link to resource"]
    #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}

impl std::fmt::Display for ListLinksResponseUnderscoreLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListLinksResponseUnderscoreLinks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(self_) = &self.self_ {
            format!("{:?}", self_)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["self_".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLinksResponse {
    #[serde(rename = "_links", default, skip_serializing_if = "Option::is_none")]
    pub underscore_links: Option<ListLinksResponseUnderscoreLinks>,
    #[serde(rename = "_results", default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<LinkResponse>>,
}

impl std::fmt::Display for ListLinksResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ListLinksResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(underscore_links) = &self.underscore_links {
                format!("{:?}", underscore_links)
            } else {
                String::new()
            },
            if let Some(results) = &self.results {
                format!("{:?}", results)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["underscore_links".to_string(), "results".to_string()]
    }
}
