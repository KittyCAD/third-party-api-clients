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

#[doc = "Represents an Access Group."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AccessGroup {
    #[doc = "The name of this access group."]
    pub name: String,
    #[doc = "Timestamp in milliseconds when the access group was created."]
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[doc = "ID of the team that this access group belongs to."]
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[doc = "Timestamp in milliseconds when the access group was last updated."]
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[doc = "ID of the access group."]
    #[serde(rename = "accessGroupId")]
    pub access_group_id: String,
    #[doc = "Number of members in the access group."]
    #[serde(rename = "membersCount")]
    pub members_count: f64,
    #[doc = "Number of projects in the access group."]
    #[serde(rename = "projectsCount")]
    pub projects_count: f64,
}

impl std::fmt::Display for AccessGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AccessGroup {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            self.created_at.clone().into(),
            self.team_id.clone().into(),
            self.updated_at.clone().into(),
            self.access_group_id.clone().into(),
            format!("{:?}", self.members_count).into(),
            format!("{:?}", self.projects_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "created_at".into(),
            "team_id".into(),
            "updated_at".into(),
            "access_group_id".into(),
            "members_count".into(),
            "projects_count".into(),
        ]
    }
}

#[doc = "Enum containing the actions that can be performed against a resource. Group operations \
         are included."]
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
pub enum Aclaction {
    #[serde(rename = "create")]
    #[display("create")]
    Create,
    #[serde(rename = "delete")]
    #[display("delete")]
    Delete,
    #[serde(rename = "read")]
    #[display("read")]
    Read,
    #[serde(rename = "update")]
    #[display("update")]
    Update,
    #[serde(rename = "list")]
    #[display("list")]
    List,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FlagJSONValue {
    String(String),
    F64(f64),
    Bool(bool),
    FlagJSONValue(Vec<FlagJSONValue>),
    StdCollectionsHashMapStringFlagJSONValue(std::collections::HashMap<String, FlagJSONValue>),
}

#[doc = "This object contains information related to the pagination of the current request, \
         including the necessary parameters to get the next or previous page of data."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Pagination {
    #[doc = "Amount of items in the current page."]
    pub count: f64,
    #[doc = "Timestamp that must be used to request the next page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<f64>,
    #[doc = "Timestamp that must be used to request the previous page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev: Option<f64>,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for Pagination {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.count).into(),
            format!("{:?}", self.next).into(),
            format!("{:?}", self.prev).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["count".into(), "next".into(), "prev".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum EdgeConfigItemValue {
    String(String),
    F64(f64),
    Bool(bool),
    StdCollectionsHashMapStringEdgeConfigItemValue(
        std::collections::HashMap<String, EdgeConfigItemValue>,
    ),
    EdgeConfigItemValue(Vec<EdgeConfigItemValue>),
}

#[doc = "The EdgeConfig."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeConfigItem {
    pub key: String,
    pub value: EdgeConfigItemValue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "edgeConfigId")]
    pub edge_config_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

impl std::fmt::Display for EdgeConfigItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeConfigItem {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.key.clone().into(),
            format!("{:?}", self.value).into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            self.edge_config_id.clone().into(),
            format!("{:?}", self.created_at).into(),
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "key".into(),
            "value".into(),
            "description".into(),
            "edge_config_id".into(),
            "created_at".into(),
            "updated_at".into(),
        ]
    }
}

#[doc = "The EdgeConfig."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeConfigToken {
    pub token: String,
    pub label: String,
    #[doc = "This is not the token itself, but rather an id to identify the token by"]
    pub id: String,
    #[serde(rename = "edgeConfigId")]
    pub edge_config_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
}

impl std::fmt::Display for EdgeConfigToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeConfigToken {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.token.clone().into(),
            self.label.clone().into(),
            self.id.clone().into(),
            self.edge_config_id.clone().into(),
            format!("{:?}", self.created_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "token".into(),
            "label".into(),
            "id".into(),
            "edge_config_id".into(),
            "created_at".into(),
        ]
    }
}

#[doc = "The type of entity."]
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
    #[serde(rename = "author")]
    #[display("author")]
    Author,
    #[serde(rename = "bitbucket_login")]
    #[display("bitbucket_login")]
    BitbucketLogin,
    #[serde(rename = "bold")]
    #[display("bold")]
    Bold,
    #[serde(rename = "deployment_host")]
    #[display("deployment_host")]
    DeploymentHost,
    #[serde(rename = "dns_record")]
    #[display("dns_record")]
    DnsRecord,
    #[serde(rename = "git_link")]
    #[display("git_link")]
    GitLink,
    #[serde(rename = "github_login")]
    #[display("github_login")]
    GithubLogin,
    #[serde(rename = "gitlab_login")]
    #[display("gitlab_login")]
    GitlabLogin,
    #[serde(rename = "hook_name")]
    #[display("hook_name")]
    HookName,
    #[serde(rename = "integration")]
    #[display("integration")]
    Integration,
    #[serde(rename = "edge-config")]
    #[display("edge-config")]
    EdgeConfig,
    #[serde(rename = "link")]
    #[display("link")]
    Link,
    #[serde(rename = "project_name")]
    #[display("project_name")]
    ProjectName,
    #[serde(rename = "scaling_rules")]
    #[display("scaling_rules")]
    ScalingRules,
    #[serde(rename = "env_var_name")]
    #[display("env_var_name")]
    EnvVarName,
    #[serde(rename = "target")]
    #[display("target")]
    Target,
    #[serde(rename = "store")]
    #[display("store")]
    Store,
    #[serde(rename = "system")]
    #[display("system")]
    System,
}

#[doc = "A list of \"entities\" within the event `text`. Useful for enhancing the displayed text \
         with additional styling and links."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Entities {
    #[doc = "The type of entity."]
    #[serde(rename = "type")]
    pub type_: Type,
    #[doc = "The index of where the entity begins within the `text` (inclusive)."]
    pub start: f64,
    #[doc = "The index of where the entity ends within the `text` (non-inclusive)."]
    pub end: f64,
}

impl std::fmt::Display for Entities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Entities {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.type_).into(),
            format!("{:?}", self.start).into(),
            format!("{:?}", self.end).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["type_".into(), "start".into(), "end".into()]
    }
}

#[doc = "Metadata for the User who generated the event."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct User {
    pub avatar: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub uid: String,
    pub username: String,
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
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.avatar.clone().into(),
            self.email.clone().into(),
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
            self.uid.clone().into(),
            self.username.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "avatar".into(),
            "email".into(),
            "slug".into(),
            "uid".into(),
            "username".into(),
        ]
    }
}

#[doc = "Array of events generated by the User."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserEvent {
    #[doc = "The unique identifier of the Event."]
    pub id: String,
    #[doc = "The human-readable text of the Event."]
    pub text: String,
    #[doc = "A list of \"entities\" within the event `text`. Useful for enhancing the displayed \
             text with additional styling and links."]
    pub entities: Vec<Entities>,
    #[doc = "Timestamp (in milliseconds) of when the event was generated."]
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    #[doc = "Metadata for the User who generated the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[doc = "The unique identifier of the User who generated the event."]
    #[serde(rename = "userId")]
    pub user_id: String,
}

impl std::fmt::Display for UserEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserEvent {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.text.clone().into(),
            format!("{:?}", self.entities).into(),
            format!("{:?}", self.created_at).into(),
            if let Some(user) = &self.user {
                format!("{:?}", user).into()
            } else {
                String::new().into()
            },
            self.user_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "text".into(),
            "entities".into(),
            "created_at".into(),
            "user".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "Data representing a Team."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Team {}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Team {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[doc = "Information for the SAML Single Sign-On configuration."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Connection {
    #[doc = "The Identity Provider \"type\", for example Okta."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Current status of the connection."]
    pub status: String,
    #[doc = "Current state of the connection."]
    pub state: String,
    #[doc = "Timestamp (in milliseconds) of when the configuration was connected."]
    #[serde(rename = "connectedAt")]
    pub connected_at: f64,
    #[doc = "Timestamp (in milliseconds) of when the last webhook event was received from WorkOS."]
    #[serde(
        rename = "lastReceivedWebhookEvent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_received_webhook_event: Option<f64>,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Connection {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.type_.clone().into(),
            self.status.clone().into(),
            self.state.clone().into(),
            format!("{:?}", self.connected_at).into(),
            if let Some(last_received_webhook_event) = &self.last_received_webhook_event {
                format!("{:?}", last_received_webhook_event).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "status".into(),
            "state".into(),
            "connected_at".into(),
            "last_received_webhook_event".into(),
        ]
    }
}

#[doc = "Information for the Directory Sync configuration."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Directory {
    #[doc = "The Identity Provider \"type\", for example Okta."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Current state of the connection."]
    pub state: String,
    #[doc = "Timestamp (in milliseconds) of when the configuration was connected."]
    #[serde(rename = "connectedAt")]
    pub connected_at: f64,
    #[doc = "Timestamp (in milliseconds) of when the last webhook event was received from WorkOS."]
    #[serde(
        rename = "lastReceivedWebhookEvent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub last_received_webhook_event: Option<f64>,
}

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Directory {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.type_.clone().into(),
            self.state.clone().into(),
            format!("{:?}", self.connected_at).into(),
            if let Some(last_received_webhook_event) = &self.last_received_webhook_event {
                format!("{:?}", last_received_webhook_event).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "state".into(),
            "connected_at".into(),
            "last_received_webhook_event".into(),
        ]
    }
}

#[doc = "When \"Single Sign-On (SAML)\" is configured, this object contains information that \
         allows the client-side to identify whether or not this Team has SAML enforced."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Saml {
    #[doc = "Information for the SAML Single Sign-On configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
    #[doc = "Information for the Directory Sync configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub directory: Option<Directory>,
    #[doc = "When `true`, interactions with the Team **must** be done with an authentication \
             token that has been authenticated with the Team's SAML Single Sign-On provider."]
    pub enforced: bool,
}

