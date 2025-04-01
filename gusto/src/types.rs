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

#[doc = "The representation of an employee in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Employee {
    #[doc = "The ID of the employee in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "A unique identifier of the employee in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The email address of the employee. This is provided to support syncing users between \
             our system and yours. You may not use this email address for any other purpose (e.g. \
             marketing)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The ID of the company the employee is employed by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<f64>,
    #[doc = "A unique identifier of the company the employee is employed by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_uuid: Option<String>,
    #[doc = "The ID of the employee's manager in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<f64>,
    #[doc = "The current version of the employee. See the versioning guide for details using this \
             field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The employee's department in the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[doc = "Whether the employee is terminated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated: Option<bool>,
    #[doc = "Whether the employee is a two percent shareholder of the company. This field only \
             applies to companies with an S-Corp entity type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub two_percent_shareholder: Option<bool>,
    #[doc = "Whether the employee has completed onboarding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eligible_paid_time_off: Option<Vec<PaidTimeOff>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminations: Option<Vec<Termination>>,
    #[doc = "The representation of an address in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_address: Option<Location>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub garnishments: Option<Vec<Garnishment>>,
    #[doc = "Custom fields are only included for the employee if the include param has the \
             custom_fields value set"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<EmployeeCustomField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[doc = "Indicates whether the employee has an SSN in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_ssn: Option<bool>,
    #[doc = "Deprecated. This field always returns an empty string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "preferred_first_name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_first_name: Option<String>,
    #[doc = "The work email address of the employee. This is provided to support syncing users \
             between our system and yours. You may not use this email address for any other \
             purpose (e.g. marketing)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Employee {
    const LENGTH: usize = 26;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(middle_initial) = &self.middle_initial {
                format!("{:?}", middle_initial).into()
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
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            if let Some(company_uuid) = &self.company_uuid {
                format!("{:?}", company_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(manager_id) = &self.manager_id {
                format!("{:?}", manager_id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(department) = &self.department {
                format!("{:?}", department).into()
            } else {
                String::new().into()
            },
            if let Some(terminated) = &self.terminated {
                format!("{:?}", terminated).into()
            } else {
                String::new().into()
            },
            if let Some(two_percent_shareholder) = &self.two_percent_shareholder {
                format!("{:?}", two_percent_shareholder).into()
            } else {
                String::new().into()
            },
            if let Some(onboarded) = &self.onboarded {
                format!("{:?}", onboarded).into()
            } else {
                String::new().into()
            },
            if let Some(jobs) = &self.jobs {
                format!("{:?}", jobs).into()
            } else {
                String::new().into()
            },
            if let Some(eligible_paid_time_off) = &self.eligible_paid_time_off {
                format!("{:?}", eligible_paid_time_off).into()
            } else {
                String::new().into()
            },
            if let Some(terminations) = &self.terminations {
                format!("{:?}", terminations).into()
            } else {
                String::new().into()
            },
            if let Some(home_address) = &self.home_address {
                format!("{:?}", home_address).into()
            } else {
                String::new().into()
            },
            if let Some(garnishments) = &self.garnishments {
                format!("{:?}", garnishments).into()
            } else {
                String::new().into()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_birth) = &self.date_of_birth {
                format!("{:?}", date_of_birth).into()
            } else {
                String::new().into()
            },
            if let Some(has_ssn) = &self.has_ssn {
                format!("{:?}", has_ssn).into()
            } else {
                String::new().into()
            },
            if let Some(ssn) = &self.ssn {
                format!("{:?}", ssn).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_first_name) = &self.preferred_first_name {
                format!("{:?}", preferred_first_name).into()
            } else {
                String::new().into()
            },
            if let Some(work_email) = &self.work_email {
                format!("{:?}", work_email).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "uuid".into(),
            "first_name".into(),
            "middle_initial".into(),
            "last_name".into(),
            "email".into(),
            "company_id".into(),
            "company_uuid".into(),
            "manager_id".into(),
            "version".into(),
            "department".into(),
            "terminated".into(),
            "two_percent_shareholder".into(),
            "onboarded".into(),
            "jobs".into(),
            "eligible_paid_time_off".into(),
            "terminations".into(),
            "home_address".into(),
            "garnishments".into(),
            "custom_fields".into(),
            "date_of_birth".into(),
            "has_ssn".into(),
            "ssn".into(),
            "phone".into(),
            "preferred_first_name".into(),
            "work_email".into(),
        ]
    }
}

#[doc = "The representation of an address in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Location {
    #[doc = "The unique identifier of the location in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID for the company to which the location belongs. Only included if the location \
             belongs to a company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<i64>,
    #[doc = "The ID for the employee to which the location belongs. Only included if the location \
             belongs to an employee."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i64>,
    #[doc = "The phone number for the location. Required for company locations. Optional for \
             employee locations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The status of the location. Inactive locations have been deleted, but may still have \
             historical data associated with them."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "Specifies if the location is the company's mailing address. Only included if the \
             location belongs to a company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<bool>,
    #[doc = "Specifies if the location is the company's filing address. Only included if the \
             location belongs to a company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_address: Option<bool>,
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
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            if let Some(phone_number) = &self.phone_number {
                format!("{:?}", phone_number).into()
            } else {
                String::new().into()
            },
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
            } else {
                String::new().into()
            },
            if let Some(mailing_address) = &self.mailing_address {
                format!("{:?}", mailing_address).into()
            } else {
                String::new().into()
            },
            if let Some(filing_address) = &self.filing_address {
                format!("{:?}", filing_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "version".into(),
            "company_id".into(),
            "employee_id".into(),
            "phone_number".into(),
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "state".into(),
            "zip".into(),
            "country".into(),
            "active".into(),
            "mailing_address".into(),
            "filing_address".into(),
        ]
    }
}

#[doc = "The representation of paid time off in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaidTimeOff {
    #[doc = "The name of the paid time off type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unit the PTO type is accrued in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_unit: Option<String>,
    #[doc = "The number of accrual units accrued per accrual period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_rate: Option<f64>,
    #[doc = "The frequency at which the PTO type is accrued."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_period: Option<String>,
    #[doc = "The number of accrual units accrued."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accrual_balance: Option<String>,
    #[doc = "The maximum number of accrual units allowed. A null value signifies no maximum."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum_accrual_balance: Option<String>,
    #[doc = "Whether the accrual balance is paid to the employee upon termination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_at_termination: Option<bool>,
}

impl std::fmt::Display for PaidTimeOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaidTimeOff {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(accrual_unit) = &self.accrual_unit {
                format!("{:?}", accrual_unit).into()
            } else {
                String::new().into()
            },
            if let Some(accrual_rate) = &self.accrual_rate {
                format!("{:?}", accrual_rate).into()
            } else {
                String::new().into()
            },
            if let Some(accrual_period) = &self.accrual_period {
                format!("{:?}", accrual_period).into()
            } else {
                String::new().into()
            },
            if let Some(accrual_balance) = &self.accrual_balance {
                format!("{:?}", accrual_balance).into()
            } else {
                String::new().into()
            },
            if let Some(maximum_accrual_balance) = &self.maximum_accrual_balance {
                format!("{:?}", maximum_accrual_balance).into()
            } else {
                String::new().into()
            },
            if let Some(paid_at_termination) = &self.paid_at_termination {
                format!("{:?}", paid_at_termination).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "accrual_unit".into(),
            "accrual_rate".into(),
            "accrual_period".into(),
            "accrual_balance".into(),
            "maximum_accrual_balance".into(),
            "paid_at_termination".into(),
        ]
    }
}

#[doc = "Garnishments, or employee deductions, are fixed amounts or percentages deducted from an \
         employee’s pay. They can be deducted a specific number of times or on a recurring basis. \
         Garnishments can also have maximum deductions on a yearly or per-pay-period bases. Common \
         uses for garnishments are court-ordered payments for child support or back taxes. Some \
         companies provide loans to their employees that are repaid via garnishments."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Garnishment {
    #[doc = "The unique identifier of the garnishment in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the employee to which this garnishment belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i64>,
    #[doc = "Whether or not this garnishment is currently active."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The amount of the garnishment. Either a percentage or a fixed dollar amount. \
             Represented as a float, e.g. \"8.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[doc = "The description of the garnishment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the garnishment is court ordered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub court_ordered: Option<bool>,
    #[doc = "The number of times to apply the garnisment. Ignored if recurring is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub times: Option<i64>,
    #[doc = "Whether the garnishment should recur indefinitely."]
    #[serde(default)]
    pub recurring: bool,
    #[doc = "The maximum deduction per annum. A null value indicates no maximum. Represented as a \
             float, e.g. \"200.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_maximum: Option<String>,
    #[doc = "The maximum deduction per pay period. A null value indicates no maximum. Represented \
             as a float, e.g. \"16.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period_maximum: Option<String>,
    #[doc = "Whether the amount should be treated as a percentage to be deducted per pay period."]
    #[serde(default)]
    pub deduct_as_percentage: bool,
}

impl std::fmt::Display for Garnishment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Garnishment {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.active).into(),
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(court_ordered) = &self.court_ordered {
                format!("{:?}", court_ordered).into()
            } else {
                String::new().into()
            },
            if let Some(times) = &self.times {
                format!("{:?}", times).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.recurring).into(),
            if let Some(annual_maximum) = &self.annual_maximum {
                format!("{:?}", annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period_maximum) = &self.pay_period_maximum {
                format!("{:?}", pay_period_maximum).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.deduct_as_percentage).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "version".into(),
            "employee_id".into(),
            "active".into(),
            "amount".into(),
            "description".into(),
            "court_ordered".into(),
            "times".into(),
            "recurring".into(),
            "annual_maximum".into(),
            "pay_period_maximum".into(),
            "deduct_as_percentage".into(),
        ]
    }
}

#[doc = "The representation of a termination in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Termination {
    #[doc = "The unique identifier of the termination in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the employee to which this termination is attached."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i64>,
    #[doc = "Whether the employee's termination has gone into effect."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "The employee's last day of work."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<chrono::NaiveDate>,
    #[doc = "If true, the employee should recieve their final wages via an offcycle payroll. If \
             false, they should recieve their final wages on their current pay schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_termination_payroll: Option<bool>,
}

impl std::fmt::Display for Termination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Termination {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
            } else {
                String::new().into()
            },
            if let Some(effective_date) = &self.effective_date {
                format!("{:?}", effective_date).into()
            } else {
                String::new().into()
            },
            if let Some(run_termination_payroll) = &self.run_termination_payroll {
                format!("{:?}", run_termination_payroll).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "version".into(),
            "employee_id".into(),
            "active".into(),
            "effective_date".into(),
            "run_termination_payroll".into(),
        ]
    }
}

#[doc = "The unit accompanying the compensation rate. If the employee is an owner, rate should be \
         'Paycheck'."]
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
pub enum PaymentUnit {
    Hour,
    Week,
    Month,
    Year,
    Paycheck,
}

#[doc = "The FLSA status for this compensation. Salaried ('Exempt') employees are paid a fixed \
         salary every pay period. Salaried with overtime ('Salaried Nonexempt') employees are paid \
         a fixed salary every pay period, and receive overtime pay when applicable. Hourly \
         ('Nonexempt') employees are paid for the hours they work, and receive overtime pay when \
         applicable. Owners ('Owner') are employees that own at least twenty percent of the \
         company. "]
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
pub enum FlsaStatus {
    Exempt,
    #[serde(rename = "Salaried Nonexempt")]
    #[display("Salaried Nonexempt")]
    SalariedNonexempt,
    Nonexempt,
    Owner,
}

#[doc = "The representation of compensation in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Compensation {
    #[doc = "The unique identifier of the compensation in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the job to which the compensation belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
    #[doc = "The dollar amount paid per payment unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[doc = "The unit accompanying the compensation rate. If the employee is an owner, rate \
             should be 'Paycheck'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_unit: Option<PaymentUnit>,
    #[doc = "The FLSA status for this compensation. Salaried ('Exempt') employees are paid a \
             fixed salary every pay period. Salaried with overtime ('Salaried Nonexempt') \
             employees are paid a fixed salary every pay period, and receive overtime pay when \
             applicable. Hourly ('Nonexempt') employees are paid for the hours they work, and \
             receive overtime pay when applicable. Owners ('Owner') are employees that own at \
             least twenty percent of the company. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flsa_status: Option<FlsaStatus>,
    #[doc = "The effective date for this compensation. For the first compensation, this defaults \
             to the job's hire date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for Compensation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Compensation {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(job_id) = &self.job_id {
                format!("{:?}", job_id).into()
            } else {
                String::new().into()
            },
            if let Some(rate) = &self.rate {
                format!("{:?}", rate).into()
            } else {
                String::new().into()
            },
            if let Some(payment_unit) = &self.payment_unit {
                format!("{:?}", payment_unit).into()
            } else {
                String::new().into()
            },
            if let Some(flsa_status) = &self.flsa_status {
                format!("{:?}", flsa_status).into()
            } else {
                String::new().into()
            },
            if let Some(effective_date) = &self.effective_date {
                format!("{:?}", effective_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "version".into(),
            "job_id".into(),
            "rate".into(),
            "payment_unit".into(),
            "flsa_status".into(),
            "effective_date".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Whether the location of the job is active."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
}

impl std::fmt::Display for JobLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobLocation {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(inactive) = &self.inactive {
                format!("{:?}", inactive).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "state".into(),
            "zip".into(),
            "country".into(),
            "inactive".into(),
        ]
    }
}

#[doc = "The representation of a job in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Job {
    #[doc = "The unique identifier of the job in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the employee to which the job belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<i64>,
    #[doc = "The ID of the job's work location."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<JobLocation>,
    #[doc = "The date when the employee was hired for the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<chrono::NaiveDate>,
    #[doc = "The title for the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Whether this is the employee’s primary job. The value will be set to true unless an \
             existing job exists for the employee."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
    #[doc = "The current compensation rate of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[doc = "The payment unit of the current compensation for the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_unit: Option<String>,
    #[doc = "The ID for the current compensation of the job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_compensation_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensations: Option<Vec<Compensation>>,
}

impl std::fmt::Display for Job {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Job {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(location) = &self.location {
                format!("{:?}", location).into()
            } else {
                String::new().into()
            },
            if let Some(hire_date) = &self.hire_date {
                format!("{:?}", hire_date).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(primary) = &self.primary {
                format!("{:?}", primary).into()
            } else {
                String::new().into()
            },
            if let Some(rate) = &self.rate {
                format!("{:?}", rate).into()
            } else {
                String::new().into()
            },
            if let Some(payment_unit) = &self.payment_unit {
                format!("{:?}", payment_unit).into()
            } else {
                String::new().into()
            },
            if let Some(current_compensation_id) = &self.current_compensation_id {
                format!("{:?}", current_compensation_id).into()
            } else {
                String::new().into()
            },
            if let Some(compensations) = &self.compensations {
                format!("{:?}", compensations).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "version".into(),
            "employee_id".into(),
            "location_id".into(),
            "location".into(),
            "hire_date".into(),
            "title".into(),
            "primary".into(),
            "rate".into(),
            "payment_unit".into(),
            "current_compensation_id".into(),
            "compensations".into(),
        ]
    }
}

#[doc = "The representation of an admin user in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Admin {
    #[doc = "The email of the admin. This is used by the admin to log in to their account."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The first name of the admin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The last name of the admin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl std::fmt::Display for Admin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Admin {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["email".into(), "first_name".into(), "last_name".into()]
    }
}

#[doc = "The tax payer type of the company."]
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
    #[serde(rename = "C-Corporation")]
    #[display("C-Corporation")]
    CCorporation,
    #[serde(rename = "S-Corporation")]
    #[display("S-Corporation")]
    SCorporation,
    #[serde(rename = "Sole proprietor")]
    #[display("Sole proprietor")]
    SoleProprietor,
    #[serde(rename = "LLC")]
    #[display("LLC")]
    Llc,
    #[serde(rename = "LLP")]
    #[display("LLP")]
    Llp,
    #[serde(rename = "Limited partnership")]
    #[display("Limited partnership")]
    LimitedPartnership,
    #[serde(rename = "Co-ownership")]
    #[display("Co-ownership")]
    CoOwnership,
    Association,
    Trusteeship,
    #[serde(rename = "General partnership")]
    #[display("General partnership")]
    GeneralPartnership,
    #[serde(rename = "Joint venture")]
    #[display("Joint venture")]
    JointVenture,
    #[serde(rename = "Non-Profit")]
    #[display("Non-Profit")]
    NonProfit,
}

