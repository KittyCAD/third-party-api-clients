#![doc = r" This module contains the generated types for the library."]
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

#[cfg(feature = "requests")]
pub mod multipart {
    #![doc = " Multipart form data types."]
    #[doc = " An attachement to a multipart form."]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Attachment {
        #[doc = " The name of the field."]
        pub name: String,
        #[doc = " The filename of the attachment."]
        pub filename: Option<String>,
        #[doc = " The content type of the attachment."]
        pub content_type: Option<String>,
        #[doc = " The data of the attachment."]
        pub data: Vec<u8>,
    }

    impl std::convert::TryFrom<Attachment> for reqwest::multipart::Part {
        type Error = reqwest::Error;
        fn try_from(attachment: Attachment) -> Result<Self, Self::Error> {
            let mut part = reqwest::multipart::Part::bytes(attachment.data);
            if let Some(filename) = attachment.filename {
                part = part.file_name(filename);
            }
            if let Some(content_type) = attachment.content_type {
                part = part.mime_str(&content_type)?;
            }
            Ok(part)
        }
    }

    impl std::convert::TryFrom<std::path::PathBuf> for Attachment {
        type Error = std::io::Error;
        fn try_from(path: std::path::PathBuf) -> Result<Self, Self::Error> {
            let filename = path
                .file_name()
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid filename")
                })?
                .to_str()
                .ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid filename")
                })?
                .to_string();
            let content_type = mime_guess::from_path(&path).first_raw();
            let data = std::fs::read(path)?;
            Ok(Attachment {
                name: "file".to_string(),
                filename: Some(filename),
                content_type: content_type.map(|s| s.to_string()),
                data,
            })
        }
    }
}

