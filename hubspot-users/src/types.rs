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

#[doc = "A user"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicUser {
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The user's primary team"]
    #[serde(
        rename = "primaryTeamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_team_id: Option<String>,
    #[serde(rename = "roleIds", default, skip_serializing_if = "Option::is_none")]
    pub role_ids: Option<Vec<String>>,
    #[serde(
        rename = "sendWelcomeEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_welcome_email: Option<bool>,
    #[doc = "The user's role"]
    #[serde(rename = "roleId", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[doc = "The user's additional teams"]
    #[serde(
        rename = "secondaryTeamIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_team_ids: Option<Vec<String>>,
    #[doc = "The user's unique ID"]
    pub id: String,
    #[serde(
        rename = "superAdmin",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub super_admin: Option<bool>,
    #[doc = "The user's email"]
    pub email: String,
}

impl std::fmt::Display for PublicUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicUser {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(primary_team_id) = &self.primary_team_id {
                format!("{:?}", primary_team_id).into()
            } else {
                String::new().into()
            },
            if let Some(role_ids) = &self.role_ids {
                format!("{:?}", role_ids).into()
            } else {
                String::new().into()
            },
            if let Some(send_welcome_email) = &self.send_welcome_email {
                format!("{:?}", send_welcome_email).into()
            } else {
                String::new().into()
            },
            if let Some(role_id) = &self.role_id {
                format!("{:?}", role_id).into()
            } else {
                String::new().into()
            },
            if let Some(secondary_team_ids) = &self.secondary_team_ids {
                format!("{:?}", secondary_team_ids).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            if let Some(super_admin) = &self.super_admin {
                format!("{:?}", super_admin).into()
            } else {
                String::new().into()
            },
            self.email.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "last_name".into(),
            "primary_team_id".into(),
            "role_ids".into(),
            "send_welcome_email".into(),
            "role_id".into(),
            "secondary_team_ids".into(),
            "id".into(),
            "super_admin".into(),
            "email".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CollectionResponsePublicPermissionSetNoPaging {
    pub results: Vec<PublicPermissionSet>,
}

impl std::fmt::Display for CollectionResponsePublicPermissionSetNoPaging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CollectionResponsePublicPermissionSetNoPaging {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.results).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into()]
    }
}

#[doc = "A team that can be assigned to a user"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicTeam {
    #[doc = "Primary members of this team"]
    #[serde(rename = "userIds")]
    pub user_ids: Vec<String>,
    #[doc = "The team's name"]
    pub name: String,
    #[doc = "The team's unique ID"]
    pub id: String,
    #[doc = "Secondary or additional members of this team"]
    #[serde(rename = "secondaryUserIds")]
    pub secondary_user_ids: Vec<String>,
}

impl std::fmt::Display for PublicTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicTeam {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user_ids).into(),
            self.name.clone().into(),
            self.id.clone().into(),
            format!("{:?}", self.secondary_user_ids).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "user_ids".into(),
            "name".into(),
            "id".into(),
            "secondary_user_ids".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CollectionResponsePublicTeamNoPaging {
    pub results: Vec<PublicTeam>,
}

impl std::fmt::Display for CollectionResponsePublicTeamNoPaging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CollectionResponsePublicTeamNoPaging {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.results).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CollectionResponsePublicUserForwardPaging {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paging: Option<ForwardPaging>,
    pub results: Vec<PublicUser>,
}

impl std::fmt::Display for CollectionResponsePublicUserForwardPaging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CollectionResponsePublicUserForwardPaging {
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

#[doc = "A user creation request"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserProvisionRequest {
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The user's primary team"]
    #[serde(
        rename = "primaryTeamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_team_id: Option<String>,
    #[doc = "Whether to send a welcome email"]
    #[serde(
        rename = "sendWelcomeEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub send_welcome_email: Option<bool>,
    #[doc = "The user's role"]
    #[serde(rename = "roleId", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[doc = "The user's additional teams"]
    #[serde(
        rename = "secondaryTeamIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_team_ids: Option<Vec<String>>,
    #[doc = "The created user's email"]
    pub email: String,
}

impl std::fmt::Display for UserProvisionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserProvisionRequest {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(primary_team_id) = &self.primary_team_id {
                format!("{:?}", primary_team_id).into()
            } else {
                String::new().into()
            },
            if let Some(send_welcome_email) = &self.send_welcome_email {
                format!("{:?}", send_welcome_email).into()
            } else {
                String::new().into()
            },
            if let Some(role_id) = &self.role_id {
                format!("{:?}", role_id).into()
            } else {
                String::new().into()
            },
            if let Some(secondary_team_ids) = &self.secondary_team_ids {
                format!("{:?}", secondary_team_ids).into()
            } else {
                String::new().into()
            },
            self.email.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "last_name".into(),
            "primary_team_id".into(),
            "send_welcome_email".into(),
            "role_id".into(),
            "secondary_team_ids".into(),
            "email".into(),
        ]
    }
}

#[doc = "A user to update"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicUserUpdate {
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The user's primary team"]
    #[serde(
        rename = "primaryTeamId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_team_id: Option<String>,
    #[doc = "The user's role"]
    #[serde(rename = "roleId", default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    #[doc = "The user's additional teams"]
    #[serde(
        rename = "secondaryTeamIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub secondary_team_ids: Option<Vec<String>>,
}

impl std::fmt::Display for PublicUserUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicUserUpdate {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(primary_team_id) = &self.primary_team_id {
                format!("{:?}", primary_team_id).into()
            } else {
                String::new().into()
            },
            if let Some(role_id) = &self.role_id {
                format!("{:?}", role_id).into()
            } else {
                String::new().into()
            },
            if let Some(secondary_team_ids) = &self.secondary_team_ids {
                format!("{:?}", secondary_team_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "last_name".into(),
            "primary_team_id".into(),
            "role_id".into(),
            "secondary_team_ids".into(),
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

#[doc = "A role that can be assigned to a user"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PublicPermissionSet {
    #[doc = "Whether this role has a paid seat and requires the billing-write scope to \
             assign/unassign to users"]
    #[serde(rename = "requiresBillingWrite")]
    pub requires_billing_write: bool,
    #[doc = "The role's name"]
    pub name: String,
    #[doc = "The role's unique ID"]
    pub id: String,
}

impl std::fmt::Display for PublicPermissionSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PublicPermissionSet {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.requires_billing_write).into(),
            self.name.clone().into(),
            self.id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["requires_billing_write".into(), "name".into(), "id".into()]
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
pub enum IdProperty {
    #[serde(rename = "USER_ID")]
    #[display("USER_ID")]
    UserId,
    #[serde(rename = "EMAIL")]
    #[display("EMAIL")]
    Email,
}