#[doc = "The Gusto product tier of the company."]
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
pub enum Tier {
    #[serde(rename = "core")]
    #[display("core")]
    Core,
    #[serde(rename = "complete")]
    #[display("complete")]
    Complete,
    #[serde(rename = "concierge")]
    #[display("concierge")]
    Concierge,
    #[serde(rename = "contractor_only")]
    #[display("contractor_only")]
    ContractorOnly,
    #[serde(rename = "basic")]
    #[display("basic")]
    Basic,
}

#[doc = "The status of the company in Gusto. \"Approved\" companies may run payroll with Gusto. \
         \"Not Approved\" companies may not yet run payroll with Gusto. In order to run payroll, \
         the company may need to complete onboarding or contact support. \"Suspended\" companies \
         may not run payroll with Gusto. In order to unsuspend their account, the company must \
         contact support."]
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
pub enum CompanyStatus {
    Approved,
    #[serde(rename = "Not Approved")]
    #[display("Not Approved")]
    NotApproved,
    Suspended,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Hourly {
    #[doc = "The name of the hourly compensation rate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The amount multiplied by the base rate of a job to calculate compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multiple: Option<f64>,
}

impl std::fmt::Display for Hourly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Hourly {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(multiple) = &self.multiple {
                format!("{:?}", multiple).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "multiple".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Fixed {
    #[doc = "The name of the fixed compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Fixed {
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
pub struct CompensationsPaidTimeOff {
    #[doc = "The name of the paid time off type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for CompensationsPaidTimeOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompensationsPaidTimeOff {
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

#[doc = "The available company-wide compensation rates for the company."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Compensations {
    #[doc = "The available hourly compensation rates for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly: Option<Vec<Hourly>>,
    #[doc = "The available fixed compensation rates for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed: Option<Vec<Fixed>>,
    #[doc = "The available types of paid time off for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_time_off: Option<Vec<CompensationsPaidTimeOff>>,
}

impl std::fmt::Display for Compensations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Compensations {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(hourly) = &self.hourly {
                format!("{:?}", hourly).into()
            } else {
                String::new().into()
            },
            if let Some(fixed) = &self.fixed {
                format!("{:?}", fixed).into()
            } else {
                String::new().into()
            },
            if let Some(paid_time_off) = &self.paid_time_off {
                format!("{:?}", paid_time_off).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["hourly".into(), "fixed".into(), "paid_time_off".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct HomeAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl std::fmt::Display for HomeAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for HomeAddress {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "state".into(),
            "zip".into(),
            "country".into(),
        ]
    }
}

#[doc = "The primary signatory of the company."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PrimarySignatory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub home_address: Option<HomeAddress>,
}

impl std::fmt::Display for PrimarySignatory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PrimarySignatory {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(middle_initial) = &self.middle_initial {
                format!("{:?}", middle_initial).into()
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
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(home_address) = &self.home_address {
                format!("{:?}", home_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "middle_initial".into(),
            "last_name".into(),
            "phone".into(),
            "email".into(),
            "home_address".into(),
        ]
    }
}

#[doc = "The primary payroll admin of the company."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PrimaryPayrollAdmin {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl std::fmt::Display for PrimaryPayrollAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PrimaryPayrollAdmin {
    const LENGTH: usize = 4;
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
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
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
        vec![
            "first_name".into(),
            "last_name".into(),
            "phone".into(),
            "email".into(),
        ]
    }
}

#[doc = "The representation of a company in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Company {
    #[doc = "The Federal Employer Identification Number of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[doc = "The tax payer type of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    #[doc = "The Gusto product tier of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier: Option<Tier>,
    #[doc = "Whether or not the company is suspended in Gusto. Suspended companies may not run \
             payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_suspended: Option<bool>,
    #[doc = "The status of the company in Gusto. \"Approved\" companies may run payroll with \
             Gusto. \"Not Approved\" companies may not yet run payroll with Gusto. In order to \
             run payroll, the company may need to complete onboarding or contact support. \
             \"Suspended\" companies may not run payroll with Gusto. In order to unsuspend their \
             account, the company must contact support."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_status: Option<CompanyStatus>,
    #[doc = "The unique identifier of the company in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "A unique identifier of the company in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The name of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The trade name of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    #[doc = "The locations of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
    #[doc = "The available company-wide compensation rates for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensations: Option<Compensations>,
    #[doc = "The primary signatory of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_signatory: Option<PrimarySignatory>,
    #[doc = "The primary payroll admin of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_payroll_admin: Option<PrimaryPayrollAdmin>,
}

impl std::fmt::Display for Company {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Company {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
            if let Some(entity_type) = &self.entity_type {
                format!("{:?}", entity_type).into()
            } else {
                String::new().into()
            },
            if let Some(tier) = &self.tier {
                format!("{:?}", tier).into()
            } else {
                String::new().into()
            },
            if let Some(is_suspended) = &self.is_suspended {
                format!("{:?}", is_suspended).into()
            } else {
                String::new().into()
            },
            if let Some(company_status) = &self.company_status {
                format!("{:?}", company_status).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(trade_name) = &self.trade_name {
                format!("{:?}", trade_name).into()
            } else {
                String::new().into()
            },
            if let Some(locations) = &self.locations {
                format!("{:?}", locations).into()
            } else {
                String::new().into()
            },
            if let Some(compensations) = &self.compensations {
                format!("{:?}", compensations).into()
            } else {
                String::new().into()
            },
            if let Some(primary_signatory) = &self.primary_signatory {
                format!("{:?}", primary_signatory).into()
            } else {
                String::new().into()
            },
            if let Some(primary_payroll_admin) = &self.primary_payroll_admin {
                format!("{:?}", primary_payroll_admin).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "ein".into(),
            "entity_type".into(),
            "tier".into(),
            "is_suspended".into(),
            "company_status".into(),
            "id".into(),
            "uuid".into(),
            "name".into(),
            "trade_name".into(),
            "locations".into(),
            "compensations".into(),
            "primary_signatory".into(),
            "primary_payroll_admin".into(),
        ]
    }
}

#[doc = "The contractor's wage type, either \"Fixed\" or \"Hourly\"."]
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
pub enum WageType {
    Fixed,
    Hourly,
}

#[doc = "The contractor's type, either \"Individual\" or \"Business\". "]
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
    Individual,
    Business,
}

#[doc = "The contractor’s home address."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
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
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "state".into(),
            "zip".into(),
            "country".into(),
        ]
    }
}

#[doc = "The representation of a contractor (individual or business) in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Contractor {
    #[doc = "The unique identifier of the contractor in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "A unique identifier of the employee in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The ID of the company the contractor is employed by."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<f64>,
    #[doc = "The contractor's wage type, either \"Fixed\" or \"Hourly\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_type: Option<WageType>,
    #[doc = "The status of the contractor with the company."]
    #[serde(default)]
    pub is_active: bool,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The contractor's type, either \"Individual\" or \"Business\". "]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[doc = "The contractor’s first name. This attribute is required for “Individual” contractors \
             and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The contractor’s last name. This attribute is required for “Individual” contractors \
             and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The contractor’s middle initial. This attribute is optional for “Individual” \
             contractors and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[doc = "The day when the contractor started working for the company.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[doc = "The name of the contractor business. This attribute is required for “Business” \
             contractors and will be ignored for “Individual” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[doc = "The Federal Employer Identification Number of the contractor business. This \
             attribute is optional for “Business” contractors and will be ignored for \
             “Individual” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[doc = "The contractor’s email address. This attribute is optional for “Individual” \
             contractors and will be ignored for “Business” contractors. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The contractor’s home address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[doc = "The contractor’s hourly rate. This attribute is required if the wage_type is \
             “Hourly”."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<String>,
}

impl std::fmt::Display for Contractor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Contractor {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            if let Some(wage_type) = &self.wage_type {
                format!("{:?}", wage_type).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_active).into(),
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
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
            if let Some(middle_initial) = &self.middle_initial {
                format!("{:?}", middle_initial).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(business_name) = &self.business_name {
                format!("{:?}", business_name).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
            if let Some(hourly_rate) = &self.hourly_rate {
                format!("{:?}", hourly_rate).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "uuid".into(),
            "company_id".into(),
            "wage_type".into(),
            "is_active".into(),
            "version".into(),
            "type_".into(),
            "first_name".into(),
            "last_name".into(),
            "middle_initial".into(),
            "start_date".into(),
            "business_name".into(),
            "ein".into(),
            "email".into(),
            "address".into(),
            "hourly_rate".into(),
        ]
    }
}

#[doc = "The payment method."]
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
    #[serde(rename = "Direct Deposit")]
    #[display("Direct Deposit")]
    DirectDeposit,
    Check,
    #[serde(rename = "Historical Payment")]
    #[display("Historical Payment")]
    HistoricalPayment,
    #[serde(rename = "Correction Payment")]
    #[display("Correction Payment")]
    CorrectionPayment,
}

#[doc = "The wage type for the payment."]
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
pub enum ContractorPaymentWageType {
    Hourly,
    Fixed,
}

#[doc = "The representation of a single contractor payment."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContractorPayment {
    #[doc = "The unique identifier of the contractor payment in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The unique identifier of the contractor in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<f64>,
    #[doc = "The bonus amount in the payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus: Option<String>,
    #[doc = "The payment date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[doc = "The number of hours worked for the payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
    #[doc = "The payment method."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<PaymentMethod>,
    #[doc = "The reimbursement amount in the payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reimbursement: Option<String>,
    #[doc = "The rate per hour worked for the payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<String>,
    #[doc = "The fixed wage of the payment, regardless of hours worked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage: Option<String>,
    #[doc = "The wage type for the payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_type: Option<ContractorPaymentWageType>,
    #[doc = "(hours * hourly_rate) + wage + bonus"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_total: Option<String>,
}

impl std::fmt::Display for ContractorPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ContractorPayment {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(contractor_id) = &self.contractor_id {
                format!("{:?}", contractor_id).into()
            } else {
                String::new().into()
            },
            if let Some(bonus) = &self.bonus {
                format!("{:?}", bonus).into()
            } else {
                String::new().into()
            },
            if let Some(date) = &self.date {
                format!("{:?}", date).into()
            } else {
                String::new().into()
            },
            if let Some(hours) = &self.hours {
                format!("{:?}", hours).into()
            } else {
                String::new().into()
            },
            if let Some(payment_method) = &self.payment_method {
                format!("{:?}", payment_method).into()
            } else {
                String::new().into()
            },
            if let Some(reimbursement) = &self.reimbursement {
                format!("{:?}", reimbursement).into()
            } else {
                String::new().into()
            },
            if let Some(hourly_rate) = &self.hourly_rate {
                format!("{:?}", hourly_rate).into()
            } else {
                String::new().into()
            },
            if let Some(wage) = &self.wage {
                format!("{:?}", wage).into()
            } else {
                String::new().into()
            },
            if let Some(wage_type) = &self.wage_type {
                format!("{:?}", wage_type).into()
            } else {
                String::new().into()
            },
            if let Some(wage_total) = &self.wage_total {
                format!("{:?}", wage_total).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "uuid".into(),
            "contractor_id".into(),
            "bonus".into(),
            "date".into(),
            "hours".into(),
            "payment_method".into(),
            "reimbursement".into(),
            "hourly_rate".into(),
            "wage".into(),
            "wage_type".into(),
            "wage_total".into(),
        ]
    }
}

#[doc = "The wage and reimbursement totals for all contractor payments within a given time period."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Total {
    #[doc = "The total reimbursements for contractor payments within a given time period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reimbursements: Option<String>,
    #[doc = "The total wages for contractor payments within a given time period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wages: Option<String>,
}

impl std::fmt::Display for Total {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Total {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(reimbursements) = &self.reimbursements {
                format!("{:?}", reimbursements).into()
            } else {
                String::new().into()
            },
            if let Some(wages) = &self.wages {
                format!("{:?}", wages).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["reimbursements".into(), "wages".into()]
    }
}

#[doc = "ContractorPayments."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContractorPayments {
    #[doc = "The ID of the contractor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contractor_id: Option<f64>,
    #[doc = "The total remibursements for the contractor within a given time period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reimbursement_total: Option<String>,
    #[doc = "The total wages for the contractor within a given time period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_total: Option<String>,
    #[doc = "The contractor’s payments within a given time period.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payments: Option<Vec<ContractorPayment>>,
}

impl std::fmt::Display for ContractorPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ContractorPayments {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(contractor_id) = &self.contractor_id {
                format!("{:?}", contractor_id).into()
            } else {
                String::new().into()
            },
            if let Some(reimbursement_total) = &self.reimbursement_total {
                format!("{:?}", reimbursement_total).into()
            } else {
                String::new().into()
            },
            if let Some(wage_total) = &self.wage_total {
                format!("{:?}", wage_total).into()
            } else {
                String::new().into()
            },
            if let Some(payments) = &self.payments {
                format!("{:?}", payments).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "contractor_id".into(),
            "reimbursement_total".into(),
            "wage_total".into(),
            "payments".into(),
        ]
    }
}

#[doc = "The representation of the summary of contractor payments for a given company in a given \
         time period."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ContractorPaymentSummary {
    #[doc = "The wage and reimbursement totals for all contractor payments within a given time \
             period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<Total>,
    #[doc = "The individual contractor payments, within a given time period, grouped by \
             contractor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contractor_payments: Option<Vec<ContractorPayments>>,
}

impl std::fmt::Display for ContractorPaymentSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ContractorPaymentSummary {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(total) = &self.total {
                format!("{:?}", total).into()
            } else {
                String::new().into()
            },
            if let Some(contractor_payments) = &self.contractor_payments {
                format!("{:?}", contractor_payments).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["total".into(), "contractor_payments".into()]
    }
}

#[doc = "The status of the time off request."]
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
    #[serde(rename = "pending")]
    #[display("pending")]
    Pending,
    #[serde(rename = "approved")]
    #[display("approved")]
    Approved,
    #[serde(rename = "denied")]
    #[display("denied")]
    Denied,
    #[serde(rename = "consumed")]
    #[display("consumed")]
    Consumed,
    #[serde(rename = "deleted")]
    #[display("deleted")]
    Deleted,
}

#[doc = "The type of time off request."]
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
pub enum RequestType {
    #[serde(rename = "parental_leave")]
    #[display("parental_leave")]
    ParentalLeave,
    #[serde(rename = "vacation")]
    #[display("vacation")]
    Vacation,
    #[serde(rename = "sick")]
    #[display("sick")]
    Sick,
}

#[doc = "TimeOffRequestEmployee."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeOffRequestEmployee {
    #[doc = "The ID of the employee the time off request is for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The full name of the employee the time off request is for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

