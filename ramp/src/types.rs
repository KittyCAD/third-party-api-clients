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
pub struct AccountingProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_provider_name: Option<serde_json::Value>,
}

impl std::fmt::Display for AccountingProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AccountingProvider {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(remote_provider_name) = &self.remote_provider_name {
                format!("{:?}", remote_provider_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "id".into(),
            "remote_provider_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingCategory {
    #[doc = "User-selected category id for transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    #[doc = "User-selected category name for transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_category_remote_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_category_remote_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tracking_category_remote_type: Option<String>,
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
            if let Some(category_id) = &self.category_id {
                format!("{:?}", category_id).into()
            } else {
                String::new().into()
            },
            if let Some(category_name) = &self.category_name {
                format!("{:?}", category_name).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_category_remote_id) = &self.tracking_category_remote_id {
                format!("{:?}", tracking_category_remote_id).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_category_remote_name) = &self.tracking_category_remote_name {
                format!("{:?}", tracking_category_remote_name).into()
            } else {
                String::new().into()
            },
            if let Some(tracking_category_remote_type) = &self.tracking_category_remote_type {
                format!("{:?}", tracking_category_remote_type).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "category_id".into(),
            "category_name".into(),
            "tracking_category_remote_id".into(),
            "tracking_category_remote_name".into(),
            "tracking_category_remote_type".into(),
        ]
    }
}

#[doc = "The input type could be SINGLE_CHOICE, BOOLEAN or FREE_FORM_TEXT."]
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
pub enum InputType {
    #[serde(rename = "BOOLEAN")]
    #[display("BOOLEAN")]
    Boolean,
    #[serde(rename = "FREE_FORM_TEXT")]
    #[display("FREE_FORM_TEXT")]
    FreeFormText,
    #[serde(rename = "SINGLE_CHOICE")]
    #[display("SINGLE_CHOICE")]
    SingleChoice,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingCustomFieldCreateRequestBody {
    #[doc = "id of the custom accounting field."]
    pub id: String,
    #[doc = "The input type could be SINGLE_CHOICE, BOOLEAN or FREE_FORM_TEXT."]
    pub input_type: InputType,
    #[doc = "If set to True, the accounting field can be used to annotate split line items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_splittable: Option<bool>,
    #[doc = "name of the custom accounting field."]
    pub name: String,
}

impl std::fmt::Display for ApiAccountingCustomFieldCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingCustomFieldCreateRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            format!("{:?}", self.input_type).into(),
            if let Some(is_splittable) = &self.is_splittable {
                format!("{:?}", is_splittable).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "input_type".into(),
            "is_splittable".into(),
            "name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingCustomFieldOptionUpdateRequestBody {
    #[doc = "reactivate a deleted custom field option"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactivate: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for ApiAccountingCustomFieldOptionUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingCustomFieldOptionUpdateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(reactivate) = &self.reactivate {
                format!("{:?}", reactivate).into()
            } else {
                String::new().into()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["reactivate".into(), "value".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingCustomFieldOptionUploadRequestBody {
    #[doc = "id to uniquely identify a custom accounting field within Ramp system"]
    pub field_id: uuid::Uuid,
    #[doc = "A list of field options for a given custom accounting field."]
    pub options: Vec<FieldOption>,
}

impl std::fmt::Display for ApiAccountingCustomFieldOptionUploadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingCustomFieldOptionUploadRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.field_id).into(),
            format!("{:?}", self.options).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["field_id".into(), "options".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingCustomFieldResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The input type could be SINGLE_CHOICE, BOOLEAN or FREE_FORM_TEXT."]
    pub input_type: InputType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_splittable: Option<bool>,
    #[doc = "name of the custom accounting field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "id to uniquely identify a custom accounting field within Ramp system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ramp_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for ApiAccountingCustomFieldResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingCustomFieldResource {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.input_type).into(),
            if let Some(is_active) = &self.is_active {
                format!("{:?}", is_active).into()
            } else {
                String::new().into()
            },
            if let Some(is_splittable) = &self.is_splittable {
                format!("{:?}", is_splittable).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(ramp_id) = &self.ramp_id {
                format!("{:?}", ramp_id).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "id".into(),
            "input_type".into(),
            "is_active".into(),
            "is_splittable".into(),
            "name".into(),
            "ramp_id".into(),
            "updated_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingCustomFieldUpdateRequestBody {
    #[doc = "If set to True, the accounting field can be used to annotate split line items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_splittable: Option<bool>,
    #[doc = "name of the custom accounting field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for ApiAccountingCustomFieldUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingCustomFieldUpdateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(is_splittable) = &self.is_splittable {
                format!("{:?}", is_splittable).into()
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
        vec!["is_splittable".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingFailedSyncRequestBody {
    #[doc = "describes the reason why the sync object failed to sync."]
    pub error: ApiAccountingSyncErrorRequestBody,
    #[doc = "ID that uniquely identifies a transaction/reimbursement in Ramp systems."]
    pub id: uuid::Uuid,
}

impl std::fmt::Display for ApiAccountingFailedSyncRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingFailedSyncRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.error).into(),
            format!("{:?}", self.id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["error".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingGLAccountUpdateRequestBody {
    #[doc = "code of the general ledger account; you could provide an empty string if you want to \
             reset the remote code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "name of the general ledger account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "reactivate a deleted general ledger account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactivate: Option<bool>,
    #[doc = "IDs of a list of subsidiaries which a general ledger account can be used with. The \
             Ramp-assigned IDs should be used here. you could provide an empty list if you want \
             to reset the subsidiaries list for this general ledger account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subsidiaries: Option<Vec<String>>,
}

impl std::fmt::Display for ApiAccountingGLAccountUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingGLAccountUpdateRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(reactivate) = &self.reactivate {
                format!("{:?}", reactivate).into()
            } else {
                String::new().into()
            },
            if let Some(subsidiaries) = &self.subsidiaries {
                format!("{:?}", subsidiaries).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "code".into(),
            "name".into(),
            "reactivate".into(),
            "subsidiaries".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingGLAccountUploadRequestBody {
    #[doc = "A list of general ledger accounts that you want to use to classify your Ramp \
             transactions."]
    pub gl_accounts: Vec<Glaccount>,
}

impl std::fmt::Display for ApiAccountingGLAccountUploadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingGLAccountUploadRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.gl_accounts).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["gl_accounts".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingProviderAccessCreateRequestBody {
    #[doc = "Set reactivate=True to try to find an existing deleted accounting connection instead \
             of creating a new one."]
    #[serde(default)]
    pub reactivate: bool,
    #[doc = "Name of the ERP system that you are using"]
    pub remote_provider_name: String,
}

impl std::fmt::Display for ApiAccountingProviderAccessCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingProviderAccessCreateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.reactivate).into(),
            self.remote_provider_name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["reactivate".into(), "remote_provider_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingSuccessfulSyncRequestBody {
    #[doc = "ID that uniquely identifies the object to sync in Ramp systems."]
    pub id: uuid::Uuid,
    #[doc = "ID that uniquely identifies the object to sync in remote ERP systems."]
    pub reference_id: String,
}

impl std::fmt::Display for ApiAccountingSuccessfulSyncRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingSuccessfulSyncRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.id).into(),
            self.reference_id.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "reference_id".into()]
    }
}

#[doc = "The type of object to sync."]
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
pub enum SyncType {
    #[serde(rename = "BILL_SYNC")]
    #[display("BILL_SYNC")]
    BillSync,
    #[serde(rename = "REIMBURSEMENT_SYNC")]
    #[display("REIMBURSEMENT_SYNC")]
    ReimbursementSync,
    #[serde(rename = "STATEMENT_CREDIT_SYNC")]
    #[display("STATEMENT_CREDIT_SYNC")]
    StatementCreditSync,
    #[serde(rename = "TRANSACTION_SYNC")]
    #[display("TRANSACTION_SYNC")]
    TransactionSync,
    #[serde(rename = "TRANSFER_SYNC")]
    #[display("TRANSFER_SYNC")]
    TransferSync,
    #[serde(rename = "WALLET_TRANSFER_SYNC")]
    #[display("WALLET_TRANSFER_SYNC")]
    WalletTransferSync,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingSyncCreateRequestBody {
    #[doc = "A list of objects that failed to be synced."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_syncs: Option<Vec<ApiAccountingFailedSyncRequestBody>>,
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
    #[doc = "A list of successfully synced objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_syncs: Option<Vec<ApiAccountingSuccessfulSyncRequestBody>>,
    #[doc = "The type of object to sync."]
    pub sync_type: SyncType,
}

impl std::fmt::Display for ApiAccountingSyncCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingSyncCreateRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(failed_syncs) = &self.failed_syncs {
                format!("{:?}", failed_syncs).into()
            } else {
                String::new().into()
            },
            self.idempotency_key.clone().into(),
            if let Some(successful_syncs) = &self.successful_syncs {
                format!("{:?}", successful_syncs).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.sync_type).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "failed_syncs".into(),
            "idempotency_key".into(),
            "successful_syncs".into(),
            "sync_type".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingSyncErrorRequestBody {
    #[doc = "an error message that explains the reason of the sync failure."]
    pub message: String,
}

impl std::fmt::Display for ApiAccountingSyncErrorRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingSyncErrorRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.message.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingTrackingCategoryUploadResponse {
    #[doc = "A list of uuids from the uploaded objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uploaded: Option<Vec<uuid::Uuid>>,
}

impl std::fmt::Display for ApiAccountingTrackingCategoryUploadResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingTrackingCategoryUploadResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(uploaded) = &self.uploaded {
            format!("{:?}", uploaded).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["uploaded".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingVendorUpdateRequestBody {
    #[doc = "Code of the vendor; you could provide an empty string to reset the remote code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Name of a vendor"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "reactivate a deleted vendor"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reactivate: Option<bool>,
    #[doc = "IDs of a list of subsidiaries associated with the vendor. The Ramp-assigned IDs \
             should be used here. You could provide an empty list to reset the subsidiaries list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subsidiaries: Option<Vec<String>>,
}

impl std::fmt::Display for ApiAccountingVendorUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingVendorUpdateRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(reactivate) = &self.reactivate {
                format!("{:?}", reactivate).into()
            } else {
                String::new().into()
            },
            if let Some(subsidiaries) = &self.subsidiaries {
                format!("{:?}", subsidiaries).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "code".into(),
            "name".into(),
            "reactivate".into(),
            "subsidiaries".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiAccountingVendorUploadRequestBody {
    #[doc = "A list of vendors that you want to use to classify your Ramp transactions."]
    pub vendors: Vec<Vendor>,
}

impl std::fmt::Display for ApiAccountingVendorUploadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiAccountingVendorUploadRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.vendors).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["vendors".into()]
    }
}

#[doc = "accounting field type"]
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
    #[serde(rename = "COST_CENTER")]
    #[display("COST_CENTER")]
    CostCenter,
    #[serde(rename = "CUSTOMERS_JOBS")]
    #[display("CUSTOMERS_JOBS")]
    CustomersJobs,
    #[serde(rename = "DEFERRAL_CODE")]
    #[display("DEFERRAL_CODE")]
    DeferralCode,
    #[serde(rename = "EXPENSE_ENTITY")]
    #[display("EXPENSE_ENTITY")]
    ExpenseEntity,
    #[serde(rename = "GL_ACCOUNT")]
    #[display("GL_ACCOUNT")]
    GlAccount,
    #[serde(rename = "INVENTORY_ITEM")]
    #[display("INVENTORY_ITEM")]
    InventoryItem,
    #[serde(rename = "JOURNAL")]
    #[display("JOURNAL")]
    Journal,
    #[serde(rename = "MERCHANT")]
    #[display("MERCHANT")]
    Merchant,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
    #[serde(rename = "PROJECT")]
    #[display("PROJECT")]
    Project,
    #[serde(rename = "REPORTING_TAG")]
    #[display("REPORTING_TAG")]
    ReportingTag,
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
pub struct ApiBillAccountingCategory {
    #[doc = "external id of accounting field; It should uniquely identify an accounting field on \
             the client end."]
    pub external_id: String,
    #[doc = "ID that uniquely identifies an accounting field within Ramp"]
    pub id: String,
    #[doc = "name of accounting field"]
    pub name: String,
    #[doc = "accounting field type"]
    #[serde(rename = "type")]
    pub type_: Type,
}

impl std::fmt::Display for ApiBillAccountingCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiBillAccountingCategory {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.external_id.clone().into(),
            self.id.clone().into(),
            self.name.clone().into(),
            format!("{:?}", self.type_).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "external_id".into(),
            "id".into(),
            "name".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiBillAccountingFieldSelection {
    #[doc = "information about the accounting category (or accounting field)."]
    pub category_info: ApiBillAccountingCategory,
    #[doc = "external id of accounting field option; It should uniquely identify an accounting \
             field option on the client end."]
    pub external_id: String,
    #[doc = "ID that uniquely identifies an accounting field option within Ramp"]
    pub id: String,
    #[doc = "name of accounting field option"]
    pub name: String,
}

impl std::fmt::Display for ApiBillAccountingFieldSelection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiBillAccountingFieldSelection {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.category_info).into(),
            self.external_id.clone().into(),
            self.id.clone().into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "category_info".into(),
            "external_id".into(),
            "id".into(),
            "name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiBillLineItem {
    #[doc = "List of accounting field options selected to code the line item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiBillAccountingFieldSelection>>,
    #[doc = "Amount of the line item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<CurrencyAmount>,
    #[doc = "Memo of the line item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl std::fmt::Display for ApiBillLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiBillLineItem {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.amount).into(),
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_field_selections".into(),
            "amount".into(),
            "memo".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiBillOwner {
    #[doc = "Bill owner's first name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Bill owner's ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Bill owner's last name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl std::fmt::Display for ApiBillOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiBillOwner {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["first_name".into(), "id".into(), "last_name".into()]
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
pub enum PaymentMethod {
    #[serde(rename = "ACH")]
    #[display("ACH")]
    Ach,
    #[serde(rename = "CARD")]
    #[display("CARD")]
    Card,
    #[serde(rename = "CHECK")]
    #[display("CHECK")]
    Check,
    #[serde(rename = "INTERNATIONAL")]
    #[display("INTERNATIONAL")]
    International,
    #[serde(rename = "ONE_TIME_CARD")]
    #[display("ONE_TIME_CARD")]
    OneTimeCard,
    #[serde(rename = "ONE_TIME_CARD_DELIVERY")]
    #[display("ONE_TIME_CARD_DELIVERY")]
    OneTimeCardDelivery,
    #[serde(rename = "PAID_MANUALLY")]
    #[display("PAID_MANUALLY")]
    PaidManually,
    #[serde(rename = "SWIFT")]
    #[display("SWIFT")]
    Swift,
    #[serde(rename = "UNSPECIFIED")]
    #[display("UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "VENDOR_CREDIT")]
    #[display("VENDOR_CREDIT")]
    VendorCredit,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiBillPayment {
    #[doc = "Amount of the bill."]
    pub amount: CurrencyAmount,
    #[doc = "The date the payment is actually initiated"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "The date the payment is scheduled to be initiated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
}

impl std::fmt::Display for ApiBillPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiBillPayment {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.amount).into(),
            if let Some(effective_date) = &self.effective_date {
                format!("{:?}", effective_date).into()
            } else {
                String::new().into()
            },
            if let Some(payment_date) = &self.payment_date {
                format!("{:?}", payment_date).into()
            } else {
                String::new().into()
            },
            if let Some(payment_method) = &self.payment_method {
                format!("{:?}", payment_method).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount".into(),
            "effective_date".into(),
            "payment_date".into(),
            "payment_method".into(),
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
pub enum ApiBillVendorType {
    #[serde(rename = "BUSINESS")]
    #[display("BUSINESS")]
    Business,
    #[serde(rename = "INDIVIDUAL")]
    #[display("INDIVIDUAL")]
    Individual,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiBillVendor {
    #[doc = "ID that uniquely identifies the vendor on the client's side."]
    pub remote_id: String,
    #[doc = "Name of the vendor."]
    pub remote_name: String,
    #[serde(rename = "type")]
    pub type_: ApiBillVendorType,
}

impl std::fmt::Display for ApiBillVendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiBillVendor {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.remote_id.clone().into(),
            self.remote_name.clone().into(),
            format!("{:?}", self.type_).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["remote_id".into(), "remote_name".into(), "type_".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardAccountingRulesDataRequestBody {
    pub tracking_category_id: uuid::Uuid,
    pub tracking_category_option_id: uuid::Uuid,
    pub tracking_category_option_remote_name: String,
}

impl std::fmt::Display for ApiCardAccountingRulesDataRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardAccountingRulesDataRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.tracking_category_id).into(),
            format!("{:?}", self.tracking_category_option_id).into(),
            self.tracking_category_option_remote_name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "tracking_category_id".into(),
            "tracking_category_option_id".into(),
            "tracking_category_option_remote_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardDeferredTaskContext {
    #[doc = "Unique identifier of the acting user that initiated the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acting_user_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the subject card in the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiCardDeferredTaskContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardDeferredTaskContext {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(acting_user_id) = &self.acting_user_id {
                format!("{:?}", acting_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(card_id) = &self.card_id {
                format!("{:?}", card_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["acting_user_id".into(), "card_id".into()]
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
pub struct ApiCardDeferredUpdateRequestBody {
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
}

impl std::fmt::Display for ApiCardDeferredUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardDeferredUpdateRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.idempotency_key.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["idempotency_key".into()]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_personalization: Option<CardPersonalization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_uuid: Option<uuid::Uuid>,
    #[doc = "Fulfillment status of the card"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment_status: Option<FulfillmentStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CardShipping>,
    #[doc = "Date on which the card is shipped out, presented in ISO8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Estimated arrival time, presented in ISO8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_eta: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Tracking url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_url: Option<String>,
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
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(card_personalization) = &self.card_personalization {
                format!("{:?}", card_personalization).into()
            } else {
                String::new().into()
            },
            if let Some(cardholder_uuid) = &self.cardholder_uuid {
                format!("{:?}", cardholder_uuid).into()
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
            if let Some(shipping_date) = &self.shipping_date {
                format!("{:?}", shipping_date).into()
            } else {
                String::new().into()
            },
            if let Some(shipping_eta) = &self.shipping_eta {
                format!("{:?}", shipping_eta).into()
            } else {
                String::new().into()
            },
            if let Some(shipping_tracking_url) = &self.shipping_tracking_url {
                format!("{:?}", shipping_tracking_url).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "card_personalization".into(),
            "cardholder_uuid".into(),
            "fulfillment_status".into(),
            "shipping".into(),
            "shipping_date".into(),
            "shipping_eta".into(),
            "shipping_tracking_url".into(),
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
pub struct ApiCardProgramCreateRequestBody {
    pub acting_user_id: i64,
    pub business_id: i64,
    #[doc = "Description of the card program."]
    pub description: String,
    #[doc = "Display name of the card program."]
    pub display_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Icon>,
    #[doc = "Whether this card program is used as default card program."]
    pub is_default: bool,
    #[doc = "Whether this card program is used for physical cards."]
    pub is_physical: bool,
    pub policy_id: i64,
    #[doc = "Spending restrictions associated with the card program."]
    pub spending_restrictions: ApiCardSpendingRestrictionsRequestBody,
}

impl std::fmt::Display for ApiCardProgramCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardProgramCreateRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.acting_user_id).into(),
            format!("{:?}", self.business_id).into(),
            self.description.clone().into(),
            self.display_name.clone().into(),
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_default).into(),
            format!("{:?}", self.is_physical).into(),
            format!("{:?}", self.policy_id).into(),
            format!("{:?}", self.spending_restrictions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "acting_user_id".into(),
            "business_id".into(),
            "description".into(),
            "display_name".into(),
            "icon".into(),
            "is_default".into(),
            "is_physical".into(),
            "policy_id".into(),
            "spending_restrictions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardProgramResource {
    #[doc = "Card program description."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name of the card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub icon: Icon,
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
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.icon).into(),
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "description".into(),
            "display_name".into(),
            "icon".into(),
            "id".into(),
            "is_default".into(),
            "is_physical".into(),
            "spending_restrictions".into(),
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
pub struct ApiCardProgramSpendingRestrictions {
    #[doc = "Amount limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[doc = "Time interval to apply limit to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    #[doc = "Date to automatically lock the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
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
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount".into(),
            "categories".into(),
            "interval".into(),
            "lock_date".into(),
            "transaction_amount_limit".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardRequestBody {
    #[doc = "Alternative method to create a card using a card program. One of \
             spending_restrictions or card_program_id must be provided. If this value is given, \
             no other attributes (other than idempotency_key) may be given."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_program_id: Option<uuid::Uuid>,
    #[doc = "Cosmetic display name of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Create card associated with business entity. If not provided, defaults to entity \
             associated with user's location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
    #[doc = "Set to true to create a physical card"]
    #[serde(default)]
    pub is_physical: bool,
    #[doc = "Set to true to create a temporary card"]
    #[serde(default)]
    pub is_temporary: bool,
    #[doc = "Specifies the spend restrictions on a Ramp card. One of spending_restrictions or \
             card_program_id must be provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiCardSpendingRestrictionsRequestBody>,
    #[doc = "Unique identifier of the card owner."]
    pub user_id: uuid::Uuid,
}

impl std::fmt::Display for ApiCardRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardRequestBody {
    const LENGTH: usize = 8;
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
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            self.idempotency_key.clone().into(),
            format!("{:?}", self.is_physical).into(),
            format!("{:?}", self.is_temporary).into(),
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "card_program_id".into(),
            "display_name".into(),
            "entity_id".into(),
            "idempotency_key".into(),
            "is_physical".into(),
            "is_temporary".into(),
            "spending_restrictions".into(),
            "user_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardSpendingRestrictionsDump {
    #[doc = "Amount limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "Date to automatically lock the card. Note that this is different from the actual \
             card expiration date. It conforms to ISO8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "List of Ramp Category Codes blocked for this card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<i64>>,
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[doc = "Time interval to apply limit to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    #[doc = "Whether the card has been locked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
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
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(auto_lock_date) = &self.auto_lock_date {
                format!("{:?}", auto_lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_categories) = &self.blocked_categories {
                format!("{:?}", blocked_categories).into()
            } else {
                String::new().into()
            },
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount".into(),
            "auto_lock_date".into(),
            "blocked_categories".into(),
            "categories".into(),
            "interval".into(),
            "suspended".into(),
            "transaction_amount_limit".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardSpendingRestrictionsRequestBody {
    #[doc = "Amount limit total per interval."]
    pub amount: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_mcc_codes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_accounting_rules: Option<Vec<ApiCardAccountingRulesDataRequestBody>>,
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_blacklist: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_whitelist: Option<Vec<i64>>,
    #[doc = "Currency in which the amount is specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Time interval to apply limit to."]
    pub interval: Interval,
    #[doc = "Date to automatically lock the card. If lock date has passed, set to a future date \
             or to null to unlock the card. Note that this is different from the actual card \
             expiration date. This date need to conforms to ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_blacklist: Option<Vec<uuid::Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_whitelist: Option<Vec<uuid::Uuid>>,
}

impl std::fmt::Display for ApiCardSpendingRestrictionsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardSpendingRestrictionsRequestBody {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.amount).into(),
            if let Some(blocked_mcc_codes) = &self.blocked_mcc_codes {
                format!("{:?}", blocked_mcc_codes).into()
            } else {
                String::new().into()
            },
            if let Some(card_accounting_rules) = &self.card_accounting_rules {
                format!("{:?}", card_accounting_rules).into()
            } else {
                String::new().into()
            },
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            if let Some(categories_blacklist) = &self.categories_blacklist {
                format!("{:?}", categories_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(categories_whitelist) = &self.categories_whitelist {
                format!("{:?}", categories_whitelist).into()
            } else {
                String::new().into()
            },
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.interval).into(),
            if let Some(lock_date) = &self.lock_date {
                format!("{:?}", lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(policy_id) = &self.policy_id {
                format!("{:?}", policy_id).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_blacklist) = &self.vendor_blacklist {
                format!("{:?}", vendor_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_whitelist) = &self.vendor_whitelist {
                format!("{:?}", vendor_whitelist).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount".into(),
            "blocked_mcc_codes".into(),
            "card_accounting_rules".into(),
            "categories".into(),
            "categories_blacklist".into(),
            "categories_whitelist".into(),
            "currency".into(),
            "interval".into(),
            "lock_date".into(),
            "policy_id".into(),
            "transaction_amount_limit".into(),
            "vendor_blacklist".into(),
            "vendor_whitelist".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiCardUpdateRequestBody {
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
    #[doc = "Specify id to update associated business entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "Flag to set to enable or disable notifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_notifications_enabled: Option<bool>,
    #[doc = "Specify id for new card owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_user_id: Option<uuid::Uuid>,
    #[doc = "Modify spending restrictions. Only the fields to be modified need to be passed (so \
             fields that will stay the same do not have to be passed)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<PartialApiCardSpendingRestrictionsUpdateRequestBody>,
}

impl std::fmt::Display for ApiCardUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiCardUpdateRequestBody {
    const LENGTH: usize = 6;
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
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            if let Some(has_notifications_enabled) = &self.has_notifications_enabled {
                format!("{:?}", has_notifications_enabled).into()
            } else {
                String::new().into()
            },
            if let Some(new_user_id) = &self.new_user_id {
                format!("{:?}", new_user_id).into()
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
            "entity_id".into(),
            "has_notifications_enabled".into(),
            "new_user_id".into(),
            "spending_restrictions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiDepartmentCreateRequestBody {
    pub name: String,
}

impl std::fmt::Display for ApiDepartmentCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiDepartmentCreateRequestBody {
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
pub struct ApiDepartmentUpdateRequestBody {
    pub id: uuid::Uuid,
    pub name: String,
}

impl std::fmt::Display for ApiDepartmentUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiDepartmentUpdateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.id).into(), self.name.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiEntityResource {
    #[doc = "Information about the entity's active accounts"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<EntityProviderAccount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    pub id: uuid::Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<uuid::Uuid>>,
}

impl std::fmt::Display for ApiEntityResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiEntityResource {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounts) = &self.accounts {
                format!("{:?}", accounts).into()
            } else {
                String::new().into()
            },
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(entity_name) = &self.entity_name {
                format!("{:?}", entity_name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(is_primary) = &self.is_primary {
                format!("{:?}", is_primary).into()
            } else {
                String::new().into()
            },
            if let Some(location_ids) = &self.location_ids {
                format!("{:?}", location_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounts".into(),
            "currency".into(),
            "entity_name".into(),
            "id".into(),
            "is_primary".into(),
            "location_ids".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiLocationCreateRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    pub name: String,
}

impl std::fmt::Display for ApiLocationCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiLocationCreateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_id".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiLocationUpdateRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    pub name: String,
}

impl std::fmt::Display for ApiLocationUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiLocationUpdateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_id".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiMemoCreateRequestBody {
    #[serde(default)]
    pub is_memo_recurring: bool,
    pub memo: String,
}

impl std::fmt::Display for ApiMemoCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiMemoCreateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.is_memo_recurring).into(),
            self.memo.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["is_memo_recurring".into(), "memo".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiMerchantLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl std::fmt::Display for ApiMerchantLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiMerchantLocation {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.city).into(),
            format!("{:?}", self.country).into(),
            format!("{:?}", self.postal_code).into(),
            format!("{:?}", self.state).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "city".into(),
            "country".into(),
            "postal_code".into(),
            "state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiPermittedSpendTypesDump {
    #[doc = "Indicates whether the user's physical card can be linked to this limit."]
    pub primary_card_enabled: bool,
    #[doc = "Indicates whether reimbursements can be submitted against this limit."]
    pub reimbursements_enabled: bool,
}

impl std::fmt::Display for ApiPermittedSpendTypesDump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiPermittedSpendTypesDump {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.primary_card_enabled).into(),
            format!("{:?}", self.reimbursements_enabled).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "primary_card_enabled".into(),
            "reimbursements_enabled".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiPermittedSpendTypesRequestBody {
    #[doc = "Dictates whether the user's physical card can be linked to this limit."]
    pub primary_card_enabled: bool,
    #[doc = "Dictates whether reimbursements can be submitted against this limit."]
    pub reimbursements_enabled: bool,
}

impl std::fmt::Display for ApiPermittedSpendTypesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiPermittedSpendTypesRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.primary_card_enabled).into(),
            format!("{:?}", self.reimbursements_enabled).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "primary_card_enabled".into(),
            "reimbursements_enabled".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiPhysicalRequestBody {
    #[doc = "Alternative method to create a card using a card program. One of \
             spending_restrictions or card_program_id must be provided. If this value is given, \
             no other attributes (other than idempotency_key) may be given."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_program_id: Option<uuid::Uuid>,
    #[doc = "Cosmetic display name of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Create card associated with business entity. If not provided, defaults to entity \
             associated with user's location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "Fulfillment details of a Ramp card. For physical cards only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<CardFulfillmentRequestBody>,
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
    #[doc = "Set to true to create a physical card"]
    #[serde(default)]
    pub is_physical: bool,
    #[doc = "Set to true to create a temporary card"]
    #[serde(default)]
    pub is_temporary: bool,
    #[doc = "Specifies the spend restrictions on a Ramp card. One of spending_restrictions or \
             card_program_id must be provided."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiCardSpendingRestrictionsRequestBody>,
    #[doc = "Unique identifier of the card owner."]
    pub user_id: uuid::Uuid,
}

impl std::fmt::Display for ApiPhysicalRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiPhysicalRequestBody {
    const LENGTH: usize = 9;
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
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            if let Some(fulfillment) = &self.fulfillment {
                format!("{:?}", fulfillment).into()
            } else {
                String::new().into()
            },
            self.idempotency_key.clone().into(),
            format!("{:?}", self.is_physical).into(),
            format!("{:?}", self.is_temporary).into(),
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "card_program_id".into(),
            "display_name".into(),
            "entity_id".into(),
            "fulfillment".into(),
            "idempotency_key".into(),
            "is_physical".into(),
            "is_temporary".into(),
            "spending_restrictions".into(),
            "user_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReceiptIntegrationOptedOutEmailCreateRequestBody {
    pub business_id: i64,
    pub email: String,
}

impl std::fmt::Display for ApiReceiptIntegrationOptedOutEmailCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReceiptIntegrationOptedOutEmailCreateRequestBody {
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
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReceiptIntegrationOptedOutEmailResource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
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
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
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
        vec!["email".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReceiptIntegrationOptedOutEmailResourceRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiReceiptIntegrationOptedOutEmailResourceRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReceiptIntegrationOptedOutEmailResourceRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
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
        vec!["email".into(), "id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReceiptUploadRequestBody {
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
    #[doc = "Transaction to be attached with the uploaded receipt"]
    pub transaction_id: uuid::Uuid,
    #[doc = "User to be associated with the uploaded receipt. Transactions from the receipt's \
             user will be prioritized for selection when performing receipt matching."]
    pub user_id: uuid::Uuid,
}

impl std::fmt::Display for ApiReceiptUploadRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReceiptUploadRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.idempotency_key.clone().into(),
            format!("{:?}", self.transaction_id).into(),
            format!("{:?}", self.user_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "idempotency_key".into(),
            "transaction_id".into(),
            "user_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReimbursementAccountingCategoryInfo {
    #[doc = "external id of accounting field; It should uniquely identify an accounting field on \
             the ERP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "ID that uniquely identifies an accounting field within Ramp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of accounting field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "accounting field type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

impl std::fmt::Display for ApiReimbursementAccountingCategoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiReimbursementAccountingCategoryInfo {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
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
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "external_id".into(),
            "id".into(),
            "name".into(),
            "type_".into(),
        ]
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
pub enum ApiReimbursementAccountingFieldSelectionType {
    #[serde(rename = "AMORTIZATION_TEMPLATE")]
    #[display("AMORTIZATION_TEMPLATE")]
    AmortizationTemplate,
    #[serde(rename = "BILLABLE")]
    #[display("BILLABLE")]
    Billable,
    #[serde(rename = "COST_CENTER")]
    #[display("COST_CENTER")]
    CostCenter,
    #[serde(rename = "CUSTOMERS_JOBS")]
    #[display("CUSTOMERS_JOBS")]
    CustomersJobs,
    #[serde(rename = "DEFERRAL_CODE")]
    #[display("DEFERRAL_CODE")]
    DeferralCode,
    #[serde(rename = "EXPENSE_ENTITY")]
    #[display("EXPENSE_ENTITY")]
    ExpenseEntity,
    #[serde(rename = "GL_ACCOUNT")]
    #[display("GL_ACCOUNT")]
    GlAccount,
    #[serde(rename = "INVENTORY_ITEM")]
    #[display("INVENTORY_ITEM")]
    InventoryItem,
    #[serde(rename = "JOURNAL")]
    #[display("JOURNAL")]
    Journal,
    #[serde(rename = "MERCHANT")]
    #[display("MERCHANT")]
    Merchant,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
    #[serde(rename = "PROJECT")]
    #[display("PROJECT")]
    Project,
    #[serde(rename = "REPORTING_TAG")]
    #[display("REPORTING_TAG")]
    ReportingTag,
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
    #[doc = "information about the accounting category (or accounting field)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_info: Option<ApiReimbursementAccountingCategoryInfo>,
    #[doc = "external code of accounting field option; Code field displayed on the ERP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_code: Option<String>,
    #[doc = "external id of accounting field option; It should uniquely identify an accounting \
             field option on the ERP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "ID that uniquely identifies an accounting field option within Ramp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of accounting field option"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Accounting field type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ApiReimbursementAccountingFieldSelectionType>,
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
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(category_info) = &self.category_info {
                format!("{:?}", category_info).into()
            } else {
                String::new().into()
            },
            if let Some(external_code) = &self.external_code {
                format!("{:?}", external_code).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
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
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "category_info".into(),
            "external_code".into(),
            "external_id".into(),
            "id".into(),
            "name".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiReimbursementLineItem {
    #[doc = "List of accounting field options selected to code the line item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiReimbursementAccountingFieldSelection>>,
    #[doc = "Amount of the line item"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<CurrencyAmount>,
    #[doc = "Memo for the line item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
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
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.amount).into(),
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_field_selections".into(),
            "amount".into(),
            "memo".into(),
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
    #[doc = "A short description of the business."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_description: Option<String>,
    #[doc = "Doing business as (DBA)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_dba: Option<String>,
    #[doc = "Legal name of the business."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_legal: Option<String>,
    #[doc = "Business's website."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_website: Option<String>,
    #[doc = "Business's incorporation date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_incorporation: Option<chrono::NaiveDate>,
    #[doc = "Employer Identification Number (EIN)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein_number: Option<String>,
    #[doc = "Type of incorporation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[doc = "Estimated monthly spend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_spend: Option<String>,
    #[doc = "Business's industry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[doc = "Business's industry group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_group: Option<String>,
    #[doc = "Office's address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<ApiSalesLeadOfficeAddress>,
    #[doc = "Office phone number. Must include country code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_phone_number: Option<String>,
    #[doc = "Business's sector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[doc = "The state in which the business is incorporated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_of_incorporation: Option<String>,
    #[doc = "sub_industry."]
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
            if let Some(business_description) = &self.business_description {
                format!("{:?}", business_description).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_dba) = &self.business_name_dba {
                format!("{:?}", business_name_dba).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_legal) = &self.business_name_legal {
                format!("{:?}", business_name_legal).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_website) = &self.business_name_website {
                format!("{:?}", business_name_website).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_incorporation) = &self.date_of_incorporation {
                format!("{:?}", date_of_incorporation).into()
            } else {
                String::new().into()
            },
            if let Some(ein_number) = &self.ein_number {
                format!("{:?}", ein_number).into()
            } else {
                String::new().into()
            },
            if let Some(entity_type) = &self.entity_type {
                format!("{:?}", entity_type).into()
            } else {
                String::new().into()
            },
            if let Some(estimated_monthly_spend) = &self.estimated_monthly_spend {
                format!("{:?}", estimated_monthly_spend).into()
            } else {
                String::new().into()
            },
            if let Some(industry) = &self.industry {
                format!("{:?}", industry).into()
            } else {
                String::new().into()
            },
            if let Some(industry_group) = &self.industry_group {
                format!("{:?}", industry_group).into()
            } else {
                String::new().into()
            },
            if let Some(office_address) = &self.office_address {
                format!("{:?}", office_address).into()
            } else {
                String::new().into()
            },
            if let Some(office_phone_number) = &self.office_phone_number {
                format!("{:?}", office_phone_number).into()
            } else {
                String::new().into()
            },
            if let Some(sector) = &self.sector {
                format!("{:?}", sector).into()
            } else {
                String::new().into()
            },
            if let Some(state_of_incorporation) = &self.state_of_incorporation {
                format!("{:?}", state_of_incorporation).into()
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
            "business_description".into(),
            "business_name_dba".into(),
            "business_name_legal".into(),
            "business_name_website".into(),
            "date_of_incorporation".into(),
            "ein_number".into(),
            "entity_type".into(),
            "estimated_monthly_spend".into(),
            "industry".into(),
            "industry_group".into(),
            "office_address".into(),
            "office_phone_number".into(),
            "sector".into(),
            "state_of_incorporation".into(),
            "sub_industry".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadBusinessRequestBody {
    #[doc = "A short description of the business."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_description: Option<String>,
    #[doc = "Doing business as (DBA)"]
    pub business_name_dba: String,
    #[doc = "Legal name of the business."]
    pub business_name_legal: String,
    #[doc = "Business's website."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_website: Option<String>,
    #[doc = "Business's incorporation date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_incorporation: Option<chrono::NaiveDate>,
    #[doc = "Employer Identification Number (EIN)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein_number: Option<String>,
    #[doc = "Type of incorporation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[doc = "Estimated monthly spend."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_monthly_spend: Option<String>,
    #[doc = "Business's industry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    #[doc = "Business's industry group"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_group: Option<String>,
    #[doc = "Office's address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_address: Option<ApiSalesLeadOfficeAddressRequestBody>,
    #[doc = "Office phone number. Must include country code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_phone_number: Option<String>,
    #[doc = "Business's sector"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    #[doc = "The state in which the business is incorporated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_of_incorporation: Option<String>,
    #[doc = "Business's subindustry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_industry: Option<String>,
}

impl std::fmt::Display for ApiSalesLeadBusinessRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSalesLeadBusinessRequestBody {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(business_description) = &self.business_description {
                format!("{:?}", business_description).into()
            } else {
                String::new().into()
            },
            self.business_name_dba.clone().into(),
            self.business_name_legal.clone().into(),
            if let Some(business_name_website) = &self.business_name_website {
                format!("{:?}", business_name_website).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_incorporation) = &self.date_of_incorporation {
                format!("{:?}", date_of_incorporation).into()
            } else {
                String::new().into()
            },
            if let Some(ein_number) = &self.ein_number {
                format!("{:?}", ein_number).into()
            } else {
                String::new().into()
            },
            if let Some(entity_type) = &self.entity_type {
                format!("{:?}", entity_type).into()
            } else {
                String::new().into()
            },
            if let Some(estimated_monthly_spend) = &self.estimated_monthly_spend {
                format!("{:?}", estimated_monthly_spend).into()
            } else {
                String::new().into()
            },
            if let Some(industry) = &self.industry {
                format!("{:?}", industry).into()
            } else {
                String::new().into()
            },
            if let Some(industry_group) = &self.industry_group {
                format!("{:?}", industry_group).into()
            } else {
                String::new().into()
            },
            if let Some(office_address) = &self.office_address {
                format!("{:?}", office_address).into()
            } else {
                String::new().into()
            },
            if let Some(office_phone_number) = &self.office_phone_number {
                format!("{:?}", office_phone_number).into()
            } else {
                String::new().into()
            },
            if let Some(sector) = &self.sector {
                format!("{:?}", sector).into()
            } else {
                String::new().into()
            },
            if let Some(state_of_incorporation) = &self.state_of_incorporation {
                format!("{:?}", state_of_incorporation).into()
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
            "business_description".into(),
            "business_name_dba".into(),
            "business_name_legal".into(),
            "business_name_website".into(),
            "date_of_incorporation".into(),
            "ein_number".into(),
            "entity_type".into(),
            "estimated_monthly_spend".into(),
            "industry".into(),
            "industry_group".into(),
            "office_address".into(),
            "office_phone_number".into(),
            "sector".into(),
            "state_of_incorporation".into(),
            "sub_industry".into(),
        ]
    }
}

#[doc = "Source."]
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
pub enum Source {
    AngelList,
    Quanta,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadCreateRequestBody {
    #[doc = "business_info."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_info: Option<ApiSalesLeadBusinessRequestBody>,
    #[doc = "email."]
    pub email: String,
    #[doc = "external_id."]
    pub external_id: String,
    #[doc = "first_name."]
    pub first_name: String,
    #[doc = "last_name."]
    pub last_name: String,
    #[doc = "phone."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "redirect_uri."]
    pub redirect_uri: String,
    #[doc = "source."]
    pub source: Source,
    pub state: String,
}

impl std::fmt::Display for ApiSalesLeadCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSalesLeadCreateRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(business_info) = &self.business_info {
                format!("{:?}", business_info).into()
            } else {
                String::new().into()
            },
            self.email.clone().into(),
            self.external_id.clone().into(),
            self.first_name.clone().into(),
            self.last_name.clone().into(),
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            self.redirect_uri.clone().into(),
            format!("{:?}", self.source).into(),
            self.state.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "business_info".into(),
            "email".into(),
            "external_id".into(),
            "first_name".into(),
            "last_name".into(),
            "phone".into(),
            "redirect_uri".into(),
            "source".into(),
            "state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadOfficeAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_apt_suite: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_street_address: Option<String>,
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
            if let Some(office_apt_suite) = &self.office_apt_suite {
                format!("{:?}", office_apt_suite).into()
            } else {
                String::new().into()
            },
            if let Some(office_city) = &self.office_city {
                format!("{:?}", office_city).into()
            } else {
                String::new().into()
            },
            if let Some(office_country) = &self.office_country {
                format!("{:?}", office_country).into()
            } else {
                String::new().into()
            },
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
            if let Some(office_street_address) = &self.office_street_address {
                format!("{:?}", office_street_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "office_apt_suite".into(),
            "office_city".into(),
            "office_country".into(),
            "office_postal_code".into(),
            "office_state".into(),
            "office_street_address".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSalesLeadOfficeAddressRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_apt_suite: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub office_street_address: Option<String>,
}

impl std::fmt::Display for ApiSalesLeadOfficeAddressRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSalesLeadOfficeAddressRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(office_apt_suite) = &self.office_apt_suite {
                format!("{:?}", office_apt_suite).into()
            } else {
                String::new().into()
            },
            if let Some(office_city) = &self.office_city {
                format!("{:?}", office_city).into()
            } else {
                String::new().into()
            },
            if let Some(office_country) = &self.office_country {
                format!("{:?}", office_country).into()
            } else {
                String::new().into()
            },
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
            if let Some(office_street_address) = &self.office_street_address {
                format!("{:?}", office_street_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "office_apt_suite".into(),
            "office_city".into(),
            "office_country".into(),
            "office_postal_code".into(),
            "office_state".into(),
            "office_street_address".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendAllocationBalance {
    #[doc = "Cleared amount on this limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleared: Option<CurrencyAmount>,
    #[doc = "Pending amount towards this limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<CurrencyAmount>,
    #[doc = "Total amount spent on this limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<CurrencyAmount>,
}

impl std::fmt::Display for ApiSpendAllocationBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendAllocationBalance {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(cleared) = &self.cleared {
                format!("{:?}", cleared).into()
            } else {
                String::new().into()
            },
            if let Some(pending) = &self.pending {
                format!("{:?}", pending).into()
            } else {
                String::new().into()
            },
            if let Some(total) = &self.total {
                format!("{:?}", total).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["cleared".into(), "pending".into(), "total".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendLimitCardResource {
    #[doc = "Unique identifier of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<uuid::Uuid>,
    #[doc = "Card created manually by Ramp for high velocity spend"]
    pub is_ap_card: bool,
    #[doc = "Card created by 'New Product or Service' option"]
    pub via_new_product_or_service: bool,
}

impl std::fmt::Display for ApiSpendLimitCardResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitCardResource {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(card_id) = &self.card_id {
                format!("{:?}", card_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_ap_card).into(),
            format!("{:?}", self.via_new_product_or_service).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "card_id".into(),
            "is_ap_card".into(),
            "via_new_product_or_service".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendLimitCreateRequestBody {
    #[doc = "Cosmetic display name of the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Fulfillment details of the limit's card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<CardFulfillmentRequestBody>,
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
    #[doc = "Dictates whether the spend limit is shareable among multiple users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shareable: Option<bool>,
    #[doc = "Specifies the permitted spend types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permitted_spend_types: Option<ApiPermittedSpendTypesRequestBody>,
    #[doc = "The id of the associated spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spend_program_id: Option<uuid::Uuid>,
    #[doc = "Specifies the spending restrictions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiSpendingRestrictionsRequestBody>,
    #[doc = "Unique identifier of the limit owner."]
    pub user_id: uuid::Uuid,
}

impl std::fmt::Display for ApiSpendLimitCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitCreateRequestBody {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(fulfillment) = &self.fulfillment {
                format!("{:?}", fulfillment).into()
            } else {
                String::new().into()
            },
            self.idempotency_key.clone().into(),
            if let Some(is_shareable) = &self.is_shareable {
                format!("{:?}", is_shareable).into()
            } else {
                String::new().into()
            },
            if let Some(permitted_spend_types) = &self.permitted_spend_types {
                format!("{:?}", permitted_spend_types).into()
            } else {
                String::new().into()
            },
            if let Some(spend_program_id) = &self.spend_program_id {
                format!("{:?}", spend_program_id).into()
            } else {
                String::new().into()
            },
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.user_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "display_name".into(),
            "fulfillment".into(),
            "idempotency_key".into(),
            "is_shareable".into(),
            "permitted_spend_types".into(),
            "spend_program_id".into(),
            "spending_restrictions".into(),
            "user_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendLimitDeferredTaskContext {
    #[doc = "Unique identifier of the acting user that initiated the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acting_user_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the subject limit in the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spend_limit_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiSpendLimitDeferredTaskContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitDeferredTaskContext {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(acting_user_id) = &self.acting_user_id {
                format!("{:?}", acting_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(spend_limit_id) = &self.spend_limit_id {
                format!("{:?}", spend_limit_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["acting_user_id".into(), "spend_limit_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendLimitDeferredTaskData {
    #[doc = "An error message if the deferred task fails."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "Unique identifier of the subject limit in the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spend_limit_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiSpendLimitDeferredTaskData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitDeferredTaskData {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            if let Some(spend_limit_id) = &self.spend_limit_id {
                format!("{:?}", spend_limit_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["error".into(), "spend_limit_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendLimitDeferredUpdateRequestBody {
    #[doc = "An idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    pub idempotency_key: String,
}

impl std::fmt::Display for ApiSpendLimitDeferredUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitDeferredUpdateRequestBody {
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
pub struct ApiSpendLimitMember {
    #[doc = "The unique identifier of the limit owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiSpendLimitMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitMember {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(user_id) = &self.user_id {
            format!("{:?}", user_id).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendLimitModifyUserAccessRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl std::fmt::Display for ApiSpendLimitModifyUserAccessRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitModifyUserAccessRequestBody {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(user_ids) = &self.user_ids {
            format!("{:?}", user_ids).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user_ids".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendLimitUpdateRequestBody {
    #[doc = "Change the display name of the limit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Dictates whether the spend limit is shareable among multiple users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shareable: Option<bool>,
    #[doc = "Change the user of the limit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_user_id: Option<uuid::Uuid>,
    #[doc = "Modify permitted spend types. All fields of permitted_spend_types must be given."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permitted_spend_types: Option<ApiPermittedSpendTypesRequestBody>,
    #[doc = "Specify a spend program to link with.\nThis will override the limit's spending \
             restrictions and permitted spend types with those of the spend program.\nPass \
             spend_program_id = None to detach the limit's current spend program.\nIf the \
             spend_program_id field is specified, no other fields may be passed.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spend_program_id: Option<uuid::Uuid>,
    #[doc = "Modify spending restrictions. If this field is passed, the entire set of new \
             spending restrictions must be passed (i.e. the given spending restrictions will \
             override all existing spending restrictions)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiSpendingRestrictionsRequestBody>,
}

impl std::fmt::Display for ApiSpendLimitUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendLimitUpdateRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(is_shareable) = &self.is_shareable {
                format!("{:?}", is_shareable).into()
            } else {
                String::new().into()
            },
            if let Some(new_user_id) = &self.new_user_id {
                format!("{:?}", new_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(permitted_spend_types) = &self.permitted_spend_types {
                format!("{:?}", permitted_spend_types).into()
            } else {
                String::new().into()
            },
            if let Some(spend_program_id) = &self.spend_program_id {
                format!("{:?}", spend_program_id).into()
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
            "display_name".into(),
            "is_shareable".into(),
            "new_user_id".into(),
            "permitted_spend_types".into(),
            "spend_program_id".into(),
            "spending_restrictions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendProgramCreateIssuanceRulesRequestBody {
    #[doc = "Set of rules for having spend programs issued by default to users"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub automatic: Option<ApiSpendProgramIssuanceRulesRequestBody>,
    #[doc = "Set of rules for users requesting spend programs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requestable: Option<ApiSpendProgramIssuanceRulesRequestBody>,
}

impl std::fmt::Display for ApiSpendProgramCreateIssuanceRulesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendProgramCreateIssuanceRulesRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(automatic) = &self.automatic {
                format!("{:?}", automatic).into()
            } else {
                String::new().into()
            },
            if let Some(requestable) = &self.requestable {
                format!("{:?}", requestable).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["automatic".into(), "requestable".into()]
    }
}

#[doc = "The template icon for the spend program."]
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
pub enum ApiSpendProgramCreateRequestBodyIcon {
    AccountingServicesIcon,
    AdvertisingIcon,
    #[serde(rename = "CONTRACTORS_AND_PROFESSIONAL_SERVICES")]
    #[display("CONTRACTORS_AND_PROFESSIONAL_SERVICES")]
    ContractorsAndProfessionalServices,
    #[serde(rename = "CUSTOM")]
    #[display("CUSTOM")]
    Custom,
    CardIcon,
    EducationStipendIcon,
    EmployeeRewardsIcon,
    GroundTransportationIcon,
    LegalFeesIcon,
    LodgingIcon,
    LunchOrderingIcon,
    OnboardingIcon,
    PerDiemCardIcon,
    #[serde(rename = "SOFTWARE")]
    #[display("SOFTWARE")]
    Software,
    SaasSubscriptionIcon,
    SoftwareTrialIcon,
    SuppliesIcon,
    TeamSocialIcon,
    TravelExpensesIcon,
    VirtualEventIcon,
    WellnessIcon,
    WorkFromHomeIcon,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendProgramCreateRequestBody {
    #[doc = "Description of the spend program."]
    pub description: String,
    #[doc = "Display name of the spend program."]
    pub display_name: String,
    #[doc = "The template icon for the spend program."]
    pub icon: ApiSpendProgramCreateRequestBodyIcon,
    #[doc = "Dictates whether the spend program is shareable among multiple users."]
    #[serde(default)]
    pub is_shareable: bool,
    #[doc = "Spend Program Issuance Rules can be set for requests or default issuance of Limits \
             from a program. Set whether a program is requestable or issued by default for a \
             given set of users and their attributes (department, locations, and custom fields). \
             If you'd like to give these permissions to all employees, you can set \
             `applies_to_all` to `True`. Feel free to ignore this if you don't want any custom \
             requestability or issuance logic."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuance_rules: Option<ApiSpendProgramCreateIssuanceRulesRequestBody>,
    #[doc = "Dictates whether the spend program should issue a physical card if the user does not \
             have one."]
    #[serde(default)]
    pub issue_physical_card_if_needed: bool,
    #[doc = "Specifies the permitted spend types for the spend program."]
    pub permitted_spend_types: ApiPermittedSpendTypesRequestBody,
    #[doc = "A set of restrictions imposed on the spend program."]
    pub spending_restrictions: ApiSpendingRestrictionsRequestBody,
}

impl std::fmt::Display for ApiSpendProgramCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendProgramCreateRequestBody {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.description.clone().into(),
            self.display_name.clone().into(),
            format!("{:?}", self.icon).into(),
            format!("{:?}", self.is_shareable).into(),
            if let Some(issuance_rules) = &self.issuance_rules {
                format!("{:?}", issuance_rules).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.issue_physical_card_if_needed).into(),
            format!("{:?}", self.permitted_spend_types).into(),
            format!("{:?}", self.spending_restrictions).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "description".into(),
            "display_name".into(),
            "icon".into(),
            "is_shareable".into(),
            "issuance_rules".into(),
            "issue_physical_card_if_needed".into(),
            "permitted_spend_types".into(),
            "spending_restrictions".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendProgramIssuanceRulesRequestBody {
    #[doc = "Dictates whether this rule should apply to all employees or not (if True, \
             location_ids, department_ids, and user_custom_field_ids should be null)."]
    #[serde(default)]
    pub applies_to_all: bool,
    #[doc = "List of departments whose users are able to request or be issued this spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<uuid::Uuid>>,
    #[doc = "List of locations whose users are able to request or be issued this spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_ids: Option<Vec<uuid::Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_custom_field_ids: Option<Vec<uuid::Uuid>>,
}

impl std::fmt::Display for ApiSpendProgramIssuanceRulesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendProgramIssuanceRulesRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.applies_to_all).into(),
            if let Some(department_ids) = &self.department_ids {
                format!("{:?}", department_ids).into()
            } else {
                String::new().into()
            },
            if let Some(location_ids) = &self.location_ids {
                format!("{:?}", location_ids).into()
            } else {
                String::new().into()
            },
            if let Some(user_custom_field_ids) = &self.user_custom_field_ids {
                format!("{:?}", user_custom_field_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "applies_to_all".into(),
            "department_ids".into(),
            "location_ids".into(),
            "user_custom_field_ids".into(),
        ]
    }
}

#[doc = "The template icon for the spend program."]
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
pub enum ApiSpendProgramResourceIcon {
    AccountingServicesIcon,
    AdvertisingIcon,
    #[serde(rename = "CONTRACTORS_AND_PROFESSIONAL_SERVICES")]
    #[display("CONTRACTORS_AND_PROFESSIONAL_SERVICES")]
    ContractorsAndProfessionalServices,
    #[serde(rename = "CUSTOM")]
    #[display("CUSTOM")]
    Custom,
    CardIcon,
    EducationStipendIcon,
    EmployeeRewardsIcon,
    GroundTransportationIcon,
    LegalFeesIcon,
    LodgingIcon,
    LunchOrderingIcon,
    OnboardingIcon,
    PerDiemCardIcon,
    #[serde(rename = "SOFTWARE")]
    #[display("SOFTWARE")]
    Software,
    SaasSubscriptionIcon,
    SoftwareTrialIcon,
    SuppliesIcon,
    TeamSocialIcon,
    TravelExpensesIcon,
    VirtualEventIcon,
    WellnessIcon,
    WorkFromHomeIcon,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendProgramResource {
    #[doc = "Description of the spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Display name of the spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The template icon for the spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ApiSpendProgramResourceIcon>,
    #[doc = "Unique identifier of the spend program."]
    pub id: uuid::Uuid,
    #[doc = "Whether this spend program is shareable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shareable: Option<bool>,
    #[doc = "Indicates whether the spend program should issue a physical card if the user does \
             not have one."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_physical_card_if_needed: Option<bool>,
    #[doc = "Permitted spend types for the spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permitted_spend_types: Option<ApiPermittedSpendTypesDump>,
    #[doc = "A set of restrictions imposed on the spend program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<SpendIntentApiSpendingRestrictionsDump>,
}

impl std::fmt::Display for ApiSpendProgramResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendProgramResource {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(icon) = &self.icon {
                format!("{:?}", icon).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(is_shareable) = &self.is_shareable {
                format!("{:?}", is_shareable).into()
            } else {
                String::new().into()
            },
            if let Some(issue_physical_card_if_needed) = &self.issue_physical_card_if_needed {
                format!("{:?}", issue_physical_card_if_needed).into()
            } else {
                String::new().into()
            },
            if let Some(permitted_spend_types) = &self.permitted_spend_types {
                format!("{:?}", permitted_spend_types).into()
            } else {
                String::new().into()
            },
            if let Some(restrictions) = &self.restrictions {
                format!("{:?}", restrictions).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "description".into(),
            "display_name".into(),
            "icon".into(),
            "id".into(),
            "is_shareable".into(),
            "issue_physical_card_if_needed".into(),
            "permitted_spend_types".into(),
            "restrictions".into(),
        ]
    }
}

#[doc = "Time interval the limit is applied on."]
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
pub enum ApiSpendingRestrictionsDumpInterval {
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
pub struct ApiSpendingRestrictionsDump {
    #[doc = " List of Ramp category codes allowed for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<i64>>,
    #[doc = "List of merchants allowed for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_vendors: Option<Vec<uuid::Uuid>>,
    #[doc = "Date to automatically to lock the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "List of Ramp category codes blocked for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<i64>>,
    #[doc = "List of merchants  blocked for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_vendors: Option<Vec<uuid::Uuid>>,
    #[doc = "Time interval the limit is applied on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<ApiSpendingRestrictionsDumpInterval>,
    #[doc = "Amount limit total per interval denominated in cents. Currency is USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<CurrencyAmount>,
    #[doc = "Date and time for the next interval reset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_interval_reset: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Date and time for the start of the current interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_of_interval: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Temporary limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary_limit: Option<CurrencyAmount>,
    #[doc = "Max amount allowed on a single transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<CurrencyAmount>,
}

impl std::fmt::Display for ApiSpendingRestrictionsDump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendingRestrictionsDump {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(allowed_categories) = &self.allowed_categories {
                format!("{:?}", allowed_categories).into()
            } else {
                String::new().into()
            },
            if let Some(allowed_vendors) = &self.allowed_vendors {
                format!("{:?}", allowed_vendors).into()
            } else {
                String::new().into()
            },
            if let Some(auto_lock_date) = &self.auto_lock_date {
                format!("{:?}", auto_lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_categories) = &self.blocked_categories {
                format!("{:?}", blocked_categories).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_vendors) = &self.blocked_vendors {
                format!("{:?}", blocked_vendors).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
            } else {
                String::new().into()
            },
            if let Some(limit) = &self.limit {
                format!("{:?}", limit).into()
            } else {
                String::new().into()
            },
            if let Some(next_interval_reset) = &self.next_interval_reset {
                format!("{:?}", next_interval_reset).into()
            } else {
                String::new().into()
            },
            if let Some(start_of_interval) = &self.start_of_interval {
                format!("{:?}", start_of_interval).into()
            } else {
                String::new().into()
            },
            if let Some(temporary_limit) = &self.temporary_limit {
                format!("{:?}", temporary_limit).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "allowed_categories".into(),
            "allowed_vendors".into(),
            "auto_lock_date".into(),
            "blocked_categories".into(),
            "blocked_vendors".into(),
            "interval".into(),
            "limit".into(),
            "next_interval_reset".into(),
            "start_of_interval".into(),
            "temporary_limit".into(),
            "transaction_amount_limit".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSpendingRestrictionsRequestBody {
    #[doc = " List of Ramp category codes allowed for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<i64>>,
    #[doc = "List of merchants allowed for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_vendors: Option<Vec<uuid::Uuid>>,
    #[doc = "List of Ramp category codes blocked for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_mcc_codes: Option<Vec<String>>,
    #[doc = "List of merchants  blocked for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_vendors: Option<Vec<uuid::Uuid>>,
    #[doc = "Time interval to apply limit to."]
    pub interval: Interval,
    #[doc = "Total amount limit per interval. Currently we expect the currency to be USD and the \
             amount need to be denominated in cents."]
    pub limit: CurrencyAmountRequestBody,
    #[doc = "Date to automatically lock the card. If lock date has passed, set to a future date \
             or to null to unlock the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Max amount per transaction. Currently we expect the currency to be USD and the \
             amount need to be denominated in cents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<CurrencyAmountRequestBody>,
}

impl std::fmt::Display for ApiSpendingRestrictionsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSpendingRestrictionsRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(allowed_categories) = &self.allowed_categories {
                format!("{:?}", allowed_categories).into()
            } else {
                String::new().into()
            },
            if let Some(allowed_vendors) = &self.allowed_vendors {
                format!("{:?}", allowed_vendors).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_categories) = &self.blocked_categories {
                format!("{:?}", blocked_categories).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_mcc_codes) = &self.blocked_mcc_codes {
                format!("{:?}", blocked_mcc_codes).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_vendors) = &self.blocked_vendors {
                format!("{:?}", blocked_vendors).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.interval).into(),
            format!("{:?}", self.limit).into(),
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "allowed_categories".into(),
            "allowed_vendors".into(),
            "blocked_categories".into(),
            "blocked_mcc_codes".into(),
            "blocked_vendors".into(),
            "interval".into(),
            "limit".into(),
            "lock_date".into(),
            "transaction_amount_limit".into(),
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
pub enum ApiStatementItemType {
    #[serde(rename = "CARD_TRANSACTION")]
    #[display("CARD_TRANSACTION")]
    CardTransaction,
    #[serde(rename = "CASHBACK")]
    #[display("CASHBACK")]
    Cashback,
    #[serde(rename = "TRANSFER_PAYMENT")]
    #[display("TRANSFER_PAYMENT")]
    TransferPayment,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiStatementItem {
    #[doc = "This field is only present for transactions, and it represents the time when the \
             transaction was cleared. If the transaction is not cleared, this field will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cleared_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "For transactions, this field will represent the time when the card is swiped. For \
             transfers / cashbacks, this will represent creation time."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ApiStatementItemType>,
}

impl std::fmt::Display for ApiStatementItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiStatementItem {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(cleared_at) = &self.cleared_at {
                format!("{:?}", cleared_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
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
            "cleared_at".into(),
            "created_at".into(),
            "id".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiSuspensionDump {
    #[doc = "Unique identifier of the user who placed the suspension."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acting_user_id: Option<uuid::Uuid>,
    #[doc = "Date and time at which suspension was placed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inserted_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Whether the suspension is placed by Ramp."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspended_by_ramp: Option<bool>,
}

impl std::fmt::Display for ApiSuspensionDump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiSuspensionDump {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(acting_user_id) = &self.acting_user_id {
                format!("{:?}", acting_user_id).into()
            } else {
                String::new().into()
            },
            if let Some(inserted_at) = &self.inserted_at {
                format!("{:?}", inserted_at).into()
            } else {
                String::new().into()
            },
            if let Some(suspended_by_ramp) = &self.suspended_by_ramp {
                format!("{:?}", suspended_by_ramp).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "acting_user_id".into(),
            "inserted_at".into(),
            "suspended_by_ramp".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionAccountingCategoryInfo {
    #[doc = "external id of accounting field; It should uniquely identify an accounting field on \
             the ERP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "ID that uniquely identifies an accounting field within Ramp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of accounting field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "accounting field type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
}

impl std::fmt::Display for ApiTransactionAccountingCategoryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionAccountingCategoryInfo {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
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
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "external_id".into(),
            "id".into(),
            "name".into(),
            "type_".into(),
        ]
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
pub enum ApiTransactionAccountingFieldSelectionType {
    #[serde(rename = "AMORTIZATION_TEMPLATE")]
    #[display("AMORTIZATION_TEMPLATE")]
    AmortizationTemplate,
    #[serde(rename = "BILLABLE")]
    #[display("BILLABLE")]
    Billable,
    #[serde(rename = "COST_CENTER")]
    #[display("COST_CENTER")]
    CostCenter,
    #[serde(rename = "CUSTOMERS_JOBS")]
    #[display("CUSTOMERS_JOBS")]
    CustomersJobs,
    #[serde(rename = "DEFERRAL_CODE")]
    #[display("DEFERRAL_CODE")]
    DeferralCode,
    #[serde(rename = "EXPENSE_ENTITY")]
    #[display("EXPENSE_ENTITY")]
    ExpenseEntity,
    #[serde(rename = "GL_ACCOUNT")]
    #[display("GL_ACCOUNT")]
    GlAccount,
    #[serde(rename = "INVENTORY_ITEM")]
    #[display("INVENTORY_ITEM")]
    InventoryItem,
    #[serde(rename = "JOURNAL")]
    #[display("JOURNAL")]
    Journal,
    #[serde(rename = "MERCHANT")]
    #[display("MERCHANT")]
    Merchant,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
    #[serde(rename = "PROJECT")]
    #[display("PROJECT")]
    Project,
    #[serde(rename = "REPORTING_TAG")]
    #[display("REPORTING_TAG")]
    ReportingTag,
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
pub struct ApiTransactionAccountingFieldSelection {
    #[doc = "information about the accounting category (or accounting field)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category_info: Option<ApiTransactionAccountingCategoryInfo>,
    #[doc = "external code of accounting field option; Code field displayed on the ERP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_code: Option<String>,
    #[doc = "external id of accounting field option; It should uniquely identify an accounting \
             field option on the ERP."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "ID that uniquely identifies an accounting field option within Ramp"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "name of accounting field option"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Accounting field type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<ApiTransactionAccountingFieldSelectionType>,
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
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(category_info) = &self.category_info {
                format!("{:?}", category_info).into()
            } else {
                String::new().into()
            },
            if let Some(external_code) = &self.external_code {
                format!("{:?}", external_code).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
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
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "category_info".into(),
            "external_code".into(),
            "external_id".into(),
            "id".into(),
            "name".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionCardHolder {
    #[doc = "ID of the card holder's department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[doc = "Name of the card holder's deparment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    #[doc = "Card holder's first name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Card holder's last name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "ID of the card holder's location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[doc = "Name of the card holder's location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
    #[doc = "Card holder's ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
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
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(department_name) = &self.department_name {
                format!("{:?}", department_name).into()
            } else {
                String::new().into()
            },
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
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(location_name) = &self.location_name {
                format!("{:?}", location_name).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "department_id".into(),
            "department_name".into(),
            "first_name".into(),
            "last_name".into(),
            "location_id".into(),
            "location_name".into(),
            "user_id".into(),
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
    #[serde(rename = "AUTHORIZER_CARD_START_DATE")]
    #[display("AUTHORIZER_CARD_START_DATE")]
    AuthorizerCardStartDate,
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
    #[serde(rename = "AUTHORIZER_OOB_BLOCKED_MERCHANT")]
    #[display("AUTHORIZER_OOB_BLOCKED_MERCHANT")]
    AuthorizerOobBlockedMerchant,
    #[serde(rename = "AUTHORIZER_OOB_DAILY_BUSINESS_BALANCE")]
    #[display("AUTHORIZER_OOB_DAILY_BUSINESS_BALANCE")]
    AuthorizerOobDailyBusinessBalance,
    #[serde(rename = "AUTHORIZER_OOB_DAILY_CARD_SPEND")]
    #[display("AUTHORIZER_OOB_DAILY_CARD_SPEND")]
    AuthorizerOobDailyCardSpend,
    #[serde(rename = "AUTHORIZER_TRANSACTION_AMOUNT_LIMIT")]
    #[display("AUTHORIZER_TRANSACTION_AMOUNT_LIMIT")]
    AuthorizerTransactionAmountLimit,
    #[serde(rename = "AUTHORIZER_USER_LIMIT")]
    #[display("AUTHORIZER_USER_LIMIT")]
    AuthorizerUserLimit,
    #[serde(rename = "AUTHORIZER_USER_SUSPENDED")]
    #[display("AUTHORIZER_USER_SUSPENDED")]
    AuthorizerUserSuspended,
    #[serde(rename = "BLOCKED_COUNTRY")]
    #[display("BLOCKED_COUNTRY")]
    BlockedCountry,
    #[serde(rename = "CARD_EXPIRED")]
    #[display("CARD_EXPIRED")]
    CardExpired,
    #[serde(rename = "CARD_TERMINATED")]
    #[display("CARD_TERMINATED")]
    CardTerminated,
    #[serde(rename = "CHIP_FAILLURE")]
    #[display("CHIP_FAILLURE")]
    ChipFaillure,
    #[serde(rename = "FORBIDDEN_CATEGORY")]
    #[display("FORBIDDEN_CATEGORY")]
    ForbiddenCategory,
    #[serde(rename = "INSECURE_AUTHORIZATION_METHOD")]
    #[display("INSECURE_AUTHORIZATION_METHOD")]
    InsecureAuthorizationMethod,
    #[serde(rename = "MOBILE_WALLET_FAILURE")]
    #[display("MOBILE_WALLET_FAILURE")]
    MobileWalletFailure,
    #[serde(rename = "NOT_ACTIVE")]
    #[display("NOT_ACTIVE")]
    NotActive,
    #[serde(rename = "NOT_ALLOWED")]
    #[display("NOT_ALLOWED")]
    NotAllowed,
    #[serde(rename = "NO_AUTO_ROUTED_LIMITS_AVAILABLE")]
    #[display("NO_AUTO_ROUTED_LIMITS_AVAILABLE")]
    NoAutoRoutedLimitsAvailable,
    #[serde(rename = "NO_LINKED_SPEND_ALLOCATION")]
    #[display("NO_LINKED_SPEND_ALLOCATION")]
    NoLinkedSpendAllocation,
    #[serde(rename = "OFAC_VERIFICATION_NEEDED")]
    #[display("OFAC_VERIFICATION_NEEDED")]
    OfacVerificationNeeded,
    #[serde(rename = "OPEN_TO_BUY_LIMIT")]
    #[display("OPEN_TO_BUY_LIMIT")]
    OpenToBuyLimit,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
    #[serde(rename = "PIN_BLOCKED")]
    #[display("PIN_BLOCKED")]
    PinBlocked,
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
    #[serde(rename = "USER_BLOCKED")]
    #[display("USER_BLOCKED")]
    UserBlocked,
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
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
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
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(reason) = &self.reason {
                format!("{:?}", reason).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["amount".into(), "reason".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionDispute {
    #[doc = "Time at which the dispute is created, presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Uniquely identifies a transaction dispute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Free form text regarding the dispute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "The dispute type; It could be one of the following values: RESOLVED_BY_RAMP, \
             CANCELLED_BY_CUSTOMER, CANCELLED_BY_RAMP, CREATED_MERCHANT_ERROR and \
             CREATED_UNRECOGNIZED_CHARGE."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
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
            "created_at".into(),
            "id".into(),
            "memo".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionLineItem {
    #[doc = "List of accounting field options selected to code the line item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiTransactionAccountingFieldSelection>>,
    #[doc = "Amount of the line item, denominated in the currency that the transaction was \
             settled in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<CurrencyAmount>,
    #[doc = "Memo associated with the line item."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
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
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.amount).into(),
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_field_selections".into(),
            "amount".into(),
            "memo".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPolicyViolation {
    #[doc = "Time at which the policy violation is created, presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Uniquely identifies a policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Free form text regarding the policy violation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "Type of the policy violation."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
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
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
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
            "created_at".into(),
            "id".into(),
            "memo".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPurchaseAutoRental {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_out: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<i64>,
}

impl std::fmt::Display for ApiTransactionPurchaseAutoRental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPurchaseAutoRental {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(check_out) = &self.check_out {
                format!("{:?}", check_out).into()
            } else {
                String::new().into()
            },
            if let Some(days) = &self.days {
                format!("{:?}", days).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["check_out".into(), "days".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPurchaseData {
    #[doc = "Auto rental purchase data provided by the merchant"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_rental: Option<ApiTransactionPurchaseAutoRental>,
    #[doc = "Flight purchase data provided by the merchant"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flight: Option<ApiTransactionPurchaseFlightData>,
    #[doc = "Lodging purchase data provided by the merchant"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lodging: Option<ApiTransactionPurchaseLodging>,
    #[doc = "Receipt purchase data provided by the merchant"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt: Option<ApiTransactionPurchaseReceipt>,
    #[doc = "Purchase data reference ID provided by the merchant"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl std::fmt::Display for ApiTransactionPurchaseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPurchaseData {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.auto_rental).into(),
            format!("{:?}", self.flight).into(),
            format!("{:?}", self.lodging).into(),
            format!("{:?}", self.receipt).into(),
            format!("{:?}", self.reference).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "auto_rental".into(),
            "flight".into(),
            "lodging".into(),
            "receipt".into(),
            "reference".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPurchaseFlightData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<ApiTransactionPurchaseFlightSegment>>,
}

impl std::fmt::Display for ApiTransactionPurchaseFlightData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPurchaseFlightData {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.departure_date).into(),
            if let Some(passenger_name) = &self.passenger_name {
                format!("{:?}", passenger_name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.segments).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "departure_date".into(),
            "passenger_name".into(),
            "segments".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPurchaseFlightSegment {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arrival_airport_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopover_allowed: Option<bool>,
}

impl std::fmt::Display for ApiTransactionPurchaseFlightSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPurchaseFlightSegment {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(arrival_airport_code) = &self.arrival_airport_code {
                format!("{:?}", arrival_airport_code).into()
            } else {
                String::new().into()
            },
            if let Some(carrier) = &self.carrier {
                format!("{:?}", carrier).into()
            } else {
                String::new().into()
            },
            if let Some(departure_airport_code) = &self.departure_airport_code {
                format!("{:?}", departure_airport_code).into()
            } else {
                String::new().into()
            },
            if let Some(flight_number) = &self.flight_number {
                format!("{:?}", flight_number).into()
            } else {
                String::new().into()
            },
            if let Some(service_class) = &self.service_class {
                format!("{:?}", service_class).into()
            } else {
                String::new().into()
            },
            if let Some(stopover_allowed) = &self.stopover_allowed {
                format!("{:?}", stopover_allowed).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "arrival_airport_code".into(),
            "carrier".into(),
            "departure_airport_code".into(),
            "flight_number".into(),
            "service_class".into(),
            "stopover_allowed".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPurchaseLodging {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_in: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nights: Option<i64>,
}

impl std::fmt::Display for ApiTransactionPurchaseLodging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPurchaseLodging {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(check_in) = &self.check_in {
                format!("{:?}", check_in).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.nights).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["check_in".into(), "nights".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPurchaseReceipt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ApiTransactionPurchaseReceiptLineItem>>,
}

impl std::fmt::Display for ApiTransactionPurchaseReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPurchaseReceipt {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(items) = &self.items {
            format!("{:?}", items).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["items".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiTransactionPurchaseReceiptLineItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commodity_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<f64>,
}

impl std::fmt::Display for ApiTransactionPurchaseReceiptLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiTransactionPurchaseReceiptLineItem {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.commodity_code).into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.discount).into(),
            if let Some(quantity) = &self.quantity {
                format!("{:?}", quantity).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.tax).into(),
            if let Some(total) = &self.total {
                format!("{:?}", total).into()
            } else {
                String::new().into()
            },
            if let Some(unit_cost) = &self.unit_cost {
                format!("{:?}", unit_cost).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "commodity_code".into(),
            "description".into(),
            "discount".into(),
            "quantity".into(),
            "tax".into(),
            "total".into(),
            "unit_cost".into(),
        ]
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
pub enum Role {
    #[serde(rename = "ADVISOR_CONSOLE_ADMIN")]
    #[display("ADVISOR_CONSOLE_ADMIN")]
    AdvisorConsoleAdmin,
    #[serde(rename = "ADVISOR_CONSOLE_USER")]
    #[display("ADVISOR_CONSOLE_USER")]
    AdvisorConsoleUser,
    #[serde(rename = "AUDITOR")]
    #[display("AUDITOR")]
    Auditor,
    #[serde(rename = "BILL_PAY_ADMIN")]
    #[display("BILL_PAY_ADMIN")]
    BillPayAdmin,
    #[serde(rename = "BUSINESS_ADMIN")]
    #[display("BUSINESS_ADMIN")]
    BusinessAdmin,
    #[serde(rename = "BUSINESS_BOOKKEEPER")]
    #[display("BUSINESS_BOOKKEEPER")]
    BusinessBookkeeper,
    #[serde(rename = "BUSINESS_OWNER")]
    #[display("BUSINESS_OWNER")]
    BusinessOwner,
    #[serde(rename = "BUSINESS_USER")]
    #[display("BUSINESS_USER")]
    BusinessUser,
    #[serde(rename = "DEVELOPER_ADMIN")]
    #[display("DEVELOPER_ADMIN")]
    DeveloperAdmin,
    #[serde(rename = "GUEST_USER")]
    #[display("GUEST_USER")]
    GuestUser,
    #[serde(rename = "IT_ADMIN")]
    #[display("IT_ADMIN")]
    ItAdmin,
    #[serde(rename = "VENDOR_NETWORK_ADMIN")]
    #[display("VENDOR_NETWORK_ADMIN")]
    VendorNetworkAdmin,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserCreateRequestBody {
    #[doc = "Unique identifier of the employee's department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the employee's direct manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<uuid::Uuid>,
    #[doc = "The employee's email address"]
    pub email: String,
    #[doc = "An alternative identifier for an employee, coming from external systems, which can \
             be used in place of an email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[doc = "First name of the employee"]
    pub first_name: String,
    #[doc = "an idempotency key is a unique value generated by the client which the server uses \
             to recognize subsequent retries of the same request. To avoid collisions, we \
             encourage clients to use random generated UUIDs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[doc = "Last name of the employee"]
    pub last_name: String,
    #[doc = "Unique identifier of the employee's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<uuid::Uuid>,
    #[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
             Bookkeeper; Note that Owner is not a invitable role."]
    pub role: Role,
}

impl std::fmt::Display for ApiUserCreateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserCreateRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(direct_manager_id) = &self.direct_manager_id {
                format!("{:?}", direct_manager_id).into()
            } else {
                String::new().into()
            },
            self.email.clone().into(),
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            self.first_name.clone().into(),
            if let Some(idempotency_key) = &self.idempotency_key {
                format!("{:?}", idempotency_key).into()
            } else {
                String::new().into()
            },
            self.last_name.clone().into(),
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.role).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "department_id".into(),
            "direct_manager_id".into(),
            "email".into(),
            "employee_id".into(),
            "first_name".into(),
            "idempotency_key".into(),
            "last_name".into(),
            "location_id".into(),
            "role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserCustomField {
    #[doc = "Name of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Value of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for ApiUserCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserCustomField {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "value".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserDeferredTaskContext {
    #[doc = "Unique identifier of the acting user that initiated the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acting_user_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiUserDeferredTaskContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserDeferredTaskContext {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(acting_user_id) = &self.acting_user_id {
            format!("{:?}", acting_user_id).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["acting_user_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserDeferredTaskData {
    #[doc = "An error message if the deferred task fails"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The subject employee's ID of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
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
            if let Some(error) = &self.error {
                format!("{:?}", error).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["error".into(), "user_id".into()]
    }
}

#[doc = "Determine whether to either, reassign pending user approvals to their manager, reassign \
         them to a specified user, or fail if there are approvals."]
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
pub enum ReassignApprovalsBehavior {
    #[serde(rename = "DO_NOT_REPLACE")]
    #[display("DO_NOT_REPLACE")]
    DoNotReplace,
    #[serde(rename = "REPLACE_WITH_MANAGER")]
    #[display("REPLACE_WITH_MANAGER")]
    ReplaceWithManager,
    #[serde(rename = "REPLACE_WITH_USER")]
    #[display("REPLACE_WITH_USER")]
    ReplaceWithUser,
}

impl std::default::Default for ReassignApprovalsBehavior {
    fn default() -> Self {
        ReassignApprovalsBehavior::ReplaceWithManager
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserDeleteOptionRequestBody {
    #[doc = "Determine whether to either, reassign pending user approvals to their manager, \
             reassign them to a specified user, or fail if there are approvals."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reassign_approvals_behavior: Option<ReassignApprovalsBehavior>,
    #[doc = "Unique identifier of the replacement user. This user is going to replace the deleted \
             user in the approval chain."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_approver_user_id: Option<uuid::Uuid>,
}

impl std::fmt::Display for ApiUserDeleteOptionRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserDeleteOptionRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(reassign_approvals_behavior) = &self.reassign_approvals_behavior {
                format!("{:?}", reassign_approvals_behavior).into()
            } else {
                String::new().into()
            },
            if let Some(replacement_approver_user_id) = &self.replacement_approver_user_id {
                format!("{:?}", replacement_approver_user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "reassign_approvals_behavior".into(),
            "replacement_approver_user_id".into(),
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
pub enum ApiUserUpdateRequestBodyRole {
    #[serde(rename = "ADVISOR_CONSOLE_ADMIN")]
    #[display("ADVISOR_CONSOLE_ADMIN")]
    AdvisorConsoleAdmin,
    #[serde(rename = "ADVISOR_CONSOLE_USER")]
    #[display("ADVISOR_CONSOLE_USER")]
    AdvisorConsoleUser,
    #[serde(rename = "AUDITOR")]
    #[display("AUDITOR")]
    Auditor,
    #[serde(rename = "BILL_PAY_ADMIN")]
    #[display("BILL_PAY_ADMIN")]
    BillPayAdmin,
    #[serde(rename = "BUSINESS_ADMIN")]
    #[display("BUSINESS_ADMIN")]
    BusinessAdmin,
    #[serde(rename = "BUSINESS_BOOKKEEPER")]
    #[display("BUSINESS_BOOKKEEPER")]
    BusinessBookkeeper,
    #[serde(rename = "BUSINESS_OWNER")]
    #[display("BUSINESS_OWNER")]
    BusinessOwner,
    #[serde(rename = "BUSINESS_USER")]
    #[display("BUSINESS_USER")]
    BusinessUser,
    #[serde(rename = "DEVELOPER_ADMIN")]
    #[display("DEVELOPER_ADMIN")]
    DeveloperAdmin,
    #[serde(rename = "GUEST_USER")]
    #[display("GUEST_USER")]
    GuestUser,
    #[serde(rename = "IT_ADMIN")]
    #[display("IT_ADMIN")]
    ItAdmin,
    #[serde(rename = "VENDOR_NETWORK_ADMIN")]
    #[display("VENDOR_NETWORK_ADMIN")]
    VendorNetworkAdmin,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ApiUserUpdateRequestBody {
    #[doc = "Whether to automatically promote the users manager to manager."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_promote: Option<bool>,
    #[doc = "Unique identifier of the employee's department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the employee's direct manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_manager_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Whether the employee is a manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_manager: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Unique identifier of the employee's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<uuid::Uuid>,
    #[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
             Bookkeeper; Note that Owner is not a permissible value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<ApiUserUpdateRequestBodyRole>,
}

impl std::fmt::Display for ApiUserUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ApiUserUpdateRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(auto_promote) = &self.auto_promote {
                format!("{:?}", auto_promote).into()
            } else {
                String::new().into()
            },
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(direct_manager_id) = &self.direct_manager_id {
                format!("{:?}", direct_manager_id).into()
            } else {
                String::new().into()
            },
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(is_manager) = &self.is_manager {
                format!("{:?}", is_manager).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "auto_promote".into(),
            "department_id".into(),
            "direct_manager_id".into(),
            "employee_id".into(),
            "first_name".into(),
            "is_manager".into(),
            "last_name".into(),
            "location_id".into(),
            "role".into(),
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
    #[serde(rename = "OPEN")]
    #[display("OPEN")]
    Open,
    #[serde(rename = "PAID")]
    #[display("PAID")]
    Paid,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Bill {
    #[doc = "List of accounting field options selected to code the bill."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiBillAccountingFieldSelection>>,
    #[doc = "Amount of the bill."]
    pub amount: CurrencyAmount,
    #[doc = "Information about the bill owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bill_owner: Option<ApiBillOwner>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_link_url: Option<String>,
    pub due_at: chrono::DateTime<chrono::Utc>,
    #[doc = "Associated business entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    pub id: uuid::Uuid,
    #[doc = "The invoice number on the bill."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    #[doc = "Pre-signed urls to download invoice files."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invoice_urls: Option<Vec<String>>,
    pub issued_at: chrono::DateTime<chrono::Utc>,
    #[doc = "List of line items related to the bill."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<ApiBillLineItem>>,
    #[doc = "Retrieve the most likely matching transaction ID for a bill, applicable only if paid \
             with a Ramp card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_transaction_id: Option<uuid::Uuid>,
    #[doc = "Memo of the bill."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "Payment information of the bill."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment: Option<ApiBillPayment>,
    #[doc = "An ID that identifies the bill on client's side."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_id: Option<String>,
    pub status: Status,
    #[doc = "Vendor information of the bill."]
    pub vendor: ApiBillVendor,
}

impl std::fmt::Display for Bill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Bill {
    const LENGTH: usize = 18;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_field_selections) = &self.accounting_field_selections {
                format!("{:?}", accounting_field_selections).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.amount).into(),
            if let Some(bill_owner) = &self.bill_owner {
                format!("{:?}", bill_owner).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(deep_link_url) = &self.deep_link_url {
                format!("{:?}", deep_link_url).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.due_at).into(),
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(invoice_number) = &self.invoice_number {
                format!("{:?}", invoice_number).into()
            } else {
                String::new().into()
            },
            if let Some(invoice_urls) = &self.invoice_urls {
                format!("{:?}", invoice_urls).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.issued_at).into(),
            if let Some(line_items) = &self.line_items {
                format!("{:?}", line_items).into()
            } else {
                String::new().into()
            },
            if let Some(matching_transaction_id) = &self.matching_transaction_id {
                format!("{:?}", matching_transaction_id).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
            if let Some(payment) = &self.payment {
                format!("{:?}", payment).into()
            } else {
                String::new().into()
            },
            if let Some(remote_id) = &self.remote_id {
                format!("{:?}", remote_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            format!("{:?}", self.vendor).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_field_selections".into(),
            "amount".into(),
            "bill_owner".into(),
            "created_at".into(),
            "deep_link_url".into(),
            "due_at".into(),
            "entity_id".into(),
            "id".into(),
            "invoice_number".into(),
            "invoice_urls".into(),
            "issued_at".into(),
            "line_items".into(),
            "matching_transaction_id".into(),
            "memo".into(),
            "payment".into(),
            "remote_id".into(),
            "status".into(),
            "vendor".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Business {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<CardShippingAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_legal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name_on_card: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_sso: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_approved_limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_integrated_with_slack: Option<bool>,
    pub is_reimbursements_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_locked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
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
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
            } else {
                String::new().into()
            },
            if let Some(billing_address) = &self.billing_address {
                format!("{:?}", billing_address).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_legal) = &self.business_name_legal {
                format!("{:?}", business_name_legal).into()
            } else {
                String::new().into()
            },
            if let Some(business_name_on_card) = &self.business_name_on_card {
                format!("{:?}", business_name_on_card).into()
            } else {
                String::new().into()
            },
            if let Some(created_time) = &self.created_time {
                format!("{:?}", created_time).into()
            } else {
                String::new().into()
            },
            if let Some(enforce_sso) = &self.enforce_sso {
                format!("{:?}", enforce_sso).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(initial_approved_limit) = &self.initial_approved_limit {
                format!("{:?}", initial_approved_limit).into()
            } else {
                String::new().into()
            },
            if let Some(is_integrated_with_slack) = &self.is_integrated_with_slack {
                format!("{:?}", is_integrated_with_slack).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_reimbursements_enabled).into(),
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
            if let Some(website) = &self.website {
                format!("{:?}", website).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "active".into(),
            "billing_address".into(),
            "business_name_legal".into(),
            "business_name_on_card".into(),
            "created_time".into(),
            "enforce_sso".into(),
            "id".into(),
            "initial_approved_limit".into(),
            "is_integrated_with_slack".into(),
            "is_reimbursements_enabled".into(),
            "limit_locked".into(),
            "phone".into(),
            "website".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BusinessBalance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_card_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_flex_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_including_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_balance_excluding_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_balance_including_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_balance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flex_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub float_balance_excluding_pending: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_balance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_billing_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prev_billing_date: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_balance: Option<f64>,
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
            if let Some(available_card_limit) = &self.available_card_limit {
                format!("{:?}", available_card_limit).into()
            } else {
                String::new().into()
            },
            if let Some(available_flex_limit) = &self.available_flex_limit {
                format!("{:?}", available_flex_limit).into()
            } else {
                String::new().into()
            },
            if let Some(balance_including_pending) = &self.balance_including_pending {
                format!("{:?}", balance_including_pending).into()
            } else {
                String::new().into()
            },
            if let Some(card_balance_excluding_pending) = &self.card_balance_excluding_pending {
                format!("{:?}", card_balance_excluding_pending).into()
            } else {
                String::new().into()
            },
            if let Some(card_balance_including_pending) = &self.card_balance_including_pending {
                format!("{:?}", card_balance_including_pending).into()
            } else {
                String::new().into()
            },
            if let Some(card_limit) = &self.card_limit {
                format!("{:?}", card_limit).into()
            } else {
                String::new().into()
            },
            if let Some(flex_balance) = &self.flex_balance {
                format!("{:?}", flex_balance).into()
            } else {
                String::new().into()
            },
            if let Some(flex_limit) = &self.flex_limit {
                format!("{:?}", flex_limit).into()
            } else {
                String::new().into()
            },
            if let Some(float_balance_excluding_pending) = &self.float_balance_excluding_pending {
                format!("{:?}", float_balance_excluding_pending).into()
            } else {
                String::new().into()
            },
            if let Some(global_limit) = &self.global_limit {
                format!("{:?}", global_limit).into()
            } else {
                String::new().into()
            },
            if let Some(max_balance) = &self.max_balance {
                format!("{:?}", max_balance).into()
            } else {
                String::new().into()
            },
            if let Some(next_billing_date) = &self.next_billing_date {
                format!("{:?}", next_billing_date).into()
            } else {
                String::new().into()
            },
            if let Some(prev_billing_date) = &self.prev_billing_date {
                format!("{:?}", prev_billing_date).into()
            } else {
                String::new().into()
            },
            if let Some(statement_balance) = &self.statement_balance {
                format!("{:?}", statement_balance).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "available_card_limit".into(),
            "available_flex_limit".into(),
            "balance_including_pending".into(),
            "card_balance_excluding_pending".into(),
            "card_balance_including_pending".into(),
            "card_limit".into(),
            "flex_balance".into(),
            "flex_limit".into(),
            "float_balance_excluding_pending".into(),
            "global_limit".into(),
            "max_balance".into(),
            "next_billing_date".into(),
            "prev_billing_date".into(),
            "statement_balance".into(),
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
    #[serde(rename = "CHIP_LOCKED")]
    #[display("CHIP_LOCKED")]
    ChipLocked,
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
    #[doc = "Unique identifier of the card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_program_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the card holder."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_id: Option<uuid::Uuid>,
    #[doc = "Card holder's full name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[doc = "Date time at which the card is created. It conforms to ISO8601 format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Cosmetic display name of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Unique identifier of the associated business entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "Expiration date in the format of MMYY."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[doc = "Fulfillment details of a Ramp card. For physical cards only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fulfillment: Option<ApiCardFulfillment>,
    #[doc = "Whether the card has overridden the default settings from its card program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_program_overridden: Option<bool>,
    #[doc = "Unique identifier of the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_physical: Option<bool>,
    #[doc = "Last four digits of the card number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,
    #[doc = "Specifies the spend restrictions on a Ramp card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spending_restrictions: Option<ApiCardSpendingRestrictionsDump>,
    #[doc = "State of the card"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
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
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.card_program_id).into(),
            if let Some(cardholder_id) = &self.cardholder_id {
                format!("{:?}", cardholder_id).into()
            } else {
                String::new().into()
            },
            if let Some(cardholder_name) = &self.cardholder_name {
                format!("{:?}", cardholder_name).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            if let Some(expiration) = &self.expiration {
                format!("{:?}", expiration).into()
            } else {
                String::new().into()
            },
            if let Some(fulfillment) = &self.fulfillment {
                format!("{:?}", fulfillment).into()
            } else {
                String::new().into()
            },
            if let Some(has_program_overridden) = &self.has_program_overridden {
                format!("{:?}", has_program_overridden).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(is_physical) = &self.is_physical {
                format!("{:?}", is_physical).into()
            } else {
                String::new().into()
            },
            if let Some(last_four) = &self.last_four {
                format!("{:?}", last_four).into()
            } else {
                String::new().into()
            },
            if let Some(spending_restrictions) = &self.spending_restrictions {
                format!("{:?}", spending_restrictions).into()
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
            "card_program_id".into(),
            "cardholder_id".into(),
            "cardholder_name".into(),
            "created_at".into(),
            "display_name".into(),
            "entity_id".into(),
            "expiration".into(),
            "fulfillment".into(),
            "has_program_overridden".into(),
            "id".into(),
            "is_physical".into(),
            "last_four".into(),
            "spending_restrictions".into(),
            "state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardDeferredTask {
    #[doc = "Further context for the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<ApiCardDeferredTaskContext>,
    #[doc = "Detailed data of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ApiCardDeferredTaskData>,
    #[doc = "Unique identifier of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Status of the deferred task. It could be one of the following values: STARTED, \
             IN_PROGRESS, ERROR, SUCCESS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(context) = &self.context {
                format!("{:?}", context).into()
            } else {
                String::new().into()
            },
            if let Some(data) = &self.data {
                format!("{:?}", data).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "context".into(),
            "data".into(),
            "id".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardFulfillmentRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_personalization: Option<CardPersonalizationRequestBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cardholder_uuid: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping: Option<CardShippingRequestBody>,
}

impl std::fmt::Display for CardFulfillmentRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardFulfillmentRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(card_personalization) = &self.card_personalization {
                format!("{:?}", card_personalization).into()
            } else {
                String::new().into()
            },
            if let Some(cardholder_uuid) = &self.cardholder_uuid {
                format!("{:?}", cardholder_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(shipping) = &self.shipping {
                format!("{:?}", shipping).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "card_personalization".into(),
            "cardholder_uuid".into(),
            "shipping".into(),
        ]
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
pub struct CardPersonalizationNameLineRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for CardPersonalizationNameLineRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardPersonalizationNameLineRequestBody {
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
pub struct CardPersonalizationRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<CardPersonalizationTextRequestBody>,
}

impl std::fmt::Display for CardPersonalizationRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardPersonalizationRequestBody {
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

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardPersonalizationText {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_line_1: Option<CardPersonalizationNameLine>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_line_2: Option<CardPersonalizationNameLine>,
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
            if let Some(name_line_1) = &self.name_line_1 {
                format!("{:?}", name_line_1).into()
            } else {
                String::new().into()
            },
            if let Some(name_line_2) = &self.name_line_2 {
                format!("{:?}", name_line_2).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name_line_1".into(), "name_line_2".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardPersonalizationTextRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_line_1: Option<CardPersonalizationNameLineRequestBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_line_2: Option<CardPersonalizationNameLineRequestBody>,
}

impl std::fmt::Display for CardPersonalizationTextRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardPersonalizationTextRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name_line_1) = &self.name_line_1 {
                format!("{:?}", name_line_1).into()
            } else {
                String::new().into()
            },
            if let Some(name_line_2) = &self.name_line_2 {
                format!("{:?}", name_line_2).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name_line_1".into(), "name_line_2".into()]
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
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_address: Option<CardShippingAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_address_verification_state: Option<RecipientAddressVerificationState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<CardShippingAddress>,
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
            if let Some(return_address) = &self.return_address {
                format!("{:?}", return_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "method".into(),
            "recipient_address".into(),
            "recipient_address_verification_state".into(),
            "return_address".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardShippingAddress {
    #[serde(rename = "address1")]
    pub address_1: String,
    #[serde(rename = "address2", default, skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    pub city: String,
    pub country: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub postal_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
            self.address_1.clone().into(),
            if let Some(address_2) = &self.address_2 {
                format!("{:?}", address_2).into()
            } else {
                String::new().into()
            },
            self.city.clone().into(),
            self.country.clone().into(),
            self.first_name.clone().into(),
            self.last_name.clone().into(),
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            self.postal_code.clone().into(),
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address_1".into(),
            "address_2".into(),
            "city".into(),
            "country".into(),
            "first_name".into(),
            "last_name".into(),
            "phone".into(),
            "postal_code".into(),
            "state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardShippingAddressRequestBody {
    #[serde(rename = "address1")]
    pub address_1: String,
    #[serde(rename = "address2", default, skip_serializing_if = "Option::is_none")]
    pub address_2: Option<String>,
    pub city: String,
    pub country: String,
    pub first_name: String,
    pub last_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub postal_code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl std::fmt::Display for CardShippingAddressRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardShippingAddressRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.address_1.clone().into(),
            if let Some(address_2) = &self.address_2 {
                format!("{:?}", address_2).into()
            } else {
                String::new().into()
            },
            self.city.clone().into(),
            self.country.clone().into(),
            self.first_name.clone().into(),
            self.last_name.clone().into(),
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            self.postal_code.clone().into(),
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address_1".into(),
            "address_2".into(),
            "city".into(),
            "country".into(),
            "first_name".into(),
            "last_name".into(),
            "phone".into(),
            "postal_code".into(),
            "state".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CardShippingRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_address: Option<CardShippingAddressRequestBody>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_address_verification_state: Option<RecipientAddressVerificationState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_address: Option<CardShippingAddressRequestBody>,
}

impl std::fmt::Display for CardShippingRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CardShippingRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(return_address) = &self.return_address {
                format!("{:?}", return_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "method".into(),
            "recipient_address".into(),
            "recipient_address_verification_state".into(),
            "return_address".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Cashback {
    #[doc = "Dollar amount of the cashback payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<CurrencyAmount>,
    #[doc = "Timestamp at which the cash payment is made. Presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Unique identifier of the associated business entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the cashback payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

impl std::fmt::Display for Cashback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Cashback {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
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
        vec![
            "amount".into(),
            "created_at".into(),
            "entity_id".into(),
            "id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CurrencyAmount {
    #[doc = "the amount of money represented in the smallest denomination of the currency. For \
             example, when the currency is USD, then the amount is expressed in cents."]
    pub amount: i64,
    #[doc = "The type of currency, in ISO 4217 format. e.g. USD for US dollars"]
    pub currency_code: String,
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
            format!("{:?}", self.amount).into(),
            self.currency_code.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["amount".into(), "currency_code".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CurrencyAmountRequestBody {
    #[doc = "the amount of money represented in the smallest denomination of the currency. For \
             example, when the currency is USD, then the amount is expressed in cents."]
    pub amount: i64,
    #[doc = "The type of currency, in ISO 4217 format. e.g. USD for US dollars"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
}

impl std::fmt::Display for CurrencyAmountRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CurrencyAmountRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.amount).into(),
            if let Some(currency_code) = &self.currency_code {
                format!("{:?}", currency_code).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["amount".into(), "currency_code".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomFieldOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "id to uniquely identify a custom field option within Ramp system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ramp_id: Option<uuid::Uuid>,
    #[doc = "A unique identifier for the custom field option in the remote ERP system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "A vendor is a person or business that provides goods or services"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl std::fmt::Display for CustomFieldOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomFieldOption {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(is_active) = &self.is_active {
                format!("{:?}", is_active).into()
            } else {
                String::new().into()
            },
            if let Some(ramp_id) = &self.ramp_id {
                format!("{:?}", ramp_id).into()
            } else {
                String::new().into()
            },
            if let Some(remote_code) = &self.remote_code {
                format!("{:?}", remote_code).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(value) = &self.value {
                format!("{:?}", value).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "id".into(),
            "is_active".into(),
            "ramp_id".into(),
            "remote_code".into(),
            "updated_at".into(),
            "value".into(),
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
pub struct Department {
    pub id: uuid::Uuid,
    pub name: String,
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
        vec![format!("{:?}", self.id).into(), self.name.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into()]
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
pub enum Classification {
    #[serde(rename = "ASSET")]
    #[display("ASSET")]
    Asset,
    #[serde(rename = "CREDCARD")]
    #[display("CREDCARD")]
    Credcard,
    #[serde(rename = "EQUITY")]
    #[display("EQUITY")]
    Equity,
    #[serde(rename = "EXPENSE")]
    #[display("EXPENSE")]
    Expense,
    #[serde(rename = "LIABILITY")]
    #[display("LIABILITY")]
    Liability,
    #[serde(rename = "REVENUE")]
    #[display("REVENUE")]
    Revenue,
    #[serde(rename = "UNKNOWN")]
    #[display("UNKNOWN")]
    Unknown,
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
pub enum UsageType {
    #[serde(rename = "BILL_PAY_ACCOUNTS_PAYABLE_ACCOUNT")]
    #[display("BILL_PAY_ACCOUNTS_PAYABLE_ACCOUNT")]
    BillPayAccountsPayableAccount,
    #[serde(rename = "BILL_PAY_BANK_ACCOUNT")]
    #[display("BILL_PAY_BANK_ACCOUNT")]
    BillPayBankAccount,
    #[serde(rename = "BILL_PAY_FX_GAIN_LOSS_ACCOUNT")]
    #[display("BILL_PAY_FX_GAIN_LOSS_ACCOUNT")]
    BillPayFxGainLossAccount,
    #[serde(rename = "CARD_LIABILITY_ACCOUNT")]
    #[display("CARD_LIABILITY_ACCOUNT")]
    CardLiabilityAccount,
    #[serde(rename = "CARD_TRANSACTION_BILLS_ACCOUNTS_PAYABLE_ACCOUNT")]
    #[display("CARD_TRANSACTION_BILLS_ACCOUNTS_PAYABLE_ACCOUNT")]
    CardTransactionBillsAccountsPayableAccount,
    #[serde(rename = "CASHBACK_ACCOUNT")]
    #[display("CASHBACK_ACCOUNT")]
    CashbackAccount,
    #[serde(rename = "CASH_ACCOUNT")]
    #[display("CASH_ACCOUNT")]
    CashAccount,
    #[serde(rename = "INTERCOMPANY_ACCOUNTS_PAYABLE_ACCOUNT")]
    #[display("INTERCOMPANY_ACCOUNTS_PAYABLE_ACCOUNT")]
    IntercompanyAccountsPayableAccount,
    #[serde(rename = "INTERCOMPANY_ACCOUNTS_RECEIVABLE_ACCOUNT")]
    #[display("INTERCOMPANY_ACCOUNTS_RECEIVABLE_ACCOUNT")]
    IntercompanyAccountsReceivableAccount,
    #[serde(rename = "INTERCOMPANY_TRANSFER_CLEARING_ACCOUNT")]
    #[display("INTERCOMPANY_TRANSFER_CLEARING_ACCOUNT")]
    IntercompanyTransferClearingAccount,
    #[serde(rename = "REIMBURSEMENT_ACCOUNTS_PAYABLE_ACCOUNT")]
    #[display("REIMBURSEMENT_ACCOUNTS_PAYABLE_ACCOUNT")]
    ReimbursementAccountsPayableAccount,
    #[serde(rename = "REIMBURSEMENT_BANK_ACCOUNT")]
    #[display("REIMBURSEMENT_BANK_ACCOUNT")]
    ReimbursementBankAccount,
    #[serde(rename = "TRANSFER_BANK_ACCOUNT")]
    #[display("TRANSFER_BANK_ACCOUNT")]
    TransferBankAccount,
    #[serde(rename = "WALLET_ACCOUNT")]
    #[display("WALLET_ACCOUNT")]
    WalletAccount,
    #[serde(rename = "WALLET_BANK_ACCOUNT")]
    #[display("WALLET_BANK_ACCOUNT")]
    WalletBankAccount,
    #[serde(rename = "WALLET_YIELD_ACCOUNT")]
    #[display("WALLET_YIELD_ACCOUNT")]
    WalletYieldAccount,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EntityProviderAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<Classification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "External ID of the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "External name of the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_name: Option<String>,
    #[doc = "Internal ID of the account, when applicable"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_type: Option<UsageType>,
}

impl std::fmt::Display for EntityProviderAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EntityProviderAccount {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(classification) = &self.classification {
                format!("{:?}", classification).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
            if let Some(external_name) = &self.external_name {
                format!("{:?}", external_name).into()
            } else {
                String::new().into()
            },
            if let Some(internal_id) = &self.internal_id {
                format!("{:?}", internal_id).into()
            } else {
                String::new().into()
            },
            if let Some(usage_type) = &self.usage_type {
                format!("{:?}", usage_type).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "classification".into(),
            "created_at".into(),
            "external_id".into(),
            "external_name".into(),
            "internal_id".into(),
            "usage_type".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FieldOption {
    pub id: String,
    #[doc = "e.g. Employees:Salaries & Wages"]
    pub value: String,
}

impl std::fmt::Display for FieldOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FieldOption {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.id.clone().into(), self.value.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "value".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Glaccount {
    pub classification: Classification,
    #[doc = "e.g. 400-100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub id: String,
    #[doc = "e.g. Travel : Travel - Lodging."]
    pub name: String,
}

impl std::fmt::Display for Glaccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Glaccount {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.classification).into(),
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "classification".into(),
            "code".into(),
            "id".into(),
            "name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GeneralLedgerAccount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classification: Option<Classification>,
    #[doc = "e.g. 400-100."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "e.g. Travel : Travel - Lodging."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "id to uniquely identify a general ledger account within Ramp system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ramp_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl std::fmt::Display for GeneralLedgerAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GeneralLedgerAccount {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(classification) = &self.classification {
                format!("{:?}", classification).into()
            } else {
                String::new().into()
            },
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(is_active) = &self.is_active {
                format!("{:?}", is_active).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(ramp_id) = &self.ramp_id {
                format!("{:?}", ramp_id).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "classification".into(),
            "code".into(),
            "created_at".into(),
            "id".into(),
            "is_active".into(),
            "name".into(),
            "ramp_id".into(),
            "updated_at".into(),
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
pub enum FinancingApplicationStatus {
    #[serde(rename = "ALLOY_COMPLETE")]
    #[display("ALLOY_COMPLETE")]
    AlloyComplete,
    #[serde(rename = "Admin Approved")]
    #[display("Admin Approved")]
    AdminApproved,
    #[serde(rename = "DELETED")]
    #[display("DELETED")]
    Deleted,
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
pub enum LeadSource {
    AngelList,
    Quanta,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Lead {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_info: Option<ApiSalesLeadBusinessDump>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub external_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub financing_application_status: Option<FinancingApplicationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub source: LeadSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
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
            if let Some(business_info) = &self.business_info {
                format!("{:?}", business_info).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            self.external_id.clone().into(),
            if let Some(financing_application_status) = &self.financing_application_status {
                format!("{:?}", financing_application_status).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.source).into(),
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "business_info".into(),
            "created_at".into(),
            "email".into(),
            "external_id".into(),
            "financing_application_status".into(),
            "first_name".into(),
            "id".into(),
            "last_name".into(),
            "phone".into(),
            "source".into(),
            "updated_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LightReceipt {
    #[doc = "Unique identifier of the receipt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}

impl std::fmt::Display for LightReceipt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LightReceipt {
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
pub struct Location {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    pub id: uuid::Uuid,
    pub name: String,
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
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["entity_id".into(), "id".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Memo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
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
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "memo".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Merchant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sk_category_name: Option<String>,
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
            format!("{:?}", self.sk_category_name).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "merchant_name".into(),
            "sk_category_name".into(),
        ]
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
pub struct PaginatedResponseApiAccountingCustomFieldOptionResourceSchema {
    pub data: Vec<CustomFieldOption>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiAccountingCustomFieldOptionResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiAccountingCustomFieldOptionResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiAccountingCustomFieldResourceSchema {
    pub data: Vec<ApiAccountingCustomFieldResource>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiAccountingCustomFieldResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiAccountingCustomFieldResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiAccountingGLAccountResourceSchema {
    pub data: Vec<GeneralLedgerAccount>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiAccountingGLAccountResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiAccountingGLAccountResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiAccountingVendorResourceSchema {
    pub data: Vec<VendorAccount>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiAccountingVendorResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiAccountingVendorResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiBillResourceSchema {
    pub data: Vec<Bill>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiBillResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiBillResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiCardProgramResourceSchema {
    pub data: Vec<ApiCardProgramResource>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiCardResourceSchema {
    pub data: Vec<Card>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiCashbackResourceSchema {
    pub data: Vec<Cashback>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiCashbackResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiCashbackResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiDepartmentResourceSchema {
    pub data: Vec<Department>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiLocationResourceSchema {
    pub data: Vec<Location>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiMemoResourceSchema {
    pub data: Vec<Memo>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiMerchantResourceSchema {
    pub data: Vec<Merchant>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiReceiptResourceSchema {
    pub data: Vec<Receipt>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiReimbursementResourceSchema {
    pub data: Vec<Reimbursement>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiSpendLimitResourceSchema {
    pub data: Vec<Limit>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiSpendLimitResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiSpendLimitResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiSpendProgramResourceSchema {
    pub data: Vec<ApiSpendProgramResource>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiSpendProgramResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiSpendProgramResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiStatementResourceSchema {
    pub data: Vec<Statement>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiStatementResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiStatementResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiTransactionCanonicalSchema {
    pub data: Vec<Transaction>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiTransferResourceSchema {
    pub data: Vec<Transfer>,
    pub page: DeveloperAPINestedPage,
}

impl std::fmt::Display for PaginatedResponseApiTransferResourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaginatedResponseApiTransferResourceSchema {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaginatedResponseApiUserResourceSchema {
    pub data: Vec<User>,
    pub page: DeveloperAPINestedPage,
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
            format!("{:?}", self.data).into(),
            format!("{:?}", self.page).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into(), "page".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PartialApiCardSpendingRestrictionsUpdateRequestBody {
    #[doc = "Amount limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_mcc_codes: Option<Vec<String>>,
    #[doc = "List of Ramp Category Codes this card is restricted to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_blacklist: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub categories_whitelist: Option<Vec<i64>>,
    #[doc = "Currency in which the amount is specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "Time interval to apply limit to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Interval>,
    #[doc = "Date to automatically lock the card. If lock date has passed, set to a future date \
             or to null to unlock the card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "Max amount limit per transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_blacklist: Option<Vec<uuid::Uuid>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vendor_whitelist: Option<Vec<uuid::Uuid>>,
}

impl std::fmt::Display for PartialApiCardSpendingRestrictionsUpdateRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PartialApiCardSpendingRestrictionsUpdateRequestBody {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_mcc_codes) = &self.blocked_mcc_codes {
                format!("{:?}", blocked_mcc_codes).into()
            } else {
                String::new().into()
            },
            if let Some(categories) = &self.categories {
                format!("{:?}", categories).into()
            } else {
                String::new().into()
            },
            if let Some(categories_blacklist) = &self.categories_blacklist {
                format!("{:?}", categories_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(categories_whitelist) = &self.categories_whitelist {
                format!("{:?}", categories_whitelist).into()
            } else {
                String::new().into()
            },
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
            } else {
                String::new().into()
            },
            if let Some(lock_date) = &self.lock_date {
                format!("{:?}", lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(policy_id) = &self.policy_id {
                format!("{:?}", policy_id).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_blacklist) = &self.vendor_blacklist {
                format!("{:?}", vendor_blacklist).into()
            } else {
                String::new().into()
            },
            if let Some(vendor_whitelist) = &self.vendor_whitelist {
                format!("{:?}", vendor_whitelist).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount".into(),
            "blocked_mcc_codes".into(),
            "categories".into(),
            "categories_blacklist".into(),
            "categories_whitelist".into(),
            "currency".into(),
            "interval".into(),
            "lock_date".into(),
            "policy_id".into(),
            "transaction_amount_limit".into(),
            "vendor_blacklist".into(),
            "vendor_whitelist".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Receipt {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Unique identifier of the receipt."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Pre-signed url to download receipt image."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    #[doc = "Unique identifier of the associated transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the person who made the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<uuid::Uuid>,
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
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
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
            if let Some(transaction_id) = &self.transaction_id {
                format!("{:?}", transaction_id).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "id".into(),
            "receipt_url".into(),
            "transaction_id".into(),
            "user_id".into(),
        ]
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

#[doc = "current state of the reimbursement."]
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
pub enum ReimbursementState {
    #[serde(rename = "APPROVED")]
    #[display("APPROVED")]
    Approved,
    #[serde(rename = "AWAITING_PAYMENT")]
    #[display("AWAITING_PAYMENT")]
    AwaitingPayment,
    #[serde(rename = "CANCELED")]
    #[display("CANCELED")]
    Canceled,
    #[serde(rename = "DELETED")]
    #[display("DELETED")]
    Deleted,
    #[serde(rename = "DRAFT")]
    #[display("DRAFT")]
    Draft,
    #[serde(rename = "FAILED_REIMBURSEMENT")]
    #[display("FAILED_REIMBURSEMENT")]
    FailedReimbursement,
    #[serde(rename = "INIT")]
    #[display("INIT")]
    Init,
    #[serde(rename = "MANUALLY_REIMBURSED")]
    #[display("MANUALLY_REIMBURSED")]
    ManuallyReimbursed,
    #[serde(rename = "MISSING_ACH")]
    #[display("MISSING_ACH")]
    MissingAch,
    #[serde(rename = "PENDING")]
    #[display("PENDING")]
    Pending,
    #[serde(rename = "PROCESSING")]
    #[display("PROCESSING")]
    Processing,
    #[serde(rename = "REIMBURSED")]
    #[display("REIMBURSED")]
    Reimbursed,
    #[serde(rename = "REJECTED")]
    #[display("REJECTED")]
    Rejected,
}

#[doc = "The type of the reimbursement."]
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
pub enum ReimbursementType {
    #[serde(rename = "MILEAGE")]
    #[display("MILEAGE")]
    Mileage,
    #[serde(rename = "OUT_OF_POCKET")]
    #[display("OUT_OF_POCKET")]
    OutOfPocket,
    #[serde(rename = "PAYBACK_FULL")]
    #[display("PAYBACK_FULL")]
    PaybackFull,
    #[serde(rename = "PAYBACK_PARTIAL")]
    #[display("PAYBACK_PARTIAL")]
    PaybackPartial,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Reimbursement {
    #[doc = "List of accounting fields selected to code the reimbursement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiReimbursementAccountingFieldSelection>>,
    #[doc = "The amount that the payor pays."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "Time at which the reimbursement is approved. Presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Time at which the reimbursement is created. Presented in ISO8601 format."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[doc = "The currency that the payor pays with."]
    pub currency: String,
    #[doc = "The direction of the reimbursement. It could be either BUSINESS_TO_USER or \
             USER_TO_BUSINESS."]
    pub direction: Direction,
    #[doc = "The distance of the reimbursement in miles, for mileage reimbursements."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance: Option<f64>,
    #[doc = "Unique identifier of the associated business entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the reimbursement."]
    pub id: uuid::Uuid,
    #[doc = "List of line items related to the reimbursement."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<ApiReimbursementLineItem>>,
    #[doc = "Reimbursement memo"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant: Option<String>,
    #[doc = "Original reimbursement amount before the currency conversion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_reimbursement_amount: Option<CurrencyAmount>,
    #[doc = "Amount and currency received by the payee."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payee_amount: Option<CurrencyAmount>,
    #[doc = "The unique identifier of the payment batch that the reimbursement is associated \
             with, once paid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<uuid::Uuid>>,
    #[doc = "Spend limit to which the reimbursement is attributed, if it exists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spend_limit_id: Option<uuid::Uuid>,
    #[doc = "current state of the reimbursement."]
    pub state: ReimbursementState,
    #[doc = "Time when reimbursement has been synced. Will be None if the reimbursement is not \
             synced."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_date: Option<chrono::NaiveDate>,
    #[doc = "Trip ID associated with the reimbursement if a Trip ID is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trip_id: Option<uuid::Uuid>,
    #[doc = "The type of the reimbursement."]
    #[serde(rename = "type")]
    pub type_: ReimbursementType,
    #[doc = "Time at which the reimbursement was last updated. Presented in ISO8601 format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Email of the person who made the reimbursement."]
    pub user_email: String,
    #[doc = "Full name of the person who made the reimbursement."]
    pub user_full_name: String,
    #[doc = "Unique identifier of the person who made the reimbursement."]
    pub user_id: uuid::Uuid,
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
    const LENGTH: usize = 26;
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
            if let Some(approved_at) = &self.approved_at {
                format!("{:?}", approved_at).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            self.currency.clone().into(),
            format!("{:?}", self.direction).into(),
            format!("{:?}", self.distance).into(),
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.id).into(),
            if let Some(line_items) = &self.line_items {
                format!("{:?}", line_items).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
            if let Some(merchant) = &self.merchant {
                format!("{:?}", merchant).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.original_reimbursement_amount).into(),
            format!("{:?}", self.payee_amount).into(),
            format!("{:?}", self.payment_id).into(),
            if let Some(receipts) = &self.receipts {
                format!("{:?}", receipts).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.spend_limit_id).into(),
            format!("{:?}", self.state).into(),
            format!("{:?}", self.synced_at).into(),
            if let Some(transaction_date) = &self.transaction_date {
                format!("{:?}", transaction_date).into()
            } else {
                String::new().into()
            },
            if let Some(trip_id) = &self.trip_id {
                format!("{:?}", trip_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.type_).into(),
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            self.user_email.clone().into(),
            self.user_full_name.clone().into(),
            format!("{:?}", self.user_id).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_field_selections".into(),
            "amount".into(),
            "approved_at".into(),
            "created_at".into(),
            "currency".into(),
            "direction".into(),
            "distance".into(),
            "entity_id".into(),
            "id".into(),
            "line_items".into(),
            "memo".into(),
            "merchant".into(),
            "original_reimbursement_amount".into(),
            "payee_amount".into(),
            "payment_id".into(),
            "receipts".into(),
            "spend_limit_id".into(),
            "state".into(),
            "synced_at".into(),
            "transaction_date".into(),
            "trip_id".into(),
            "type_".into(),
            "updated_at".into(),
            "user_email".into(),
            "user_full_name".into(),
            "user_id".into(),
        ]
    }
}

#[doc = "Time interval the limit is applied on."]
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
pub enum SpendIntentApiSpendingRestrictionsDumpInterval {
    #[serde(rename = "ANNUAL")]
    #[display("ANNUAL")]
    Annual,
    #[serde(rename = "CUSTOM")]
    #[display("CUSTOM")]
    Custom,
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
pub struct SpendIntentApiSpendingRestrictionsDump {
    #[doc = " List of Ramp category codes allowed for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<i64>>,
    #[doc = "List of merchants allowed for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_vendors: Option<Vec<uuid::Uuid>>,
    #[doc = "Date to automatically to lock the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_lock_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "List of Ramp category codes blocked for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<i64>>,
    #[doc = "List of merchants  blocked for the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_vendors: Option<Vec<uuid::Uuid>>,
    #[doc = "Time interval the limit is applied on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<SpendIntentApiSpendingRestrictionsDumpInterval>,
    #[doc = "Amount limit total per interval denominated in cents. Currency is USD."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<CurrencyAmount>,
    #[doc = "Date and time for the next interval reset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_interval_reset: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Date and time for the start of the current interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_of_interval: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Temporary limit total per interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub temporary_limit: Option<CurrencyAmount>,
    #[doc = "Max amount allowed on a single transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transaction_amount_limit: Option<CurrencyAmount>,
}

impl std::fmt::Display for SpendIntentApiSpendingRestrictionsDump {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SpendIntentApiSpendingRestrictionsDump {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(allowed_categories) = &self.allowed_categories {
                format!("{:?}", allowed_categories).into()
            } else {
                String::new().into()
            },
            if let Some(allowed_vendors) = &self.allowed_vendors {
                format!("{:?}", allowed_vendors).into()
            } else {
                String::new().into()
            },
            if let Some(auto_lock_date) = &self.auto_lock_date {
                format!("{:?}", auto_lock_date).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_categories) = &self.blocked_categories {
                format!("{:?}", blocked_categories).into()
            } else {
                String::new().into()
            },
            if let Some(blocked_vendors) = &self.blocked_vendors {
                format!("{:?}", blocked_vendors).into()
            } else {
                String::new().into()
            },
            if let Some(interval) = &self.interval {
                format!("{:?}", interval).into()
            } else {
                String::new().into()
            },
            if let Some(limit) = &self.limit {
                format!("{:?}", limit).into()
            } else {
                String::new().into()
            },
            if let Some(next_interval_reset) = &self.next_interval_reset {
                format!("{:?}", next_interval_reset).into()
            } else {
                String::new().into()
            },
            if let Some(start_of_interval) = &self.start_of_interval {
                format!("{:?}", start_of_interval).into()
            } else {
                String::new().into()
            },
            if let Some(temporary_limit) = &self.temporary_limit {
                format!("{:?}", temporary_limit).into()
            } else {
                String::new().into()
            },
            if let Some(transaction_amount_limit) = &self.transaction_amount_limit {
                format!("{:?}", transaction_amount_limit).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "allowed_categories".into(),
            "allowed_vendors".into(),
            "auto_lock_date".into(),
            "blocked_categories".into(),
            "blocked_vendors".into(),
            "interval".into(),
            "limit".into(),
            "next_interval_reset".into(),
            "start_of_interval".into(),
            "temporary_limit".into(),
            "transaction_amount_limit".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SpendLimitDeferredTask {
    #[doc = "Further context for the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<ApiSpendLimitDeferredTaskContext>,
    #[doc = "Detailed data of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ApiSpendLimitDeferredTaskData>,
    #[doc = "Unique identifier of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Status of the deferred task. It could be one of the following values: STARTED, \
             IN_PROGRESS, ERROR, SUCCESS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl std::fmt::Display for SpendLimitDeferredTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SpendLimitDeferredTask {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(context) = &self.context {
                format!("{:?}", context).into()
            } else {
                String::new().into()
            },
            if let Some(data) = &self.data {
                format!("{:?}", data).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "context".into(),
            "data".into(),
            "id".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Statement {
    #[doc = "Breakdown of statement balances by entity and currency."]
    pub balance_sections: Vec<Statement>,
    #[doc = "Total charges incurred during the statement period"]
    #[deprecated]
    pub charges: CurrencyAmount,
    #[doc = "Total credits accumulated during the statement period"]
    #[deprecated]
    pub credits: CurrencyAmount,
    pub end_date: chrono::DateTime<chrono::Utc>,
    #[doc = "Balance at the end of the statement period"]
    #[deprecated]
    pub ending_balance: CurrencyAmount,
    pub id: uuid::Uuid,
    #[doc = "Balance at the beginning of the statement period"]
    #[deprecated]
    pub opening_balance: CurrencyAmount,
    #[doc = "Total payments made during the statement period"]
    #[deprecated]
    pub payments: CurrencyAmount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preceding_statement_id: Option<uuid::Uuid>,
    pub start_date: chrono::DateTime<chrono::Utc>,
    #[doc = "Statement lines encompass all the financial activities, including card transactions, \
             transfers, and other charges or credits that have taken place within a specific \
             period."]
    pub statement_lines: Vec<ApiStatementItem>,
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Statement {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.balance_sections).into(),
            format!("{:?}", self.charges).into(),
            format!("{:?}", self.credits).into(),
            format!("{:?}", self.end_date).into(),
            format!("{:?}", self.ending_balance).into(),
            format!("{:?}", self.id).into(),
            format!("{:?}", self.opening_balance).into(),
            format!("{:?}", self.payments).into(),
            if let Some(preceding_statement_id) = &self.preceding_statement_id {
                format!("{:?}", preceding_statement_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.start_date).into(),
            format!("{:?}", self.statement_lines).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "balance_sections".into(),
            "charges".into(),
            "credits".into(),
            "end_date".into(),
            "ending_balance".into(),
            "id".into(),
            "opening_balance".into(),
            "payments".into(),
            "preceding_statement_id".into(),
            "start_date".into(),
            "statement_lines".into(),
        ]
    }
}

#[doc = "Grant type, must be one of: ['authorization_code', 'client_credentials', \
         'refresh_token']. Refresh token and client credentials grants must be added to app's \
         permitted Grant Types."]
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
pub enum GrantType {
    #[serde(rename = "authorization_code")]
    #[display("authorization_code")]
    AuthorizationCode,
    #[serde(rename = "client_credentials")]
    #[display("client_credentials")]
    ClientCredentials,
    #[serde(rename = "refresh_token")]
    #[display("refresh_token")]
    RefreshToken,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TokenRequestBody {
    #[doc = "Authorization code obtained from request to https://app.ramp.com/v1/authorize, to be \
             exchanged for access token. Field is required if grant_type=authorization_code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Grant type, must be one of: ['authorization_code', 'client_credentials', \
             'refresh_token']. Refresh token and client credentials grants must be added to app's \
             permitted Grant Types."]
    pub grant_type: GrantType,
    #[doc = "The redirect URI previously used in the authorization request, used to verify app. \
             Field is required if grant_type=authorization_code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<String>,
    #[doc = "The refresh token issued to you to get a new access token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "Space-separated list of scopes to be granted to the returned token. These must be a \
             subset (including full set) of the app's allowed scopes. Field is required if \
             grant_type=client_credentials."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

impl std::fmt::Display for TokenRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TokenRequestBody {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.grant_type).into(),
            if let Some(redirect_uri) = &self.redirect_uri {
                format!("{:?}", redirect_uri).into()
            } else {
                String::new().into()
            },
            if let Some(refresh_token) = &self.refresh_token {
                format!("{:?}", refresh_token).into()
            } else {
                String::new().into()
            },
            if let Some(scope) = &self.scope {
                format!("{:?}", scope).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "code".into(),
            "grant_type".into(),
            "redirect_uri".into(),
            "refresh_token".into(),
            "scope".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TokenResponse {
    #[doc = "The access token string. Prefixed with ramp_tok_."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "The duration of which the token is valid, measured in seconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[doc = "Refresh token can be used to get a new access token. Note that refresh token won't \
             appear in the response for client_credentials grant type. Prefixed with ramp_tok_."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "Space-separated list of scopes of the access token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[doc = "The type of the token. This should always be 'Bearer'. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}

impl std::fmt::Display for TokenResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TokenResponse {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(access_token) = &self.access_token {
                format!("{:?}", access_token).into()
            } else {
                String::new().into()
            },
            if let Some(expires_in) = &self.expires_in {
                format!("{:?}", expires_in).into()
            } else {
                String::new().into()
            },
            if let Some(refresh_token) = &self.refresh_token {
                format!("{:?}", refresh_token).into()
            } else {
                String::new().into()
            },
            if let Some(scope) = &self.scope {
                format!("{:?}", scope).into()
            } else {
                String::new().into()
            },
            if let Some(token_type) = &self.token_type {
                format!("{:?}", token_type).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "access_token".into(),
            "expires_in".into(),
            "refresh_token".into(),
            "scope".into(),
            "token_type".into(),
        ]
    }
}

#[doc = "Type of the token to be revoked."]
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
pub enum TokenTypeHint {
    #[serde(rename = "access_token")]
    #[display("access_token")]
    AccessToken,
    #[serde(rename = "refresh_token")]
    #[display("refresh_token")]
    RefreshToken,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TokenRevokeRequestBody {
    #[doc = "Access token or refresh token to be revoked."]
    pub token: String,
    #[doc = "Type of the token to be revoked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type_hint: Option<TokenTypeHint>,
}

impl std::fmt::Display for TokenRevokeRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TokenRevokeRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.token.clone().into(),
            if let Some(token_type_hint) = &self.token_type_hint {
                format!("{:?}", token_type_hint).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["token".into(), "token_type_hint".into()]
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
    #[doc = "[Deprecated - use accounting_field_selections instead] Accounting categories related \
             to the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[deprecated]
    pub accounting_categories: Option<Vec<ApiAccountingCategory>>,
    #[doc = "List of accounting fields selected to code the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accounting_field_selections: Option<Vec<ApiTransactionAccountingFieldSelection>>,
    #[doc = "Settled amount of the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[doc = "Information about the card holder."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<ApiTransactionCardHolder>,
    #[doc = "Identifier of the physical or virtual card associated with the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub card_id: Option<uuid::Uuid>,
    #[doc = "Currency that the transaction is settled in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[doc = "Details about a transaction decline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decline_details: Option<ApiTransactionDeclineDetails>,
    #[doc = "A list of disputes sorted in descending order by their creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disputes: Option<Vec<ApiTransactionDispute>>,
    #[doc = "Unique identifier of business entity associated with the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the spend limit associated with the transaction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_id: Option<uuid::Uuid>,
    #[doc = "List of line items related to the transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<ApiTransactionLineItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[doc = "Merchant category code is a four-digit number in ISP 18245 used to classify a \
             business by the types of goods and services it provides."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_category_code: Option<String>,
    #[doc = "Description about the merchant category code."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_category_code_description: Option<String>,
    #[doc = "Purchase data associated related to a transaction provided by the merchant"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_data: Option<ApiTransactionPurchaseData>,
    #[doc = "A merchant descriptor is the name that appears on a customer's bank statement when \
             they make a purchase from that merchant."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_descriptor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<uuid::Uuid>,
    #[doc = "Card acceptor data such as country, city, state, and postal code if available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_location: Option<ApiMerchantLocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    #[doc = "the original transaction amount before the currency conversion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_transaction_amount: Option<CurrencyAmount>,
    #[doc = "A list of policy violations sorted in descending order by their creation time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_violations: Option<Vec<ApiTransactionPolicyViolation>>,
    #[doc = "Receipts listed in ascending order by their creation time, related to the \
             transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub receipts: Option<Vec<uuid::Uuid>>,
    #[doc = "Time when funds were moved for a transaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Ramp-internal category id."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sk_category_id: Option<i64>,
    #[doc = "Ramp-internal category name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sk_category_name: Option<String>,
    #[doc = "transaction state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<TransactionState>,
    #[doc = "Statement ID associated with the transaction if one is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<uuid::Uuid>,
    #[doc = "Time when transaction has been synced. Will be None if the transaction is not synced."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Trip ID associated with the transaction if one is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trip_id: Option<uuid::Uuid>,
    #[doc = "Trip name associated with the transaction if one is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trip_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_transaction_time: Option<chrono::DateTime<chrono::Utc>>,
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
    const LENGTH: usize = 32;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(accounting_categories) = &self.accounting_categories {
                format!("{:?}", accounting_categories).into()
            } else {
                String::new().into()
            },
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
            if let Some(card_holder) = &self.card_holder {
                format!("{:?}", card_holder).into()
            } else {
                String::new().into()
            },
            if let Some(card_id) = &self.card_id {
                format!("{:?}", card_id).into()
            } else {
                String::new().into()
            },
            if let Some(currency_code) = &self.currency_code {
                format!("{:?}", currency_code).into()
            } else {
                String::new().into()
            },
            if let Some(decline_details) = &self.decline_details {
                format!("{:?}", decline_details).into()
            } else {
                String::new().into()
            },
            if let Some(disputes) = &self.disputes {
                format!("{:?}", disputes).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.entity_id).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(limit_id) = &self.limit_id {
                format!("{:?}", limit_id).into()
            } else {
                String::new().into()
            },
            if let Some(line_items) = &self.line_items {
                format!("{:?}", line_items).into()
            } else {
                String::new().into()
            },
            if let Some(memo) = &self.memo {
                format!("{:?}", memo).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_category_code) = &self.merchant_category_code {
                format!("{:?}", merchant_category_code).into()
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
            if let Some(merchant_data) = &self.merchant_data {
                format!("{:?}", merchant_data).into()
            } else {
                String::new().into()
            },
            if let Some(merchant_descriptor) = &self.merchant_descriptor {
                format!("{:?}", merchant_descriptor).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.merchant_id).into(),
            if let Some(merchant_location) = &self.merchant_location {
                format!("{:?}", merchant_location).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.merchant_name).into(),
            format!("{:?}", self.original_transaction_amount).into(),
            if let Some(policy_violations) = &self.policy_violations {
                format!("{:?}", policy_violations).into()
            } else {
                String::new().into()
            },
            if let Some(receipts) = &self.receipts {
                format!("{:?}", receipts).into()
            } else {
                String::new().into()
            },
            if let Some(settlement_date) = &self.settlement_date {
                format!("{:?}", settlement_date).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.sk_category_id).into(),
            format!("{:?}", self.sk_category_name).into(),
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(statement_id) = &self.statement_id {
                format!("{:?}", statement_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.synced_at).into(),
            if let Some(trip_id) = &self.trip_id {
                format!("{:?}", trip_id).into()
            } else {
                String::new().into()
            },
            if let Some(trip_name) = &self.trip_name {
                format!("{:?}", trip_name).into()
            } else {
                String::new().into()
            },
            if let Some(user_transaction_time) = &self.user_transaction_time {
                format!("{:?}", user_transaction_time).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "accounting_categories".into(),
            "accounting_field_selections".into(),
            "amount".into(),
            "card_holder".into(),
            "card_id".into(),
            "currency_code".into(),
            "decline_details".into(),
            "disputes".into(),
            "entity_id".into(),
            "id".into(),
            "limit_id".into(),
            "line_items".into(),
            "memo".into(),
            "merchant_category_code".into(),
            "merchant_category_code_description".into(),
            "merchant_data".into(),
            "merchant_descriptor".into(),
            "merchant_id".into(),
            "merchant_location".into(),
            "merchant_name".into(),
            "original_transaction_amount".into(),
            "policy_violations".into(),
            "receipts".into(),
            "settlement_date".into(),
            "sk_category_id".into(),
            "sk_category_name".into(),
            "state".into(),
            "statement_id".into(),
            "synced_at".into(),
            "trip_id".into(),
            "trip_name".into(),
            "user_transaction_time".into(),
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
pub enum TransferStatus {
    #[serde(rename = "ACH_CONFIRMED")]
    #[display("ACH_CONFIRMED")]
    AchConfirmed,
    #[serde(rename = "CANCELED")]
    #[display("CANCELED")]
    Canceled,
    #[serde(rename = "COMPLETED")]
    #[display("COMPLETED")]
    Completed,
    #[serde(rename = "ERROR")]
    #[display("ERROR")]
    Error,
    #[serde(rename = "INITIATED")]
    #[display("INITIATED")]
    Initiated,
    #[serde(rename = "NOT_ACKED")]
    #[display("NOT_ACKED")]
    NotAcked,
    #[serde(rename = "NOT_ENOUGH_FUNDS")]
    #[display("NOT_ENOUGH_FUNDS")]
    NotEnoughFunds,
    #[serde(rename = "PROCESSING_BY_ODFI")]
    #[display("PROCESSING_BY_ODFI")]
    ProcessingByOdfi,
    #[serde(rename = "REJECTED_BY_ODFI")]
    #[display("REJECTED_BY_ODFI")]
    RejectedByOdfi,
    #[serde(rename = "RETURNED_BY_RDFI")]
    #[display("RETURNED_BY_RDFI")]
    ReturnedByRdfi,
    #[serde(rename = "SUBMITTED_TO_FED")]
    #[display("SUBMITTED_TO_FED")]
    SubmittedToFed,
    #[serde(rename = "SUBMITTED_TO_RDFI")]
    #[display("SUBMITTED_TO_RDFI")]
    SubmittedToRdfi,
    #[serde(rename = "UNNECESSARY")]
    #[display("UNNECESSARY")]
    Unnecessary,
    #[serde(rename = "UPLOADED")]
    #[display("UPLOADED")]
    Uploaded,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Transfer {
    #[doc = "Amount of the transfer payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<CurrencyAmount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TransferStatus>,
}

impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Transfer {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "amount".into(),
            "created_at".into(),
            "entity_id".into(),
            "id".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Upload {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sales_lead_id: Option<uuid::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
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
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
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
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "created_at".into(),
            "document_type".into(),
            "sales_lead_id".into(),
            "updated_at".into(),
        ]
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
pub enum UserStatus {
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
    #[doc = "Unique identifier of the company that the employee's working for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_id: Option<uuid::Uuid>,
    #[doc = "A list of custom fields of the user."]
    pub custom_fields: Vec<ApiUserCustomField>,
    #[doc = "Unique identifier of the employee's department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<uuid::Uuid>,
    #[doc = "The employee's email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "An alternative identifier for an employee, coming from external systems, which can \
             be used in place of an email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    #[doc = "Unique identifier of business entity user is associated with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "First name of the employee"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Unique employee identifier"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Whether the employee is a manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_manager: Option<bool>,
    #[doc = "Last name of the employee"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Unique identifier of the employee's location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<uuid::Uuid>,
    #[doc = "Unique identifier of the employee's manager"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<uuid::Uuid>,
    #[doc = "The employee's phone number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The employee's role; It could be one of the following values: Admin, Cardholder, \
             Owner, Bookkeeper"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "The employee's status; It could be one of the following values: INVITE_PENDING, \
             INVITE_DELETED, INVITE_EXPIRED, USER_ONBOARDING, USER_ACTIVE and USER_SUSPENDED"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
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
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.business_id).into(),
            format!("{:?}", self.custom_fields).into(),
            format!("{:?}", self.department_id).into(),
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.entity_id).into(),
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(is_manager) = &self.is_manager {
                format!("{:?}", is_manager).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.location_id).into(),
            format!("{:?}", self.manager_id).into(),
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "business_id".into(),
            "custom_fields".into(),
            "department_id".into(),
            "email".into(),
            "employee_id".into(),
            "entity_id".into(),
            "first_name".into(),
            "id".into(),
            "is_manager".into(),
            "last_name".into(),
            "location_id".into(),
            "manager_id".into(),
            "phone".into(),
            "role".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserDeferredTask {
    #[doc = "Further context for the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<ApiUserDeferredTaskContext>,
    #[doc = "Detailed data of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ApiUserDeferredTaskData>,
    #[doc = "Unique identifier of the deferred task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Status of the deferred task. It could be one of the following values: STARTED, \
             IN_PROGRESS, ERROR, SUCCESS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(context) = &self.context {
                format!("{:?}", context).into()
            } else {
                String::new().into()
            },
            if let Some(data) = &self.data {
                format!("{:?}", data).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "context".into(),
            "data".into(),
            "id".into(),
            "status".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Vendor {
    #[doc = "id of the vendor."]
    pub id: String,
    #[doc = "name of the vendor"]
    pub name: String,
}

impl std::fmt::Display for Vendor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Vendor {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.id.clone().into(), self.name.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct VendorAccount {
    #[doc = "e.g. 19566"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[doc = "Date time at which the vendor account was created on Ramp."]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Whether this vendor account is active on Ramp"]
    pub is_active: bool,
    #[doc = "Whether this vendor has been synced to remote ERP."]
    pub is_synced: bool,
    #[doc = "Name of the vendor."]
    pub name: String,
    #[doc = "id to uniquely identify a vendor account within Ramp system"]
    pub ramp_id: uuid::Uuid,
    #[doc = "Date time at which the vendor account was most recently updated."]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for VendorAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for VendorAccount {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.created_at).into(),
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_active).into(),
            format!("{:?}", self.is_synced).into(),
            self.name.clone().into(),
            format!("{:?}", self.ramp_id).into(),
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "code".into(),
            "created_at".into(),
            "id".into(),
            "is_active".into(),
            "is_synced".into(),
            "name".into(),
            "ramp_id".into(),
            "updated_at".into(),
        ]
    }
}

#[doc = "Current state of the limit."]
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
pub enum LimitState {
    #[serde(rename = "ACTIVE")]
    #[display("ACTIVE")]
    Active,
    #[serde(rename = "RAMP_TERMINATED")]
    #[display("RAMP_TERMINATED")]
    RampTerminated,
    #[serde(rename = "SUSPENDED")]
    #[display("SUSPENDED")]
    Suspended,
    #[serde(rename = "TERMINATED")]
    #[display("TERMINATED")]
    Terminated,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Limit {
    #[doc = "Details about the current balance of the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance: Option<ApiSpendAllocationBalance>,
    #[doc = "List of cards linked to this limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cards: Option<Vec<ApiSpendLimitCardResource>>,
    #[doc = "Display name of the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Associated business entity of the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<uuid::Uuid>,
    #[doc = "Indicates if the limit's settings override those of its Spend Program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_program_overridden: Option<bool>,
    #[doc = "Unique identifier of the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Whether this spend limit is shareable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_shareable: Option<bool>,
    #[doc = "Specifies the permitted spend types of the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permitted_spend_types: Option<ApiPermittedSpendTypesDump>,
    #[doc = "Restrictions imposed on this limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<ApiSpendingRestrictionsDump>,
    #[doc = "Unique identifier of the associated Spend Program."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spend_program_id: Option<uuid::Uuid>,
    #[doc = "Current state of the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<LimitState>,
    #[doc = "Suspension (lock) on the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suspension: Option<ApiSuspensionDump>,
    #[doc = "Members who can spend from the limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ApiSpendLimitMember>>,
}

impl std::fmt::Display for Limit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Limit {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(balance) = &self.balance {
                format!("{:?}", balance).into()
            } else {
                String::new().into()
            },
            if let Some(cards) = &self.cards {
                format!("{:?}", cards).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(entity_id) = &self.entity_id {
                format!("{:?}", entity_id).into()
            } else {
                String::new().into()
            },
            if let Some(has_program_overridden) = &self.has_program_overridden {
                format!("{:?}", has_program_overridden).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(is_shareable) = &self.is_shareable {
                format!("{:?}", is_shareable).into()
            } else {
                String::new().into()
            },
            if let Some(permitted_spend_types) = &self.permitted_spend_types {
                format!("{:?}", permitted_spend_types).into()
            } else {
                String::new().into()
            },
            if let Some(restrictions) = &self.restrictions {
                format!("{:?}", restrictions).into()
            } else {
                String::new().into()
            },
            if let Some(spend_program_id) = &self.spend_program_id {
                format!("{:?}", spend_program_id).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(suspension) = &self.suspension {
                format!("{:?}", suspension).into()
            } else {
                String::new().into()
            },
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "balance".into(),
            "cards".into(),
            "display_name".into(),
            "entity_id".into(),
            "has_program_overridden".into(),
            "id".into(),
            "is_shareable".into(),
            "permitted_spend_types".into(),
            "restrictions".into(),
            "spend_program_id".into(),
            "state".into(),
            "suspension".into(),
            "users".into(),
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
pub enum PaymentStatus {
    #[serde(rename = "OPEN")]
    #[display("OPEN")]
    Open,
    #[serde(rename = "PAID")]
    #[display("PAID")]
    Paid,
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
pub enum GetTransferListWithPaginationStatus {
    #[serde(rename = "ACH_CONFIRMED")]
    #[display("ACH_CONFIRMED")]
    AchConfirmed,
    #[serde(rename = "CANCELED")]
    #[display("CANCELED")]
    Canceled,
    #[serde(rename = "COMPLETED")]
    #[display("COMPLETED")]
    Completed,
    #[serde(rename = "ERROR")]
    #[display("ERROR")]
    Error,
    #[serde(rename = "INITIATED")]
    #[display("INITIATED")]
    Initiated,
    #[serde(rename = "NOT_ACKED")]
    #[display("NOT_ACKED")]
    NotAcked,
    #[serde(rename = "NOT_ENOUGH_FUNDS")]
    #[display("NOT_ENOUGH_FUNDS")]
    NotEnoughFunds,
    #[serde(rename = "PROCESSING_BY_ODFI")]
    #[display("PROCESSING_BY_ODFI")]
    ProcessingByOdfi,
    #[serde(rename = "REJECTED_BY_ODFI")]
    #[display("REJECTED_BY_ODFI")]
    RejectedByOdfi,
    #[serde(rename = "RETURNED_BY_RDFI")]
    #[display("RETURNED_BY_RDFI")]
    ReturnedByRdfi,
    #[serde(rename = "SUBMITTED_TO_FED")]
    #[display("SUBMITTED_TO_FED")]
    SubmittedToFed,
    #[serde(rename = "SUBMITTED_TO_RDFI")]
    #[display("SUBMITTED_TO_RDFI")]
    SubmittedToRdfi,
    #[serde(rename = "UNNECESSARY")]
    #[display("UNNECESSARY")]
    Unnecessary,
    #[serde(rename = "UPLOADED")]
    #[display("UPLOADED")]
    Uploaded,
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
pub enum GetListWithPaginationRole {
    #[serde(rename = "ADVISOR_CONSOLE_ADMIN")]
    #[display("ADVISOR_CONSOLE_ADMIN")]
    AdvisorConsoleAdmin,
    #[serde(rename = "ADVISOR_CONSOLE_USER")]
    #[display("ADVISOR_CONSOLE_USER")]
    AdvisorConsoleUser,
    #[serde(rename = "AUDITOR")]
    #[display("AUDITOR")]
    Auditor,
    #[serde(rename = "BILL_PAY_ADMIN")]
    #[display("BILL_PAY_ADMIN")]
    BillPayAdmin,
    #[serde(rename = "BUSINESS_ADMIN")]
    #[display("BUSINESS_ADMIN")]
    BusinessAdmin,
    #[serde(rename = "BUSINESS_BOOKKEEPER")]
    #[display("BUSINESS_BOOKKEEPER")]
    BusinessBookkeeper,
    #[serde(rename = "BUSINESS_OWNER")]
    #[display("BUSINESS_OWNER")]
    BusinessOwner,
    #[serde(rename = "BUSINESS_USER")]
    #[display("BUSINESS_USER")]
    BusinessUser,
    #[serde(rename = "DEVELOPER_ADMIN")]
    #[display("DEVELOPER_ADMIN")]
    DeveloperAdmin,
    #[serde(rename = "GUEST_USER")]
    #[display("GUEST_USER")]
    GuestUser,
    #[serde(rename = "IT_ADMIN")]
    #[display("IT_ADMIN")]
    ItAdmin,
    #[serde(rename = "VENDOR_NETWORK_ADMIN")]
    #[display("VENDOR_NETWORK_ADMIN")]
    VendorNetworkAdmin,
}
