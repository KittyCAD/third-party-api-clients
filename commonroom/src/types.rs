#![doc = r" This module contains the generated types for the library."]
use tabled::Tabled;
pub mod base64 {
    #![doc = " Base64 data that encodes to url safe base64, but can decode from multiple"]
    #![doc = " base64 implementations to account for various clients and libraries. Compatible"]
    #![doc = " with serde and JsonSchema."]
    use serde::de::{Error, Unexpected, Visitor};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::convert::TryFrom;
    use std::fmt;
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
        use super::Base64Data;
        use std::convert::TryFrom;
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
    #![doc = " A library to implement phone numbers for our database and JSON serialization and deserialization."]
    use schemars::JsonSchema;
    use std::str::FromStr;
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
        use super::PhoneNumber;
        use pretty_assertions::assert_eq;
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
pub struct ApiToken {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
    #[serde(
        rename = "communityName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub community_name: Option<String>,
    #[serde(
        rename = "communityId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub community_id: Option<String>,
}

impl std::fmt::Display for ApiToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ApiToken {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(jti) = &self.jti {
                format!("{:?}", jti)
            } else {
                String::new()
            },
            if let Some(community_name) = &self.community_name {
                format!("{:?}", community_name)
            } else {
                String::new()
            },
            if let Some(community_id) = &self.community_id {
                format!("{:?}", community_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "jti".to_string(),
            "community_name".to_string(),
            "community_id".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CommunityMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl std::fmt::Display for CommunityMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CommunityMember {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(id) = &self.id {
            format!("{:?}", id)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["id".to_string()]
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum StatusStatus {
    #[serde(rename = "ok")]
    #[display("ok")]
    Ok,
    #[serde(rename = "failure")]
    #[display("failure")]
    Failure,
    #[serde(rename = "not-found")]
    #[display("not-found")]
    NotFound,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Status {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Status {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(reason) = &self.reason {
                format!("{:?}", reason)
            } else {
                String::new()
            },
            if let Some(errors) = &self.errors {
                format!("{:?}", errors)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "status".to_string(),
            "reason".to_string(),
            "errors".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Name {
    #[serde(rename = "givenName", default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(
        rename = "familyName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub family_name: Option<String>,
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Name {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(given_name) = &self.given_name {
                format!("{:?}", given_name)
            } else {
                String::new()
            },
            if let Some(family_name) = &self.family_name {
                format!("{:?}", family_name)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["given_name".to_string(), "family_name".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Emails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl std::fmt::Display for Emails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Emails {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
            if let Some(primary) = &self.primary {
                format!("{:?}", primary)
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
            "value".to_string(),
            "primary".to_string(),
            "type_".to_string(),
        ]
    }
}

#[doc = "Represents the user account in the Common Room community\nThis account is used for identity mangement via SCIM\nSpecification can be found at https://datatracker.ietf.org/doc/html/rfc7643#section-4.1\n"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Emails>>,
    #[doc = "Indicates whether the user has a login account in the community"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
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

impl tabled::Tabled for User {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(user_name) = &self.user_name {
                format!("{:?}", user_name)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(emails) = &self.emails {
                format!("{:?}", emails)
            } else {
                String::new()
            },
            if let Some(active) = &self.active {
                format!("{:?}", active)
            } else {
                String::new()
            },
            if let Some(schemas) = &self.schemas {
                format!("{:?}", schemas)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "user_name".to_string(),
            "name".to_string(),
            "emails".to_string(),
            "active".to_string(),
            "schemas".to_string(),
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Type {
    #[serde(rename = "github")]
    #[display("github")]
    Github,
    #[serde(rename = "linkedin")]
    #[display("linkedin")]
    Linkedin,
    #[serde(rename = "twitter")]
    #[display("twitter")]
    Twitter,
    #[serde(rename = "youtube")]
    #[display("youtube")]
    Youtube,
    #[serde(rename = "discord")]
    #[display("discord")]
    Discord,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Value {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handle: Option<String>,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Value {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(handle) = &self.handle {
            format!("{:?}", handle)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["handle".to_string()]
    }
}

#[doc = "A user's handle on a social network"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SocialsOneOf {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl std::fmt::Display for SocialsOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SocialsOneOf {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["type_".to_string(), "value".to_string()]
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum SocialsOneOfOneOfType {
    #[serde(rename = "email")]
    #[display("email")]
    Email,
}

impl std::default::Default for SocialsOneOfOneOfType {
    fn default() -> Self {
        SocialsOneOfOneOfType::Email
    }
}

#[doc = "A user's email address"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SocialsOneOfOneOf {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SocialsOneOfOneOfType>,
    #[doc = "email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for SocialsOneOfOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SocialsOneOfOneOf {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["type_".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
pub enum Socials {
    Social(SocialsOneOf),
    Email(SocialsOneOfOneOf),
}

#[doc = "Geographical data for the user"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Location {
    #[doc = "The city where the user lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The state or province where the user lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The country where the user lives"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
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

impl tabled::Tabled for Location {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(city) = &self.city {
                format!("{:?}", city)
            } else {
                String::new()
            },
            if let Some(region) = &self.region {
                format!("{:?}", region)
            } else {
                String::new()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "city".to_string(),
            "region".to_string(),
            "country".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateOrUpdateCommunityMemberRequestBody {
    #[doc = "Source of the information"]
    pub source: String,
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "avatarUrl", default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[doc = "Member Bio"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub socials: Option<Vec<Socials>>,
    #[doc = "Segment Id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment: Option<f64>,
    #[doc = "Name of the organization member is employed at"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "Domain of the organization member is employed at"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[doc = "Member's job title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Member's job category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "Geographical data for the user"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

impl std::fmt::Display for CreateOrUpdateCommunityMemberRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateOrUpdateCommunityMemberRequestBody {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<String> {
        vec![
            self.source.clone(),
            if let Some(full_name) = &self.full_name {
                format!("{:?}", full_name)
            } else {
                String::new()
            },
            if let Some(avatar_url) = &self.avatar_url {
                format!("{:?}", avatar_url)
            } else {
                String::new()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description)
            } else {
                String::new()
            },
            if let Some(socials) = &self.socials {
                format!("{:?}", socials)
            } else {
                String::new()
            },
            if let Some(segment) = &self.segment {
                format!("{:?}", segment)
            } else {
                String::new()
            },
            if let Some(company) = &self.company {
                format!("{:?}", company)
            } else {
                String::new()
            },
            if let Some(domain) = &self.domain {
                format!("{:?}", domain)
            } else {
                String::new()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title)
            } else {
                String::new()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role)
            } else {
                String::new()
            },
            if let Some(location) = &self.location {
                format!("{:?}", location)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "source".to_string(),
            "full_name".to_string(),
            "avatar_url".to_string(),
            "description".to_string(),
            "socials".to_string(),
            "segment".to_string(),
            "company".to_string(),
            "domain".to_string(),
            "title".to_string(),
            "role".to_string(),
            "location".to_string(),
        ]
    }
}

#[doc = "A user's handle on a social network"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateOrUpdateCommunityMemberRequestBodySocialsOneOf {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
}

impl std::fmt::Display for CreateOrUpdateCommunityMemberRequestBodySocialsOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateOrUpdateCommunityMemberRequestBodySocialsOneOf {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["type_".to_string(), "value".to_string()]
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOfType {
    #[serde(rename = "email")]
    #[display("email")]
    Email,
}

impl std::default::Default for CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOfType {
    fn default() -> Self {
        CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOfType::Email
    }
}

#[doc = "A user's email address"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOf {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOfType>,
    #[doc = "email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOf {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["type_".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
pub enum CreateOrUpdateCommunityMemberRequestBodySocials {
    Social(CreateOrUpdateCommunityMemberRequestBodySocialsOneOf),
    Email(CreateOrUpdateCommunityMemberRequestBodySocialsOneOfOneOf),
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddNoteToMemberRequestBody {
    #[doc = "Must be email, twitter, github, or linkedin"]
    #[serde(
        rename = "socialType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub social_type: Option<String>,
    #[doc = "Value corresponding to socialType"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Note content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl std::fmt::Display for AddNoteToMemberRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AddNoteToMemberRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(social_type) = &self.social_type {
                format!("{:?}", social_type)
            } else {
                String::new()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
            if let Some(note) = &self.note {
                format!("{:?}", note)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "social_type".to_string(),
            "value".to_string(),
            "note".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetMemberCustomFieldsResponse {
    #[doc = "Custom field name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Custom field type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl std::fmt::Display for GetMemberCustomFieldsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetMemberCustomFieldsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
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
            if let Some(values) = &self.values {
                format!("{:?}", values)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "type_".to_string(),
            "values".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomFieldValue {
    #[doc = "enum, string, date, int, url, boolean"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Value of custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl std::fmt::Display for CustomFieldValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CustomFieldValue {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["type_".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SetMemberCustomFieldValueRequestBody {
    #[doc = "Must be email, twitter, github, or linkedin"]
    #[serde(
        rename = "socialType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub social_type: Option<String>,
    #[doc = "Value corresponding to socialType"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Id of the custom field to set"]
    #[serde(
        rename = "customFieldId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_field_id: Option<f64>,
    #[serde(
        rename = "customFieldValue",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_field_value: Option<CustomFieldValue>,
}

impl std::fmt::Display for SetMemberCustomFieldValueRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SetMemberCustomFieldValueRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(social_type) = &self.social_type {
                format!("{:?}", social_type)
            } else {
                String::new()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
            if let Some(custom_field_id) = &self.custom_field_id {
                format!("{:?}", custom_field_id)
            } else {
                String::new()
            },
            if let Some(custom_field_value) = &self.custom_field_value {
                format!("{:?}", custom_field_value)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "social_type".to_string(),
            "value".to_string(),
            "custom_field_id".to_string(),
            "custom_field_value".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddTagsToMemberRequestBody {
    #[doc = "Must be email, twitter, github, or linkedin"]
    #[serde(
        rename = "socialType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub social_type: Option<String>,
    #[doc = "Value corresponding to socialType"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "Comma separated list of tags"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

impl std::fmt::Display for AddTagsToMemberRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AddTagsToMemberRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(social_type) = &self.social_type {
                format!("{:?}", social_type)
            } else {
                String::new()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "social_type".to_string(),
            "value".to_string(),
            "tags".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Segments {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for Segments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Segments {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["id".to_string(), "name".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "enum, string, date, int, url, boolean"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Value of custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl std::fmt::Display for CustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CustomFields {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
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
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "type_".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetMemberBySocialsResponse {
    #[serde(rename = "fullName", default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activities_count: Option<f64>,
    #[doc = "Avatar URL string"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Date when member appeared in your community in ISO 8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_seen: Option<String>,
    #[doc = "Date of the member's last activity in ISO 8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_active: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segments>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub github: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkedin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discord: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub youtube: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_room_member_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFields>>,
}

impl std::fmt::Display for GetMemberBySocialsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetMemberBySocialsResponse {
    const LENGTH: usize = 19;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(full_name) = &self.full_name {
                format!("{:?}", full_name)
            } else {
                String::new()
            },
            if let Some(activities_count) = &self.activities_count {
                format!("{:?}", activities_count)
            } else {
                String::new()
            },
            if let Some(avatar) = &self.avatar {
                format!("{:?}", avatar)
            } else {
                String::new()
            },
            if let Some(bio) = &self.bio {
                format!("{:?}", bio)
            } else {
                String::new()
            },
            if let Some(organization) = &self.organization {
                format!("{:?}", organization)
            } else {
                String::new()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title)
            } else {
                String::new()
            },
            if let Some(first_seen) = &self.first_seen {
                format!("{:?}", first_seen)
            } else {
                String::new()
            },
            if let Some(last_active) = &self.last_active {
                format!("{:?}", last_active)
            } else {
                String::new()
            },
            if let Some(location) = &self.location {
                format!("{:?}", location)
            } else {
                String::new()
            },
            if let Some(member_tags) = &self.member_tags {
                format!("{:?}", member_tags)
            } else {
                String::new()
            },
            if let Some(segments) = &self.segments {
                format!("{:?}", segments)
            } else {
                String::new()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url)
            } else {
                String::new()
            },
            if let Some(twitter) = &self.twitter {
                format!("{:?}", twitter)
            } else {
                String::new()
            },
            if let Some(github) = &self.github {
                format!("{:?}", github)
            } else {
                String::new()
            },
            if let Some(linkedin) = &self.linkedin {
                format!("{:?}", linkedin)
            } else {
                String::new()
            },
            if let Some(discord) = &self.discord {
                format!("{:?}", discord)
            } else {
                String::new()
            },
            if let Some(youtube) = &self.youtube {
                format!("{:?}", youtube)
            } else {
                String::new()
            },
            if let Some(common_room_member_url) = &self.common_room_member_url {
                format!("{:?}", common_room_member_url)
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
            "full_name".to_string(),
            "activities_count".to_string(),
            "avatar".to_string(),
            "bio".to_string(),
            "organization".to_string(),
            "title".to_string(),
            "first_seen".to_string(),
            "last_active".to_string(),
            "location".to_string(),
            "member_tags".to_string(),
            "segments".to_string(),
            "url".to_string(),
            "twitter".to_string(),
            "github".to_string(),
            "linkedin".to_string(),
            "discord".to_string(),
            "youtube".to_string(),
            "common_room_member_url".to_string(),
            "custom_fields".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetMemberBySocialsResponseSegments {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for GetMemberBySocialsResponseSegments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetMemberBySocialsResponseSegments {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["id".to_string(), "name".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetMemberBySocialsResponseCustomFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "enum, string, date, int, url, boolean"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[doc = "Value of custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl std::fmt::Display for GetMemberBySocialsResponseCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetMemberBySocialsResponseCustomFields {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
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
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "type_".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ActiveCommunityMembersByRoleResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    #[serde(rename = "Resources", default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<User>>,
    #[doc = "Number of resources in the list"]
    #[serde(
        rename = "totalResults",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_results: Option<i64>,
    #[doc = "Constant number of items per page"]
    #[serde(
        rename = "itemsPerPage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub items_per_page: Option<i64>,
    #[doc = "1-based index for the returned list"]
    #[serde(
        rename = "startIndex",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_index: Option<i64>,
}

impl std::fmt::Display for ActiveCommunityMembersByRoleResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for ActiveCommunityMembersByRoleResponse {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(schemas) = &self.schemas {
                format!("{:?}", schemas)
            } else {
                String::new()
            },
            if let Some(resources) = &self.resources {
                format!("{:?}", resources)
            } else {
                String::new()
            },
            if let Some(total_results) = &self.total_results {
                format!("{:?}", total_results)
            } else {
                String::new()
            },
            if let Some(items_per_page) = &self.items_per_page {
                format!("{:?}", items_per_page)
            } else {
                String::new()
            },
            if let Some(start_index) = &self.start_index {
                format!("{:?}", start_index)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "schemas".to_string(),
            "resources".to_string(),
            "total_results".to_string(),
            "items_per_page".to_string(),
            "start_index".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCommunityMemberWithSpecificRoleRequestBodyEmails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

impl std::fmt::Display for CreateCommunityMemberWithSpecificRoleRequestBodyEmails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateCommunityMemberWithSpecificRoleRequestBodyEmails {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(value) = &self.value {
                format!("{:?}", value)
            } else {
                String::new()
            },
            if let Some(primary) = &self.primary {
                format!("{:?}", primary)
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
            "value".to_string(),
            "primary".to_string(),
            "type_".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCommunityMemberWithSpecificRoleRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<CreateCommunityMemberWithSpecificRoleRequestBodyEmails>>,
    #[doc = "Indicates whether the user has a login account in the community"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl std::fmt::Display for CreateCommunityMemberWithSpecificRoleRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateCommunityMemberWithSpecificRoleRequestBody {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(schemas) = &self.schemas {
                format!("{:?}", schemas)
            } else {
                String::new()
            },
            if let Some(user_name) = &self.user_name {
                format!("{:?}", user_name)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(emails) = &self.emails {
                format!("{:?}", emails)
            } else {
                String::new()
            },
            if let Some(active) = &self.active {
                format!("{:?}", active)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "schemas".to_string(),
            "user_name".to_string(),
            "name".to_string(),
            "emails".to_string(),
            "active".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct OperationsValue {
    #[doc = "Will upgrade or downgrade user account's role based on this property. When set to false\nuser account is downgraded\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl std::fmt::Display for OperationsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for OperationsValue {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(active) = &self.active {
            format!("{:?}", active)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["active".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Operations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<OperationsValue>,
}

impl std::fmt::Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Operations {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(op) = &self.op {
                format!("{:?}", op)
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
        vec!["op".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUserAccountRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
    #[serde(
        rename = "Operations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operations: Option<Vec<Operations>>,
}

impl std::fmt::Display for UpdateUserAccountRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateUserAccountRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(schemas) = &self.schemas {
                format!("{:?}", schemas)
            } else {
                String::new()
            },
            if let Some(operations) = &self.operations {
                format!("{:?}", operations)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["schemas".to_string(), "operations".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUserAccountRequestBodyOperationsValue {
    #[doc = "Will upgrade or downgrade user account's role based on this property. When set to false\nuser account is downgraded\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl std::fmt::Display for UpdateUserAccountRequestBodyOperationsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateUserAccountRequestBodyOperationsValue {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(active) = &self.active {
            format!("{:?}", active)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["active".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUserAccountRequestBodyOperations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<UpdateUserAccountRequestBodyOperationsValue>,
}

impl std::fmt::Display for UpdateUserAccountRequestBodyOperations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for UpdateUserAccountRequestBodyOperations {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(op) = &self.op {
                format!("{:?}", op)
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
        vec!["op".to_string(), "value".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateUserInputActivityRequestBody {
    #[doc = "Source of the information"]
    pub source: String,
    #[doc = "Must be email, twitter, github, or linkedin"]
    #[serde(rename = "socialType")]
    pub social_type: String,
    #[doc = "Value corresponding to socialType"]
    pub value: String,
    #[doc = "ActivityType. Use the GetActivityType API to get a list of types"]
    #[serde(rename = "activityType")]
    pub activity_type: ActivityType,
    #[serde(rename = "activityBody")]
    pub activity_body: String,
    #[doc = "Date when the activity occurred in ISO 8601 format"]
    #[serde(
        rename = "occurredAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub occurred_at: Option<String>,
    #[doc = "Comma separated string of tags to apply to the activity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
}

impl std::fmt::Display for CreateUserInputActivityRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CreateUserInputActivityRequestBody {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            self.source.clone(),
            self.social_type.clone(),
            self.value.clone(),
            format!("{:?}", self.activity_type),
            self.activity_body.clone(),
            if let Some(occurred_at) = &self.occurred_at {
                format!("{:?}", occurred_at)
            } else {
                String::new()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "source".to_string(),
            "social_type".to_string(),
            "value".to_string(),
            "activity_type".to_string(),
            "activity_body".to_string(),
            "occurred_at".to_string(),
            "tags".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetActivityTypesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "displayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for GetActivityTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetActivityTypesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id)
            } else {
                String::new()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name)
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
            "id".to_string(),
            "display_name".to_string(),
            "name".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSegmentsResponse {
    #[doc = "Segment id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "Display name of segment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for GetSegmentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetSegmentsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["id".to_string(), "name".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSegmentStatusesResponse {
    #[doc = "Status id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "Display text of status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for GetSegmentStatusesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for GetSegmentStatusesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
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
        vec!["id".to_string(), "name".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddMembersToSegmentRequestBody {
    #[doc = "Must be email, twitter, github, or linkedin"]
    #[serde(rename = "socialType")]
    pub social_type: String,
    #[doc = "Comma separated list of values corresponding to socialType"]
    pub value: String,
    #[serde(rename = "statusId", default, skip_serializing_if = "Option::is_none")]
    pub status_id: Option<f64>,
}

impl std::fmt::Display for AddMembersToSegmentRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AddMembersToSegmentRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            self.social_type.clone(),
            self.value.clone(),
            if let Some(status_id) = &self.status_id {
                format!("{:?}", status_id)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "social_type".to_string(),
            "value".to_string(),
            "status_id".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddNoteToSegmentRequestBody {
    #[serde(rename = "segmentId", default, skip_serializing_if = "Option::is_none")]
    pub segment_id: Option<f64>,
    #[doc = "Note content"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl std::fmt::Display for AddNoteToSegmentRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for AddNoteToSegmentRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(segment_id) = &self.segment_id {
                format!("{:?}", segment_id)
            } else {
                String::new()
            },
            if let Some(note) = &self.note {
                format!("{:?}", note)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec!["segment_id".to_string(), "note".to_string()]
    }
}