impl std::fmt::Display for TimeOffRequestEmployee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeOffRequestEmployee {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(full_name) = &self.full_name {
                format!("{:?}", full_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["uuid".into(), "full_name".into()]
    }
}

#[doc = "Initiator."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Initiator {
    #[doc = "The ID of the employee who initiated the time off request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The full name of the employee who initiated the time off request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

impl std::fmt::Display for Initiator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Initiator {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(full_name) = &self.full_name {
                format!("{:?}", full_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["uuid".into(), "full_name".into()]
    }
}

#[doc = "This value will be null if the request has not been approved."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Approver {
    #[doc = "The ID of the employee who approved the time off request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The full name of the employee who approved the time off request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
}

impl std::fmt::Display for Approver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Approver {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(full_name) = &self.full_name {
                format!("{:?}", full_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["uuid".into(), "full_name".into()]
    }
}

#[doc = "The representation of a time off request. "]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeOffRequest {
    #[doc = "The ID of the time off request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The status of the time off request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "A note about the time off request, from the employee to the employer."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_note: Option<String>,
    #[doc = "A note about the time off request, from the employer to the employee."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer_note: Option<String>,
    #[doc = "The type of time off request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_type: Option<RequestType>,
    #[doc = "An object that represents the days in the time off request. The keys of the object \
             are the dates, formatted as a YYYY-MM-DD string. The values of the object are the \
             number of hours requested off for each day, formatted as a string representation of \
             a numeric decimal to the thousands place."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days: Option<std::collections::HashMap<String, String>>,
    #[doc = "employee."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee: Option<TimeOffRequestEmployee>,
    #[doc = "initiator."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[doc = "This value will be null if the request has not been approved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver: Option<Approver>,
}

impl std::fmt::Display for TimeOffRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeOffRequest {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(employee_note) = &self.employee_note {
                format!("{:?}", employee_note).into()
            } else {
                String::new().into()
            },
            if let Some(employer_note) = &self.employer_note {
                format!("{:?}", employer_note).into()
            } else {
                String::new().into()
            },
            if let Some(request_type) = &self.request_type {
                format!("{:?}", request_type).into()
            } else {
                String::new().into()
            },
            if let Some(days) = &self.days {
                format!("{:?}", days).into()
            } else {
                String::new().into()
            },
            if let Some(employee) = &self.employee {
                format!("{:?}", employee).into()
            } else {
                String::new().into()
            },
            if let Some(initiator) = &self.initiator {
                format!("{:?}", initiator).into()
            } else {
                String::new().into()
            },
            if let Some(approver) = &self.approver {
                format!("{:?}", approver).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "uuid".into(),
            "status".into(),
            "employee_note".into(),
            "employer_note".into(),
            "request_type".into(),
            "days".into(),
            "employee".into(),
            "initiator".into(),
            "approver".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Companies {
    #[doc = "The ID of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "The name of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The trade name of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    #[doc = "A list of the company locations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<Location>>,
}

impl std::fmt::Display for Companies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Companies {
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
            if let Some(trade_name) = &self.trade_name {
                format!("{:?}", trade_name).into()
            } else {
                String::new().into()
            },
            if let Some(locations) = &self.locations {
                format!("{:?}", locations).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "trade_name".into(),
            "locations".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayrollAdmin {
    #[doc = "A lists of companies for which the current user has admin permissions. Users (most \
             notably accountants) can have priviliges with multiple companies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub companies: Option<Vec<Companies>>,
}

impl std::fmt::Display for PayrollAdmin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PayrollAdmin {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(companies) = &self.companies {
            format!("{:?}", companies).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["companies".into()]
    }
}

#[doc = "An object containing each of the user's permissions."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Roles {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_admin: Option<PayrollAdmin>,
}

impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Roles {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(payroll_admin) = &self.payroll_admin {
            format!("{:?}", payroll_admin).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["payroll_admin".into()]
    }
}

#[doc = "CurrentUser."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CurrentUser {
    #[doc = "The ID of the current user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "The email address of the authenticated user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "An object containing each of the user's permissions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Roles>,
}

impl std::fmt::Display for CurrentUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CurrentUser {
    const LENGTH: usize = 3;
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
            if let Some(roles) = &self.roles {
                format!("{:?}", roles).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "email".into(), "roles".into()]
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
pub enum Frequency {
    #[serde(rename = "Twice per month")]
    #[display("Twice per month")]
    TwicePerMonth,
    Monthly,
    #[serde(rename = "Every other week")]
    #[display("Every other week")]
    EveryOtherWeek,
    #[serde(rename = "Every week")]
    #[display("Every week")]
    EveryWeek,
}

#[doc = "The representation of a pay schedule."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PaySchedule {
    #[doc = "The identifier of the pay schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The unique identifier of the pay schedule in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[doc = "The first date that employees on this pay schedule are paid with Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_pay_date: Option<String>,
    #[doc = "An integer between 1 and 31 indicating the first day of the month that employees are \
             paid. This field is only relevant for pay schedules with the “Twice per month” and \
             “Monthly” frequencies. It will be null for pay schedules with other frequencies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_1: Option<i64>,
    #[doc = "An integer between 1 and 31 indicating the second day of the month that employees \
             are paid. This field is the second pay date for pay schedules with the “Twice per \
             month” frequency. It will be null for pay schedules with other frequencies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_2: Option<i64>,
    #[doc = "Hourly when the pay schedule is for hourly employees. Salaried when the pay schedule \
             is for salaried employees. It will be null when the pay schedule is for all \
             employees."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "With Autopilot® enabled, payroll will run automatically one day before your payroll \
             deadlines."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_pilot: Option<bool>,
}

impl std::fmt::Display for PaySchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PaySchedule {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(frequency) = &self.frequency {
                format!("{:?}", frequency).into()
            } else {
                String::new().into()
            },
            if let Some(anchor_pay_date) = &self.anchor_pay_date {
                format!("{:?}", anchor_pay_date).into()
            } else {
                String::new().into()
            },
            if let Some(day_1) = &self.day_1 {
                format!("{:?}", day_1).into()
            } else {
                String::new().into()
            },
            if let Some(day_2) = &self.day_2 {
                format!("{:?}", day_2).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(auto_pilot) = &self.auto_pilot {
                format!("{:?}", auto_pilot).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "uuid".into(),
            "frequency".into(),
            "anchor_pay_date".into(),
            "day_1".into(),
            "day_2".into(),
            "name".into(),
            "auto_pilot".into(),
        ]
    }
}

#[doc = "Bank account type"]
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
pub enum AccountType {
    Checking,
    Savings,
}

#[doc = "The verification status of the bank account.\n\n'awaiting_deposits' means the bank \
         account is just created and money is being transferred.\n'ready_for_verification' means \
         the micro-deposits are completed and the verification process can begin by using the \
         verify endpoint.\n'verified' means the bank account is verified."]
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
pub enum VerificationStatus {
    #[serde(rename = "awaiting_deposits")]
    #[display("awaiting_deposits")]
    AwaitingDeposits,
    #[serde(rename = "ready_for_verification")]
    #[display("ready_for_verification")]
    ReadyForVerification,
    #[serde(rename = "verified")]
    #[display("verified")]
    Verified,
}

#[doc = "The company bank account"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyBankAccount {
    #[doc = "UUID of the bank account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "UUID of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_uuid: Option<String>,
    #[doc = "Bank account type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    #[doc = "The bank account's routing number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[doc = "masked bank account number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_account_number: Option<String>,
    #[doc = "The verification status of the bank account.\n\n'awaiting_deposits' means the bank \
             account is just created and money is being transferred.\n'ready_for_verification' \
             means the micro-deposits are completed and the verification process can begin by \
             using the verify endpoint.\n'verified' means the bank account is verified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_status: Option<VerificationStatus>,
}

impl std::fmt::Display for CompanyBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyBankAccount {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
            if let Some(company_uuid) = &self.company_uuid {
                format!("{:?}", company_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(account_type) = &self.account_type {
                format!("{:?}", account_type).into()
            } else {
                String::new().into()
            },
            if let Some(routing_number) = &self.routing_number {
                format!("{:?}", routing_number).into()
            } else {
                String::new().into()
            },
            if let Some(hidden_account_number) = &self.hidden_account_number {
                format!("{:?}", hidden_account_number).into()
            } else {
                String::new().into()
            },
            if let Some(verification_status) = &self.verification_status {
                format!("{:?}", verification_status).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "uuid".into(),
            "company_uuid".into(),
            "account_type".into(),
            "routing_number".into(),
            "hidden_account_number".into(),
            "verification_status".into(),
        ]
    }
}

#[doc = "SupportedBenefit."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SupportedBenefit {
    #[doc = "The ID of the benefit type in Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "The name of the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The description of the benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the benefit is deducted before tax calculations, thus reducing one’s taxable \
             income"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pretax: Option<bool>,
    #[doc = "Whether the benefit is deducted after tax calculations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub posttax: Option<bool>,
    #[doc = "Whether the benefit is considered imputed income."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imputed: Option<bool>,
    #[doc = "Whether the benefit is healthcare related."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub healthcare: Option<bool>,
    #[doc = "Whether the benefit is associated with retirement planning."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retirement: Option<bool>,
    #[doc = "Whether the benefit has a government mandated yearly limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yearly_limit: Option<bool>,
}

impl std::fmt::Display for SupportedBenefit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SupportedBenefit {
    const LENGTH: usize = 9;
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
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(pretax) = &self.pretax {
                format!("{:?}", pretax).into()
            } else {
                String::new().into()
            },
            if let Some(posttax) = &self.posttax {
                format!("{:?}", posttax).into()
            } else {
                String::new().into()
            },
            if let Some(imputed) = &self.imputed {
                format!("{:?}", imputed).into()
            } else {
                String::new().into()
            },
            if let Some(healthcare) = &self.healthcare {
                format!("{:?}", healthcare).into()
            } else {
                String::new().into()
            },
            if let Some(retirement) = &self.retirement {
                format!("{:?}", retirement).into()
            } else {
                String::new().into()
            },
            if let Some(yearly_limit) = &self.yearly_limit {
                format!("{:?}", yearly_limit).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "description".into(),
            "pretax".into(),
            "posttax".into(),
            "imputed".into(),
            "healthcare".into(),
            "retirement".into(),
            "yearly_limit".into(),
        ]
    }
}

#[doc = "The representation of a company benefit."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyBenefit {
    #[doc = "The ID of the company benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the company to which the company benefit belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<f64>,
    #[doc = "The ID of the benefitt to which the company benefit belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benefit_id: Option<f64>,
    #[doc = "Whether this benefit is active for employee participation. Company benefits may only \
             be deactivated if no employees are actively participating."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The description of the company benefit.For example, a company may offer multiple \
             benefits with an ID of 1 (for Medical Insurance). The description would show \
             something more specific like “Kaiser Permanente” or “Blue Cross/ Blue Shield”."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether employee deductions and company contributions can be set as percentages of \
             payroll for an individual employee. This is determined by the type of benefit and is \
             not configurable by the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supports_percentage_amounts: Option<bool>,
    #[doc = "Whether the employer is subject to pay employer taxes when an employee is on leave. \
             Only applicable to third party sick pay benefits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_for_employer_taxes: Option<bool>,
    #[doc = "Whether the employer is subject to file W-2 forms for an employee on leave. Only \
             applicable to third party sick pay benefits."]
    #[serde(
        rename = "responsible_for_employee_w2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_for_employee_w_2: Option<bool>,
}

impl std::fmt::Display for CompanyBenefit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyBenefit {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            if let Some(benefit_id) = &self.benefit_id {
                format!("{:?}", benefit_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.active).into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(supports_percentage_amounts) = &self.supports_percentage_amounts {
                format!("{:?}", supports_percentage_amounts).into()
            } else {
                String::new().into()
            },
            if let Some(responsible_for_employer_taxes) = &self.responsible_for_employer_taxes {
                format!("{:?}", responsible_for_employer_taxes).into()
            } else {
                String::new().into()
            },
            if let Some(responsible_for_employee_w_2) = &self.responsible_for_employee_w_2 {
                format!("{:?}", responsible_for_employee_w_2).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "version".into(),
            "company_id".into(),
            "benefit_id".into(),
            "active".into(),
            "description".into(),
            "supports_percentage_amounts".into(),
            "responsible_for_employer_taxes".into(),
            "responsible_for_employee_w_2".into(),
        ]
    }
}

#[doc = "EarningType."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EarningType {
    #[doc = "The name of the earning type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The ID of the earning type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

impl std::fmt::Display for EarningType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EarningType {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "uuid".into()]
    }
}

#[doc = "Whether the employee deduction reduces taxable income or not. Only valid for Group Term \
         Life benefits. Note: when the value is not \"unset\", coverage amount and coverage salary \
         multiplier are ignored."]
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
pub enum DeductionReducesTaxableIncome {
    #[serde(rename = "unset")]
    #[display("unset")]
    Unset,
    #[serde(rename = "reduces_taxable_income")]
    #[display("reduces_taxable_income")]
    ReducesTaxableIncome,
    #[serde(rename = "does_not_reduce_taxable_income")]
    #[display("does_not_reduce_taxable_income")]
    DoesNotReduceTaxableIncome,
}

impl std::default::Default for DeductionReducesTaxableIncome {
    fn default() -> Self {
        DeductionReducesTaxableIncome::Unset
    }
}

#[doc = "The representation of an employee benefit."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmployeeBenefit {
    #[doc = "The ID of the employee benefit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The ID of the employee to which the benefit belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<f64>,
    #[doc = "The ID of the company to which the benefit belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_benefit_id: Option<f64>,
    #[doc = "Whether the employee benefit is active."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The amount to be deducted, per pay period, from the employee's pay."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_deduction: Option<String>,
    #[doc = "The amount to be paid, per pay period, by the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_contribution: Option<String>,
    #[doc = "The maximum employee deduction amount per year. A null value signifies no limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_deduction_annual_maximum: Option<String>,
    #[doc = "The maximum company contribution amount per year. A null value signifies no limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_contribution_annual_maximum: Option<String>,
    #[doc = "Some benefits require additional information to determine their limit. For example, \
             for an HSA benefit, the limit option should be either \"Family\" or \"Individual\". \
             For a Dependent Care FSA benefit, the limit option should be either \"Joint Filing \
             or Single\" or \"Married and Filing Separately\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_option: Option<String>,
    #[doc = "Whether the employee deduction amount should be treated as a percentage to be \
             deducted from each payroll."]
    #[serde(default)]
    pub deduct_as_percentage: bool,
    #[doc = "Whether the company contribution amount should be treated as a percentage to be \
             deducted from each payroll."]
    #[serde(default)]
    pub contribute_as_percentage: bool,
    #[doc = "Whether the employee should use a benefit’s \"catch up\" rate. Only Roth 401k and \
             401k benefits use this value for employees over 50."]
    #[serde(default)]
    pub catch_up: bool,
    #[doc = "The amount that the employee is insured for. Note: company contribution cannot be \
             present if coverage amount is set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_amount: Option<String>,
    #[doc = "Whether the employee deduction reduces taxable income or not. Only valid for Group \
             Term Life benefits. Note: when the value is not \"unset\", coverage amount and \
             coverage salary multiplier are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduction_reduces_taxable_income: Option<DeductionReducesTaxableIncome>,
    #[doc = "The coverage amount as a multiple of the employee’s salary. Only applicable for \
             Group Term Life benefits. Note: cannot be set if coverage amount is also set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_salary_multiplier: Option<String>,
}