#[cfg(feature = "requests")]
pub mod paginate {
    #![doc = " Utility functions used for pagination."]
    use anyhow::Result;
    #[doc = " A trait for types that allow pagination."]
    pub trait Pagination {
        #[doc = " The item that is paginated."]
        type Item: serde::de::DeserializeOwned;
        #[doc = " Returns true if the response has more pages."]
        fn has_more_pages(&self) -> bool;
        #[doc = " Returns the next page token."]
        fn next_page_token(&self) -> Option<String>;
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

#[cfg(feature = "requests")]
pub mod error {
    #![doc = " Error methods."]
    #[doc = " Error produced by generated client methods."]
    pub enum Error {
        #[doc = " The request did not conform to API requirements."]
        InvalidRequest(String),
        #[cfg(feature = "retry")]
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
            #[cfg(feature = "retry")]
            #[doc = " The error."]
            error: reqwest_middleware::Error,
            #[cfg(not(feature = "retry"))]
            #[doc = " The error."]
            error: reqwest::Error,
            #[doc = " The full response."]
            response: reqwest::Response,
        },
        #[doc = " An error from the server."]
        Server {
            #[doc = " The text from the body."]
            body: String,
            #[doc = " The response status."]
            status: reqwest::StatusCode,
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
                #[cfg(feature = "retry")]
                Error::CommunicationError(reqwest_middleware::Error::Reqwest(e)) => e.status(),
                #[cfg(feature = "retry")]
                Error::CommunicationError(reqwest_middleware::Error::Middleware(_)) => None,
                Error::SerdeError { error: _, status } => Some(*status),
                Error::InvalidResponsePayload { error: _, response } => Some(response.status()),
                Error::Server { body: _, status } => Some(*status),
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

    #[cfg(feature = "retry")]
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

    impl From<serde_json::Error> for Error {
        fn from(e: serde_json::Error) -> Self {
            Self::SerdeError {
                error: format_serde_error::SerdeError::new(String::new(), e),
                status: reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Error::InvalidRequest(s) => {
                    write!(f, "Invalid Request: {}", s)
                }
                #[cfg(feature = "retry")]
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
                Error::Server { body, status } => {
                    write!(f, "Server Error: {} {}", status, body)
                }
                Error::UnexpectedResponse(r) => {
                    write!(f, "Unexpected Response: {:?}", r)
                }
            }
        }
    }

    impl std::fmt::Debug for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            std::fmt::Display::fmt(self, f)
        }
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                #[cfg(feature = "retry")]
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
pub struct StandardError {
    #[serde(
        rename = "subCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_category: Option<String>,
    pub context: std::collections::HashMap<String, Vec<String>>,
    pub links: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub category: String,
    pub message: String,
    pub errors: Vec<ErrorDetail>,
    pub status: String,
}

impl std::fmt::Display for StandardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StandardError {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(sub_category) = &self.sub_category {
                format!("{:?}", sub_category).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.context).into(),
            format!("{:?}", self.links).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            self.category.clone().into(),
            self.message.clone().into(),
            format!("{:?}", self.errors).into(),
            self.status.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "sub_category".into(),
            "context".into(),
            "links".into(),
            "id".into(),
            "category".into(),
            "message".into(),
            "errors".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CollectionResponseAssociatedId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paging: Option<Paging>,
    pub results: Vec<AssociatedId>,
}

impl std::fmt::Display for CollectionResponseAssociatedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CollectionResponseAssociatedId {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(paging) = &self.paging {
                format!("{:?}", paging).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["paging".into(), "results".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicAssociationsForObject {
    pub types: Vec<AssociationSpec>,
    pub to: PublicObjectId,
}

impl std::fmt::Display for PublicAssociationsForObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicAssociationsForObject {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.types).into(),
            format!("{:?}", self.to).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["types".into(), "to".into()]
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
pub enum Status {
    #[serde(rename = "PENDING")]
    #[display("PENDING")]
    Pending,
    #[serde(rename = "PROCESSING")]
    #[display("PROCESSING")]
    Processing,
    #[serde(rename = "CANCELED")]
    #[display("CANCELED")]
    Canceled,
    #[serde(rename = "COMPLETE")]
    #[display("COMPLETE")]
    Complete,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchResponseSimplePublicObject {
    #[serde(rename = "completedAt")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(
        rename = "requestedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "startedAt")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::HashMap<String, String>>,
    pub results: Vec<SimplePublicObject>,
    pub status: Status,
}

impl std::fmt::Display for BatchResponseSimplePublicObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchResponseSimplePublicObject {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.completed_at).into(),
            if let Some(requested_at) = &self.requested_at {
                format!("{:?}", requested_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.started_at).into(),
            if let Some(links) = &self.links {
                format!("{:?}", links).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            format!("{:?}", self.status).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "requested_at".into(),
            "started_at".into(),
            "links".into(),
            "results".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FilterGroup {
    pub filters: Vec<Filter>,
}

impl std::fmt::Display for FilterGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FilterGroup {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.filters).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["filters".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ErrorDetail {
    #[doc = "A specific category that contains more specific detail about the error"]
    #[serde(
        rename = "subCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_category: Option<String>,
    #[doc = "The status code associated with the error detail"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "The name of the field or parameter in which the error was found."]
    #[serde(rename = "in", default, skip_serializing_if = "Option::is_none")]
    pub in_: Option<String>,
    #[doc = "Context about the error condition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, Vec<String>>>,
    #[doc = "A human readable message describing the error along with remediation steps where \
             appropriate"]
    pub message: String,
}

impl std::fmt::Display for ErrorDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ErrorDetail {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(sub_category) = &self.sub_category {
                format!("{:?}", sub_category).into()
            } else {
                String::new().into()
            },
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            if let Some(in_) = &self.in_ {
                format!("{:?}", in_).into()
            } else {
                String::new().into()
            },
            if let Some(context) = &self.context {
                format!("{:?}", context).into()
            } else {
                String::new().into()
            },
            self.message.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "sub_category".into(),
            "code".into(),
            "in_".into(),
            "context".into(),
            "message".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ForwardPaging {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<NextPage>,
}

impl std::fmt::Display for ForwardPaging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ForwardPaging {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(next) = &self.next {
            format!("{:?}", next).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["next".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SimplePublicObjectId {
    pub id: String,
}

impl std::fmt::Display for SimplePublicObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SimplePublicObjectId {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.id.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchReadInputSimplePublicObjectId {
    #[serde(rename = "propertiesWithHistory")]
    pub properties_with_history: Vec<String>,
    #[serde(
        rename = "idProperty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id_property: Option<String>,
    pub inputs: Vec<SimplePublicObjectId>,
    pub properties: Vec<String>,
}

impl std::fmt::Display for BatchReadInputSimplePublicObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchReadInputSimplePublicObjectId {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.properties_with_history).into(),
            if let Some(id_property) = &self.id_property {
                format!("{:?}", id_property).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.inputs).into(),
            format!("{:?}", self.properties).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "properties_with_history".into(),
            "id_property".into(),
            "inputs".into(),
            "properties".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchInputSimplePublicObjectId {
    pub inputs: Vec<SimplePublicObjectId>,
}

impl std::fmt::Display for BatchInputSimplePublicObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchInputSimplePublicObjectId {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.inputs).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["inputs".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ValueWithTimestamp {
    #[serde(rename = "sourceId", default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "sourceType")]
    pub source_type: String,
    #[serde(
        rename = "sourceLabel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_label: Option<String>,
    #[serde(
        rename = "updatedByUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub updated_by_user_id: Option<i32>,
    pub value: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for ValueWithTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ValueWithTimestamp {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(source_id) = &self.source_id {
                format!("{:?}", source_id).into()
            } else {
                String::new().into()
            },
            self.source_type.clone().into(),
            if let Some(source_label) = &self.source_label {
                format!("{:?}", source_label).into()
            } else {
                String::new().into()
            },
            if let Some(updated_by_user_id) = &self.updated_by_user_id {
                format!("{:?}", updated_by_user_id).into()
            } else {
                String::new().into()
            },
            self.value.clone().into(),
            format!("{:?}", self.timestamp).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "source_id".into(),
            "source_type".into(),
            "source_label".into(),
            "updated_by_user_id".into(),
            "value".into(),
            "timestamp".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CollectionResponseWithTotalSimplePublicObjectForwardPaging {
    pub total: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paging: Option<ForwardPaging>,
    pub results: Vec<SimplePublicObject>,
}

impl std::fmt::Display for CollectionResponseWithTotalSimplePublicObjectForwardPaging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CollectionResponseWithTotalSimplePublicObjectForwardPaging {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.total).into(),
            if let Some(paging) = &self.paging {
                format!("{:?}", paging).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["total".into(), "paging".into(), "results".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SimplePublicObject {
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        rename = "archivedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "propertiesWithHistory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub properties_with_history: Option<std::collections::HashMap<String, Vec<ValueWithTimestamp>>>,
    pub id: String,
    pub properties: std::collections::HashMap<String, Option<String>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for SimplePublicObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SimplePublicObject {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.created_at).into(),
            if let Some(archived) = &self.archived {
                format!("{:?}", archived).into()
            } else {
                String::new().into()
            },
            if let Some(archived_at) = &self.archived_at {
                format!("{:?}", archived_at).into()
            } else {
                String::new().into()
            },
            if let Some(properties_with_history) = &self.properties_with_history {
                format!("{:?}", properties_with_history).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            format!("{:?}", self.properties).into(),
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "archived".into(),
            "archived_at".into(),
            "properties_with_history".into(),
            "id".into(),
            "properties".into(),
            "updated_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicObjectId {
    pub id: String,
}

impl std::fmt::Display for PublicObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicObjectId {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.id.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Paging {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<NextPage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev: Option<PreviousPage>,
}

impl std::fmt::Display for Paging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Paging {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(next) = &self.next {
                format!("{:?}", next).into()
            } else {
                String::new().into()
            },
            if let Some(prev) = &self.prev {
                format!("{:?}", prev).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["next".into(), "prev".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicObjectSearchRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    pub limit: i32,
    pub after: String,
    pub sorts: Vec<String>,
    pub properties: Vec<String>,
    #[serde(rename = "filterGroups")]
    pub filter_groups: Vec<FilterGroup>,
}

impl std::fmt::Display for PublicObjectSearchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicObjectSearchRequest {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(query) = &self.query {
                format!("{:?}", query).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.limit).into(),
            self.after.clone().into(),
            format!("{:?}", self.sorts).into(),
            format!("{:?}", self.properties).into(),
            format!("{:?}", self.filter_groups).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "query".into(),
            "limit".into(),
            "after".into(),
            "sorts".into(),
            "properties".into(),
            "filter_groups".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Error {
    #[doc = "A specific category that contains more specific detail about the error"]
    #[serde(
        rename = "subCategory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sub_category: Option<String>,
    #[doc = "Context about the error condition"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<std::collections::HashMap<String, Vec<String>>>,
    #[doc = "A unique identifier for the request. Include this value with any error reports or \
             support tickets"]
    #[serde(rename = "correlationId")]
    pub correlation_id: uuid::Uuid,
    #[doc = "A map of link names to associated URIs containing documentation about the error or \
             recommended remediation steps"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::HashMap<String, String>>,
    #[doc = "A human readable message describing the error along with remediation steps where \
             appropriate"]
    pub message: String,
    #[doc = "The error category"]
    pub category: String,
    #[doc = "further information about the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ErrorDetail>>,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Error {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(sub_category) = &self.sub_category {
                format!("{:?}", sub_category).into()
            } else {
                String::new().into()
            },
            if let Some(context) = &self.context {
                format!("{:?}", context).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.correlation_id).into(),
            if let Some(links) = &self.links {
                format!("{:?}", links).into()
            } else {
                String::new().into()
            },
            self.message.clone().into(),
            self.category.clone().into(),
            if let Some(errors) = &self.errors {
                format!("{:?}", errors).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "sub_category".into(),
            "context".into(),
            "correlation_id".into(),
            "links".into(),
            "message".into(),
            "category".into(),
            "errors".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchResponseSimplePublicObjectWithErrors {
    #[serde(rename = "completedAt")]
    pub completed_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "numErrors", default, skip_serializing_if = "Option::is_none")]
    pub num_errors: Option<i32>,
    #[serde(
        rename = "requestedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "startedAt")]
    pub started_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<std::collections::HashMap<String, String>>,
    pub results: Vec<SimplePublicObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<StandardError>>,
    pub status: Status,
}

impl std::fmt::Display for BatchResponseSimplePublicObjectWithErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchResponseSimplePublicObjectWithErrors {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.completed_at).into(),
            if let Some(num_errors) = &self.num_errors {
                format!("{:?}", num_errors).into()
            } else {
                String::new().into()
            },
            if let Some(requested_at) = &self.requested_at {
                format!("{:?}", requested_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.started_at).into(),
            if let Some(links) = &self.links {
                format!("{:?}", links).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(errors) = &self.errors {
                format!("{:?}", errors).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "completed_at".into(),
            "num_errors".into(),
            "requested_at".into(),
            "started_at".into(),
            "links".into(),
            "results".into(),
            "errors".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicGdprDeleteInput {
    #[serde(
        rename = "idProperty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id_property: Option<String>,
    #[serde(rename = "objectId")]
    pub object_id: String,
}

impl std::fmt::Display for PublicGdprDeleteInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicGdprDeleteInput {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id_property) = &self.id_property {
                format!("{:?}", id_property).into()
            } else {
                String::new().into()
            },
            self.object_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id_property".into(), "object_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SimplePublicObjectInput {
    pub properties: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for SimplePublicObjectInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SimplePublicObjectInput {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.properties).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["properties".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CollectionResponseSimplePublicObjectWithAssociationsForwardPaging {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paging: Option<ForwardPaging>,
    pub results: Vec<SimplePublicObjectWithAssociations>,
}

impl std::fmt::Display for CollectionResponseSimplePublicObjectWithAssociationsForwardPaging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CollectionResponseSimplePublicObjectWithAssociationsForwardPaging {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(paging) = &self.paging {
                format!("{:?}", paging).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["paging".into(), "results".into()]
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
pub enum AssociationCategory {
    #[serde(rename = "HUBSPOT_DEFINED")]
    #[display("HUBSPOT_DEFINED")]
    HubspotDefined,
    #[serde(rename = "USER_DEFINED")]
    #[display("USER_DEFINED")]
    UserDefined,
    #[serde(rename = "INTEGRATOR_DEFINED")]
    #[display("INTEGRATOR_DEFINED")]
    IntegratorDefined,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AssociationSpec {
    #[serde(rename = "associationCategory")]
    pub association_category: AssociationCategory,
    #[serde(rename = "associationTypeId")]
    pub association_type_id: i32,
}

impl std::fmt::Display for AssociationSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AssociationSpec {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.association_category).into(),
            format!("{:?}", self.association_type_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["association_category".into(), "association_type_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicMergeInput {
    #[serde(rename = "objectIdToMerge")]
    pub object_id_to_merge: String,
    #[serde(rename = "primaryObjectId")]
    pub primary_object_id: String,
}

impl std::fmt::Display for PublicMergeInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicMergeInput {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.object_id_to_merge.clone().into(),
            self.primary_object_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["object_id_to_merge".into(), "primary_object_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SimplePublicObjectWithAssociations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associations: Option<std::collections::HashMap<String, CollectionResponseAssociatedId>>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        rename = "archivedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(
        rename = "propertiesWithHistory",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub properties_with_history: Option<std::collections::HashMap<String, Vec<ValueWithTimestamp>>>,
    pub id: String,
    pub properties: std::collections::HashMap<String, Option<String>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for SimplePublicObjectWithAssociations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SimplePublicObjectWithAssociations {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(associations) = &self.associations {
                format!("{:?}", associations).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(archived) = &self.archived {
                format!("{:?}", archived).into()
            } else {
                String::new().into()
            },
            if let Some(archived_at) = &self.archived_at {
                format!("{:?}", archived_at).into()
            } else {
                String::new().into()
            },
            if let Some(properties_with_history) = &self.properties_with_history {
                format!("{:?}", properties_with_history).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            format!("{:?}", self.properties).into(),
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "associations".into(),
            "created_at".into(),
            "archived".into(),
            "archived_at".into(),
            "properties_with_history".into(),
            "id".into(),
            "properties".into(),
            "updated_at".into(),
        ]
    }
}

#[doc = "null"]
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
pub enum Operator {
    #[serde(rename = "EQ")]
    #[display("EQ")]
    Eq,
    #[serde(rename = "NEQ")]
    #[display("NEQ")]
    Neq,
    #[serde(rename = "LT")]
    #[display("LT")]
    Lt,
    #[serde(rename = "LTE")]
    #[display("LTE")]
    Lte,
    #[serde(rename = "GT")]
    #[display("GT")]
    Gt,
    #[serde(rename = "GTE")]
    #[display("GTE")]
    Gte,
    #[serde(rename = "BETWEEN")]
    #[display("BETWEEN")]
    Between,
    #[serde(rename = "IN")]
    #[display("IN")]
    In,
    #[serde(rename = "NOT_IN")]
    #[display("NOT_IN")]
    NotIn,
    #[serde(rename = "HAS_PROPERTY")]
    #[display("HAS_PROPERTY")]
    HasProperty,
    #[serde(rename = "NOT_HAS_PROPERTY")]
    #[display("NOT_HAS_PROPERTY")]
    NotHasProperty,
    #[serde(rename = "CONTAINS_TOKEN")]
    #[display("CONTAINS_TOKEN")]
    ContainsToken,
    #[serde(rename = "NOT_CONTAINS_TOKEN")]
    #[display("NOT_CONTAINS_TOKEN")]
    NotContainsToken,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Filter {
    #[serde(rename = "highValue", default, skip_serializing_if = "Option::is_none")]
    pub high_value: Option<String>,
    #[serde(rename = "propertyName")]
    pub property_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "null"]
    pub operator: Operator,
}

impl std::fmt::Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Filter {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(high_value) = &self.high_value {
                format!("{:?}", high_value).into()
            } else {
                String::new().into()
            },
            self.property_name.clone().into(),
            if let Some(values) = &self.values {
                format!("{:?}", values).into()
            } else {
                String::new().into()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.operator).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "high_value".into(),
            "property_name".into(),
            "values".into(),
            "value".into(),
            "operator".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchInputSimplePublicObjectBatchInput {
    pub inputs: Vec<SimplePublicObjectBatchInput>,
}

impl std::fmt::Display for BatchInputSimplePublicObjectBatchInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchInputSimplePublicObjectBatchInput {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.inputs).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["inputs".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchInputSimplePublicObjectInputForCreate {
    pub inputs: Vec<SimplePublicObjectInputForCreate>,
}

impl std::fmt::Display for BatchInputSimplePublicObjectInputForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchInputSimplePublicObjectInputForCreate {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.inputs).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["inputs".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PreviousPage {
    pub before: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}

impl std::fmt::Display for PreviousPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PreviousPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.before.clone().into(),
            if let Some(link) = &self.link {
                format!("{:?}", link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["before".into(), "link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SimplePublicObjectBatchInput {
    #[serde(
        rename = "idProperty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id_property: Option<String>,
    pub id: String,
    pub properties: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for SimplePublicObjectBatchInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SimplePublicObjectBatchInput {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id_property) = &self.id_property {
                format!("{:?}", id_property).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            format!("{:?}", self.properties).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id_property".into(), "id".into(), "properties".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AssociatedId {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
}

impl std::fmt::Display for AssociatedId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AssociatedId {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.id.clone().into(), self.type_.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "type_".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct NextPage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    pub after: String,
}

impl std::fmt::Display for NextPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NextPage {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(link) = &self.link {
                format!("{:?}", link).into()
            } else {
                String::new().into()
            },
            self.after.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["link".into(), "after".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SimplePublicObjectInputForCreate {
    pub associations: Vec<PublicAssociationsForObject>,
    pub properties: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for SimplePublicObjectInputForCreate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SimplePublicObjectInputForCreate {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.associations).into(),
            format!("{:?}", self.properties).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["associations".into(), "properties".into()]
    }
}