impl std::fmt::Display for Saml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Saml {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(connection) = &self.connection {
                format!("{:?}", connection).into()
            } else {
                String::new().into()
            },
            if let Some(directory) = &self.directory {
                format!("{:?}", directory).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.enforced).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["connection".into(), "directory".into(), "enforced".into()]
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
pub enum Role {
    #[serde(rename = "OWNER")]
    #[display("OWNER")]
    Owner,
    #[serde(rename = "MEMBER")]
    #[display("MEMBER")]
    Member,
    #[serde(rename = "DEVELOPER")]
    #[display("DEVELOPER")]
    Developer,
    #[serde(rename = "BILLING")]
    #[display("BILLING")]
    Billing,
    #[serde(rename = "VIEWER")]
    #[display("VIEWER")]
    Viewer,
    #[serde(rename = "CONTRIBUTOR")]
    #[display("CONTRIBUTOR")]
    Contributor,
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
pub enum Origin {
    #[serde(rename = "link")]
    #[display("link")]
    Link,
    #[serde(rename = "saml")]
    #[display("saml")]
    Saml,
    #[serde(rename = "mail")]
    #[display("mail")]
    Mail,
    #[serde(rename = "import")]
    #[display("import")]
    Import,
    #[serde(rename = "teams")]
    #[display("teams")]
    Teams,
    #[serde(rename = "github")]
    #[display("github")]
    Github,
    #[serde(rename = "gitlab")]
    #[display("gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    #[display("bitbucket")]
    Bitbucket,
    #[serde(rename = "dsync")]
    #[display("dsync")]
    Dsync,
    #[serde(rename = "feedback")]
    #[display("feedback")]
    Feedback,
    #[serde(rename = "organization-teams")]
    #[display("organization-teams")]
    OrganizationTeams,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum GitUserId {
    String(String),
    F64(f64),
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JoinedFrom {
    pub origin: Origin,
    #[serde(rename = "commitId", default, skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(rename = "repoId", default, skip_serializing_if = "Option::is_none")]
    pub repo_id: Option<String>,
    #[serde(rename = "repoPath", default, skip_serializing_if = "Option::is_none")]
    pub repo_path: Option<String>,
    #[serde(rename = "gitUserId", default, skip_serializing_if = "Option::is_none")]
    pub git_user_id: Option<GitUserId>,
    #[serde(
        rename = "gitUserLogin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub git_user_login: Option<String>,
    #[serde(rename = "ssoUserId", default, skip_serializing_if = "Option::is_none")]
    pub sso_user_id: Option<String>,
    #[serde(
        rename = "ssoConnectedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sso_connected_at: Option<f64>,
    #[serde(rename = "idpUserId", default, skip_serializing_if = "Option::is_none")]
    pub idp_user_id: Option<String>,
    #[serde(
        rename = "dsyncUserId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dsync_user_id: Option<String>,
    #[serde(
        rename = "dsyncConnectedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dsync_connected_at: Option<f64>,
}

impl std::fmt::Display for JoinedFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JoinedFrom {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.origin).into(),
            if let Some(commit_id) = &self.commit_id {
                format!("{:?}", commit_id).into()
            } else {
                String::new().into()
            },
            if let Some(repo_id) = &self.repo_id {
                format!("{:?}", repo_id).into()
            } else {
                String::new().into()
            },
            if let Some(repo_path) = &self.repo_path {
                format!("{:?}", repo_path).into()
            } else {
                String::new().into()
            },
            if let Some(git_user_id) = &self.git_user_id {
                format!("{:?}", git_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(git_user_login) = &self.git_user_login {
                format!("{:?}", git_user_login).into()
            } else {
                String::new().into()
            },
            if let Some(sso_user_id) = &self.sso_user_id {
                format!("{:?}", sso_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(sso_connected_at) = &self.sso_connected_at {
                format!("{:?}", sso_connected_at).into()
            } else {
                String::new().into()
            },
            if let Some(idp_user_id) = &self.idp_user_id {
                format!("{:?}", idp_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(dsync_user_id) = &self.dsync_user_id {
                format!("{:?}", dsync_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(dsync_connected_at) = &self.dsync_connected_at {
                format!("{:?}", dsync_connected_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "origin".into(),
            "commit_id".into(),
            "repo_id".into(),
            "repo_path".into(),
            "git_user_id".into(),
            "git_user_login".into(),
            "sso_user_id".into(),
            "sso_connected_at".into(),
            "idp_user_id".into(),
            "dsync_user_id".into(),
            "dsync_connected_at".into(),
        ]
    }
}

#[doc = "The membership of the authenticated User in relation to the Team."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Membership {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed: Option<bool>,
    #[serde(
        rename = "confirmedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confirmed_at: Option<f64>,
    #[serde(
        rename = "accessRequestedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_requested_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(
        rename = "joinedFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub joined_from: Option<JoinedFrom>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl std::fmt::Display for Membership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Membership {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(confirmed) = &self.confirmed {
                format!("{:?}", confirmed).into()
            } else {
                String::new().into()
            },
            if let Some(confirmed_at) = &self.confirmed_at {
                format!("{:?}", confirmed_at).into()
            } else {
                String::new().into()
            },
            if let Some(access_requested_at) = &self.access_requested_at {
                format!("{:?}", access_requested_at).into()
            } else {
                String::new().into()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(team_id) = &self.team_id {
                format!("{:?}", team_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created) = &self.created {
                format!("{:?}", created).into()
            } else {
                String::new().into()
            },
            if let Some(joined_from) = &self.joined_from {
                format!("{:?}", joined_from).into()
            } else {
                String::new().into()
            },
            if let Some(uid) = &self.uid {
                format!("{:?}", uid).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "confirmed".into(),
            "confirmed_at".into(),
            "access_requested_at".into(),
            "role".into(),
            "team_id".into(),
            "created_at".into(),
            "created".into(),
            "joined_from".into(),
            "uid".into(),
        ]
    }
}

#[doc = "A limited form of data representing a Team, due to the authentication token missing \
         privileges to read the full Team data."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeamLimited {
    #[doc = "Property indicating that this Team data contains only limited information, due to \
             the authentication token missing privileges to read the full Team data. Re-login \
             with the Team's configured SAML Single Sign-On provider in order to upgrade the \
             authentication token with the necessary privileges."]
    pub limited: bool,
    #[doc = "When \"Single Sign-On (SAML)\" is configured, this object contains information that \
             allows the client-side to identify whether or not this Team has SAML enforced."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saml: Option<Saml>,
    #[doc = "The Team's unique identifier."]
    pub id: String,
    #[doc = "The Team's slug, which is unique across the Vercel platform."]
    pub slug: String,
    #[doc = "Name associated with the Team account, or `null` if none has been provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the file used as avatar for this Team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[doc = "The membership of the authenticated User in relation to the Team."]
    pub membership: Membership,
    #[doc = "Will remain undocumented. Remove in v3 API."]
    pub created: String,
    #[doc = "UNIX timestamp (in milliseconds) when the Team was created."]
    #[serde(rename = "createdAt")]
    pub created_at: f64,
}

impl std::fmt::Display for TeamLimited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TeamLimited {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.limited).into(),
            if let Some(saml) = &self.saml {
                format!("{:?}", saml).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.slug.clone().into(),
            format!("{:?}", self.name).into(),
            format!("{:?}", self.avatar).into(),
            format!("{:?}", self.membership).into(),
            self.created.clone().into(),
            format!("{:?}", self.created_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "limited".into(),
            "saml".into(),
            "id".into(),
            "slug".into(),
            "name".into(),
            "avatar".into(),
            "membership".into(),
            "created".into(),
            "created_at".into(),
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
    #[serde(rename = "SUBSCRIPTION_CANCELED")]
    #[display("SUBSCRIPTION_CANCELED")]
    SubscriptionCanceled,
    #[serde(rename = "SUBSCRIPTION_EXPIRED")]
    #[display("SUBSCRIPTION_EXPIRED")]
    SubscriptionExpired,
    #[serde(rename = "UNPAID_INVOICE")]
    #[display("UNPAID_INVOICE")]
    UnpaidInvoice,
    #[serde(rename = "ENTERPRISE_TRIAL_ENDED")]
    #[display("ENTERPRISE_TRIAL_ENDED")]
    EnterpriseTrialEnded,
    #[serde(rename = "FAIR_USE_LIMITS_EXCEEDED")]
    #[display("FAIR_USE_LIMITS_EXCEEDED")]
    FairUseLimitsExceeded,
    #[serde(rename = "BLOCKED_FOR_PLATFORM_ABUSE")]
    #[display("BLOCKED_FOR_PLATFORM_ABUSE")]
    BlockedForPlatformAbuse,
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
pub enum BlockedDueToOverageType {
    #[serde(rename = "analyticsUsage")]
    #[display("analyticsUsage")]
    AnalyticsUsage,
    #[serde(rename = "artifacts")]
    #[display("artifacts")]
    Artifacts,
    #[serde(rename = "bandwidth")]
    #[display("bandwidth")]
    Bandwidth,
    #[serde(rename = "blobStores")]
    #[display("blobStores")]
    BlobStores,
    #[serde(rename = "blobTotalAdvancedRequests")]
    #[display("blobTotalAdvancedRequests")]
    BlobTotalAdvancedRequests,
    #[serde(rename = "blobTotalAvgSizeInBytes")]
    #[display("blobTotalAvgSizeInBytes")]
    BlobTotalAvgSizeInBytes,
    #[serde(rename = "blobTotalGetResponseObjectSizeInBytes")]
    #[display("blobTotalGetResponseObjectSizeInBytes")]
    BlobTotalGetResponseObjectSizeInBytes,
    #[serde(rename = "blobTotalSimpleRequests")]
    #[display("blobTotalSimpleRequests")]
    BlobTotalSimpleRequests,
    #[serde(rename = "buildMinute")]
    #[display("buildMinute")]
    BuildMinute,
    #[serde(rename = "dataCacheRead")]
    #[display("dataCacheRead")]
    DataCacheRead,
    #[serde(rename = "dataCacheRevalidation")]
    #[display("dataCacheRevalidation")]
    DataCacheRevalidation,
    #[serde(rename = "dataCacheWrite")]
    #[display("dataCacheWrite")]
    DataCacheWrite,
    #[serde(rename = "edgeConfigRead")]
    #[display("edgeConfigRead")]
    EdgeConfigRead,
    #[serde(rename = "edgeConfigWrite")]
    #[display("edgeConfigWrite")]
    EdgeConfigWrite,
    #[serde(rename = "edgeFunctionExecutionUnits")]
    #[display("edgeFunctionExecutionUnits")]
    EdgeFunctionExecutionUnits,
    #[serde(rename = "edgeMiddlewareInvocations")]
    #[display("edgeMiddlewareInvocations")]
    EdgeMiddlewareInvocations,
    #[serde(rename = "edgeRequest")]
    #[display("edgeRequest")]
    EdgeRequest,
    #[serde(rename = "edgeRequestAdditionalCpuDuration")]
    #[display("edgeRequestAdditionalCpuDuration")]
    EdgeRequestAdditionalCpuDuration,
    #[serde(rename = "fastDataTransfer")]
    #[display("fastDataTransfer")]
    FastDataTransfer,
    #[serde(rename = "fastOriginTransfer")]
    #[display("fastOriginTransfer")]
    FastOriginTransfer,
    #[serde(rename = "functionDuration")]
    #[display("functionDuration")]
    FunctionDuration,
    #[serde(rename = "functionInvocation")]
    #[display("functionInvocation")]
    FunctionInvocation,
    #[serde(rename = "logDrainsVolume")]
    #[display("logDrainsVolume")]
    LogDrainsVolume,
    #[serde(rename = "monitoringMetric")]
    #[display("monitoringMetric")]
    MonitoringMetric,
    #[serde(rename = "postgresComputeTime")]
    #[display("postgresComputeTime")]
    PostgresComputeTime,
    #[serde(rename = "postgresDataStorage")]
    #[display("postgresDataStorage")]
    PostgresDataStorage,
    #[serde(rename = "postgresDataTransfer")]
    #[display("postgresDataTransfer")]
    PostgresDataTransfer,
    #[serde(rename = "postgresDatabase")]
    #[display("postgresDatabase")]
    PostgresDatabase,
    #[serde(rename = "postgresWrittenData")]
    #[display("postgresWrittenData")]
    PostgresWrittenData,
    #[serde(rename = "serverlessFunctionExecution")]
    #[display("serverlessFunctionExecution")]
    ServerlessFunctionExecution,
    #[serde(rename = "sourceImages")]
    #[display("sourceImages")]
    SourceImages,
    #[serde(rename = "storageRedisTotalBandwidthInBytes")]
    #[display("storageRedisTotalBandwidthInBytes")]
    StorageRedisTotalBandwidthInBytes,
    #[serde(rename = "storageRedisTotalCommands")]
    #[display("storageRedisTotalCommands")]
    StorageRedisTotalCommands,
    #[serde(rename = "storageRedisTotalDailyAvgStorageInBytes")]
    #[display("storageRedisTotalDailyAvgStorageInBytes")]
    StorageRedisTotalDailyAvgStorageInBytes,
    #[serde(rename = "storageRedisTotalDatabases")]
    #[display("storageRedisTotalDatabases")]
    StorageRedisTotalDatabases,
    #[serde(rename = "wafOwaspExcessBytes")]
    #[display("wafOwaspExcessBytes")]
    WafOwaspExcessBytes,
    #[serde(rename = "wafOwaspRequests")]
    #[display("wafOwaspRequests")]
    WafOwaspRequests,
    #[serde(rename = "webAnalyticsEvent")]
    #[display("webAnalyticsEvent")]
    WebAnalyticsEvent,
}

#[doc = "When the User account has been \"soft blocked\", this property will contain the date when \
         the restriction was enacted, and the identifier for why."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SoftBlock {
    #[serde(rename = "blockedAt")]
    pub blocked_at: f64,
    pub reason: Reason,
    #[serde(
        rename = "blockedDueToOverageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_due_to_overage_type: Option<BlockedDueToOverageType>,
}

impl std::fmt::Display for SoftBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SoftBlock {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.blocked_at).into(),
            format!("{:?}", self.reason).into(),
            if let Some(blocked_due_to_overage_type) = &self.blocked_due_to_overage_type {
                format!("{:?}", blocked_due_to_overage_type).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "blocked_at".into(),
            "reason".into(),
            "blocked_due_to_overage_type".into(),
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
pub enum Currency {
    #[serde(rename = "usd")]
    #[display("usd")]
    Usd,
    #[serde(rename = "eur")]
    #[display("eur")]
    Eur,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Period {
    pub start: f64,
    pub end: f64,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Period {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.start).into(),
            format!("{:?}", self.end).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["start".into(), "end".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Contract {
    pub start: f64,
    pub end: f64,
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Contract {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.start).into(),
            format!("{:?}", self.end).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["start".into(), "end".into()]
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
pub enum Plan {
    #[serde(rename = "pro")]
    #[display("pro")]
    Pro,
    #[serde(rename = "enterprise")]
    #[display("enterprise")]
    Enterprise,
    #[serde(rename = "hobby")]
    #[display("hobby")]
    Hobby,
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
pub enum Platform {
    #[serde(rename = "stripe")]
    #[display("stripe")]
    Stripe,
    #[serde(rename = "stripeTestMode")]
    #[display("stripeTestMode")]
    StripeTestMode,
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
pub enum ProgramType {
    #[serde(rename = "startup")]
    #[display("startup")]
    Startup,
    #[serde(rename = "agency")]
    #[display("agency")]
    Agency,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Trial {
    pub start: f64,
    pub end: f64,
}

impl std::fmt::Display for Trial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Trial {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.start).into(),
            format!("{:?}", self.end).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["start".into(), "end".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Tax {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
}

impl std::fmt::Display for Tax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Tax {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.type_.clone().into(), self.id.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["type_".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Address {
    #[serde(rename = "line1", default, skip_serializing_if = "Option::is_none")]
    pub line_1: Option<String>,
    #[serde(rename = "line2", default, skip_serializing_if = "Option::is_none")]
    pub line_2: Option<String>,
    #[serde(
        rename = "postalCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Address {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(line_1) = &self.line_1 {
                format!("{:?}", line_1).into()
            } else {
                String::new().into()
            },
            if let Some(line_2) = &self.line_2 {
                format!("{:?}", line_2).into()
            } else {
                String::new().into()
            },
            if let Some(postal_code) = &self.postal_code {
                format!("{:?}", postal_code).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "line_1".into(),
            "line_2".into(),
            "postal_code".into(),
            "city".into(),
            "country".into(),
            "state".into(),
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
pub enum Interval {
    #[serde(rename = "month")]
    #[display("month")]
    Month,
}

impl std::default::Default for Interval {
    fn default() -> Self {
        Interval::Month
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Frequency {
    pub interval: Interval,
    #[serde(rename = "intervalCount")]
    pub interval_count: f64,
}

impl std::fmt::Display for Frequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Frequency {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.interval).into(),
            format!("{:?}", self.interval_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["interval".into(), "interval_count".into()]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ConcurrentBuilds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for ConcurrentBuilds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ConcurrentBuilds {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WebAnalytics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for WebAnalytics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WebAnalytics {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Pro {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for Pro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Pro {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Enterprise {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for Enterprise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Enterprise {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Analytics {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for Analytics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Analytics {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DeveloperExperiencePlatform {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for DeveloperExperiencePlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DeveloperExperiencePlatform {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct IncludedAllocationMiu {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for IncludedAllocationMiu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for IncludedAllocationMiu {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ManagedInfrastructureCommitment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for ManagedInfrastructureCommitment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ManagedInfrastructureCommitment {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Monitoring {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for Monitoring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Monitoring {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PasswordProtection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for PasswordProtection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PasswordProtection {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PreviewDeploymentSuffix {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for PreviewDeploymentSuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PreviewDeploymentSuffix {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InvoiceItemsSaml {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for InvoiceItemsSaml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InvoiceItemsSaml {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TeamSeats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for TeamSeats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TeamSeats {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct VercelMarketplace {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
    #[doc = "The highest quantity in the current period. Used to render the correct \
             enable/disable UI for add-ons."]
    #[serde(
        rename = "highestQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub highest_quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(
        rename = "maxQuantity",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_quantity: Option<f64>,
}

impl std::fmt::Display for VercelMarketplace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for VercelMarketplace {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.quantity).into(),
            if let Some(highest_quantity) = &self.highest_quantity {
                format!("{:?}", highest_quantity).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tier".into(),
            "price".into(),
            "quantity".into(),
            "highest_quantity".into(),
            "name".into(),
            "hidden".into(),
            "created_at".into(),
            "disabled_at".into(),
            "frequency".into(),
            "max_quantity".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Matrix {
    #[serde(rename = "defaultUnitPrice")]
    pub default_unit_price: String,
    #[serde(rename = "dimensionPrices")]
    pub dimension_prices: std::collections::HashMap<String, String>,
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Matrix {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.default_unit_price.clone().into(),
            format!("{:?}", self.dimension_prices).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["default_unit_price".into(), "dimension_prices".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for AnalyticsUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AnalyticsUsage {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Artifacts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for Artifacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Artifacts {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Bandwidth {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for Bandwidth {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Bandwidth {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BlobStores {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for BlobStores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BlobStores {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BlobTotalAdvancedRequests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for BlobTotalAdvancedRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BlobTotalAdvancedRequests {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BlobTotalAvgSizeInBytes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for BlobTotalAvgSizeInBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BlobTotalAvgSizeInBytes {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BlobTotalGetResponseObjectSizeInBytes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for BlobTotalGetResponseObjectSizeInBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BlobTotalGetResponseObjectSizeInBytes {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BlobTotalSimpleRequests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for BlobTotalSimpleRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BlobTotalSimpleRequests {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BuildMinute {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for BuildMinute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BuildMinute {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DataCacheRead {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for DataCacheRead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DataCacheRead {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DataCacheRevalidation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for DataCacheRevalidation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DataCacheRevalidation {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DataCacheWrite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for DataCacheWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DataCacheWrite {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeConfigRead {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for EdgeConfigRead {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeConfigRead {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeConfigWrite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for EdgeConfigWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeConfigWrite {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeFunctionExecutionUnits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for EdgeFunctionExecutionUnits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeFunctionExecutionUnits {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeMiddlewareInvocations {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for EdgeMiddlewareInvocations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeMiddlewareInvocations {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for EdgeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeRequest {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeRequestAdditionalCpuDuration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for EdgeRequestAdditionalCpuDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EdgeRequestAdditionalCpuDuration {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FastDataTransfer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for FastDataTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FastDataTransfer {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FastOriginTransfer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for FastOriginTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FastOriginTransfer {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FunctionDuration {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for FunctionDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FunctionDuration {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FunctionInvocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for FunctionInvocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FunctionInvocation {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LogDrainsVolume {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for LogDrainsVolume {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LogDrainsVolume {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MonitoringMetric {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for MonitoringMetric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MonitoringMetric {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresComputeTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for PostgresComputeTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostgresComputeTime {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresDataStorage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for PostgresDataStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostgresDataStorage {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresDataTransfer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for PostgresDataTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostgresDataTransfer {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresDatabase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for PostgresDatabase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostgresDatabase {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresWrittenData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for PostgresWrittenData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostgresWrittenData {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ServerlessFunctionExecution {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for ServerlessFunctionExecution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ServerlessFunctionExecution {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SourceImages {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for SourceImages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SourceImages {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalBandwidthInBytes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for StorageRedisTotalBandwidthInBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StorageRedisTotalBandwidthInBytes {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalCommands {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for StorageRedisTotalCommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StorageRedisTotalCommands {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalDailyAvgStorageInBytes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for StorageRedisTotalDailyAvgStorageInBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StorageRedisTotalDailyAvgStorageInBytes {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalDatabases {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for StorageRedisTotalDatabases {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for StorageRedisTotalDatabases {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WafOwaspExcessBytes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for WafOwaspExcessBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WafOwaspExcessBytes {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WafOwaspRequests {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for WafOwaspRequests {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WafOwaspRequests {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WebAnalyticsEvent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matrix: Option<Matrix>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub batch: f64,
    pub threshold: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub hidden: bool,
    #[serde(
        rename = "disabledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disabled_at: Option<f64>,
    #[serde(rename = "enabledAt", default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
}

impl std::fmt::Display for WebAnalyticsEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WebAnalyticsEvent {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(matrix) = &self.matrix {
                format!("{:?}", matrix).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.price).into(),
            format!("{:?}", self.batch).into(),
            format!("{:?}", self.threshold).into(),
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.hidden).into(),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at).into()
            } else {
                String::new().into()
            },
            if let Some(enabled_at) = &self.enabled_at {
                format!("{:?}", enabled_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "matrix".into(),
            "tier".into(),
            "price".into(),
            "batch".into(),
            "threshold".into(),
            "name".into(),
            "hidden".into(),
            "disabled_at".into(),
            "enabled_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InvoiceItems {
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "concurrentBuilds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub concurrent_builds: Option<ConcurrentBuilds>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "webAnalytics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_analytics: Option<WebAnalytics>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pro: Option<Pro>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Enterprise>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics: Option<Analytics>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "developerExperiencePlatform",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub developer_experience_platform: Option<DeveloperExperiencePlatform>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "includedAllocationMiu",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub included_allocation_miu: Option<IncludedAllocationMiu>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "managedInfrastructureCommitment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_infrastructure_commitment: Option<ManagedInfrastructureCommitment>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<Monitoring>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "passwordProtection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub password_protection: Option<PasswordProtection>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "previewDeploymentSuffix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_deployment_suffix: Option<PreviewDeploymentSuffix>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saml: Option<InvoiceItemsSaml>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(rename = "teamSeats", default, skip_serializing_if = "Option::is_none")]
    pub team_seats: Option<TeamSeats>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "vercelMarketplace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub vercel_marketplace: Option<VercelMarketplace>,
    #[serde(
        rename = "analyticsUsage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_usage: Option<AnalyticsUsage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Artifacts>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<Bandwidth>,
    #[serde(
        rename = "blobStores",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_stores: Option<BlobStores>,
    #[serde(
        rename = "blobTotalAdvancedRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_total_advanced_requests: Option<BlobTotalAdvancedRequests>,
    #[serde(
        rename = "blobTotalAvgSizeInBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_total_avg_size_in_bytes: Option<BlobTotalAvgSizeInBytes>,
    #[serde(
        rename = "blobTotalGetResponseObjectSizeInBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_total_get_response_object_size_in_bytes: Option<BlobTotalGetResponseObjectSizeInBytes>,
    #[serde(
        rename = "blobTotalSimpleRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_total_simple_requests: Option<BlobTotalSimpleRequests>,
    #[serde(
        rename = "buildMinute",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub build_minute: Option<BuildMinute>,
    #[serde(
        rename = "dataCacheRead",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_cache_read: Option<DataCacheRead>,
    #[serde(
        rename = "dataCacheRevalidation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_cache_revalidation: Option<DataCacheRevalidation>,
    #[serde(
        rename = "dataCacheWrite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_cache_write: Option<DataCacheWrite>,
    #[serde(
        rename = "edgeConfigRead",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_config_read: Option<EdgeConfigRead>,
    #[serde(
        rename = "edgeConfigWrite",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_config_write: Option<EdgeConfigWrite>,
    #[serde(
        rename = "edgeFunctionExecutionUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_function_execution_units: Option<EdgeFunctionExecutionUnits>,
    #[serde(
        rename = "edgeMiddlewareInvocations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_middleware_invocations: Option<EdgeMiddlewareInvocations>,
    #[serde(
        rename = "edgeRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_request: Option<EdgeRequest>,
    #[serde(
        rename = "edgeRequestAdditionalCpuDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_request_additional_cpu_duration: Option<EdgeRequestAdditionalCpuDuration>,
    #[serde(
        rename = "fastDataTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fast_data_transfer: Option<FastDataTransfer>,
    #[serde(
        rename = "fastOriginTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub fast_origin_transfer: Option<FastOriginTransfer>,
    #[serde(
        rename = "functionDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_duration: Option<FunctionDuration>,
    #[serde(
        rename = "functionInvocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub function_invocation: Option<FunctionInvocation>,
    #[serde(
        rename = "logDrainsVolume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_drains_volume: Option<LogDrainsVolume>,
    #[serde(
        rename = "monitoringMetric",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monitoring_metric: Option<MonitoringMetric>,
    #[serde(
        rename = "postgresComputeTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_compute_time: Option<PostgresComputeTime>,
    #[serde(
        rename = "postgresDataStorage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_data_storage: Option<PostgresDataStorage>,
    #[serde(
        rename = "postgresDataTransfer",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_data_transfer: Option<PostgresDataTransfer>,
    #[serde(
        rename = "postgresDatabase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_database: Option<PostgresDatabase>,
    #[serde(
        rename = "postgresWrittenData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_written_data: Option<PostgresWrittenData>,
    #[serde(
        rename = "serverlessFunctionExecution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub serverless_function_execution: Option<ServerlessFunctionExecution>,
    #[serde(
        rename = "sourceImages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub source_images: Option<SourceImages>,
    #[serde(
        rename = "storageRedisTotalBandwidthInBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_redis_total_bandwidth_in_bytes: Option<StorageRedisTotalBandwidthInBytes>,
    #[serde(
        rename = "storageRedisTotalCommands",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_redis_total_commands: Option<StorageRedisTotalCommands>,
    #[serde(
        rename = "storageRedisTotalDailyAvgStorageInBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_redis_total_daily_avg_storage_in_bytes:
        Option<StorageRedisTotalDailyAvgStorageInBytes>,
    #[serde(
        rename = "storageRedisTotalDatabases",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_redis_total_databases: Option<StorageRedisTotalDatabases>,
    #[serde(
        rename = "wafOwaspExcessBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub waf_owasp_excess_bytes: Option<WafOwaspExcessBytes>,
    #[serde(
        rename = "wafOwaspRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub waf_owasp_requests: Option<WafOwaspRequests>,
    #[serde(
        rename = "webAnalyticsEvent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_analytics_event: Option<WebAnalyticsEvent>,
}

impl std::fmt::Display for InvoiceItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InvoiceItems {
    const LENGTH: usize = 52;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(concurrent_builds) = &self.concurrent_builds {
                format!("{:?}", concurrent_builds).into()
            } else {
                String::new().into()
            },
            if let Some(web_analytics) = &self.web_analytics {
                format!("{:?}", web_analytics).into()
            } else {
                String::new().into()
            },
            if let Some(pro) = &self.pro {
                format!("{:?}", pro).into()
            } else {
                String::new().into()
            },
            if let Some(enterprise) = &self.enterprise {
                format!("{:?}", enterprise).into()
            } else {
                String::new().into()
            },
            if let Some(analytics) = &self.analytics {
                format!("{:?}", analytics).into()
            } else {
                String::new().into()
            },
            if let Some(developer_experience_platform) = &self.developer_experience_platform {
                format!("{:?}", developer_experience_platform).into()
            } else {
                String::new().into()
            },
            if let Some(included_allocation_miu) = &self.included_allocation_miu {
                format!("{:?}", included_allocation_miu).into()
            } else {
                String::new().into()
            },
            if let Some(managed_infrastructure_commitment) = &self.managed_infrastructure_commitment
            {
                format!("{:?}", managed_infrastructure_commitment).into()
            } else {
                String::new().into()
            },
            if let Some(monitoring) = &self.monitoring {
                format!("{:?}", monitoring).into()
            } else {
                String::new().into()
            },
            if let Some(password_protection) = &self.password_protection {
                format!("{:?}", password_protection).into()
            } else {
                String::new().into()
            },
            if let Some(preview_deployment_suffix) = &self.preview_deployment_suffix {
                format!("{:?}", preview_deployment_suffix).into()
            } else {
                String::new().into()
            },
            if let Some(saml) = &self.saml {
                format!("{:?}", saml).into()
            } else {
                String::new().into()
            },
            if let Some(team_seats) = &self.team_seats {
                format!("{:?}", team_seats).into()
            } else {
                String::new().into()
            },
            if let Some(vercel_marketplace) = &self.vercel_marketplace {
                format!("{:?}", vercel_marketplace).into()
            } else {
                String::new().into()
            },
            if let Some(analytics_usage) = &self.analytics_usage {
                format!("{:?}", analytics_usage).into()
            } else {
                String::new().into()
            },
            if let Some(artifacts) = &self.artifacts {
                format!("{:?}", artifacts).into()
            } else {
                String::new().into()
            },
            if let Some(bandwidth) = &self.bandwidth {
                format!("{:?}", bandwidth).into()
            } else {
                String::new().into()
            },
            if let Some(blob_stores) = &self.blob_stores {
                format!("{:?}", blob_stores).into()
            } else {
                String::new().into()
            },
            if let Some(blob_total_advanced_requests) = &self.blob_total_advanced_requests {
                format!("{:?}", blob_total_advanced_requests).into()
            } else {
                String::new().into()
            },
            if let Some(blob_total_avg_size_in_bytes) = &self.blob_total_avg_size_in_bytes {
                format!("{:?}", blob_total_avg_size_in_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(blob_total_get_response_object_size_in_bytes) =
                &self.blob_total_get_response_object_size_in_bytes
            {
                format!("{:?}", blob_total_get_response_object_size_in_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(blob_total_simple_requests) = &self.blob_total_simple_requests {
                format!("{:?}", blob_total_simple_requests).into()
            } else {
                String::new().into()
            },
            if let Some(build_minute) = &self.build_minute {
                format!("{:?}", build_minute).into()
            } else {
                String::new().into()
            },
            if let Some(data_cache_read) = &self.data_cache_read {
                format!("{:?}", data_cache_read).into()
            } else {
                String::new().into()
            },
            if let Some(data_cache_revalidation) = &self.data_cache_revalidation {
                format!("{:?}", data_cache_revalidation).into()
            } else {
                String::new().into()
            },
            if let Some(data_cache_write) = &self.data_cache_write {
                format!("{:?}", data_cache_write).into()
            } else {
                String::new().into()
            },
            if let Some(edge_config_read) = &self.edge_config_read {
                format!("{:?}", edge_config_read).into()
            } else {
                String::new().into()
            },
            if let Some(edge_config_write) = &self.edge_config_write {
                format!("{:?}", edge_config_write).into()
            } else {
                String::new().into()
            },
            if let Some(edge_function_execution_units) = &self.edge_function_execution_units {
                format!("{:?}", edge_function_execution_units).into()
            } else {
                String::new().into()
            },
            if let Some(edge_middleware_invocations) = &self.edge_middleware_invocations {
                format!("{:?}", edge_middleware_invocations).into()
            } else {
                String::new().into()
            },
            if let Some(edge_request) = &self.edge_request {
                format!("{:?}", edge_request).into()
            } else {
                String::new().into()
            },
            if let Some(edge_request_additional_cpu_duration) =
                &self.edge_request_additional_cpu_duration
            {
                format!("{:?}", edge_request_additional_cpu_duration).into()
            } else {
                String::new().into()
            },
            if let Some(fast_data_transfer) = &self.fast_data_transfer {
                format!("{:?}", fast_data_transfer).into()
            } else {
                String::new().into()
            },
            if let Some(fast_origin_transfer) = &self.fast_origin_transfer {
                format!("{:?}", fast_origin_transfer).into()
            } else {
                String::new().into()
            },
            if let Some(function_duration) = &self.function_duration {
                format!("{:?}", function_duration).into()
            } else {
                String::new().into()
            },
            if let Some(function_invocation) = &self.function_invocation {
                format!("{:?}", function_invocation).into()
            } else {
                String::new().into()
            },
            if let Some(log_drains_volume) = &self.log_drains_volume {
                format!("{:?}", log_drains_volume).into()
            } else {
                String::new().into()
            },
            if let Some(monitoring_metric) = &self.monitoring_metric {
                format!("{:?}", monitoring_metric).into()
            } else {
                String::new().into()
            },
            if let Some(postgres_compute_time) = &self.postgres_compute_time {
                format!("{:?}", postgres_compute_time).into()
            } else {
                String::new().into()
            },
            if let Some(postgres_data_storage) = &self.postgres_data_storage {
                format!("{:?}", postgres_data_storage).into()
            } else {
                String::new().into()
            },
            if let Some(postgres_data_transfer) = &self.postgres_data_transfer {
                format!("{:?}", postgres_data_transfer).into()
            } else {
                String::new().into()
            },
            if let Some(postgres_database) = &self.postgres_database {
                format!("{:?}", postgres_database).into()
            } else {
                String::new().into()
            },
            if let Some(postgres_written_data) = &self.postgres_written_data {
                format!("{:?}", postgres_written_data).into()
            } else {
                String::new().into()
            },
            if let Some(serverless_function_execution) = &self.serverless_function_execution {
                format!("{:?}", serverless_function_execution).into()
            } else {
                String::new().into()
            },
            if let Some(source_images) = &self.source_images {
                format!("{:?}", source_images).into()
            } else {
                String::new().into()
            },
            if let Some(storage_redis_total_bandwidth_in_bytes) =
                &self.storage_redis_total_bandwidth_in_bytes
            {
                format!("{:?}", storage_redis_total_bandwidth_in_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(storage_redis_total_commands) = &self.storage_redis_total_commands {
                format!("{:?}", storage_redis_total_commands).into()
            } else {
                String::new().into()
            },
            if let Some(storage_redis_total_daily_avg_storage_in_bytes) =
                &self.storage_redis_total_daily_avg_storage_in_bytes
            {
                format!("{:?}", storage_redis_total_daily_avg_storage_in_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(storage_redis_total_databases) = &self.storage_redis_total_databases {
                format!("{:?}", storage_redis_total_databases).into()
            } else {
                String::new().into()
            },
            if let Some(waf_owasp_excess_bytes) = &self.waf_owasp_excess_bytes {
                format!("{:?}", waf_owasp_excess_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(waf_owasp_requests) = &self.waf_owasp_requests {
                format!("{:?}", waf_owasp_requests).into()
            } else {
                String::new().into()
            },
            if let Some(web_analytics_event) = &self.web_analytics_event {
                format!("{:?}", web_analytics_event).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "concurrent_builds".into(),
            "web_analytics".into(),
            "pro".into(),
            "enterprise".into(),
            "analytics".into(),
            "developer_experience_platform".into(),
            "included_allocation_miu".into(),
            "managed_infrastructure_commitment".into(),
            "monitoring".into(),
            "password_protection".into(),
            "preview_deployment_suffix".into(),
            "saml".into(),
            "team_seats".into(),
            "vercel_marketplace".into(),
            "analytics_usage".into(),
            "artifacts".into(),
            "bandwidth".into(),
            "blob_stores".into(),
            "blob_total_advanced_requests".into(),
            "blob_total_avg_size_in_bytes".into(),
            "blob_total_get_response_object_size_in_bytes".into(),
            "blob_total_simple_requests".into(),
            "build_minute".into(),
            "data_cache_read".into(),
            "data_cache_revalidation".into(),
            "data_cache_write".into(),
            "edge_config_read".into(),
            "edge_config_write".into(),
            "edge_function_execution_units".into(),
            "edge_middleware_invocations".into(),
            "edge_request".into(),
            "edge_request_additional_cpu_duration".into(),
            "fast_data_transfer".into(),
            "fast_origin_transfer".into(),
            "function_duration".into(),
            "function_invocation".into(),
            "log_drains_volume".into(),
            "monitoring_metric".into(),
            "postgres_compute_time".into(),
            "postgres_data_storage".into(),
            "postgres_data_transfer".into(),
            "postgres_database".into(),
            "postgres_written_data".into(),
            "serverless_function_execution".into(),
            "source_images".into(),
            "storage_redis_total_bandwidth_in_bytes".into(),
            "storage_redis_total_commands".into(),
            "storage_redis_total_daily_avg_storage_in_bytes".into(),
            "storage_redis_total_databases".into(),
            "waf_owasp_excess_bytes".into(),
            "waf_owasp_requests".into(),
            "web_analytics_event".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InvoiceSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
}

impl std::fmt::Display for InvoiceSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for InvoiceSettings {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(footer) = &self.footer {
            format!("{:?}", footer).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["footer".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SubscriptionsPeriod {
    pub start: f64,
    pub end: f64,
}

impl std::fmt::Display for SubscriptionsPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SubscriptionsPeriod {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.start).into(),
            format!("{:?}", self.end).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["start".into(), "end".into()]
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
pub enum SubscriptionsFrequencyInterval {
    #[serde(rename = "month")]
    #[display("month")]
    Month,
    #[serde(rename = "day")]
    #[display("day")]
    Day,
    #[serde(rename = "week")]
    #[display("week")]
    Week,
    #[serde(rename = "year")]
    #[display("year")]
    Year,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SubscriptionsFrequency {
    pub interval: SubscriptionsFrequencyInterval,
    #[serde(rename = "intervalCount")]
    pub interval_count: f64,
}

impl std::fmt::Display for SubscriptionsFrequency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SubscriptionsFrequency {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.interval).into(),
            format!("{:?}", self.interval_count).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["interval".into(), "interval_count".into()]
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
pub enum Duration {
    #[serde(rename = "forever")]
    #[display("forever")]
    Forever,
    #[serde(rename = "repeating")]
    #[display("repeating")]
    Repeating,
    #[serde(rename = "once")]
    #[display("once")]
    Once,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Coupon {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "amountOff", default, skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<f64>,
    #[serde(
        rename = "percentageOff",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub percentage_off: Option<f64>,
    #[serde(
        rename = "durationInMonths",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_in_months: Option<f64>,
    pub duration: Duration,
}

impl std::fmt::Display for Coupon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Coupon {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            format!("{:?}", self.name).into(),
            format!("{:?}", self.amount_off).into(),
            format!("{:?}", self.percentage_off).into(),
            format!("{:?}", self.duration_in_months).into(),
            format!("{:?}", self.duration).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "amount_off".into(),
            "percentage_off".into(),
            "duration_in_months".into(),
            "duration".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Discount {
    pub id: String,
    pub coupon: Coupon,
}

impl std::fmt::Display for Discount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Discount {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.id.clone().into(), format!("{:?}", self.coupon).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "coupon".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Items {
    pub id: String,
    #[serde(rename = "priceId")]
    pub price_id: String,
    #[serde(rename = "productId")]
    pub product_id: String,
    pub amount: f64,
    pub quantity: f64,
}

impl std::fmt::Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Items {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.price_id.clone().into(),
            self.product_id.clone().into(),
            format!("{:?}", self.amount).into(),
            format!("{:?}", self.quantity).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "price_id".into(),
            "product_id".into(),
            "amount".into(),
            "quantity".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Subscriptions {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trial: Option<Trial>,
    pub period: SubscriptionsPeriod,
    pub frequency: SubscriptionsFrequency,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    pub items: Vec<Items>,
}

impl std::fmt::Display for Subscriptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Subscriptions {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            format!("{:?}", self.trial).into(),
            format!("{:?}", self.period).into(),
            format!("{:?}", self.frequency).into(),
            format!("{:?}", self.discount).into(),
            format!("{:?}", self.items).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "trial".into(),
            "period".into(),
            "frequency".into(),
            "discount".into(),
            "items".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Controls {
    #[serde(
        rename = "analyticsSampleRateInPercent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_sample_rate_in_percent: Option<f64>,
    #[serde(
        rename = "analyticsSpendLimitInDollars",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub analytics_spend_limit_in_dollars: Option<f64>,
}

impl std::fmt::Display for Controls {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Controls {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(analytics_sample_rate_in_percent) = &self.analytics_sample_rate_in_percent {
                format!("{:?}", analytics_sample_rate_in_percent).into()
            } else {
                String::new().into()
            },
            if let Some(analytics_spend_limit_in_dollars) = &self.analytics_spend_limit_in_dollars {
                format!("{:?}", analytics_spend_limit_in_dollars).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "analytics_sample_rate_in_percent".into(),
            "analytics_spend_limit_in_dollars".into(),
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
pub enum Status {
    #[serde(rename = "active")]
    #[display("active")]
    Active,
    #[serde(rename = "trialing")]
    #[display("trialing")]
    Trialing,
    #[serde(rename = "overdue")]
    #[display("overdue")]
    Overdue,
    #[serde(rename = "expired")]
    #[display("expired")]
    Expired,
    #[serde(rename = "canceled")]
    #[display("canceled")]
    Canceled,
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
pub enum PricingExperiment {
    #[serde(rename = "august-2022")]
    #[display("august-2022")]
    August2022,
}

impl std::default::Default for PricingExperiment {
    fn default() -> Self {
        PricingExperiment::August2022
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AwsMarketplace {
    #[serde(rename = "productCode")]
    pub product_code: String,
    #[serde(rename = "offerId", default, skip_serializing_if = "Option::is_none")]
    pub offer_id: Option<String>,
    #[serde(rename = "customerId")]
    pub customer_id: String,
}

impl std::fmt::Display for AwsMarketplace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AwsMarketplace {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.product_code.clone().into(),
            if let Some(offer_id) = &self.offer_id {
                format!("{:?}", offer_id).into()
            } else {
                String::new().into()
            },
            self.customer_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "product_code".into(),
            "offer_id".into(),
            "customer_id".into(),
        ]
    }
}

#[doc = "An object containing billing infomation associated with the User account."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Billing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    pub plan: Plan,
    #[serde(
        rename = "planIteration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_iteration: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(
        rename = "orbCustomerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orb_customer_id: Option<String>,
    #[serde(rename = "syncedAt", default, skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<f64>,
    #[serde(
        rename = "programType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_type: Option<ProgramType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trial: Option<Trial>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<Tax>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "invoiceItems",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invoice_items: Option<InvoiceItems>,
    #[serde(
        rename = "invoiceSettings",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub invoice_settings: Option<InvoiceSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscriptions>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controls: Option<Controls>,
    #[serde(
        rename = "purchaseOrder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub purchase_order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(
        rename = "pricingExperiment",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pricing_experiment: Option<PricingExperiment>,
    #[serde(
        rename = "orbMigrationScheduledAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orb_migration_scheduled_at: Option<f64>,
    #[serde(
        rename = "forceOrbMigration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub force_orb_migration: Option<bool>,
    #[serde(
        rename = "awsMarketplace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_marketplace: Option<AwsMarketplace>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reseller: Option<String>,
}

impl std::fmt::Display for Billing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Billing {
    const LENGTH: usize = 27;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(cancelation) = &self.cancelation {
                format!("{:?}", cancelation).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.period).into(),
            if let Some(contract) = &self.contract {
                format!("{:?}", contract).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.plan).into(),
            if let Some(plan_iteration) = &self.plan_iteration {
                format!("{:?}", plan_iteration).into()
            } else {
                String::new().into()
            },
            if let Some(platform) = &self.platform {
                format!("{:?}", platform).into()
            } else {
                String::new().into()
            },
            if let Some(orb_customer_id) = &self.orb_customer_id {
                format!("{:?}", orb_customer_id).into()
            } else {
                String::new().into()
            },
            if let Some(synced_at) = &self.synced_at {
                format!("{:?}", synced_at).into()
            } else {
                String::new().into()
            },
            if let Some(program_type) = &self.program_type {
                format!("{:?}", program_type).into()
            } else {
                String::new().into()
            },
            if let Some(trial) = &self.trial {
                format!("{:?}", trial).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(tax) = &self.tax {
                format!("{:?}", tax).into()
            } else {
                String::new().into()
            },
            if let Some(language) = &self.language {
                format!("{:?}", language).into()
            } else {
                String::new().into()
            },
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(invoice_items) = &self.invoice_items {
                format!("{:?}", invoice_items).into()
            } else {
                String::new().into()
            },
            if let Some(invoice_settings) = &self.invoice_settings {
                format!("{:?}", invoice_settings).into()
            } else {
                String::new().into()
            },
            if let Some(subscriptions) = &self.subscriptions {
                format!("{:?}", subscriptions).into()
            } else {
                String::new().into()
            },
            if let Some(controls) = &self.controls {
                format!("{:?}", controls).into()
            } else {
                String::new().into()
            },
            if let Some(purchase_order) = &self.purchase_order {
                format!("{:?}", purchase_order).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(pricing_experiment) = &self.pricing_experiment {
                format!("{:?}", pricing_experiment).into()
            } else {
                String::new().into()
            },
            if let Some(orb_migration_scheduled_at) = &self.orb_migration_scheduled_at {
                format!("{:?}", orb_migration_scheduled_at).into()
            } else {
                String::new().into()
            },
            if let Some(force_orb_migration) = &self.force_orb_migration {
                format!("{:?}", force_orb_migration).into()
            } else {
                String::new().into()
            },
            if let Some(aws_marketplace) = &self.aws_marketplace {
                format!("{:?}", aws_marketplace).into()
            } else {
                String::new().into()
            },
            if let Some(reseller) = &self.reseller {
                format!("{:?}", reseller).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "currency".into(),
            "cancelation".into(),
            "period".into(),
            "contract".into(),
            "plan".into(),
            "plan_iteration".into(),
            "platform".into(),
            "orb_customer_id".into(),
            "synced_at".into(),
            "program_type".into(),
            "trial".into(),
            "email".into(),
            "tax".into(),
            "language".into(),
            "address".into(),
            "name".into(),
            "invoice_items".into(),
            "invoice_settings".into(),
            "subscriptions".into(),
            "controls".into(),
            "purchase_order".into(),
            "status".into(),
            "pricing_experiment".into(),
            "orb_migration_scheduled_at".into(),
            "force_orb_migration".into(),
            "aws_marketplace".into(),
            "reseller".into(),
        ]
    }
}

#[doc = "An object containing infomation related to the amount of platform resources may be \
         allocated to the User account."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ResourceConfig {
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "blobStores",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_stores: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(rename = "nodeType", default, skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "concurrentBuilds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub concurrent_builds: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "awsAccountType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_account_type: Option<String>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "awsAccountIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_account_ids: Option<Vec<String>>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "cfZoneName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cf_zone_name: Option<String>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "imageOptimizationType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_optimization_type: Option<String>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "edgeConfigs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_configs: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "edgeConfigSize",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_config_size: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "edgeFunctionMaxSizeBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_function_max_size_bytes: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "edgeFunctionExecutionTimeoutMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_function_execution_timeout_ms: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "serverlessFunctionDefaultMaxExecutionTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub serverless_function_default_max_execution_time: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "kvDatabases",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub kv_databases: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "postgresDatabases",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_databases: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "integrationStores",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_stores: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(rename = "cronJobs", default, skip_serializing_if = "Option::is_none")]
    pub cron_jobs: Option<f64>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(
        rename = "cronJobsPerProject",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cron_jobs_per_project: Option<f64>,
}

impl std::fmt::Display for ResourceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ResourceConfig {
    const LENGTH: usize = 17;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(blob_stores) = &self.blob_stores {
                format!("{:?}", blob_stores).into()
            } else {
                String::new().into()
            },
            if let Some(node_type) = &self.node_type {
                format!("{:?}", node_type).into()
            } else {
                String::new().into()
            },
            if let Some(concurrent_builds) = &self.concurrent_builds {
                format!("{:?}", concurrent_builds).into()
            } else {
                String::new().into()
            },
            if let Some(aws_account_type) = &self.aws_account_type {
                format!("{:?}", aws_account_type).into()
            } else {
                String::new().into()
            },
            if let Some(aws_account_ids) = &self.aws_account_ids {
                format!("{:?}", aws_account_ids).into()
            } else {
                String::new().into()
            },
            if let Some(cf_zone_name) = &self.cf_zone_name {
                format!("{:?}", cf_zone_name).into()
            } else {
                String::new().into()
            },
            if let Some(image_optimization_type) = &self.image_optimization_type {
                format!("{:?}", image_optimization_type).into()
            } else {
                String::new().into()
            },
            if let Some(edge_configs) = &self.edge_configs {
                format!("{:?}", edge_configs).into()
            } else {
                String::new().into()
            },
            if let Some(edge_config_size) = &self.edge_config_size {
                format!("{:?}", edge_config_size).into()
            } else {
                String::new().into()
            },
            if let Some(edge_function_max_size_bytes) = &self.edge_function_max_size_bytes {
                format!("{:?}", edge_function_max_size_bytes).into()
            } else {
                String::new().into()
            },
            if let Some(edge_function_execution_timeout_ms) =
                &self.edge_function_execution_timeout_ms
            {
                format!("{:?}", edge_function_execution_timeout_ms).into()
            } else {
                String::new().into()
            },
            if let Some(serverless_function_default_max_execution_time) =
                &self.serverless_function_default_max_execution_time
            {
                format!("{:?}", serverless_function_default_max_execution_time).into()
            } else {
                String::new().into()
            },
            if let Some(kv_databases) = &self.kv_databases {
                format!("{:?}", kv_databases).into()
            } else {
                String::new().into()
            },
            if let Some(postgres_databases) = &self.postgres_databases {
                format!("{:?}", postgres_databases).into()
            } else {
                String::new().into()
            },
            if let Some(integration_stores) = &self.integration_stores {
                format!("{:?}", integration_stores).into()
            } else {
                String::new().into()
            },
            if let Some(cron_jobs) = &self.cron_jobs {
                format!("{:?}", cron_jobs).into()
            } else {
                String::new().into()
            },
            if let Some(cron_jobs_per_project) = &self.cron_jobs_per_project {
                format!("{:?}", cron_jobs_per_project).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "blob_stores".into(),
            "node_type".into(),
            "concurrent_builds".into(),
            "aws_account_type".into(),
            "aws_account_ids".into(),
            "cf_zone_name".into(),
            "image_optimization_type".into(),
            "edge_configs".into(),
            "edge_config_size".into(),
            "edge_function_max_size_bytes".into(),
            "edge_function_execution_timeout_ms".into(),
            "serverless_function_default_max_execution_time".into(),
            "kv_databases".into(),
            "postgres_databases".into(),
            "integration_stores".into(),
            "cron_jobs".into(),
            "cron_jobs_per_project".into(),
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
pub enum ViewPreference {
    #[serde(rename = "list")]
    #[display("list")]
    List,
    #[serde(rename = "cards")]
    #[display("cards")]
    Cards,
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
pub enum FavoritesViewPreference {
    #[serde(rename = "open")]
    #[display("open")]
    Open,
    #[serde(rename = "closed")]
    #[display("closed")]
    Closed,
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
pub enum RecentsViewPreference {
    #[serde(rename = "open")]
    #[display("open")]
    Open,
    #[serde(rename = "closed")]
    #[display("closed")]
    Closed,
}

#[doc = "set of dashboard view preferences (cards or list) per scopeId"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ActiveDashboardViews {
    #[serde(rename = "scopeId")]
    pub scope_id: String,
    #[serde(
        rename = "viewPreference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub view_preference: Option<ViewPreference>,
    #[serde(
        rename = "favoritesViewPreference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub favorites_view_preference: Option<FavoritesViewPreference>,
    #[serde(
        rename = "recentsViewPreference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub recents_view_preference: Option<RecentsViewPreference>,
}

impl std::fmt::Display for ActiveDashboardViews {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ActiveDashboardViews {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.scope_id.clone().into(),
            if let Some(view_preference) = &self.view_preference {
                format!("{:?}", view_preference).into()
            } else {
                String::new().into()
            },
            if let Some(favorites_view_preference) = &self.favorites_view_preference {
                format!("{:?}", favorites_view_preference).into()
            } else {
                String::new().into()
            },
            if let Some(recents_view_preference) = &self.recents_view_preference {
                format!("{:?}", recents_view_preference).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "scope_id".into(),
            "view_preference".into(),
            "favorites_view_preference".into(),
            "recents_view_preference".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ImportFlowGitNamespace {
    String(String),
    F64(f64),
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum ImportFlowGitNamespaceId {
    String(String),
    F64(f64),
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
pub enum ImportFlowGitProvider {
    #[serde(rename = "github")]
    #[display("github")]
    Github,
    #[serde(rename = "gitlab")]
    #[display("gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    #[display("bitbucket")]
    Bitbucket,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum GitNamespaceId {
    String(String),
    F64(f64),
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PreferredScopesAndGitNamespaces {
    #[serde(rename = "scopeId")]
    pub scope_id: String,
    #[serde(rename = "gitNamespaceId")]
    pub git_namespace_id: GitNamespaceId,
}

impl std::fmt::Display for PreferredScopesAndGitNamespaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PreferredScopesAndGitNamespaces {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.scope_id.clone().into(),
            format!("{:?}", self.git_namespace_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["scope_id".into(), "git_namespace_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Dismissals {
    #[serde(rename = "scopeId")]
    pub scope_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
}

impl std::fmt::Display for Dismissals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Dismissals {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.scope_id.clone().into(),
            format!("{:?}", self.created_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["scope_id".into(), "created_at".into()]
    }
}

#[doc = "A record of when, under a certain scopeId, a toast was dismissed"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DismissedToasts {
    pub name: String,
    pub dismissals: Vec<Dismissals>,
}

impl std::fmt::Display for DismissedToasts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DismissedToasts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.dismissals).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "dismissals".into()]
    }
}

#[doc = "A list of projects and spaces across teams that a user has marked as a favorite."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FavoriteProjectsAndSpacesProjectId {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "scopeSlug")]
    pub scope_slug: String,
    #[serde(rename = "scopeId")]
    pub scope_id: String,
}

impl std::fmt::Display for FavoriteProjectsAndSpacesProjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FavoriteProjectsAndSpacesProjectId {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.project_id.clone().into(),
            self.scope_slug.clone().into(),
            self.scope_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["project_id".into(), "scope_slug".into(), "scope_id".into()]
    }
}

#[doc = "A list of projects and spaces across teams that a user has marked as a favorite."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FavoriteProjectsAndSpacesSpaceId {
    #[serde(rename = "spaceId")]
    pub space_id: String,
    #[serde(rename = "scopeSlug")]
    pub scope_slug: String,
    #[serde(rename = "scopeId")]
    pub scope_id: String,
}

impl std::fmt::Display for FavoriteProjectsAndSpacesSpaceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FavoriteProjectsAndSpacesSpaceId {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.space_id.clone().into(),
            self.scope_slug.clone().into(),
            self.scope_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["space_id".into(), "scope_slug".into(), "scope_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum FavoriteProjectsAndSpaces {
    FavoriteProjectsAndSpacesProjectId(FavoriteProjectsAndSpacesProjectId),
    FavoriteProjectsAndSpacesSpaceId(FavoriteProjectsAndSpacesSpaceId),
}

#[doc = "remote caching settings"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RemoteCaching {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl std::fmt::Display for RemoteCaching {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RemoteCaching {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(enabled) = &self.enabled {
            format!("{:?}", enabled).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["enabled".into()]
    }
}

#[doc = "data cache settings"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DataCache {
    #[serde(
        rename = "excessBillingEnabled",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub excess_billing_enabled: Option<bool>,
}

impl std::fmt::Display for DataCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DataCache {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(excess_billing_enabled) = &self.excess_billing_enabled {
                format!("{:?}", excess_billing_enabled).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["excess_billing_enabled".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FeatureBlocksWebAnalytics {
    #[serde(
        rename = "blockedFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_from: Option<f64>,
    #[serde(
        rename = "blockedUntil",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blocked_until: Option<f64>,
    #[serde(rename = "isCurrentlyBlocked")]
    pub is_currently_blocked: bool,
}

impl std::fmt::Display for FeatureBlocksWebAnalytics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FeatureBlocksWebAnalytics {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(blocked_from) = &self.blocked_from {
                format!("{:?}", blocked_from).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_until) = &self.blocked_until {
                format!("{:?}", blocked_until).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_currently_blocked).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "blocked_from".into(),
            "blocked_until".into(),
            "is_currently_blocked".into(),
        ]
    }
}

#[doc = "Feature blocks for the user"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FeatureBlocks {
    #[serde(
        rename = "webAnalytics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_analytics: Option<FeatureBlocksWebAnalytics>,
}

impl std::fmt::Display for FeatureBlocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FeatureBlocks {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(web_analytics) = &self.web_analytics {
            format!("{:?}", web_analytics).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["web_analytics".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct NorthstarMigration {
    #[doc = "The ID of the team we created for this user."]
    #[serde(rename = "teamId")]
    pub team_id: String,
    #[doc = "The number of projects migrated for this user."]
    pub projects: f64,
    #[doc = "The number of stores migrated for this user."]
    pub stores: f64,
    #[doc = "The number of integration configurations migrated for this user."]
    #[serde(rename = "integrationConfigurations")]
    pub integration_configurations: f64,
    #[doc = "The number of integration clients migrated for this user."]
    #[serde(rename = "integrationClients")]
    pub integration_clients: f64,
    #[doc = "The migration start time timestamp for this user."]
    #[serde(rename = "startTime")]
    pub start_time: f64,
    #[doc = "The migration end time timestamp for this user."]
    #[serde(rename = "endTime")]
    pub end_time: f64,
}

impl std::fmt::Display for NorthstarMigration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NorthstarMigration {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.team_id.clone().into(),
            format!("{:?}", self.projects).into(),
            format!("{:?}", self.stores).into(),
            format!("{:?}", self.integration_configurations).into(),
            format!("{:?}", self.integration_clients).into(),
            format!("{:?}", self.start_time).into(),
            format!("{:?}", self.end_time).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "team_id".into(),
            "projects".into(),
            "stores".into(),
            "integration_configurations".into(),
            "integration_clients".into(),
            "start_time".into(),
            "end_time".into(),
        ]
    }
}

#[doc = "The user's version. Will either be unset or `northstar`."]
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
pub enum Version {
    #[serde(rename = "northstar")]
    #[display("northstar")]
    Northstar,
}

impl std::default::Default for Version {
    fn default() -> Self {
        Version::Northstar
    }
}

#[doc = "Data for the currently authenticated User."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AuthUser {
    #[doc = "UNIX timestamp (in milliseconds) when the User account was created."]
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    #[doc = "When the User account has been \"soft blocked\", this property will contain the date \
             when the restriction was enacted, and the identifier for why."]
    #[serde(rename = "softBlock", default, skip_serializing_if = "Option::is_none")]
    pub soft_block: Option<SoftBlock>,
    #[doc = "An object containing billing infomation associated with the User account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing: Option<Billing>,
    #[doc = "An object containing infomation related to the amount of platform resources may be \
             allocated to the User account."]
    #[serde(rename = "resourceConfig")]
    pub resource_config: ResourceConfig,
    #[doc = "Prefix that will be used in the URL of \"Preview\" deployments created by the User \
             account."]
    #[serde(rename = "stagingPrefix")]
    pub staging_prefix: String,
    #[doc = "set of dashboard view preferences (cards or list) per scopeId"]
    #[serde(
        rename = "activeDashboardViews",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub active_dashboard_views: Option<Vec<ActiveDashboardViews>>,
    #[serde(
        rename = "importFlowGitNamespace",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_flow_git_namespace: Option<ImportFlowGitNamespace>,
    #[serde(
        rename = "importFlowGitNamespaceId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_flow_git_namespace_id: Option<ImportFlowGitNamespaceId>,
    #[serde(
        rename = "importFlowGitProvider",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub import_flow_git_provider: Option<ImportFlowGitProvider>,
    #[serde(
        rename = "preferredScopesAndGitNamespaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_scopes_and_git_namespaces: Option<Vec<PreferredScopesAndGitNamespaces>>,
    #[doc = "A record of when, under a certain scopeId, a toast was dismissed"]
    #[serde(
        rename = "dismissedToasts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dismissed_toasts: Option<Vec<DismissedToasts>>,
    #[doc = "A list of projects and spaces across teams that a user has marked as a favorite."]
    #[serde(
        rename = "favoriteProjectsAndSpaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub favorite_projects_and_spaces: Option<Vec<FavoriteProjectsAndSpaces>>,
    #[doc = "Whether the user has a trial available for a paid plan subscription."]
    #[serde(rename = "hasTrialAvailable")]
    pub has_trial_available: bool,
    #[doc = "remote caching settings"]
    #[serde(
        rename = "remoteCaching",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_caching: Option<RemoteCaching>,
    #[doc = "data cache settings"]
    #[serde(rename = "dataCache", default, skip_serializing_if = "Option::is_none")]
    pub data_cache: Option<DataCache>,
    #[doc = "Feature blocks for the user"]
    #[serde(
        rename = "featureBlocks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub feature_blocks: Option<FeatureBlocks>,
    #[serde(
        rename = "northstarMigration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub northstar_migration: Option<NorthstarMigration>,
    #[doc = "The User's unique identifier."]
    pub id: String,
    #[doc = "Email address associated with the User account."]
    pub email: String,
    #[doc = "Name associated with the User account, or `null` if none has been provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Unique username associated with the User account."]
    pub username: String,
    #[doc = "SHA1 hash of the avatar for the User account. Can be used in conjuction with the ... \
             endpoint to retrieve the avatar image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[doc = "The user's default team. Only applies if the user's `version` is `'northstar'`."]
    #[serde(
        rename = "defaultTeamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team_id: Option<String>,
    #[doc = "The user's version. Will either be unset or `northstar`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

impl std::fmt::Display for AuthUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AuthUser {
    const LENGTH: usize = 24;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.created_at).into(),
            format!("{:?}", self.soft_block).into(),
            format!("{:?}", self.billing).into(),
            format!("{:?}", self.resource_config).into(),
            self.staging_prefix.clone().into(),
            if let Some(active_dashboard_views) = &self.active_dashboard_views {
                format!("{:?}", active_dashboard_views).into()
            } else {
                String::new().into()
            },
            if let Some(import_flow_git_namespace) = &self.import_flow_git_namespace {
                format!("{:?}", import_flow_git_namespace).into()
            } else {
                String::new().into()
            },
            if let Some(import_flow_git_namespace_id) = &self.import_flow_git_namespace_id {
                format!("{:?}", import_flow_git_namespace_id).into()
            } else {
                String::new().into()
            },
            if let Some(import_flow_git_provider) = &self.import_flow_git_provider {
                format!("{:?}", import_flow_git_provider).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_scopes_and_git_namespaces) =
                &self.preferred_scopes_and_git_namespaces
            {
                format!("{:?}", preferred_scopes_and_git_namespaces).into()
            } else {
                String::new().into()
            },
            if let Some(dismissed_toasts) = &self.dismissed_toasts {
                format!("{:?}", dismissed_toasts).into()
            } else {
                String::new().into()
            },
            if let Some(favorite_projects_and_spaces) = &self.favorite_projects_and_spaces {
                format!("{:?}", favorite_projects_and_spaces).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.has_trial_available).into(),
            if let Some(remote_caching) = &self.remote_caching {
                format!("{:?}", remote_caching).into()
            } else {
                String::new().into()
            },
            if let Some(data_cache) = &self.data_cache {
                format!("{:?}", data_cache).into()
            } else {
                String::new().into()
            },
            if let Some(feature_blocks) = &self.feature_blocks {
                format!("{:?}", feature_blocks).into()
            } else {
                String::new().into()
            },
            if let Some(northstar_migration) = &self.northstar_migration {
                format!("{:?}", northstar_migration).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.email.clone().into(),
            format!("{:?}", self.name).into(),
            self.username.clone().into(),
            format!("{:?}", self.avatar).into(),
            format!("{:?}", self.default_team_id).into(),
            format!("{:?}", self.version).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "soft_block".into(),
            "billing".into(),
            "resource_config".into(),
            "staging_prefix".into(),
            "active_dashboard_views".into(),
            "import_flow_git_namespace".into(),
            "import_flow_git_namespace_id".into(),
            "import_flow_git_provider".into(),
            "preferred_scopes_and_git_namespaces".into(),
            "dismissed_toasts".into(),
            "favorite_projects_and_spaces".into(),
            "has_trial_available".into(),
            "remote_caching".into(),
            "data_cache".into(),
            "feature_blocks".into(),
            "northstar_migration".into(),
            "id".into(),
            "email".into(),
            "name".into(),
            "username".into(),
            "avatar".into(),
            "default_team_id".into(),
            "version".into(),
        ]
    }
}

#[doc = "A limited form of data for the currently authenticated User, due to the authentication \
         token missing privileges to read the full User data."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AuthUserLimited {
    #[doc = "Property indicating that this User data contains only limited information, due to \
             the authentication token missing privileges to read the full User data. Re-login \
             with email, GitHub, GitLab or Bitbucket in order to upgrade the authentication token \
             with the necessary privileges."]
    pub limited: bool,
    #[doc = "The User's unique identifier."]
    pub id: String,
    #[doc = "Email address associated with the User account."]
    pub email: String,
    #[doc = "Name associated with the User account, or `null` if none has been provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Unique username associated with the User account."]
    pub username: String,
    #[doc = "SHA1 hash of the avatar for the User account. Can be used in conjuction with the ... \
             endpoint to retrieve the avatar image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[doc = "The user's default team. Only applies if the user's `version` is `'northstar'`."]
    #[serde(
        rename = "defaultTeamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_team_id: Option<String>,
    #[doc = "The user's version. Will either be unset or `northstar`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

impl std::fmt::Display for AuthUserLimited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AuthUserLimited {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.limited).into(),
            self.id.clone().into(),
            self.email.clone().into(),
            format!("{:?}", self.name).into(),
            self.username.clone().into(),
            format!("{:?}", self.avatar).into(),
            format!("{:?}", self.default_team_id).into(),
            format!("{:?}", self.version).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "limited".into(),
            "id".into(),
            "email".into(),
            "name".into(),
            "username".into(),
            "avatar".into(),
            "default_team_id".into(),
            "version".into(),
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
pub enum UserOrigin {
    #[serde(rename = "saml")]
    #[display("saml")]
    Saml,
    #[serde(rename = "github")]
    #[display("github")]
    Github,
    #[serde(rename = "gitlab")]
    #[display("gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    #[display("bitbucket")]
    Bitbucket,
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "manual")]
    #[display("manual")]
    Manual,
    #[serde(rename = "passkey")]
    #[display("passkey")]
    Passkey,
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
pub enum TeamOrigin {
    #[serde(rename = "saml")]
    #[display("saml")]
    Saml,
    #[serde(rename = "github")]
    #[display("github")]
    Github,
    #[serde(rename = "gitlab")]
    #[display("gitlab")]
    Gitlab,
    #[serde(rename = "bitbucket")]
    #[display("bitbucket")]
    Bitbucket,
    #[serde(rename = "email")]
    #[display("email")]
    Email,
    #[serde(rename = "manual")]
    #[display("manual")]
    Manual,
    #[serde(rename = "passkey")]
    #[display("passkey")]
    Passkey,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
#[serde(tag = "type")]
pub enum Scopes {
    #[doc = "The access scopes granted to the token."]
    #[serde(rename = "user")]
    User {
        origin: UserOrigin,
        #[serde(rename = "createdAt")]
        created_at: f64,
        #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
        expires_at: Option<f64>,
    },
    #[doc = "The access scopes granted to the token."]
    #[serde(rename = "team")]
    Team {
        #[serde(rename = "teamId")]
        team_id: String,
        origin: TeamOrigin,
        #[serde(rename = "createdAt")]
        created_at: f64,
        #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
        expires_at: Option<f64>,
    },
}

#[doc = "Authentication token metadata."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AuthToken {
    #[doc = "The unique identifier of the token."]
    pub id: String,
    #[doc = "The human-readable name of the token."]
    pub name: String,
    #[doc = "The type of the token."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The origin of how the token was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[doc = "The access scopes granted to the token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<Scopes>>,
    #[doc = "Timestamp (in milliseconds) of when the token expires."]
    #[serde(rename = "expiresAt", default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    #[doc = "Timestamp (in milliseconds) of when the token was most recently used."]
    #[serde(rename = "activeAt")]
    pub active_at: f64,
    #[doc = "Timestamp (in milliseconds) of when the token was created."]
    #[serde(rename = "createdAt")]
    pub created_at: f64,
}

impl std::fmt::Display for AuthToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AuthToken {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.name.clone().into(),
            self.type_.clone().into(),
            if let Some(origin) = &self.origin {
                format!("{:?}", origin).into()
            } else {
                String::new().into()
            },
            if let Some(scopes) = &self.scopes {
                format!("{:?}", scopes).into()
            } else {
                String::new().into()
            },
            if let Some(expires_at) = &self.expires_at {
                format!("{:?}", expires_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.active_at).into(),
            format!("{:?}", self.created_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "type_".into(),
            "origin".into(),
            "scopes".into(),
            "expires_at".into(),
            "active_at".into(),
            "created_at".into(),
        ]
    }
}

#[doc = "String indicating the type of file tree entry."]
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
pub enum FileTreeType {
    #[serde(rename = "directory")]
    #[display("directory")]
    Directory,
    #[serde(rename = "file")]
    #[display("file")]
    File,
    #[serde(rename = "symlink")]
    #[display("symlink")]
    Symlink,
    #[serde(rename = "lambda")]
    #[display("lambda")]
    Lambda,
    #[serde(rename = "middleware")]
    #[display("middleware")]
    Middleware,
    #[serde(rename = "invalid")]
    #[display("invalid")]
    Invalid,
}

#[doc = "A deployment file tree entry"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FileTree {
    #[doc = "The name of the file tree entry"]
    pub name: String,
    #[doc = "String indicating the type of file tree entry."]
    #[serde(rename = "type")]
    pub type_: FileTreeType,
    #[doc = "The unique identifier of the file (only valid for the `file` type)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[doc = "The list of children files of the directory (only valid for the `directory` type)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<FileTree>>,
    #[doc = "The content-type of the file (only valid for the `file` type)"]
    #[serde(
        rename = "contentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub content_type: Option<String>,
    #[doc = "The file \"mode\" indicating file type and permissions."]
    pub mode: f64,
    #[doc = "Not currently used. See `file-list-to-tree.ts`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symlink: Option<String>,
}

impl std::fmt::Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FileTree {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.type_).into(),
            if let Some(uid) = &self.uid {
                format!("{:?}", uid).into()
            } else {
                String::new().into()
            },
            if let Some(children) = &self.children {
                format!("{:?}", children).into()
            } else {
                String::new().into()
            },
            if let Some(content_type) = &self.content_type {
                format!("{:?}", content_type).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.mode).into(),
            if let Some(symlink) = &self.symlink {
                format!("{:?}", symlink).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "type_".into(),
            "uid".into(),
            "children".into(),
            "content_type".into(),
            "mode".into(),
            "symlink".into(),
        ]
    }
}