impl std::fmt::Display for EmployeeBenefit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmployeeBenefit {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            if let Some(company_benefit_id) = &self.company_benefit_id {
                format!("{:?}", company_benefit_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.active).into(),
            if let Some(employee_deduction) = &self.employee_deduction {
                format!("{:?}", employee_deduction).into()
            } else {
                String::new().into()
            },
            if let Some(company_contribution) = &self.company_contribution {
                format!("{:?}", company_contribution).into()
            } else {
                String::new().into()
            },
            if let Some(employee_deduction_annual_maximum) = &self.employee_deduction_annual_maximum
            {
                format!("{:?}", employee_deduction_annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(company_contribution_annual_maximum) =
                &self.company_contribution_annual_maximum
            {
                format!("{:?}", company_contribution_annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(limit_option) = &self.limit_option {
                format!("{:?}", limit_option).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.deduct_as_percentage).into(),
            format!("{:?}", self.contribute_as_percentage).into(),
            format!("{:?}", self.catch_up).into(),
            if let Some(coverage_amount) = &self.coverage_amount {
                format!("{:?}", coverage_amount).into()
            } else {
                String::new().into()
            },
            if let Some(deduction_reduces_taxable_income) = &self.deduction_reduces_taxable_income {
                format!("{:?}", deduction_reduces_taxable_income).into()
            } else {
                String::new().into()
            },
            if let Some(coverage_salary_multiplier) = &self.coverage_salary_multiplier {
                format!("{:?}", coverage_salary_multiplier).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "version".into(),
            "employee_id".into(),
            "company_benefit_id".into(),
            "active".into(),
            "employee_deduction".into(),
            "company_contribution".into(),
            "employee_deduction_annual_maximum".into(),
            "company_contribution_annual_maximum".into(),
            "limit_option".into(),
            "deduct_as_percentage".into(),
            "contribute_as_percentage".into(),
            "catch_up".into(),
            "coverage_amount".into(),
            "deduction_reduces_taxable_income".into(),
            "coverage_salary_multiplier".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EligibleEmployees {
    #[doc = "The ID of the employee that is eligible for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<f64>,
    #[doc = "The employee's job IDs that are eligible for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_ids: Option<Vec<f64>>,
}

impl std::fmt::Display for EligibleEmployees {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EligibleEmployees {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(job_ids) = &self.job_ids {
                format!("{:?}", job_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "job_ids".into()]
    }
}

#[doc = "Information about the payroll for the pay period."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayPeriodPayroll {
    #[doc = "Whether or not the payroll has been successfully processed. Note that processed \
             payrolls cannot be updated. Additionally, a payroll is not guaranteed to be \
             processed just because the payroll deadline has passed. Late payrolls are not \
             uncommon. Conversely, users may choose to run payroll before the payroll deadline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<bool>,
    #[doc = "The date by which payroll should be run for employees to be paid on time. Payroll \
             data, such as time and attendance data, should be submitted on or before this date. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_deadline: Option<String>,
}

impl std::fmt::Display for PayPeriodPayroll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PayPeriodPayroll {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(processed) = &self.processed {
                format!("{:?}", processed).into()
            } else {
                String::new().into()
            },
            if let Some(payroll_deadline) = &self.payroll_deadline {
                format!("{:?}", payroll_deadline).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["processed".into(), "payroll_deadline".into()]
    }
}

#[doc = "The representation of a pay period."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayPeriod {
    #[doc = "The start date, inclusive, of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[doc = "The end date, inclusive, of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[doc = "The ID of the pay schedule to which the pay period belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_schedule_id: Option<f64>,
    #[doc = "A unique identifier of the pay schedule to which the pay period belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_schedule_uuid: Option<String>,
    #[doc = "The employees who are (or were) eligible during the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eligible_employees: Option<Vec<EligibleEmployees>>,
    #[doc = "Information about the payroll for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll: Option<PayPeriodPayroll>,
}

impl std::fmt::Display for PayPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PayPeriod {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(end_date) = &self.end_date {
                format!("{:?}", end_date).into()
            } else {
                String::new().into()
            },
            if let Some(pay_schedule_id) = &self.pay_schedule_id {
                format!("{:?}", pay_schedule_id).into()
            } else {
                String::new().into()
            },
            if let Some(pay_schedule_uuid) = &self.pay_schedule_uuid {
                format!("{:?}", pay_schedule_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(eligible_employees) = &self.eligible_employees {
                format!("{:?}", eligible_employees).into()
            } else {
                String::new().into()
            },
            if let Some(payroll) = &self.payroll {
                format!("{:?}", payroll).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_date".into(),
            "end_date".into(),
            "pay_schedule_id".into(),
            "pay_schedule_uuid".into(),
            "eligible_employees".into(),
            "payroll".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayrollPayPeriod {
    #[doc = "The start date, inclusive, of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[doc = "The start date, inclusive, of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[doc = "The ID of the pay schedule for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_schedule_id: Option<f64>,
    #[doc = "A unique identifier of the pay schedule for the payroll.."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_schedule_uuid: Option<String>,
}

impl std::fmt::Display for PayrollPayPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PayrollPayPeriod {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(end_date) = &self.end_date {
                format!("{:?}", end_date).into()
            } else {
                String::new().into()
            },
            if let Some(pay_schedule_id) = &self.pay_schedule_id {
                format!("{:?}", pay_schedule_id).into()
            } else {
                String::new().into()
            },
            if let Some(pay_schedule_uuid) = &self.pay_schedule_uuid {
                format!("{:?}", pay_schedule_uuid).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_date".into(),
            "end_date".into(),
            "pay_schedule_id".into(),
            "pay_schedule_uuid".into(),
        ]
    }
}

#[doc = "The subtotals for the payroll."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Totals {
    #[doc = "The total company debit for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_debit: Option<String>,
    #[doc = "The total company net pay for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_pay_debit: Option<String>,
    #[doc = "The total tax debit for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_debit: Option<String>,
    #[doc = "The total reimbursement debit for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reimbursement_debit: Option<String>,
    #[doc = "The total child support debit for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub child_support_debit: Option<String>,
    #[doc = "The total reimbursements for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reimbursements: Option<String>,
    #[doc = "The net pay amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_pay: Option<String>,
    #[doc = "The gross pay amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_pay: Option<String>,
    #[doc = "The total employee bonuses amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_bonuses: Option<String>,
    #[doc = "The total employee commissions amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_commissions: Option<String>,
    #[doc = "The total employee cash tips amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_cash_tips: Option<String>,
    #[doc = "The total employee paycheck tips amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_paycheck_tips: Option<String>,
    #[doc = "The total additional earnings amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_earnings: Option<String>,
    #[doc = "The total owner's draw for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owners_draw: Option<String>,
    #[doc = "The total check amount for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_amount: Option<String>,
    #[doc = "The total amount of employer paid taxes for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employer_taxes: Option<String>,
    #[doc = "The total amount of employee paid taxes for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_taxes: Option<String>,
    #[doc = "The total amount of company contributed benefits for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benefits: Option<String>,
    #[doc = "The total amount of employee deducted benefits for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_benefits_deductions: Option<String>,
    #[doc = "The total amount of payroll taxes deferred for the payroll, such as allowed by the \
             CARES act."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred_payroll_taxes: Option<String>,
}

impl std::fmt::Display for Totals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Totals {
    const LENGTH: usize = 20;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company_debit) = &self.company_debit {
                format!("{:?}", company_debit).into()
            } else {
                String::new().into()
            },
            if let Some(net_pay_debit) = &self.net_pay_debit {
                format!("{:?}", net_pay_debit).into()
            } else {
                String::new().into()
            },
            if let Some(tax_debit) = &self.tax_debit {
                format!("{:?}", tax_debit).into()
            } else {
                String::new().into()
            },
            if let Some(reimbursement_debit) = &self.reimbursement_debit {
                format!("{:?}", reimbursement_debit).into()
            } else {
                String::new().into()
            },
            if let Some(child_support_debit) = &self.child_support_debit {
                format!("{:?}", child_support_debit).into()
            } else {
                String::new().into()
            },
            if let Some(reimbursements) = &self.reimbursements {
                format!("{:?}", reimbursements).into()
            } else {
                String::new().into()
            },
            if let Some(net_pay) = &self.net_pay {
                format!("{:?}", net_pay).into()
            } else {
                String::new().into()
            },
            if let Some(gross_pay) = &self.gross_pay {
                format!("{:?}", gross_pay).into()
            } else {
                String::new().into()
            },
            if let Some(employee_bonuses) = &self.employee_bonuses {
                format!("{:?}", employee_bonuses).into()
            } else {
                String::new().into()
            },
            if let Some(employee_commissions) = &self.employee_commissions {
                format!("{:?}", employee_commissions).into()
            } else {
                String::new().into()
            },
            if let Some(employee_cash_tips) = &self.employee_cash_tips {
                format!("{:?}", employee_cash_tips).into()
            } else {
                String::new().into()
            },
            if let Some(employee_paycheck_tips) = &self.employee_paycheck_tips {
                format!("{:?}", employee_paycheck_tips).into()
            } else {
                String::new().into()
            },
            if let Some(additional_earnings) = &self.additional_earnings {
                format!("{:?}", additional_earnings).into()
            } else {
                String::new().into()
            },
            if let Some(owners_draw) = &self.owners_draw {
                format!("{:?}", owners_draw).into()
            } else {
                String::new().into()
            },
            if let Some(check_amount) = &self.check_amount {
                format!("{:?}", check_amount).into()
            } else {
                String::new().into()
            },
            if let Some(employer_taxes) = &self.employer_taxes {
                format!("{:?}", employer_taxes).into()
            } else {
                String::new().into()
            },
            if let Some(employee_taxes) = &self.employee_taxes {
                format!("{:?}", employee_taxes).into()
            } else {
                String::new().into()
            },
            if let Some(benefits) = &self.benefits {
                format!("{:?}", benefits).into()
            } else {
                String::new().into()
            },
            if let Some(employee_benefits_deductions) = &self.employee_benefits_deductions {
                format!("{:?}", employee_benefits_deductions).into()
            } else {
                String::new().into()
            },
            if let Some(deferred_payroll_taxes) = &self.deferred_payroll_taxes {
                format!("{:?}", deferred_payroll_taxes).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company_debit".into(),
            "net_pay_debit".into(),
            "tax_debit".into(),
            "reimbursement_debit".into(),
            "child_support_debit".into(),
            "reimbursements".into(),
            "net_pay".into(),
            "gross_pay".into(),
            "employee_bonuses".into(),
            "employee_commissions".into(),
            "employee_cash_tips".into(),
            "employee_paycheck_tips".into(),
            "additional_earnings".into(),
            "owners_draw".into(),
            "check_amount".into(),
            "employer_taxes".into(),
            "employee_taxes".into(),
            "benefits".into(),
            "employee_benefits_deductions".into(),
            "deferred_payroll_taxes".into(),
        ]
    }
}

#[doc = "The employee's compensation payment method. This value is only available for processed \
         payrolls."]
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
pub enum EmployeeCompensationsPaymentMethod {
    Check,
    #[serde(rename = "Direct Deposit")]
    #[display("Direct Deposit")]
    DirectDeposit,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FixedCompensations {
    #[doc = "The name of the compensation. This also serves as the unique, immutable identifier \
             for this compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The amount of the compensation for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[doc = "The ID of the job for the compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<f64>,
}

impl std::fmt::Display for FixedCompensations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FixedCompensations {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(job_id) = &self.job_id {
                format!("{:?}", job_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "amount".into(), "job_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct HourlyCompensations {
    #[doc = "The name of the compensation. This also serves as the unique, immutable identifier \
             for this compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of hours to be compensated for this pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
    #[doc = "The ID of the job for the compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<f64>,
    #[doc = "The amount multiplied by the base rate to calculate total compensation per hour \
             worked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_multiplier: Option<f64>,
}

impl std::fmt::Display for HourlyCompensations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for HourlyCompensations {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(hours) = &self.hours {
                format!("{:?}", hours).into()
            } else {
                String::new().into()
            },
            if let Some(job_id) = &self.job_id {
                format!("{:?}", job_id).into()
            } else {
                String::new().into()
            },
            if let Some(compensation_multiplier) = &self.compensation_multiplier {
                format!("{:?}", compensation_multiplier).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "hours".into(),
            "job_id".into(),
            "compensation_multiplier".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmployeeCompensationsPaidTimeOff {
    #[doc = "The name of the PTO. This also serves as the unique, immutable identifier for the \
             PTO."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hours of this PTO taken during the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
}

impl std::fmt::Display for EmployeeCompensationsPaidTimeOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmployeeCompensationsPaidTimeOff {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(hours) = &self.hours {
                format!("{:?}", hours).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "hours".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Benefits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_deduction: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_contribution: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imputed: Option<bool>,
}

impl std::fmt::Display for Benefits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Benefits {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(employee_deduction) = &self.employee_deduction {
                format!("{:?}", employee_deduction).into()
            } else {
                String::new().into()
            },
            if let Some(company_contribution) = &self.company_contribution {
                format!("{:?}", company_contribution).into()
            } else {
                String::new().into()
            },
            if let Some(imputed) = &self.imputed {
                format!("{:?}", imputed).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "employee_deduction".into(),
            "company_contribution".into(),
            "imputed".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Deductions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl std::fmt::Display for Deductions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Deductions {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
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
        vec!["name".into(), "amount".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Taxes {
    pub name: String,
    pub employer: bool,
    pub amount: String,
}

impl std::fmt::Display for Taxes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Taxes {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            format!("{:?}", self.employer).into(),
            self.amount.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "employer".into(), "amount".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmployeeCompensations {
    #[doc = "The ID of the employee."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<f64>,
    #[doc = "The employee's gross pay. This value is only available for processed payrolls."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gross_pay: Option<String>,
    #[doc = "The employee's net pay. This value is only available for processed payrolls."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_pay: Option<String>,
    #[doc = "The employee's compensation payment method. This value is only available for \
             processed payrolls."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<EmployeeCompensationsPaymentMethod>,
    #[doc = "An array of fixed compensations for the employee. Fixed compensations include tips, \
             bonuses, and one time reimbursements. If this payroll has been procesed, only fixed \
             compensations with a value greater than 0.00 are returned. For an unprocess payroll, \
             all active fixed compensations are returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_compensations: Option<Vec<FixedCompensations>>,
    #[doc = "An array of hourly compensations for the employee. Hourly compensations include \
             regular, overtime, and double overtime hours. If this payroll has been procesed, \
             only hourly compensations with a value greater than 0.00 are returned. For an \
             unprocess payroll, all active hourly compensations are returned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_compensations: Option<Vec<HourlyCompensations>>,
    #[doc = "An array of all paid time off the employee is eligible for this pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_time_off: Option<Vec<EmployeeCompensationsPaidTimeOff>>,
    #[doc = "An array of employee benefits for the pay period. Benefits are only included for \
             processed payroll when the include parameter is present."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub benefits: Option<Vec<Benefits>>,
    #[doc = "An array of employee deductions for the pay period. Deductions are only included for \
             processed payroll when the include parameter is present."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deductions: Option<Vec<Deductions>>,
    #[doc = "An array of employer and employee taxes for the pay period. Taxes are only included \
             for processed payroll when the include parameter is present."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<Taxes>>,
}

impl std::fmt::Display for EmployeeCompensations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmployeeCompensations {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(employee_id) = &self.employee_id {
                format!("{:?}", employee_id).into()
            } else {
                String::new().into()
            },
            if let Some(gross_pay) = &self.gross_pay {
                format!("{:?}", gross_pay).into()
            } else {
                String::new().into()
            },
            if let Some(net_pay) = &self.net_pay {
                format!("{:?}", net_pay).into()
            } else {
                String::new().into()
            },
            if let Some(payment_method) = &self.payment_method {
                format!("{:?}", payment_method).into()
            } else {
                String::new().into()
            },
            if let Some(fixed_compensations) = &self.fixed_compensations {
                format!("{:?}", fixed_compensations).into()
            } else {
                String::new().into()
            },
            if let Some(hourly_compensations) = &self.hourly_compensations {
                format!("{:?}", hourly_compensations).into()
            } else {
                String::new().into()
            },
            if let Some(paid_time_off) = &self.paid_time_off {
                format!("{:?}", paid_time_off).into()
            } else {
                String::new().into()
            },
            if let Some(benefits) = &self.benefits {
                format!("{:?}", benefits).into()
            } else {
                String::new().into()
            },
            if let Some(deductions) = &self.deductions {
                format!("{:?}", deductions).into()
            } else {
                String::new().into()
            },
            if let Some(taxes) = &self.taxes {
                format!("{:?}", taxes).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "employee_id".into(),
            "gross_pay".into(),
            "net_pay".into(),
            "payment_method".into(),
            "fixed_compensations".into(),
            "hourly_compensations".into(),
            "paid_time_off".into(),
            "benefits".into(),
            "deductions".into(),
            "taxes".into(),
        ]
    }
}

#[doc = "Payroll."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Payroll {
    #[doc = "The current version of the object. See the versioning guide for details using this \
             field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The deadline for the payroll to be run in order for employees to be paid on time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_deadline: Option<String>,
    #[doc = "The date on which employees will be paid for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_date: Option<String>,
    #[doc = "Whether or not the payroll has been successfully processed. Note that processed \
             payrolls cannot be updated. Additionally, a payroll is not guaranteed to be \
             processed just because the payroll deadline has passed. Late payrolls are not \
             uncommon. Conversely, users may choose to run payroll before the payroll deadline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<bool>,
    #[doc = "The date at which the payroll was processed. Null if the payroll isn't processed yet."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processed_date: Option<String>,
    #[doc = "A timestamp of the last valid payroll calculation. Null is there isn't a valid \
             calculation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calculated_at: Option<String>,
    #[doc = "The ID of the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_id: Option<f64>,
    #[doc = "A unique identifier of the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_uuid: Option<String>,
    #[doc = "The ID of the company for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<f64>,
    #[doc = "A unique identifier of the company for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period: Option<PayrollPayPeriod>,
    #[doc = "The subtotals for the payroll."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub totals: Option<Totals>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_compensations: Option<Vec<EmployeeCompensations>>,
}

impl std::fmt::Display for Payroll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Payroll {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(payroll_deadline) = &self.payroll_deadline {
                format!("{:?}", payroll_deadline).into()
            } else {
                String::new().into()
            },
            if let Some(check_date) = &self.check_date {
                format!("{:?}", check_date).into()
            } else {
                String::new().into()
            },
            if let Some(processed) = &self.processed {
                format!("{:?}", processed).into()
            } else {
                String::new().into()
            },
            if let Some(processed_date) = &self.processed_date {
                format!("{:?}", processed_date).into()
            } else {
                String::new().into()
            },
            if let Some(calculated_at) = &self.calculated_at {
                format!("{:?}", calculated_at).into()
            } else {
                String::new().into()
            },
            if let Some(payroll_id) = &self.payroll_id {
                format!("{:?}", payroll_id).into()
            } else {
                String::new().into()
            },
            if let Some(payroll_uuid) = &self.payroll_uuid {
                format!("{:?}", payroll_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            if let Some(company_uuid) = &self.company_uuid {
                format!("{:?}", company_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period) = &self.pay_period {
                format!("{:?}", pay_period).into()
            } else {
                String::new().into()
            },
            if let Some(totals) = &self.totals {
                format!("{:?}", totals).into()
            } else {
                String::new().into()
            },
            if let Some(employee_compensations) = &self.employee_compensations {
                format!("{:?}", employee_compensations).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "payroll_deadline".into(),
            "check_date".into(),
            "processed".into(),
            "processed_date".into(),
            "calculated_at".into(),
            "payroll_id".into(),
            "payroll_uuid".into(),
            "company_id".into(),
            "company_uuid".into(),
            "pay_period".into(),
            "totals".into(),
            "employee_compensations".into(),
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
pub enum CustomFieldType {
    #[serde(rename = "text")]
    #[display("text")]
    Text,
    #[serde(rename = "currency")]
    #[display("currency")]
    Currency,
    #[serde(rename = "number")]
    #[display("number")]
    Number,
    #[serde(rename = "date")]
    #[display("date")]
    Date,
    #[serde(rename = "radio")]
    #[display("radio")]
    Radio,
}

#[doc = "A custom field of an employee"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmployeeCustomField {
    pub id: String,
    #[doc = "This is the id of the response object from when you get the company custom fields"]
    pub company_custom_field_id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: CustomFieldType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub value: String,
    #[doc = "An array of options for fields of type radio. Otherwise, null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_options: Option<Vec<String>>,
}

impl std::fmt::Display for EmployeeCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmployeeCustomField {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.company_custom_field_id.clone().into(),
            self.name.clone().into(),
            format!("{:?}", self.type_).into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            self.value.clone().into(),
            if let Some(selection_options) = &self.selection_options {
                format!("{:?}", selection_options).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "company_custom_field_id".into(),
            "name".into(),
            "type_".into(),
            "description".into(),
            "value".into(),
            "selection_options".into(),
        ]
    }
}

#[doc = "A custom field on a company"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyCustomField {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: CustomFieldType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An array of options for fields of type radio. Otherwise, null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_options: Option<Vec<String>>,
}

impl std::fmt::Display for CompanyCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyCustomField {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.name.clone().into(),
            format!("{:?}", self.type_).into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(selection_options) = &self.selection_options {
                format!("{:?}", selection_options).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "name".into(),
            "type_".into(),
            "description".into(),
            "selection_options".into(),
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
pub enum GustoPersonType {
    Employee,
    Contractor,
    Candidate,
}

#[doc = "The representation of a job applicant in Gusto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobApplicant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gusto_person_type: Option<GustoPersonType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gusto_person_id: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gusto_person_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_uuid: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for JobApplicant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobApplicant {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(uuid) = &self.uuid {
                format!("{:?}", uuid).into()
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
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(gusto_person_type) = &self.gusto_person_type {
                format!("{:?}", gusto_person_type).into()
            } else {
                String::new().into()
            },
            if let Some(gusto_person_id) = &self.gusto_person_id {
                format!("{:?}", gusto_person_id).into()
            } else {
                String::new().into()
            },
            if let Some(gusto_person_uuid) = &self.gusto_person_uuid {
                format!("{:?}", gusto_person_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(company_uuid) = &self.company_uuid {
                format!("{:?}", company_uuid).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            if let Some(job_title) = &self.job_title {
                format!("{:?}", job_title).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_birth) = &self.date_of_birth {
                format!("{:?}", date_of_birth).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "uuid".into(),
            "first_name".into(),
            "last_name".into(),
            "email".into(),
            "phone".into(),
            "gusto_person_type".into(),
            "gusto_person_id".into(),
            "gusto_person_uuid".into(),
            "company_uuid".into(),
            "company_id".into(),
            "job_title".into(),
            "start_date".into(),
            "date_of_birth".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct FederalTaxDetails {
    #[doc = "The current version of the object. See the versioning guide for details using this \
             field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "What type of tax entity the company is. One of:\n- C-Corporation\n- S-Corporation\n- \
             Sole proprietor\n- LLC\n- LLP\n- Limited partnership\n- Co-ownership\n- \
             Association\n- Trusteeship\n- General partnership\n- Joint venture\n- Non-Profit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_type: Option<String>,
    #[doc = "The company's Employer Identification Number (EIN) registered with the IRS"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[doc = "Whether the company is taxed as an S-Corporation. Tax payer types that may be taxed \
             as an S-Corporation include:\n- S-Corporation\n- C-Corporation\n- LLC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxable_as_scorp: Option<String>,
    #[doc = "The form used by the company for federal tax filing. One of:\n- 941 (Quarterly \
             federal tax return form)\n- 944 (Annual federal tax return form)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_form: Option<String>,
    #[doc = "Whether the EIN was able to be verified as a valid EIN with the IRS. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein_verified: Option<bool>,
    #[doc = "The legal name of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
}

impl std::fmt::Display for FederalTaxDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for FederalTaxDetails {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(tax_payer_type) = &self.tax_payer_type {
                format!("{:?}", tax_payer_type).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
            if let Some(taxable_as_scorp) = &self.taxable_as_scorp {
                format!("{:?}", taxable_as_scorp).into()
            } else {
                String::new().into()
            },
            if let Some(filing_form) = &self.filing_form {
                format!("{:?}", filing_form).into()
            } else {
                String::new().into()
            },
            if let Some(ein_verified) = &self.ein_verified {
                format!("{:?}", ein_verified).into()
            } else {
                String::new().into()
            },
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "tax_payer_type".into(),
            "ein".into(),
            "taxable_as_scorp".into(),
            "filing_form".into(),
            "ein_verified".into(),
            "legal_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompensationObjectApplicationXml {}

impl std::fmt::Display for CompensationObjectApplicationXml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompensationObjectApplicationXml {
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
pub struct EarningTypeListApplicationJson {
    #[doc = "The default earning types for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<EarningType>>,
    #[doc = "The custom earning types for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<EarningType>>,
}

impl std::fmt::Display for EarningTypeListApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EarningTypeListApplicationJson {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(default) = &self.default {
                format!("{:?}", default).into()
            } else {
                String::new().into()
            },
            if let Some(custom) = &self.custom {
                format!("{:?}", custom).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["default".into(), "custom".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostEmployeeYtdBenefitAmountsFromDifferentCompanyApplicationJson {
    #[doc = "The id for the benefit got from the benefits api."]
    pub benefit_id: f64,
    #[doc = "The tax year for which this amount applies."]
    pub tax_year: f64,
    #[doc = "The year-to-date employee deduction made outside the current company."]
    pub ytd_employee_deduction_amount: String,
    #[doc = "The year-to-date company contribution made outside the current company."]
    pub ytd_company_contribution_amount: String,
}

impl std::fmt::Display for PostEmployeeYtdBenefitAmountsFromDifferentCompanyApplicationJson {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostEmployeeYtdBenefitAmountsFromDifferentCompanyApplicationJson {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.benefit_id).into(),
            format!("{:?}", self.tax_year).into(),
            self.ytd_employee_deduction_amount.clone().into(),
            self.ytd_company_contribution_amount.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "benefit_id".into(),
            "tax_year".into(),
            "ytd_employee_deduction_amount".into(),
            "ytd_company_contribution_amount".into(),
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
pub enum Include {
    #[serde(rename = "custom_fields")]
    #[display("custom_fields")]
    CustomFields,
}

impl std::default::Default for Include {
    fn default() -> Self {
        Include::CustomFields
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
pub enum GetInclude {
    #[serde(rename = "custom_fields")]
    #[display("custom_fields")]
    CustomFields,
}

impl std::default::Default for GetInclude {
    fn default() -> Self {
        GetInclude::CustomFields
    }
}

#[doc = "PutEmployeesRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutEmployeesRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub two_percent_shareholder: Option<bool>,
}

impl std::fmt::Display for PutEmployeesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutEmployeesRequestBody {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(middle_initial) = &self.middle_initial {
                format!("{:?}", middle_initial).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_birth) = &self.date_of_birth {
                format!("{:?}", date_of_birth).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(ssn) = &self.ssn {
                format!("{:?}", ssn).into()
            } else {
                String::new().into()
            },
            if let Some(two_percent_shareholder) = &self.two_percent_shareholder {
                format!("{:?}", two_percent_shareholder).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "first_name".into(),
            "middle_initial".into(),
            "last_name".into(),
            "date_of_birth".into(),
            "email".into(),
            "ssn".into(),
            "two_percent_shareholder".into(),
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
pub enum GetCompaniesCompanyIdInclude {
    #[serde(rename = "custom_fields")]
    #[display("custom_fields")]
    CustomFields,
}

impl std::default::Default for GetCompaniesCompanyIdInclude {
    fn default() -> Self {
        GetCompaniesCompanyIdInclude::CustomFields
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCompaniesCompanyIdEmployeesRequestBody {}

impl std::fmt::Display for GetCompaniesCompanyIdEmployeesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCompaniesCompanyIdEmployeesRequestBody {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[doc = "PostEmployeesRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostEmployeesRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
}

impl std::fmt::Display for PostEmployeesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostEmployeesRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(first_name) = &self.first_name {
                format!("{:?}", first_name).into()
            } else {
                String::new().into()
            },
            if let Some(middle_initial) = &self.middle_initial {
                format!("{:?}", middle_initial).into()
            } else {
                String::new().into()
            },
            if let Some(last_name) = &self.last_name {
                format!("{:?}", last_name).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_birth) = &self.date_of_birth {
                format!("{:?}", date_of_birth).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(ssn) = &self.ssn {
                format!("{:?}", ssn).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "middle_initial".into(),
            "last_name".into(),
            "date_of_birth".into(),
            "email".into(),
            "ssn".into(),
        ]
    }
}

#[doc = "PutJobsJobIdRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutJobsJobIdRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for PutJobsJobIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutJobsJobIdRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(hire_date) = &self.hire_date {
                format!("{:?}", hire_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "title".into(),
            "location_id".into(),
            "hire_date".into(),
        ]
    }
}

#[doc = "PostJobsJobIdRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostJobsJobIdRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hire_date: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for PostJobsJobIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostJobsJobIdRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(location_id) = &self.location_id {
                format!("{:?}", location_id).into()
            } else {
                String::new().into()
            },
            if let Some(hire_date) = &self.hire_date {
                format!("{:?}", hire_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["title".into(), "location_id".into(), "hire_date".into()]
    }
}

#[doc = "PostCompaniesCompanyIdLocationsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdLocationsRequestBody {
    pub phone_number: String,
    pub street_1: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Specify if this location is the company's mailing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<bool>,
    #[doc = "Specify if this location is the company's filing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_address: Option<bool>,
}

impl std::fmt::Display for PostCompaniesCompanyIdLocationsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdLocationsRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.phone_number.clone().into(),
            self.street_1.clone().into(),
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            self.city.clone().into(),
            self.state.clone().into(),
            self.zip.clone().into(),
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(mailing_address) = &self.mailing_address {
                format!("{:?}", mailing_address).into()
            } else {
                String::new().into()
            },
            if let Some(filing_address) = &self.filing_address {
                format!("{:?}", filing_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "phone_number".into(),
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "state".into(),
            "zip".into(),
            "country".into(),
            "mailing_address".into(),
            "filing_address".into(),
        ]
    }
}

#[doc = "PutLocationsLocationIdRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutLocationsLocationIdRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "For a company location, specify if this location is the company's mailing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<bool>,
    #[doc = "For a company location, specify if this location is the company's filing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_address: Option<bool>,
}

impl std::fmt::Display for PutLocationsLocationIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutLocationsLocationIdRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(phone_number) = &self.phone_number {
                format!("{:?}", phone_number).into()
            } else {
                String::new().into()
            },
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(mailing_address) = &self.mailing_address {
                format!("{:?}", mailing_address).into()
            } else {
                String::new().into()
            },
            if let Some(filing_address) = &self.filing_address {
                format!("{:?}", filing_address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "phone_number".into(),
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "state".into(),
            "zip".into(),
            "country".into(),
            "mailing_address".into(),
            "filing_address".into(),
        ]
    }
}

#[doc = "The contractor’s wage type, either “Fixed” or “Hourly”.\n"]
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
pub enum PutContractorsContractorIdRequestBodyWageType {
    Fixed,
    Hourly,
}

#[doc = "PutContractorsContractorIdRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutContractorsContractorIdRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "The day when the contractor will start working for the company.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[doc = "The contractor’s first name. This attribute is required for “Individual” contractors \
             and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The contractor’s last name. This attribute is required for “Individual” contractors \
             and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "null\tThe contractor’s middle initial. This attribute is optional for “Individual” \
             contractors and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[doc = "The contractor’s wage type, either “Fixed” or “Hourly”.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wage_type: Option<PutContractorsContractorIdRequestBodyWageType>,
    #[doc = "The contractor’s hourly rate. This attribute is required if the wage_type is \
             “Hourly”."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_rate: Option<String>,
    #[doc = "The name of the contractor business. This attribute is required for “Business” \
             contractors and will be ignored for “Individual” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[doc = "The employer identification number of the contractor business. This attribute is \
             optional for “Business” contractors and will be ignored for “Individual” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
}

impl std::fmt::Display for PutContractorsContractorIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutContractorsContractorIdRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
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
            if let Some(middle_initial) = &self.middle_initial {
                format!("{:?}", middle_initial).into()
            } else {
                String::new().into()
            },
            if let Some(wage_type) = &self.wage_type {
                format!("{:?}", wage_type).into()
            } else {
                String::new().into()
            },
            if let Some(hourly_rate) = &self.hourly_rate {
                format!("{:?}", hourly_rate).into()
            } else {
                String::new().into()
            },
            if let Some(business_name) = &self.business_name {
                format!("{:?}", business_name).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "start_date".into(),
            "first_name".into(),
            "last_name".into(),
            "middle_initial".into(),
            "wage_type".into(),
            "hourly_rate".into(),
            "business_name".into(),
            "ein".into(),
        ]
    }
}

#[doc = "The contractor type, either an “Individual” or a “Business”.\n"]
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
pub enum PostCompaniesCompanyIdContractorsRequestBodyType {
    Individual,
    Business,
}

#[doc = "The contractor’s wage type, either “Fixed” or “Hourly”.\n"]
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
pub enum PostCompaniesCompanyIdContractorsRequestBodyWageType {
    Hourly,
    Fixed,
}

#[doc = "PostCompaniesCompanyIdContractorsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdContractorsRequestBody {
    #[doc = "The contractor type, either an “Individual” or a “Business”.\n"]
    #[serde(rename = "type")]
    pub type_: PostCompaniesCompanyIdContractorsRequestBodyType,
    #[doc = "The contractor’s wage type, either “Fixed” or “Hourly”.\n"]
    pub wage_type: PostCompaniesCompanyIdContractorsRequestBodyWageType,
    #[doc = "The contractor’s first name. This attribute is required for “Individual” contractors \
             and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "The contractor’s last_name. This attribute is optional for “Individual” contractors \
             and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "The day when the contractor will start working for the company."]
    pub start_date: chrono::NaiveDate,
    #[doc = "Whether the contractor or the payroll admin will complete onboarding in Gusto. \
             Self-onboarding is recommended so that contractors receive Gusto accounts. If \
             self_onboarding is true, then email is required. "]
    #[serde(default)]
    pub self_onboarding: bool,
    #[doc = "The contractor’s email address. This attribute is optional for “Individual” \
             contractors and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The contractor’s middle initial. This attribute is optional for “Individual” \
             contractors and will be ignored for “Business” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[doc = "The name of the contractor business. This attribute is required for “Business” \
             contractors and will be ignored for “Individual” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[doc = "The employer identification number of the contractor business. This attribute is \
             optional for “Business” contractors and will be ignored for “Individual” contractors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
}

impl std::fmt::Display for PostCompaniesCompanyIdContractorsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdContractorsRequestBody {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.type_).into(),
            format!("{:?}", self.wage_type).into(),
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
            format!("{:?}", self.start_date).into(),
            format!("{:?}", self.self_onboarding).into(),
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(middle_initial) = &self.middle_initial {
                format!("{:?}", middle_initial).into()
            } else {
                String::new().into()
            },
            if let Some(business_name) = &self.business_name {
                format!("{:?}", business_name).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "wage_type".into(),
            "first_name".into(),
            "last_name".into(),
            "start_date".into(),
            "self_onboarding".into(),
            "email".into(),
            "middle_initial".into(),
            "business_name".into(),
            "ein".into(),
        ]
    }
}

#[doc = "PutCompensationsCompensationIdRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompensationsCompensationIdRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
    #[doc = "The dollar amount paid per payment unit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[doc = "The unit accompanying the compensation rate. If the employee is an owner, rate \
             should be 'Paycheck'."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_unit: Option<PaymentUnit>,
    #[doc = "The FLSA status for this compensation. Salaried ('Exempt') employees are paid a \
             fixed salary every pay period. Salaried with overtime ('Salaried Nonexempt') \
             employees are paid a fixed salary every pay period, and receive overtime pay when \
             applicable. Hourly ('Nonexempt') employees are paid for the hours they work, and \
             receive overtime pay when applicable. Owners ('Owner') are employees that own at \
             least twenty percent of the company. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flsa_status: Option<FlsaStatus>,
}

impl std::fmt::Display for PutCompensationsCompensationIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompensationsCompensationIdRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            if let Some(rate) = &self.rate {
                format!("{:?}", rate).into()
            } else {
                String::new().into()
            },
            if let Some(payment_unit) = &self.payment_unit {
                format!("{:?}", payment_unit).into()
            } else {
                String::new().into()
            },
            if let Some(flsa_status) = &self.flsa_status {
                format!("{:?}", flsa_status).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "rate".into(),
            "payment_unit".into(),
            "flsa_status".into(),
        ]
    }
}

#[doc = "PostJobsJobIdCompensationsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostJobsJobIdCompensationsRequestBody {
    #[doc = "The dollar amount paid per payment unit."]
    pub rate: String,
    #[doc = "The unit accompanying the compensation rate. If the employee is an owner, rate \
             should be 'Paycheck'."]
    pub payment_unit: PaymentUnit,
    #[doc = "The FLSA status for this compensation. Salaried ('Exempt') employees are paid a \
             fixed salary every pay period. Salaried with overtime ('Salaried Nonexempt') \
             employees are paid a fixed salary every pay period, and receive overtime pay when \
             applicable. Hourly ('Nonexempt') employees are paid for the hours they work, and \
             receive overtime pay when applicable. Owners ('Owner') are employees that own at \
             least twenty percent of the company. "]
    pub flsa_status: FlsaStatus,
    #[doc = "The effective date for this compensation. For the first compensation, this defaults \
             to the job's hire date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for PostJobsJobIdCompensationsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostJobsJobIdCompensationsRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.rate.clone().into(),
            format!("{:?}", self.payment_unit).into(),
            format!("{:?}", self.flsa_status).into(),
            if let Some(effective_date) = &self.effective_date {
                format!("{:?}", effective_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "rate".into(),
            "payment_unit".into(),
            "flsa_status".into(),
            "effective_date".into(),
        ]
    }
}

#[doc = "PostEmployeesEmployeeIdGarnishmentsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostEmployeesEmployeeIdGarnishmentsRequestBody {
    #[doc = "Whether or not this garnishment is currently active."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The amount of the garnishment. Either a percentage or a fixed dollar amount. \
             Represented as a float, e.g. \"8.00\"."]
    pub amount: String,
    #[doc = "The description of the garnishment."]
    pub description: String,
    #[doc = "Whether the garnishment is court ordered."]
    pub court_ordered: bool,
    #[doc = "The number of times to apply the garnisment. Ignored if recurring is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub times: Option<i64>,
    #[doc = "Whether the garnishment should recur indefinitely."]
    #[serde(default)]
    pub recurring: bool,
    #[doc = "The maximum deduction per annum. A null value indicates no maximum. Represented as a \
             float, e.g. \"200.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_maximum: Option<String>,
    #[doc = "The maximum deduction per pay period. A null value indicates no maximum. Represented \
             as a float, e.g. \"16.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period_maximum: Option<String>,
    #[doc = "Whether the amount should be treated as a percentage to be deducted per pay period."]
    #[serde(default)]
    pub deduct_as_percentage: bool,
}

impl std::fmt::Display for PostEmployeesEmployeeIdGarnishmentsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostEmployeesEmployeeIdGarnishmentsRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.active).into(),
            self.amount.clone().into(),
            self.description.clone().into(),
            format!("{:?}", self.court_ordered).into(),
            if let Some(times) = &self.times {
                format!("{:?}", times).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.recurring).into(),
            if let Some(annual_maximum) = &self.annual_maximum {
                format!("{:?}", annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period_maximum) = &self.pay_period_maximum {
                format!("{:?}", pay_period_maximum).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.deduct_as_percentage).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "active".into(),
            "amount".into(),
            "description".into(),
            "court_ordered".into(),
            "times".into(),
            "recurring".into(),
            "annual_maximum".into(),
            "pay_period_maximum".into(),
            "deduct_as_percentage".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutGarnishmentsGarnishmentIdRequestBody {
    #[doc = "Whether or not this garnishment is currently active."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The amount of the garnishment. Either a percentage or a fixed dollar amount. \
             Represented as a float, e.g. \"8.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[doc = "The description of the garnishment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the garnishment is court ordered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub court_ordered: Option<bool>,
    #[doc = "The number of times to apply the garnisment. Ignored if recurring is true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub times: Option<i64>,
    #[doc = "Whether the garnishment should recur indefinitely."]
    #[serde(default)]
    pub recurring: bool,
    #[doc = "The maximum deduction per annum. A null value indicates no maximum. Represented as a \
             float, e.g. \"200.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_maximum: Option<String>,
    #[doc = "The maximum deduction per pay period. A null value indicates no maximum. Represented \
             as a float, e.g. \"16.00\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period_maximum: Option<String>,
    #[doc = "Whether the amount should be treated as a percentage to be deducted per pay period."]
    #[serde(default)]
    pub deduct_as_percentage: bool,
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
}

impl std::fmt::Display for PutGarnishmentsGarnishmentIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutGarnishmentsGarnishmentIdRequestBody {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.active).into(),
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(court_ordered) = &self.court_ordered {
                format!("{:?}", court_ordered).into()
            } else {
                String::new().into()
            },
            if let Some(times) = &self.times {
                format!("{:?}", times).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.recurring).into(),
            if let Some(annual_maximum) = &self.annual_maximum {
                format!("{:?}", annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period_maximum) = &self.pay_period_maximum {
                format!("{:?}", pay_period_maximum).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.deduct_as_percentage).into(),
            self.version.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "active".into(),
            "amount".into(),
            "description".into(),
            "court_ordered".into(),
            "times".into(),
            "recurring".into(),
            "annual_maximum".into(),
            "pay_period_maximum".into(),
            "deduct_as_percentage".into(),
            "version".into(),
        ]
    }
}

#[doc = "PostEmployeesEmployeeIdTerminationsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostEmployeesEmployeeIdTerminationsRequestBody {
    #[doc = "The employee's last day of work."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<chrono::NaiveDate>,
    #[doc = "If true, the employee should recieve their final wages via an offcycle payroll. If \
             false, they should recieve their final wages on their current pay schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub run_termination_payroll: Option<bool>,
}

impl std::fmt::Display for PostEmployeesEmployeeIdTerminationsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostEmployeesEmployeeIdTerminationsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(effective_date) = &self.effective_date {
                format!("{:?}", effective_date).into()
            } else {
                String::new().into()
            },
            if let Some(run_termination_payroll) = &self.run_termination_payroll {
                format!("{:?}", run_termination_payroll).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["effective_date".into(), "run_termination_payroll".into()]
    }
}

#[doc = "PutEmployeesEmployeeIdHomeAddressRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutEmployeesEmployeeIdHomeAddressRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl std::fmt::Display for PutEmployeesEmployeeIdHomeAddressRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutEmployeesEmployeeIdHomeAddressRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(state) = &self.state {
                format!("{:?}", state).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "state".into(),
            "zip".into(),
        ]
    }
}

#[doc = "PutCompaniesCompanyIdPaySchedulesPayScheduleIdRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPaySchedulesPayScheduleIdRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
    #[doc = "With Autopilot® enabled, payroll will run automatically one day before your payroll \
             deadlines."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_pilot: Option<bool>,
}

impl std::fmt::Display for PutCompaniesCompanyIdPaySchedulesPayScheduleIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdPaySchedulesPayScheduleIdRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            if let Some(auto_pilot) = &self.auto_pilot {
                format!("{:?}", auto_pilot).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["version".into(), "auto_pilot".into()]
    }
}

#[doc = "The bank account type"]
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
pub enum PostCompaniesCompanyIdBankAccountsRequestBodyAccountType {
    Checking,
    Savings,
}

#[doc = "PostCompaniesCompanyIdBankAccountsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdBankAccountsRequestBody {
    #[doc = "The bank routing number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[doc = "The bank account number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[doc = "The bank account type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PostCompaniesCompanyIdBankAccountsRequestBodyAccountType>,
}

impl std::fmt::Display for PostCompaniesCompanyIdBankAccountsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdBankAccountsRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(routing_number) = &self.routing_number {
                format!("{:?}", routing_number).into()
            } else {
                String::new().into()
            },
            if let Some(account_number) = &self.account_number {
                format!("{:?}", account_number).into()
            } else {
                String::new().into()
            },
            if let Some(account_type) = &self.account_type {
                format!("{:?}", account_type).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "routing_number".into(),
            "account_number".into(),
            "account_type".into(),
        ]
    }
}

#[doc = "PutCompaniesCompanyIdBankAccountsVerifyRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdBankAccountsVerifyRequestBody {
    #[doc = "The dollar amount of the first micro-deposit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_1: Option<f64>,
    #[doc = "The dollar amount of the second micro-deposit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deposit_2: Option<f64>,
}

impl std::fmt::Display for PutCompaniesCompanyIdBankAccountsVerifyRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdBankAccountsVerifyRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(deposit_1) = &self.deposit_1 {
                format!("{:?}", deposit_1).into()
            } else {
                String::new().into()
            },
            if let Some(deposit_2) = &self.deposit_2 {
                format!("{:?}", deposit_2).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["deposit_1".into(), "deposit_2".into()]
    }
}

#[doc = "PostCompaniesCompanyIdCompanyBenefitsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdCompanyBenefitsRequestBody {
    #[doc = "The ID of the benefit to which the company benefit belongs."]
    pub benefit_id: f64,
    #[doc = "Whether this benefit is active for employee participation."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The description of the company benefit.For example, a company may offer multiple \
             benefits with an ID of 1 (for Medical Insurance). The description would show \
             something more specific like “Kaiser Permanente” or “Blue Cross/ Blue Shield”."]
    pub description: String,
    #[doc = "Whether the employer is subject to pay employer taxes when an employee is on leave. \
             Only applicable to third party sick pay benefits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub responsible_for_employer_taxes: Option<bool>,
    #[doc = "Whether the employer is subject to file W-2 forms for an employee on leave. Only \
             applicable to third party sick pay benefits."]
    #[serde(
        rename = "responsible_for_employee_w2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub responsible_for_employee_w_2: Option<bool>,
}

impl std::fmt::Display for PostCompaniesCompanyIdCompanyBenefitsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdCompanyBenefitsRequestBody {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.benefit_id).into(),
            format!("{:?}", self.active).into(),
            self.description.clone().into(),
            if let Some(responsible_for_employer_taxes) = &self.responsible_for_employer_taxes {
                format!("{:?}", responsible_for_employer_taxes).into()
            } else {
                String::new().into()
            },
            if let Some(responsible_for_employee_w_2) = &self.responsible_for_employee_w_2 {
                format!("{:?}", responsible_for_employee_w_2).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "benefit_id".into(),
            "active".into(),
            "description".into(),
            "responsible_for_employer_taxes".into(),
            "responsible_for_employee_w_2".into(),
        ]
    }
}

#[doc = "PutCompanyBenefitsCompanyBenefitIdRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompanyBenefitsCompanyBenefitIdRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
    #[doc = "Whether this benefit is active for employee participation. Company benefits may only \
             be deactivated if no employees are actively participating."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "The description of the company benefit.For example, a company may offer multiple \
             benefits with an ID of 1 (for Medical Insurance). The description would show \
             something more specific like “Kaiser Permanente” or “Blue Cross/ Blue Shield”."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl std::fmt::Display for PutCompanyBenefitsCompanyBenefitIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompanyBenefitsCompanyBenefitIdRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["version".into(), "active".into(), "description".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCompaniesCompanyIdEarningTypesResponse {
    #[doc = "The default earning types for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Vec<EarningType>>,
    #[doc = "The custom earning types for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<Vec<EarningType>>,
}

impl std::fmt::Display for GetCompaniesCompanyIdEarningTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCompaniesCompanyIdEarningTypesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(default) = &self.default {
                format!("{:?}", default).into()
            } else {
                String::new().into()
            },
            if let Some(custom) = &self.custom {
                format!("{:?}", custom).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["default".into(), "custom".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdEarningTypesRequestBody {
    #[doc = "The name of the custom earning type."]
    pub name: String,
}

impl std::fmt::Display for PostCompaniesCompanyIdEarningTypesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdEarningTypesRequestBody {
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
pub struct PutCompaniesCompanyIdEarningTypesEarningTypeUuidRequestBody {
    #[doc = "The name of the custom earning type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for PutCompaniesCompanyIdEarningTypesEarningTypeUuidRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdEarningTypesEarningTypeUuidRequestBody {
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

#[doc = "Whether the employee deduction reduces taxable income or not. Only valid for Group Term \
         Life benefits. Note: when the value is not \"unset\", coverage amount and coverage salary \
         multiplier are ignored."]
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
pub enum PostEmployeesEmployeeIdEmployeeBenefitsRequestBodyDeductionReducesTaxableIncome {
    #[serde(rename = "unset")]
    #[display("unset")]
    Unset,
    #[serde(rename = "reduces_taxable_income")]
    #[display("reduces_taxable_income")]
    ReducesTaxableIncome,
    #[serde(rename = "does_not_reduce_taxable_income")]
    #[display("does_not_reduce_taxable_income")]
    DoesNotReduceTaxableIncome,
}

#[doc = "PostEmployeesEmployeeIdEmployeeBenefitsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostEmployeesEmployeeIdEmployeeBenefitsRequestBody {
    #[doc = "The ID of the company to which the benefit belongs."]
    pub company_benefit_id: f64,
    #[doc = "Whether the employee benefit is active."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The amount to be deducted, per pay period, from the employee's pay."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_deduction: Option<String>,
    #[doc = "The amount to be paid, per pay period, by the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_contribution: Option<String>,
    #[doc = "The maximum employee deduction amount per year. A null value signifies no limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_deduction_annual_maximum: Option<String>,
    #[doc = "The maximum company contribution amount per year. A null value signifies no limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_contribution_annual_maximum: Option<String>,
    #[doc = "Some benefits require additional information to determine their limit. For example, \
             for an HSA benefit, the limit option should be either \"Family\" or \"Individual\". \
             For a Dependent Care FSA benefit, the limit option should be either \"Joint Filing \
             or Single\" or \"Married and Filing Separately\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_option: Option<String>,
    #[doc = "Whether the employee deduction amount should be treated as a percentage to be \
             deducted from each payroll."]
    #[serde(default)]
    pub deduct_as_percentage: bool,
    #[doc = "Whether the company contribution amount should be treated as a percentage to be \
             deducted from each payroll."]
    #[serde(default)]
    pub contribute_as_percentage: bool,
    #[doc = "Whether the employee should use a benefit’s \"catch up\" rate. Only Roth 401k and \
             401k benefits use this value for employees over 50."]
    #[serde(default)]
    pub catch_up: bool,
    #[doc = "The amount that the employee is insured for. Note: company contribution cannot be \
             present if coverage amount is set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_amount: Option<String>,
    #[doc = "Whether the employee deduction reduces taxable income or not. Only valid for Group \
             Term Life benefits. Note: when the value is not \"unset\", coverage amount and \
             coverage salary multiplier are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduction_reduces_taxable_income:
        Option<PostEmployeesEmployeeIdEmployeeBenefitsRequestBodyDeductionReducesTaxableIncome>,
    #[doc = "The coverage amount as a multiple of the employee’s salary. Only applicable for \
             Group Term Life benefits. Note: cannot be set if coverage amount is also set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_salary_multiplier: Option<String>,
}

impl std::fmt::Display for PostEmployeesEmployeeIdEmployeeBenefitsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostEmployeesEmployeeIdEmployeeBenefitsRequestBody {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.company_benefit_id).into(),
            format!("{:?}", self.active).into(),
            if let Some(employee_deduction) = &self.employee_deduction {
                format!("{:?}", employee_deduction).into()
            } else {
                String::new().into()
            },
            if let Some(company_contribution) = &self.company_contribution {
                format!("{:?}", company_contribution).into()
            } else {
                String::new().into()
            },
            if let Some(employee_deduction_annual_maximum) = &self.employee_deduction_annual_maximum
            {
                format!("{:?}", employee_deduction_annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(company_contribution_annual_maximum) =
                &self.company_contribution_annual_maximum
            {
                format!("{:?}", company_contribution_annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(limit_option) = &self.limit_option {
                format!("{:?}", limit_option).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.deduct_as_percentage).into(),
            format!("{:?}", self.contribute_as_percentage).into(),
            format!("{:?}", self.catch_up).into(),
            if let Some(coverage_amount) = &self.coverage_amount {
                format!("{:?}", coverage_amount).into()
            } else {
                String::new().into()
            },
            if let Some(deduction_reduces_taxable_income) = &self.deduction_reduces_taxable_income {
                format!("{:?}", deduction_reduces_taxable_income).into()
            } else {
                String::new().into()
            },
            if let Some(coverage_salary_multiplier) = &self.coverage_salary_multiplier {
                format!("{:?}", coverage_salary_multiplier).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company_benefit_id".into(),
            "active".into(),
            "employee_deduction".into(),
            "company_contribution".into(),
            "employee_deduction_annual_maximum".into(),
            "company_contribution_annual_maximum".into(),
            "limit_option".into(),
            "deduct_as_percentage".into(),
            "contribute_as_percentage".into(),
            "catch_up".into(),
            "coverage_amount".into(),
            "deduction_reduces_taxable_income".into(),
            "coverage_salary_multiplier".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequestBody {
    #[doc = "The id for the benefit got from the benefits api."]
    pub benefit_id: f64,
    #[doc = "The tax year for which this amount applies."]
    pub tax_year: f64,
    #[doc = "The year-to-date employee deduction made outside the current company."]
    pub ytd_employee_deduction_amount: String,
    #[doc = "The year-to-date company contribution made outside the current company."]
    pub ytd_company_contribution_amount: String,
}

impl std::fmt::Display for PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostEmployeeYtdBenefitAmountsFromDifferentCompanyRequestBody {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.benefit_id).into(),
            format!("{:?}", self.tax_year).into(),
            self.ytd_employee_deduction_amount.clone().into(),
            self.ytd_company_contribution_amount.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "benefit_id".into(),
            "tax_year".into(),
            "ytd_employee_deduction_amount".into(),
            "ytd_company_contribution_amount".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutEmployeeBenefitsEmployeeBenefitIdRequestBody {
    #[doc = "The current version of the object. See the versioning guide for information on how \
             to use this field."]
    pub version: String,
    #[doc = "Whether the employee benefit is active."]
    #[serde(default)]
    pub active: bool,
    #[doc = "The amount to be deducted, per pay period, from the employee's pay."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_deduction: Option<String>,
    #[doc = "The amount to be paid, per pay period, by the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_contribution: Option<String>,
    #[doc = "The maximum employee deduction amount per year. A null value signifies no limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_deduction_annual_maximum: Option<String>,
    #[doc = "The maximum company contribution amount per year. A null value signifies no limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_contribution_annual_maximum: Option<String>,
    #[doc = "Some benefits require additional information to determine their limit. For example, \
             for an HSA benefit, the limit option should be either \"Family\" or \"Individual\". \
             For a Dependent Care FSA benefit, the limit option should be either \"Joint Filing \
             or Single\" or \"Married and Filing Separately\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_option: Option<String>,
    #[doc = "Whether the employee deduction amount should be treated as a percentage to be \
             deducted from each payroll."]
    #[serde(default)]
    pub deduct_as_percentage: bool,
    #[doc = "Whether the company contribution amount should be treated as a percentage to be \
             deducted from each payroll."]
    #[serde(default)]
    pub contribute_as_percentage: bool,
    #[doc = "Whether the employee should use a benefit’s \"catch up\" rate. Only Roth 401k and \
             401k benefits use this value for employees over 50."]
    #[serde(default)]
    pub catch_up: bool,
    #[doc = "The amount that the employee is insured for. Note: company contribution cannot be \
             present if coverage amount is set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_amount: Option<String>,
    #[doc = "Whether the employee deduction reduces taxable income or not. Only valid for Group \
             Term Life benefits. Note: when the value is not \"unset\", coverage amount and \
             coverage salary multiplier are ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deduction_reduces_taxable_income: Option<DeductionReducesTaxableIncome>,
    #[doc = "The coverage amount as a multiple of the employee’s salary. Only applicable for \
             Group Term Life benefits. Note: cannot be set if coverage amount is also set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage_salary_multiplier: Option<String>,
}

impl std::fmt::Display for PutEmployeeBenefitsEmployeeBenefitIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutEmployeeBenefitsEmployeeBenefitIdRequestBody {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            format!("{:?}", self.active).into(),
            if let Some(employee_deduction) = &self.employee_deduction {
                format!("{:?}", employee_deduction).into()
            } else {
                String::new().into()
            },
            if let Some(company_contribution) = &self.company_contribution {
                format!("{:?}", company_contribution).into()
            } else {
                String::new().into()
            },
            if let Some(employee_deduction_annual_maximum) = &self.employee_deduction_annual_maximum
            {
                format!("{:?}", employee_deduction_annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(company_contribution_annual_maximum) =
                &self.company_contribution_annual_maximum
            {
                format!("{:?}", company_contribution_annual_maximum).into()
            } else {
                String::new().into()
            },
            if let Some(limit_option) = &self.limit_option {
                format!("{:?}", limit_option).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.deduct_as_percentage).into(),
            format!("{:?}", self.contribute_as_percentage).into(),
            format!("{:?}", self.catch_up).into(),
            if let Some(coverage_amount) = &self.coverage_amount {
                format!("{:?}", coverage_amount).into()
            } else {
                String::new().into()
            },
            if let Some(deduction_reduces_taxable_income) = &self.deduction_reduces_taxable_income {
                format!("{:?}", deduction_reduces_taxable_income).into()
            } else {
                String::new().into()
            },
            if let Some(coverage_salary_multiplier) = &self.coverage_salary_multiplier {
                format!("{:?}", coverage_salary_multiplier).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "active".into(),
            "employee_deduction".into(),
            "company_contribution".into(),
            "employee_deduction_annual_maximum".into(),
            "company_contribution_annual_maximum".into(),
            "limit_option".into(),
            "deduct_as_percentage".into(),
            "contribute_as_percentage".into(),
            "catch_up".into(),
            "coverage_amount".into(),
            "deduction_reduces_taxable_income".into(),
            "coverage_salary_multiplier".into(),
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
pub enum GetCompaniesCompanyIdPayrollsInclude {
    #[serde(rename = "benefits")]
    #[display("benefits")]
    Benefits,
    #[serde(rename = "deductions")]
    #[display("deductions")]
    Deductions,
    #[serde(rename = "taxes")]
    #[display("taxes")]
    Taxes,
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
pub enum OffCycleReason {
    Bonus,
    Correction,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdPayrollsRequestBody {
    pub off_cycle: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub off_cycle_reason: Option<OffCycleReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employee_ids: Option<Vec<i64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_date: Option<String>,
}

impl std::fmt::Display for PostCompaniesCompanyIdPayrollsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdPayrollsRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.off_cycle.clone().into(),
            if let Some(off_cycle_reason) = &self.off_cycle_reason {
                format!("{:?}", off_cycle_reason).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(end_date) = &self.end_date {
                format!("{:?}", end_date).into()
            } else {
                String::new().into()
            },
            if let Some(employee_ids) = &self.employee_ids {
                format!("{:?}", employee_ids).into()
            } else {
                String::new().into()
            },
            if let Some(check_date) = &self.check_date {
                format!("{:?}", check_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "off_cycle".into(),
            "off_cycle_reason".into(),
            "start_date".into(),
            "end_date".into(),
            "employee_ids".into(),
            "check_date".into(),
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
pub enum GetCompaniesCompanyIdPayrollsIdInclude {
    #[serde(rename = "benefits")]
    #[display("benefits")]
    Benefits,
    #[serde(rename = "deductions")]
    #[display("deductions")]
    Deductions,
    #[serde(rename = "taxes")]
    #[display("taxes")]
    Taxes,
}

#[doc = "An array of fixed compensations for the employee. Fixed compensations include tips, \
         bonuses, and one time reimbursements."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsFixedCompensations {
    #[doc = "The name of the compensation. This also serves as the unique, immutable identifier \
             for this compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The amount of the compensation for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[doc = "The ID of the job for the compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
}

impl std::fmt::Display
    for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsFixedCompensations
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled
    for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsFixedCompensations
{
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(amount) = &self.amount {
                format!("{:?}", amount).into()
            } else {
                String::new().into()
            },
            if let Some(job_id) = &self.job_id {
                format!("{:?}", job_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "amount".into(), "job_id".into()]
    }
}

#[doc = "An array of hourly compensations for the employee. Hourly compensations include regular, \
         overtime, and double overtime hours."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsHourlyCompensations {
    #[doc = "The name of the compensation. This also serves as the unique, immutable identifier \
             for this compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of hours to be compensated for this pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
    #[doc = "The ID of the job for the compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
}

impl std::fmt::Display
    for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsHourlyCompensations
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled
    for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsHourlyCompensations
{
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(hours) = &self.hours {
                format!("{:?}", hours).into()
            } else {
                String::new().into()
            },
            if let Some(job_id) = &self.job_id {
                format!("{:?}", job_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "hours".into(), "job_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsPaidTimeOff {
    #[doc = "The name of the PTO. This also serves as the unique, immutable identifier for the \
             PTO."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hours of this PTO taken during the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
}

impl std::fmt::Display
    for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsPaidTimeOff
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsPaidTimeOff {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(hours) = &self.hours {
                format!("{:?}", hours).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "hours".into()]
    }
}

#[doc = "PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensations."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensations {
    #[doc = "The ID of the employee."]
    pub employee_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_compensations: Option<
        Vec<PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsFixedCompensations>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_compensations: Option<
        Vec<PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsHourlyCompensations>,
    >,
    #[doc = "An array of all paid time off the employee is eligible for this pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_time_off:
        Option<Vec<PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensationsPaidTimeOff>>,
}

impl std::fmt::Display for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensations {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.employee_id).into(),
            if let Some(fixed_compensations) = &self.fixed_compensations {
                format!("{:?}", fixed_compensations).into()
            } else {
                String::new().into()
            },
            if let Some(hourly_compensations) = &self.hourly_compensations {
                format!("{:?}", hourly_compensations).into()
            } else {
                String::new().into()
            },
            if let Some(paid_time_off) = &self.paid_time_off {
                format!("{:?}", paid_time_off).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "employee_id".into(),
            "fixed_compensations".into(),
            "hourly_compensations".into(),
            "paid_time_off".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsRequestBody {
    #[doc = "The current version of the object. See the versioning guide for details using this \
             field."]
    pub version: String,
    pub employee_compensations: Vec<PutCompaniesCompanyIdPayrollsRequestBodyEmployeeCompensations>,
}

impl std::fmt::Display for PutCompaniesCompanyIdPayrollsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdPayrollsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            format!("{:?}", self.employee_compensations).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["version".into(), "employee_compensations".into()]
    }
}

#[doc = "An array of fixed compensations for the employee. Fixed compensations include tips, \
         bonuses, and one time reimbursements."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsFixedCompensations
{
    #[doc = "The name of the compensation. This also serves as the unique, immutable identifier \
             for this compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The amount of the compensation for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[doc = "The ID of the job for the compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
}

impl std :: fmt :: Display for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsFixedCompensations { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "{}" , serde_json :: to_string_pretty (self) . map_err (| _ | std :: fmt :: Error) ?) } }

#[cfg(feature = "tabled")]
impl tabled :: Tabled for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsFixedCompensations { const LENGTH : usize = 3 ; fn fields (& self) -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! [if let Some (name) = & self . name { format ! ("{:?}" , name) . into () } else { String :: new () . into () } , if let Some (amount) = & self . amount { format ! ("{:?}" , amount) . into () } else { String :: new () . into () } , if let Some (job_id) = & self . job_id { format ! ("{:?}" , job_id) . into () } else { String :: new () . into () }] } fn headers () -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! ["name" . into () , "amount" . into () , "job_id" . into ()] } }

#[doc = "An array of hourly compensations for the employee. Hourly compensations include regular, \
         overtime, and double overtime hours."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsHourlyCompensations
{
    #[doc = "The name of the compensation. This also serves as the unique, immutable identifier \
             for this compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of hours to be compensated for this pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
    #[doc = "The ID of the job for the compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<i64>,
}

impl std :: fmt :: Display for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsHourlyCompensations { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "{}" , serde_json :: to_string_pretty (self) . map_err (| _ | std :: fmt :: Error) ?) } }

#[cfg(feature = "tabled")]
impl tabled :: Tabled for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsHourlyCompensations { const LENGTH : usize = 3 ; fn fields (& self) -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! [if let Some (name) = & self . name { format ! ("{:?}" , name) . into () } else { String :: new () . into () } , if let Some (hours) = & self . hours { format ! ("{:?}" , hours) . into () } else { String :: new () . into () } , if let Some (job_id) = & self . job_id { format ! ("{:?}" , job_id) . into () } else { String :: new () . into () }] } fn headers () -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! ["name" . into () , "hours" . into () , "job_id" . into ()] } }

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsPaidTimeOff
{
    #[doc = "The name of the PTO. This also serves as the unique, immutable identifier for the \
             PTO."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The hours of this PTO taken during the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<String>,
}

impl std :: fmt :: Display for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsPaidTimeOff { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "{}" , serde_json :: to_string_pretty (self) . map_err (| _ | std :: fmt :: Error) ?) } }

#[cfg(feature = "tabled")]
impl tabled :: Tabled for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsPaidTimeOff { const LENGTH : usize = 2 ; fn fields (& self) -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! [if let Some (name) = & self . name { format ! ("{:?}" , name) . into () } else { String :: new () . into () } , if let Some (hours) = & self . hours { format ! ("{:?}" , hours) . into () } else { String :: new () . into () }] } fn headers () -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! ["name" . into () , "hours" . into ()] } }

#[doc = "PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensations."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensations { # [doc = "The ID of the employee."] pub employee_id : i64 , # [serde (default , skip_serializing_if = "Option::is_none")] pub fixed_compensations : Option < Vec < PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsFixedCompensations > > , # [serde (default , skip_serializing_if = "Option::is_none")] pub hourly_compensations : Option < Vec < PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsHourlyCompensations > > , # [doc = "An array of all paid time off the employee is eligible for this pay period."] # [serde (default , skip_serializing_if = "Option::is_none")] pub paid_time_off : Option < Vec < PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensationsPaidTimeOff > > , }

impl std :: fmt :: Display for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensations { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> Result < () , std :: fmt :: Error > { write ! (f , "{}" , serde_json :: to_string_pretty (self) . map_err (| _ | std :: fmt :: Error) ?) } }

#[cfg(feature = "tabled")]
impl tabled :: Tabled for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensations { const LENGTH : usize = 4 ; fn fields (& self) -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! [format ! ("{:?}" , self . employee_id) . into () , if let Some (fixed_compensations) = & self . fixed_compensations { format ! ("{:?}" , fixed_compensations) . into () } else { String :: new () . into () } , if let Some (hourly_compensations) = & self . hourly_compensations { format ! ("{:?}" , hourly_compensations) . into () } else { String :: new () . into () } , if let Some (paid_time_off) = & self . paid_time_off { format ! ("{:?}" , paid_time_off) . into () } else { String :: new () . into () }] } fn headers () -> Vec < std :: borrow :: Cow < 'static , str >> { vec ! ["employee_id" . into () , "fixed_compensations" . into () , "hourly_compensations" . into () , "paid_time_off" . into ()] } }

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBody { # [doc = "The current version of the object. See the versioning guide for details using this field."] pub version : String , pub employee_compensations : Vec < PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBodyEmployeeCompensations > , }

impl std::fmt::Display
    for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBody
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdPayrollsPayPeriodStartDatePayPeriodEndDateRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.version.clone().into(),
            format!("{:?}", self.employee_compensations).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["version".into(), "employee_compensations".into()]
    }
}

#[doc = "Information for the user who will be the primary payroll administrator for the new \
         company."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct User {
    #[doc = "The first name of the user who will be the primary payroll admin."]
    pub first_name: String,
    #[doc = "The last name of the user who will be the primary payroll admin."]
    pub last_name: String,
    #[doc = "The email of the user who will be the primary payroll admin."]
    pub email: String,
    #[doc = "The phone number of the user who will be the primary payroll admin."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
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
            self.first_name.clone().into(),
            self.last_name.clone().into(),
            self.email.clone().into(),
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "last_name".into(),
            "email".into(),
            "phone".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostPartnerManagedCompaniesRequestBodyCompany {
    #[doc = "The legal name of the company."]
    pub name: String,
    #[doc = "The name of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    #[doc = "The employer identification number (EIN) of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
}

impl std::fmt::Display for PostPartnerManagedCompaniesRequestBodyCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostPartnerManagedCompaniesRequestBodyCompany {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(trade_name) = &self.trade_name {
                format!("{:?}", trade_name).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "trade_name".into(), "ein".into()]
    }
}

#[doc = "PostPartnerManagedCompaniesRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostPartnerManagedCompaniesRequestBody {
    #[doc = "Information for the user who will be the primary payroll administrator for the new \
             company."]
    pub user: User,
    pub company: PostPartnerManagedCompaniesRequestBodyCompany,
}

impl std::fmt::Display for PostPartnerManagedCompaniesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostPartnerManagedCompaniesRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user).into(),
            format!("{:?}", self.company).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user".into(), "company".into()]
    }
}

