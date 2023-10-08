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

impl tabled::Tabled for Pagination {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.count),
            format!("{:?}", self.next),
            format!("{:?}", self.prev),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["count".to_string(), "next".to_string(), "prev".to_string()]
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

impl tabled::Tabled for EdgeConfigItem {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.key.clone(),
            format!("{:?}", self.value),
            self.edge_config_id.clone(),
            format!("{:?}", self.created_at),
            format!("{:?}", self.updated_at),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "key".to_string(),
            "value".to_string(),
            "edge_config_id".to_string(),
            "created_at".to_string(),
            "updated_at".to_string(),
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

impl tabled::Tabled for EdgeConfigToken {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.token.clone(),
            self.label.clone(),
            self.id.clone(),
            self.edge_config_id.clone(),
            format!("{:?}", self.created_at),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "token".to_string(),
            "label".to_string(),
            "id".to_string(),
            "edge_config_id".to_string(),
            "created_at".to_string(),
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
pub enum Type {
    #[serde(rename = "target")]
    #[display("target")]
    Target,
    #[serde(rename = "bold")]
    #[display("bold")]
    Bold,
    #[serde(rename = "link")]
    #[display("link")]
    Link,
    #[serde(rename = "author")]
    #[display("author")]
    Author,
    #[serde(rename = "bitbucket_login")]
    #[display("bitbucket_login")]
    BitbucketLogin,
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
    #[serde(rename = "project_name")]
    #[display("project_name")]
    ProjectName,
    #[serde(rename = "scaling_rules")]
    #[display("scaling_rules")]
    ScalingRules,
    #[serde(rename = "env_var_name")]
    #[display("env_var_name")]
    EnvVarName,
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

impl tabled::Tabled for Entities {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.type_),
            format!("{:?}", self.start),
            format!("{:?}", self.end),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["type_".to_string(), "start".to_string(), "end".to_string()]
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

impl tabled::Tabled for User {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.avatar.clone(),
            self.email.clone(),
            if let Some(slug) = &self.slug {
                format!("{:?}", slug)
            } else {
                String::new()
            },
            self.uid.clone(),
            self.username.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "avatar".to_string(),
            "email".to_string(),
            "slug".to_string(),
            "uid".to_string(),
            "username".to_string(),
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

impl tabled::Tabled for UserEvent {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.text.clone(),
            format!("{:?}", self.entities),
            format!("{:?}", self.created_at),
            if let Some(user) = &self.user {
                format!("{:?}", user)
            } else {
                String::new()
            },
            self.user_id.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "text".to_string(),
            "entities".to_string(),
            "created_at".to_string(),
            "user".to_string(),
            "user_id".to_string(),
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
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
    #[serde(rename = "count")]
    #[display("count")]
    Count,
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

impl tabled::Tabled for Team {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<String> {
        vec![]
    }

