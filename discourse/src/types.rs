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
    impl Visitor<'_> for Base64DataVisitor {
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
    use std::path::PathBuf;
    #[doc = " An attachement to a multipart form."]
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Attachment {
        #[doc = " The name of the field."]
        pub name: String,
        #[doc = " The file path of the attachment."]
        pub filepath: Option<PathBuf>,
        #[doc = " The content type of the attachment."]
        pub content_type: Option<String>,
        #[doc = " The data of the attachment."]
        pub data: Vec<u8>,
    }

    impl std::convert::TryFrom<Attachment> for reqwest::multipart::Part {
        type Error = reqwest::Error;
        fn try_from(attachment: Attachment) -> Result<Self, Self::Error> {
            let mut part = reqwest::multipart::Part::bytes(attachment.data);
            if let Some(filepath) = attachment.filepath {
                part = part.file_name(filepath.to_string_lossy().to_string());
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
            let content_type = mime_guess::from_path(&path).first_raw();
            let data = std::fs::read(&path)?;
            Ok(Attachment {
                name: "file".to_string(),
                filepath: Some(path),
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
pub enum IncludeDetails {
    #[serde(rename = "true")]
    #[display("true")]
    True,
    #[serde(rename = "false")]
    #[display("false")]
    False,
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
pub enum IncludeSubcategories {
    #[serde(rename = "true")]
    #[display("true")]
    True,
    #[serde(rename = "false")]
    #[display("false")]
    False,
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
pub enum Order {
    #[serde(rename = "asc")]
    #[display("asc")]
    Asc,
    #[serde(rename = "desc")]
    #[display("desc")]
    Desc,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TagsDescriptions {}

impl std::fmt::Display for TagsDescriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TagsDescriptions {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Topic {
    pub id: i64,
    pub title: String,
    pub tags: Vec<String>,
    pub tags_descriptions: TagsDescriptions,
}

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Topic {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.title.clone().into(),
            format!("{:?}", self.tags).into(),
            format!("{:?}", self.tags_descriptions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "tags".into(),
            "tags_descriptions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Post {
    pub id: i64,
    pub post_number: i64,
    pub url: String,
    pub category_slug: String,
    pub topic: Topic,
}

impl std::fmt::Display for Post {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Post {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.post_number).into(),
            self.url.clone().into(),
            self.category_slug.clone().into(),
            format!("{:?}", self.topic).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "post_number".into(),
            "url".into(),
            "category_slug".into(),
            "topic".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Occurrences {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<String>,
}

impl std::fmt::Display for Occurrences {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Occurrences {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.starts_at).into(),
            format!("{:?}", self.ends_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["starts_at".into(), "ends_at".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Creator {
    pub id: i64,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
}

impl std::fmt::Display for Creator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Creator {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomFields {}

impl std::fmt::Display for CustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomFields {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
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
pub enum Period {
    #[serde(rename = "before")]
    #[display("before")]
    Before,
    #[serde(rename = "after")]
    #[display("after")]
    After,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Reminders {
    pub value: i64,
    pub unit: String,
    pub period: Period,
    #[serde(rename = "type")]
    pub type_: String,
}

impl std::fmt::Display for Reminders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Reminders {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.value).into(),
            self.unit.clone().into(),
            format!("{:?}", self.period).into(),
            self.type_.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "value".into(),
            "unit".into(),
            "period".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SampleInvitees {}

impl std::fmt::Display for SampleInvitees {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SampleInvitees {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Stats {}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Stats {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
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
    #[serde(rename = "public")]
    #[display("public")]
    Public,
    #[serde(rename = "private")]
    #[display("private")]
    Private,
    #[serde(rename = "standalone")]
    #[display("standalone")]
    Standalone,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WatchingInvitee {}

impl std::fmt::Display for WatchingInvitee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WatchingInvitee {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Channel {}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Channel {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Events {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurrence_until: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starts_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ends_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rrule: Option<String>,
    pub show_local_time: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    pub post: Post,
    pub occurrences: Vec<Occurrences>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_act_on_discourse_post_event: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_update_attendance: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<CustomFields>,
    pub is_closed: bool,
    pub is_expired: bool,
    pub is_ongoing: bool,
    pub is_private: bool,
    pub is_public: bool,
    pub is_standalone: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimal: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_invitees: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reminders: Option<Vec<Reminders>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_invitees: Option<Vec<SampleInvitees>>,
    pub should_display_invitees: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<Stats>,
    pub status: Status,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watching_invitee: Option<WatchingInvitee>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_attendees: Option<i64>,
    pub at_capacity: bool,
}

impl std::fmt::Display for Events {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Events {
    const LENGTH: usize = 38;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.category_id).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(recurrence) = &self.recurrence {
                format!("{:?}", recurrence).into()
            } else {
                String::new().into()
            },
            if let Some(recurrence_until) = &self.recurrence_until {
                format!("{:?}", recurrence_until).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.starts_at).into(),
            format!("{:?}", self.ends_at).into(),
            if let Some(rrule) = &self.rrule {
                format!("{:?}", rrule).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.show_local_time).into(),
            format!("{:?}", self.timezone).into(),
            if let Some(duration) = &self.duration {
                format!("{:?}", duration).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.post).into(),
            format!("{:?}", self.occurrences).into(),
            format!("{:?}", self.can_act_on_discourse_post_event).into(),
            format!("{:?}", self.can_update_attendance).into(),
            if let Some(creator) = &self.creator {
                format!("{:?}", creator).into()
            } else {
                String::new().into()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_closed).into(),
            format!("{:?}", self.is_expired).into(),
            format!("{:?}", self.is_ongoing).into(),
            format!("{:?}", self.is_private).into(),
            format!("{:?}", self.is_public).into(),
            format!("{:?}", self.is_standalone).into(),
            if let Some(minimal) = &self.minimal {
                format!("{:?}", minimal).into()
            } else {
                String::new().into()
            },
            if let Some(raw_invitees) = &self.raw_invitees {
                format!("{:?}", raw_invitees).into()
            } else {
                String::new().into()
            },
            if let Some(reminders) = &self.reminders {
                format!("{:?}", reminders).into()
            } else {
                String::new().into()
            },
            if let Some(sample_invitees) = &self.sample_invitees {
                format!("{:?}", sample_invitees).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.should_display_invitees).into(),
            if let Some(stats) = &self.stats {
                format!("{:?}", stats).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            if let Some(url) = &self.url {
                format!("{:?}", url).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(location) = &self.location {
                format!("{:?}", location).into()
            } else {
                String::new().into()
            },
            if let Some(watching_invitee) = &self.watching_invitee {
                format!("{:?}", watching_invitee).into()
            } else {
                String::new().into()
            },
            if let Some(chat_enabled) = &self.chat_enabled {
                format!("{:?}", chat_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(channel) = &self.channel {
                format!("{:?}", channel).into()
            } else {
                String::new().into()
            },
            if let Some(max_attendees) = &self.max_attendees {
                format!("{:?}", max_attendees).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.at_capacity).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "category_id".into(),
            "name".into(),
            "recurrence".into(),
            "recurrence_until".into(),
            "starts_at".into(),
            "ends_at".into(),
            "rrule".into(),
            "show_local_time".into(),
            "timezone".into(),
            "duration".into(),
            "post".into(),
            "occurrences".into(),
            "can_act_on_discourse_post_event".into(),
            "can_update_attendance".into(),
            "creator".into(),
            "custom_fields".into(),
            "is_closed".into(),
            "is_expired".into(),
            "is_ongoing".into(),
            "is_private".into(),
            "is_public".into(),
            "is_standalone".into(),
            "minimal".into(),
            "raw_invitees".into(),
            "reminders".into(),
            "sample_invitees".into(),
            "should_display_invitees".into(),
            "stats".into(),
            "status".into(),
            "url".into(),
            "description".into(),
            "location".into(),
            "watching_invitee".into(),
            "chat_enabled".into(),
            "channel".into(),
            "max_attendees".into(),
            "at_capacity".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListEventsResponse {
    pub events: Vec<Events>,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListEventsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.events).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["events".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetBackupsResponse {
    pub filename: String,
    pub size: i64,
    pub last_modified: String,
}

impl std::fmt::Display for GetBackupsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetBackupsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.filename.clone().into(),
            format!("{:?}", self.size).into(),
            self.last_modified.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["filename".into(), "size".into(), "last_modified".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateBackupRequestBody {
    pub with_uploads: bool,
}

impl std::fmt::Display for CreateBackupRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateBackupRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.with_uploads).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["with_uploads".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateBackupResponse {
    pub success: String,
}

impl std::fmt::Display for CreateBackupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateBackupResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Badges {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub grant_count: i64,
    pub allow_title: bool,
    pub multiple_grant: bool,
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub listable: bool,
    pub enabled: bool,
    pub badge_grouping_id: i64,
    pub system: bool,
    pub long_description: String,
    pub slug: String,
    pub manually_grantable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<i64>,
    pub target_posts: bool,
    pub auto_revoke: bool,
    pub show_posts: bool,
    #[serde(rename = "i18n_name", default, skip_serializing_if = "Option::is_none")]
    pub i_1_8n_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_upload_id: Option<i64>,
    pub badge_type_id: i64,
    pub show_in_post_header: bool,
}

impl std::fmt::Display for Badges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Badges {
    const LENGTH: usize = 24;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.description.clone().into(),
            format!("{:?}", self.grant_count).into(),
            format!("{:?}", self.allow_title).into(),
            format!("{:?}", self.multiple_grant).into(),
            self.icon.clone().into(),
            format!("{:?}", self.image_url).into(),
            format!("{:?}", self.listable).into(),
            format!("{:?}", self.enabled).into(),
            format!("{:?}", self.badge_grouping_id).into(),
            format!("{:?}", self.system).into(),
            self.long_description.clone().into(),
            self.slug.clone().into(),
            format!("{:?}", self.manually_grantable).into(),
            format!("{:?}", self.query).into(),
            format!("{:?}", self.trigger).into(),
            format!("{:?}", self.target_posts).into(),
            format!("{:?}", self.auto_revoke).into(),
            format!("{:?}", self.show_posts).into(),
            if let Some(i_1_8n_name) = &self.i_1_8n_name {
                format!("{:?}", i_1_8n_name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.image_upload_id).into(),
            format!("{:?}", self.badge_type_id).into(),
            format!("{:?}", self.show_in_post_header).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "description".into(),
            "grant_count".into(),
            "allow_title".into(),
            "multiple_grant".into(),
            "icon".into(),
            "image_url".into(),
            "listable".into(),
            "enabled".into(),
            "badge_grouping_id".into(),
            "system".into(),
            "long_description".into(),
            "slug".into(),
            "manually_grantable".into(),
            "query".into(),
            "trigger".into(),
            "target_posts".into(),
            "auto_revoke".into(),
            "show_posts".into(),
            "i_1_8n_name".into(),
            "image_upload_id".into(),
            "badge_type_id".into(),
            "show_in_post_header".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BadgeTypes {
    pub id: i64,
    pub name: String,
    pub sort_order: i64,
}

impl std::fmt::Display for BadgeTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BadgeTypes {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.sort_order).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into(), "sort_order".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BadgeGroupings {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub position: i64,
    pub system: bool,
}

impl std::fmt::Display for BadgeGroupings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BadgeGroupings {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.description).into(),
            format!("{:?}", self.position).into(),
            format!("{:?}", self.system).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "description".into(),
            "position".into(),
            "system".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Triggers {
    pub user_change: i64,
    pub none: i64,
    pub post_revision: i64,
    pub trust_level_change: i64,
    pub post_action: i64,
}

impl std::fmt::Display for Triggers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Triggers {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user_change).into(),
            format!("{:?}", self.none).into(),
            format!("{:?}", self.post_revision).into(),
            format!("{:?}", self.trust_level_change).into(),
            format!("{:?}", self.post_action).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "user_change".into(),
            "none".into(),
            "post_revision".into(),
            "trust_level_change".into(),
            "post_action".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AdminBadges {
    pub protected_system_fields: Vec<serde_json::Value>,
    pub triggers: Triggers,
    pub badge_ids: Vec<serde_json::Value>,
    pub badge_grouping_ids: Vec<serde_json::Value>,
    pub badge_type_ids: Vec<serde_json::Value>,
}

impl std::fmt::Display for AdminBadges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AdminBadges {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.protected_system_fields).into(),
            format!("{:?}", self.triggers).into(),
            format!("{:?}", self.badge_ids).into(),
            format!("{:?}", self.badge_grouping_ids).into(),
            format!("{:?}", self.badge_type_ids).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "protected_system_fields".into(),
            "triggers".into(),
            "badge_ids".into(),
            "badge_grouping_ids".into(),
            "badge_type_ids".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AdminListBadgesResponse {
    pub badges: Vec<Badges>,
    pub badge_types: Vec<BadgeTypes>,
    pub badge_groupings: Vec<BadgeGroupings>,
    pub admin_badges: AdminBadges,
}

impl std::fmt::Display for AdminListBadgesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AdminListBadgesResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.badges).into(),
            format!("{:?}", self.badge_types).into(),
            format!("{:?}", self.badge_groupings).into(),
            format!("{:?}", self.admin_badges).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "badges".into(),
            "badge_types".into(),
            "badge_groupings".into(),
            "admin_badges".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateBadgeRequestBody {
    #[doc = "The name for the new badge."]
    pub name: String,
    #[doc = "The ID for the badge type. 1 for Gold, 2 for Silver,\n3 for Bronze."]
    pub badge_type_id: i64,
}

impl std::fmt::Display for CreateBadgeRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateBadgeRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.badge_type_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "badge_type_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Badge {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub grant_count: i64,
    pub allow_title: bool,
    pub multiple_grant: bool,
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_upload_id: Option<i64>,
    pub listable: bool,
    pub enabled: bool,
    pub badge_grouping_id: i64,
    pub system: bool,
    pub long_description: String,
    pub slug: String,
    pub manually_grantable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<String>,
    pub target_posts: bool,
    pub auto_revoke: bool,
    pub show_posts: bool,
    pub badge_type_id: i64,
    pub show_in_post_header: bool,
}

impl std::fmt::Display for Badge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Badge {
    const LENGTH: usize = 23;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.description.clone().into(),
            format!("{:?}", self.grant_count).into(),
            format!("{:?}", self.allow_title).into(),
            format!("{:?}", self.multiple_grant).into(),
            self.icon.clone().into(),
            format!("{:?}", self.image_url).into(),
            format!("{:?}", self.image_upload_id).into(),
            format!("{:?}", self.listable).into(),
            format!("{:?}", self.enabled).into(),
            format!("{:?}", self.badge_grouping_id).into(),
            format!("{:?}", self.system).into(),
            self.long_description.clone().into(),
            self.slug.clone().into(),
            format!("{:?}", self.manually_grantable).into(),
            format!("{:?}", self.query).into(),
            format!("{:?}", self.trigger).into(),
            format!("{:?}", self.target_posts).into(),
            format!("{:?}", self.auto_revoke).into(),
            format!("{:?}", self.show_posts).into(),
            format!("{:?}", self.badge_type_id).into(),
            format!("{:?}", self.show_in_post_header).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "description".into(),
            "grant_count".into(),
            "allow_title".into(),
            "multiple_grant".into(),
            "icon".into(),
            "image_url".into(),
            "image_upload_id".into(),
            "listable".into(),
            "enabled".into(),
            "badge_grouping_id".into(),
            "system".into(),
            "long_description".into(),
            "slug".into(),
            "manually_grantable".into(),
            "query".into(),
            "trigger".into(),
            "target_posts".into(),
            "auto_revoke".into(),
            "show_posts".into(),
            "badge_type_id".into(),
            "show_in_post_header".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateBadgeResponse {
    pub badge_types: Vec<BadgeTypes>,
    pub badge: Badge,
}

impl std::fmt::Display for CreateBadgeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateBadgeResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.badge_types).into(),
            format!("{:?}", self.badge).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["badge_types".into(), "badge".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateBadgeRequestBody {
    #[doc = "The name for the new badge."]
    pub name: String,
    #[doc = "The ID for the badge type. 1 for Gold, 2 for Silver,\n3 for Bronze."]
    pub badge_type_id: i64,
}

impl std::fmt::Display for UpdateBadgeRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateBadgeRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.badge_type_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "badge_type_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateBadgeResponse {
    pub badge_types: Vec<BadgeTypes>,
    pub badge: Badge,
}

impl std::fmt::Display for UpdateBadgeResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateBadgeResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.badge_types).into(),
            format!("{:?}", self.badge).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["badge_types".into(), "badge".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Categories {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub text_color: String,
    pub style_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub slug: String,
    pub topic_count: i64,
    pub post_count: i64,
    pub position: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_excerpt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_url: Option<String>,
    pub read_restricted: bool,
    pub permission: i64,
    pub notification_level: i64,
    pub can_edit: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_template: Option<String>,
    pub has_children: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcategory_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_ascending: Option<String>,
    pub show_subcategory_list: bool,
    pub num_featured_topics: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    pub subcategory_list_style: String,
    pub default_top_period: String,
    pub default_list_filter: String,
    pub minimum_required_tags: i64,
    pub navigate_to_first_post_after_read: bool,
    pub topics_day: i64,
    pub topics_week: i64,
    pub topics_month: i64,
    pub topics_year: i64,
    pub topics_all_time: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_uncategorized: Option<bool>,
    pub subcategory_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcategory_list: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo_dark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background_dark: Option<String>,
}

impl std::fmt::Display for Categories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Categories {
    const LENGTH: usize = 44;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.color.clone().into(),
            self.text_color.clone().into(),
            self.style_type.clone().into(),
            format!("{:?}", self.emoji).into(),
            format!("{:?}", self.icon).into(),
            self.slug.clone().into(),
            format!("{:?}", self.topic_count).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.position).into(),
            format!("{:?}", self.description).into(),
            format!("{:?}", self.description_text).into(),
            format!("{:?}", self.description_excerpt).into(),
            format!("{:?}", self.topic_url).into(),
            format!("{:?}", self.read_restricted).into(),
            format!("{:?}", self.permission).into(),
            format!("{:?}", self.notification_level).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.topic_template).into(),
            format!("{:?}", self.has_children).into(),
            format!("{:?}", self.subcategory_count).into(),
            format!("{:?}", self.sort_order).into(),
            format!("{:?}", self.sort_ascending).into(),
            format!("{:?}", self.show_subcategory_list).into(),
            format!("{:?}", self.num_featured_topics).into(),
            format!("{:?}", self.default_view).into(),
            self.subcategory_list_style.clone().into(),
            self.default_top_period.clone().into(),
            self.default_list_filter.clone().into(),
            format!("{:?}", self.minimum_required_tags).into(),
            format!("{:?}", self.navigate_to_first_post_after_read).into(),
            format!("{:?}", self.topics_day).into(),
            format!("{:?}", self.topics_week).into(),
            format!("{:?}", self.topics_month).into(),
            format!("{:?}", self.topics_year).into(),
            format!("{:?}", self.topics_all_time).into(),
            if let Some(is_uncategorized) = &self.is_uncategorized {
                format!("{:?}", is_uncategorized).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.subcategory_ids).into(),
            if let Some(subcategory_list) = &self.subcategory_list {
                format!("{:?}", subcategory_list).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.uploaded_logo).into(),
            format!("{:?}", self.uploaded_logo_dark).into(),
            format!("{:?}", self.uploaded_background).into(),
            format!("{:?}", self.uploaded_background_dark).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "color".into(),
            "text_color".into(),
            "style_type".into(),
            "emoji".into(),
            "icon".into(),
            "slug".into(),
            "topic_count".into(),
            "post_count".into(),
            "position".into(),
            "description".into(),
            "description_text".into(),
            "description_excerpt".into(),
            "topic_url".into(),
            "read_restricted".into(),
            "permission".into(),
            "notification_level".into(),
            "can_edit".into(),
            "topic_template".into(),
            "has_children".into(),
            "subcategory_count".into(),
            "sort_order".into(),
            "sort_ascending".into(),
            "show_subcategory_list".into(),
            "num_featured_topics".into(),
            "default_view".into(),
            "subcategory_list_style".into(),
            "default_top_period".into(),
            "default_list_filter".into(),
            "minimum_required_tags".into(),
            "navigate_to_first_post_after_read".into(),
            "topics_day".into(),
            "topics_week".into(),
            "topics_month".into(),
            "topics_year".into(),
            "topics_all_time".into(),
            "is_uncategorized".into(),
            "subcategory_ids".into(),
            "subcategory_list".into(),
            "uploaded_logo".into(),
            "uploaded_logo_dark".into(),
            "uploaded_background".into(),
            "uploaded_background_dark".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CategoryList {
    pub can_create_category: bool,
    pub can_create_topic: bool,
    pub categories: Vec<Categories>,
}

impl std::fmt::Display for CategoryList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CategoryList {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.can_create_category).into(),
            format!("{:?}", self.can_create_topic).into(),
            format!("{:?}", self.categories).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_create_category".into(),
            "can_create_topic".into(),
            "categories".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCategoriesResponse {
    pub category_list: CategoryList,
}

impl std::fmt::Display for ListCategoriesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCategoriesResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.category_list).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["category_list".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Permissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everyone: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staff: Option<i64>,
}

impl std::fmt::Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Permissions {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(everyone) = &self.everyone {
                format!("{:?}", everyone).into()
            } else {
                String::new().into()
            },
            if let Some(staff) = &self.staff {
                format!("{:?}", staff).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["everyone".into(), "staff".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CategoryLocalizations {
    #[doc = "The unique identifier for an existing localization.\nMust be included otherwise the \
             record will be deleted."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The locale for the localization, e.g., 'en',\n'zh_CN'. Locale should be in the list \
             of SiteSetting.content_localization_supported_locales."]
    pub locale: String,
    #[doc = "The name of the category in the specified locale."]
    pub name: String,
    #[doc = "The description excerpt of the category in the\nspecified locale."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl std::fmt::Display for CategoryLocalizations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CategoryLocalizations {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            self.locale.clone().into(),
            self.name.clone().into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "locale".into(),
            "name".into(),
            "description".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCategoryRequestBody {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_badges: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_featured_links_allowed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_template_ids: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_localizations: Option<Vec<CategoryLocalizations>>,
}

impl std::fmt::Display for CreateCategoryRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateCategoryRequestBody {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(color) = &self.color {
                format!("{:?}", color).into()
            } else {
                String::new().into()
            },
            if let Some(text_color) = &self.text_color {
                format!("{:?}", text_color).into()
            } else {
                String::new().into()
            },
            if let Some(style_type) = &self.style_type {
                format!("{:?}", style_type).into()
            } else {
                String::new().into()
            },
            if let Some(emoji) = &self.emoji {
                format!("{:?}", emoji).into()
            } else {
                String::new().into()
            },
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
            if let Some(parent_category_id) = &self.parent_category_id {
                format!("{:?}", parent_category_id).into()
            } else {
                String::new().into()
            },
            if let Some(allow_badges) = &self.allow_badges {
                format!("{:?}", allow_badges).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(topic_featured_links_allowed) = &self.topic_featured_links_allowed {
                format!("{:?}", topic_featured_links_allowed).into()
            } else {
                String::new().into()
            },
            if let Some(permissions) = &self.permissions {
                format!("{:?}", permissions).into()
            } else {
                String::new().into()
            },
            if let Some(search_priority) = &self.search_priority {
                format!("{:?}", search_priority).into()
            } else {
                String::new().into()
            },
            if let Some(form_template_ids) = &self.form_template_ids {
                format!("{:?}", form_template_ids).into()
            } else {
                String::new().into()
            },
            if let Some(category_localizations) = &self.category_localizations {
                format!("{:?}", category_localizations).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "color".into(),
            "text_color".into(),
            "style_type".into(),
            "emoji".into(),
            "icon".into(),
            "parent_category_id".into(),
            "allow_badges".into(),
            "slug".into(),
            "topic_featured_links_allowed".into(),
            "permissions".into(),
            "search_priority".into(),
            "form_template_ids".into(),
            "category_localizations".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CategoryCustomFields {}

impl std::fmt::Display for CategoryCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CategoryCustomFields {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RequiredTagGroups {
    pub name: String,
    pub min_count: i64,
}

impl std::fmt::Display for RequiredTagGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RequiredTagGroups {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.min_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "min_count".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CategorySetting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_bump_cooldown_days: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_auto_bump_daily: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_reply_approval: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_topic_approval: Option<bool>,
}

impl std::fmt::Display for CategorySetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CategorySetting {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(auto_bump_cooldown_days) = &self.auto_bump_cooldown_days {
                format!("{:?}", auto_bump_cooldown_days).into()
            } else {
                String::new().into()
            },
            if let Some(num_auto_bump_daily) = &self.num_auto_bump_daily {
                format!("{:?}", num_auto_bump_daily).into()
            } else {
                String::new().into()
            },
            if let Some(require_reply_approval) = &self.require_reply_approval {
                format!("{:?}", require_reply_approval).into()
            } else {
                String::new().into()
            },
            if let Some(require_topic_approval) = &self.require_topic_approval {
                format!("{:?}", require_topic_approval).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "auto_bump_cooldown_days".into(),
            "num_auto_bump_daily".into(),
            "require_reply_approval".into(),
            "require_topic_approval".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GroupPermissions {
    pub permission_type: i64,
    pub group_name: String,
    pub group_id: i64,
}

impl std::fmt::Display for GroupPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GroupPermissions {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.permission_type).into(),
            self.group_name.clone().into(),
            format!("{:?}", self.group_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "permission_type".into(),
            "group_name".into(),
            "group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub text_color: String,
    pub style_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub slug: String,
    pub topic_count: i64,
    pub post_count: i64,
    pub position: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_excerpt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_url: Option<String>,
    pub read_restricted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i64>,
    pub notification_level: i64,
    pub can_edit: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_template_ids: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_children: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcategory_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_ascending: Option<String>,
    pub show_subcategory_list: bool,
    pub num_featured_topics: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    pub subcategory_list_style: String,
    pub default_top_period: String,
    pub default_list_filter: String,
    pub minimum_required_tags: i64,
    pub navigate_to_first_post_after_read: bool,
    pub custom_fields: CategoryCustomFields,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tag_groups: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_global_tags: Option<bool>,
    pub required_tag_groups: Vec<RequiredTagGroups>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_setting: Option<CategorySetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_localizations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_banner: Option<String>,
    pub available_groups: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_close_hours: Option<String>,
    pub auto_close_based_on_last_post: bool,
    pub allow_unlimited_owner_edits_on_first_post: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_slow_mode_seconds: Option<String>,
    pub group_permissions: Vec<GroupPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_in: Option<String>,
    pub email_in_allow_strangers: bool,
    pub mailinglist_mirror: bool,
    pub all_topics_wiki: bool,
    pub can_delete: bool,
    pub allow_badges: bool,
    pub topic_featured_link_allowed: bool,
    pub search_priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo_dark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background_dark: Option<String>,
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Category {
    const LENGTH: usize = 59;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.color.clone().into(),
            self.text_color.clone().into(),
            self.style_type.clone().into(),
            format!("{:?}", self.emoji).into(),
            format!("{:?}", self.icon).into(),
            self.slug.clone().into(),
            format!("{:?}", self.topic_count).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.position).into(),
            format!("{:?}", self.description).into(),
            format!("{:?}", self.description_text).into(),
            format!("{:?}", self.description_excerpt).into(),
            format!("{:?}", self.topic_url).into(),
            format!("{:?}", self.read_restricted).into(),
            format!("{:?}", self.permission).into(),
            format!("{:?}", self.notification_level).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.topic_template).into(),
            if let Some(form_template_ids) = &self.form_template_ids {
                format!("{:?}", form_template_ids).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.has_children).into(),
            format!("{:?}", self.subcategory_count).into(),
            format!("{:?}", self.sort_order).into(),
            format!("{:?}", self.sort_ascending).into(),
            format!("{:?}", self.show_subcategory_list).into(),
            format!("{:?}", self.num_featured_topics).into(),
            format!("{:?}", self.default_view).into(),
            self.subcategory_list_style.clone().into(),
            self.default_top_period.clone().into(),
            self.default_list_filter.clone().into(),
            format!("{:?}", self.minimum_required_tags).into(),
            format!("{:?}", self.navigate_to_first_post_after_read).into(),
            format!("{:?}", self.custom_fields).into(),
            if let Some(allowed_tags) = &self.allowed_tags {
                format!("{:?}", allowed_tags).into()
            } else {
                String::new().into()
            },
            if let Some(allowed_tag_groups) = &self.allowed_tag_groups {
                format!("{:?}", allowed_tag_groups).into()
            } else {
                String::new().into()
            },
            if let Some(allow_global_tags) = &self.allow_global_tags {
                format!("{:?}", allow_global_tags).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.required_tag_groups).into(),
            if let Some(category_setting) = &self.category_setting {
                format!("{:?}", category_setting).into()
            } else {
                String::new().into()
            },
            if let Some(category_localizations) = &self.category_localizations {
                format!("{:?}", category_localizations).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.read_only_banner).into(),
            format!("{:?}", self.available_groups).into(),
            format!("{:?}", self.auto_close_hours).into(),
            format!("{:?}", self.auto_close_based_on_last_post).into(),
            format!("{:?}", self.allow_unlimited_owner_edits_on_first_post).into(),
            format!("{:?}", self.default_slow_mode_seconds).into(),
            format!("{:?}", self.group_permissions).into(),
            format!("{:?}", self.email_in).into(),
            format!("{:?}", self.email_in_allow_strangers).into(),
            format!("{:?}", self.mailinglist_mirror).into(),
            format!("{:?}", self.all_topics_wiki).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.allow_badges).into(),
            format!("{:?}", self.topic_featured_link_allowed).into(),
            format!("{:?}", self.search_priority).into(),
            format!("{:?}", self.uploaded_logo).into(),
            format!("{:?}", self.uploaded_logo_dark).into(),
            format!("{:?}", self.uploaded_background).into(),
            format!("{:?}", self.uploaded_background_dark).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "color".into(),
            "text_color".into(),
            "style_type".into(),
            "emoji".into(),
            "icon".into(),
            "slug".into(),
            "topic_count".into(),
            "post_count".into(),
            "position".into(),
            "description".into(),
            "description_text".into(),
            "description_excerpt".into(),
            "topic_url".into(),
            "read_restricted".into(),
            "permission".into(),
            "notification_level".into(),
            "can_edit".into(),
            "topic_template".into(),
            "form_template_ids".into(),
            "has_children".into(),
            "subcategory_count".into(),
            "sort_order".into(),
            "sort_ascending".into(),
            "show_subcategory_list".into(),
            "num_featured_topics".into(),
            "default_view".into(),
            "subcategory_list_style".into(),
            "default_top_period".into(),
            "default_list_filter".into(),
            "minimum_required_tags".into(),
            "navigate_to_first_post_after_read".into(),
            "custom_fields".into(),
            "allowed_tags".into(),
            "allowed_tag_groups".into(),
            "allow_global_tags".into(),
            "required_tag_groups".into(),
            "category_setting".into(),
            "category_localizations".into(),
            "read_only_banner".into(),
            "available_groups".into(),
            "auto_close_hours".into(),
            "auto_close_based_on_last_post".into(),
            "allow_unlimited_owner_edits_on_first_post".into(),
            "default_slow_mode_seconds".into(),
            "group_permissions".into(),
            "email_in".into(),
            "email_in_allow_strangers".into(),
            "mailinglist_mirror".into(),
            "all_topics_wiki".into(),
            "can_delete".into(),
            "allow_badges".into(),
            "topic_featured_link_allowed".into(),
            "search_priority".into(),
            "uploaded_logo".into(),
            "uploaded_logo_dark".into(),
            "uploaded_background".into(),
            "uploaded_background_dark".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCategoryResponse {
    pub category: Category,
}

impl std::fmt::Display for CreateCategoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateCategoryResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.category).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["category".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCategoryRequestBody {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_badges: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_featured_links_allowed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_template_ids: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_localizations: Option<Vec<CategoryLocalizations>>,
}

impl std::fmt::Display for UpdateCategoryRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCategoryRequestBody {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(color) = &self.color {
                format!("{:?}", color).into()
            } else {
                String::new().into()
            },
            if let Some(text_color) = &self.text_color {
                format!("{:?}", text_color).into()
            } else {
                String::new().into()
            },
            if let Some(style_type) = &self.style_type {
                format!("{:?}", style_type).into()
            } else {
                String::new().into()
            },
            if let Some(emoji) = &self.emoji {
                format!("{:?}", emoji).into()
            } else {
                String::new().into()
            },
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
            if let Some(parent_category_id) = &self.parent_category_id {
                format!("{:?}", parent_category_id).into()
            } else {
                String::new().into()
            },
            if let Some(allow_badges) = &self.allow_badges {
                format!("{:?}", allow_badges).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(topic_featured_links_allowed) = &self.topic_featured_links_allowed {
                format!("{:?}", topic_featured_links_allowed).into()
            } else {
                String::new().into()
            },
            if let Some(permissions) = &self.permissions {
                format!("{:?}", permissions).into()
            } else {
                String::new().into()
            },
            if let Some(search_priority) = &self.search_priority {
                format!("{:?}", search_priority).into()
            } else {
                String::new().into()
            },
            if let Some(form_template_ids) = &self.form_template_ids {
                format!("{:?}", form_template_ids).into()
            } else {
                String::new().into()
            },
            if let Some(category_localizations) = &self.category_localizations {
                format!("{:?}", category_localizations).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "color".into(),
            "text_color".into(),
            "style_type".into(),
            "emoji".into(),
            "icon".into(),
            "parent_category_id".into(),
            "allow_badges".into(),
            "slug".into(),
            "topic_featured_links_allowed".into(),
            "permissions".into(),
            "search_priority".into(),
            "form_template_ids".into(),
            "category_localizations".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCategoryResponseCategoryCustomFields {}

impl std::fmt::Display for UpdateCategoryResponseCategoryCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCategoryResponseCategoryCustomFields {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCategoryResponseCategory {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub text_color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub slug: String,
    pub topic_count: i64,
    pub post_count: i64,
    pub position: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_excerpt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_url: Option<String>,
    pub read_restricted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i64>,
    pub notification_level: i64,
    pub can_edit: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_template: Option<String>,
    pub form_template_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_children: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcategory_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_ascending: Option<String>,
    pub show_subcategory_list: bool,
    pub num_featured_topics: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    pub subcategory_list_style: String,
    pub default_top_period: String,
    pub default_list_filter: String,
    pub minimum_required_tags: i64,
    pub navigate_to_first_post_after_read: bool,
    pub custom_fields: UpdateCategoryResponseCategoryCustomFields,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_tag_groups: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_global_tags: Option<bool>,
    pub required_tag_groups: Vec<RequiredTagGroups>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_setting: Option<CategorySetting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_localizations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_banner: Option<String>,
    pub available_groups: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_close_hours: Option<String>,
    pub auto_close_based_on_last_post: bool,
    pub allow_unlimited_owner_edits_on_first_post: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_slow_mode_seconds: Option<String>,
    pub group_permissions: Vec<GroupPermissions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_in: Option<String>,
    pub email_in_allow_strangers: bool,
    pub mailinglist_mirror: bool,
    pub all_topics_wiki: bool,
    pub can_delete: bool,
    pub allow_badges: bool,
    pub topic_featured_link_allowed: bool,
    pub search_priority: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo_dark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background_dark: Option<String>,
}

impl std::fmt::Display for UpdateCategoryResponseCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCategoryResponseCategory {
    const LENGTH: usize = 59;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.color.clone().into(),
            self.text_color.clone().into(),
            if let Some(style_type) = &self.style_type {
                format!("{:?}", style_type).into()
            } else {
                String::new().into()
            },
            if let Some(emoji) = &self.emoji {
                format!("{:?}", emoji).into()
            } else {
                String::new().into()
            },
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
            self.slug.clone().into(),
            format!("{:?}", self.topic_count).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.position).into(),
            format!("{:?}", self.description).into(),
            format!("{:?}", self.description_text).into(),
            format!("{:?}", self.description_excerpt).into(),
            format!("{:?}", self.topic_url).into(),
            format!("{:?}", self.read_restricted).into(),
            format!("{:?}", self.permission).into(),
            format!("{:?}", self.notification_level).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.topic_template).into(),
            format!("{:?}", self.form_template_ids).into(),
            format!("{:?}", self.has_children).into(),
            format!("{:?}", self.subcategory_count).into(),
            format!("{:?}", self.sort_order).into(),
            format!("{:?}", self.sort_ascending).into(),
            format!("{:?}", self.show_subcategory_list).into(),
            format!("{:?}", self.num_featured_topics).into(),
            format!("{:?}", self.default_view).into(),
            self.subcategory_list_style.clone().into(),
            self.default_top_period.clone().into(),
            self.default_list_filter.clone().into(),
            format!("{:?}", self.minimum_required_tags).into(),
            format!("{:?}", self.navigate_to_first_post_after_read).into(),
            format!("{:?}", self.custom_fields).into(),
            if let Some(allowed_tags) = &self.allowed_tags {
                format!("{:?}", allowed_tags).into()
            } else {
                String::new().into()
            },
            if let Some(allowed_tag_groups) = &self.allowed_tag_groups {
                format!("{:?}", allowed_tag_groups).into()
            } else {
                String::new().into()
            },
            if let Some(allow_global_tags) = &self.allow_global_tags {
                format!("{:?}", allow_global_tags).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.required_tag_groups).into(),
            if let Some(category_setting) = &self.category_setting {
                format!("{:?}", category_setting).into()
            } else {
                String::new().into()
            },
            if let Some(category_localizations) = &self.category_localizations {
                format!("{:?}", category_localizations).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.read_only_banner).into(),
            format!("{:?}", self.available_groups).into(),
            format!("{:?}", self.auto_close_hours).into(),
            format!("{:?}", self.auto_close_based_on_last_post).into(),
            format!("{:?}", self.allow_unlimited_owner_edits_on_first_post).into(),
            format!("{:?}", self.default_slow_mode_seconds).into(),
            format!("{:?}", self.group_permissions).into(),
            format!("{:?}", self.email_in).into(),
            format!("{:?}", self.email_in_allow_strangers).into(),
            format!("{:?}", self.mailinglist_mirror).into(),
            format!("{:?}", self.all_topics_wiki).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.allow_badges).into(),
            format!("{:?}", self.topic_featured_link_allowed).into(),
            format!("{:?}", self.search_priority).into(),
            format!("{:?}", self.uploaded_logo).into(),
            format!("{:?}", self.uploaded_logo_dark).into(),
            format!("{:?}", self.uploaded_background).into(),
            format!("{:?}", self.uploaded_background_dark).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "color".into(),
            "text_color".into(),
            "style_type".into(),
            "emoji".into(),
            "icon".into(),
            "slug".into(),
            "topic_count".into(),
            "post_count".into(),
            "position".into(),
            "description".into(),
            "description_text".into(),
            "description_excerpt".into(),
            "topic_url".into(),
            "read_restricted".into(),
            "permission".into(),
            "notification_level".into(),
            "can_edit".into(),
            "topic_template".into(),
            "form_template_ids".into(),
            "has_children".into(),
            "subcategory_count".into(),
            "sort_order".into(),
            "sort_ascending".into(),
            "show_subcategory_list".into(),
            "num_featured_topics".into(),
            "default_view".into(),
            "subcategory_list_style".into(),
            "default_top_period".into(),
            "default_list_filter".into(),
            "minimum_required_tags".into(),
            "navigate_to_first_post_after_read".into(),
            "custom_fields".into(),
            "allowed_tags".into(),
            "allowed_tag_groups".into(),
            "allow_global_tags".into(),
            "required_tag_groups".into(),
            "category_setting".into(),
            "category_localizations".into(),
            "read_only_banner".into(),
            "available_groups".into(),
            "auto_close_hours".into(),
            "auto_close_based_on_last_post".into(),
            "allow_unlimited_owner_edits_on_first_post".into(),
            "default_slow_mode_seconds".into(),
            "group_permissions".into(),
            "email_in".into(),
            "email_in_allow_strangers".into(),
            "mailinglist_mirror".into(),
            "all_topics_wiki".into(),
            "can_delete".into(),
            "allow_badges".into(),
            "topic_featured_link_allowed".into(),
            "search_priority".into(),
            "uploaded_logo".into(),
            "uploaded_logo_dark".into(),
            "uploaded_background".into(),
            "uploaded_background_dark".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCategoryResponse {
    pub success: String,
    pub category: UpdateCategoryResponseCategory,
}

impl std::fmt::Display for UpdateCategoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCategoryResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.success.clone().into(),
            format!("{:?}", self.category).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into(), "category".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Users {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
}

impl std::fmt::Display for Users {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Users {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Posters {
    pub extras: String,
    pub description: String,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
}

impl std::fmt::Display for Posters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Posters {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.extras.clone().into(),
            self.description.clone().into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.primary_group_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "extras".into(),
            "description".into(),
            "user_id".into(),
            "primary_group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Topics {
    pub id: i64,
    pub title: String,
    pub fancy_title: String,
    pub slug: String,
    pub posts_count: i64,
    pub reply_count: i64,
    pub highest_post_number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub created_at: String,
    pub last_posted_at: String,
    pub bumped: bool,
    pub bumped_at: String,
    pub archetype: String,
    pub unseen: bool,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<String>,
    pub excerpt: String,
    pub visible: bool,
    pub closed: bool,
    pub archived: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liked: Option<String>,
    pub views: i64,
    pub like_count: i64,
    pub has_summary: bool,
    pub last_poster_username: String,
    pub category_id: i64,
    pub pinned_globally: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    pub posters: Vec<Posters>,
}

impl std::fmt::Display for Topics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Topics {
    const LENGTH: usize = 30;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.title.clone().into(),
            self.fancy_title.clone().into(),
            self.slug.clone().into(),
            format!("{:?}", self.posts_count).into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.highest_post_number).into(),
            format!("{:?}", self.image_url).into(),
            self.created_at.clone().into(),
            self.last_posted_at.clone().into(),
            format!("{:?}", self.bumped).into(),
            self.bumped_at.clone().into(),
            self.archetype.clone().into(),
            format!("{:?}", self.unseen).into(),
            format!("{:?}", self.pinned).into(),
            format!("{:?}", self.unpinned).into(),
            self.excerpt.clone().into(),
            format!("{:?}", self.visible).into(),
            format!("{:?}", self.closed).into(),
            format!("{:?}", self.archived).into(),
            format!("{:?}", self.bookmarked).into(),
            format!("{:?}", self.liked).into(),
            format!("{:?}", self.views).into(),
            format!("{:?}", self.like_count).into(),
            format!("{:?}", self.has_summary).into(),
            self.last_poster_username.clone().into(),
            format!("{:?}", self.category_id).into(),
            format!("{:?}", self.pinned_globally).into(),
            format!("{:?}", self.featured_link).into(),
            format!("{:?}", self.posters).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
            "reply_count".into(),
            "highest_post_number".into(),
            "image_url".into(),
            "created_at".into(),
            "last_posted_at".into(),
            "bumped".into(),
            "bumped_at".into(),
            "archetype".into(),
            "unseen".into(),
            "pinned".into(),
            "unpinned".into(),
            "excerpt".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "bookmarked".into(),
            "liked".into(),
            "views".into(),
            "like_count".into(),
            "has_summary".into(),
            "last_poster_username".into(),
            "category_id".into(),
            "pinned_globally".into(),
            "featured_link".into(),
            "posters".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TopicList {
    pub can_create_topic: bool,
    pub per_page: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub top_tags: Option<Vec<serde_json::Value>>,
    pub topics: Vec<Topics>,
}

impl std::fmt::Display for TopicList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TopicList {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.can_create_topic).into(),
            format!("{:?}", self.per_page).into(),
            if let Some(top_tags) = &self.top_tags {
                format!("{:?}", top_tags).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.topics).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_create_topic".into(),
            "per_page".into(),
            "top_tags".into(),
            "topics".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCategoryTopicsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<Users>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_groups: Option<Vec<serde_json::Value>>,
    pub topic_list: TopicList,
}

impl std::fmt::Display for ListCategoryTopicsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCategoryTopicsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
            if let Some(primary_groups) = &self.primary_groups {
                format!("{:?}", primary_groups).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.topic_list).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "primary_groups".into(), "topic_list".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCategoryResponse {
    pub category: Category,
}

impl std::fmt::Display for GetCategoryResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCategoryResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.category).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["category".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Group {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[doc = "About Group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[doc = "comma,separated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usernames: Option<String>,
    #[doc = "comma,separated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_usernames: Option<String>,
    #[doc = "pipe|separated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic_membership_email_domains: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visibility_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_icon: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_upload_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_admission: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_exit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_notification_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted_category_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_category_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watching_category_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_category_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watching_first_post_category_ids: Option<Vec<i64>>,
}

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Group {
    const LENGTH: usize = 19;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(full_name) = &self.full_name {
                format!("{:?}", full_name).into()
            } else {
                String::new().into()
            },
            if let Some(bio_raw) = &self.bio_raw {
                format!("{:?}", bio_raw).into()
            } else {
                String::new().into()
            },
            if let Some(usernames) = &self.usernames {
                format!("{:?}", usernames).into()
            } else {
                String::new().into()
            },
            if let Some(owner_usernames) = &self.owner_usernames {
                format!("{:?}", owner_usernames).into()
            } else {
                String::new().into()
            },
            if let Some(automatic_membership_email_domains) =
                &self.automatic_membership_email_domains
            {
                format!("{:?}", automatic_membership_email_domains).into()
            } else {
                String::new().into()
            },
            if let Some(visibility_level) = &self.visibility_level {
                format!("{:?}", visibility_level).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group) = &self.primary_group {
                format!("{:?}", primary_group).into()
            } else {
                String::new().into()
            },
            if let Some(flair_icon) = &self.flair_icon {
                format!("{:?}", flair_icon).into()
            } else {
                String::new().into()
            },
            if let Some(flair_upload_id) = &self.flair_upload_id {
                format!("{:?}", flair_upload_id).into()
            } else {
                String::new().into()
            },
            if let Some(flair_bg_color) = &self.flair_bg_color {
                format!("{:?}", flair_bg_color).into()
            } else {
                String::new().into()
            },
            if let Some(public_admission) = &self.public_admission {
                format!("{:?}", public_admission).into()
            } else {
                String::new().into()
            },
            if let Some(public_exit) = &self.public_exit {
                format!("{:?}", public_exit).into()
            } else {
                String::new().into()
            },
            if let Some(default_notification_level) = &self.default_notification_level {
                format!("{:?}", default_notification_level).into()
            } else {
                String::new().into()
            },
            if let Some(muted_category_ids) = &self.muted_category_ids {
                format!("{:?}", muted_category_ids).into()
            } else {
                String::new().into()
            },
            if let Some(regular_category_ids) = &self.regular_category_ids {
                format!("{:?}", regular_category_ids).into()
            } else {
                String::new().into()
            },
            if let Some(watching_category_ids) = &self.watching_category_ids {
                format!("{:?}", watching_category_ids).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_category_ids) = &self.tracking_category_ids {
                format!("{:?}", tracking_category_ids).into()
            } else {
                String::new().into()
            },
            if let Some(watching_first_post_category_ids) = &self.watching_first_post_category_ids {
                format!("{:?}", watching_first_post_category_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "full_name".into(),
            "bio_raw".into(),
            "usernames".into(),
            "owner_usernames".into(),
            "automatic_membership_email_domains".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "flair_icon".into(),
            "flair_upload_id".into(),
            "flair_bg_color".into(),
            "public_admission".into(),
            "public_exit".into(),
            "default_notification_level".into(),
            "muted_category_ids".into(),
            "regular_category_ids".into(),
            "watching_category_ids".into(),
            "tracking_category_ids".into(),
            "watching_first_post_category_ids".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateGroupRequestBody {
    pub group: Group,
}

impl std::fmt::Display for CreateGroupRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateGroupRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.group).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["group".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BasicGroup {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_group: Option<bool>,
    pub publish_read_state: bool,
}

impl std::fmt::Display for BasicGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BasicGroup {
    const LENGTH: usize = 29;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            if let Some(can_edit_group) = &self.can_edit_group {
                format!("{:?}", can_edit_group).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.publish_read_state).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "can_edit_group".into(),
            "publish_read_state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateGroupResponse {
    pub basic_group: BasicGroup,
}

impl std::fmt::Display for CreateGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateGroupResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.basic_group).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["basic_group".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeleteGroupResponse {
    pub success: String,
}

impl std::fmt::Display for DeleteGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeleteGroupResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SmtpUpdatedBy {}

impl std::fmt::Display for SmtpUpdatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SmtpUpdatedBy {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ImapUpdatedBy {}

impl std::fmt::Display for ImapUpdatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ImapUpdatedBy {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetGroupResponseGroup {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    pub is_group_user: bool,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_group: Option<bool>,
    pub publish_read_state: bool,
    pub is_group_owner_display: bool,
    pub mentionable: bool,
    pub messageable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic_membership_email_domains: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_updated_by: Option<SmtpUpdatedBy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_server: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_ssl_mode: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_updated_by: Option<ImapUpdatedBy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_server: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_ssl: Option<String>,
    pub imap_mailbox_name: String,
    pub imap_mailboxes: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_from_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_last_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_old_emails: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_new_emails: Option<String>,
    pub message_count: i64,
    pub allow_unknown_sender_topic_replies: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated_group_ids: Option<Vec<serde_json::Value>>,
    pub watching_category_ids: Vec<serde_json::Value>,
    pub tracking_category_ids: Vec<serde_json::Value>,
    pub watching_first_post_category_ids: Vec<serde_json::Value>,
    pub regular_category_ids: Vec<serde_json::Value>,
    pub muted_category_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watching_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watching_first_post_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted_tags: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for GetGroupResponseGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetGroupResponseGroup {
    const LENGTH: usize = 67;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            format!("{:?}", self.is_group_user).into(),
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            if let Some(can_edit_group) = &self.can_edit_group {
                format!("{:?}", can_edit_group).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.publish_read_state).into(),
            format!("{:?}", self.is_group_owner_display).into(),
            format!("{:?}", self.mentionable).into(),
            format!("{:?}", self.messageable).into(),
            format!("{:?}", self.automatic_membership_email_domains).into(),
            if let Some(smtp_updated_at) = &self.smtp_updated_at {
                format!("{:?}", smtp_updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(smtp_updated_by) = &self.smtp_updated_by {
                format!("{:?}", smtp_updated_by).into()
            } else {
                String::new().into()
            },
            if let Some(smtp_enabled) = &self.smtp_enabled {
                format!("{:?}", smtp_enabled).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.smtp_server).into(),
            format!("{:?}", self.smtp_port).into(),
            format!("{:?}", self.smtp_ssl_mode).into(),
            if let Some(imap_enabled) = &self.imap_enabled {
                format!("{:?}", imap_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(imap_updated_at) = &self.imap_updated_at {
                format!("{:?}", imap_updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(imap_updated_by) = &self.imap_updated_by {
                format!("{:?}", imap_updated_by).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.imap_server).into(),
            format!("{:?}", self.imap_port).into(),
            format!("{:?}", self.imap_ssl).into(),
            self.imap_mailbox_name.clone().into(),
            format!("{:?}", self.imap_mailboxes).into(),
            format!("{:?}", self.email_username).into(),
            if let Some(email_from_alias) = &self.email_from_alias {
                format!("{:?}", email_from_alias).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.email_password).into(),
            format!("{:?}", self.imap_last_error).into(),
            format!("{:?}", self.imap_old_emails).into(),
            format!("{:?}", self.imap_new_emails).into(),
            format!("{:?}", self.message_count).into(),
            format!("{:?}", self.allow_unknown_sender_topic_replies).into(),
            if let Some(associated_group_ids) = &self.associated_group_ids {
                format!("{:?}", associated_group_ids).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.watching_category_ids).into(),
            format!("{:?}", self.tracking_category_ids).into(),
            format!("{:?}", self.watching_first_post_category_ids).into(),
            format!("{:?}", self.regular_category_ids).into(),
            format!("{:?}", self.muted_category_ids).into(),
            if let Some(watching_tags) = &self.watching_tags {
                format!("{:?}", watching_tags).into()
            } else {
                String::new().into()
            },
            if let Some(watching_first_post_tags) = &self.watching_first_post_tags {
                format!("{:?}", watching_first_post_tags).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_tags) = &self.tracking_tags {
                format!("{:?}", tracking_tags).into()
            } else {
                String::new().into()
            },
            if let Some(regular_tags) = &self.regular_tags {
                format!("{:?}", regular_tags).into()
            } else {
                String::new().into()
            },
            if let Some(muted_tags) = &self.muted_tags {
                format!("{:?}", muted_tags).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "is_group_user".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "can_edit_group".into(),
            "publish_read_state".into(),
            "is_group_owner_display".into(),
            "mentionable".into(),
            "messageable".into(),
            "automatic_membership_email_domains".into(),
            "smtp_updated_at".into(),
            "smtp_updated_by".into(),
            "smtp_enabled".into(),
            "smtp_server".into(),
            "smtp_port".into(),
            "smtp_ssl_mode".into(),
            "imap_enabled".into(),
            "imap_updated_at".into(),
            "imap_updated_by".into(),
            "imap_server".into(),
            "imap_port".into(),
            "imap_ssl".into(),
            "imap_mailbox_name".into(),
            "imap_mailboxes".into(),
            "email_username".into(),
            "email_from_alias".into(),
            "email_password".into(),
            "imap_last_error".into(),
            "imap_old_emails".into(),
            "imap_new_emails".into(),
            "message_count".into(),
            "allow_unknown_sender_topic_replies".into(),
            "associated_group_ids".into(),
            "watching_category_ids".into(),
            "tracking_category_ids".into(),
            "watching_first_post_category_ids".into(),
            "regular_category_ids".into(),
            "muted_category_ids".into(),
            "watching_tags".into(),
            "watching_first_post_tags".into(),
            "tracking_tags".into(),
            "regular_tags".into(),
            "muted_tags".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Extras {
    pub visible_group_names: Vec<serde_json::Value>,
}

impl std::fmt::Display for Extras {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Extras {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.visible_group_names).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["visible_group_names".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetGroupResponse {
    pub group: GetGroupResponseGroup,
    pub extras: Extras,
}

impl std::fmt::Display for GetGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetGroupResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.group).into(),
            format!("{:?}", self.extras).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["group".into(), "extras".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateGroupRequestBody {
    pub group: Group,
}

impl std::fmt::Display for UpdateGroupRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateGroupRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.group).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["group".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateGroupResponse {
    pub success: String,
}

impl std::fmt::Display for UpdateGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateGroupResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetGroupByIdResponseGroup {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    pub is_group_user: bool,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_group: Option<bool>,
    pub publish_read_state: bool,
    pub is_group_owner_display: bool,
    pub mentionable: bool,
    pub messageable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic_membership_email_domains: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_updated_by: Option<SmtpUpdatedBy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_server: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_ssl_mode: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_updated_by: Option<ImapUpdatedBy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_server: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_port: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_ssl: Option<String>,
    pub imap_mailbox_name: String,
    pub imap_mailboxes: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_from_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_last_error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_old_emails: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imap_new_emails: Option<String>,
    pub message_count: i64,
    pub allow_unknown_sender_topic_replies: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated_group_ids: Option<Vec<serde_json::Value>>,
    pub watching_category_ids: Vec<serde_json::Value>,
    pub tracking_category_ids: Vec<serde_json::Value>,
    pub watching_first_post_category_ids: Vec<serde_json::Value>,
    pub regular_category_ids: Vec<serde_json::Value>,
    pub muted_category_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watching_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watching_first_post_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub muted_tags: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for GetGroupByIdResponseGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetGroupByIdResponseGroup {
    const LENGTH: usize = 67;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            format!("{:?}", self.is_group_user).into(),
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            if let Some(can_edit_group) = &self.can_edit_group {
                format!("{:?}", can_edit_group).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.publish_read_state).into(),
            format!("{:?}", self.is_group_owner_display).into(),
            format!("{:?}", self.mentionable).into(),
            format!("{:?}", self.messageable).into(),
            format!("{:?}", self.automatic_membership_email_domains).into(),
            if let Some(smtp_updated_at) = &self.smtp_updated_at {
                format!("{:?}", smtp_updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(smtp_updated_by) = &self.smtp_updated_by {
                format!("{:?}", smtp_updated_by).into()
            } else {
                String::new().into()
            },
            if let Some(smtp_enabled) = &self.smtp_enabled {
                format!("{:?}", smtp_enabled).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.smtp_server).into(),
            format!("{:?}", self.smtp_port).into(),
            format!("{:?}", self.smtp_ssl_mode).into(),
            if let Some(imap_enabled) = &self.imap_enabled {
                format!("{:?}", imap_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(imap_updated_at) = &self.imap_updated_at {
                format!("{:?}", imap_updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(imap_updated_by) = &self.imap_updated_by {
                format!("{:?}", imap_updated_by).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.imap_server).into(),
            format!("{:?}", self.imap_port).into(),
            format!("{:?}", self.imap_ssl).into(),
            self.imap_mailbox_name.clone().into(),
            format!("{:?}", self.imap_mailboxes).into(),
            format!("{:?}", self.email_username).into(),
            if let Some(email_from_alias) = &self.email_from_alias {
                format!("{:?}", email_from_alias).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.email_password).into(),
            format!("{:?}", self.imap_last_error).into(),
            format!("{:?}", self.imap_old_emails).into(),
            format!("{:?}", self.imap_new_emails).into(),
            format!("{:?}", self.message_count).into(),
            format!("{:?}", self.allow_unknown_sender_topic_replies).into(),
            if let Some(associated_group_ids) = &self.associated_group_ids {
                format!("{:?}", associated_group_ids).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.watching_category_ids).into(),
            format!("{:?}", self.tracking_category_ids).into(),
            format!("{:?}", self.watching_first_post_category_ids).into(),
            format!("{:?}", self.regular_category_ids).into(),
            format!("{:?}", self.muted_category_ids).into(),
            if let Some(watching_tags) = &self.watching_tags {
                format!("{:?}", watching_tags).into()
            } else {
                String::new().into()
            },
            if let Some(watching_first_post_tags) = &self.watching_first_post_tags {
                format!("{:?}", watching_first_post_tags).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_tags) = &self.tracking_tags {
                format!("{:?}", tracking_tags).into()
            } else {
                String::new().into()
            },
            if let Some(regular_tags) = &self.regular_tags {
                format!("{:?}", regular_tags).into()
            } else {
                String::new().into()
            },
            if let Some(muted_tags) = &self.muted_tags {
                format!("{:?}", muted_tags).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "is_group_user".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "can_edit_group".into(),
            "publish_read_state".into(),
            "is_group_owner_display".into(),
            "mentionable".into(),
            "messageable".into(),
            "automatic_membership_email_domains".into(),
            "smtp_updated_at".into(),
            "smtp_updated_by".into(),
            "smtp_enabled".into(),
            "smtp_server".into(),
            "smtp_port".into(),
            "smtp_ssl_mode".into(),
            "imap_enabled".into(),
            "imap_updated_at".into(),
            "imap_updated_by".into(),
            "imap_server".into(),
            "imap_port".into(),
            "imap_ssl".into(),
            "imap_mailbox_name".into(),
            "imap_mailboxes".into(),
            "email_username".into(),
            "email_from_alias".into(),
            "email_password".into(),
            "imap_last_error".into(),
            "imap_old_emails".into(),
            "imap_new_emails".into(),
            "message_count".into(),
            "allow_unknown_sender_topic_replies".into(),
            "associated_group_ids".into(),
            "watching_category_ids".into(),
            "tracking_category_ids".into(),
            "watching_first_post_category_ids".into(),
            "regular_category_ids".into(),
            "muted_category_ids".into(),
            "watching_tags".into(),
            "watching_first_post_tags".into(),
            "tracking_tags".into(),
            "regular_tags".into(),
            "muted_tags".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetGroupByIdResponse {
    pub group: GetGroupByIdResponseGroup,
    pub extras: Extras,
}

impl std::fmt::Display for GetGroupByIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetGroupByIdResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.group).into(),
            format!("{:?}", self.extras).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["group".into(), "extras".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Members {
    pub id: i64,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub last_posted_at: String,
    pub last_seen_at: String,
    pub added_at: String,
    pub timezone: String,
}

impl std::fmt::Display for Members {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Members {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            format!("{:?}", self.name).into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.title).into(),
            self.last_posted_at.clone().into(),
            self.last_seen_at.clone().into(),
            self.added_at.clone().into(),
            self.timezone.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "title".into(),
            "last_posted_at".into(),
            "last_seen_at".into(),
            "added_at".into(),
            "timezone".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Owners {
    pub id: i64,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub last_posted_at: String,
    pub last_seen_at: String,
    pub added_at: String,
    pub timezone: String,
}

impl std::fmt::Display for Owners {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Owners {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            format!("{:?}", self.name).into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.title).into(),
            self.last_posted_at.clone().into(),
            self.last_seen_at.clone().into(),
            self.added_at.clone().into(),
            self.timezone.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "title".into(),
            "last_posted_at".into(),
            "last_seen_at".into(),
            "added_at".into(),
            "timezone".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Meta {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for Meta {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.total).into(),
            format!("{:?}", self.limit).into(),
            format!("{:?}", self.offset).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["total".into(), "limit".into(), "offset".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListGroupMembersResponse {
    pub members: Vec<Members>,
    pub owners: Vec<Owners>,
    pub meta: Meta,
}

impl std::fmt::Display for ListGroupMembersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListGroupMembersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.members).into(),
            format!("{:?}", self.owners).into(),
            format!("{:?}", self.meta).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["members".into(), "owners".into(), "meta".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddGroupMembersRequestBody {
    #[doc = "comma separated list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usernames: Option<String>,
}

impl std::fmt::Display for AddGroupMembersRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AddGroupMembersRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(usernames) = &self.usernames {
            format!("{:?}", usernames).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["usernames".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddGroupMembersResponse {
    pub success: String,
    pub usernames: Vec<serde_json::Value>,
    pub emails: Vec<serde_json::Value>,
}

impl std::fmt::Display for AddGroupMembersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AddGroupMembersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.success.clone().into(),
            format!("{:?}", self.usernames).into(),
            format!("{:?}", self.emails).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into(), "usernames".into(), "emails".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RemoveGroupMembersRequestBody {
    #[doc = "comma separated list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usernames: Option<String>,
}

impl std::fmt::Display for RemoveGroupMembersRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RemoveGroupMembersRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(usernames) = &self.usernames {
            format!("{:?}", usernames).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["usernames".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RemoveGroupMembersResponse {
    pub success: String,
    pub usernames: Vec<serde_json::Value>,
    pub skipped_usernames: Vec<serde_json::Value>,
}

impl std::fmt::Display for RemoveGroupMembersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RemoveGroupMembersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.success.clone().into(),
            format!("{:?}", self.usernames).into(),
            format!("{:?}", self.skipped_usernames).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "success".into(),
            "usernames".into(),
            "skipped_usernames".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Groups {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub display_name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_group_user: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_group_owner: Option<bool>,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit_group: Option<bool>,
    pub publish_read_state: bool,
}

impl std::fmt::Display for Groups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Groups {
    const LENGTH: usize = 32;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            self.display_name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            if let Some(is_group_user) = &self.is_group_user {
                format!("{:?}", is_group_user).into()
            } else {
                String::new().into()
            },
            if let Some(is_group_owner) = &self.is_group_owner {
                format!("{:?}", is_group_owner).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            if let Some(can_edit_group) = &self.can_edit_group {
                format!("{:?}", can_edit_group).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.publish_read_state).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "display_name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "is_group_user".into(),
            "is_group_owner".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "can_edit_group".into(),
            "publish_read_state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListGroupsResponseExtras {
    pub type_filters: Vec<serde_json::Value>,
}

impl std::fmt::Display for ListGroupsResponseExtras {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListGroupsResponseExtras {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.type_filters).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["type_filters".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListGroupsResponse {
    pub groups: Vec<Groups>,
    pub extras: ListGroupsResponseExtras,
    pub total_rows_groups: i64,
    pub load_more_groups: String,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListGroupsResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.extras).into(),
            format!("{:?}", self.total_rows_groups).into(),
            self.load_more_groups.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "groups".into(),
            "extras".into(),
            "total_rows_groups".into(),
            "load_more_groups".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateInviteRequestBody {
    #[doc = "required for email invites only"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub skip_email: bool,
    #[doc = "optional, for email invites"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[doc = "optional, for link invites"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_redemptions_allowed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    #[doc = "Optional, either this or `group_names`. Comma separated\nlist for multiple ids."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    #[doc = "Optional, either this or `group_ids`. Comma separated\nlist for multiple names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_names: Option<String>,
    #[doc = "optional, if not supplied, the invite_expiry_days site\nsetting is used"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl std::fmt::Display for CreateInviteRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateInviteRequestBody {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.skip_email).into(),
            if let Some(custom_message) = &self.custom_message {
                format!("{:?}", custom_message).into()
            } else {
                String::new().into()
            },
            if let Some(max_redemptions_allowed) = &self.max_redemptions_allowed {
                format!("{:?}", max_redemptions_allowed).into()
            } else {
                String::new().into()
            },
            if let Some(topic_id) = &self.topic_id {
                format!("{:?}", topic_id).into()
            } else {
                String::new().into()
            },
            if let Some(group_ids) = &self.group_ids {
                format!("{:?}", group_ids).into()
            } else {
                String::new().into()
            },
            if let Some(group_names) = &self.group_names {
                format!("{:?}", group_names).into()
            } else {
                String::new().into()
            },
            if let Some(expires_at) = &self.expires_at {
                format!("{:?}", expires_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "email".into(),
            "skip_email".into(),
            "custom_message".into(),
            "max_redemptions_allowed".into(),
            "topic_id".into(),
            "group_ids".into(),
            "group_names".into(),
            "expires_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateInviteResponseTopics {}

impl std::fmt::Display for CreateInviteResponseTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateInviteResponseTopics {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateInviteResponseGroups {}

impl std::fmt::Display for CreateInviteResponseGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateInviteResponseGroups {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateInviteResponse {
    pub id: i64,
    pub invite_key: String,
    pub link: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    pub emailed: bool,
    pub can_delete_invite: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub expires_at: String,
    pub expired: bool,
    pub topics: Vec<CreateInviteResponseTopics>,
    pub groups: Vec<CreateInviteResponseGroups>,
}

impl std::fmt::Display for CreateInviteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateInviteResponse {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.invite_key.clone().into(),
            self.link.clone().into(),
            format!("{:?}", self.description).into(),
            self.email.clone().into(),
            format!("{:?}", self.domain).into(),
            format!("{:?}", self.emailed).into(),
            format!("{:?}", self.can_delete_invite).into(),
            format!("{:?}", self.custom_message).into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.expires_at.clone().into(),
            format!("{:?}", self.expired).into(),
            format!("{:?}", self.topics).into(),
            format!("{:?}", self.groups).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "invite_key".into(),
            "link".into(),
            "description".into(),
            "email".into(),
            "domain".into(),
            "emailed".into(),
            "can_delete_invite".into(),
            "custom_message".into(),
            "created_at".into(),
            "updated_at".into(),
            "expires_at".into(),
            "expired".into(),
            "topics".into(),
            "groups".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateMultipleInvitesRequestBody {
    #[doc = "pass 1 email per invite to be generated. other properties\nwill be shared by each \
             invite."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default)]
    pub skip_email: bool,
    #[doc = "optional, for email invites"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
    #[doc = "optional, for link invites"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_redemptions_allowed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    #[doc = "Optional, either this or `group_names`. Comma separated\nlist for multiple ids."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<String>,
    #[doc = "Optional, either this or `group_ids`. Comma separated\nlist for multiple names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_names: Option<String>,
    #[doc = "optional, if not supplied, the invite_expiry_days site\nsetting is used"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
}

impl std::fmt::Display for CreateMultipleInvitesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateMultipleInvitesRequestBody {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.skip_email).into(),
            if let Some(custom_message) = &self.custom_message {
                format!("{:?}", custom_message).into()
            } else {
                String::new().into()
            },
            if let Some(max_redemptions_allowed) = &self.max_redemptions_allowed {
                format!("{:?}", max_redemptions_allowed).into()
            } else {
                String::new().into()
            },
            if let Some(topic_id) = &self.topic_id {
                format!("{:?}", topic_id).into()
            } else {
                String::new().into()
            },
            if let Some(group_ids) = &self.group_ids {
                format!("{:?}", group_ids).into()
            } else {
                String::new().into()
            },
            if let Some(group_names) = &self.group_names {
                format!("{:?}", group_names).into()
            } else {
                String::new().into()
            },
            if let Some(expires_at) = &self.expires_at {
                format!("{:?}", expires_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "email".into(),
            "skip_email".into(),
            "custom_message".into(),
            "max_redemptions_allowed".into(),
            "topic_id".into(),
            "group_ids".into(),
            "group_names".into(),
            "expires_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateMultipleInvitesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_successfully_created_invitations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_failed_invitations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_invitations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_invitations: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for CreateMultipleInvitesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateMultipleInvitesResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(num_successfully_created_invitations) =
                &self.num_successfully_created_invitations
            {
                format!("{:?}", num_successfully_created_invitations).into()
            } else {
                String::new().into()
            },
            if let Some(num_failed_invitations) = &self.num_failed_invitations {
                format!("{:?}", num_failed_invitations).into()
            } else {
                String::new().into()
            },
            if let Some(failed_invitations) = &self.failed_invitations {
                format!("{:?}", failed_invitations).into()
            } else {
                String::new().into()
            },
            if let Some(successful_invitations) = &self.successful_invitations {
                format!("{:?}", successful_invitations).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "num_successfully_created_invitations".into(),
            "num_failed_invitations".into(),
            "failed_invitations".into(),
            "successful_invitations".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_title: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Data {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(badge_id) = &self.badge_id {
                format!("{:?}", badge_id).into()
            } else {
                String::new().into()
            },
            if let Some(badge_name) = &self.badge_name {
                format!("{:?}", badge_name).into()
            } else {
                String::new().into()
            },
            if let Some(badge_slug) = &self.badge_slug {
                format!("{:?}", badge_slug).into()
            } else {
                String::new().into()
            },
            if let Some(badge_title) = &self.badge_title {
                format!("{:?}", badge_title).into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "badge_id".into(),
            "badge_name".into(),
            "badge_slug".into(),
            "badge_title".into(),
            "username".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Notifications {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl std::fmt::Display for Notifications {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Notifications {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(notification_type) = &self.notification_type {
                format!("{:?}", notification_type).into()
            } else {
                String::new().into()
            },
            if let Some(read) = &self.read {
                format!("{:?}", read).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(post_number) = &self.post_number {
                format!("{:?}", post_number).into()
            } else {
                String::new().into()
            },
            if let Some(topic_id) = &self.topic_id {
                format!("{:?}", topic_id).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
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
        vec![
            "id".into(),
            "user_id".into(),
            "notification_type".into(),
            "read".into(),
            "created_at".into(),
            "post_number".into(),
            "topic_id".into(),
            "slug".into(),
            "data".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetNotificationsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notifications: Option<Vec<Notifications>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_rows_notifications: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seen_notification_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_more_notifications: Option<String>,
}

impl std::fmt::Display for GetNotificationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetNotificationsResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(notifications) = &self.notifications {
                format!("{:?}", notifications).into()
            } else {
                String::new().into()
            },
            if let Some(total_rows_notifications) = &self.total_rows_notifications {
                format!("{:?}", total_rows_notifications).into()
            } else {
                String::new().into()
            },
            if let Some(seen_notification_id) = &self.seen_notification_id {
                format!("{:?}", seen_notification_id).into()
            } else {
                String::new().into()
            },
            if let Some(load_more_notifications) = &self.load_more_notifications {
                format!("{:?}", load_more_notifications).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "notifications".into(),
            "total_rows_notifications".into(),
            "seen_notification_id".into(),
            "load_more_notifications".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MarkNotificationsAsReadRequestBody {
    #[doc = "(optional) Leave off to mark all notifications as\nread"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl std::fmt::Display for MarkNotificationsAsReadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MarkNotificationsAsReadRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(id) = &self.id {
            format!("{:?}", id).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MarkNotificationsAsReadResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
}

impl std::fmt::Display for MarkNotificationsAsReadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MarkNotificationsAsReadResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(success) = &self.success {
            format!("{:?}", success).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ActionsSummary {
    pub id: i64,
    pub can_act: bool,
}

impl std::fmt::Display for ActionsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ActionsSummary {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.can_act).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "can_act".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LatestPosts {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: i64,
    pub post_type: i64,
    pub posts_count: i64,
    pub updated_at: String,
    pub reply_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<String>,
    pub quote_count: i64,
    pub incoming_link_count: i64,
    pub reads: i64,
    pub readers_count: i64,
    pub score: f64,
    pub yours: bool,
    pub topic_id: i64,
    pub topic_slug: String,
    pub topic_title: String,
    pub topic_html_title: String,
    pub category_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<String>,
    pub badges_granted: Vec<serde_json::Value>,
    pub version: i64,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_recover: bool,
    pub can_see_hidden_post: bool,
    pub can_wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    pub bookmarked: bool,
    pub raw: String,
    pub actions_summary: Vec<ActionsSummary>,
    pub moderator: bool,
    pub admin: bool,
    pub staff: bool,
    pub user_id: i64,
    pub hidden: bool,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub user_deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    pub can_view_edit_history: bool,
    pub wiki: bool,
    pub excerpt: String,
    pub truncated: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_id: Option<String>,
    pub reviewable_score_count: i64,
    pub reviewable_score_pending_count: i64,
    pub post_url: String,
}

impl std::fmt::Display for LatestPosts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LatestPosts {
    const LENGTH: usize = 58;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.name).into(),
            self.username.clone().into(),
            self.avatar_template.clone().into(),
            self.created_at.clone().into(),
            self.cooked.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_type).into(),
            format!("{:?}", self.posts_count).into(),
            self.updated_at.clone().into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.reply_to_post_number).into(),
            format!("{:?}", self.quote_count).into(),
            format!("{:?}", self.incoming_link_count).into(),
            format!("{:?}", self.reads).into(),
            format!("{:?}", self.readers_count).into(),
            format!("{:?}", self.score).into(),
            format!("{:?}", self.yours).into(),
            format!("{:?}", self.topic_id).into(),
            self.topic_slug.clone().into(),
            self.topic_title.clone().into(),
            self.topic_html_title.clone().into(),
            format!("{:?}", self.category_id).into(),
            format!("{:?}", self.display_username).into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.flair_group_id).into(),
            format!("{:?}", self.badges_granted).into(),
            format!("{:?}", self.version).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_recover).into(),
            format!("{:?}", self.can_see_hidden_post).into(),
            format!("{:?}", self.can_wiki).into(),
            format!("{:?}", self.user_title).into(),
            format!("{:?}", self.bookmarked).into(),
            self.raw.clone().into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.staff).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_deleted).into(),
            format!("{:?}", self.edit_reason).into(),
            format!("{:?}", self.can_view_edit_history).into(),
            format!("{:?}", self.wiki).into(),
            self.excerpt.clone().into(),
            format!("{:?}", self.truncated).into(),
            format!("{:?}", self.reviewable_id).into(),
            format!("{:?}", self.reviewable_score_count).into(),
            format!("{:?}", self.reviewable_score_pending_count).into(),
            self.post_url.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "posts_count".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "topic_title".into(),
            "topic_html_title".into(),
            "category_id".into(),
            "display_username".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "flair_group_id".into(),
            "badges_granted".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_see_hidden_post".into(),
            "can_wiki".into(),
            "user_title".into(),
            "bookmarked".into(),
            "raw".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "excerpt".into(),
            "truncated".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
            "post_url".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListPostsResponse {
    pub latest_posts: Vec<LatestPosts>,
}

impl std::fmt::Display for ListPostsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListPostsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.latest_posts).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["latest_posts".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTopicPostPMRequestBody {
    #[doc = "Required if creating a new topic or new private message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub raw: String,
    #[doc = "Required if creating a new post."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    #[doc = "Optional if creating a new topic, and ignored if creating\na new post."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<i64>,
    #[doc = "Required for private message, comma separated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_recipients: Option<String>,
    #[doc = "Deprecated. Use target_recipients instead."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub target_usernames: Option<String>,
    #[doc = "Required for new private message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "Optional, the post number to reply to inside a topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<i64>,
    #[doc = "Provide a URL from a remote system to associate a forum\ntopic with that URL, \
             typically for using Discourse as a comments\nsystem for an external blog."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub embed_url: Option<String>,
    #[doc = "Provide an external_id from a remote system to associate\na forum topic with that id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "If false, the user will not track the topic. By default,\nthe user will track the \
             topic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_track: Option<bool>,
}

impl std::fmt::Display for CreateTopicPostPMRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateTopicPostPMRequestBody {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            self.raw.clone().into(),
            if let Some(topic_id) = &self.topic_id {
                format!("{:?}", topic_id).into()
            } else {
                String::new().into()
            },
            if let Some(category) = &self.category {
                format!("{:?}", category).into()
            } else {
                String::new().into()
            },
            if let Some(target_recipients) = &self.target_recipients {
                format!("{:?}", target_recipients).into()
            } else {
                String::new().into()
            },
            if let Some(target_usernames) = &self.target_usernames {
                format!("{:?}", target_usernames).into()
            } else {
                String::new().into()
            },
            if let Some(archetype) = &self.archetype {
                format!("{:?}", archetype).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(reply_to_post_number) = &self.reply_to_post_number {
                format!("{:?}", reply_to_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(embed_url) = &self.embed_url {
                format!("{:?}", embed_url).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
            if let Some(auto_track) = &self.auto_track {
                format!("{:?}", auto_track).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "title".into(),
            "raw".into(),
            "topic_id".into(),
            "category".into(),
            "target_recipients".into(),
            "target_usernames".into(),
            "archetype".into(),
            "created_at".into(),
            "reply_to_post_number".into(),
            "embed_url".into(),
            "external_id".into(),
            "auto_track".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTopicPostPMResponse {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    pub cooked: String,
    pub post_number: i64,
    pub post_type: i64,
    pub posts_count: i64,
    pub updated_at: String,
    pub reply_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<String>,
    pub quote_count: i64,
    pub incoming_link_count: i64,
    pub reads: i64,
    pub readers_count: i64,
    pub score: f64,
    pub yours: bool,
    pub topic_id: i64,
    pub topic_slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badges_granted: Option<Vec<serde_json::Value>>,
    pub version: i64,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_recover: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_see_hidden_post: Option<bool>,
    pub can_wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    pub bookmarked: bool,
    pub actions_summary: Vec<ActionsSummary>,
    pub moderator: bool,
    pub admin: bool,
    pub staff: bool,
    pub user_id: i64,
    pub draft_sequence: i64,
    pub hidden: bool,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub user_deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    pub can_view_edit_history: bool,
    pub wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_id: Option<i64>,
    pub reviewable_score_count: i64,
    pub reviewable_score_pending_count: i64,
    pub post_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_localizations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentioned_users: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for CreateTopicPostPMResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateTopicPostPMResponse {
    const LENGTH: usize = 56;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.name).into(),
            self.username.clone().into(),
            self.avatar_template.clone().into(),
            self.created_at.clone().into(),
            if let Some(raw) = &self.raw {
                format!("{:?}", raw).into()
            } else {
                String::new().into()
            },
            self.cooked.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_type).into(),
            format!("{:?}", self.posts_count).into(),
            self.updated_at.clone().into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.reply_to_post_number).into(),
            format!("{:?}", self.quote_count).into(),
            format!("{:?}", self.incoming_link_count).into(),
            format!("{:?}", self.reads).into(),
            format!("{:?}", self.readers_count).into(),
            format!("{:?}", self.score).into(),
            format!("{:?}", self.yours).into(),
            format!("{:?}", self.topic_id).into(),
            self.topic_slug.clone().into(),
            format!("{:?}", self.display_username).into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            if let Some(flair_group_id) = &self.flair_group_id {
                format!("{:?}", flair_group_id).into()
            } else {
                String::new().into()
            },
            if let Some(badges_granted) = &self.badges_granted {
                format!("{:?}", badges_granted).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.version).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_recover).into(),
            if let Some(can_see_hidden_post) = &self.can_see_hidden_post {
                format!("{:?}", can_see_hidden_post).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_wiki).into(),
            format!("{:?}", self.user_title).into(),
            format!("{:?}", self.bookmarked).into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.staff).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.draft_sequence).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_deleted).into(),
            format!("{:?}", self.edit_reason).into(),
            format!("{:?}", self.can_view_edit_history).into(),
            format!("{:?}", self.wiki).into(),
            format!("{:?}", self.reviewable_id).into(),
            format!("{:?}", self.reviewable_score_count).into(),
            format!("{:?}", self.reviewable_score_pending_count).into(),
            self.post_url.clone().into(),
            if let Some(post_localizations) = &self.post_localizations {
                format!("{:?}", post_localizations).into()
            } else {
                String::new().into()
            },
            if let Some(mentioned_users) = &self.mentioned_users {
                format!("{:?}", mentioned_users).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "raw".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "posts_count".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "display_username".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "flair_group_id".into(),
            "badges_granted".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_see_hidden_post".into(),
            "can_wiki".into(),
            "user_title".into(),
            "bookmarked".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "draft_sequence".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
            "post_url".into(),
            "post_localizations".into(),
            "mentioned_users".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetPostResponseActionsSummary {
    #[doc = "`2`: like, `3`, `4`, `6`, `7`, `8`: flag"]
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_undo: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_act: Option<bool>,
}

impl std::fmt::Display for GetPostResponseActionsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetPostResponseActionsSummary {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            if let Some(count) = &self.count {
                format!("{:?}", count).into()
            } else {
                String::new().into()
            },
            if let Some(acted) = &self.acted {
                format!("{:?}", acted).into()
            } else {
                String::new().into()
            },
            if let Some(can_undo) = &self.can_undo {
                format!("{:?}", can_undo).into()
            } else {
                String::new().into()
            },
            if let Some(can_act) = &self.can_act {
                format!("{:?}", can_act).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "count".into(),
            "acted".into(),
            "can_undo".into(),
            "can_act".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetPostResponse {
    pub id: i64,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: i64,
    pub post_type: i64,
    pub posts_count: i64,
    pub updated_at: String,
    pub reply_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<String>,
    pub quote_count: i64,
    pub incoming_link_count: i64,
    pub reads: i64,
    pub readers_count: i64,
    pub score: f64,
    pub yours: bool,
    pub topic_id: i64,
    pub topic_slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    pub version: i64,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_recover: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_see_hidden_post: Option<bool>,
    pub can_wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    pub bookmarked: bool,
    pub raw: String,
    pub actions_summary: Vec<GetPostResponseActionsSummary>,
    pub moderator: bool,
    pub admin: bool,
    pub staff: bool,
    pub user_id: i64,
    pub hidden: bool,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub user_deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    pub can_view_edit_history: bool,
    pub wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_id: Option<i64>,
    pub reviewable_score_count: i64,
    pub reviewable_score_pending_count: i64,
    pub post_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentioned_users: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_username: Option<String>,
}

impl std::fmt::Display for GetPostResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetPostResponse {
    const LENGTH: usize = 53;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.avatar_template.clone().into(),
            self.created_at.clone().into(),
            self.cooked.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_type).into(),
            format!("{:?}", self.posts_count).into(),
            self.updated_at.clone().into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.reply_to_post_number).into(),
            format!("{:?}", self.quote_count).into(),
            format!("{:?}", self.incoming_link_count).into(),
            format!("{:?}", self.reads).into(),
            format!("{:?}", self.readers_count).into(),
            format!("{:?}", self.score).into(),
            format!("{:?}", self.yours).into(),
            format!("{:?}", self.topic_id).into(),
            self.topic_slug.clone().into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            if let Some(flair_group_id) = &self.flair_group_id {
                format!("{:?}", flair_group_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.version).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_recover).into(),
            if let Some(can_see_hidden_post) = &self.can_see_hidden_post {
                format!("{:?}", can_see_hidden_post).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_wiki).into(),
            format!("{:?}", self.user_title).into(),
            format!("{:?}", self.bookmarked).into(),
            self.raw.clone().into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.staff).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_deleted).into(),
            format!("{:?}", self.edit_reason).into(),
            format!("{:?}", self.can_view_edit_history).into(),
            format!("{:?}", self.wiki).into(),
            format!("{:?}", self.reviewable_id).into(),
            format!("{:?}", self.reviewable_score_count).into(),
            format!("{:?}", self.reviewable_score_pending_count).into(),
            self.post_url.clone().into(),
            if let Some(mentioned_users) = &self.mentioned_users {
                format!("{:?}", mentioned_users).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(display_username) = &self.display_username {
                format!("{:?}", display_username).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "posts_count".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "flair_group_id".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_see_hidden_post".into(),
            "can_wiki".into(),
            "user_title".into(),
            "bookmarked".into(),
            "raw".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
            "post_url".into(),
            "mentioned_users".into(),
            "name".into(),
            "display_username".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdatePostRequestBodyPost {
    pub raw: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
}

impl std::fmt::Display for UpdatePostRequestBodyPost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdatePostRequestBodyPost {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.raw.clone().into(),
            if let Some(edit_reason) = &self.edit_reason {
                format!("{:?}", edit_reason).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["raw".into(), "edit_reason".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdatePostRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post: Option<UpdatePostRequestBodyPost>,
}

impl std::fmt::Display for UpdatePostRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdatePostRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(post) = &self.post {
            format!("{:?}", post).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["post".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdatePostResponsePost {
    pub id: i64,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: i64,
    pub post_type: i64,
    pub posts_count: i64,
    pub updated_at: String,
    pub reply_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<String>,
    pub quote_count: i64,
    pub incoming_link_count: i64,
    pub reads: i64,
    pub readers_count: i64,
    pub score: f64,
    pub yours: bool,
    pub topic_id: i64,
    pub topic_slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badges_granted: Option<Vec<serde_json::Value>>,
    pub version: i64,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_recover: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_see_hidden_post: Option<bool>,
    pub can_wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    pub bookmarked: bool,
    pub raw: String,
    pub actions_summary: Vec<ActionsSummary>,
    pub moderator: bool,
    pub admin: bool,
    pub staff: bool,
    pub user_id: i64,
    pub draft_sequence: i64,
    pub hidden: bool,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub user_deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    pub can_view_edit_history: bool,
    pub wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_id: Option<i64>,
    pub reviewable_score_count: i64,
    pub reviewable_score_pending_count: i64,
    pub post_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_localizations: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mentioned_users: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_username: Option<String>,
}

impl std::fmt::Display for UpdatePostResponsePost {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdatePostResponsePost {
    const LENGTH: usize = 56;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.avatar_template.clone().into(),
            self.created_at.clone().into(),
            self.cooked.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_type).into(),
            format!("{:?}", self.posts_count).into(),
            self.updated_at.clone().into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.reply_to_post_number).into(),
            format!("{:?}", self.quote_count).into(),
            format!("{:?}", self.incoming_link_count).into(),
            format!("{:?}", self.reads).into(),
            format!("{:?}", self.readers_count).into(),
            format!("{:?}", self.score).into(),
            format!("{:?}", self.yours).into(),
            format!("{:?}", self.topic_id).into(),
            self.topic_slug.clone().into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            if let Some(flair_group_id) = &self.flair_group_id {
                format!("{:?}", flair_group_id).into()
            } else {
                String::new().into()
            },
            if let Some(badges_granted) = &self.badges_granted {
                format!("{:?}", badges_granted).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.version).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_recover).into(),
            if let Some(can_see_hidden_post) = &self.can_see_hidden_post {
                format!("{:?}", can_see_hidden_post).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_wiki).into(),
            format!("{:?}", self.user_title).into(),
            format!("{:?}", self.bookmarked).into(),
            self.raw.clone().into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.staff).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.draft_sequence).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_deleted).into(),
            format!("{:?}", self.edit_reason).into(),
            format!("{:?}", self.can_view_edit_history).into(),
            format!("{:?}", self.wiki).into(),
            format!("{:?}", self.reviewable_id).into(),
            format!("{:?}", self.reviewable_score_count).into(),
            format!("{:?}", self.reviewable_score_pending_count).into(),
            self.post_url.clone().into(),
            if let Some(post_localizations) = &self.post_localizations {
                format!("{:?}", post_localizations).into()
            } else {
                String::new().into()
            },
            if let Some(mentioned_users) = &self.mentioned_users {
                format!("{:?}", mentioned_users).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(display_username) = &self.display_username {
                format!("{:?}", display_username).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "posts_count".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "flair_group_id".into(),
            "badges_granted".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_see_hidden_post".into(),
            "can_wiki".into(),
            "user_title".into(),
            "bookmarked".into(),
            "raw".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "draft_sequence".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
            "post_url".into(),
            "post_localizations".into(),
            "mentioned_users".into(),
            "name".into(),
            "display_username".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdatePostResponse {
    pub post: UpdatePostResponsePost,
}

impl std::fmt::Display for UpdatePostResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdatePostResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.post).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["post".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeletePostRequestBody {
    #[doc = "The `SiteSetting.can_permanently_delete` needs to be\nenabled first before this \
             param can be used. Also this endpoint\nneeds to be called first without \
             `force_destroy` and then followed\nup with a second call 5 minutes later with \
             `force_destroy` to\npermanently delete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_destroy: Option<bool>,
}

impl std::fmt::Display for DeletePostRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeletePostRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(force_destroy) = &self.force_destroy {
            format!("{:?}", force_destroy).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["force_destroy".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ReplyToUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
}

impl std::fmt::Display for ReplyToUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ReplyToUser {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            self.username.clone().into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostRepliesResponse {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: i64,
    pub post_type: i64,
    pub posts_count: i64,
    pub updated_at: String,
    pub reply_count: i64,
    pub reply_to_post_number: i64,
    pub quote_count: i64,
    pub incoming_link_count: i64,
    pub reads: i64,
    pub readers_count: i64,
    pub score: f64,
    pub yours: bool,
    pub topic_id: i64,
    pub topic_slug: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    pub version: i64,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_recover: bool,
    pub can_see_hidden_post: bool,
    pub can_wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    pub reply_to_user: ReplyToUser,
    pub bookmarked: bool,
    pub actions_summary: Vec<ActionsSummary>,
    pub moderator: bool,
    pub admin: bool,
    pub staff: bool,
    pub user_id: i64,
    pub hidden: bool,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub user_deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    pub can_view_edit_history: bool,
    pub wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_id: Option<i64>,
    pub reviewable_score_count: i64,
    pub reviewable_score_pending_count: i64,
    pub post_url: String,
}

impl std::fmt::Display for PostRepliesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostRepliesResponse {
    const LENGTH: usize = 52;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.name).into(),
            self.username.clone().into(),
            self.avatar_template.clone().into(),
            self.created_at.clone().into(),
            self.cooked.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_type).into(),
            format!("{:?}", self.posts_count).into(),
            self.updated_at.clone().into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.reply_to_post_number).into(),
            format!("{:?}", self.quote_count).into(),
            format!("{:?}", self.incoming_link_count).into(),
            format!("{:?}", self.reads).into(),
            format!("{:?}", self.readers_count).into(),
            format!("{:?}", self.score).into(),
            format!("{:?}", self.yours).into(),
            format!("{:?}", self.topic_id).into(),
            self.topic_slug.clone().into(),
            format!("{:?}", self.display_username).into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            if let Some(flair_group_id) = &self.flair_group_id {
                format!("{:?}", flair_group_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.version).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_recover).into(),
            format!("{:?}", self.can_see_hidden_post).into(),
            format!("{:?}", self.can_wiki).into(),
            format!("{:?}", self.user_title).into(),
            format!("{:?}", self.reply_to_user).into(),
            format!("{:?}", self.bookmarked).into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.staff).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_deleted).into(),
            format!("{:?}", self.edit_reason).into(),
            format!("{:?}", self.can_view_edit_history).into(),
            format!("{:?}", self.wiki).into(),
            format!("{:?}", self.reviewable_id).into(),
            format!("{:?}", self.reviewable_score_count).into(),
            format!("{:?}", self.reviewable_score_pending_count).into(),
            self.post_url.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "posts_count".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "display_username".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "flair_group_id".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_see_hidden_post".into(),
            "can_wiki".into(),
            "user_title".into(),
            "reply_to_user".into(),
            "bookmarked".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
            "post_url".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LockPostRequestBody {
    #[doc = "Whether to lock the post (true/false)"]
    pub locked: String,
}

impl std::fmt::Display for LockPostRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LockPostRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.locked.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["locked".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LockPostResponse {
    #[doc = "Whether the post is locked"]
    pub locked: bool,
}

impl std::fmt::Display for LockPostResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LockPostResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.locked).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["locked".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PerformPostActionRequestBody {
    #[doc = "The ID of the post to perform the action on"]
    pub id: i64,
    #[doc = "The ID of the post action type (e.g., 2 for like)"]
    pub post_action_type_id: i64,
    #[doc = "Whether to flag the entire topic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flag_topic: Option<bool>,
}

impl std::fmt::Display for PerformPostActionRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PerformPostActionRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.post_action_type_id).into(),
            if let Some(flag_topic) = &self.flag_topic {
                format!("{:?}", flag_topic).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "post_action_type_id".into(),
            "flag_topic".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PerformPostActionResponseActionsSummary {
    #[doc = "ID of the action type (e.g., 2 for like)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Number of times this action has been performed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Whether the current user has performed this\naction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acted: Option<bool>,
    #[doc = "Whether the current user can undo this action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_undo: Option<bool>,
    #[doc = "Whether the current user can perform this action"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_act: Option<bool>,
}

impl std::fmt::Display for PerformPostActionResponseActionsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PerformPostActionResponseActionsSummary {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(count) = &self.count {
                format!("{:?}", count).into()
            } else {
                String::new().into()
            },
            if let Some(acted) = &self.acted {
                format!("{:?}", acted).into()
            } else {
                String::new().into()
            },
            if let Some(can_undo) = &self.can_undo {
                format!("{:?}", can_undo).into()
            } else {
                String::new().into()
            },
            if let Some(can_act) = &self.can_act {
                format!("{:?}", can_act).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "count".into(),
            "acted".into(),
            "can_undo".into(),
            "can_act".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PerformPostActionResponse {
    #[doc = "The ID of the post"]
    pub id: i64,
    #[doc = "The name of the post author"]
    pub name: String,
    #[doc = "The username of the post author"]
    pub username: String,
    #[doc = "Template for the author's avatar URL"]
    pub avatar_template: String,
    #[doc = "When the post was created"]
    pub created_at: String,
    #[doc = "The HTML content of the post"]
    pub cooked: String,
    #[doc = "The post number within the topic"]
    pub post_number: i64,
    #[doc = "The type of post"]
    pub post_type: i64,
    #[doc = "Total posts count for the user"]
    pub posts_count: i64,
    #[doc = "When the post was last updated"]
    pub updated_at: String,
    #[doc = "Number of replies to this post"]
    pub reply_count: i64,
    #[doc = "Post number this post is replying to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<String>,
    #[doc = "Number of times this post has been quoted"]
    pub quote_count: i64,
    #[doc = "Number of incoming links to this post"]
    pub incoming_link_count: i64,
    #[doc = "Number of reads"]
    pub reads: i64,
    #[doc = "Number of readers"]
    pub readers_count: i64,
    #[doc = "Post score"]
    pub score: f64,
    #[doc = "Whether this post belongs to the current user"]
    pub yours: bool,
    #[doc = "ID of the topic this post belongs to"]
    pub topic_id: i64,
    #[doc = "Slug of the topic this post belongs to"]
    pub topic_slug: String,
    #[doc = "Display username of the post author"]
    pub display_username: String,
    #[doc = "Primary group name of the author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[doc = "Flair name of the author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[doc = "Flair URL of the author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[doc = "Flair background color of the author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[doc = "Flair color of the author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[doc = "Flair group ID of the author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    #[doc = "Badges granted to the user"]
    pub badges_granted: Vec<serde_json::Value>,
    #[doc = "Version number of the post"]
    pub version: i64,
    #[doc = "Whether the current user can edit this post"]
    pub can_edit: bool,
    #[doc = "Whether the current user can delete this post"]
    pub can_delete: bool,
    #[doc = "Whether the current user can recover this post"]
    pub can_recover: bool,
    #[doc = "Whether the current user can see hidden posts"]
    pub can_see_hidden_post: bool,
    #[doc = "Whether the current user can wiki this post"]
    pub can_wiki: bool,
    #[doc = "Title of the post author"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    #[doc = "Whether the post is bookmarked by the current user"]
    pub bookmarked: bool,
    #[doc = "Summary of actions performed on this post"]
    pub actions_summary: Vec<PerformPostActionResponseActionsSummary>,
    #[doc = "Whether the post author is a moderator"]
    pub moderator: bool,
    #[doc = "Whether the post author is an admin"]
    pub admin: bool,
    #[doc = "Whether the post author is staff"]
    pub staff: bool,
    #[doc = "ID of the post author"]
    pub user_id: i64,
    #[doc = "Whether the post is hidden"]
    pub hidden: bool,
    #[doc = "Trust level of the post author"]
    pub trust_level: i64,
    #[doc = "When the post was deleted"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[doc = "Whether the post was deleted by the user"]
    pub user_deleted: bool,
    #[doc = "Reason for the last edit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    #[doc = "Whether the current user can view edit history"]
    pub can_view_edit_history: bool,
    #[doc = "Whether this is a wiki post"]
    pub wiki: bool,
    #[doc = "ID of the reviewable if this post is under review"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_id: Option<i64>,
    #[doc = "Number of reviewable scores"]
    pub reviewable_score_count: i64,
    #[doc = "Number of pending reviewable scores"]
    pub reviewable_score_pending_count: i64,
    #[doc = "URL of the post"]
    pub post_url: String,
}

impl std::fmt::Display for PerformPostActionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PerformPostActionResponse {
    const LENGTH: usize = 52;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.username.clone().into(),
            self.avatar_template.clone().into(),
            self.created_at.clone().into(),
            self.cooked.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_type).into(),
            format!("{:?}", self.posts_count).into(),
            self.updated_at.clone().into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.reply_to_post_number).into(),
            format!("{:?}", self.quote_count).into(),
            format!("{:?}", self.incoming_link_count).into(),
            format!("{:?}", self.reads).into(),
            format!("{:?}", self.readers_count).into(),
            format!("{:?}", self.score).into(),
            format!("{:?}", self.yours).into(),
            format!("{:?}", self.topic_id).into(),
            self.topic_slug.clone().into(),
            self.display_username.clone().into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.flair_group_id).into(),
            format!("{:?}", self.badges_granted).into(),
            format!("{:?}", self.version).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_recover).into(),
            format!("{:?}", self.can_see_hidden_post).into(),
            format!("{:?}", self.can_wiki).into(),
            format!("{:?}", self.user_title).into(),
            format!("{:?}", self.bookmarked).into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.staff).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_deleted).into(),
            format!("{:?}", self.edit_reason).into(),
            format!("{:?}", self.can_view_edit_history).into(),
            format!("{:?}", self.wiki).into(),
            format!("{:?}", self.reviewable_id).into(),
            format!("{:?}", self.reviewable_score_count).into(),
            format!("{:?}", self.reviewable_score_pending_count).into(),
            self.post_url.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "posts_count".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "display_username".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "flair_group_id".into(),
            "badges_granted".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_see_hidden_post".into(),
            "can_wiki".into(),
            "user_title".into(),
            "bookmarked".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
            "post_url".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserPrivateMessagesResponseUsers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_template: Option<String>,
}

impl std::fmt::Display for ListUserPrivateMessagesResponseUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserPrivateMessagesResponseUsers {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(avatar_template) = &self.avatar_template {
                format!("{:?}", avatar_template).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserPrivateMessagesResponseTopicListTopicsPosters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
}

impl std::fmt::Display for ListUserPrivateMessagesResponseTopicListTopicsPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserPrivateMessagesResponseTopicListTopicsPosters {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(extras) = &self.extras {
                format!("{:?}", extras).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group_id) = &self.primary_group_id {
                format!("{:?}", primary_group_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "extras".into(),
            "description".into(),
            "user_id".into(),
            "primary_group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Participants {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
}

impl std::fmt::Display for Participants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Participants {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(extras) = &self.extras {
                format!("{:?}", extras).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group_id) = &self.primary_group_id {
                format!("{:?}", primary_group_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "extras".into(),
            "description".into(),
            "user_id".into(),
            "primary_group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserPrivateMessagesResponseTopicListTopics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fancy_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unseen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_posts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_poster_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_globally: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_user_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posters: Option<Vec<ListUserPrivateMessagesResponseTopicListTopicsPosters>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<Participants>>,
}

impl std::fmt::Display for ListUserPrivateMessagesResponseTopicListTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserPrivateMessagesResponseTopicListTopics {
    const LENGTH: usize = 34;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(fancy_title) = &self.fancy_title {
                format!("{:?}", fancy_title).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(posts_count) = &self.posts_count {
                format!("{:?}", posts_count).into()
            } else {
                String::new().into()
            },
            if let Some(reply_count) = &self.reply_count {
                format!("{:?}", reply_count).into()
            } else {
                String::new().into()
            },
            if let Some(highest_post_number) = &self.highest_post_number {
                format!("{:?}", highest_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(image_url) = &self.image_url {
                format!("{:?}", image_url).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(last_posted_at) = &self.last_posted_at {
                format!("{:?}", last_posted_at).into()
            } else {
                String::new().into()
            },
            if let Some(bumped) = &self.bumped {
                format!("{:?}", bumped).into()
            } else {
                String::new().into()
            },
            if let Some(bumped_at) = &self.bumped_at {
                format!("{:?}", bumped_at).into()
            } else {
                String::new().into()
            },
            if let Some(archetype) = &self.archetype {
                format!("{:?}", archetype).into()
            } else {
                String::new().into()
            },
            if let Some(unseen) = &self.unseen {
                format!("{:?}", unseen).into()
            } else {
                String::new().into()
            },
            if let Some(last_read_post_number) = &self.last_read_post_number {
                format!("{:?}", last_read_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(unread_posts) = &self.unread_posts {
                format!("{:?}", unread_posts).into()
            } else {
                String::new().into()
            },
            if let Some(pinned) = &self.pinned {
                format!("{:?}", pinned).into()
            } else {
                String::new().into()
            },
            if let Some(unpinned) = &self.unpinned {
                format!("{:?}", unpinned).into()
            } else {
                String::new().into()
            },
            if let Some(visible) = &self.visible {
                format!("{:?}", visible).into()
            } else {
                String::new().into()
            },
            if let Some(closed) = &self.closed {
                format!("{:?}", closed).into()
            } else {
                String::new().into()
            },
            if let Some(archived) = &self.archived {
                format!("{:?}", archived).into()
            } else {
                String::new().into()
            },
            if let Some(notification_level) = &self.notification_level {
                format!("{:?}", notification_level).into()
            } else {
                String::new().into()
            },
            if let Some(bookmarked) = &self.bookmarked {
                format!("{:?}", bookmarked).into()
            } else {
                String::new().into()
            },
            if let Some(liked) = &self.liked {
                format!("{:?}", liked).into()
            } else {
                String::new().into()
            },
            if let Some(views) = &self.views {
                format!("{:?}", views).into()
            } else {
                String::new().into()
            },
            if let Some(like_count) = &self.like_count {
                format!("{:?}", like_count).into()
            } else {
                String::new().into()
            },
            if let Some(has_summary) = &self.has_summary {
                format!("{:?}", has_summary).into()
            } else {
                String::new().into()
            },
            if let Some(last_poster_username) = &self.last_poster_username {
                format!("{:?}", last_poster_username).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
            if let Some(pinned_globally) = &self.pinned_globally {
                format!("{:?}", pinned_globally).into()
            } else {
                String::new().into()
            },
            if let Some(featured_link) = &self.featured_link {
                format!("{:?}", featured_link).into()
            } else {
                String::new().into()
            },
            if let Some(allowed_user_count) = &self.allowed_user_count {
                format!("{:?}", allowed_user_count).into()
            } else {
                String::new().into()
            },
            if let Some(posters) = &self.posters {
                format!("{:?}", posters).into()
            } else {
                String::new().into()
            },
            if let Some(participants) = &self.participants {
                format!("{:?}", participants).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
            "reply_count".into(),
            "highest_post_number".into(),
            "image_url".into(),
            "created_at".into(),
            "last_posted_at".into(),
            "bumped".into(),
            "bumped_at".into(),
            "archetype".into(),
            "unseen".into(),
            "last_read_post_number".into(),
            "unread_posts".into(),
            "pinned".into(),
            "unpinned".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "notification_level".into(),
            "bookmarked".into(),
            "liked".into(),
            "views".into(),
            "like_count".into(),
            "has_summary".into(),
            "last_poster_username".into(),
            "category_id".into(),
            "pinned_globally".into(),
            "featured_link".into(),
            "allowed_user_count".into(),
            "posters".into(),
            "participants".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserPrivateMessagesResponseTopicList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_create_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_sequence: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<ListUserPrivateMessagesResponseTopicListTopics>>,
}

impl std::fmt::Display for ListUserPrivateMessagesResponseTopicList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserPrivateMessagesResponseTopicList {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(can_create_topic) = &self.can_create_topic {
                format!("{:?}", can_create_topic).into()
            } else {
                String::new().into()
            },
            if let Some(draft) = &self.draft {
                format!("{:?}", draft).into()
            } else {
                String::new().into()
            },
            if let Some(draft_key) = &self.draft_key {
                format!("{:?}", draft_key).into()
            } else {
                String::new().into()
            },
            if let Some(draft_sequence) = &self.draft_sequence {
                format!("{:?}", draft_sequence).into()
            } else {
                String::new().into()
            },
            if let Some(per_page) = &self.per_page {
                format!("{:?}", per_page).into()
            } else {
                String::new().into()
            },
            if let Some(topics) = &self.topics {
                format!("{:?}", topics).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_create_topic".into(),
            "draft".into(),
            "draft_key".into(),
            "draft_sequence".into(),
            "per_page".into(),
            "topics".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserPrivateMessagesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ListUserPrivateMessagesResponseUsers>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_groups: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_list: Option<ListUserPrivateMessagesResponseTopicList>,
}

impl std::fmt::Display for ListUserPrivateMessagesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserPrivateMessagesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
            if let Some(primary_groups) = &self.primary_groups {
                format!("{:?}", primary_groups).into()
            } else {
                String::new().into()
            },
            if let Some(topic_list) = &self.topic_list {
                format!("{:?}", topic_list).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "primary_groups".into(), "topic_list".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserSentPrivateMessagesResponseUsers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_template: Option<String>,
}

impl std::fmt::Display for GetUserSentPrivateMessagesResponseUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserSentPrivateMessagesResponseUsers {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(avatar_template) = &self.avatar_template {
                format!("{:?}", avatar_template).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserSentPrivateMessagesResponseTopicListTopicsPosters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
}

impl std::fmt::Display for GetUserSentPrivateMessagesResponseTopicListTopicsPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserSentPrivateMessagesResponseTopicListTopicsPosters {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(extras) = &self.extras {
                format!("{:?}", extras).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group_id) = &self.primary_group_id {
                format!("{:?}", primary_group_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "extras".into(),
            "description".into(),
            "user_id".into(),
            "primary_group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserSentPrivateMessagesResponseTopicListTopics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fancy_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unseen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_posts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_poster_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_globally: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_user_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posters: Option<Vec<GetUserSentPrivateMessagesResponseTopicListTopicsPosters>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for GetUserSentPrivateMessagesResponseTopicListTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserSentPrivateMessagesResponseTopicListTopics {
    const LENGTH: usize = 34;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(fancy_title) = &self.fancy_title {
                format!("{:?}", fancy_title).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(posts_count) = &self.posts_count {
                format!("{:?}", posts_count).into()
            } else {
                String::new().into()
            },
            if let Some(reply_count) = &self.reply_count {
                format!("{:?}", reply_count).into()
            } else {
                String::new().into()
            },
            if let Some(highest_post_number) = &self.highest_post_number {
                format!("{:?}", highest_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(image_url) = &self.image_url {
                format!("{:?}", image_url).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(last_posted_at) = &self.last_posted_at {
                format!("{:?}", last_posted_at).into()
            } else {
                String::new().into()
            },
            if let Some(bumped) = &self.bumped {
                format!("{:?}", bumped).into()
            } else {
                String::new().into()
            },
            if let Some(bumped_at) = &self.bumped_at {
                format!("{:?}", bumped_at).into()
            } else {
                String::new().into()
            },
            if let Some(archetype) = &self.archetype {
                format!("{:?}", archetype).into()
            } else {
                String::new().into()
            },
            if let Some(unseen) = &self.unseen {
                format!("{:?}", unseen).into()
            } else {
                String::new().into()
            },
            if let Some(last_read_post_number) = &self.last_read_post_number {
                format!("{:?}", last_read_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(unread_posts) = &self.unread_posts {
                format!("{:?}", unread_posts).into()
            } else {
                String::new().into()
            },
            if let Some(pinned) = &self.pinned {
                format!("{:?}", pinned).into()
            } else {
                String::new().into()
            },
            if let Some(unpinned) = &self.unpinned {
                format!("{:?}", unpinned).into()
            } else {
                String::new().into()
            },
            if let Some(visible) = &self.visible {
                format!("{:?}", visible).into()
            } else {
                String::new().into()
            },
            if let Some(closed) = &self.closed {
                format!("{:?}", closed).into()
            } else {
                String::new().into()
            },
            if let Some(archived) = &self.archived {
                format!("{:?}", archived).into()
            } else {
                String::new().into()
            },
            if let Some(notification_level) = &self.notification_level {
                format!("{:?}", notification_level).into()
            } else {
                String::new().into()
            },
            if let Some(bookmarked) = &self.bookmarked {
                format!("{:?}", bookmarked).into()
            } else {
                String::new().into()
            },
            if let Some(liked) = &self.liked {
                format!("{:?}", liked).into()
            } else {
                String::new().into()
            },
            if let Some(views) = &self.views {
                format!("{:?}", views).into()
            } else {
                String::new().into()
            },
            if let Some(like_count) = &self.like_count {
                format!("{:?}", like_count).into()
            } else {
                String::new().into()
            },
            if let Some(has_summary) = &self.has_summary {
                format!("{:?}", has_summary).into()
            } else {
                String::new().into()
            },
            if let Some(last_poster_username) = &self.last_poster_username {
                format!("{:?}", last_poster_username).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
            if let Some(pinned_globally) = &self.pinned_globally {
                format!("{:?}", pinned_globally).into()
            } else {
                String::new().into()
            },
            if let Some(featured_link) = &self.featured_link {
                format!("{:?}", featured_link).into()
            } else {
                String::new().into()
            },
            if let Some(allowed_user_count) = &self.allowed_user_count {
                format!("{:?}", allowed_user_count).into()
            } else {
                String::new().into()
            },
            if let Some(posters) = &self.posters {
                format!("{:?}", posters).into()
            } else {
                String::new().into()
            },
            if let Some(participants) = &self.participants {
                format!("{:?}", participants).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
            "reply_count".into(),
            "highest_post_number".into(),
            "image_url".into(),
            "created_at".into(),
            "last_posted_at".into(),
            "bumped".into(),
            "bumped_at".into(),
            "archetype".into(),
            "unseen".into(),
            "last_read_post_number".into(),
            "unread_posts".into(),
            "pinned".into(),
            "unpinned".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "notification_level".into(),
            "bookmarked".into(),
            "liked".into(),
            "views".into(),
            "like_count".into(),
            "has_summary".into(),
            "last_poster_username".into(),
            "category_id".into(),
            "pinned_globally".into(),
            "featured_link".into(),
            "allowed_user_count".into(),
            "posters".into(),
            "participants".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserSentPrivateMessagesResponseTopicList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_create_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_sequence: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<GetUserSentPrivateMessagesResponseTopicListTopics>>,
}

impl std::fmt::Display for GetUserSentPrivateMessagesResponseTopicList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserSentPrivateMessagesResponseTopicList {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(can_create_topic) = &self.can_create_topic {
                format!("{:?}", can_create_topic).into()
            } else {
                String::new().into()
            },
            if let Some(draft) = &self.draft {
                format!("{:?}", draft).into()
            } else {
                String::new().into()
            },
            if let Some(draft_key) = &self.draft_key {
                format!("{:?}", draft_key).into()
            } else {
                String::new().into()
            },
            if let Some(draft_sequence) = &self.draft_sequence {
                format!("{:?}", draft_sequence).into()
            } else {
                String::new().into()
            },
            if let Some(per_page) = &self.per_page {
                format!("{:?}", per_page).into()
            } else {
                String::new().into()
            },
            if let Some(topics) = &self.topics {
                format!("{:?}", topics).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_create_topic".into(),
            "draft".into(),
            "draft_key".into(),
            "draft_sequence".into(),
            "per_page".into(),
            "topics".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserSentPrivateMessagesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GetUserSentPrivateMessagesResponseUsers>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_groups: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_list: Option<GetUserSentPrivateMessagesResponseTopicList>,
}

impl std::fmt::Display for GetUserSentPrivateMessagesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserSentPrivateMessagesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
            if let Some(primary_groups) = &self.primary_groups {
                format!("{:?}", primary_groups).into()
            } else {
                String::new().into()
            },
            if let Some(topic_list) = &self.topic_list {
                format!("{:?}", topic_list).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "primary_groups".into(), "topic_list".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Extra {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for Extra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Extra {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(categories) = &self.categories {
            format!("{:?}", categories).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["categories".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GroupedSearchResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub more_posts: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub more_users: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub more_categories: Option<String>,
    pub term: String,
    pub search_log_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub more_full_page_results: Option<String>,
    pub can_create_topic: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Extra>,
    pub post_ids: Vec<serde_json::Value>,
    pub user_ids: Vec<serde_json::Value>,
    pub category_ids: Vec<serde_json::Value>,
    pub tag_ids: Vec<serde_json::Value>,
    pub group_ids: Vec<serde_json::Value>,
}

impl std::fmt::Display for GroupedSearchResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GroupedSearchResult {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.more_posts).into(),
            format!("{:?}", self.more_users).into(),
            format!("{:?}", self.more_categories).into(),
            self.term.clone().into(),
            format!("{:?}", self.search_log_id).into(),
            format!("{:?}", self.more_full_page_results).into(),
            format!("{:?}", self.can_create_topic).into(),
            format!("{:?}", self.error).into(),
            if let Some(extra) = &self.extra {
                format!("{:?}", extra).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.post_ids).into(),
            format!("{:?}", self.user_ids).into(),
            format!("{:?}", self.category_ids).into(),
            format!("{:?}", self.tag_ids).into(),
            format!("{:?}", self.group_ids).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "more_posts".into(),
            "more_users".into(),
            "more_categories".into(),
            "term".into(),
            "search_log_id".into(),
            "more_full_page_results".into(),
            "can_create_topic".into(),
            "error".into(),
            "extra".into(),
            "post_ids".into(),
            "user_ids".into(),
            "category_ids".into(),
            "tag_ids".into(),
            "group_ids".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SearchResponse {
    pub posts: Vec<serde_json::Value>,
    pub users: Vec<serde_json::Value>,
    pub categories: Vec<serde_json::Value>,
    pub tags: Vec<serde_json::Value>,
    pub groups: Vec<serde_json::Value>,
    pub grouped_search_result: GroupedSearchResult,
}

impl std::fmt::Display for SearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SearchResponse {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.posts).into(),
            format!("{:?}", self.users).into(),
            format!("{:?}", self.categories).into(),
            format!("{:?}", self.tags).into(),
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.grouped_search_result).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "posts".into(),
            "users".into(),
            "categories".into(),
            "tags".into(),
            "groups".into(),
            "grouped_search_result".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct NotificationTypes {
    pub mentioned: i64,
    pub replied: i64,
    pub quoted: i64,
    pub edited: i64,
    pub liked: i64,
    pub private_message: i64,
    pub invited_to_private_message: i64,
    pub invitee_accepted: i64,
    pub posted: i64,
    pub watching_category_or_tag: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_features: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_problems: Option<i64>,
    pub moved_post: i64,
    pub linked: i64,
    pub granted_badge: i64,
    pub invited_to_topic: i64,
    pub custom: i64,
    pub group_mentioned: i64,
    pub group_message_summary: i64,
    pub watching_first_post: i64,
    pub topic_reminder: i64,
    pub liked_consolidated: i64,
    pub linked_consolidated: i64,
    pub post_approved: i64,
    pub code_review_commit_approved: i64,
    pub membership_request_accepted: i64,
    pub membership_request_consolidated: i64,
    pub bookmark_reminder: i64,
    pub reaction: i64,
    pub votes_released: i64,
    pub event_reminder: i64,
    pub event_invitation: i64,
    pub chat_mention: i64,
    pub chat_message: i64,
    pub chat_invitation: i64,
    pub chat_group_mention: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_quoted: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_watched_thread: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub question_answer_user_commented: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_created_topic: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub following_replied: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circles_activity: Option<i64>,
}

impl std::fmt::Display for NotificationTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NotificationTypes {
    const LENGTH: usize = 44;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.mentioned).into(),
            format!("{:?}", self.replied).into(),
            format!("{:?}", self.quoted).into(),
            format!("{:?}", self.edited).into(),
            format!("{:?}", self.liked).into(),
            format!("{:?}", self.private_message).into(),
            format!("{:?}", self.invited_to_private_message).into(),
            format!("{:?}", self.invitee_accepted).into(),
            format!("{:?}", self.posted).into(),
            format!("{:?}", self.watching_category_or_tag).into(),
            if let Some(new_features) = &self.new_features {
                format!("{:?}", new_features).into()
            } else {
                String::new().into()
            },
            if let Some(admin_problems) = &self.admin_problems {
                format!("{:?}", admin_problems).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.moved_post).into(),
            format!("{:?}", self.linked).into(),
            format!("{:?}", self.granted_badge).into(),
            format!("{:?}", self.invited_to_topic).into(),
            format!("{:?}", self.custom).into(),
            format!("{:?}", self.group_mentioned).into(),
            format!("{:?}", self.group_message_summary).into(),
            format!("{:?}", self.watching_first_post).into(),
            format!("{:?}", self.topic_reminder).into(),
            format!("{:?}", self.liked_consolidated).into(),
            format!("{:?}", self.linked_consolidated).into(),
            format!("{:?}", self.post_approved).into(),
            format!("{:?}", self.code_review_commit_approved).into(),
            format!("{:?}", self.membership_request_accepted).into(),
            format!("{:?}", self.membership_request_consolidated).into(),
            format!("{:?}", self.bookmark_reminder).into(),
            format!("{:?}", self.reaction).into(),
            format!("{:?}", self.votes_released).into(),
            format!("{:?}", self.event_reminder).into(),
            format!("{:?}", self.event_invitation).into(),
            format!("{:?}", self.chat_mention).into(),
            format!("{:?}", self.chat_message).into(),
            format!("{:?}", self.chat_invitation).into(),
            format!("{:?}", self.chat_group_mention).into(),
            if let Some(chat_quoted) = &self.chat_quoted {
                format!("{:?}", chat_quoted).into()
            } else {
                String::new().into()
            },
            if let Some(chat_watched_thread) = &self.chat_watched_thread {
                format!("{:?}", chat_watched_thread).into()
            } else {
                String::new().into()
            },
            if let Some(assigned) = &self.assigned {
                format!("{:?}", assigned).into()
            } else {
                String::new().into()
            },
            if let Some(question_answer_user_commented) = &self.question_answer_user_commented {
                format!("{:?}", question_answer_user_commented).into()
            } else {
                String::new().into()
            },
            if let Some(following) = &self.following {
                format!("{:?}", following).into()
            } else {
                String::new().into()
            },
            if let Some(following_created_topic) = &self.following_created_topic {
                format!("{:?}", following_created_topic).into()
            } else {
                String::new().into()
            },
            if let Some(following_replied) = &self.following_replied {
                format!("{:?}", following_replied).into()
            } else {
                String::new().into()
            },
            if let Some(circles_activity) = &self.circles_activity {
                format!("{:?}", circles_activity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "mentioned".into(),
            "replied".into(),
            "quoted".into(),
            "edited".into(),
            "liked".into(),
            "private_message".into(),
            "invited_to_private_message".into(),
            "invitee_accepted".into(),
            "posted".into(),
            "watching_category_or_tag".into(),
            "new_features".into(),
            "admin_problems".into(),
            "moved_post".into(),
            "linked".into(),
            "granted_badge".into(),
            "invited_to_topic".into(),
            "custom".into(),
            "group_mentioned".into(),
            "group_message_summary".into(),
            "watching_first_post".into(),
            "topic_reminder".into(),
            "liked_consolidated".into(),
            "linked_consolidated".into(),
            "post_approved".into(),
            "code_review_commit_approved".into(),
            "membership_request_accepted".into(),
            "membership_request_consolidated".into(),
            "bookmark_reminder".into(),
            "reaction".into(),
            "votes_released".into(),
            "event_reminder".into(),
            "event_invitation".into(),
            "chat_mention".into(),
            "chat_message".into(),
            "chat_invitation".into(),
            "chat_group_mention".into(),
            "chat_quoted".into(),
            "chat_watched_thread".into(),
            "assigned".into(),
            "question_answer_user_commented".into(),
            "following".into(),
            "following_created_topic".into(),
            "following_replied".into(),
            "circles_activity".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostTypes {
    pub regular: i64,
    pub moderator_action: i64,
    pub small_action: i64,
    pub whisper: i64,
}

impl std::fmt::Display for PostTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostTypes {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.regular).into(),
            format!("{:?}", self.moderator_action).into(),
            format!("{:?}", self.small_action).into(),
            format!("{:?}", self.whisper).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "regular".into(),
            "moderator_action".into(),
            "small_action".into(),
            "whisper".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TrustLevels {
    pub newuser: i64,
    pub basic: i64,
    pub member: i64,
    pub regular: i64,
    pub leader: i64,
}

impl std::fmt::Display for TrustLevels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TrustLevels {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.newuser).into(),
            format!("{:?}", self.basic).into(),
            format!("{:?}", self.member).into(),
            format!("{:?}", self.regular).into(),
            format!("{:?}", self.leader).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "newuser".into(),
            "basic".into(),
            "member".into(),
            "regular".into(),
            "leader".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserTips {
    pub first_notification: i64,
    pub topic_timeline: i64,
    pub post_menu: i64,
    pub topic_notification_levels: i64,
    pub suggested_topics: i64,
}

impl std::fmt::Display for UserTips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserTips {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.first_notification).into(),
            format!("{:?}", self.topic_timeline).into(),
            format!("{:?}", self.post_menu).into(),
            format!("{:?}", self.topic_notification_levels).into(),
            format!("{:?}", self.suggested_topics).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_notification".into(),
            "topic_timeline".into(),
            "post_menu".into(),
            "topic_notification_levels".into(),
            "suggested_topics".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSiteResponseGroups {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    pub automatic: bool,
}

impl std::fmt::Display for GetSiteResponseGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetSiteResponseGroups {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.automatic).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "automatic".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostActionTypes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_key: Option<String>,
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub is_flag: bool,
    pub require_message: bool,
    pub enabled: bool,
    pub applies_to: Vec<serde_json::Value>,
    pub is_used: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    pub auto_action_type: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

impl std::fmt::Display for PostActionTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostActionTypes {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.name_key).into(),
            self.name.clone().into(),
            self.description.clone().into(),
            self.short_description.clone().into(),
            format!("{:?}", self.is_flag).into(),
            format!("{:?}", self.require_message).into(),
            format!("{:?}", self.enabled).into(),
            format!("{:?}", self.applies_to).into(),
            format!("{:?}", self.is_used).into(),
            if let Some(position) = &self.position {
                format!("{:?}", position).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.auto_action_type).into(),
            if let Some(system) = &self.system {
                format!("{:?}", system).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name_key".into(),
            "name".into(),
            "description".into(),
            "short_description".into(),
            "is_flag".into(),
            "require_message".into(),
            "enabled".into(),
            "applies_to".into(),
            "is_used".into(),
            "position".into(),
            "auto_action_type".into(),
            "system".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TopicFlagTypes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_key: Option<String>,
    pub name: String,
    pub description: String,
    pub short_description: String,
    pub is_flag: bool,
    pub require_message: bool,
    pub enabled: bool,
    pub applies_to: Vec<serde_json::Value>,
    pub is_used: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    pub auto_action_type: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
}

impl std::fmt::Display for TopicFlagTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TopicFlagTypes {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.name_key).into(),
            self.name.clone().into(),
            self.description.clone().into(),
            self.short_description.clone().into(),
            format!("{:?}", self.is_flag).into(),
            format!("{:?}", self.require_message).into(),
            format!("{:?}", self.enabled).into(),
            format!("{:?}", self.applies_to).into(),
            format!("{:?}", self.is_used).into(),
            if let Some(position) = &self.position {
                format!("{:?}", position).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.auto_action_type).into(),
            if let Some(system) = &self.system {
                format!("{:?}", system).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name_key".into(),
            "name".into(),
            "description".into(),
            "short_description".into(),
            "is_flag".into(),
            "require_message".into(),
            "enabled".into(),
            "applies_to".into(),
            "is_used".into(),
            "position".into(),
            "auto_action_type".into(),
            "system".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserThemes {
    pub theme_id: i64,
    pub name: String,
    pub default: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_scheme_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_color_scheme_id: Option<i64>,
}

impl std::fmt::Display for UserThemes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserThemes {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.theme_id).into(),
            self.name.clone().into(),
            format!("{:?}", self.default).into(),
            format!("{:?}", self.color_scheme_id).into(),
            if let Some(dark_color_scheme_id) = &self.dark_color_scheme_id {
                format!("{:?}", dark_color_scheme_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "theme_id".into(),
            "name".into(),
            "default".into(),
            "color_scheme_id".into(),
            "dark_color_scheme_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserColorSchemes {
    pub id: i64,
    pub name: String,
    pub is_dark: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme_id: Option<i64>,
    pub colors: Vec<serde_json::Value>,
}

impl std::fmt::Display for UserColorSchemes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserColorSchemes {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.is_dark).into(),
            if let Some(theme_id) = &self.theme_id {
                format!("{:?}", theme_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.colors).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "is_dark".into(),
            "theme_id".into(),
            "colors".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DefaultLightColorScheme {}

impl std::fmt::Display for DefaultLightColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DefaultLightColorScheme {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DefaultDarkColorScheme {}

impl std::fmt::Display for DefaultDarkColorScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DefaultDarkColorScheme {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CensoredRegexp {}

impl std::fmt::Display for CensoredRegexp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CensoredRegexp {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomEmojiTranslation {}

impl std::fmt::Display for CustomEmojiTranslation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomEmojiTranslation {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MarkdownAdditionalOptions {}

impl std::fmt::Display for MarkdownAdditionalOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MarkdownAdditionalOptions {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct HashtagConfigurations {}

impl std::fmt::Display for HashtagConfigurations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for HashtagConfigurations {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct HashtagIcons {}

impl std::fmt::Display for HashtagIcons {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for HashtagIcons {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSiteResponseCategoriesCustomFields {}

impl std::fmt::Display for GetSiteResponseCategoriesCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetSiteResponseCategoriesCustomFields {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSiteResponseCategories {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub text_color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    pub slug: String,
    pub topic_count: i64,
    pub post_count: i64,
    pub position: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_excerpt: Option<String>,
    pub topic_url: String,
    pub read_restricted: bool,
    pub permission: i64,
    pub notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_template: Option<String>,
    pub has_children: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcategory_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort_ascending: Option<String>,
    pub show_subcategory_list: bool,
    pub num_featured_topics: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_view: Option<String>,
    pub subcategory_list_style: String,
    pub default_top_period: String,
    pub default_list_filter: String,
    pub minimum_required_tags: i64,
    pub navigate_to_first_post_after_read: bool,
    pub allowed_tags: Vec<serde_json::Value>,
    pub allowed_tag_groups: Vec<serde_json::Value>,
    pub allow_global_tags: bool,
    pub required_tag_groups: Vec<RequiredTagGroups>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_banner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_logo_dark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_background_dark: Option<String>,
    pub can_edit: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<GetSiteResponseCategoriesCustomFields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub form_template_ids: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for GetSiteResponseCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetSiteResponseCategories {
    const LENGTH: usize = 44;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.color.clone().into(),
            self.text_color.clone().into(),
            if let Some(style_type) = &self.style_type {
                format!("{:?}", style_type).into()
            } else {
                String::new().into()
            },
            if let Some(emoji) = &self.emoji {
                format!("{:?}", emoji).into()
            } else {
                String::new().into()
            },
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
            self.slug.clone().into(),
            format!("{:?}", self.topic_count).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.position).into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(description_text) = &self.description_text {
                format!("{:?}", description_text).into()
            } else {
                String::new().into()
            },
            if let Some(description_excerpt) = &self.description_excerpt {
                format!("{:?}", description_excerpt).into()
            } else {
                String::new().into()
            },
            self.topic_url.clone().into(),
            format!("{:?}", self.read_restricted).into(),
            format!("{:?}", self.permission).into(),
            format!("{:?}", self.notification_level).into(),
            format!("{:?}", self.topic_template).into(),
            format!("{:?}", self.has_children).into(),
            format!("{:?}", self.subcategory_count).into(),
            format!("{:?}", self.sort_order).into(),
            format!("{:?}", self.sort_ascending).into(),
            format!("{:?}", self.show_subcategory_list).into(),
            format!("{:?}", self.num_featured_topics).into(),
            format!("{:?}", self.default_view).into(),
            self.subcategory_list_style.clone().into(),
            self.default_top_period.clone().into(),
            self.default_list_filter.clone().into(),
            format!("{:?}", self.minimum_required_tags).into(),
            format!("{:?}", self.navigate_to_first_post_after_read).into(),
            format!("{:?}", self.allowed_tags).into(),
            format!("{:?}", self.allowed_tag_groups).into(),
            format!("{:?}", self.allow_global_tags).into(),
            format!("{:?}", self.required_tag_groups).into(),
            format!("{:?}", self.read_only_banner).into(),
            format!("{:?}", self.uploaded_logo).into(),
            format!("{:?}", self.uploaded_logo_dark).into(),
            format!("{:?}", self.uploaded_background).into(),
            format!("{:?}", self.uploaded_background_dark).into(),
            format!("{:?}", self.can_edit).into(),
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields).into()
            } else {
                String::new().into()
            },
            if let Some(parent_category_id) = &self.parent_category_id {
                format!("{:?}", parent_category_id).into()
            } else {
                String::new().into()
            },
            if let Some(form_template_ids) = &self.form_template_ids {
                format!("{:?}", form_template_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "color".into(),
            "text_color".into(),
            "style_type".into(),
            "emoji".into(),
            "icon".into(),
            "slug".into(),
            "topic_count".into(),
            "post_count".into(),
            "position".into(),
            "description".into(),
            "description_text".into(),
            "description_excerpt".into(),
            "topic_url".into(),
            "read_restricted".into(),
            "permission".into(),
            "notification_level".into(),
            "topic_template".into(),
            "has_children".into(),
            "subcategory_count".into(),
            "sort_order".into(),
            "sort_ascending".into(),
            "show_subcategory_list".into(),
            "num_featured_topics".into(),
            "default_view".into(),
            "subcategory_list_style".into(),
            "default_top_period".into(),
            "default_list_filter".into(),
            "minimum_required_tags".into(),
            "navigate_to_first_post_after_read".into(),
            "allowed_tags".into(),
            "allowed_tag_groups".into(),
            "allow_global_tags".into(),
            "required_tag_groups".into(),
            "read_only_banner".into(),
            "uploaded_logo".into(),
            "uploaded_logo_dark".into(),
            "uploaded_background".into(),
            "uploaded_background_dark".into(),
            "can_edit".into(),
            "custom_fields".into(),
            "parent_category_id".into(),
            "form_template_ids".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Archetypes {
    pub id: String,
    pub name: String,
    pub options: Vec<serde_json::Value>,
}

impl std::fmt::Display for Archetypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Archetypes {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.name.clone().into(),
            format!("{:?}", self.options).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into(), "options".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSiteResponse {
    pub default_archetype: String,
    pub notification_types: NotificationTypes,
    pub post_types: PostTypes,
    pub trust_levels: TrustLevels,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_tips: Option<UserTips>,
    pub groups: Vec<GetSiteResponseGroups>,
    pub filters: Vec<serde_json::Value>,
    pub periods: Vec<serde_json::Value>,
    pub top_menu_items: Vec<serde_json::Value>,
    pub anonymous_top_menu_items: Vec<serde_json::Value>,
    pub uncategorized_category_id: i64,
    pub user_field_max_length: i64,
    pub post_action_types: Vec<PostActionTypes>,
    pub topic_flag_types: Vec<TopicFlagTypes>,
    pub can_create_tag: bool,
    pub can_tag_topics: bool,
    pub can_tag_pms: bool,
    pub tags_filter_regexp: String,
    pub top_tags: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wizard_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_associate_groups: Option<bool>,
    pub topic_featured_link_allowed_category_ids: Vec<serde_json::Value>,
    pub user_themes: Vec<UserThemes>,
    pub user_color_schemes: Vec<UserColorSchemes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_light_color_scheme: Option<DefaultLightColorScheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_dark_color_scheme: Option<DefaultDarkColorScheme>,
    pub censored_regexp: Vec<CensoredRegexp>,
    pub custom_emoji_translation: CustomEmojiTranslation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watched_words_replace: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watched_words_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markdown_additional_options: Option<MarkdownAdditionalOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashtag_configurations: Option<HashtagConfigurations>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hashtag_icons: Option<HashtagIcons>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub displayed_about_plugin_stat_groups: Option<Vec<serde_json::Value>>,
    pub categories: Vec<GetSiteResponseCategories>,
    pub archetypes: Vec<Archetypes>,
    pub user_fields: Vec<serde_json::Value>,
    pub auth_providers: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub whispers_allowed_groups_names: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denied_emojis: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_flag_applies_to_types: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub navigation_menu_site_top_tags: Option<Vec<serde_json::Value>>,
    pub full_name_required_for_signup: bool,
    pub full_name_visible_in_signup: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_config_login_routes: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for GetSiteResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetSiteResponse {
    const LENGTH: usize = 45;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.default_archetype.clone().into(),
            format!("{:?}", self.notification_types).into(),
            format!("{:?}", self.post_types).into(),
            format!("{:?}", self.trust_levels).into(),
            if let Some(user_tips) = &self.user_tips {
                format!("{:?}", user_tips).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.filters).into(),
            format!("{:?}", self.periods).into(),
            format!("{:?}", self.top_menu_items).into(),
            format!("{:?}", self.anonymous_top_menu_items).into(),
            format!("{:?}", self.uncategorized_category_id).into(),
            format!("{:?}", self.user_field_max_length).into(),
            format!("{:?}", self.post_action_types).into(),
            format!("{:?}", self.topic_flag_types).into(),
            format!("{:?}", self.can_create_tag).into(),
            format!("{:?}", self.can_tag_topics).into(),
            format!("{:?}", self.can_tag_pms).into(),
            self.tags_filter_regexp.clone().into(),
            format!("{:?}", self.top_tags).into(),
            if let Some(wizard_required) = &self.wizard_required {
                format!("{:?}", wizard_required).into()
            } else {
                String::new().into()
            },
            if let Some(can_associate_groups) = &self.can_associate_groups {
                format!("{:?}", can_associate_groups).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.topic_featured_link_allowed_category_ids).into(),
            format!("{:?}", self.user_themes).into(),
            format!("{:?}", self.user_color_schemes).into(),
            format!("{:?}", self.default_light_color_scheme).into(),
            format!("{:?}", self.default_dark_color_scheme).into(),
            format!("{:?}", self.censored_regexp).into(),
            format!("{:?}", self.custom_emoji_translation).into(),
            format!("{:?}", self.watched_words_replace).into(),
            format!("{:?}", self.watched_words_link).into(),
            if let Some(markdown_additional_options) = &self.markdown_additional_options {
                format!("{:?}", markdown_additional_options).into()
            } else {
                String::new().into()
            },
            if let Some(hashtag_configurations) = &self.hashtag_configurations {
                format!("{:?}", hashtag_configurations).into()
            } else {
                String::new().into()
            },
            if let Some(hashtag_icons) = &self.hashtag_icons {
                format!("{:?}", hashtag_icons).into()
            } else {
                String::new().into()
            },
            if let Some(displayed_about_plugin_stat_groups) =
                &self.displayed_about_plugin_stat_groups
            {
                format!("{:?}", displayed_about_plugin_stat_groups).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.categories).into(),
            format!("{:?}", self.archetypes).into(),
            format!("{:?}", self.user_fields).into(),
            format!("{:?}", self.auth_providers).into(),
            if let Some(whispers_allowed_groups_names) = &self.whispers_allowed_groups_names {
                format!("{:?}", whispers_allowed_groups_names).into()
            } else {
                String::new().into()
            },
            if let Some(denied_emojis) = &self.denied_emojis {
                format!("{:?}", denied_emojis).into()
            } else {
                String::new().into()
            },
            if let Some(valid_flag_applies_to_types) = &self.valid_flag_applies_to_types {
                format!("{:?}", valid_flag_applies_to_types).into()
            } else {
                String::new().into()
            },
            if let Some(navigation_menu_site_top_tags) = &self.navigation_menu_site_top_tags {
                format!("{:?}", navigation_menu_site_top_tags).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.full_name_required_for_signup).into(),
            format!("{:?}", self.full_name_visible_in_signup).into(),
            if let Some(admin_config_login_routes) = &self.admin_config_login_routes {
                format!("{:?}", admin_config_login_routes).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "default_archetype".into(),
            "notification_types".into(),
            "post_types".into(),
            "trust_levels".into(),
            "user_tips".into(),
            "groups".into(),
            "filters".into(),
            "periods".into(),
            "top_menu_items".into(),
            "anonymous_top_menu_items".into(),
            "uncategorized_category_id".into(),
            "user_field_max_length".into(),
            "post_action_types".into(),
            "topic_flag_types".into(),
            "can_create_tag".into(),
            "can_tag_topics".into(),
            "can_tag_pms".into(),
            "tags_filter_regexp".into(),
            "top_tags".into(),
            "wizard_required".into(),
            "can_associate_groups".into(),
            "topic_featured_link_allowed_category_ids".into(),
            "user_themes".into(),
            "user_color_schemes".into(),
            "default_light_color_scheme".into(),
            "default_dark_color_scheme".into(),
            "censored_regexp".into(),
            "custom_emoji_translation".into(),
            "watched_words_replace".into(),
            "watched_words_link".into(),
            "markdown_additional_options".into(),
            "hashtag_configurations".into(),
            "hashtag_icons".into(),
            "displayed_about_plugin_stat_groups".into(),
            "categories".into(),
            "archetypes".into(),
            "user_fields".into(),
            "auth_providers".into(),
            "whispers_allowed_groups_names".into(),
            "denied_emojis".into(),
            "valid_flag_applies_to_types".into(),
            "navigation_menu_site_top_tags".into(),
            "full_name_required_for_signup".into(),
            "full_name_visible_in_signup".into(),
            "admin_config_login_routes".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSiteBasicInfoResponse {
    pub logo_url: String,
    pub logo_small_url: String,
    pub apple_touch_icon_url: String,
    pub favicon_url: String,
    pub title: String,
    pub description: String,
    pub header_primary_color: String,
    pub header_background_color: String,
    pub login_required: bool,
    pub locale: String,
    pub include_in_discourse_discover: bool,
    pub mobile_logo_url: String,
}

impl std::fmt::Display for GetSiteBasicInfoResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetSiteBasicInfoResponse {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.logo_url.clone().into(),
            self.logo_small_url.clone().into(),
            self.apple_touch_icon_url.clone().into(),
            self.favicon_url.clone().into(),
            self.title.clone().into(),
            self.description.clone().into(),
            self.header_primary_color.clone().into(),
            self.header_background_color.clone().into(),
            format!("{:?}", self.login_required).into(),
            self.locale.clone().into(),
            format!("{:?}", self.include_in_discourse_discover).into(),
            self.mobile_logo_url.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "logo_url".into(),
            "logo_small_url".into(),
            "apple_touch_icon_url".into(),
            "favicon_url".into(),
            "title".into(),
            "description".into(),
            "header_primary_color".into(),
            "header_background_color".into(),
            "login_required".into(),
            "locale".into(),
            "include_in_discourse_discover".into(),
            "mobile_logo_url".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TagGroups {
    pub id: i64,
    pub name: String,
    pub tag_names: Vec<String>,
    pub parent_tag_name: Vec<String>,
    pub one_per_topic: bool,
    pub permissions: std::collections::HashMap<String, i64>,
}

impl std::fmt::Display for TagGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TagGroups {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.tag_names).into(),
            format!("{:?}", self.parent_tag_name).into(),
            format!("{:?}", self.one_per_topic).into(),
            format!("{:?}", self.permissions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "tag_names".into(),
            "parent_tag_name".into(),
            "one_per_topic".into(),
            "permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTagGroupsResponse {
    pub tag_groups: Vec<TagGroups>,
}

impl std::fmt::Display for ListTagGroupsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTagGroupsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.tag_groups).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["tag_groups".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTagGroupRequestBody {
    pub name: String,
}

impl std::fmt::Display for CreateTagGroupRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateTagGroupRequestBody {
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
pub struct TagGroupPermissions {}

impl std::fmt::Display for TagGroupPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TagGroupPermissions {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TagGroup {
    pub id: i64,
    pub name: String,
    pub tag_names: Vec<serde_json::Value>,
    pub parent_tag_name: Vec<serde_json::Value>,
    pub one_per_topic: bool,
    pub permissions: TagGroupPermissions,
}

impl std::fmt::Display for TagGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TagGroup {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            format!("{:?}", self.tag_names).into(),
            format!("{:?}", self.parent_tag_name).into(),
            format!("{:?}", self.one_per_topic).into(),
            format!("{:?}", self.permissions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "tag_names".into(),
            "parent_tag_name".into(),
            "one_per_topic".into(),
            "permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTagGroupResponse {
    pub tag_group: TagGroup,
}

impl std::fmt::Display for CreateTagGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateTagGroupResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.tag_group).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["tag_group".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagGroupResponseTagGroupPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everyone: Option<i64>,
}

impl std::fmt::Display for GetTagGroupResponseTagGroupPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagGroupResponseTagGroupPermissions {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(everyone) = &self.everyone {
            format!("{:?}", everyone).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["everyone".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagGroupResponseTagGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_names: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_tag_name: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub one_per_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<GetTagGroupResponseTagGroupPermissions>,
}

impl std::fmt::Display for GetTagGroupResponseTagGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagGroupResponseTagGroup {
    const LENGTH: usize = 6;
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
            if let Some(tag_names) = &self.tag_names {
                format!("{:?}", tag_names).into()
            } else {
                String::new().into()
            },
            if let Some(parent_tag_name) = &self.parent_tag_name {
                format!("{:?}", parent_tag_name).into()
            } else {
                String::new().into()
            },
            if let Some(one_per_topic) = &self.one_per_topic {
                format!("{:?}", one_per_topic).into()
            } else {
                String::new().into()
            },
            if let Some(permissions) = &self.permissions {
                format!("{:?}", permissions).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "tag_names".into(),
            "parent_tag_name".into(),
            "one_per_topic".into(),
            "permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagGroupResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_group: Option<GetTagGroupResponseTagGroup>,
}

impl std::fmt::Display for GetTagGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagGroupResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(tag_group) = &self.tag_group {
            format!("{:?}", tag_group).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["tag_group".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTagGroupRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for UpdateTagGroupRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTagGroupRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(name) = &self.name {
            format!("{:?}", name).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTagGroupResponseTagGroupPermissions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everyone: Option<i64>,
}

impl std::fmt::Display for UpdateTagGroupResponseTagGroupPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTagGroupResponseTagGroupPermissions {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(everyone) = &self.everyone {
            format!("{:?}", everyone).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["everyone".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTagGroupResponseTagGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_names: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_tag_name: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub one_per_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<UpdateTagGroupResponseTagGroupPermissions>,
}

impl std::fmt::Display for UpdateTagGroupResponseTagGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTagGroupResponseTagGroup {
    const LENGTH: usize = 6;
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
            if let Some(tag_names) = &self.tag_names {
                format!("{:?}", tag_names).into()
            } else {
                String::new().into()
            },
            if let Some(parent_tag_name) = &self.parent_tag_name {
                format!("{:?}", parent_tag_name).into()
            } else {
                String::new().into()
            },
            if let Some(one_per_topic) = &self.one_per_topic {
                format!("{:?}", one_per_topic).into()
            } else {
                String::new().into()
            },
            if let Some(permissions) = &self.permissions {
                format!("{:?}", permissions).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "tag_names".into(),
            "parent_tag_name".into(),
            "one_per_topic".into(),
            "permissions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTagGroupResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_group: Option<UpdateTagGroupResponseTagGroup>,
}

impl std::fmt::Display for UpdateTagGroupResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTagGroupResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(success) = &self.success {
                format!("{:?}", success).into()
            } else {
                String::new().into()
            },
            if let Some(tag_group) = &self.tag_group {
                format!("{:?}", tag_group).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into(), "tag_group".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Tags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pm_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_tag: Option<String>,
}

impl std::fmt::Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Tags {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(text) = &self.text {
                format!("{:?}", text).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(count) = &self.count {
                format!("{:?}", count).into()
            } else {
                String::new().into()
            },
            if let Some(pm_count) = &self.pm_count {
                format!("{:?}", pm_count).into()
            } else {
                String::new().into()
            },
            if let Some(target_tag) = &self.target_tag {
                format!("{:?}", target_tag).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "text".into(),
            "name".into(),
            "count".into(),
            "pm_count".into(),
            "target_tag".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTagsResponseExtras {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<serde_json::Value>>,
}

impl std::fmt::Display for ListTagsResponseExtras {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTagsResponseExtras {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(categories) = &self.categories {
            format!("{:?}", categories).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["categories".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTagsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tags>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<ListTagsResponseExtras>,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTagsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tags) = &self.tags {
                format!("{:?}", tags).into()
            } else {
                String::new().into()
            },
            if let Some(extras) = &self.extras {
                format!("{:?}", extras).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["tags".into(), "extras".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagResponseUsers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_template: Option<String>,
}

impl std::fmt::Display for GetTagResponseUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagResponseUsers {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(avatar_template) = &self.avatar_template {
                format!("{:?}", avatar_template).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagResponseTopicListTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staff: Option<bool>,
}

impl std::fmt::Display for GetTagResponseTopicListTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagResponseTopicListTags {
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
            if let Some(topic_count) = &self.topic_count {
                format!("{:?}", topic_count).into()
            } else {
                String::new().into()
            },
            if let Some(staff) = &self.staff {
                format!("{:?}", staff).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "topic_count".into(),
            "staff".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagResponseTopicListTopicsPosters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
}

impl std::fmt::Display for GetTagResponseTopicListTopicsPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagResponseTopicListTopicsPosters {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(extras) = &self.extras {
                format!("{:?}", extras).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group_id) = &self.primary_group_id {
                format!("{:?}", primary_group_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "extras".into(),
            "description".into(),
            "user_id".into(),
            "primary_group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagResponseTopicListTopics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fancy_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unseen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_posts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_poster_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_globally: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posters: Option<Vec<GetTagResponseTopicListTopicsPosters>>,
}

impl std::fmt::Display for GetTagResponseTopicListTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagResponseTopicListTopics {
    const LENGTH: usize = 33;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(fancy_title) = &self.fancy_title {
                format!("{:?}", fancy_title).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(posts_count) = &self.posts_count {
                format!("{:?}", posts_count).into()
            } else {
                String::new().into()
            },
            if let Some(reply_count) = &self.reply_count {
                format!("{:?}", reply_count).into()
            } else {
                String::new().into()
            },
            if let Some(highest_post_number) = &self.highest_post_number {
                format!("{:?}", highest_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(image_url) = &self.image_url {
                format!("{:?}", image_url).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(last_posted_at) = &self.last_posted_at {
                format!("{:?}", last_posted_at).into()
            } else {
                String::new().into()
            },
            if let Some(bumped) = &self.bumped {
                format!("{:?}", bumped).into()
            } else {
                String::new().into()
            },
            if let Some(bumped_at) = &self.bumped_at {
                format!("{:?}", bumped_at).into()
            } else {
                String::new().into()
            },
            if let Some(archetype) = &self.archetype {
                format!("{:?}", archetype).into()
            } else {
                String::new().into()
            },
            if let Some(unseen) = &self.unseen {
                format!("{:?}", unseen).into()
            } else {
                String::new().into()
            },
            if let Some(last_read_post_number) = &self.last_read_post_number {
                format!("{:?}", last_read_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(unread_posts) = &self.unread_posts {
                format!("{:?}", unread_posts).into()
            } else {
                String::new().into()
            },
            if let Some(pinned) = &self.pinned {
                format!("{:?}", pinned).into()
            } else {
                String::new().into()
            },
            if let Some(unpinned) = &self.unpinned {
                format!("{:?}", unpinned).into()
            } else {
                String::new().into()
            },
            if let Some(visible) = &self.visible {
                format!("{:?}", visible).into()
            } else {
                String::new().into()
            },
            if let Some(closed) = &self.closed {
                format!("{:?}", closed).into()
            } else {
                String::new().into()
            },
            if let Some(archived) = &self.archived {
                format!("{:?}", archived).into()
            } else {
                String::new().into()
            },
            if let Some(notification_level) = &self.notification_level {
                format!("{:?}", notification_level).into()
            } else {
                String::new().into()
            },
            if let Some(bookmarked) = &self.bookmarked {
                format!("{:?}", bookmarked).into()
            } else {
                String::new().into()
            },
            if let Some(liked) = &self.liked {
                format!("{:?}", liked).into()
            } else {
                String::new().into()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags).into()
            } else {
                String::new().into()
            },
            if let Some(views) = &self.views {
                format!("{:?}", views).into()
            } else {
                String::new().into()
            },
            if let Some(like_count) = &self.like_count {
                format!("{:?}", like_count).into()
            } else {
                String::new().into()
            },
            if let Some(has_summary) = &self.has_summary {
                format!("{:?}", has_summary).into()
            } else {
                String::new().into()
            },
            if let Some(last_poster_username) = &self.last_poster_username {
                format!("{:?}", last_poster_username).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
            if let Some(pinned_globally) = &self.pinned_globally {
                format!("{:?}", pinned_globally).into()
            } else {
                String::new().into()
            },
            if let Some(featured_link) = &self.featured_link {
                format!("{:?}", featured_link).into()
            } else {
                String::new().into()
            },
            if let Some(posters) = &self.posters {
                format!("{:?}", posters).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
            "reply_count".into(),
            "highest_post_number".into(),
            "image_url".into(),
            "created_at".into(),
            "last_posted_at".into(),
            "bumped".into(),
            "bumped_at".into(),
            "archetype".into(),
            "unseen".into(),
            "last_read_post_number".into(),
            "unread_posts".into(),
            "pinned".into(),
            "unpinned".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "notification_level".into(),
            "bookmarked".into(),
            "liked".into(),
            "tags".into(),
            "views".into(),
            "like_count".into(),
            "has_summary".into(),
            "last_poster_username".into(),
            "category_id".into(),
            "pinned_globally".into(),
            "featured_link".into(),
            "posters".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagResponseTopicList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_create_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_sequence: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GetTagResponseTopicListTags>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<GetTagResponseTopicListTopics>>,
}

impl std::fmt::Display for GetTagResponseTopicList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagResponseTopicList {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(can_create_topic) = &self.can_create_topic {
                format!("{:?}", can_create_topic).into()
            } else {
                String::new().into()
            },
            if let Some(draft) = &self.draft {
                format!("{:?}", draft).into()
            } else {
                String::new().into()
            },
            if let Some(draft_key) = &self.draft_key {
                format!("{:?}", draft_key).into()
            } else {
                String::new().into()
            },
            if let Some(draft_sequence) = &self.draft_sequence {
                format!("{:?}", draft_sequence).into()
            } else {
                String::new().into()
            },
            if let Some(per_page) = &self.per_page {
                format!("{:?}", per_page).into()
            } else {
                String::new().into()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags).into()
            } else {
                String::new().into()
            },
            if let Some(topics) = &self.topics {
                format!("{:?}", topics).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_create_topic".into(),
            "draft".into(),
            "draft_key".into(),
            "draft_sequence".into(),
            "per_page".into(),
            "tags".into(),
            "topics".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTagResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<GetTagResponseUsers>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_groups: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_list: Option<GetTagResponseTopicList>,
}

impl std::fmt::Display for GetTagResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTagResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
            if let Some(primary_groups) = &self.primary_groups {
                format!("{:?}", primary_groups).into()
            } else {
                String::new().into()
            },
            if let Some(topic_list) = &self.topic_list {
                format!("{:?}", topic_list).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "primary_groups".into(), "topic_list".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSpecificPostsFromTopicRequestBody {
    #[serde(rename = "post_ids[]")]
    pub post_ids: i64,
}

impl std::fmt::Display for GetSpecificPostsFromTopicRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetSpecificPostsFromTopicRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.post_ids).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["post_ids".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostsActionsSummary {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_act: Option<bool>,
}

impl std::fmt::Display for PostsActionsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostsActionsSummary {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(can_act) = &self.can_act {
                format!("{:?}", can_act).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "can_act".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Posts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_template: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_type: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_link_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reads: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readers_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yours: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_recover: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_wiki: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions_summary: Option<Vec<PostsActionsSummary>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderator: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staff: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_deleted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_view_edit_history: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wiki: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_score_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewable_score_pending_count: Option<i64>,
}

impl std::fmt::Display for Posts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Posts {
    const LENGTH: usize = 47;
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
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
            if let Some(avatar_template) = &self.avatar_template {
                format!("{:?}", avatar_template).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(cooked) = &self.cooked {
                format!("{:?}", cooked).into()
            } else {
                String::new().into()
            },
            if let Some(post_number) = &self.post_number {
                format!("{:?}", post_number).into()
            } else {
                String::new().into()
            },
            if let Some(post_type) = &self.post_type {
                format!("{:?}", post_type).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(reply_count) = &self.reply_count {
                format!("{:?}", reply_count).into()
            } else {
                String::new().into()
            },
            if let Some(reply_to_post_number) = &self.reply_to_post_number {
                format!("{:?}", reply_to_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(quote_count) = &self.quote_count {
                format!("{:?}", quote_count).into()
            } else {
                String::new().into()
            },
            if let Some(incoming_link_count) = &self.incoming_link_count {
                format!("{:?}", incoming_link_count).into()
            } else {
                String::new().into()
            },
            if let Some(reads) = &self.reads {
                format!("{:?}", reads).into()
            } else {
                String::new().into()
            },
            if let Some(readers_count) = &self.readers_count {
                format!("{:?}", readers_count).into()
            } else {
                String::new().into()
            },
            if let Some(score) = &self.score {
                format!("{:?}", score).into()
            } else {
                String::new().into()
            },
            if let Some(yours) = &self.yours {
                format!("{:?}", yours).into()
            } else {
                String::new().into()
            },
            if let Some(topic_id) = &self.topic_id {
                format!("{:?}", topic_id).into()
            } else {
                String::new().into()
            },
            if let Some(topic_slug) = &self.topic_slug {
                format!("{:?}", topic_slug).into()
            } else {
                String::new().into()
            },
            if let Some(display_username) = &self.display_username {
                format!("{:?}", display_username).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group_name) = &self.primary_group_name {
                format!("{:?}", primary_group_name).into()
            } else {
                String::new().into()
            },
            if let Some(flair_name) = &self.flair_name {
                format!("{:?}", flair_name).into()
            } else {
                String::new().into()
            },
            if let Some(flair_url) = &self.flair_url {
                format!("{:?}", flair_url).into()
            } else {
                String::new().into()
            },
            if let Some(flair_bg_color) = &self.flair_bg_color {
                format!("{:?}", flair_bg_color).into()
            } else {
                String::new().into()
            },
            if let Some(flair_color) = &self.flair_color {
                format!("{:?}", flair_color).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(can_edit) = &self.can_edit {
                format!("{:?}", can_edit).into()
            } else {
                String::new().into()
            },
            if let Some(can_delete) = &self.can_delete {
                format!("{:?}", can_delete).into()
            } else {
                String::new().into()
            },
            if let Some(can_recover) = &self.can_recover {
                format!("{:?}", can_recover).into()
            } else {
                String::new().into()
            },
            if let Some(can_wiki) = &self.can_wiki {
                format!("{:?}", can_wiki).into()
            } else {
                String::new().into()
            },
            if let Some(read) = &self.read {
                format!("{:?}", read).into()
            } else {
                String::new().into()
            },
            if let Some(user_title) = &self.user_title {
                format!("{:?}", user_title).into()
            } else {
                String::new().into()
            },
            if let Some(actions_summary) = &self.actions_summary {
                format!("{:?}", actions_summary).into()
            } else {
                String::new().into()
            },
            if let Some(moderator) = &self.moderator {
                format!("{:?}", moderator).into()
            } else {
                String::new().into()
            },
            if let Some(admin) = &self.admin {
                format!("{:?}", admin).into()
            } else {
                String::new().into()
            },
            if let Some(staff) = &self.staff {
                format!("{:?}", staff).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(hidden) = &self.hidden {
                format!("{:?}", hidden).into()
            } else {
                String::new().into()
            },
            if let Some(trust_level) = &self.trust_level {
                format!("{:?}", trust_level).into()
            } else {
                String::new().into()
            },
            if let Some(deleted_at) = &self.deleted_at {
                format!("{:?}", deleted_at).into()
            } else {
                String::new().into()
            },
            if let Some(user_deleted) = &self.user_deleted {
                format!("{:?}", user_deleted).into()
            } else {
                String::new().into()
            },
            if let Some(edit_reason) = &self.edit_reason {
                format!("{:?}", edit_reason).into()
            } else {
                String::new().into()
            },
            if let Some(can_view_edit_history) = &self.can_view_edit_history {
                format!("{:?}", can_view_edit_history).into()
            } else {
                String::new().into()
            },
            if let Some(wiki) = &self.wiki {
                format!("{:?}", wiki).into()
            } else {
                String::new().into()
            },
            if let Some(reviewable_id) = &self.reviewable_id {
                format!("{:?}", reviewable_id).into()
            } else {
                String::new().into()
            },
            if let Some(reviewable_score_count) = &self.reviewable_score_count {
                format!("{:?}", reviewable_score_count).into()
            } else {
                String::new().into()
            },
            if let Some(reviewable_score_pending_count) = &self.reviewable_score_pending_count {
                format!("{:?}", reviewable_score_pending_count).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "display_username".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_wiki".into(),
            "read".into(),
            "user_title".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostStream {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts: Option<Vec<Posts>>,
}

impl std::fmt::Display for PostStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostStream {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(posts) = &self.posts {
            format!("{:?}", posts).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["posts".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetSpecificPostsFromTopicResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_stream: Option<PostStream>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}

impl std::fmt::Display for GetSpecificPostsFromTopicResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetSpecificPostsFromTopicResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(post_stream) = &self.post_stream {
                format!("{:?}", post_stream).into()
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
        vec!["post_stream".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LinkCounts {
    pub url: String,
    pub internal: bool,
    pub reflection: bool,
    pub title: String,
    pub clicks: i64,
}

impl std::fmt::Display for LinkCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LinkCounts {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.url.clone().into(),
            format!("{:?}", self.internal).into(),
            format!("{:?}", self.reflection).into(),
            self.title.clone().into(),
            format!("{:?}", self.clicks).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "url".into(),
            "internal".into(),
            "reflection".into(),
            "title".into(),
            "clicks".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTopicResponsePostStreamPosts {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub avatar_template: String,
    pub created_at: String,
    pub cooked: String,
    pub post_number: i64,
    pub post_type: i64,
    pub updated_at: String,
    pub reply_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_to_post_number: Option<String>,
    pub quote_count: i64,
    pub incoming_link_count: i64,
    pub reads: i64,
    pub readers_count: i64,
    pub score: f64,
    pub yours: bool,
    pub topic_id: i64,
    pub topic_slug: String,
    pub display_username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    pub version: i64,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_recover: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_see_hidden_post: Option<bool>,
    pub can_wiki: bool,
    pub link_counts: Vec<LinkCounts>,
    pub read: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_title: Option<String>,
    pub bookmarked: bool,
    pub actions_summary: Vec<ActionsSummary>,
    pub moderator: bool,
    pub admin: bool,
    pub staff: bool,
    pub user_id: i64,
    pub hidden: bool,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub user_deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_reason: Option<String>,
    pub can_view_edit_history: bool,
    pub wiki: bool,
    pub reviewable_id: i64,
    pub reviewable_score_count: i64,
    pub reviewable_score_pending_count: i64,
}

impl std::fmt::Display for GetTopicResponsePostStreamPosts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTopicResponsePostStreamPosts {
    const LENGTH: usize = 50;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.username.clone().into(),
            self.avatar_template.clone().into(),
            self.created_at.clone().into(),
            self.cooked.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_type).into(),
            self.updated_at.clone().into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.reply_to_post_number).into(),
            format!("{:?}", self.quote_count).into(),
            format!("{:?}", self.incoming_link_count).into(),
            format!("{:?}", self.reads).into(),
            format!("{:?}", self.readers_count).into(),
            format!("{:?}", self.score).into(),
            format!("{:?}", self.yours).into(),
            format!("{:?}", self.topic_id).into(),
            self.topic_slug.clone().into(),
            self.display_username.clone().into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.version).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_recover).into(),
            if let Some(can_see_hidden_post) = &self.can_see_hidden_post {
                format!("{:?}", can_see_hidden_post).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_wiki).into(),
            format!("{:?}", self.link_counts).into(),
            format!("{:?}", self.read).into(),
            format!("{:?}", self.user_title).into(),
            format!("{:?}", self.bookmarked).into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.staff).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_deleted).into(),
            format!("{:?}", self.edit_reason).into(),
            format!("{:?}", self.can_view_edit_history).into(),
            format!("{:?}", self.wiki).into(),
            format!("{:?}", self.reviewable_id).into(),
            format!("{:?}", self.reviewable_score_count).into(),
            format!("{:?}", self.reviewable_score_pending_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "username".into(),
            "avatar_template".into(),
            "created_at".into(),
            "cooked".into(),
            "post_number".into(),
            "post_type".into(),
            "updated_at".into(),
            "reply_count".into(),
            "reply_to_post_number".into(),
            "quote_count".into(),
            "incoming_link_count".into(),
            "reads".into(),
            "readers_count".into(),
            "score".into(),
            "yours".into(),
            "topic_id".into(),
            "topic_slug".into(),
            "display_username".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "version".into(),
            "can_edit".into(),
            "can_delete".into(),
            "can_recover".into(),
            "can_see_hidden_post".into(),
            "can_wiki".into(),
            "link_counts".into(),
            "read".into(),
            "user_title".into(),
            "bookmarked".into(),
            "actions_summary".into(),
            "moderator".into(),
            "admin".into(),
            "staff".into(),
            "user_id".into(),
            "hidden".into(),
            "trust_level".into(),
            "deleted_at".into(),
            "user_deleted".into(),
            "edit_reason".into(),
            "can_view_edit_history".into(),
            "wiki".into(),
            "reviewable_id".into(),
            "reviewable_score_count".into(),
            "reviewable_score_pending_count".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTopicResponsePostStream {
    pub posts: Vec<GetTopicResponsePostStreamPosts>,
    pub stream: Vec<serde_json::Value>,
}

impl std::fmt::Display for GetTopicResponsePostStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTopicResponsePostStream {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.posts).into(),
            format!("{:?}", self.stream).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["posts".into(), "stream".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SuggestedTopicsTagsDescriptions {}

impl std::fmt::Display for SuggestedTopicsTagsDescriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SuggestedTopicsTagsDescriptions {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
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
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SuggestedTopicsPosters {
    pub extras: String,
    pub description: String,
    pub user: User,
}

impl std::fmt::Display for SuggestedTopicsPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SuggestedTopicsPosters {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.extras.clone().into(),
            self.description.clone().into(),
            format!("{:?}", self.user).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["extras".into(), "description".into(), "user".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SuggestedTopics {
    pub id: i64,
    pub title: String,
    pub fancy_title: String,
    pub slug: String,
    pub posts_count: i64,
    pub reply_count: i64,
    pub highest_post_number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    pub bumped: bool,
    pub bumped_at: String,
    pub archetype: String,
    pub unseen: bool,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<String>,
    pub excerpt: String,
    pub visible: bool,
    pub closed: bool,
    pub archived: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liked: Option<String>,
    pub tags: Vec<serde_json::Value>,
    pub tags_descriptions: SuggestedTopicsTagsDescriptions,
    pub like_count: i64,
    pub views: i64,
    pub category_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    pub posters: Vec<SuggestedTopicsPosters>,
}

impl std::fmt::Display for SuggestedTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SuggestedTopics {
    const LENGTH: usize = 29;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.title.clone().into(),
            self.fancy_title.clone().into(),
            self.slug.clone().into(),
            format!("{:?}", self.posts_count).into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.highest_post_number).into(),
            format!("{:?}", self.image_url).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.last_posted_at).into(),
            format!("{:?}", self.bumped).into(),
            self.bumped_at.clone().into(),
            self.archetype.clone().into(),
            format!("{:?}", self.unseen).into(),
            format!("{:?}", self.pinned).into(),
            format!("{:?}", self.unpinned).into(),
            self.excerpt.clone().into(),
            format!("{:?}", self.visible).into(),
            format!("{:?}", self.closed).into(),
            format!("{:?}", self.archived).into(),
            format!("{:?}", self.bookmarked).into(),
            format!("{:?}", self.liked).into(),
            format!("{:?}", self.tags).into(),
            format!("{:?}", self.tags_descriptions).into(),
            format!("{:?}", self.like_count).into(),
            format!("{:?}", self.views).into(),
            format!("{:?}", self.category_id).into(),
            format!("{:?}", self.featured_link).into(),
            format!("{:?}", self.posters).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
            "reply_count".into(),
            "highest_post_number".into(),
            "image_url".into(),
            "created_at".into(),
            "last_posted_at".into(),
            "bumped".into(),
            "bumped_at".into(),
            "archetype".into(),
            "unseen".into(),
            "pinned".into(),
            "unpinned".into(),
            "excerpt".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "bookmarked".into(),
            "liked".into(),
            "tags".into(),
            "tags_descriptions".into(),
            "like_count".into(),
            "views".into(),
            "category_id".into(),
            "featured_link".into(),
            "posters".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTopicResponseTagsDescriptions {}

impl std::fmt::Display for GetTopicResponseTagsDescriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTopicResponseTagsDescriptions {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTopicResponseActionsSummary {
    pub id: i64,
    pub count: i64,
    pub hidden: bool,
    pub can_act: bool,
}

impl std::fmt::Display for GetTopicResponseActionsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTopicResponseActionsSummary {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.count).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.can_act).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "count".into(),
            "hidden".into(),
            "can_act".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DetailsParticipants {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
    pub post_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    pub admin: bool,
    pub moderator: bool,
    pub trust_level: i64,
}

impl std::fmt::Display for DetailsParticipants {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DetailsParticipants {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.flair_bg_color).into(),
            if let Some(flair_group_id) = &self.flair_group_id {
                format!("{:?}", flair_group_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.trust_level).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "post_count".into(),
            "primary_group_name".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_color".into(),
            "flair_bg_color".into(),
            "flair_group_id".into(),
            "admin".into(),
            "moderator".into(),
            "trust_level".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreatedBy {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
}

impl std::fmt::Display for CreatedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreatedBy {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LastPoster {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
}

impl std::fmt::Display for LastPoster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LastPoster {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Details {
    pub can_edit: bool,
    pub notification_level: i64,
    pub can_move_posts: bool,
    pub can_delete: bool,
    pub can_remove_allowed_users: bool,
    pub can_create_post: bool,
    pub can_reply_as_new_topic: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_invite_to: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_invite_via_email: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_flag_topic: Option<bool>,
    pub can_convert_topic: bool,
    pub can_review_topic: bool,
    pub can_close_topic: bool,
    pub can_archive_topic: bool,
    pub can_split_merge_topic: bool,
    pub can_edit_staff_notes: bool,
    pub can_toggle_topic_visibility: bool,
    pub can_pin_unpin_topic: bool,
    pub can_moderate_category: bool,
    pub can_remove_self_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<DetailsParticipants>>,
    pub created_by: CreatedBy,
    pub last_poster: LastPoster,
}

impl std::fmt::Display for Details {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Details {
    const LENGTH: usize = 23;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.notification_level).into(),
            format!("{:?}", self.can_move_posts).into(),
            format!("{:?}", self.can_delete).into(),
            format!("{:?}", self.can_remove_allowed_users).into(),
            format!("{:?}", self.can_create_post).into(),
            format!("{:?}", self.can_reply_as_new_topic).into(),
            if let Some(can_invite_to) = &self.can_invite_to {
                format!("{:?}", can_invite_to).into()
            } else {
                String::new().into()
            },
            if let Some(can_invite_via_email) = &self.can_invite_via_email {
                format!("{:?}", can_invite_via_email).into()
            } else {
                String::new().into()
            },
            if let Some(can_flag_topic) = &self.can_flag_topic {
                format!("{:?}", can_flag_topic).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_convert_topic).into(),
            format!("{:?}", self.can_review_topic).into(),
            format!("{:?}", self.can_close_topic).into(),
            format!("{:?}", self.can_archive_topic).into(),
            format!("{:?}", self.can_split_merge_topic).into(),
            format!("{:?}", self.can_edit_staff_notes).into(),
            format!("{:?}", self.can_toggle_topic_visibility).into(),
            format!("{:?}", self.can_pin_unpin_topic).into(),
            format!("{:?}", self.can_moderate_category).into(),
            format!("{:?}", self.can_remove_self_id).into(),
            if let Some(participants) = &self.participants {
                format!("{:?}", participants).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_by).into(),
            format!("{:?}", self.last_poster).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_edit".into(),
            "notification_level".into(),
            "can_move_posts".into(),
            "can_delete".into(),
            "can_remove_allowed_users".into(),
            "can_create_post".into(),
            "can_reply_as_new_topic".into(),
            "can_invite_to".into(),
            "can_invite_via_email".into(),
            "can_flag_topic".into(),
            "can_convert_topic".into(),
            "can_review_topic".into(),
            "can_close_topic".into(),
            "can_archive_topic".into(),
            "can_split_merge_topic".into(),
            "can_edit_staff_notes".into(),
            "can_toggle_topic_visibility".into(),
            "can_pin_unpin_topic".into(),
            "can_moderate_category".into(),
            "can_remove_self_id".into(),
            "participants".into(),
            "created_by".into(),
            "last_poster".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTopicResponse {
    pub post_stream: GetTopicResponsePostStream,
    pub timeline_lookup: Vec<serde_json::Value>,
    pub suggested_topics: Vec<SuggestedTopics>,
    pub tags: Vec<serde_json::Value>,
    pub tags_descriptions: GetTopicResponseTagsDescriptions,
    pub id: i64,
    pub title: String,
    pub fancy_title: String,
    pub posts_count: i64,
    pub created_at: String,
    pub views: i64,
    pub reply_count: i64,
    pub like_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    pub visible: bool,
    pub closed: bool,
    pub archived: bool,
    pub has_summary: bool,
    pub archetype: String,
    pub slug: String,
    pub category_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<String>,
    pub user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    pub pinned_globally: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_until: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub slow_mode_seconds: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<String>,
    pub draft_key: String,
    pub draft_sequence: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<String>,
    pub pinned: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_by: Option<String>,
    pub has_deleted: bool,
    pub actions_summary: Vec<GetTopicResponseActionsSummary>,
    pub chunk_size: i64,
    pub bookmarked: bool,
    pub bookmarks: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_timer: Option<String>,
    pub message_bus_last_id: i64,
    pub participant_count: i64,
    pub show_read_indicator: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnails: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slow_mode_enabled_until: Option<String>,
    pub details: Details,
}

impl std::fmt::Display for GetTopicResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTopicResponse {
    const LENGTH: usize = 50;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.post_stream).into(),
            format!("{:?}", self.timeline_lookup).into(),
            format!("{:?}", self.suggested_topics).into(),
            format!("{:?}", self.tags).into(),
            format!("{:?}", self.tags_descriptions).into(),
            format!("{:?}", self.id).into(),
            self.title.clone().into(),
            self.fancy_title.clone().into(),
            format!("{:?}", self.posts_count).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.views).into(),
            format!("{:?}", self.reply_count).into(),
            format!("{:?}", self.like_count).into(),
            format!("{:?}", self.last_posted_at).into(),
            format!("{:?}", self.visible).into(),
            format!("{:?}", self.closed).into(),
            format!("{:?}", self.archived).into(),
            format!("{:?}", self.has_summary).into(),
            self.archetype.clone().into(),
            self.slug.clone().into(),
            format!("{:?}", self.category_id).into(),
            format!("{:?}", self.word_count).into(),
            format!("{:?}", self.deleted_at).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.featured_link).into(),
            format!("{:?}", self.pinned_globally).into(),
            format!("{:?}", self.pinned_at).into(),
            format!("{:?}", self.pinned_until).into(),
            format!("{:?}", self.image_url).into(),
            format!("{:?}", self.slow_mode_seconds).into(),
            format!("{:?}", self.draft).into(),
            self.draft_key.clone().into(),
            format!("{:?}", self.draft_sequence).into(),
            format!("{:?}", self.unpinned).into(),
            format!("{:?}", self.pinned).into(),
            if let Some(current_post_number) = &self.current_post_number {
                format!("{:?}", current_post_number).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.highest_post_number).into(),
            format!("{:?}", self.deleted_by).into(),
            format!("{:?}", self.has_deleted).into(),
            format!("{:?}", self.actions_summary).into(),
            format!("{:?}", self.chunk_size).into(),
            format!("{:?}", self.bookmarked).into(),
            format!("{:?}", self.bookmarks).into(),
            format!("{:?}", self.topic_timer).into(),
            format!("{:?}", self.message_bus_last_id).into(),
            format!("{:?}", self.participant_count).into(),
            format!("{:?}", self.show_read_indicator).into(),
            format!("{:?}", self.thumbnails).into(),
            format!("{:?}", self.slow_mode_enabled_until).into(),
            format!("{:?}", self.details).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "post_stream".into(),
            "timeline_lookup".into(),
            "suggested_topics".into(),
            "tags".into(),
            "tags_descriptions".into(),
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "posts_count".into(),
            "created_at".into(),
            "views".into(),
            "reply_count".into(),
            "like_count".into(),
            "last_posted_at".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "has_summary".into(),
            "archetype".into(),
            "slug".into(),
            "category_id".into(),
            "word_count".into(),
            "deleted_at".into(),
            "user_id".into(),
            "featured_link".into(),
            "pinned_globally".into(),
            "pinned_at".into(),
            "pinned_until".into(),
            "image_url".into(),
            "slow_mode_seconds".into(),
            "draft".into(),
            "draft_key".into(),
            "draft_sequence".into(),
            "unpinned".into(),
            "pinned".into(),
            "current_post_number".into(),
            "highest_post_number".into(),
            "deleted_by".into(),
            "has_deleted".into(),
            "actions_summary".into(),
            "chunk_size".into(),
            "bookmarked".into(),
            "bookmarks".into(),
            "topic_timer".into(),
            "message_bus_last_id".into(),
            "participant_count".into(),
            "show_read_indicator".into(),
            "thumbnails".into(),
            "slow_mode_enabled_until".into(),
            "details".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTopicRequestBodyTopic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
}

impl std::fmt::Display for UpdateTopicRequestBodyTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTopicRequestBodyTopic {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["title".into(), "category_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTopicRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<UpdateTopicRequestBodyTopic>,
}

impl std::fmt::Display for UpdateTopicRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTopicRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(topic) = &self.topic {
            format!("{:?}", topic).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["topic".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BasicTopic {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fancy_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
}

impl std::fmt::Display for BasicTopic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BasicTopic {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(fancy_title) = &self.fancy_title {
                format!("{:?}", fancy_title).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(posts_count) = &self.posts_count {
                format!("{:?}", posts_count).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTopicResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basic_topic: Option<BasicTopic>,
}

impl std::fmt::Display for UpdateTopicResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTopicResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(basic_topic) = &self.basic_topic {
            format!("{:?}", basic_topic).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["basic_topic".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InviteToTopicRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl std::fmt::Display for InviteToTopicRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InviteToTopicRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(user) = &self.user {
                format!("{:?}", user).into()
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
        vec!["user".into(), "email".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InviteToTopicResponseUser {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_template: Option<String>,
}

impl std::fmt::Display for InviteToTopicResponseUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InviteToTopicResponseUser {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(avatar_template) = &self.avatar_template {
                format!("{:?}", avatar_template).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InviteToTopicResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<InviteToTopicResponseUser>,
}

impl std::fmt::Display for InviteToTopicResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InviteToTopicResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(user) = &self.user {
            format!("{:?}", user).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InviteGroupToTopicRequestBody {
    #[doc = "The name of the group to invite"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[doc = "Whether to notify the group, it defaults to true"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub should_notify: Option<bool>,
}

impl std::fmt::Display for InviteGroupToTopicRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InviteGroupToTopicRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(group) = &self.group {
                format!("{:?}", group).into()
            } else {
                String::new().into()
            },
            if let Some(should_notify) = &self.should_notify {
                format!("{:?}", should_notify).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["group".into(), "should_notify".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InviteGroupToTopicResponseGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for InviteGroupToTopicResponseGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InviteGroupToTopicResponseGroup {
    const LENGTH: usize = 2;
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InviteGroupToTopicResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<InviteGroupToTopicResponseGroup>,
}

impl std::fmt::Display for InviteGroupToTopicResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InviteGroupToTopicResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(group) = &self.group {
            format!("{:?}", group).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["group".into()]
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
pub enum UpdateTopicStatusRequestBodyStatus {
    #[serde(rename = "closed")]
    #[display("closed")]
    Closed,
    #[serde(rename = "pinned")]
    #[display("pinned")]
    Pinned,
    #[serde(rename = "pinned_globally")]
    #[display("pinned_globally")]
    PinnedGlobally,
    #[serde(rename = "archived")]
    #[display("archived")]
    Archived,
    #[serde(rename = "visible")]
    #[display("visible")]
    Visible,
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
pub enum Enabled {
    #[serde(rename = "true")]
    #[display("true")]
    True,
    #[serde(rename = "false")]
    #[display("false")]
    False,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTopicStatusRequestBody {
    pub status: UpdateTopicStatusRequestBodyStatus,
    pub enabled: Enabled,
    #[doc = "Only required for `pinned` and `pinned_globally`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl std::fmt::Display for UpdateTopicStatusRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTopicStatusRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.status).into(),
            format!("{:?}", self.enabled).into(),
            if let Some(until) = &self.until {
                format!("{:?}", until).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["status".into(), "enabled".into(), "until".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTopicStatusResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_status_update: Option<String>,
}

impl std::fmt::Display for UpdateTopicStatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTopicStatusResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(success) = &self.success {
                format!("{:?}", success).into()
            } else {
                String::new().into()
            },
            if let Some(topic_status_update) = &self.topic_status_update {
                format!("{:?}", topic_status_update).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into(), "topic_status_update".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLatestTopicsResponseUsers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_template: Option<String>,
}

impl std::fmt::Display for ListLatestTopicsResponseUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLatestTopicsResponseUsers {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(avatar_template) = &self.avatar_template {
                format!("{:?}", avatar_template).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLatestTopicsResponseTopicListTopicsPosters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
}

impl std::fmt::Display for ListLatestTopicsResponseTopicListTopicsPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLatestTopicsResponseTopicListTopicsPosters {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(extras) = &self.extras {
                format!("{:?}", extras).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group_id) = &self.primary_group_id {
                format!("{:?}", primary_group_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "extras".into(),
            "description".into(),
            "user_id".into(),
            "primary_group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLatestTopicsResponseTopicListTopics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fancy_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unseen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_posts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_poster_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op_like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_globally: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posters: Option<Vec<ListLatestTopicsResponseTopicListTopicsPosters>>,
}

impl std::fmt::Display for ListLatestTopicsResponseTopicListTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLatestTopicsResponseTopicListTopics {
    const LENGTH: usize = 33;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(fancy_title) = &self.fancy_title {
                format!("{:?}", fancy_title).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(posts_count) = &self.posts_count {
                format!("{:?}", posts_count).into()
            } else {
                String::new().into()
            },
            if let Some(reply_count) = &self.reply_count {
                format!("{:?}", reply_count).into()
            } else {
                String::new().into()
            },
            if let Some(highest_post_number) = &self.highest_post_number {
                format!("{:?}", highest_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(image_url) = &self.image_url {
                format!("{:?}", image_url).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(last_posted_at) = &self.last_posted_at {
                format!("{:?}", last_posted_at).into()
            } else {
                String::new().into()
            },
            if let Some(bumped) = &self.bumped {
                format!("{:?}", bumped).into()
            } else {
                String::new().into()
            },
            if let Some(bumped_at) = &self.bumped_at {
                format!("{:?}", bumped_at).into()
            } else {
                String::new().into()
            },
            if let Some(archetype) = &self.archetype {
                format!("{:?}", archetype).into()
            } else {
                String::new().into()
            },
            if let Some(unseen) = &self.unseen {
                format!("{:?}", unseen).into()
            } else {
                String::new().into()
            },
            if let Some(last_read_post_number) = &self.last_read_post_number {
                format!("{:?}", last_read_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(unread_posts) = &self.unread_posts {
                format!("{:?}", unread_posts).into()
            } else {
                String::new().into()
            },
            if let Some(pinned) = &self.pinned {
                format!("{:?}", pinned).into()
            } else {
                String::new().into()
            },
            if let Some(unpinned) = &self.unpinned {
                format!("{:?}", unpinned).into()
            } else {
                String::new().into()
            },
            if let Some(visible) = &self.visible {
                format!("{:?}", visible).into()
            } else {
                String::new().into()
            },
            if let Some(closed) = &self.closed {
                format!("{:?}", closed).into()
            } else {
                String::new().into()
            },
            if let Some(archived) = &self.archived {
                format!("{:?}", archived).into()
            } else {
                String::new().into()
            },
            if let Some(notification_level) = &self.notification_level {
                format!("{:?}", notification_level).into()
            } else {
                String::new().into()
            },
            if let Some(bookmarked) = &self.bookmarked {
                format!("{:?}", bookmarked).into()
            } else {
                String::new().into()
            },
            if let Some(liked) = &self.liked {
                format!("{:?}", liked).into()
            } else {
                String::new().into()
            },
            if let Some(views) = &self.views {
                format!("{:?}", views).into()
            } else {
                String::new().into()
            },
            if let Some(like_count) = &self.like_count {
                format!("{:?}", like_count).into()
            } else {
                String::new().into()
            },
            if let Some(has_summary) = &self.has_summary {
                format!("{:?}", has_summary).into()
            } else {
                String::new().into()
            },
            if let Some(last_poster_username) = &self.last_poster_username {
                format!("{:?}", last_poster_username).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
            if let Some(op_like_count) = &self.op_like_count {
                format!("{:?}", op_like_count).into()
            } else {
                String::new().into()
            },
            if let Some(pinned_globally) = &self.pinned_globally {
                format!("{:?}", pinned_globally).into()
            } else {
                String::new().into()
            },
            if let Some(featured_link) = &self.featured_link {
                format!("{:?}", featured_link).into()
            } else {
                String::new().into()
            },
            if let Some(posters) = &self.posters {
                format!("{:?}", posters).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
            "reply_count".into(),
            "highest_post_number".into(),
            "image_url".into(),
            "created_at".into(),
            "last_posted_at".into(),
            "bumped".into(),
            "bumped_at".into(),
            "archetype".into(),
            "unseen".into(),
            "last_read_post_number".into(),
            "unread_posts".into(),
            "pinned".into(),
            "unpinned".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "notification_level".into(),
            "bookmarked".into(),
            "liked".into(),
            "views".into(),
            "like_count".into(),
            "has_summary".into(),
            "last_poster_username".into(),
            "category_id".into(),
            "op_like_count".into(),
            "pinned_globally".into(),
            "featured_link".into(),
            "posters".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLatestTopicsResponseTopicList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_create_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_sequence: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<ListLatestTopicsResponseTopicListTopics>>,
}

impl std::fmt::Display for ListLatestTopicsResponseTopicList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLatestTopicsResponseTopicList {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(can_create_topic) = &self.can_create_topic {
                format!("{:?}", can_create_topic).into()
            } else {
                String::new().into()
            },
            if let Some(draft) = &self.draft {
                format!("{:?}", draft).into()
            } else {
                String::new().into()
            },
            if let Some(draft_key) = &self.draft_key {
                format!("{:?}", draft_key).into()
            } else {
                String::new().into()
            },
            if let Some(draft_sequence) = &self.draft_sequence {
                format!("{:?}", draft_sequence).into()
            } else {
                String::new().into()
            },
            if let Some(per_page) = &self.per_page {
                format!("{:?}", per_page).into()
            } else {
                String::new().into()
            },
            if let Some(topics) = &self.topics {
                format!("{:?}", topics).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_create_topic".into(),
            "draft".into(),
            "draft_key".into(),
            "draft_sequence".into(),
            "per_page".into(),
            "topics".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLatestTopicsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ListLatestTopicsResponseUsers>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_groups: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_list: Option<ListLatestTopicsResponseTopicList>,
}

impl std::fmt::Display for ListLatestTopicsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLatestTopicsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
            if let Some(primary_groups) = &self.primary_groups {
                format!("{:?}", primary_groups).into()
            } else {
                String::new().into()
            },
            if let Some(topic_list) = &self.topic_list {
                format!("{:?}", topic_list).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "primary_groups".into(), "topic_list".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTopTopicsResponseUsers {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_template: Option<String>,
}

impl std::fmt::Display for ListTopTopicsResponseUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTopTopicsResponseUsers {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(username) = &self.username {
                format!("{:?}", username).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(avatar_template) = &self.avatar_template {
                format!("{:?}", avatar_template).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTopTopicsResponseTopicListTopicsPosters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extras: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
}

impl std::fmt::Display for ListTopTopicsResponseTopicListTopicsPosters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTopTopicsResponseTopicListTopicsPosters {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(extras) = &self.extras {
                format!("{:?}", extras).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(primary_group_id) = &self.primary_group_id {
                format!("{:?}", primary_group_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "extras".into(),
            "description".into(),
            "user_id".into(),
            "primary_group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTopTopicsResponseTopicListTopics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fancy_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posts_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bumped_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archetype: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unseen: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_read_post_number: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unread_posts: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpinned: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification_level: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmarked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub liked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub views: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_poster_username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub op_like_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_globally: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posters: Option<Vec<ListTopTopicsResponseTopicListTopicsPosters>>,
}

impl std::fmt::Display for ListTopTopicsResponseTopicListTopics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTopTopicsResponseTopicListTopics {
    const LENGTH: usize = 33;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(fancy_title) = &self.fancy_title {
                format!("{:?}", fancy_title).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            if let Some(posts_count) = &self.posts_count {
                format!("{:?}", posts_count).into()
            } else {
                String::new().into()
            },
            if let Some(reply_count) = &self.reply_count {
                format!("{:?}", reply_count).into()
            } else {
                String::new().into()
            },
            if let Some(highest_post_number) = &self.highest_post_number {
                format!("{:?}", highest_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(image_url) = &self.image_url {
                format!("{:?}", image_url).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(last_posted_at) = &self.last_posted_at {
                format!("{:?}", last_posted_at).into()
            } else {
                String::new().into()
            },
            if let Some(bumped) = &self.bumped {
                format!("{:?}", bumped).into()
            } else {
                String::new().into()
            },
            if let Some(bumped_at) = &self.bumped_at {
                format!("{:?}", bumped_at).into()
            } else {
                String::new().into()
            },
            if let Some(archetype) = &self.archetype {
                format!("{:?}", archetype).into()
            } else {
                String::new().into()
            },
            if let Some(unseen) = &self.unseen {
                format!("{:?}", unseen).into()
            } else {
                String::new().into()
            },
            if let Some(last_read_post_number) = &self.last_read_post_number {
                format!("{:?}", last_read_post_number).into()
            } else {
                String::new().into()
            },
            if let Some(unread_posts) = &self.unread_posts {
                format!("{:?}", unread_posts).into()
            } else {
                String::new().into()
            },
            if let Some(pinned) = &self.pinned {
                format!("{:?}", pinned).into()
            } else {
                String::new().into()
            },
            if let Some(unpinned) = &self.unpinned {
                format!("{:?}", unpinned).into()
            } else {
                String::new().into()
            },
            if let Some(visible) = &self.visible {
                format!("{:?}", visible).into()
            } else {
                String::new().into()
            },
            if let Some(closed) = &self.closed {
                format!("{:?}", closed).into()
            } else {
                String::new().into()
            },
            if let Some(archived) = &self.archived {
                format!("{:?}", archived).into()
            } else {
                String::new().into()
            },
            if let Some(notification_level) = &self.notification_level {
                format!("{:?}", notification_level).into()
            } else {
                String::new().into()
            },
            if let Some(bookmarked) = &self.bookmarked {
                format!("{:?}", bookmarked).into()
            } else {
                String::new().into()
            },
            if let Some(liked) = &self.liked {
                format!("{:?}", liked).into()
            } else {
                String::new().into()
            },
            if let Some(views) = &self.views {
                format!("{:?}", views).into()
            } else {
                String::new().into()
            },
            if let Some(like_count) = &self.like_count {
                format!("{:?}", like_count).into()
            } else {
                String::new().into()
            },
            if let Some(has_summary) = &self.has_summary {
                format!("{:?}", has_summary).into()
            } else {
                String::new().into()
            },
            if let Some(last_poster_username) = &self.last_poster_username {
                format!("{:?}", last_poster_username).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
            if let Some(op_like_count) = &self.op_like_count {
                format!("{:?}", op_like_count).into()
            } else {
                String::new().into()
            },
            if let Some(pinned_globally) = &self.pinned_globally {
                format!("{:?}", pinned_globally).into()
            } else {
                String::new().into()
            },
            if let Some(featured_link) = &self.featured_link {
                format!("{:?}", featured_link).into()
            } else {
                String::new().into()
            },
            if let Some(posters) = &self.posters {
                format!("{:?}", posters).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "title".into(),
            "fancy_title".into(),
            "slug".into(),
            "posts_count".into(),
            "reply_count".into(),
            "highest_post_number".into(),
            "image_url".into(),
            "created_at".into(),
            "last_posted_at".into(),
            "bumped".into(),
            "bumped_at".into(),
            "archetype".into(),
            "unseen".into(),
            "last_read_post_number".into(),
            "unread_posts".into(),
            "pinned".into(),
            "unpinned".into(),
            "visible".into(),
            "closed".into(),
            "archived".into(),
            "notification_level".into(),
            "bookmarked".into(),
            "liked".into(),
            "views".into(),
            "like_count".into(),
            "has_summary".into(),
            "last_poster_username".into(),
            "category_id".into(),
            "op_like_count".into(),
            "pinned_globally".into(),
            "featured_link".into(),
            "posters".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTopTopicsResponseTopicList {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_create_topic: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_sequence: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub for_period: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<ListTopTopicsResponseTopicListTopics>>,
}

impl std::fmt::Display for ListTopTopicsResponseTopicList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTopTopicsResponseTopicList {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(can_create_topic) = &self.can_create_topic {
                format!("{:?}", can_create_topic).into()
            } else {
                String::new().into()
            },
            if let Some(draft) = &self.draft {
                format!("{:?}", draft).into()
            } else {
                String::new().into()
            },
            if let Some(draft_key) = &self.draft_key {
                format!("{:?}", draft_key).into()
            } else {
                String::new().into()
            },
            if let Some(draft_sequence) = &self.draft_sequence {
                format!("{:?}", draft_sequence).into()
            } else {
                String::new().into()
            },
            if let Some(for_period) = &self.for_period {
                format!("{:?}", for_period).into()
            } else {
                String::new().into()
            },
            if let Some(per_page) = &self.per_page {
                format!("{:?}", per_page).into()
            } else {
                String::new().into()
            },
            if let Some(topics) = &self.topics {
                format!("{:?}", topics).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "can_create_topic".into(),
            "draft".into(),
            "draft_key".into(),
            "draft_sequence".into(),
            "for_period".into(),
            "per_page".into(),
            "topics".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTopTopicsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ListTopTopicsResponseUsers>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_groups: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic_list: Option<ListTopTopicsResponseTopicList>,
}

impl std::fmt::Display for ListTopTopicsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTopTopicsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
            if let Some(primary_groups) = &self.primary_groups {
                format!("{:?}", primary_groups).into()
            } else {
                String::new().into()
            },
            if let Some(topic_list) = &self.topic_list {
                format!("{:?}", topic_list).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["users".into(), "primary_groups".into(), "topic_list".into()]
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
pub enum NotificationLevel {
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
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SetNotificationLevelRequestBody {
    pub notification_level: NotificationLevel,
}

impl std::fmt::Display for SetNotificationLevelRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SetNotificationLevelRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.notification_level).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["notification_level".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SetNotificationLevelResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
}

impl std::fmt::Display for SetNotificationLevelResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SetNotificationLevelResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(success) = &self.success {
            format!("{:?}", success).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTopicTimestampRequestBody {
    pub timestamp: String,
}

impl std::fmt::Display for UpdateTopicTimestampRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTopicTimestampRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.timestamp.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["timestamp".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateTopicTimestampResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
}

impl std::fmt::Display for UpdateTopicTimestampResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateTopicTimestampResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(success) = &self.success {
            format!("{:?}", success).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTopicTimerRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub based_on_last_post: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
}

impl std::fmt::Display for CreateTopicTimerRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateTopicTimerRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(time) = &self.time {
                format!("{:?}", time).into()
            } else {
                String::new().into()
            },
            if let Some(status_type) = &self.status_type {
                format!("{:?}", status_type).into()
            } else {
                String::new().into()
            },
            if let Some(based_on_last_post) = &self.based_on_last_post {
                format!("{:?}", based_on_last_post).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "time".into(),
            "status_type".into(),
            "based_on_last_post".into(),
            "category_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTopicTimerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub success: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execute_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub based_on_last_post: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<i64>,
}

impl std::fmt::Display for CreateTopicTimerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateTopicTimerResponse {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(success) = &self.success {
                format!("{:?}", success).into()
            } else {
                String::new().into()
            },
            if let Some(execute_at) = &self.execute_at {
                format!("{:?}", execute_at).into()
            } else {
                String::new().into()
            },
            if let Some(duration) = &self.duration {
                format!("{:?}", duration).into()
            } else {
                String::new().into()
            },
            if let Some(based_on_last_post) = &self.based_on_last_post {
                format!("{:?}", based_on_last_post).into()
            } else {
                String::new().into()
            },
            if let Some(closed) = &self.closed {
                format!("{:?}", closed).into()
            } else {
                String::new().into()
            },
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "success".into(),
            "execute_at".into(),
            "duration".into(),
            "based_on_last_post".into(),
            "closed".into(),
            "category_id".into(),
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
pub enum Type {
    #[serde(rename = "avatar")]
    #[display("avatar")]
    Avatar,
    #[serde(rename = "profile_background")]
    #[display("profile_background")]
    ProfileBackground,
    #[serde(rename = "card_background")]
    #[display("card_background")]
    CardBackground,
    #[serde(rename = "custom_emoji")]
    #[display("custom_emoji")]
    CustomEmoji,
    #[serde(rename = "composer")]
    #[display("composer")]
    Composer,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateUploadRequestBody {
    #[serde(rename = "type")]
    pub type_: Type,
    #[doc = "required if uploading an avatar"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[doc = "Use this flag to return an id and url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synchronous: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file: Option<bytes::Bytes>,
}

impl std::fmt::Display for CreateUploadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateUploadRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.type_).into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(synchronous) = &self.synchronous {
                format!("{:?}", synchronous).into()
            } else {
                String::new().into()
            },
            if let Some(file) = &self.file {
                format!("{:?}", file).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "user_id".into(),
            "synchronous".into(),
            "file".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Thumbnail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i64>,
}

impl std::fmt::Display for Thumbnail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Thumbnail {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(upload_id) = &self.upload_id {
                format!("{:?}", upload_id).into()
            } else {
                String::new().into()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url).into()
            } else {
                String::new().into()
            },
            if let Some(extension) = &self.extension {
                format!("{:?}", extension).into()
            } else {
                String::new().into()
            },
            if let Some(width) = &self.width {
                format!("{:?}", width).into()
            } else {
                String::new().into()
            },
            if let Some(height) = &self.height {
                format!("{:?}", height).into()
            } else {
                String::new().into()
            },
            if let Some(filesize) = &self.filesize {
                format!("{:?}", filesize).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "upload_id".into(),
            "url".into(),
            "extension".into(),
            "width".into(),
            "height".into(),
            "filesize".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct OptimizedVideo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filesize: Option<i64>,
    #[serde(rename = "sha1", default, skip_serializing_if = "Option::is_none")]
    pub sha_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_filename: Option<String>,
}

impl std::fmt::Display for OptimizedVideo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for OptimizedVideo {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(upload_id) = &self.upload_id {
                format!("{:?}", upload_id).into()
            } else {
                String::new().into()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url).into()
            } else {
                String::new().into()
            },
            if let Some(extension) = &self.extension {
                format!("{:?}", extension).into()
            } else {
                String::new().into()
            },
            if let Some(filesize) = &self.filesize {
                format!("{:?}", filesize).into()
            } else {
                String::new().into()
            },
            if let Some(sha_1) = &self.sha_1 {
                format!("{:?}", sha_1).into()
            } else {
                String::new().into()
            },
            if let Some(original_filename) = &self.original_filename {
                format!("{:?}", original_filename).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "upload_id".into(),
            "url".into(),
            "extension".into(),
            "filesize".into(),
            "sha_1".into(),
            "original_filename".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateUploadResponse {
    pub id: i64,
    pub url: String,
    pub original_filename: String,
    pub filesize: i64,
    pub width: i64,
    pub height: i64,
    pub thumbnail_width: i64,
    pub thumbnail_height: i64,
    pub extension: String,
    pub short_url: String,
    pub short_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain_hours: Option<String>,
    pub human_filesize: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dominant_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimized_video: Option<OptimizedVideo>,
}

impl std::fmt::Display for CreateUploadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateUploadResponse {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.url.clone().into(),
            self.original_filename.clone().into(),
            format!("{:?}", self.filesize).into(),
            format!("{:?}", self.width).into(),
            format!("{:?}", self.height).into(),
            format!("{:?}", self.thumbnail_width).into(),
            format!("{:?}", self.thumbnail_height).into(),
            self.extension.clone().into(),
            self.short_url.clone().into(),
            self.short_path.clone().into(),
            format!("{:?}", self.retain_hours).into(),
            self.human_filesize.clone().into(),
            if let Some(dominant_color) = &self.dominant_color {
                format!("{:?}", dominant_color).into()
            } else {
                String::new().into()
            },
            if let Some(thumbnail) = &self.thumbnail {
                format!("{:?}", thumbnail).into()
            } else {
                String::new().into()
            },
            if let Some(optimized_video) = &self.optimized_video {
                format!("{:?}", optimized_video).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "url".into(),
            "original_filename".into(),
            "filesize".into(),
            "width".into(),
            "height".into(),
            "thumbnail_width".into(),
            "thumbnail_height".into(),
            "extension".into(),
            "short_url".into(),
            "short_path".into(),
            "retain_hours".into(),
            "human_filesize".into(),
            "dominant_color".into(),
            "thumbnail".into(),
            "optimized_video".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Metadata {
    #[doc = "The SHA1 checksum of the upload binary blob. Optionally\nbe provided and serves as \
             an additional security check when\nlater processing the file in \
             complete-external-upload endpoint."]
    #[serde(
        rename = "sha1-checksum",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sha_1_checksum: Option<String>,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for Metadata {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(sha_1_checksum) = &self.sha_1_checksum {
            format!("{:?}", sha_1_checksum).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["sha_1_checksum".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GeneratePresignedPutRequestBody {
    #[serde(rename = "type")]
    pub type_: Type,
    pub file_name: String,
    #[doc = "File size should be represented in bytes."]
    pub file_size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl std::fmt::Display for GeneratePresignedPutRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GeneratePresignedPutRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.type_).into(),
            self.file_name.clone().into(),
            format!("{:?}", self.file_size).into(),
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "file_name".into(),
            "file_size".into(),
            "metadata".into(),
        ]
    }
}

#[doc = "A map of headers that must be sent with the PUT request."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SignedHeaders {}

impl std::fmt::Display for SignedHeaders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SignedHeaders {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GeneratePresignedPutResponse {
    #[doc = "The path of the temporary file on the external storage\nservice."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[doc = "A presigned PUT URL which must be used to upload\nthe file binary blob to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "A map of headers that must be sent with the PUT request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_headers: Option<SignedHeaders>,
    #[doc = "A unique string that identifies the external upload.\nThis must be stored and then \
             sent in the /complete-external-upload\nendpoint to complete the direct upload."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_identifier: Option<String>,
}

impl std::fmt::Display for GeneratePresignedPutResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GeneratePresignedPutResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(key) = &self.key {
                format!("{:?}", key).into()
            } else {
                String::new().into()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url).into()
            } else {
                String::new().into()
            },
            if let Some(signed_headers) = &self.signed_headers {
                format!("{:?}", signed_headers).into()
            } else {
                String::new().into()
            },
            if let Some(unique_identifier) = &self.unique_identifier {
                format!("{:?}", unique_identifier).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "key".into(),
            "url".into(),
            "signed_headers".into(),
            "unique_identifier".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompleteExternalUploadRequestBody {
    #[doc = "The unique identifier returned in the original /generate-presigned-put\nrequest."]
    pub unique_identifier: String,
    #[doc = "Optionally set this to true if the upload is for a\nprivate message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub for_private_message: Option<String>,
    #[doc = "Optionally set this to true if the upload is for a\nsite setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub for_site_setting: Option<String>,
    #[doc = "Optionally set this to true if the upload was pasted\ninto the upload area. This \
             will convert PNG files to JPEG."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pasted: Option<String>,
}

impl std::fmt::Display for CompleteExternalUploadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompleteExternalUploadRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.unique_identifier.clone().into(),
            if let Some(for_private_message) = &self.for_private_message {
                format!("{:?}", for_private_message).into()
            } else {
                String::new().into()
            },
            if let Some(for_site_setting) = &self.for_site_setting {
                format!("{:?}", for_site_setting).into()
            } else {
                String::new().into()
            },
            if let Some(pasted) = &self.pasted {
                format!("{:?}", pasted).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "unique_identifier".into(),
            "for_private_message".into(),
            "for_site_setting".into(),
            "pasted".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompleteExternalUploadResponse {
    pub id: i64,
    pub url: String,
    pub original_filename: String,
    pub filesize: i64,
    pub width: i64,
    pub height: i64,
    pub thumbnail_width: i64,
    pub thumbnail_height: i64,
    pub extension: String,
    pub short_url: String,
    pub short_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain_hours: Option<String>,
    pub human_filesize: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dominant_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimized_video: Option<OptimizedVideo>,
}

impl std::fmt::Display for CompleteExternalUploadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompleteExternalUploadResponse {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.url.clone().into(),
            self.original_filename.clone().into(),
            format!("{:?}", self.filesize).into(),
            format!("{:?}", self.width).into(),
            format!("{:?}", self.height).into(),
            format!("{:?}", self.thumbnail_width).into(),
            format!("{:?}", self.thumbnail_height).into(),
            self.extension.clone().into(),
            self.short_url.clone().into(),
            self.short_path.clone().into(),
            format!("{:?}", self.retain_hours).into(),
            self.human_filesize.clone().into(),
            if let Some(dominant_color) = &self.dominant_color {
                format!("{:?}", dominant_color).into()
            } else {
                String::new().into()
            },
            if let Some(thumbnail) = &self.thumbnail {
                format!("{:?}", thumbnail).into()
            } else {
                String::new().into()
            },
            if let Some(optimized_video) = &self.optimized_video {
                format!("{:?}", optimized_video).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "url".into(),
            "original_filename".into(),
            "filesize".into(),
            "width".into(),
            "height".into(),
            "thumbnail_width".into(),
            "thumbnail_height".into(),
            "extension".into(),
            "short_url".into(),
            "short_path".into(),
            "retain_hours".into(),
            "human_filesize".into(),
            "dominant_color".into(),
            "thumbnail".into(),
            "optimized_video".into(),
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
pub enum UploadType {
    #[serde(rename = "avatar")]
    #[display("avatar")]
    Avatar,
    #[serde(rename = "profile_background")]
    #[display("profile_background")]
    ProfileBackground,
    #[serde(rename = "card_background")]
    #[display("card_background")]
    CardBackground,
    #[serde(rename = "custom_emoji")]
    #[display("custom_emoji")]
    CustomEmoji,
    #[serde(rename = "composer")]
    #[display("composer")]
    Composer,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateMultipartUploadRequestBody {
    pub upload_type: UploadType,
    pub file_name: String,
    #[doc = "File size should be represented in bytes."]
    pub file_size: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

impl std::fmt::Display for CreateMultipartUploadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateMultipartUploadRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.upload_type).into(),
            self.file_name.clone().into(),
            format!("{:?}", self.file_size).into(),
            if let Some(metadata) = &self.metadata {
                format!("{:?}", metadata).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "upload_type".into(),
            "file_name".into(),
            "file_size".into(),
            "metadata".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateMultipartUploadResponse {
    #[doc = "The path of the temporary file on the external storage\nservice."]
    pub key: String,
    #[doc = "The identifier of the multipart upload in the external\nstorage provider. This is \
             the multipart upload_id in AWS S3."]
    pub external_upload_identifier: String,
    #[doc = "A unique string that identifies the external upload.\nThis must be stored and then \
             sent in the /complete-multipart\nand /batch-presign-multipart-parts endpoints."]
    pub unique_identifier: String,
}

impl std::fmt::Display for CreateMultipartUploadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateMultipartUploadResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.key.clone().into(),
            self.external_upload_identifier.clone().into(),
            self.unique_identifier.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "key".into(),
            "external_upload_identifier".into(),
            "unique_identifier".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchPresignMultipartPartsRequestBody {
    #[doc = "The part numbers to generate the presigned URLs for,\nmust be between 1 and 10000."]
    pub part_numbers: Vec<serde_json::Value>,
    #[doc = "The unique identifier returned in the original /create-multipart\nrequest."]
    pub unique_identifier: String,
}

impl std::fmt::Display for BatchPresignMultipartPartsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchPresignMultipartPartsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.part_numbers).into(),
            self.unique_identifier.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["part_numbers".into(), "unique_identifier".into()]
    }
}

#[doc = "The presigned URLs for each part number, which has\nthe part numbers as keys."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PresignedUrls {}

impl std::fmt::Display for PresignedUrls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PresignedUrls {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BatchPresignMultipartPartsResponse {
    #[doc = "The presigned URLs for each part number, which has\nthe part numbers as keys."]
    pub presigned_urls: PresignedUrls,
}

impl std::fmt::Display for BatchPresignMultipartPartsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BatchPresignMultipartPartsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.presigned_urls).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["presigned_urls".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AbortMultipartRequestBody {
    #[doc = "The identifier of the multipart upload in the external\nstorage provider. This is \
             the multipart upload_id in AWS S3."]
    pub external_upload_identifier: String,
}

impl std::fmt::Display for AbortMultipartRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AbortMultipartRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.external_upload_identifier.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["external_upload_identifier".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AbortMultipartResponse {
    pub success: String,
}

impl std::fmt::Display for AbortMultipartResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AbortMultipartResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompleteMultipartRequestBody {
    #[doc = "The unique identifier returned in the original /create-multipart\nrequest."]
    pub unique_identifier: String,
    #[doc = "All of the part numbers and their corresponding ETags\nthat have been uploaded must \
             be provided."]
    pub parts: Vec<serde_json::Value>,
}

impl std::fmt::Display for CompleteMultipartRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompleteMultipartRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.unique_identifier.clone().into(),
            format!("{:?}", self.parts).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["unique_identifier".into(), "parts".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompleteMultipartResponse {
    pub id: i64,
    pub url: String,
    pub original_filename: String,
    pub filesize: i64,
    pub width: i64,
    pub height: i64,
    pub thumbnail_width: i64,
    pub thumbnail_height: i64,
    pub extension: String,
    pub short_url: String,
    pub short_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retain_hours: Option<String>,
    pub human_filesize: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dominant_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Thumbnail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optimized_video: Option<OptimizedVideo>,
}

impl std::fmt::Display for CompleteMultipartResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompleteMultipartResponse {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.url.clone().into(),
            self.original_filename.clone().into(),
            format!("{:?}", self.filesize).into(),
            format!("{:?}", self.width).into(),
            format!("{:?}", self.height).into(),
            format!("{:?}", self.thumbnail_width).into(),
            format!("{:?}", self.thumbnail_height).into(),
            self.extension.clone().into(),
            self.short_url.clone().into(),
            self.short_path.clone().into(),
            format!("{:?}", self.retain_hours).into(),
            self.human_filesize.clone().into(),
            if let Some(dominant_color) = &self.dominant_color {
                format!("{:?}", dominant_color).into()
            } else {
                String::new().into()
            },
            if let Some(thumbnail) = &self.thumbnail {
                format!("{:?}", thumbnail).into()
            } else {
                String::new().into()
            },
            if let Some(optimized_video) = &self.optimized_video {
                format!("{:?}", optimized_video).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "url".into(),
            "original_filename".into(),
            "filesize".into(),
            "width".into(),
            "height".into(),
            "thumbnail_width".into(),
            "thumbnail_height".into(),
            "extension".into(),
            "short_url".into(),
            "short_path".into(),
            "retain_hours".into(),
            "human_filesize".into(),
            "dominant_color".into(),
            "thumbnail".into(),
            "optimized_video".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserBadgesResponseBadges {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub grant_count: i64,
    pub allow_title: bool,
    pub multiple_grant: bool,
    pub icon: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub listable: bool,
    pub enabled: bool,
    pub badge_grouping_id: i64,
    pub system: bool,
    pub slug: String,
    pub manually_grantable: bool,
    pub badge_type_id: i64,
}

impl std::fmt::Display for ListUserBadgesResponseBadges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserBadgesResponseBadges {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
            self.description.clone().into(),
            format!("{:?}", self.grant_count).into(),
            format!("{:?}", self.allow_title).into(),
            format!("{:?}", self.multiple_grant).into(),
            self.icon.clone().into(),
            format!("{:?}", self.image_url).into(),
            format!("{:?}", self.listable).into(),
            format!("{:?}", self.enabled).into(),
            format!("{:?}", self.badge_grouping_id).into(),
            format!("{:?}", self.system).into(),
            self.slug.clone().into(),
            format!("{:?}", self.manually_grantable).into(),
            format!("{:?}", self.badge_type_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "description".into(),
            "grant_count".into(),
            "allow_title".into(),
            "multiple_grant".into(),
            "icon".into(),
            "image_url".into(),
            "listable".into(),
            "enabled".into(),
            "badge_grouping_id".into(),
            "system".into(),
            "slug".into(),
            "manually_grantable".into(),
            "badge_type_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GrantedBies {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    pub admin: bool,
    pub moderator: bool,
    pub trust_level: i64,
}

impl std::fmt::Display for GrantedBies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GrantedBies {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.trust_level).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "flair_name".into(),
            "admin".into(),
            "moderator".into(),
            "trust_level".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserBadges {
    pub id: i64,
    pub granted_at: String,
    pub grouping_position: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_favorite: Option<String>,
    pub can_favorite: bool,
    pub badge_id: i64,
    pub granted_by_id: i64,
}

impl std::fmt::Display for UserBadges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserBadges {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.granted_at.clone().into(),
            format!("{:?}", self.grouping_position).into(),
            format!("{:?}", self.is_favorite).into(),
            format!("{:?}", self.can_favorite).into(),
            format!("{:?}", self.badge_id).into(),
            format!("{:?}", self.granted_by_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "granted_at".into(),
            "grouping_position".into(),
            "is_favorite".into(),
            "can_favorite".into(),
            "badge_id".into(),
            "granted_by_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserBadgesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badges: Option<Vec<ListUserBadgesResponseBadges>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge_types: Option<Vec<BadgeTypes>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_bies: Option<Vec<GrantedBies>>,
    pub user_badges: Vec<UserBadges>,
}

impl std::fmt::Display for ListUserBadgesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserBadgesResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(badges) = &self.badges {
                format!("{:?}", badges).into()
            } else {
                String::new().into()
            },
            if let Some(badge_types) = &self.badge_types {
                format!("{:?}", badge_types).into()
            } else {
                String::new().into()
            },
            if let Some(granted_bies) = &self.granted_bies {
                format!("{:?}", granted_bies).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_badges).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "badges".into(),
            "badge_types".into(),
            "granted_bies".into(),
            "user_badges".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserFields {
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub field_1: Option<bool>,
}

impl std::fmt::Display for UserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserFields {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(field_1) = &self.field_1 {
            format!("{:?}", field_1).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["field_1".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ExternalIds {}

impl std::fmt::Display for ExternalIds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ExternalIds {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateUserRequestBody {
    pub name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    #[doc = "This param requires an admin api key in the request\nheader or it will be ignored"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<UserFields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<ExternalIds>,
}

impl std::fmt::Display for CreateUserRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateUserRequestBody {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            self.email.clone().into(),
            self.password.clone().into(),
            self.username.clone().into(),
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
            } else {
                String::new().into()
            },
            if let Some(approved) = &self.approved {
                format!("{:?}", approved).into()
            } else {
                String::new().into()
            },
            if let Some(user_fields) = &self.user_fields {
                format!("{:?}", user_fields).into()
            } else {
                String::new().into()
            },
            if let Some(external_ids) = &self.external_ids {
                format!("{:?}", external_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "email".into(),
            "password".into(),
            "username".into(),
            "active".into(),
            "approved".into(),
            "user_fields".into(),
            "external_ids".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateUserResponse {
    pub success: bool,
    pub active: bool,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

impl std::fmt::Display for CreateUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateUserResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.success).into(),
            format!("{:?}", self.active).into(),
            self.message.clone().into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "success".into(),
            "active".into(),
            "message".into(),
            "user_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserResponseUserUserFields {
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub field_1: Option<String>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub field_2: Option<String>,
}

impl std::fmt::Display for GetUserResponseUserUserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserResponseUserUserFields {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.field_1).into(),
            format!("{:?}", self.field_2).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["field_1".into(), "field_2".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserResponseUserCustomFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

impl std::fmt::Display for GetUserResponseUserCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserResponseUserCustomFields {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(first_name) = &self.first_name {
            format!("{:?}", first_name).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["first_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserAuthTokens {
    pub id: i64,
    pub client_ip: String,
    pub location: String,
    pub browser: String,
    pub device: String,
    pub os: String,
    pub icon: String,
    pub created_at: String,
    pub seen_at: String,
    pub is_active: bool,
}

impl std::fmt::Display for UserAuthTokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserAuthTokens {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.client_ip.clone().into(),
            self.location.clone().into(),
            self.browser.clone().into(),
            self.device.clone().into(),
            self.os.clone().into(),
            self.icon.clone().into(),
            self.created_at.clone().into(),
            self.seen_at.clone().into(),
            format!("{:?}", self.is_active).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "client_ip".into(),
            "location".into(),
            "browser".into(),
            "device".into(),
            "os".into(),
            "icon".into(),
            "created_at".into(),
            "seen_at".into(),
            "is_active".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserNotificationSchedule {
    pub enabled: bool,
    pub day_0_start_time: i64,
    pub day_0_end_time: i64,
    pub day_1_start_time: i64,
    pub day_1_end_time: i64,
    pub day_2_start_time: i64,
    pub day_2_end_time: i64,
    pub day_3_start_time: i64,
    pub day_3_end_time: i64,
    pub day_4_start_time: i64,
    pub day_4_end_time: i64,
    pub day_5_start_time: i64,
    pub day_5_end_time: i64,
    pub day_6_start_time: i64,
    pub day_6_end_time: i64,
}

impl std::fmt::Display for UserNotificationSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserNotificationSchedule {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.enabled).into(),
            format!("{:?}", self.day_0_start_time).into(),
            format!("{:?}", self.day_0_end_time).into(),
            format!("{:?}", self.day_1_start_time).into(),
            format!("{:?}", self.day_1_end_time).into(),
            format!("{:?}", self.day_2_start_time).into(),
            format!("{:?}", self.day_2_end_time).into(),
            format!("{:?}", self.day_3_start_time).into(),
            format!("{:?}", self.day_3_end_time).into(),
            format!("{:?}", self.day_4_start_time).into(),
            format!("{:?}", self.day_4_end_time).into(),
            format!("{:?}", self.day_5_start_time).into(),
            format!("{:?}", self.day_5_end_time).into(),
            format!("{:?}", self.day_6_start_time).into(),
            format!("{:?}", self.day_6_end_time).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "enabled".into(),
            "day_0_start_time".into(),
            "day_0_end_time".into(),
            "day_1_start_time".into(),
            "day_1_end_time".into(),
            "day_2_start_time".into(),
            "day_2_end_time".into(),
            "day_3_start_time".into(),
            "day_3_end_time".into(),
            "day_4_start_time".into(),
            "day_4_end_time".into(),
            "day_5_start_time".into(),
            "day_5_end_time".into(),
            "day_6_start_time".into(),
            "day_6_end_time".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserResponseUserGroups {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub display_name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    pub publish_read_state: bool,
}

impl std::fmt::Display for GetUserResponseUserGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserResponseUserGroups {
    const LENGTH: usize = 29;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            self.display_name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            format!("{:?}", self.publish_read_state).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "display_name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "publish_read_state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GroupUsers {
    pub group_id: i64,
    pub user_id: i64,
    pub notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
}

impl std::fmt::Display for GroupUsers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GroupUsers {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.group_id).into(),
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.notification_level).into(),
            if let Some(owner) = &self.owner {
                format!("{:?}", owner).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "group_id".into(),
            "user_id".into(),
            "notification_level".into(),
            "owner".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserOption {
    pub user_id: i64,
    pub mailing_list_mode: bool,
    pub mailing_list_mode_frequency: i64,
    pub email_digests: bool,
    pub email_level: i64,
    pub email_messages_level: i64,
    pub external_links_in_new_tab: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bookmark_auto_delete_preference: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_scheme_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dark_scheme_id: Option<String>,
    pub dynamic_favicon: bool,
    pub enable_quoting: bool,
    pub enable_smart_lists: bool,
    pub enable_markdown_monospace_font: bool,
    pub enable_defer: bool,
    pub digest_after_minutes: i64,
    pub automatically_unpin_topics: bool,
    pub auto_track_topics_after_msecs: i64,
    pub notification_level_when_replying: i64,
    pub new_topic_duration_minutes: i64,
    pub email_previous_replies: i64,
    pub email_in_reply_to: bool,
    pub like_notification_frequency: i64,
    pub notify_on_linked_posts: bool,
    #[serde(rename = "include_tl0_in_digests")]
    pub include_tl_0_in_digests: bool,
    pub theme_ids: Vec<serde_json::Value>,
    pub theme_key_seq: i64,
    pub allow_private_messages: bool,
    pub enable_allowed_pm_users: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage_id: Option<String>,
    pub hide_profile_and_presence: bool,
    pub hide_profile: bool,
    pub hide_presence: bool,
    pub text_size: String,
    pub text_size_seq: i64,
    pub title_count_mode: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    pub skip_new_user_tips: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_calendar: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_search_log_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_link_to_filtered_list: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_show_count_of_new_items: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub watched_precedence_over_muted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seen_popups: Option<Vec<serde_json::Value>>,
    pub topics_unread_when_closed: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composition_mode: Option<i64>,
    pub interface_color_mode: i64,
}

impl std::fmt::Display for UserOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserOption {
    const LENGTH: usize = 47;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user_id).into(),
            format!("{:?}", self.mailing_list_mode).into(),
            format!("{:?}", self.mailing_list_mode_frequency).into(),
            format!("{:?}", self.email_digests).into(),
            format!("{:?}", self.email_level).into(),
            format!("{:?}", self.email_messages_level).into(),
            format!("{:?}", self.external_links_in_new_tab).into(),
            if let Some(bookmark_auto_delete_preference) = &self.bookmark_auto_delete_preference {
                format!("{:?}", bookmark_auto_delete_preference).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.color_scheme_id).into(),
            format!("{:?}", self.dark_scheme_id).into(),
            format!("{:?}", self.dynamic_favicon).into(),
            format!("{:?}", self.enable_quoting).into(),
            format!("{:?}", self.enable_smart_lists).into(),
            format!("{:?}", self.enable_markdown_monospace_font).into(),
            format!("{:?}", self.enable_defer).into(),
            format!("{:?}", self.digest_after_minutes).into(),
            format!("{:?}", self.automatically_unpin_topics).into(),
            format!("{:?}", self.auto_track_topics_after_msecs).into(),
            format!("{:?}", self.notification_level_when_replying).into(),
            format!("{:?}", self.new_topic_duration_minutes).into(),
            format!("{:?}", self.email_previous_replies).into(),
            format!("{:?}", self.email_in_reply_to).into(),
            format!("{:?}", self.like_notification_frequency).into(),
            format!("{:?}", self.notify_on_linked_posts).into(),
            format!("{:?}", self.include_tl_0_in_digests).into(),
            format!("{:?}", self.theme_ids).into(),
            format!("{:?}", self.theme_key_seq).into(),
            format!("{:?}", self.allow_private_messages).into(),
            format!("{:?}", self.enable_allowed_pm_users).into(),
            format!("{:?}", self.homepage_id).into(),
            format!("{:?}", self.hide_profile_and_presence).into(),
            format!("{:?}", self.hide_profile).into(),
            format!("{:?}", self.hide_presence).into(),
            self.text_size.clone().into(),
            format!("{:?}", self.text_size_seq).into(),
            self.title_count_mode.clone().into(),
            format!("{:?}", self.timezone).into(),
            format!("{:?}", self.skip_new_user_tips).into(),
            if let Some(default_calendar) = &self.default_calendar {
                format!("{:?}", default_calendar).into()
            } else {
                String::new().into()
            },
            if let Some(oldest_search_log_date) = &self.oldest_search_log_date {
                format!("{:?}", oldest_search_log_date).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_link_to_filtered_list) = &self.sidebar_link_to_filtered_list {
                format!("{:?}", sidebar_link_to_filtered_list).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_show_count_of_new_items) = &self.sidebar_show_count_of_new_items {
                format!("{:?}", sidebar_show_count_of_new_items).into()
            } else {
                String::new().into()
            },
            if let Some(watched_precedence_over_muted) = &self.watched_precedence_over_muted {
                format!("{:?}", watched_precedence_over_muted).into()
            } else {
                String::new().into()
            },
            if let Some(seen_popups) = &self.seen_popups {
                format!("{:?}", seen_popups).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.topics_unread_when_closed).into(),
            if let Some(composition_mode) = &self.composition_mode {
                format!("{:?}", composition_mode).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.interface_color_mode).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "user_id".into(),
            "mailing_list_mode".into(),
            "mailing_list_mode_frequency".into(),
            "email_digests".into(),
            "email_level".into(),
            "email_messages_level".into(),
            "external_links_in_new_tab".into(),
            "bookmark_auto_delete_preference".into(),
            "color_scheme_id".into(),
            "dark_scheme_id".into(),
            "dynamic_favicon".into(),
            "enable_quoting".into(),
            "enable_smart_lists".into(),
            "enable_markdown_monospace_font".into(),
            "enable_defer".into(),
            "digest_after_minutes".into(),
            "automatically_unpin_topics".into(),
            "auto_track_topics_after_msecs".into(),
            "notification_level_when_replying".into(),
            "new_topic_duration_minutes".into(),
            "email_previous_replies".into(),
            "email_in_reply_to".into(),
            "like_notification_frequency".into(),
            "notify_on_linked_posts".into(),
            "include_tl_0_in_digests".into(),
            "theme_ids".into(),
            "theme_key_seq".into(),
            "allow_private_messages".into(),
            "enable_allowed_pm_users".into(),
            "homepage_id".into(),
            "hide_profile_and_presence".into(),
            "hide_profile".into(),
            "hide_presence".into(),
            "text_size".into(),
            "text_size_seq".into(),
            "title_count_mode".into(),
            "timezone".into(),
            "skip_new_user_tips".into(),
            "default_calendar".into(),
            "oldest_search_log_date".into(),
            "sidebar_link_to_filtered_list".into(),
            "sidebar_show_count_of_new_items".into(),
            "watched_precedence_over_muted".into(),
            "seen_popups".into(),
            "topics_unread_when_closed".into(),
            "composition_mode".into(),
            "interface_color_mode".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserResponseUser {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    pub created_at: String,
    pub ignored: bool,
    pub muted: bool,
    pub can_ignore_user: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_ignore_users: Option<bool>,
    pub can_mute_user: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_mute_users: Option<bool>,
    pub can_send_private_messages: bool,
    pub can_send_private_message_to_user: bool,
    pub trust_level: i64,
    pub moderator: bool,
    pub admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub badge_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_factor_backup_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<GetUserResponseUserUserFields>,
    pub custom_fields: GetUserResponseUserCustomFields,
    pub time_read: i64,
    pub recent_time_read: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_topic: Option<String>,
    pub staged: bool,
    pub can_edit: bool,
    pub can_edit_username: bool,
    pub can_edit_email: bool,
    pub can_edit_name: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_avatar_id: Option<i64>,
    pub has_title_badges: bool,
    pub pending_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_posts_count: Option<i64>,
    pub profile_view_count: i64,
    pub second_factor_enabled: bool,
    pub can_upload_profile_header: bool,
    pub can_upload_user_card_background: bool,
    pub post_count: i64,
    pub can_be_deleted: bool,
    pub can_delete_all_posts: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    pub muted_category_ids: Vec<serde_json::Value>,
    pub regular_category_ids: Vec<serde_json::Value>,
    pub watched_tags: Vec<serde_json::Value>,
    pub watching_first_post_tags: Vec<serde_json::Value>,
    pub tracked_tags: Vec<serde_json::Value>,
    pub muted_tags: Vec<serde_json::Value>,
    pub tracked_category_ids: Vec<serde_json::Value>,
    pub watched_category_ids: Vec<serde_json::Value>,
    pub watched_first_post_category_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_avatar_upload_id: Option<String>,
    pub system_avatar_template: String,
    pub muted_usernames: Vec<serde_json::Value>,
    pub ignored_usernames: Vec<serde_json::Value>,
    pub allowed_pm_usernames: Vec<serde_json::Value>,
    pub mailing_list_posts_per_day: i64,
    pub can_change_bio: bool,
    pub can_change_location: bool,
    pub can_change_website: bool,
    pub can_change_tracking_preferences: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_api_keys: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_passkeys: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_category_ids: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_sidebar_tags: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_pick_theme_with_custom_homepage: Option<bool>,
    pub user_auth_tokens: Vec<UserAuthTokens>,
    pub user_notification_schedule: UserNotificationSchedule,
    pub use_logo_small_as_avatar: bool,
    pub featured_user_badge_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
    pub groups: Vec<GetUserResponseUserGroups>,
    pub group_users: Vec<GroupUsers>,
    pub user_option: UserOption,
}

impl std::fmt::Display for GetUserResponseUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserResponseUser {
    const LENGTH: usize = 83;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.last_posted_at).into(),
            format!("{:?}", self.last_seen_at).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.ignored).into(),
            format!("{:?}", self.muted).into(),
            format!("{:?}", self.can_ignore_user).into(),
            if let Some(can_ignore_users) = &self.can_ignore_users {
                format!("{:?}", can_ignore_users).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_mute_user).into(),
            if let Some(can_mute_users) = &self.can_mute_users {
                format!("{:?}", can_mute_users).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_send_private_messages).into(),
            format!("{:?}", self.can_send_private_message_to_user).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.badge_count).into(),
            if let Some(second_factor_backup_enabled) = &self.second_factor_backup_enabled {
                format!("{:?}", second_factor_backup_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(user_fields) = &self.user_fields {
                format!("{:?}", user_fields).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.custom_fields).into(),
            format!("{:?}", self.time_read).into(),
            format!("{:?}", self.recent_time_read).into(),
            format!("{:?}", self.primary_group_id).into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_group_id).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.featured_topic).into(),
            format!("{:?}", self.staged).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_edit_username).into(),
            format!("{:?}", self.can_edit_email).into(),
            format!("{:?}", self.can_edit_name).into(),
            format!("{:?}", self.uploaded_avatar_id).into(),
            format!("{:?}", self.has_title_badges).into(),
            format!("{:?}", self.pending_count).into(),
            if let Some(pending_posts_count) = &self.pending_posts_count {
                format!("{:?}", pending_posts_count).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.profile_view_count).into(),
            format!("{:?}", self.second_factor_enabled).into(),
            format!("{:?}", self.can_upload_profile_header).into(),
            format!("{:?}", self.can_upload_user_card_background).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.can_be_deleted).into(),
            format!("{:?}", self.can_delete_all_posts).into(),
            format!("{:?}", self.locale).into(),
            format!("{:?}", self.muted_category_ids).into(),
            format!("{:?}", self.regular_category_ids).into(),
            format!("{:?}", self.watched_tags).into(),
            format!("{:?}", self.watching_first_post_tags).into(),
            format!("{:?}", self.tracked_tags).into(),
            format!("{:?}", self.muted_tags).into(),
            format!("{:?}", self.tracked_category_ids).into(),
            format!("{:?}", self.watched_category_ids).into(),
            format!("{:?}", self.watched_first_post_category_ids).into(),
            format!("{:?}", self.system_avatar_upload_id).into(),
            self.system_avatar_template.clone().into(),
            format!("{:?}", self.muted_usernames).into(),
            format!("{:?}", self.ignored_usernames).into(),
            format!("{:?}", self.allowed_pm_usernames).into(),
            format!("{:?}", self.mailing_list_posts_per_day).into(),
            format!("{:?}", self.can_change_bio).into(),
            format!("{:?}", self.can_change_location).into(),
            format!("{:?}", self.can_change_website).into(),
            format!("{:?}", self.can_change_tracking_preferences).into(),
            format!("{:?}", self.user_api_keys).into(),
            if let Some(user_passkeys) = &self.user_passkeys {
                format!("{:?}", user_passkeys).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_tags) = &self.sidebar_tags {
                format!("{:?}", sidebar_tags).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_category_ids) = &self.sidebar_category_ids {
                format!("{:?}", sidebar_category_ids).into()
            } else {
                String::new().into()
            },
            if let Some(display_sidebar_tags) = &self.display_sidebar_tags {
                format!("{:?}", display_sidebar_tags).into()
            } else {
                String::new().into()
            },
            if let Some(can_pick_theme_with_custom_homepage) =
                &self.can_pick_theme_with_custom_homepage
            {
                format!("{:?}", can_pick_theme_with_custom_homepage).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_auth_tokens).into(),
            format!("{:?}", self.user_notification_schedule).into(),
            format!("{:?}", self.use_logo_small_as_avatar).into(),
            format!("{:?}", self.featured_user_badge_ids).into(),
            format!("{:?}", self.invited_by).into(),
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.group_users).into(),
            format!("{:?}", self.user_option).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "last_posted_at".into(),
            "last_seen_at".into(),
            "created_at".into(),
            "ignored".into(),
            "muted".into(),
            "can_ignore_user".into(),
            "can_ignore_users".into(),
            "can_mute_user".into(),
            "can_mute_users".into(),
            "can_send_private_messages".into(),
            "can_send_private_message_to_user".into(),
            "trust_level".into(),
            "moderator".into(),
            "admin".into(),
            "title".into(),
            "badge_count".into(),
            "second_factor_backup_enabled".into(),
            "user_fields".into(),
            "custom_fields".into(),
            "time_read".into(),
            "recent_time_read".into(),
            "primary_group_id".into(),
            "primary_group_name".into(),
            "flair_group_id".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "featured_topic".into(),
            "staged".into(),
            "can_edit".into(),
            "can_edit_username".into(),
            "can_edit_email".into(),
            "can_edit_name".into(),
            "uploaded_avatar_id".into(),
            "has_title_badges".into(),
            "pending_count".into(),
            "pending_posts_count".into(),
            "profile_view_count".into(),
            "second_factor_enabled".into(),
            "can_upload_profile_header".into(),
            "can_upload_user_card_background".into(),
            "post_count".into(),
            "can_be_deleted".into(),
            "can_delete_all_posts".into(),
            "locale".into(),
            "muted_category_ids".into(),
            "regular_category_ids".into(),
            "watched_tags".into(),
            "watching_first_post_tags".into(),
            "tracked_tags".into(),
            "muted_tags".into(),
            "tracked_category_ids".into(),
            "watched_category_ids".into(),
            "watched_first_post_category_ids".into(),
            "system_avatar_upload_id".into(),
            "system_avatar_template".into(),
            "muted_usernames".into(),
            "ignored_usernames".into(),
            "allowed_pm_usernames".into(),
            "mailing_list_posts_per_day".into(),
            "can_change_bio".into(),
            "can_change_location".into(),
            "can_change_website".into(),
            "can_change_tracking_preferences".into(),
            "user_api_keys".into(),
            "user_passkeys".into(),
            "sidebar_tags".into(),
            "sidebar_category_ids".into(),
            "display_sidebar_tags".into(),
            "can_pick_theme_with_custom_homepage".into(),
            "user_auth_tokens".into(),
            "user_notification_schedule".into(),
            "use_logo_small_as_avatar".into(),
            "featured_user_badge_ids".into(),
            "invited_by".into(),
            "groups".into(),
            "group_users".into(),
            "user_option".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserResponse {
    pub user_badges: Vec<serde_json::Value>,
    pub user: GetUserResponseUser,
}

impl std::fmt::Display for GetUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user_badges).into(),
            format!("{:?}", self.user).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_badges".into(), "user".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUserRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<ExternalIds>,
}

impl std::fmt::Display for UpdateUserRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateUserRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(external_ids) = &self.external_ids {
                format!("{:?}", external_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "external_ids".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUserResponseUser {}

impl std::fmt::Display for UpdateUserResponseUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateUserResponseUser {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUserResponse {
    pub success: String,
    pub user: UpdateUserResponseUser,
}

impl std::fmt::Display for UpdateUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateUserResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.success.clone().into(),
            format!("{:?}", self.user).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into(), "user".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserExternalIdResponseUserUserFields {
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub field_1: Option<String>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub field_2: Option<String>,
}

impl std::fmt::Display for GetUserExternalIdResponseUserUserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserExternalIdResponseUserUserFields {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.field_1).into(),
            format!("{:?}", self.field_2).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["field_1".into(), "field_2".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserExternalIdResponseUserCustomFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

impl std::fmt::Display for GetUserExternalIdResponseUserCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserExternalIdResponseUserCustomFields {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(first_name) = &self.first_name {
            format!("{:?}", first_name).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["first_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserExternalIdResponseUserGroups {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub display_name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    pub publish_read_state: bool,
}

impl std::fmt::Display for GetUserExternalIdResponseUserGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserExternalIdResponseUserGroups {
    const LENGTH: usize = 29;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            self.display_name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            format!("{:?}", self.publish_read_state).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "display_name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "publish_read_state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserExternalIdResponseUser {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    pub created_at: String,
    pub ignored: bool,
    pub muted: bool,
    pub can_ignore_user: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_ignore_users: Option<bool>,
    pub can_mute_user: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_mute_users: Option<bool>,
    pub can_send_private_messages: bool,
    pub can_send_private_message_to_user: bool,
    pub trust_level: i64,
    pub moderator: bool,
    pub admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub badge_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_factor_backup_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<GetUserExternalIdResponseUserUserFields>,
    pub custom_fields: GetUserExternalIdResponseUserCustomFields,
    pub time_read: i64,
    pub recent_time_read: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_topic: Option<String>,
    pub staged: bool,
    pub can_edit: bool,
    pub can_edit_username: bool,
    pub can_edit_email: bool,
    pub can_edit_name: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_avatar_id: Option<i64>,
    pub has_title_badges: bool,
    pub pending_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_posts_count: Option<i64>,
    pub profile_view_count: i64,
    pub second_factor_enabled: bool,
    pub can_upload_profile_header: bool,
    pub can_upload_user_card_background: bool,
    pub post_count: i64,
    pub can_be_deleted: bool,
    pub can_delete_all_posts: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    pub muted_category_ids: Vec<serde_json::Value>,
    pub regular_category_ids: Vec<serde_json::Value>,
    pub watched_tags: Vec<serde_json::Value>,
    pub watching_first_post_tags: Vec<serde_json::Value>,
    pub tracked_tags: Vec<serde_json::Value>,
    pub muted_tags: Vec<serde_json::Value>,
    pub tracked_category_ids: Vec<serde_json::Value>,
    pub watched_category_ids: Vec<serde_json::Value>,
    pub watched_first_post_category_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_avatar_upload_id: Option<String>,
    pub system_avatar_template: String,
    pub muted_usernames: Vec<serde_json::Value>,
    pub ignored_usernames: Vec<serde_json::Value>,
    pub allowed_pm_usernames: Vec<serde_json::Value>,
    pub mailing_list_posts_per_day: i64,
    pub can_change_bio: bool,
    pub can_change_location: bool,
    pub can_change_website: bool,
    pub can_change_tracking_preferences: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_api_keys: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_passkeys: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_category_ids: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_sidebar_tags: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_pick_theme_with_custom_homepage: Option<bool>,
    pub user_auth_tokens: Vec<UserAuthTokens>,
    pub user_notification_schedule: UserNotificationSchedule,
    pub use_logo_small_as_avatar: bool,
    pub featured_user_badge_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
    pub groups: Vec<GetUserExternalIdResponseUserGroups>,
    pub group_users: Vec<GroupUsers>,
    pub user_option: UserOption,
}

impl std::fmt::Display for GetUserExternalIdResponseUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserExternalIdResponseUser {
    const LENGTH: usize = 83;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.last_posted_at).into(),
            format!("{:?}", self.last_seen_at).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.ignored).into(),
            format!("{:?}", self.muted).into(),
            format!("{:?}", self.can_ignore_user).into(),
            if let Some(can_ignore_users) = &self.can_ignore_users {
                format!("{:?}", can_ignore_users).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_mute_user).into(),
            if let Some(can_mute_users) = &self.can_mute_users {
                format!("{:?}", can_mute_users).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_send_private_messages).into(),
            format!("{:?}", self.can_send_private_message_to_user).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.badge_count).into(),
            if let Some(second_factor_backup_enabled) = &self.second_factor_backup_enabled {
                format!("{:?}", second_factor_backup_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(user_fields) = &self.user_fields {
                format!("{:?}", user_fields).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.custom_fields).into(),
            format!("{:?}", self.time_read).into(),
            format!("{:?}", self.recent_time_read).into(),
            format!("{:?}", self.primary_group_id).into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_group_id).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.featured_topic).into(),
            format!("{:?}", self.staged).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_edit_username).into(),
            format!("{:?}", self.can_edit_email).into(),
            format!("{:?}", self.can_edit_name).into(),
            format!("{:?}", self.uploaded_avatar_id).into(),
            format!("{:?}", self.has_title_badges).into(),
            format!("{:?}", self.pending_count).into(),
            if let Some(pending_posts_count) = &self.pending_posts_count {
                format!("{:?}", pending_posts_count).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.profile_view_count).into(),
            format!("{:?}", self.second_factor_enabled).into(),
            format!("{:?}", self.can_upload_profile_header).into(),
            format!("{:?}", self.can_upload_user_card_background).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.can_be_deleted).into(),
            format!("{:?}", self.can_delete_all_posts).into(),
            format!("{:?}", self.locale).into(),
            format!("{:?}", self.muted_category_ids).into(),
            format!("{:?}", self.regular_category_ids).into(),
            format!("{:?}", self.watched_tags).into(),
            format!("{:?}", self.watching_first_post_tags).into(),
            format!("{:?}", self.tracked_tags).into(),
            format!("{:?}", self.muted_tags).into(),
            format!("{:?}", self.tracked_category_ids).into(),
            format!("{:?}", self.watched_category_ids).into(),
            format!("{:?}", self.watched_first_post_category_ids).into(),
            format!("{:?}", self.system_avatar_upload_id).into(),
            self.system_avatar_template.clone().into(),
            format!("{:?}", self.muted_usernames).into(),
            format!("{:?}", self.ignored_usernames).into(),
            format!("{:?}", self.allowed_pm_usernames).into(),
            format!("{:?}", self.mailing_list_posts_per_day).into(),
            format!("{:?}", self.can_change_bio).into(),
            format!("{:?}", self.can_change_location).into(),
            format!("{:?}", self.can_change_website).into(),
            format!("{:?}", self.can_change_tracking_preferences).into(),
            format!("{:?}", self.user_api_keys).into(),
            if let Some(user_passkeys) = &self.user_passkeys {
                format!("{:?}", user_passkeys).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_tags) = &self.sidebar_tags {
                format!("{:?}", sidebar_tags).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_category_ids) = &self.sidebar_category_ids {
                format!("{:?}", sidebar_category_ids).into()
            } else {
                String::new().into()
            },
            if let Some(display_sidebar_tags) = &self.display_sidebar_tags {
                format!("{:?}", display_sidebar_tags).into()
            } else {
                String::new().into()
            },
            if let Some(can_pick_theme_with_custom_homepage) =
                &self.can_pick_theme_with_custom_homepage
            {
                format!("{:?}", can_pick_theme_with_custom_homepage).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_auth_tokens).into(),
            format!("{:?}", self.user_notification_schedule).into(),
            format!("{:?}", self.use_logo_small_as_avatar).into(),
            format!("{:?}", self.featured_user_badge_ids).into(),
            format!("{:?}", self.invited_by).into(),
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.group_users).into(),
            format!("{:?}", self.user_option).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "last_posted_at".into(),
            "last_seen_at".into(),
            "created_at".into(),
            "ignored".into(),
            "muted".into(),
            "can_ignore_user".into(),
            "can_ignore_users".into(),
            "can_mute_user".into(),
            "can_mute_users".into(),
            "can_send_private_messages".into(),
            "can_send_private_message_to_user".into(),
            "trust_level".into(),
            "moderator".into(),
            "admin".into(),
            "title".into(),
            "badge_count".into(),
            "second_factor_backup_enabled".into(),
            "user_fields".into(),
            "custom_fields".into(),
            "time_read".into(),
            "recent_time_read".into(),
            "primary_group_id".into(),
            "primary_group_name".into(),
            "flair_group_id".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "featured_topic".into(),
            "staged".into(),
            "can_edit".into(),
            "can_edit_username".into(),
            "can_edit_email".into(),
            "can_edit_name".into(),
            "uploaded_avatar_id".into(),
            "has_title_badges".into(),
            "pending_count".into(),
            "pending_posts_count".into(),
            "profile_view_count".into(),
            "second_factor_enabled".into(),
            "can_upload_profile_header".into(),
            "can_upload_user_card_background".into(),
            "post_count".into(),
            "can_be_deleted".into(),
            "can_delete_all_posts".into(),
            "locale".into(),
            "muted_category_ids".into(),
            "regular_category_ids".into(),
            "watched_tags".into(),
            "watching_first_post_tags".into(),
            "tracked_tags".into(),
            "muted_tags".into(),
            "tracked_category_ids".into(),
            "watched_category_ids".into(),
            "watched_first_post_category_ids".into(),
            "system_avatar_upload_id".into(),
            "system_avatar_template".into(),
            "muted_usernames".into(),
            "ignored_usernames".into(),
            "allowed_pm_usernames".into(),
            "mailing_list_posts_per_day".into(),
            "can_change_bio".into(),
            "can_change_location".into(),
            "can_change_website".into(),
            "can_change_tracking_preferences".into(),
            "user_api_keys".into(),
            "user_passkeys".into(),
            "sidebar_tags".into(),
            "sidebar_category_ids".into(),
            "display_sidebar_tags".into(),
            "can_pick_theme_with_custom_homepage".into(),
            "user_auth_tokens".into(),
            "user_notification_schedule".into(),
            "use_logo_small_as_avatar".into(),
            "featured_user_badge_ids".into(),
            "invited_by".into(),
            "groups".into(),
            "group_users".into(),
            "user_option".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserExternalIdResponse {
    pub user_badges: Vec<serde_json::Value>,
    pub user: GetUserExternalIdResponseUser,
}

impl std::fmt::Display for GetUserExternalIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserExternalIdResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user_badges).into(),
            format!("{:?}", self.user).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_badges".into(), "user".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserIdentiyProviderExternalIdResponseUserUserFields {
    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub field_1: Option<String>,
    #[serde(rename = "2", default, skip_serializing_if = "Option::is_none")]
    pub field_2: Option<String>,
}

impl std::fmt::Display for GetUserIdentiyProviderExternalIdResponseUserUserFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserIdentiyProviderExternalIdResponseUserUserFields {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.field_1).into(),
            format!("{:?}", self.field_2).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["field_1".into(), "field_2".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserIdentiyProviderExternalIdResponseUserCustomFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

impl std::fmt::Display for GetUserIdentiyProviderExternalIdResponseUserCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserIdentiyProviderExternalIdResponseUserCustomFields {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(first_name) = &self.first_name {
            format!("{:?}", first_name).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["first_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserIdentiyProviderExternalIdResponseUserGroups {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub display_name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    pub publish_read_state: bool,
}

impl std::fmt::Display for GetUserIdentiyProviderExternalIdResponseUserGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserIdentiyProviderExternalIdResponseUserGroups {
    const LENGTH: usize = 29;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            self.display_name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            format!("{:?}", self.publish_read_state).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "display_name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "publish_read_state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserIdentiyProviderExternalIdResponseUser {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_posted_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    pub created_at: String,
    pub ignored: bool,
    pub muted: bool,
    pub can_ignore_user: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_ignore_users: Option<bool>,
    pub can_mute_user: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_mute_users: Option<bool>,
    pub can_send_private_messages: bool,
    pub can_send_private_message_to_user: bool,
    pub trust_level: i64,
    pub moderator: bool,
    pub admin: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub badge_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub second_factor_backup_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_fields: Option<GetUserIdentiyProviderExternalIdResponseUserUserFields>,
    pub custom_fields: GetUserIdentiyProviderExternalIdResponseUserCustomFields,
    pub time_read: i64,
    pub recent_time_read: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_topic: Option<String>,
    pub staged: bool,
    pub can_edit: bool,
    pub can_edit_username: bool,
    pub can_edit_email: bool,
    pub can_edit_name: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded_avatar_id: Option<i64>,
    pub has_title_badges: bool,
    pub pending_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_posts_count: Option<i64>,
    pub profile_view_count: i64,
    pub second_factor_enabled: bool,
    pub can_upload_profile_header: bool,
    pub can_upload_user_card_background: bool,
    pub post_count: i64,
    pub can_be_deleted: bool,
    pub can_delete_all_posts: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    pub muted_category_ids: Vec<serde_json::Value>,
    pub regular_category_ids: Vec<serde_json::Value>,
    pub watched_tags: Vec<serde_json::Value>,
    pub watching_first_post_tags: Vec<serde_json::Value>,
    pub tracked_tags: Vec<serde_json::Value>,
    pub muted_tags: Vec<serde_json::Value>,
    pub tracked_category_ids: Vec<serde_json::Value>,
    pub watched_category_ids: Vec<serde_json::Value>,
    pub watched_first_post_category_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_avatar_upload_id: Option<String>,
    pub system_avatar_template: String,
    pub muted_usernames: Vec<serde_json::Value>,
    pub ignored_usernames: Vec<serde_json::Value>,
    pub allowed_pm_usernames: Vec<serde_json::Value>,
    pub mailing_list_posts_per_day: i64,
    pub can_change_bio: bool,
    pub can_change_location: bool,
    pub can_change_website: bool,
    pub can_change_tracking_preferences: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_api_keys: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_passkeys: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_tags: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sidebar_category_ids: Option<Vec<serde_json::Value>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_sidebar_tags: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_pick_theme_with_custom_homepage: Option<bool>,
    pub user_auth_tokens: Vec<UserAuthTokens>,
    pub user_notification_schedule: UserNotificationSchedule,
    pub use_logo_small_as_avatar: bool,
    pub featured_user_badge_ids: Vec<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invited_by: Option<String>,
    pub groups: Vec<GetUserIdentiyProviderExternalIdResponseUserGroups>,
    pub group_users: Vec<GroupUsers>,
    pub user_option: UserOption,
}

impl std::fmt::Display for GetUserIdentiyProviderExternalIdResponseUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserIdentiyProviderExternalIdResponseUser {
    const LENGTH: usize = 83;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.last_posted_at).into(),
            format!("{:?}", self.last_seen_at).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.ignored).into(),
            format!("{:?}", self.muted).into(),
            format!("{:?}", self.can_ignore_user).into(),
            if let Some(can_ignore_users) = &self.can_ignore_users {
                format!("{:?}", can_ignore_users).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_mute_user).into(),
            if let Some(can_mute_users) = &self.can_mute_users {
                format!("{:?}", can_mute_users).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_send_private_messages).into(),
            format!("{:?}", self.can_send_private_message_to_user).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.badge_count).into(),
            if let Some(second_factor_backup_enabled) = &self.second_factor_backup_enabled {
                format!("{:?}", second_factor_backup_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(user_fields) = &self.user_fields {
                format!("{:?}", user_fields).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.custom_fields).into(),
            format!("{:?}", self.time_read).into(),
            format!("{:?}", self.recent_time_read).into(),
            format!("{:?}", self.primary_group_id).into(),
            format!("{:?}", self.primary_group_name).into(),
            format!("{:?}", self.flair_group_id).into(),
            format!("{:?}", self.flair_name).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            format!("{:?}", self.featured_topic).into(),
            format!("{:?}", self.staged).into(),
            format!("{:?}", self.can_edit).into(),
            format!("{:?}", self.can_edit_username).into(),
            format!("{:?}", self.can_edit_email).into(),
            format!("{:?}", self.can_edit_name).into(),
            format!("{:?}", self.uploaded_avatar_id).into(),
            format!("{:?}", self.has_title_badges).into(),
            format!("{:?}", self.pending_count).into(),
            if let Some(pending_posts_count) = &self.pending_posts_count {
                format!("{:?}", pending_posts_count).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.profile_view_count).into(),
            format!("{:?}", self.second_factor_enabled).into(),
            format!("{:?}", self.can_upload_profile_header).into(),
            format!("{:?}", self.can_upload_user_card_background).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.can_be_deleted).into(),
            format!("{:?}", self.can_delete_all_posts).into(),
            format!("{:?}", self.locale).into(),
            format!("{:?}", self.muted_category_ids).into(),
            format!("{:?}", self.regular_category_ids).into(),
            format!("{:?}", self.watched_tags).into(),
            format!("{:?}", self.watching_first_post_tags).into(),
            format!("{:?}", self.tracked_tags).into(),
            format!("{:?}", self.muted_tags).into(),
            format!("{:?}", self.tracked_category_ids).into(),
            format!("{:?}", self.watched_category_ids).into(),
            format!("{:?}", self.watched_first_post_category_ids).into(),
            format!("{:?}", self.system_avatar_upload_id).into(),
            self.system_avatar_template.clone().into(),
            format!("{:?}", self.muted_usernames).into(),
            format!("{:?}", self.ignored_usernames).into(),
            format!("{:?}", self.allowed_pm_usernames).into(),
            format!("{:?}", self.mailing_list_posts_per_day).into(),
            format!("{:?}", self.can_change_bio).into(),
            format!("{:?}", self.can_change_location).into(),
            format!("{:?}", self.can_change_website).into(),
            format!("{:?}", self.can_change_tracking_preferences).into(),
            format!("{:?}", self.user_api_keys).into(),
            if let Some(user_passkeys) = &self.user_passkeys {
                format!("{:?}", user_passkeys).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_tags) = &self.sidebar_tags {
                format!("{:?}", sidebar_tags).into()
            } else {
                String::new().into()
            },
            if let Some(sidebar_category_ids) = &self.sidebar_category_ids {
                format!("{:?}", sidebar_category_ids).into()
            } else {
                String::new().into()
            },
            if let Some(display_sidebar_tags) = &self.display_sidebar_tags {
                format!("{:?}", display_sidebar_tags).into()
            } else {
                String::new().into()
            },
            if let Some(can_pick_theme_with_custom_homepage) =
                &self.can_pick_theme_with_custom_homepage
            {
                format!("{:?}", can_pick_theme_with_custom_homepage).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_auth_tokens).into(),
            format!("{:?}", self.user_notification_schedule).into(),
            format!("{:?}", self.use_logo_small_as_avatar).into(),
            format!("{:?}", self.featured_user_badge_ids).into(),
            format!("{:?}", self.invited_by).into(),
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.group_users).into(),
            format!("{:?}", self.user_option).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "last_posted_at".into(),
            "last_seen_at".into(),
            "created_at".into(),
            "ignored".into(),
            "muted".into(),
            "can_ignore_user".into(),
            "can_ignore_users".into(),
            "can_mute_user".into(),
            "can_mute_users".into(),
            "can_send_private_messages".into(),
            "can_send_private_message_to_user".into(),
            "trust_level".into(),
            "moderator".into(),
            "admin".into(),
            "title".into(),
            "badge_count".into(),
            "second_factor_backup_enabled".into(),
            "user_fields".into(),
            "custom_fields".into(),
            "time_read".into(),
            "recent_time_read".into(),
            "primary_group_id".into(),
            "primary_group_name".into(),
            "flair_group_id".into(),
            "flair_name".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "featured_topic".into(),
            "staged".into(),
            "can_edit".into(),
            "can_edit_username".into(),
            "can_edit_email".into(),
            "can_edit_name".into(),
            "uploaded_avatar_id".into(),
            "has_title_badges".into(),
            "pending_count".into(),
            "pending_posts_count".into(),
            "profile_view_count".into(),
            "second_factor_enabled".into(),
            "can_upload_profile_header".into(),
            "can_upload_user_card_background".into(),
            "post_count".into(),
            "can_be_deleted".into(),
            "can_delete_all_posts".into(),
            "locale".into(),
            "muted_category_ids".into(),
            "regular_category_ids".into(),
            "watched_tags".into(),
            "watching_first_post_tags".into(),
            "tracked_tags".into(),
            "muted_tags".into(),
            "tracked_category_ids".into(),
            "watched_category_ids".into(),
            "watched_first_post_category_ids".into(),
            "system_avatar_upload_id".into(),
            "system_avatar_template".into(),
            "muted_usernames".into(),
            "ignored_usernames".into(),
            "allowed_pm_usernames".into(),
            "mailing_list_posts_per_day".into(),
            "can_change_bio".into(),
            "can_change_location".into(),
            "can_change_website".into(),
            "can_change_tracking_preferences".into(),
            "user_api_keys".into(),
            "user_passkeys".into(),
            "sidebar_tags".into(),
            "sidebar_category_ids".into(),
            "display_sidebar_tags".into(),
            "can_pick_theme_with_custom_homepage".into(),
            "user_auth_tokens".into(),
            "user_notification_schedule".into(),
            "use_logo_small_as_avatar".into(),
            "featured_user_badge_ids".into(),
            "invited_by".into(),
            "groups".into(),
            "group_users".into(),
            "user_option".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserIdentiyProviderExternalIdResponse {
    pub user_badges: Vec<serde_json::Value>,
    pub user: GetUserIdentiyProviderExternalIdResponseUser,
}

impl std::fmt::Display for GetUserIdentiyProviderExternalIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserIdentiyProviderExternalIdResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user_badges).into(),
            format!("{:?}", self.user).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_badges".into(), "user".into()]
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
pub enum UpdateAvatarRequestBodyType {
    #[serde(rename = "uploaded")]
    #[display("uploaded")]
    Uploaded,
    #[serde(rename = "custom")]
    #[display("custom")]
    Custom,
    #[serde(rename = "gravatar")]
    #[display("gravatar")]
    Gravatar,
    #[serde(rename = "system")]
    #[display("system")]
    System,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateAvatarRequestBody {
    pub upload_id: i64,
    #[serde(rename = "type")]
    pub type_: UpdateAvatarRequestBodyType,
}

impl std::fmt::Display for UpdateAvatarRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateAvatarRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.upload_id).into(),
            format!("{:?}", self.type_).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["upload_id".into(), "type_".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateAvatarResponse {
    pub success: String,
}

impl std::fmt::Display for UpdateAvatarResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateAvatarResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateEmailRequestBody {
    pub email: String,
}

impl std::fmt::Display for UpdateEmailRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateEmailRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.email.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["email".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateUsernameRequestBody {
    pub new_username: String,
}

impl std::fmt::Display for UpdateUsernameRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateUsernameRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.new_username.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["new_username".into()]
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
pub enum Asc {
    #[serde(rename = "true")]
    #[display("true")]
    True,
}

impl std::default::Default for Asc {
    fn default() -> Self {
        Asc::True
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
pub enum ListPublicOrder {
    #[serde(rename = "likes_received")]
    #[display("likes_received")]
    LikesReceived,
    #[serde(rename = "likes_given")]
    #[display("likes_given")]
    LikesGiven,
    #[serde(rename = "topic_count")]
    #[display("topic_count")]
    TopicCount,
    #[serde(rename = "post_count")]
    #[display("post_count")]
    PostCount,
    #[serde(rename = "topics_entered")]
    #[display("topics_entered")]
    TopicsEntered,
    #[serde(rename = "posts_read")]
    #[display("posts_read")]
    PostsRead,
    #[serde(rename = "days_visited")]
    #[display("days_visited")]
    DaysVisited,
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
pub enum ListPublicPeriod {
    #[serde(rename = "daily")]
    #[display("daily")]
    Daily,
    #[serde(rename = "weekly")]
    #[display("weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    #[display("monthly")]
    Monthly,
    #[serde(rename = "quarterly")]
    #[display("quarterly")]
    Quarterly,
    #[serde(rename = "yearly")]
    #[display("yearly")]
    Yearly,
    #[serde(rename = "all")]
    #[display("all")]
    All,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DirectoryItemsUser {
    pub id: i64,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl std::fmt::Display for DirectoryItemsUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DirectoryItemsUser {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            format!("{:?}", self.name).into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.title).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "title".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DirectoryItems {
    pub id: i64,
    pub likes_received: i64,
    pub likes_given: i64,
    pub topics_entered: i64,
    pub topic_count: i64,
    pub post_count: i64,
    pub posts_read: i64,
    pub days_visited: i64,
    pub user: DirectoryItemsUser,
}

impl std::fmt::Display for DirectoryItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DirectoryItems {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.likes_received).into(),
            format!("{:?}", self.likes_given).into(),
            format!("{:?}", self.topics_entered).into(),
            format!("{:?}", self.topic_count).into(),
            format!("{:?}", self.post_count).into(),
            format!("{:?}", self.posts_read).into(),
            format!("{:?}", self.days_visited).into(),
            format!("{:?}", self.user).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "likes_received".into(),
            "likes_given".into(),
            "topics_entered".into(),
            "topic_count".into(),
            "post_count".into(),
            "posts_read".into(),
            "days_visited".into(),
            "user".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUsersPublicResponseMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    pub total_rows_directory_items: i64,
    pub load_more_directory_items: String,
}

impl std::fmt::Display for ListUsersPublicResponseMeta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUsersPublicResponseMeta {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.last_updated_at).into(),
            format!("{:?}", self.total_rows_directory_items).into(),
            self.load_more_directory_items.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "last_updated_at".into(),
            "total_rows_directory_items".into(),
            "load_more_directory_items".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUsersPublicResponse {
    pub directory_items: Vec<DirectoryItems>,
    pub meta: ListUsersPublicResponseMeta,
}

impl std::fmt::Display for ListUsersPublicResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUsersPublicResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.directory_items).into(),
            format!("{:?}", self.meta).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["directory_items".into(), "meta".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LatestExport {}

impl std::fmt::Display for LatestExport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LatestExport {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApprovedBy {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
}

impl std::fmt::Display for ApprovedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApprovedBy {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PenaltyCounts {
    pub silenced: i64,
    pub suspended: i64,
}

impl std::fmt::Display for PenaltyCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PenaltyCounts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.silenced).into(),
            format!("{:?}", self.suspended).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["silenced".into(), "suspended".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Tl3RequirementsPenaltyCounts {
    pub silenced: i64,
    pub suspended: i64,
    pub total: i64,
}

impl std::fmt::Display for Tl3RequirementsPenaltyCounts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Tl3RequirementsPenaltyCounts {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.silenced).into(),
            format!("{:?}", self.suspended).into(),
            format!("{:?}", self.total).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["silenced".into(), "suspended".into(), "total".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Tl3Requirements {
    pub time_period: i64,
    pub requirements_met: bool,
    pub requirements_lost: bool,
    pub trust_level_locked: bool,
    pub on_grace_period: bool,
    pub days_visited: i64,
    pub min_days_visited: i64,
    pub num_topics_replied_to: i64,
    pub min_topics_replied_to: i64,
    pub topics_viewed: i64,
    pub min_topics_viewed: i64,
    pub posts_read: i64,
    pub min_posts_read: i64,
    pub topics_viewed_all_time: i64,
    pub min_topics_viewed_all_time: i64,
    pub posts_read_all_time: i64,
    pub min_posts_read_all_time: i64,
    pub num_flagged_posts: i64,
    pub max_flagged_posts: i64,
    pub num_flagged_by_users: i64,
    pub max_flagged_by_users: i64,
    pub num_likes_given: i64,
    pub min_likes_given: i64,
    pub num_likes_received: i64,
    pub min_likes_received: i64,
    pub num_likes_received_days: i64,
    pub min_likes_received_days: i64,
    pub num_likes_received_users: i64,
    pub min_likes_received_users: i64,
    pub penalty_counts: Tl3RequirementsPenaltyCounts,
}

impl std::fmt::Display for Tl3Requirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Tl3Requirements {
    const LENGTH: usize = 30;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.time_period).into(),
            format!("{:?}", self.requirements_met).into(),
            format!("{:?}", self.requirements_lost).into(),
            format!("{:?}", self.trust_level_locked).into(),
            format!("{:?}", self.on_grace_period).into(),
            format!("{:?}", self.days_visited).into(),
            format!("{:?}", self.min_days_visited).into(),
            format!("{:?}", self.num_topics_replied_to).into(),
            format!("{:?}", self.min_topics_replied_to).into(),
            format!("{:?}", self.topics_viewed).into(),
            format!("{:?}", self.min_topics_viewed).into(),
            format!("{:?}", self.posts_read).into(),
            format!("{:?}", self.min_posts_read).into(),
            format!("{:?}", self.topics_viewed_all_time).into(),
            format!("{:?}", self.min_topics_viewed_all_time).into(),
            format!("{:?}", self.posts_read_all_time).into(),
            format!("{:?}", self.min_posts_read_all_time).into(),
            format!("{:?}", self.num_flagged_posts).into(),
            format!("{:?}", self.max_flagged_posts).into(),
            format!("{:?}", self.num_flagged_by_users).into(),
            format!("{:?}", self.max_flagged_by_users).into(),
            format!("{:?}", self.num_likes_given).into(),
            format!("{:?}", self.min_likes_given).into(),
            format!("{:?}", self.num_likes_received).into(),
            format!("{:?}", self.min_likes_received).into(),
            format!("{:?}", self.num_likes_received_days).into(),
            format!("{:?}", self.min_likes_received_days).into(),
            format!("{:?}", self.num_likes_received_users).into(),
            format!("{:?}", self.min_likes_received_users).into(),
            format!("{:?}", self.penalty_counts).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "time_period".into(),
            "requirements_met".into(),
            "requirements_lost".into(),
            "trust_level_locked".into(),
            "on_grace_period".into(),
            "days_visited".into(),
            "min_days_visited".into(),
            "num_topics_replied_to".into(),
            "min_topics_replied_to".into(),
            "topics_viewed".into(),
            "min_topics_viewed".into(),
            "posts_read".into(),
            "min_posts_read".into(),
            "topics_viewed_all_time".into(),
            "min_topics_viewed_all_time".into(),
            "posts_read_all_time".into(),
            "min_posts_read_all_time".into(),
            "num_flagged_posts".into(),
            "max_flagged_posts".into(),
            "num_flagged_by_users".into(),
            "max_flagged_by_users".into(),
            "num_likes_given".into(),
            "min_likes_given".into(),
            "num_likes_received".into(),
            "min_likes_received".into(),
            "num_likes_received_days".into(),
            "min_likes_received_days".into(),
            "num_likes_received_users".into(),
            "min_likes_received_users".into(),
            "penalty_counts".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AdminGetUserResponseGroups {
    pub id: i64,
    pub automatic: bool,
    pub name: String,
    pub display_name: String,
    pub user_count: i64,
    pub mentionable_level: i64,
    pub messageable_level: i64,
    pub visibility_level: i64,
    pub primary_group: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grant_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incoming_email: Option<String>,
    pub has_messages: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_bg_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flair_group_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_raw: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_cooked: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bio_excerpt: Option<String>,
    pub public_admission: bool,
    pub public_exit: bool,
    pub allow_membership_requests: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub default_notification_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub membership_request_template: Option<String>,
    pub members_visibility_level: i64,
    pub can_see_members: bool,
    pub can_admin_group: bool,
    pub publish_read_state: bool,
}

impl std::fmt::Display for AdminGetUserResponseGroups {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AdminGetUserResponseGroups {
    const LENGTH: usize = 30;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            format!("{:?}", self.automatic).into(),
            self.name.clone().into(),
            self.display_name.clone().into(),
            format!("{:?}", self.user_count).into(),
            format!("{:?}", self.mentionable_level).into(),
            format!("{:?}", self.messageable_level).into(),
            format!("{:?}", self.visibility_level).into(),
            format!("{:?}", self.primary_group).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.grant_trust_level).into(),
            format!("{:?}", self.incoming_email).into(),
            format!("{:?}", self.has_messages).into(),
            format!("{:?}", self.flair_url).into(),
            format!("{:?}", self.flair_bg_color).into(),
            format!("{:?}", self.flair_color).into(),
            if let Some(flair_group_id) = &self.flair_group_id {
                format!("{:?}", flair_group_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.bio_raw).into(),
            format!("{:?}", self.bio_cooked).into(),
            format!("{:?}", self.bio_excerpt).into(),
            format!("{:?}", self.public_admission).into(),
            format!("{:?}", self.public_exit).into(),
            format!("{:?}", self.allow_membership_requests).into(),
            format!("{:?}", self.full_name).into(),
            format!("{:?}", self.default_notification_level).into(),
            format!("{:?}", self.membership_request_template).into(),
            format!("{:?}", self.members_visibility_level).into(),
            format!("{:?}", self.can_see_members).into(),
            format!("{:?}", self.can_admin_group).into(),
            format!("{:?}", self.publish_read_state).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "automatic".into(),
            "name".into(),
            "display_name".into(),
            "user_count".into(),
            "mentionable_level".into(),
            "messageable_level".into(),
            "visibility_level".into(),
            "primary_group".into(),
            "title".into(),
            "grant_trust_level".into(),
            "incoming_email".into(),
            "has_messages".into(),
            "flair_url".into(),
            "flair_bg_color".into(),
            "flair_color".into(),
            "flair_group_id".into(),
            "bio_raw".into(),
            "bio_cooked".into(),
            "bio_excerpt".into(),
            "public_admission".into(),
            "public_exit".into(),
            "allow_membership_requests".into(),
            "full_name".into(),
            "default_notification_level".into(),
            "membership_request_template".into(),
            "members_visibility_level".into(),
            "can_see_members".into(),
            "can_admin_group".into(),
            "publish_read_state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AdminGetUserResponse {
    pub id: i64,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
    pub active: bool,
    pub admin: bool,
    pub moderator: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_emailed_at: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_age: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_emailed_age: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at_age: Option<f64>,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_locked_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub time_read: i64,
    pub staged: bool,
    pub days_visited: i64,
    pub posts_read_count: i64,
    pub topics_entered: i64,
    pub post_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated_accounts: Option<Vec<serde_json::Value>>,
    pub can_send_activation_email: bool,
    pub can_activate: bool,
    pub can_deactivate: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_change_trust_level: Option<bool>,
    pub ip_address: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_ip_address: Option<String>,
    pub can_grant_admin: bool,
    pub can_revoke_admin: bool,
    pub can_grant_moderation: bool,
    pub can_revoke_moderation: bool,
    pub can_impersonate: bool,
    pub like_count: i64,
    pub like_given_count: i64,
    pub topic_count: i64,
    pub flags_given_count: i64,
    pub flags_received_count: i64,
    pub private_topics_count: i64,
    pub can_delete_all_posts: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_be_deleted: Option<bool>,
    pub can_be_anonymized: bool,
    pub can_be_merged: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_suspend_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_export: Option<LatestExport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silence_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_edits_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_group_id: Option<i64>,
    pub badge_count: i64,
    pub warnings_received_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounce_score: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_bounce_score_after: Option<String>,
    pub can_view_action_logs: bool,
    pub can_disable_second_factor: bool,
    pub can_delete_sso_record: bool,
    pub api_key_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similar_users_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub single_sign_on_record: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<ApprovedBy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub silenced_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub penalty_counts: Option<PenaltyCounts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_penalty: Option<String>,
    #[serde(
        rename = "tl3_requirements",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub tl_3_requirements: Option<Tl3Requirements>,
    pub groups: Vec<AdminGetUserResponseGroups>,
    pub external_ids: ExternalIds,
    pub include_ip: bool,
}

impl std::fmt::Display for AdminGetUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AdminGetUserResponse {
    const LENGTH: usize = 68;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            format!("{:?}", self.name).into(),
            self.avatar_template.clone().into(),
            format!("{:?}", self.active).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.last_seen_at).into(),
            format!("{:?}", self.last_emailed_at).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.last_seen_age).into(),
            format!("{:?}", self.last_emailed_age).into(),
            format!("{:?}", self.created_at_age).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.manual_locked_trust_level).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.time_read).into(),
            format!("{:?}", self.staged).into(),
            format!("{:?}", self.days_visited).into(),
            format!("{:?}", self.posts_read_count).into(),
            format!("{:?}", self.topics_entered).into(),
            format!("{:?}", self.post_count).into(),
            if let Some(associated_accounts) = &self.associated_accounts {
                format!("{:?}", associated_accounts).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_send_activation_email).into(),
            format!("{:?}", self.can_activate).into(),
            format!("{:?}", self.can_deactivate).into(),
            if let Some(can_change_trust_level) = &self.can_change_trust_level {
                format!("{:?}", can_change_trust_level).into()
            } else {
                String::new().into()
            },
            self.ip_address.clone().into(),
            format!("{:?}", self.registration_ip_address).into(),
            format!("{:?}", self.can_grant_admin).into(),
            format!("{:?}", self.can_revoke_admin).into(),
            format!("{:?}", self.can_grant_moderation).into(),
            format!("{:?}", self.can_revoke_moderation).into(),
            format!("{:?}", self.can_impersonate).into(),
            format!("{:?}", self.like_count).into(),
            format!("{:?}", self.like_given_count).into(),
            format!("{:?}", self.topic_count).into(),
            format!("{:?}", self.flags_given_count).into(),
            format!("{:?}", self.flags_received_count).into(),
            format!("{:?}", self.private_topics_count).into(),
            format!("{:?}", self.can_delete_all_posts).into(),
            if let Some(can_be_deleted) = &self.can_be_deleted {
                format!("{:?}", can_be_deleted).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.can_be_anonymized).into(),
            format!("{:?}", self.can_be_merged).into(),
            format!("{:?}", self.full_suspend_reason).into(),
            if let Some(latest_export) = &self.latest_export {
                format!("{:?}", latest_export).into()
            } else {
                String::new().into()
            },
            if let Some(silence_reason) = &self.silence_reason {
                format!("{:?}", silence_reason).into()
            } else {
                String::new().into()
            },
            if let Some(post_edits_count) = &self.post_edits_count {
                format!("{:?}", post_edits_count).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.primary_group_id).into(),
            format!("{:?}", self.badge_count).into(),
            format!("{:?}", self.warnings_received_count).into(),
            format!("{:?}", self.bounce_score).into(),
            format!("{:?}", self.reset_bounce_score_after).into(),
            format!("{:?}", self.can_view_action_logs).into(),
            format!("{:?}", self.can_disable_second_factor).into(),
            format!("{:?}", self.can_delete_sso_record).into(),
            format!("{:?}", self.api_key_count).into(),
            if let Some(similar_users_count) = &self.similar_users_count {
                format!("{:?}", similar_users_count).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.single_sign_on_record).into(),
            format!("{:?}", self.approved_by).into(),
            format!("{:?}", self.suspended_by).into(),
            format!("{:?}", self.silenced_by).into(),
            if let Some(penalty_counts) = &self.penalty_counts {
                format!("{:?}", penalty_counts).into()
            } else {
                String::new().into()
            },
            if let Some(next_penalty) = &self.next_penalty {
                format!("{:?}", next_penalty).into()
            } else {
                String::new().into()
            },
            if let Some(tl_3_requirements) = &self.tl_3_requirements {
                format!("{:?}", tl_3_requirements).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.groups).into(),
            format!("{:?}", self.external_ids).into(),
            format!("{:?}", self.include_ip).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "active".into(),
            "admin".into(),
            "moderator".into(),
            "last_seen_at".into(),
            "last_emailed_at".into(),
            "created_at".into(),
            "last_seen_age".into(),
            "last_emailed_age".into(),
            "created_at_age".into(),
            "trust_level".into(),
            "manual_locked_trust_level".into(),
            "title".into(),
            "time_read".into(),
            "staged".into(),
            "days_visited".into(),
            "posts_read_count".into(),
            "topics_entered".into(),
            "post_count".into(),
            "associated_accounts".into(),
            "can_send_activation_email".into(),
            "can_activate".into(),
            "can_deactivate".into(),
            "can_change_trust_level".into(),
            "ip_address".into(),
            "registration_ip_address".into(),
            "can_grant_admin".into(),
            "can_revoke_admin".into(),
            "can_grant_moderation".into(),
            "can_revoke_moderation".into(),
            "can_impersonate".into(),
            "like_count".into(),
            "like_given_count".into(),
            "topic_count".into(),
            "flags_given_count".into(),
            "flags_received_count".into(),
            "private_topics_count".into(),
            "can_delete_all_posts".into(),
            "can_be_deleted".into(),
            "can_be_anonymized".into(),
            "can_be_merged".into(),
            "full_suspend_reason".into(),
            "latest_export".into(),
            "silence_reason".into(),
            "post_edits_count".into(),
            "primary_group_id".into(),
            "badge_count".into(),
            "warnings_received_count".into(),
            "bounce_score".into(),
            "reset_bounce_score_after".into(),
            "can_view_action_logs".into(),
            "can_disable_second_factor".into(),
            "can_delete_sso_record".into(),
            "api_key_count".into(),
            "similar_users_count".into(),
            "single_sign_on_record".into(),
            "approved_by".into(),
            "suspended_by".into(),
            "silenced_by".into(),
            "penalty_counts".into(),
            "next_penalty".into(),
            "tl_3_requirements".into(),
            "groups".into(),
            "external_ids".into(),
            "include_ip".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeleteUserRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_posts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_email: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_urls: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_ip: Option<bool>,
}

impl std::fmt::Display for DeleteUserRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeleteUserRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(delete_posts) = &self.delete_posts {
                format!("{:?}", delete_posts).into()
            } else {
                String::new().into()
            },
            if let Some(block_email) = &self.block_email {
                format!("{:?}", block_email).into()
            } else {
                String::new().into()
            },
            if let Some(block_urls) = &self.block_urls {
                format!("{:?}", block_urls).into()
            } else {
                String::new().into()
            },
            if let Some(block_ip) = &self.block_ip {
                format!("{:?}", block_ip).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "delete_posts".into(),
            "block_email".into(),
            "block_urls".into(),
            "block_ip".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeleteUserResponse {
    pub deleted: bool,
}

impl std::fmt::Display for DeleteUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeleteUserResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.deleted).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["deleted".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ActivateUserResponse {
    pub success: String,
}

impl std::fmt::Display for ActivateUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ActivateUserResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeactivateUserResponse {
    pub success: String,
}

impl std::fmt::Display for DeactivateUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeactivateUserResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SuspendUserRequestBody {
    pub suspend_until: String,
    pub reason: String,
    #[doc = "Will send an email with this message when present"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_action: Option<String>,
}

impl std::fmt::Display for SuspendUserRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SuspendUserRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.suspend_until.clone().into(),
            self.reason.clone().into(),
            if let Some(message) = &self.message {
                format!("{:?}", message).into()
            } else {
                String::new().into()
            },
            if let Some(post_action) = &self.post_action {
                format!("{:?}", post_action).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "suspend_until".into(),
            "reason".into(),
            "message".into(),
            "post_action".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SuspendedBy {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
}

impl std::fmt::Display for SuspendedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SuspendedBy {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Suspension {
    pub suspend_reason: String,
    pub full_suspend_reason: String,
    pub suspended_till: String,
    pub suspended_at: String,
    pub suspended_by: SuspendedBy,
}

impl std::fmt::Display for Suspension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Suspension {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.suspend_reason.clone().into(),
            self.full_suspend_reason.clone().into(),
            self.suspended_till.clone().into(),
            self.suspended_at.clone().into(),
            format!("{:?}", self.suspended_by).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "suspend_reason".into(),
            "full_suspend_reason".into(),
            "suspended_till".into(),
            "suspended_at".into(),
            "suspended_by".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SuspendUserResponse {
    pub suspension: Suspension,
}

impl std::fmt::Display for SuspendUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SuspendUserResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.suspension).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["suspension".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SilenceUserRequestBody {
    pub silenced_till: String,
    pub reason: String,
    #[doc = "Will send an email with this message when present"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_action: Option<String>,
}

impl std::fmt::Display for SilenceUserRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SilenceUserRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.silenced_till.clone().into(),
            self.reason.clone().into(),
            if let Some(message) = &self.message {
                format!("{:?}", message).into()
            } else {
                String::new().into()
            },
            if let Some(post_action) = &self.post_action {
                format!("{:?}", post_action).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "silenced_till".into(),
            "reason".into(),
            "message".into(),
            "post_action".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SilencedBy {
    pub id: i64,
    pub username: String,
    pub name: String,
    pub avatar_template: String,
}

impl std::fmt::Display for SilencedBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SilencedBy {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            self.name.clone().into(),
            self.avatar_template.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Silence {
    pub silenced: bool,
    pub silence_reason: String,
    pub silenced_till: String,
    pub silenced_at: String,
    pub silenced_by: SilencedBy,
}

impl std::fmt::Display for Silence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Silence {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.silenced).into(),
            self.silence_reason.clone().into(),
            self.silenced_till.clone().into(),
            self.silenced_at.clone().into(),
            format!("{:?}", self.silenced_by).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "silenced".into(),
            "silence_reason".into(),
            "silenced_till".into(),
            "silenced_at".into(),
            "silenced_by".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SilenceUserResponse {
    pub silence: Silence,
}

impl std::fmt::Display for SilenceUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SilenceUserResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.silence).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["silence".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnonymizeUserResponse {
    pub success: String,
    pub username: String,
}

impl std::fmt::Display for AnonymizeUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AnonymizeUserResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into(), self.username.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into(), "username".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LogOutUserResponse {
    pub success: String,
}

impl std::fmt::Display for LogOutUserResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LogOutUserResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.success.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RefreshGravatarResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_upload_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gravatar_avatar_template: Option<String>,
}

impl std::fmt::Display for RefreshGravatarResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RefreshGravatarResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.gravatar_upload_id).into(),
            format!("{:?}", self.gravatar_avatar_template).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "gravatar_upload_id".into(),
            "gravatar_avatar_template".into(),
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
pub enum AdminListOrder {
    #[serde(rename = "created")]
    #[display("created")]
    Created,
    #[serde(rename = "last_emailed")]
    #[display("last_emailed")]
    LastEmailed,
    #[serde(rename = "seen")]
    #[display("seen")]
    Seen,
    #[serde(rename = "username")]
    #[display("username")]
    Username,
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "trust_level")]
    #[display("trust_level")]
    TrustLevel,
    #[serde(rename = "days_visited")]
    #[display("days_visited")]
    DaysVisited,
    #[serde(rename = "posts_read")]
    #[display("posts_read")]
    PostsRead,
    #[serde(rename = "topics_viewed")]
    #[display("topics_viewed")]
    TopicsViewed,
    #[serde(rename = "posts")]
    #[display("posts")]
    Posts,
    #[serde(rename = "read_time")]
    #[display("read_time")]
    ReadTime,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AdminListUsersResponse {
    pub id: i64,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_emails: Option<Vec<serde_json::Value>>,
    pub active: bool,
    pub admin: bool,
    pub moderator: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_emailed_at: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_age: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_emailed_age: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at_age: Option<f64>,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_locked_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub time_read: i64,
    pub staged: bool,
    pub days_visited: i64,
    pub posts_read_count: i64,
    pub topics_entered: i64,
    pub post_count: i64,
}

impl std::fmt::Display for AdminListUsersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AdminListUsersResponse {
    const LENGTH: usize = 24;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            format!("{:?}", self.name).into(),
            self.avatar_template.clone().into(),
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(secondary_emails) = &self.secondary_emails {
                format!("{:?}", secondary_emails).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.active).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.last_seen_at).into(),
            format!("{:?}", self.last_emailed_at).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.last_seen_age).into(),
            format!("{:?}", self.last_emailed_age).into(),
            format!("{:?}", self.created_at_age).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.manual_locked_trust_level).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.time_read).into(),
            format!("{:?}", self.staged).into(),
            format!("{:?}", self.days_visited).into(),
            format!("{:?}", self.posts_read_count).into(),
            format!("{:?}", self.topics_entered).into(),
            format!("{:?}", self.post_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "email".into(),
            "secondary_emails".into(),
            "active".into(),
            "admin".into(),
            "moderator".into(),
            "last_seen_at".into(),
            "last_emailed_at".into(),
            "created_at".into(),
            "last_seen_age".into(),
            "last_emailed_age".into(),
            "created_at_age".into(),
            "trust_level".into(),
            "manual_locked_trust_level".into(),
            "title".into(),
            "time_read".into(),
            "staged".into(),
            "days_visited".into(),
            "posts_read_count".into(),
            "topics_entered".into(),
            "post_count".into(),
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
pub enum Flag {
    #[serde(rename = "active")]
    #[display("active")]
    Active,
    #[serde(rename = "new")]
    #[display("new")]
    New,
    #[serde(rename = "staff")]
    #[display("staff")]
    Staff,
    #[serde(rename = "suspended")]
    #[display("suspended")]
    Suspended,
    #[serde(rename = "blocked")]
    #[display("blocked")]
    Blocked,
    #[serde(rename = "suspect")]
    #[display("suspect")]
    Suspect,
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
pub enum AdminListFlagOrder {
    #[serde(rename = "created")]
    #[display("created")]
    Created,
    #[serde(rename = "last_emailed")]
    #[display("last_emailed")]
    LastEmailed,
    #[serde(rename = "seen")]
    #[display("seen")]
    Seen,
    #[serde(rename = "username")]
    #[display("username")]
    Username,
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "trust_level")]
    #[display("trust_level")]
    TrustLevel,
    #[serde(rename = "days_visited")]
    #[display("days_visited")]
    DaysVisited,
    #[serde(rename = "posts_read")]
    #[display("posts_read")]
    PostsRead,
    #[serde(rename = "topics_viewed")]
    #[display("topics_viewed")]
    TopicsViewed,
    #[serde(rename = "posts")]
    #[display("posts")]
    Posts,
    #[serde(rename = "read_time")]
    #[display("read_time")]
    ReadTime,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AdminListUsersFlagResponse {
    pub id: i64,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub avatar_template: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secondary_emails: Option<Vec<serde_json::Value>>,
    pub active: bool,
    pub admin: bool,
    pub moderator: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_emailed_at: Option<String>,
    pub created_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_seen_age: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_emailed_age: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at_age: Option<f64>,
    pub trust_level: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_locked_trust_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub time_read: i64,
    pub staged: bool,
    pub days_visited: i64,
    pub posts_read_count: i64,
    pub topics_entered: i64,
    pub post_count: i64,
}

impl std::fmt::Display for AdminListUsersFlagResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AdminListUsersFlagResponse {
    const LENGTH: usize = 24;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.username.clone().into(),
            format!("{:?}", self.name).into(),
            self.avatar_template.clone().into(),
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(secondary_emails) = &self.secondary_emails {
                format!("{:?}", secondary_emails).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.active).into(),
            format!("{:?}", self.admin).into(),
            format!("{:?}", self.moderator).into(),
            format!("{:?}", self.last_seen_at).into(),
            format!("{:?}", self.last_emailed_at).into(),
            self.created_at.clone().into(),
            format!("{:?}", self.last_seen_age).into(),
            format!("{:?}", self.last_emailed_age).into(),
            format!("{:?}", self.created_at_age).into(),
            format!("{:?}", self.trust_level).into(),
            format!("{:?}", self.manual_locked_trust_level).into(),
            format!("{:?}", self.title).into(),
            format!("{:?}", self.time_read).into(),
            format!("{:?}", self.staged).into(),
            format!("{:?}", self.days_visited).into(),
            format!("{:?}", self.posts_read_count).into(),
            format!("{:?}", self.topics_entered).into(),
            format!("{:?}", self.post_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "username".into(),
            "name".into(),
            "avatar_template".into(),
            "email".into(),
            "secondary_emails".into(),
            "active".into(),
            "admin".into(),
            "moderator".into(),
            "last_seen_at".into(),
            "last_emailed_at".into(),
            "created_at".into(),
            "last_seen_age".into(),
            "last_emailed_age".into(),
            "created_at_age".into(),
            "trust_level".into(),
            "manual_locked_trust_level".into(),
            "title".into(),
            "time_read".into(),
            "staged".into(),
            "days_visited".into(),
            "posts_read_count".into(),
            "topics_entered".into(),
            "post_count".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserActions {
    pub excerpt: String,
    pub action_type: i64,
    pub created_at: String,
    pub avatar_template: String,
    pub acting_avatar_template: String,
    pub slug: String,
    pub topic_id: i64,
    pub target_user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_name: Option<String>,
    pub target_username: String,
    pub post_number: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_id: Option<String>,
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub user_id: i64,
    pub acting_username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acting_name: Option<String>,
    pub acting_user_id: i64,
    pub title: String,
    pub deleted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_code: Option<String>,
    pub category_id: i64,
    pub closed: bool,
    pub archived: bool,
}

impl std::fmt::Display for UserActions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserActions {
    const LENGTH: usize = 26;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.excerpt.clone().into(),
            format!("{:?}", self.action_type).into(),
            self.created_at.clone().into(),
            self.avatar_template.clone().into(),
            self.acting_avatar_template.clone().into(),
            self.slug.clone().into(),
            format!("{:?}", self.topic_id).into(),
            format!("{:?}", self.target_user_id).into(),
            format!("{:?}", self.target_name).into(),
            self.target_username.clone().into(),
            format!("{:?}", self.post_number).into(),
            format!("{:?}", self.post_id).into(),
            self.username.clone().into(),
            format!("{:?}", self.name).into(),
            format!("{:?}", self.user_id).into(),
            self.acting_username.clone().into(),
            format!("{:?}", self.acting_name).into(),
            format!("{:?}", self.acting_user_id).into(),
            self.title.clone().into(),
            format!("{:?}", self.deleted).into(),
            format!("{:?}", self.hidden).into(),
            format!("{:?}", self.post_type).into(),
            format!("{:?}", self.action_code).into(),
            format!("{:?}", self.category_id).into(),
            format!("{:?}", self.closed).into(),
            format!("{:?}", self.archived).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "excerpt".into(),
            "action_type".into(),
            "created_at".into(),
            "avatar_template".into(),
            "acting_avatar_template".into(),
            "slug".into(),
            "topic_id".into(),
            "target_user_id".into(),
            "target_name".into(),
            "target_username".into(),
            "post_number".into(),
            "post_id".into(),
            "username".into(),
            "name".into(),
            "user_id".into(),
            "acting_username".into(),
            "acting_name".into(),
            "acting_user_id".into(),
            "title".into(),
            "deleted".into(),
            "hidden".into(),
            "post_type".into(),
            "action_code".into(),
            "category_id".into(),
            "closed".into(),
            "archived".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUserActionsResponse {
    pub user_actions: Vec<UserActions>,
}

impl std::fmt::Display for ListUserActionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUserActionsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.user_actions).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_actions".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SendPasswordResetEmailRequestBody {
    pub login: String,
}

impl std::fmt::Display for SendPasswordResetEmailRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SendPasswordResetEmailRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.login.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["login".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SendPasswordResetEmailResponse {
    pub success: String,
    pub user_found: bool,
}

impl std::fmt::Display for SendPasswordResetEmailResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SendPasswordResetEmailResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.success.clone().into(),
            format!("{:?}", self.user_found).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["success".into(), "user_found".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ChangePasswordRequestBody {
    pub username: String,
    pub password: String,
}

impl std::fmt::Display for ChangePasswordRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ChangePasswordRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.username.clone().into(), self.password.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["username".into(), "password".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUserEmailsResponse {
    pub email: String,
    pub secondary_emails: Vec<serde_json::Value>,
    pub unconfirmed_emails: Vec<serde_json::Value>,
    pub associated_accounts: Vec<serde_json::Value>,
}

impl std::fmt::Display for GetUserEmailsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUserEmailsResponse {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.email.clone().into(),
            format!("{:?}", self.secondary_emails).into(),
            format!("{:?}", self.unconfirmed_emails).into(),
            format!("{:?}", self.associated_accounts).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "email".into(),
            "secondary_emails".into(),
            "unconfirmed_emails".into(),
            "associated_accounts".into(),
        ]
    }
}