#[doc = "PostPartnerManagedCompaniesResponse."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostPartnerManagedCompaniesResponse {
    #[doc = "Access token that can be used for OAuth access to the account. Access tokens expire \
             2 hours after they are issued."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "Refresh token that can be exchanged for a new access token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "Gusto’s UUID for the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_uuid: Option<String>,
}

impl std::fmt::Display for PostPartnerManagedCompaniesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostPartnerManagedCompaniesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(access_token) = &self.access_token {
                format!("{:?}", access_token).into()
            } else {
                String::new().into()
            },
            if let Some(refresh_token) = &self.refresh_token {
                format!("{:?}", refresh_token).into()
            } else {
                String::new().into()
            },
            if let Some(company_uuid) = &self.company_uuid {
                format!("{:?}", company_uuid).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "access_token".into(),
            "refresh_token".into(),
            "company_uuid".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Addresses {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_1: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Whether or not this is a primary address for the company. If set to true, the \
             address will be used as the mailing and filing address for the company and will be \
             added as a work location. If set to false or not included, the address will only be \
             added as a work location for the company. If multiple addresses are included, only \
             one should be marked as primary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<String>,
}

impl std::fmt::Display for Addresses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Addresses {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(street_1) = &self.street_1 {
                format!("{:?}", street_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_2) = &self.street_2 {
                format!("{:?}", street_2).into()
            } else {
                String::new().into()
            },
            if let Some(city) = &self.city {
                format!("{:?}", city).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
            } else {
                String::new().into()
            },
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
            if let Some(is_primary) = &self.is_primary {
                format!("{:?}", is_primary).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "street_1".into(),
            "street_2".into(),
            "city".into(),
            "zip".into(),
            "state".into(),
            "phone".into(),
            "is_primary".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostProvisionRequestBodyCompany {
    #[doc = "The legal name of the company."]
    pub name: String,
    #[doc = "The name of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trade_name: Option<String>,
    #[doc = "The employer identification number (EIN) of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[doc = "The states in which the company operates. States should be included by their two \
             letter code, i.e. NY for New York. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    #[doc = "The number of employees in the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_employees: Option<f64>,
    #[doc = "The locations for the company. This includes mailing, work, and filing addresses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<Addresses>>,
}

impl std::fmt::Display for PostProvisionRequestBodyCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostProvisionRequestBodyCompany {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(trade_name) = &self.trade_name {
                format!("{:?}", trade_name).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
            if let Some(states) = &self.states {
                format!("{:?}", states).into()
            } else {
                String::new().into()
            },
            if let Some(number_employees) = &self.number_employees {
                format!("{:?}", number_employees).into()
            } else {
                String::new().into()
            },
            if let Some(addresses) = &self.addresses {
                format!("{:?}", addresses).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "trade_name".into(),
            "ein".into(),
            "states".into(),
            "number_employees".into(),
            "addresses".into(),
        ]
    }
}

#[doc = "PostProvisionRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostProvisionRequestBody {
    #[doc = "Information for the user who will be the primary payroll administrator for the new \
             company."]
    pub user: User,
    pub company: PostProvisionRequestBodyCompany,
}

impl std::fmt::Display for PostProvisionRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostProvisionRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.user).into(),
            format!("{:?}", self.company).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["user".into(), "company".into()]
    }
}