    fn headers() -> Vec<String> {
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

impl tabled::Tabled for Connection {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.type_.clone(),
            self.status.clone(),
            self.state.clone(),
            format!("{:?}", self.connected_at),
            if let Some(last_received_webhook_event) = &self.last_received_webhook_event {
                format!("{:?}", last_received_webhook_event)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "type_".to_string(),
            "status".to_string(),
            "state".to_string(),
            "connected_at".to_string(),
            "last_received_webhook_event".to_string(),
        ]
    }
}

#[doc = "Information for the SAML Single Sign-On configuration."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Directory {
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

impl std::fmt::Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Directory {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.type_.clone(),
            self.status.clone(),
            self.state.clone(),
            format!("{:?}", self.connected_at),
            if let Some(last_received_webhook_event) = &self.last_received_webhook_event {
                format!("{:?}", last_received_webhook_event)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "type_".to_string(),
            "status".to_string(),
            "state".to_string(),
            "connected_at".to_string(),
            "last_received_webhook_event".to_string(),
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
    #[doc = "Information for the SAML Single Sign-On configuration."]
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

impl tabled::Tabled for Saml {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(connection) = &self.connection {
                format!("{:?}", connection)
            } else {
                String::new()
            },
            if let Some(directory) = &self.directory {
                format!("{:?}", directory)
            } else {
                String::new()
            },
            format!("{:?}", self.enforced),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "connection".to_string(),
            "directory".to_string(),
            "enforced".to_string(),
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
pub enum Role {
    #[serde(rename = "MEMBER")]
    #[display("MEMBER")]
    Member,
    #[serde(rename = "OWNER")]
    #[display("OWNER")]
    Owner,
    #[serde(rename = "VIEWER")]
    #[display("VIEWER")]
    Viewer,
    #[serde(rename = "DEVELOPER")]
    #[display("DEVELOPER")]
    Developer,
    #[serde(rename = "BILLING")]
    #[display("BILLING")]
    Billing,
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
pub enum Origin {
    #[serde(rename = "link")]
    #[display("link")]
    Link,
    #[serde(rename = "import")]
    #[display("import")]
    Import,
    #[serde(rename = "saml")]
    #[display("saml")]
    Saml,
    #[serde(rename = "mail")]
    #[display("mail")]
    Mail,
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
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
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

impl tabled::Tabled for JoinedFrom {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.origin),
            if let Some(commit_id) = &self.commit_id {
                format!("{:?}", commit_id)
            } else {
                String::new()
            },
            if let Some(repo_id) = &self.repo_id {
                format!("{:?}", repo_id)
            } else {
                String::new()
            },
            if let Some(repo_path) = &self.repo_path {
                format!("{:?}", repo_path)
            } else {
                String::new()
            },
            if let Some(git_user_id) = &self.git_user_id {
                format!("{:?}", git_user_id)
            } else {
                String::new()
            },
            if let Some(git_user_login) = &self.git_user_login {
                format!("{:?}", git_user_login)
            } else {
                String::new()
            },
            if let Some(sso_user_id) = &self.sso_user_id {
                format!("{:?}", sso_user_id)
            } else {
                String::new()
            },
            if let Some(sso_connected_at) = &self.sso_connected_at {
                format!("{:?}", sso_connected_at)
            } else {
                String::new()
            },
            if let Some(idp_user_id) = &self.idp_user_id {
                format!("{:?}", idp_user_id)
            } else {
                String::new()
            },
            if let Some(dsync_user_id) = &self.dsync_user_id {
                format!("{:?}", dsync_user_id)
            } else {
                String::new()
            },
            if let Some(dsync_connected_at) = &self.dsync_connected_at {
                format!("{:?}", dsync_connected_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "origin".to_string(),
            "commit_id".to_string(),
            "repo_id".to_string(),
            "repo_path".to_string(),
            "git_user_id".to_string(),
            "git_user_login".to_string(),
            "sso_user_id".to_string(),
            "sso_connected_at".to_string(),
            "idp_user_id".to_string(),
            "dsync_user_id".to_string(),
            "dsync_connected_at".to_string(),
        ]
    }
}

#[doc = "The membership of the authenticated User in relation to the Team."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MembershipOneOf {
    pub confirmed: bool,
    #[serde(rename = "confirmedAt")]
    pub confirmed_at: f64,
    #[serde(
        rename = "accessRequestedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub access_requested_at: Option<f64>,
    pub role: Role,
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    pub uid: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    pub created: f64,
    #[serde(
        rename = "joinedFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub joined_from: Option<JoinedFrom>,
}

impl std::fmt::Display for MembershipOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MembershipOneOf {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.confirmed),
            format!("{:?}", self.confirmed_at),
            if let Some(access_requested_at) = &self.access_requested_at {
                format!("{:?}", access_requested_at)
            } else {
                String::new()
            },
            format!("{:?}", self.role),
            if let Some(team_id) = &self.team_id {
                format!("{:?}", team_id)
            } else {
                String::new()
            },
            self.uid.clone(),
            format!("{:?}", self.created_at),
            format!("{:?}", self.created),
            if let Some(joined_from) = &self.joined_from {
                format!("{:?}", joined_from)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "confirmed".to_string(),
            "confirmed_at".to_string(),
            "access_requested_at".to_string(),
            "role".to_string(),
            "team_id".to_string(),
            "uid".to_string(),
            "created_at".to_string(),
            "created".to_string(),
            "joined_from".to_string(),
        ]
    }
}

#[doc = "The membership of the authenticated User in relation to the Team."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MembershipOneOfOneOf {
    pub confirmed: bool,
    #[serde(
        rename = "confirmedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub confirmed_at: Option<f64>,
    #[serde(rename = "accessRequestedAt")]
    pub access_requested_at: f64,
    pub role: Role,
    #[serde(rename = "teamId", default, skip_serializing_if = "Option::is_none")]
    pub team_id: Option<String>,
    pub uid: String,
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    pub created: f64,
    #[serde(
        rename = "joinedFrom",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub joined_from: Option<JoinedFrom>,
}

impl std::fmt::Display for MembershipOneOfOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for MembershipOneOfOneOf {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.confirmed),
            if let Some(confirmed_at) = &self.confirmed_at {
                format!("{:?}", confirmed_at)
            } else {
                String::new()
            },
            format!("{:?}", self.access_requested_at),
            format!("{:?}", self.role),
            if let Some(team_id) = &self.team_id {
                format!("{:?}", team_id)
            } else {
                String::new()
            },
            self.uid.clone(),
            format!("{:?}", self.created_at),
            format!("{:?}", self.created),
            if let Some(joined_from) = &self.joined_from {
                format!("{:?}", joined_from)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "confirmed".to_string(),
            "confirmed_at".to_string(),
            "access_requested_at".to_string(),
            "role".to_string(),
            "team_id".to_string(),
            "uid".to_string(),
            "created_at".to_string(),
            "created".to_string(),
            "joined_from".to_string(),
        ]
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
pub enum Membership {
    MembershipOneOf(MembershipOneOf),
    MembershipOneOfOneOf(MembershipOneOfOneOf),
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

impl tabled::Tabled for TeamLimited {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.limited),
            if let Some(saml) = &self.saml {
                format!("{:?}", saml)
            } else {
                String::new()
            },
            self.id.clone(),
            self.slug.clone(),
            format!("{:?}", self.name),
            format!("{:?}", self.avatar),
            format!("{:?}", self.membership),
            self.created.clone(),
            format!("{:?}", self.created_at),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "limited".to_string(),
            "saml".to_string(),
            "id".to_string(),
            "slug".to_string(),
            "name".to_string(),
            "avatar".to_string(),
            "membership".to_string(),
            "created".to_string(),
            "created_at".to_string(),
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
pub enum Reason {
    #[serde(rename = "FAIR_USE_LIMITS_EXCEEDED")]
    #[display("FAIR_USE_LIMITS_EXCEEDED")]
    FairUseLimitsExceeded,
    #[serde(rename = "ENTERPRISE_TRIAL_ENDED")]
    #[display("ENTERPRISE_TRIAL_ENDED")]
    EnterpriseTrialEnded,
    #[serde(rename = "BLOCKED_FOR_PLATFORM_ABUSE")]
    #[display("BLOCKED_FOR_PLATFORM_ABUSE")]
    BlockedForPlatformAbuse,
    #[serde(rename = "UNPAID_INVOICE")]
    #[display("UNPAID_INVOICE")]
    UnpaidInvoice,
    #[serde(rename = "SUBSCRIPTION_EXPIRED")]
    #[display("SUBSCRIPTION_EXPIRED")]
    SubscriptionExpired,
    #[serde(rename = "SUBSCRIPTION_CANCELED")]
    #[display("SUBSCRIPTION_CANCELED")]
    SubscriptionCanceled,
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

impl tabled::Tabled for SoftBlock {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.blocked_at),
            format!("{:?}", self.reason),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["blocked_at".to_string(), "reason".to_string()]
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
pub enum Currency {
    #[serde(rename = "usd")]
    #[display("usd")]
    Usd,
    #[serde(rename = "eur")]
    #[display("eur")]
    Eur,
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
pub enum Addons {
    #[serde(rename = "custom-deployment-suffix")]
    #[display("custom-deployment-suffix")]
    CustomDeploymentSuffix,
    #[serde(rename = "live-support")]
    #[display("live-support")]
    LiveSupport,
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

impl tabled::Tabled for Period {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.start), format!("{:?}", self.end)]
    }

    fn headers() -> Vec<String> {
        vec!["start".to_string(), "end".to_string()]
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

impl tabled::Tabled for Contract {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.start), format!("{:?}", self.end)]
    }

    fn headers() -> Vec<String> {
        vec!["start".to_string(), "end".to_string()]
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
pub enum Plan {
    #[serde(rename = "hobby")]
    #[display("hobby")]
    Hobby,
    #[serde(rename = "enterprise")]
    #[display("enterprise")]
    Enterprise,
    #[serde(rename = "pro")]
    #[display("pro")]
    Pro,
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
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

impl tabled::Tabled for Trial {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.start), format!("{:?}", self.end)]
    }

    fn headers() -> Vec<String> {
        vec!["start".to_string(), "end".to_string()]
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

impl tabled::Tabled for Tax {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.type_.clone(), self.id.clone()]
    }

    fn headers() -> Vec<String> {
        vec!["type_".to_string(), "id".to_string()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Address {
    #[serde(rename = "line1")]
    pub line_1: String,
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

impl tabled::Tabled for Address {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            self.line_1.clone(),
            if let Some(line_2) = &self.line_2 {
                format!("{:?}", line_2)
            } else {
                String::new()
            },
            if let Some(postal_code) = &self.postal_code {
                format!("{:?}", postal_code)
            } else {
                String::new()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city)
            } else {
                String::new()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country)
            } else {
                String::new()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "line_1".to_string(),
            "line_2".to_string(),
            "postal_code".to_string(),
            "city".to_string(),
            "country".to_string(),
            "state".to_string(),
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
#[derive(Default)]
pub enum Interval {
    #[serde(rename = "month")]
    #[display("month")]
    #[default]
    Month,
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

impl tabled::Tabled for Frequency {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.interval),
            format!("{:?}", self.interval_count),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["interval".to_string(), "interval_count".to_string()]
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

impl tabled::Tabled for Pro {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for Enterprise {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
        ]
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

impl tabled::Tabled for ConcurrentBuilds {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for InvoiceItemsSaml {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for TeamSeats {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomCerts {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
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

impl std::fmt::Display for CustomCerts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CustomCerts {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for PreviewDeploymentSuffix {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for PasswordProtection {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
        ]
    }
}

#[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SsoProtection {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<f64>,
    pub price: f64,
    pub quantity: f64,
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

impl std::fmt::Display for SsoProtection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for SsoProtection {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for Analytics {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for Monitoring {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
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

impl tabled::Tabled for WebAnalytics {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.quantity),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at)
            } else {
                String::new()
            },
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency)
            } else {
                String::new()
            },
            if let Some(max_quantity) = &self.max_quantity {
                format!("{:?}", max_quantity)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "quantity".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "created_at".to_string(),
            "disabled_at".to_string(),
            "frequency".to_string(),
            "max_quantity".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AnalyticsUsage {
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

impl tabled::Tabled for AnalyticsUsage {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Artifacts {
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

impl tabled::Tabled for Artifacts {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Bandwidth {
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

impl tabled::Tabled for Bandwidth {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Builds {
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
}

impl std::fmt::Display for Builds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for Builds {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeMiddlewareInvocations {
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

impl tabled::Tabled for EdgeMiddlewareInvocations {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeFunctionExecutionUnits {
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

impl tabled::Tabled for EdgeFunctionExecutionUnits {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MonitoringMetric {
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

impl tabled::Tabled for MonitoringMetric {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ServerlessFunctionExecution {
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

impl tabled::Tabled for ServerlessFunctionExecution {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SourceImages {
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

impl tabled::Tabled for SourceImages {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WebAnalyticsEvent {
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

impl tabled::Tabled for WebAnalyticsEvent {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeConfigRead {
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

impl tabled::Tabled for EdgeConfigRead {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EdgeConfigWrite {
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

impl tabled::Tabled for EdgeConfigWrite {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CronJobInvocation {
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
}

impl std::fmt::Display for CronJobInvocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for CronJobInvocation {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresComputeTime {
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

impl tabled::Tabled for PostgresComputeTime {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresDataStorage {
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

impl tabled::Tabled for PostgresDataStorage {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresDataTransfer {
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

impl tabled::Tabled for PostgresDataTransfer {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostgresWrittenData {
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

impl tabled::Tabled for PostgresWrittenData {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalCommands {
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

impl tabled::Tabled for StorageRedisTotalCommands {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalBandwidthInBytes {
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

impl tabled::Tabled for StorageRedisTotalBandwidthInBytes {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalDailyAvgStorageInBytes {
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

impl tabled::Tabled for StorageRedisTotalDailyAvgStorageInBytes {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct StorageRedisTotalDatabases {
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

impl tabled::Tabled for StorageRedisTotalDatabases {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(tier) = &self.tier {
                format!("{:?}", tier)
            } else {
                String::new()
            },
            format!("{:?}", self.price),
            format!("{:?}", self.batch),
            format!("{:?}", self.threshold),
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            format!("{:?}", self.hidden),
            if let Some(disabled_at) = &self.disabled_at {
                format!("{:?}", disabled_at)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "tier".to_string(),
            "price".to_string(),
            "batch".to_string(),
            "threshold".to_string(),
            "name".to_string(),
            "hidden".to_string(),
            "disabled_at".to_string(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct InvoiceItems {
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pro: Option<Pro>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enterprise: Option<Enterprise>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "concurrentBuilds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub concurrent_builds: Option<ConcurrentBuilds>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saml: Option<InvoiceItemsSaml>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(rename = "teamSeats", default, skip_serializing_if = "Option::is_none")]
    pub team_seats: Option<TeamSeats>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "customCerts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_certs: Option<CustomCerts>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "previewDeploymentSuffix",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preview_deployment_suffix: Option<PreviewDeploymentSuffix>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "passwordProtection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub password_protection: Option<PasswordProtection>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "ssoProtection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub sso_protection: Option<SsoProtection>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics: Option<Analytics>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monitoring: Option<Monitoring>,
    #[doc = "Will be used to create an invoice item. The price must be in cents: 2000 for $20."]
    #[serde(
        rename = "webAnalytics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_analytics: Option<WebAnalytics>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builds: Option<Builds>,
    #[serde(
        rename = "edgeMiddlewareInvocations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_middleware_invocations: Option<EdgeMiddlewareInvocations>,
    #[serde(
        rename = "edgeFunctionExecutionUnits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub edge_function_execution_units: Option<EdgeFunctionExecutionUnits>,
    #[serde(
        rename = "monitoringMetric",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub monitoring_metric: Option<MonitoringMetric>,
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
        rename = "webAnalyticsEvent",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_analytics_event: Option<WebAnalyticsEvent>,
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
        rename = "cronJobInvocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cron_job_invocation: Option<CronJobInvocation>,
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
        rename = "postgresWrittenData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_written_data: Option<PostgresWrittenData>,
    #[serde(
        rename = "storageRedisTotalCommands",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_redis_total_commands: Option<StorageRedisTotalCommands>,
    #[serde(
        rename = "storageRedisTotalBandwidthInBytes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub storage_redis_total_bandwidth_in_bytes: Option<StorageRedisTotalBandwidthInBytes>,
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

impl tabled::Tabled for InvoiceItems {
    const LENGTH: usize = 33;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(pro) = &self.pro {
                format!("{:?}", pro)
            } else {
                String::new()
            },
            if let Some(enterprise) = &self.enterprise {
                format!("{:?}", enterprise)
            } else {
                String::new()
            },
            if let Some(concurrent_builds) = &self.concurrent_builds {
                format!("{:?}", concurrent_builds)
            } else {
                String::new()
            },
            if let Some(saml) = &self.saml {
                format!("{:?}", saml)
            } else {
                String::new()
            },
            if let Some(team_seats) = &self.team_seats {
                format!("{:?}", team_seats)
            } else {
                String::new()
            },
            if let Some(custom_certs) = &self.custom_certs {
                format!("{:?}", custom_certs)
            } else {
                String::new()
            },
            if let Some(preview_deployment_suffix) = &self.preview_deployment_suffix {
                format!("{:?}", preview_deployment_suffix)
            } else {
                String::new()
            },
            if let Some(password_protection) = &self.password_protection {
                format!("{:?}", password_protection)
            } else {
                String::new()
            },
            if let Some(sso_protection) = &self.sso_protection {
                format!("{:?}", sso_protection)
            } else {
                String::new()
            },
            if let Some(analytics) = &self.analytics {
                format!("{:?}", analytics)
            } else {
                String::new()
            },
            if let Some(monitoring) = &self.monitoring {
                format!("{:?}", monitoring)
            } else {
                String::new()
            },
            if let Some(web_analytics) = &self.web_analytics {
                format!("{:?}", web_analytics)
            } else {
                String::new()
            },
            if let Some(analytics_usage) = &self.analytics_usage {
                format!("{:?}", analytics_usage)
            } else {
                String::new()
            },
            if let Some(artifacts) = &self.artifacts {
                format!("{:?}", artifacts)
            } else {
                String::new()
            },
            if let Some(bandwidth) = &self.bandwidth {
                format!("{:?}", bandwidth)
            } else {
                String::new()
            },
            if let Some(builds) = &self.builds {
                format!("{:?}", builds)
            } else {
                String::new()
            },
            if let Some(edge_middleware_invocations) = &self.edge_middleware_invocations {
                format!("{:?}", edge_middleware_invocations)
            } else {
                String::new()
            },
            if let Some(edge_function_execution_units) = &self.edge_function_execution_units {
                format!("{:?}", edge_function_execution_units)
            } else {
                String::new()
            },
            if let Some(monitoring_metric) = &self.monitoring_metric {
                format!("{:?}", monitoring_metric)
            } else {
                String::new()
            },
            if let Some(serverless_function_execution) = &self.serverless_function_execution {
                format!("{:?}", serverless_function_execution)
            } else {
                String::new()
            },
            if let Some(source_images) = &self.source_images {
                format!("{:?}", source_images)
            } else {
                String::new()
            },
            if let Some(web_analytics_event) = &self.web_analytics_event {
                format!("{:?}", web_analytics_event)
            } else {
                String::new()
            },
            if let Some(edge_config_read) = &self.edge_config_read {
                format!("{:?}", edge_config_read)
            } else {
                String::new()
            },
            if let Some(edge_config_write) = &self.edge_config_write {
                format!("{:?}", edge_config_write)
            } else {
                String::new()
            },
            if let Some(cron_job_invocation) = &self.cron_job_invocation {
                format!("{:?}", cron_job_invocation)
            } else {
                String::new()
            },
            if let Some(postgres_compute_time) = &self.postgres_compute_time {
                format!("{:?}", postgres_compute_time)
            } else {
                String::new()
            },
            if let Some(postgres_data_storage) = &self.postgres_data_storage {
                format!("{:?}", postgres_data_storage)
            } else {
                String::new()
            },
            if let Some(postgres_data_transfer) = &self.postgres_data_transfer {
                format!("{:?}", postgres_data_transfer)
            } else {
                String::new()
            },
            if let Some(postgres_written_data) = &self.postgres_written_data {
                format!("{:?}", postgres_written_data)
            } else {
                String::new()
            },
            if let Some(storage_redis_total_commands) = &self.storage_redis_total_commands {
                format!("{:?}", storage_redis_total_commands)
            } else {
                String::new()
            },
            if let Some(storage_redis_total_bandwidth_in_bytes) =
                &self.storage_redis_total_bandwidth_in_bytes
            {
                format!("{:?}", storage_redis_total_bandwidth_in_bytes)
            } else {
                String::new()
            },
            if let Some(storage_redis_total_daily_avg_storage_in_bytes) =
                &self.storage_redis_total_daily_avg_storage_in_bytes
            {
                format!("{:?}", storage_redis_total_daily_avg_storage_in_bytes)
            } else {
                String::new()
            },
            if let Some(storage_redis_total_databases) = &self.storage_redis_total_databases {
                format!("{:?}", storage_redis_total_databases)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "pro".to_string(),
            "enterprise".to_string(),
            "concurrent_builds".to_string(),
            "saml".to_string(),
            "team_seats".to_string(),
            "custom_certs".to_string(),
            "preview_deployment_suffix".to_string(),
            "password_protection".to_string(),
            "sso_protection".to_string(),
            "analytics".to_string(),
            "monitoring".to_string(),
            "web_analytics".to_string(),
            "analytics_usage".to_string(),
            "artifacts".to_string(),
            "bandwidth".to_string(),
            "builds".to_string(),
            "edge_middleware_invocations".to_string(),
            "edge_function_execution_units".to_string(),
            "monitoring_metric".to_string(),
            "serverless_function_execution".to_string(),
            "source_images".to_string(),
            "web_analytics_event".to_string(),
            "edge_config_read".to_string(),
            "edge_config_write".to_string(),
            "cron_job_invocation".to_string(),
            "postgres_compute_time".to_string(),
            "postgres_data_storage".to_string(),
            "postgres_data_transfer".to_string(),
            "postgres_written_data".to_string(),
            "storage_redis_total_commands".to_string(),
            "storage_redis_total_bandwidth_in_bytes".to_string(),
            "storage_redis_total_daily_avg_storage_in_bytes".to_string(),
            "storage_redis_total_databases".to_string(),
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

impl tabled::Tabled for InvoiceSettings {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(footer) = &self.footer {
            format!("{:?}", footer)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["footer".to_string()]
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

impl tabled::Tabled for SubscriptionsPeriod {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![format!("{:?}", self.start), format!("{:?}", self.end)]
    }

    fn headers() -> Vec<String> {
        vec!["start".to_string(), "end".to_string()]
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

impl tabled::Tabled for SubscriptionsFrequency {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.interval),
            format!("{:?}", self.interval_count),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["interval".to_string(), "interval_count".to_string()]
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

impl tabled::Tabled for Coupon {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            format!("{:?}", self.name),
            format!("{:?}", self.amount_off),
            format!("{:?}", self.percentage_off),
            format!("{:?}", self.duration_in_months),
            format!("{:?}", self.duration),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "name".to_string(),
            "amount_off".to_string(),
            "percentage_off".to_string(),
            "duration_in_months".to_string(),
            "duration".to_string(),
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

impl tabled::Tabled for Discount {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.id.clone(), format!("{:?}", self.coupon)]
    }

    fn headers() -> Vec<String> {
        vec!["id".to_string(), "coupon".to_string()]
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

impl tabled::Tabled for Items {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.price_id.clone(),
            self.product_id.clone(),
            format!("{:?}", self.amount),
            format!("{:?}", self.quantity),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "price_id".to_string(),
            "product_id".to_string(),
            "amount".to_string(),
            "quantity".to_string(),
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

impl tabled::Tabled for Subscriptions {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            format!("{:?}", self.trial),
            format!("{:?}", self.period),
            format!("{:?}", self.frequency),
            format!("{:?}", self.discount),
            format!("{:?}", self.items),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "trial".to_string(),
            "period".to_string(),
            "frequency".to_string(),
            "discount".to_string(),
            "items".to_string(),
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

impl tabled::Tabled for Controls {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(analytics_sample_rate_in_percent) = &self.analytics_sample_rate_in_percent {
                format!("{:?}", analytics_sample_rate_in_percent)
            } else {
                String::new()
            },
            if let Some(analytics_spend_limit_in_dollars) = &self.analytics_spend_limit_in_dollars {
                format!("{:?}", analytics_spend_limit_in_dollars)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "analytics_sample_rate_in_percent".to_string(),
            "analytics_spend_limit_in_dollars".to_string(),
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
pub enum Status {
    #[serde(rename = "active")]
    #[display("active")]
    Active,
    #[serde(rename = "canceled")]
    #[display("canceled")]
    Canceled,
    #[serde(rename = "trialing")]
    #[display("trialing")]
    Trialing,
    #[serde(rename = "overdue")]
    #[display("overdue")]
    Overdue,
    #[serde(rename = "expired")]
    #[display("expired")]
    Expired,
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
#[derive(Default)]
pub enum PricingExperiment {
    #[serde(rename = "august-2022")]
    #[display("august-2022")]
    #[default]
    August2022,
}



#[doc = "An object containing billing infomation associated with the User account."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Billing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addons: Option<Vec<Addons>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract: Option<Contract>,
    pub plan: Plan,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,
    #[serde(
        rename = "orbCustomerId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub orb_customer_id: Option<String>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overdue: Option<bool>,
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

impl tabled::Tabled for Billing {
    const LENGTH: usize = 23;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(currency) = &self.currency {
                format!("{:?}", currency)
            } else {
                String::new()
            },
            if let Some(addons) = &self.addons {
                format!("{:?}", addons)
            } else {
                String::new()
            },
            if let Some(cancelation) = &self.cancelation {
                format!("{:?}", cancelation)
            } else {
                String::new()
            },
            format!("{:?}", self.period),
            if let Some(contract) = &self.contract {
                format!("{:?}", contract)
            } else {
                String::new()
            },
            format!("{:?}", self.plan),
            if let Some(platform) = &self.platform {
                format!("{:?}", platform)
            } else {
                String::new()
            },
            if let Some(orb_customer_id) = &self.orb_customer_id {
                format!("{:?}", orb_customer_id)
            } else {
                String::new()
            },
            if let Some(program_type) = &self.program_type {
                format!("{:?}", program_type)
            } else {
                String::new()
            },
            if let Some(trial) = &self.trial {
                format!("{:?}", trial)
            } else {
                String::new()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email)
            } else {
                String::new()
            },
            if let Some(tax) = &self.tax {
                format!("{:?}", tax)
            } else {
                String::new()
            },
            if let Some(language) = &self.language {
                format!("{:?}", language)
            } else {
                String::new()
            },
            if let Some(address) = &self.address {
                format!("{:?}", address)
            } else {
                String::new()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name)
            } else {
                String::new()
            },
            if let Some(overdue) = &self.overdue {
                format!("{:?}", overdue)
            } else {
                String::new()
            },
            if let Some(invoice_items) = &self.invoice_items {
                format!("{:?}", invoice_items)
            } else {
                String::new()
            },
            if let Some(invoice_settings) = &self.invoice_settings {
                format!("{:?}", invoice_settings)
            } else {
                String::new()
            },
            if let Some(subscriptions) = &self.subscriptions {
                format!("{:?}", subscriptions)
            } else {
                String::new()
            },
            if let Some(controls) = &self.controls {
                format!("{:?}", controls)
            } else {
                String::new()
            },
            if let Some(purchase_order) = &self.purchase_order {
                format!("{:?}", purchase_order)
            } else {
                String::new()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status)
            } else {
                String::new()
            },
            if let Some(pricing_experiment) = &self.pricing_experiment {
                format!("{:?}", pricing_experiment)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "currency".to_string(),
            "addons".to_string(),
            "cancelation".to_string(),
            "period".to_string(),
            "contract".to_string(),
            "plan".to_string(),
            "platform".to_string(),
            "orb_customer_id".to_string(),
            "program_type".to_string(),
            "trial".to_string(),
            "email".to_string(),
            "tax".to_string(),
            "language".to_string(),
            "address".to_string(),
            "name".to_string(),
            "overdue".to_string(),
            "invoice_items".to_string(),
            "invoice_settings".to_string(),
            "subscriptions".to_string(),
            "controls".to_string(),
            "purchase_order".to_string(),
            "status".to_string(),
            "pricing_experiment".to_string(),
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
        rename = "blobStores",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub blob_stores: Option<f64>,
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

impl tabled::Tabled for ResourceConfig {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<String> {
        vec![
            if let Some(node_type) = &self.node_type {
                format!("{:?}", node_type)
            } else {
                String::new()
            },
            if let Some(concurrent_builds) = &self.concurrent_builds {
                format!("{:?}", concurrent_builds)
            } else {
                String::new()
            },
            if let Some(aws_account_type) = &self.aws_account_type {
                format!("{:?}", aws_account_type)
            } else {
                String::new()
            },
            if let Some(aws_account_ids) = &self.aws_account_ids {
                format!("{:?}", aws_account_ids)
            } else {
                String::new()
            },
            if let Some(cf_zone_name) = &self.cf_zone_name {
                format!("{:?}", cf_zone_name)
            } else {
                String::new()
            },
            if let Some(edge_configs) = &self.edge_configs {
                format!("{:?}", edge_configs)
            } else {
                String::new()
            },
            if let Some(edge_config_size) = &self.edge_config_size {
                format!("{:?}", edge_config_size)
            } else {
                String::new()
            },
            if let Some(edge_function_max_size_bytes) = &self.edge_function_max_size_bytes {
                format!("{:?}", edge_function_max_size_bytes)
            } else {
                String::new()
            },
            if let Some(edge_function_execution_timeout_ms) =
                &self.edge_function_execution_timeout_ms
            {
                format!("{:?}", edge_function_execution_timeout_ms)
            } else {
                String::new()
            },
            if let Some(kv_databases) = &self.kv_databases {
                format!("{:?}", kv_databases)
            } else {
                String::new()
            },
            if let Some(postgres_databases) = &self.postgres_databases {
                format!("{:?}", postgres_databases)
            } else {
                String::new()
            },
            if let Some(blob_stores) = &self.blob_stores {
                format!("{:?}", blob_stores)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "node_type".to_string(),
            "concurrent_builds".to_string(),
            "aws_account_type".to_string(),
            "aws_account_ids".to_string(),
            "cf_zone_name".to_string(),
            "edge_configs".to_string(),
            "edge_config_size".to_string(),
            "edge_function_max_size_bytes".to_string(),
            "edge_function_execution_timeout_ms".to_string(),
            "kv_databases".to_string(),
            "postgres_databases".to_string(),
            "blob_stores".to_string(),
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
pub enum ViewPreference {
    #[serde(rename = "list")]
    #[display("list")]
    List,
    #[serde(rename = "cards")]
    #[display("cards")]
    Cards,
}

#[doc = "set of dashboard view preferences (cards or list) per scopeId"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ActiveDashboardViews {
    #[serde(rename = "scopeId")]
    pub scope_id: String,
    #[serde(rename = "viewPreference")]
    pub view_preference: ViewPreference,
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

impl tabled::Tabled for ActiveDashboardViews {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.scope_id.clone(), format!("{:?}", self.view_preference)]
    }

    fn headers() -> Vec<String> {
        vec!["scope_id".to_string(), "view_preference".to_string()]
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
pub enum ImportFlowGitNamespace {
    String(String),
    F64(f64),
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
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
    serde :: Serialize,
    serde :: Deserialize,
    PartialEq,
    Debug,
    Clone,
    schemars :: JsonSchema,
    tabled :: Tabled,
)]
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

impl tabled::Tabled for PreferredScopesAndGitNamespaces {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![
            self.scope_id.clone(),
            format!("{:?}", self.git_namespace_id),
        ]
    }

    fn headers() -> Vec<String> {
        vec!["scope_id".to_string(), "git_namespace_id".to_string()]
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

impl tabled::Tabled for Dismissals {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.scope_id.clone(), format!("{:?}", self.created_at)]
    }

    fn headers() -> Vec<String> {
        vec!["scope_id".to_string(), "created_at".to_string()]
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

impl tabled::Tabled for DismissedToasts {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<String> {
        vec![self.name.clone(), format!("{:?}", self.dismissals)]
    }

    fn headers() -> Vec<String> {
        vec!["name".to_string(), "dismissals".to_string()]
    }
}

#[doc = "A list of projects and spaces across teams that a user has marked as a favorite."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FavoriteProjectsAndSpacesOneOf {
    #[serde(rename = "projectId")]
    pub project_id: String,
    #[serde(rename = "scopeSlug")]
    pub scope_slug: String,
    #[serde(rename = "scopeId")]
    pub scope_id: String,
}

impl std::fmt::Display for FavoriteProjectsAndSpacesOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for FavoriteProjectsAndSpacesOneOf {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            self.project_id.clone(),
            self.scope_slug.clone(),
            self.scope_id.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "project_id".to_string(),
            "scope_slug".to_string(),
            "scope_id".to_string(),
        ]
    }
}

#[doc = "A list of projects and spaces across teams that a user has marked as a favorite."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FavoriteProjectsAndSpacesOneOfOneOf {
    #[serde(rename = "spaceId")]
    pub space_id: String,
    #[serde(rename = "scopeSlug")]
    pub scope_slug: String,
    #[serde(rename = "scopeId")]
    pub scope_id: String,
}

impl std::fmt::Display for FavoriteProjectsAndSpacesOneOfOneOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl tabled::Tabled for FavoriteProjectsAndSpacesOneOfOneOf {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<String> {
        vec![
            self.space_id.clone(),
            self.scope_slug.clone(),
            self.scope_id.clone(),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "space_id".to_string(),
            "scope_slug".to_string(),
            "scope_id".to_string(),
        ]
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
pub enum FavoriteProjectsAndSpaces {
    FavoriteProjectsAndSpacesOneOf(FavoriteProjectsAndSpacesOneOf),
    FavoriteProjectsAndSpacesOneOfOneOf(FavoriteProjectsAndSpacesOneOfOneOf),
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

impl tabled::Tabled for RemoteCaching {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<String> {
        vec![if let Some(enabled) = &self.enabled {
            format!("{:?}", enabled)
        } else {
            String::new()
        }]
    }

    fn headers() -> Vec<String> {
        vec!["enabled".to_string()]
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

impl tabled::Tabled for AuthUser {
    const LENGTH: usize = 19;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.created_at),
            format!("{:?}", self.soft_block),
            format!("{:?}", self.billing),
            format!("{:?}", self.resource_config),
            self.staging_prefix.clone(),
            if let Some(active_dashboard_views) = &self.active_dashboard_views {
                format!("{:?}", active_dashboard_views)
            } else {
                String::new()
            },
            if let Some(import_flow_git_namespace) = &self.import_flow_git_namespace {
                format!("{:?}", import_flow_git_namespace)
            } else {
                String::new()
            },
            if let Some(import_flow_git_namespace_id) = &self.import_flow_git_namespace_id {
                format!("{:?}", import_flow_git_namespace_id)
            } else {
                String::new()
            },
            if let Some(import_flow_git_provider) = &self.import_flow_git_provider {
                format!("{:?}", import_flow_git_provider)
            } else {
                String::new()
            },
            if let Some(preferred_scopes_and_git_namespaces) =
                &self.preferred_scopes_and_git_namespaces
            {
                format!("{:?}", preferred_scopes_and_git_namespaces)
            } else {
                String::new()
            },
            if let Some(dismissed_toasts) = &self.dismissed_toasts {
                format!("{:?}", dismissed_toasts)
            } else {
                String::new()
            },
            if let Some(favorite_projects_and_spaces) = &self.favorite_projects_and_spaces {
                format!("{:?}", favorite_projects_and_spaces)
            } else {
                String::new()
            },
            format!("{:?}", self.has_trial_available),
            if let Some(remote_caching) = &self.remote_caching {
                format!("{:?}", remote_caching)
            } else {
                String::new()
            },
            self.id.clone(),
            self.email.clone(),
            format!("{:?}", self.name),
            self.username.clone(),
            format!("{:?}", self.avatar),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "created_at".to_string(),
            "soft_block".to_string(),
            "billing".to_string(),
            "resource_config".to_string(),
            "staging_prefix".to_string(),
            "active_dashboard_views".to_string(),
            "import_flow_git_namespace".to_string(),
            "import_flow_git_namespace_id".to_string(),
            "import_flow_git_provider".to_string(),
            "preferred_scopes_and_git_namespaces".to_string(),
            "dismissed_toasts".to_string(),
            "favorite_projects_and_spaces".to_string(),
            "has_trial_available".to_string(),
            "remote_caching".to_string(),
            "id".to_string(),
            "email".to_string(),
            "name".to_string(),
            "username".to_string(),
            "avatar".to_string(),
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

impl tabled::Tabled for AuthUserLimited {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<String> {
        vec![
            format!("{:?}", self.limited),
            self.id.clone(),
            self.email.clone(),
            format!("{:?}", self.name),
            self.username.clone(),
            format!("{:?}", self.avatar),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "limited".to_string(),
            "id".to_string(),
            "email".to_string(),
            "name".to_string(),
            "username".to_string(),
            "avatar".to_string(),
        ]
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
#[serde(tag = "type")]
pub enum Scopes {
    #[serde(rename = "user")]
    User {
        origin: Origin,
        createdAt: f64,
        expiresAt: Option<f64>,
    },
    #[serde(rename = "team")]
    Team {
        teamId: String,
        origin: Origin,
        createdAt: f64,
        expiresAt: Option<f64>,
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

impl tabled::Tabled for AuthToken {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<String> {
        vec![
            self.id.clone(),
            self.name.clone(),
            self.type_.clone(),
            if let Some(origin) = &self.origin {
                format!("{:?}", origin)
            } else {
                String::new()
            },
            if let Some(scopes) = &self.scopes {
                format!("{:?}", scopes)
            } else {
                String::new()
            },
            if let Some(expires_at) = &self.expires_at {
                format!("{:?}", expires_at)
            } else {
                String::new()
            },
            format!("{:?}", self.active_at),
            format!("{:?}", self.created_at),
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "id".to_string(),
            "name".to_string(),
            "type_".to_string(),
            "origin".to_string(),
            "scopes".to_string(),
            "expires_at".to_string(),
            "active_at".to_string(),
            "created_at".to_string(),
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
    tabled :: Tabled,
    clap :: ValueEnum,
    parse_display :: FromStr,
    parse_display :: Display,
)]
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

impl tabled::Tabled for FileTree {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            format!("{:?}", self.type_),
            if let Some(uid) = &self.uid {
                format!("{:?}", uid)
            } else {
                String::new()
            },
            if let Some(children) = &self.children {
                format!("{:?}", children)
            } else {
                String::new()
            },
            if let Some(content_type) = &self.content_type {
                format!("{:?}", content_type)
            } else {
                String::new()
            },
            format!("{:?}", self.mode),
            if let Some(symlink) = &self.symlink {
                format!("{:?}", symlink)
            } else {
                String::new()
            },
        ]
    }

    fn headers() -> Vec<String> {
        vec![
            "name".to_string(),
            "type_".to_string(),
            "uid".to_string(),
            "children".to_string(),
            "content_type".to_string(),
            "mode".to_string(),
            "symlink".to_string(),
        ]
    }
}