#[doc = "PostProvisionResponse."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostProvisionResponse {
    #[doc = "A URL where the user should be redirected to complete their account setup inside of \
             Gusto."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_claim_url: Option<String>,
}

impl std::fmt::Display for PostProvisionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostProvisionResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(account_claim_url) = &self.account_claim_url {
            format!("{:?}", account_claim_url).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["account_claim_url".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetEmployeesEmployeeIdCustomFieldsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<EmployeeCustomField>>,
}

impl std::fmt::Display for GetEmployeesEmployeeIdCustomFieldsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetEmployeesEmployeeIdCustomFieldsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(custom_fields) = &self.custom_fields {
            format!("{:?}", custom_fields).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["custom_fields".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCompaniesCompanyIdCustomFieldsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CompanyCustomField>>,
}

impl std::fmt::Display for GetCompaniesCompanyIdCustomFieldsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCompaniesCompanyIdCustomFieldsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(custom_fields) = &self.custom_fields {
            format!("{:?}", custom_fields).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["custom_fields".into()]
    }
}

#[doc = "Must be \"Employee\" if send_offer is set to true."]
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
pub enum OnboardingPersonType {
    Employee,
    Contractor,
}

#[doc = "PostCompaniesCompanyIdJobApplicantsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdJobApplicantsRequestBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Must be \"Employee\" if send_offer is set to true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_person_type: Option<OnboardingPersonType>,
    #[doc = "Required if onboarding_person_type is set to \"Employee\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_offer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for PostCompaniesCompanyIdJobApplicantsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdJobApplicantsRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.first_name.clone().into(),
            self.last_name.clone().into(),
            self.email.clone().into(),
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(onboarding_person_type) = &self.onboarding_person_type {
                format!("{:?}", onboarding_person_type).into()
            } else {
                String::new().into()
            },
            if let Some(send_offer) = &self.send_offer {
                format!("{:?}", send_offer).into()
            } else {
                String::new().into()
            },
            if let Some(job_title) = &self.job_title {
                format!("{:?}", job_title).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_birth) = &self.date_of_birth {
                format!("{:?}", date_of_birth).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "last_name".into(),
            "email".into(),
            "phone".into(),
            "onboarding_person_type".into(),
            "send_offer".into(),
            "job_title".into(),
            "date_of_birth".into(),
            "start_date".into(),
        ]
    }
}

#[doc = "PutCompaniesCompanyIdJobApplicantsJobApplicantUuidRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdJobApplicantsJobApplicantUuidRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "Must be \"Employee\" if send_offer is set to true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_person_type: Option<OnboardingPersonType>,
    #[doc = "Required if onboarding_person_type is set to \"Employee\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub send_offer: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for PutCompaniesCompanyIdJobApplicantsJobApplicantUuidRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdJobApplicantsJobApplicantUuidRequestBody {
    const LENGTH: usize = 9;
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
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(onboarding_person_type) = &self.onboarding_person_type {
                format!("{:?}", onboarding_person_type).into()
            } else {
                String::new().into()
            },
            if let Some(send_offer) = &self.send_offer {
                format!("{:?}", send_offer).into()
            } else {
                String::new().into()
            },
            if let Some(job_title) = &self.job_title {
                format!("{:?}", job_title).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_birth) = &self.date_of_birth {
                format!("{:?}", date_of_birth).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "first_name".into(),
            "last_name".into(),
            "email".into(),
            "phone".into(),
            "onboarding_person_type".into(),
            "send_offer".into(),
            "job_title".into(),
            "date_of_birth".into(),
            "start_date".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCompaniesCompanyIdOrUuidPayrollReversalsResponse {
    #[doc = "The payroll run being reversed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reversed_payroll_id: Option<i64>,
    #[doc = "The payroll where the reversal was applied."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reversal_payroll_id: Option<i64>,
    #[doc = "A reason provided by the admin who created the reversal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Timestamp of when the reversal was approved."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[doc = "Category chosen by the admin who requested the reversal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Array of employee ids affected."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reversed_employee_ids: Option<Vec<i64>>,
}

impl std::fmt::Display for GetCompaniesCompanyIdOrUuidPayrollReversalsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCompaniesCompanyIdOrUuidPayrollReversalsResponse {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(reversed_payroll_id) = &self.reversed_payroll_id {
                format!("{:?}", reversed_payroll_id).into()
            } else {
                String::new().into()
            },
            if let Some(reversal_payroll_id) = &self.reversal_payroll_id {
                format!("{:?}", reversal_payroll_id).into()
            } else {
                String::new().into()
            },
            if let Some(reason) = &self.reason {
                format!("{:?}", reason).into()
            } else {
                String::new().into()
            },
            if let Some(approved_at) = &self.approved_at {
                format!("{:?}", approved_at).into()
            } else {
                String::new().into()
            },
            if let Some(category) = &self.category {
                format!("{:?}", category).into()
            } else {
                String::new().into()
            },
            if let Some(reversed_employee_ids) = &self.reversed_employee_ids {
                format!("{:?}", reversed_employee_ids).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "reversed_payroll_id".into(),
            "reversal_payroll_id".into(),
            "reason".into(),
            "approved_at".into(),
            "category".into(),
            "reversed_employee_ids".into(),
        ]
    }
}

#[doc = "PostCompaniesCompanyIdAdminsRequestBody."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostCompaniesCompanyIdAdminsRequestBody {
    #[doc = "The email of the admin. This will be used for the admin to log in to their account. \
             If the email matches an existing user, this will create an admin account for them."]
    pub first_name: String,
    #[doc = "The first name of the admin."]
    pub last_name: String,
    #[doc = "The last name of the admin."]
    pub email: String,
}

impl std::fmt::Display for PostCompaniesCompanyIdAdminsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostCompaniesCompanyIdAdminsRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.first_name.clone().into(),
            self.last_name.clone().into(),
            self.email.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["first_name".into(), "last_name".into(), "email".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCompaniesCompanyIdOrUuidFederalTaxDetailsResponse {
    #[doc = "The current version of the object. See the versioning guide for details using this \
             field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "What type of tax entity the company is. One of:\n- C-Corporation\n- S-Corporation\n- \
             Sole proprietor\n- LLC\n- LLP\n- Limited partnership\n- Co-ownership\n- \
             Association\n- Trusteeship\n- General partnership\n- Joint venture\n- Non-Profit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_type: Option<String>,
    #[doc = "The company's EIN"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[doc = "Whether the company is taxed as an S-Corporation. Tax payer types that may be taxed \
             as an S-Corporation include:\n- S-Corporation\n- C-Corporation\n- LLC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxable_as_scorp: Option<bool>,
    #[doc = "The form used by the company for federal tax filing. One of:\n- 941 (Quarterly \
             federal tax return form)\n- 944 (Annual federal tax return form)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_form: Option<String>,
    #[doc = "Whether the EIN was able to be verified as a valid EIN with the IRS. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein_verified: Option<bool>,
    #[doc = "The legal name of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
}

impl std::fmt::Display for GetCompaniesCompanyIdOrUuidFederalTaxDetailsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCompaniesCompanyIdOrUuidFederalTaxDetailsResponse {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(tax_payer_type) = &self.tax_payer_type {
                format!("{:?}", tax_payer_type).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
            if let Some(taxable_as_scorp) = &self.taxable_as_scorp {
                format!("{:?}", taxable_as_scorp).into()
            } else {
                String::new().into()
            },
            if let Some(filing_form) = &self.filing_form {
                format!("{:?}", filing_form).into()
            } else {
                String::new().into()
            },
            if let Some(ein_verified) = &self.ein_verified {
                format!("{:?}", ein_verified).into()
            } else {
                String::new().into()
            },
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "tax_payer_type".into(),
            "ein".into(),
            "taxable_as_scorp".into(),
            "filing_form".into(),
            "ein_verified".into(),
            "legal_name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdOrUuidFederalTaxDetailsRequestBody {
    #[doc = "The legal name of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[doc = "The EIN of of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[doc = "What type of tax entity the company is"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_type: Option<String>,
    #[doc = "The form used by the company for federal tax filing. One of:\n- 941 (Quarterly \
             federal tax return)\n- 944 (Annual federal tax return)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_form: Option<String>,
    #[doc = "Whether this company should be taxed as an S-Corporation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxable_as_scorp: Option<bool>,
    #[doc = "The current version of the object. See the versioning guide for details using this \
             field."]
    pub version: String,
}

impl std::fmt::Display for PutCompaniesCompanyIdOrUuidFederalTaxDetailsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdOrUuidFederalTaxDetailsRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
            if let Some(tax_payer_type) = &self.tax_payer_type {
                format!("{:?}", tax_payer_type).into()
            } else {
                String::new().into()
            },
            if let Some(filing_form) = &self.filing_form {
                format!("{:?}", filing_form).into()
            } else {
                String::new().into()
            },
            if let Some(taxable_as_scorp) = &self.taxable_as_scorp {
                format!("{:?}", taxable_as_scorp).into()
            } else {
                String::new().into()
            },
            self.version.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "legal_name".into(),
            "ein".into(),
            "tax_payer_type".into(),
            "filing_form".into(),
            "taxable_as_scorp".into(),
            "version".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PutCompaniesCompanyIdOrUuidFederalTaxDetailsResponse {
    #[doc = "The current version of the object. See the versioning guide for details using this \
             field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "What type of tax entity the company is. One of:\n- C-Corporation\n- S-Corporation\n- \
             Sole proprietor\n- LLC\n- LLP\n- Limited partnership\n- Co-ownership\n- \
             Association\n- Trusteeship\n- General partnership\n- Joint venture\n- Non-Profit"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_payer_type: Option<String>,
    #[doc = "The company's EIN"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    #[doc = "Whether the company is taxed as an S-Corporation. Tax payer types that may be taxed \
             as an S-Corporation include:\n- S-Corporation\n- C-Corporation\n- LLC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub taxable_as_scorp: Option<bool>,
    #[doc = "The form used by the company for federal tax filing. One of:\n- 941 (Quarterly \
             federal tax return form)\n- 944 (Annual federal tax return form)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filing_form: Option<String>,
    #[doc = "Whether the EIN was able to be verified as a valid EIN with the IRS. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ein_verified: Option<bool>,
    #[doc = "The legal name of the company"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
}

impl std::fmt::Display for PutCompaniesCompanyIdOrUuidFederalTaxDetailsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PutCompaniesCompanyIdOrUuidFederalTaxDetailsResponse {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
            if let Some(tax_payer_type) = &self.tax_payer_type {
                format!("{:?}", tax_payer_type).into()
            } else {
                String::new().into()
            },
            if let Some(ein) = &self.ein {
                format!("{:?}", ein).into()
            } else {
                String::new().into()
            },
            if let Some(taxable_as_scorp) = &self.taxable_as_scorp {
                format!("{:?}", taxable_as_scorp).into()
            } else {
                String::new().into()
            },
            if let Some(filing_form) = &self.filing_form {
                format!("{:?}", filing_form).into()
            } else {
                String::new().into()
            },
            if let Some(ein_verified) = &self.ein_verified {
                format!("{:?}", ein_verified).into()
            } else {
                String::new().into()
            },
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "version".into(),
            "tax_payer_type".into(),
            "ein".into(),
            "taxable_as_scorp".into(),
            "filing_form".into(),
            "ein_verified".into(),
            "legal_name".into(),
        ]
    }
}
