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

#[doc = "An ENUM of employment type"]
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
pub enum EmploymentType {
    #[serde(rename = "CONTRACTOR")]
    #[display("CONTRACTOR")]
    Contractor,
    #[serde(rename = "SALARIED_FT")]
    #[display("SALARIED_FT")]
    SalariedFt,
    #[serde(rename = "SALARIED_PT")]
    #[display("SALARIED_PT")]
    SalariedPt,
    #[serde(rename = "HOURLY_FT")]
    #[display("HOURLY_FT")]
    HourlyFt,
    #[serde(rename = "HOURLY_PT")]
    #[display("HOURLY_PT")]
    HourlyPt,
    #[serde(rename = "TEMP")]
    #[display("TEMP")]
    Temp,
}

#[doc = "The employee's gender"]
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
pub enum Gender {
    #[serde(rename = "MALE")]
    #[display("MALE")]
    Male,
    #[serde(rename = "FEMALE")]
    #[display("FEMALE")]
    Female,
}

#[doc = "The employee's identified gender"]
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
pub enum IdentifiedGender {
    #[serde(rename = "MALE")]
    #[display("MALE")]
    Male,
    #[serde(rename = "FEMALE")]
    #[display("FEMALE")]
    Female,
    #[serde(rename = "NONBINARY")]
    #[display("NONBINARY")]
    Nonbinary,
}

#[doc = "The employee's role status - roleState meanings:\n\nINIT: An initial record of an \
         individual. An offer has not been made and they have not started working at the \
         company.\n\nHIRED: An offer has been made but they have not accepted or started \
         yet.\n\nACCEPTED: An offer has been made and they have accepted, but they have not \
         started yet.\n\nACTIVE: The employee currently works at the company and their start date \
         is today or in the past.\n\nTERMINATED: The employee is no longer active."]
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
pub enum RoleState {
    #[serde(rename = "INIT")]
    #[display("INIT")]
    Init,
    #[serde(rename = "HIRED")]
    #[display("HIRED")]
    Hired,
    #[serde(rename = "ACCEPTED")]
    #[display("ACCEPTED")]
    Accepted,
    #[serde(rename = "ACTIVE")]
    #[display("ACTIVE")]
    Active,
    #[serde(rename = "TERMINATED")]
    #[display("TERMINATED")]
    Terminated,
}

#[doc = "An employee model object."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Employee {
    #[doc = "This is the unique role ID of the employee. A role ID exists per 1 and only 1 \
             company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "This is the unique user ID of the employee. A userID can span across 1 or many \
             companies."]
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "Full name of the employee"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "preferredFirstName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_first_name: Option<String>,
    #[serde(
        rename = "preferredLastName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_last_name: Option<String>,
    #[doc = "First name of the employee"]
    #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Last name of the employee"]
    #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "An ENUM of employment type"]
    #[serde(
        rename = "employmentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub employment_type: Option<EmploymentType>,
    #[doc = "The employee's work title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The employee's gender"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[doc = "The employee's identified gender"]
    #[serde(
        rename = "identifiedGender",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub identified_gender: Option<IdentifiedGender>,
    #[doc = "The employee's department name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[doc = "An address object as stored within Rippling."]
    #[serde(
        rename = "workLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_location: Option<Address>,
    #[doc = "The work location nickname"]
    #[serde(
        rename = "worklocationNickname",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub worklocation_nickname: Option<String>,
    #[serde(rename = "spokeId", default, skip_serializing_if = "Option::is_none")]
    pub spoke_id: Option<String>,
    #[doc = "The employee's end date"]
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The employee's role status - roleState meanings:\n\nINIT: An initial record of an \
             individual. An offer has not been made and they have not started working at the \
             company.\n\nHIRED: An offer has been made but they have not accepted or started \
             yet.\n\nACCEPTED: An offer has been made and they have accepted, but they have not \
             started yet.\n\nACTIVE: The employee currently works at the company and their start \
             date is today or in the past.\n\nTERMINATED: The employee is no longer active."]
    #[serde(rename = "roleState", default, skip_serializing_if = "Option::is_none")]
    pub role_state: Option<RoleState>,
    #[doc = "The employee's work email"]
    #[serde(rename = "workEmail", default, skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
    #[doc = "The unique identifier of the employee's manager. This value can be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<String>,
    #[doc = "custom_fields."]
    #[serde(
        rename = "customFields",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_fields: Option<CustomField>,
    #[doc = "Whether the employee is an international employee or not."]
    #[serde(
        rename = "isInternational",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_international: Option<bool>,
    #[doc = "Whether the employee is a manger"]
    #[serde(rename = "isManager", default, skip_serializing_if = "Option::is_none")]
    pub is_manager: Option<bool>,
    #[doc = "The employee's weekly work schedule"]
    #[serde(
        rename = "workSchedule",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_schedule: Option<serde_json::Value>,
    #[doc = "Whether the employee's job is remote"]
    #[serde(rename = "isRemote", default, skip_serializing_if = "Option::is_none")]
    pub is_remote: Option<bool>,
    #[doc = "This indicates the sequential employee number within their company. This number \
             continues to grow as each employee is onboarded. i.e if you are the 65th employee to \
             join the company with 32 active employees, the employeeNumber would be 65."]
    #[serde(
        rename = "employeeNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub employee_number: Option<String>,
    #[doc = "The level of the employee"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "An array of the teams that the employee is on"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<serde_json::Value>>,
    #[doc = "The photo of the employee stored in Rippling"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photo: Option<String>,
    #[doc = "The small photo of the employee stored in Rippling"]
    #[serde(
        rename = "smallPhoto",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub small_photo: Option<String>,
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
    const LENGTH: usize = 29;
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
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_first_name) = &self.preferred_first_name {
                format!("{:?}", preferred_first_name).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_last_name) = &self.preferred_last_name {
                format!("{:?}", preferred_last_name).into()
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
            if let Some(employment_type) = &self.employment_type {
                format!("{:?}", employment_type).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(gender) = &self.gender {
                format!("{:?}", gender).into()
            } else {
                String::new().into()
            },
            if let Some(identified_gender) = &self.identified_gender {
                format!("{:?}", identified_gender).into()
            } else {
                String::new().into()
            },
            if let Some(department) = &self.department {
                format!("{:?}", department).into()
            } else {
                String::new().into()
            },
            if let Some(work_location) = &self.work_location {
                format!("{:?}", work_location).into()
            } else {
                String::new().into()
            },
            if let Some(worklocation_nickname) = &self.worklocation_nickname {
                format!("{:?}", worklocation_nickname).into()
            } else {
                String::new().into()
            },
            if let Some(spoke_id) = &self.spoke_id {
                format!("{:?}", spoke_id).into()
            } else {
                String::new().into()
            },
            if let Some(end_date) = &self.end_date {
                format!("{:?}", end_date).into()
            } else {
                String::new().into()
            },
            if let Some(role_state) = &self.role_state {
                format!("{:?}", role_state).into()
            } else {
                String::new().into()
            },
            if let Some(work_email) = &self.work_email {
                format!("{:?}", work_email).into()
            } else {
                String::new().into()
            },
            if let Some(manager) = &self.manager {
                format!("{:?}", manager).into()
            } else {
                String::new().into()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields).into()
            } else {
                String::new().into()
            },
            if let Some(is_international) = &self.is_international {
                format!("{:?}", is_international).into()
            } else {
                String::new().into()
            },
            if let Some(is_manager) = &self.is_manager {
                format!("{:?}", is_manager).into()
            } else {
                String::new().into()
            },
            if let Some(work_schedule) = &self.work_schedule {
                format!("{:?}", work_schedule).into()
            } else {
                String::new().into()
            },
            if let Some(is_remote) = &self.is_remote {
                format!("{:?}", is_remote).into()
            } else {
                String::new().into()
            },
            if let Some(employee_number) = &self.employee_number {
                format!("{:?}", employee_number).into()
            } else {
                String::new().into()
            },
            if let Some(level) = &self.level {
                format!("{:?}", level).into()
            } else {
                String::new().into()
            },
            if let Some(teams) = &self.teams {
                format!("{:?}", teams).into()
            } else {
                String::new().into()
            },
            if let Some(photo) = &self.photo {
                format!("{:?}", photo).into()
            } else {
                String::new().into()
            },
            if let Some(small_photo) = &self.small_photo {
                format!("{:?}", small_photo).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "user_id".into(),
            "name".into(),
            "preferred_first_name".into(),
            "preferred_last_name".into(),
            "first_name".into(),
            "last_name".into(),
            "employment_type".into(),
            "title".into(),
            "gender".into(),
            "identified_gender".into(),
            "department".into(),
            "work_location".into(),
            "worklocation_nickname".into(),
            "spoke_id".into(),
            "end_date".into(),
            "role_state".into(),
            "work_email".into(),
            "manager".into(),
            "custom_fields".into(),
            "is_international".into(),
            "is_manager".into(),
            "work_schedule".into(),
            "is_remote".into(),
            "employee_number".into(),
            "level".into(),
            "teams".into(),
            "photo".into(),
            "small_photo".into(),
        ]
    }
}

#[doc = "A work location object."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WorkLocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[doc = "An address object as stored within Rippling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
}

impl std::fmt::Display for WorkLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkLocation {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(nickname) = &self.nickname {
                format!("{:?}", nickname).into()
            } else {
                String::new().into()
            },
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "nickname".into(), "address".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct SteLocationCode {
    #[serde(
        rename = "locationCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub location_code: Option<String>,
    #[serde(rename = "stateCode", default, skip_serializing_if = "Option::is_none")]
    pub state_code: Option<String>,
    #[serde(
        rename = "countyCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub county_code: Option<String>,
    #[serde(rename = "cityCode", default, skip_serializing_if = "Option::is_none")]
    pub city_code: Option<String>,
    #[serde(
        rename = "schoolCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub school_code: Option<String>,
    #[serde(
        rename = "municipalityCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub municipality_code: Option<String>,
    #[serde(rename = "psdCode", default, skip_serializing_if = "Option::is_none")]
    pub psd_code: Option<String>,
    #[serde(
        rename = "transitDistrictCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transit_district_code: Option<String>,
    #[serde(
        rename = "isOverridden",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_overridden: Option<String>,
}

impl std::fmt::Display for SteLocationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for SteLocationCode {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(location_code) = &self.location_code {
                format!("{:?}", location_code).into()
            } else {
                String::new().into()
            },
            if let Some(state_code) = &self.state_code {
                format!("{:?}", state_code).into()
            } else {
                String::new().into()
            },
            if let Some(county_code) = &self.county_code {
                format!("{:?}", county_code).into()
            } else {
                String::new().into()
            },
            if let Some(city_code) = &self.city_code {
                format!("{:?}", city_code).into()
            } else {
                String::new().into()
            },
            if let Some(school_code) = &self.school_code {
                format!("{:?}", school_code).into()
            } else {
                String::new().into()
            },
            if let Some(municipality_code) = &self.municipality_code {
                format!("{:?}", municipality_code).into()
            } else {
                String::new().into()
            },
            if let Some(psd_code) = &self.psd_code {
                format!("{:?}", psd_code).into()
            } else {
                String::new().into()
            },
            if let Some(transit_district_code) = &self.transit_district_code {
                format!("{:?}", transit_district_code).into()
            } else {
                String::new().into()
            },
            if let Some(is_overridden) = &self.is_overridden {
                format!("{:?}", is_overridden).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "location_code".into(),
            "state_code".into(),
            "county_code".into(),
            "city_code".into(),
            "school_code".into(),
            "municipality_code".into(),
            "psd_code".into(),
            "transit_district_code".into(),
            "is_overridden".into(),
        ]
    }
}

#[doc = "An address object as stored within Rippling."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Address {
    #[serde(
        rename = "streetLine1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_line_1: Option<String>,
    #[serde(
        rename = "streetLine2",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub street_line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(rename = "isRemote", default, skip_serializing_if = "Option::is_none")]
    pub is_remote: Option<bool>,
    #[serde(
        rename = "steLocationCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ste_location_code: Option<SteLocationCode>,
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
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(street_line_1) = &self.street_line_1 {
                format!("{:?}", street_line_1).into()
            } else {
                String::new().into()
            },
            if let Some(street_line_2) = &self.street_line_2 {
                format!("{:?}", street_line_2).into()
            } else {
                String::new().into()
            },
            if let Some(zip) = &self.zip {
                format!("{:?}", zip).into()
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
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(is_remote) = &self.is_remote {
                format!("{:?}", is_remote).into()
            } else {
                String::new().into()
            },
            if let Some(ste_location_code) = &self.ste_location_code {
                format!("{:?}", ste_location_code).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "street_line_1".into(),
            "street_line_2".into(),
            "zip".into(),
            "city".into(),
            "state".into(),
            "country".into(),
            "phone".into(),
            "is_remote".into(),
            "ste_location_code".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Group {
    #[doc = "User-readable name of a Rippling group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Your id for the group; this should a unique string identifier."]
    #[serde(rename = "spokeId", default, skip_serializing_if = "Option::is_none")]
    pub spoke_id: Option<String>,
    #[doc = "The version unique identifier of the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "An array of employee Rippling ids."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
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
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(spoke_id) = &self.spoke_id {
                format!("{:?}", spoke_id).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
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
            "name".into(),
            "id".into(),
            "spoke_id".into(),
            "version".into(),
            "users".into(),
        ]
    }
}

#[doc = "A company department object."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Department {
    #[doc = "Name of the department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Unique identifier of the department"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "id of the parent department, if one exists"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
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
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "id".into(), "parent".into()]
    }
}

#[doc = "Denotes the type of the custom field."]
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
    #[serde(rename = "TEXT")]
    #[display("TEXT")]
    Text,
    #[serde(rename = "DATE")]
    #[display("DATE")]
    Date,
    #[serde(rename = "NUMBER")]
    #[display("NUMBER")]
    Number,
    #[serde(rename = "CURRENCY")]
    #[display("CURRENCY")]
    Currency,
    #[serde(rename = "PERCENTAGE")]
    #[display("PERCENTAGE")]
    Percentage,
    #[serde(rename = "SELECT")]
    #[display("SELECT")]
    Select,
    #[serde(rename = "FILE")]
    #[display("FILE")]
    File,
    #[serde(rename = "ID")]
    #[display("ID")]
    Id,
    #[serde(rename = "RADIO")]
    #[display("RADIO")]
    Radio,
    #[serde(rename = "TEXTAREA")]
    #[display("TEXTAREA")]
    Textarea,
}

#[doc = "A Custom Fields object within Rippling."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomFields {
    #[doc = "The identifier of the specific custom field."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Denotes the type of the custom field."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[doc = "The title of the custom field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Denotes whether the custom field is or is not mandatory"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomFields {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(required) = &self.required {
                format!("{:?}", required).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "type_".into(),
            "title".into(),
            "required".into(),
        ]
    }
}

#[doc = "A company object as represented within Rippling."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Company {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "An address object as stored within Rippling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(
        rename = "workLocations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_locations: Option<Vec<WorkLocation>>,
    #[serde(
        rename = "primaryEmail",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "This model represents the legal entities inside of a given company. Legal entities \
             based in Canada (CA) are currently supported at this time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Entities>,
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
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
            if let Some(work_locations) = &self.work_locations {
                format!("{:?}", work_locations).into()
            } else {
                String::new().into()
            },
            if let Some(primary_email) = &self.primary_email {
                format!("{:?}", primary_email).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(entities) = &self.entities {
                format!("{:?}", entities).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "address".into(),
            "work_locations".into(),
            "primary_email".into(),
            "phone".into(),
            "name".into(),
            "entities".into(),
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
    #[serde(rename = "PENDING")]
    #[display("PENDING")]
    Pending,
    #[serde(rename = "APPROVED")]
    #[display("APPROVED")]
    Approved,
    #[serde(rename = "REJECTED")]
    #[display("REJECTED")]
    Rejected,
    #[serde(rename = "CANCELED")]
    #[display("CANCELED")]
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
pub enum LeaveTypeUniqueId {
    #[serde(rename = "VACATION")]
    #[display("VACATION")]
    Vacation,
    #[serde(rename = "SICK")]
    #[display("SICK")]
    Sick,
    #[serde(rename = "JURY_DUTY")]
    #[display("JURY_DUTY")]
    JuryDuty,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Dates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<chrono::NaiveDate>,
    #[serde(
        rename = "numMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub num_minutes: Option<i64>,
}

impl std::fmt::Display for Dates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Dates {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(date) = &self.date {
                format!("{:?}", date).into()
            } else {
                String::new().into()
            },
            if let Some(num_minutes) = &self.num_minutes {
                format!("{:?}", num_minutes).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["date".into(), "num_minutes".into()]
    }
}

#[doc = "This indicates the system that manages the Leave Request. PTO = managed by Rippling's \
         Time Off app. LEAVES = managed by Rippling's Leave Management app. TILT = managed by \
         third-party partner Tilt."]
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
pub enum ManagedBy {
    #[serde(rename = "PTO")]
    #[display("PTO")]
    Pto,
    #[serde(rename = "LEAVES")]
    #[display("LEAVES")]
    Leaves,
    #[serde(rename = "TILT")]
    #[display("TILT")]
    Tilt,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PartialDays {
    #[serde(
        rename = "partialDay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub partial_day: Option<chrono::NaiveDate>,
    #[serde(
        rename = "numMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub num_minutes: Option<i64>,
}

impl std::fmt::Display for PartialDays {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PartialDays {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(partial_day) = &self.partial_day {
                format!("{:?}", partial_day).into()
            } else {
                String::new().into()
            },
            if let Some(num_minutes) = &self.num_minutes {
                format!("{:?}", num_minutes).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["partial_day".into(), "num_minutes".into()]
    }
}

#[doc = "Leave request object."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LeaveRequest {
    #[doc = "Unique identifier of the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "createdAt", default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt", default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[doc = "Unique identifier of the employee who is taking leave."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "roleName", default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<String>,
    #[doc = "Unique identifier of the employee who made the request (in most cases this is the \
             same as role)."]
    #[serde(
        rename = "requestedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(
        rename = "startDateStartTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_start_time: Option<String>,
    #[serde(
        rename = "endDateEndTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date_end_time: Option<String>,
    #[serde(
        rename = "startDateCustomHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_custom_hours: Option<String>,
    #[serde(
        rename = "endDateCustomHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date_custom_hours: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "numHours", default, skip_serializing_if = "Option::is_none")]
    pub num_hours: Option<i64>,
    #[serde(
        rename = "numMinutes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub num_minutes: Option<i64>,
    #[serde(
        rename = "leavePolicy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub leave_policy: Option<String>,
    #[serde(
        rename = "leaveTypeUniqueId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub leave_type_unique_id: Option<LeaveTypeUniqueId>,
    #[serde(
        rename = "policyDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub policy_display_name: Option<String>,
    #[serde(
        rename = "reasonForLeave",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reason_for_leave: Option<String>,
    #[serde(
        rename = "processedAt",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub processed_at: Option<String>,
    #[doc = "Unique identifier of the employee who approved or rejected the request. This may be \
             null."]
    #[serde(
        rename = "processedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub processed_by: Option<String>,
    #[serde(
        rename = "processedByName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub processed_by_name: Option<String>,
    #[doc = "Timezone of the role. This will be work location timezone, or home timezone for \
             employees without a work location."]
    #[serde(
        rename = "roleTimezone",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub role_timezone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<Dates>>,
    #[doc = "If the leave request is paid this will be TRUE. Otherwise, this will be FALSE."]
    #[serde(rename = "isPaid", default, skip_serializing_if = "Option::is_none")]
    pub is_paid: Option<bool>,
    #[doc = "This indicates the system that manages the Leave Request. PTO = managed by \
             Rippling's Time Off app. LEAVES = managed by Rippling's Leave Management app. TILT = \
             managed by third-party partner Tilt."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<ManagedBy>,
    #[serde(
        rename = "partialDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub partial_days: Option<Vec<Option<PartialDays>>>,
}

impl std::fmt::Display for LeaveRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LeaveRequest {
    const LENGTH: usize = 29;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(role_name) = &self.role_name {
                format!("{:?}", role_name).into()
            } else {
                String::new().into()
            },
            if let Some(requested_by) = &self.requested_by {
                format!("{:?}", requested_by).into()
            } else {
                String::new().into()
            },
            if let Some(requested_by_name) = &self.requested_by_name {
                format!("{:?}", requested_by_name).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
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
            if let Some(start_date_start_time) = &self.start_date_start_time {
                format!("{:?}", start_date_start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_date_end_time) = &self.end_date_end_time {
                format!("{:?}", end_date_end_time).into()
            } else {
                String::new().into()
            },
            if let Some(start_date_custom_hours) = &self.start_date_custom_hours {
                format!("{:?}", start_date_custom_hours).into()
            } else {
                String::new().into()
            },
            if let Some(end_date_custom_hours) = &self.end_date_custom_hours {
                format!("{:?}", end_date_custom_hours).into()
            } else {
                String::new().into()
            },
            if let Some(comments) = &self.comments {
                format!("{:?}", comments).into()
            } else {
                String::new().into()
            },
            if let Some(num_hours) = &self.num_hours {
                format!("{:?}", num_hours).into()
            } else {
                String::new().into()
            },
            if let Some(num_minutes) = &self.num_minutes {
                format!("{:?}", num_minutes).into()
            } else {
                String::new().into()
            },
            if let Some(leave_policy) = &self.leave_policy {
                format!("{:?}", leave_policy).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type_unique_id) = &self.leave_type_unique_id {
                format!("{:?}", leave_type_unique_id).into()
            } else {
                String::new().into()
            },
            if let Some(policy_display_name) = &self.policy_display_name {
                format!("{:?}", policy_display_name).into()
            } else {
                String::new().into()
            },
            if let Some(reason_for_leave) = &self.reason_for_leave {
                format!("{:?}", reason_for_leave).into()
            } else {
                String::new().into()
            },
            if let Some(processed_at) = &self.processed_at {
                format!("{:?}", processed_at).into()
            } else {
                String::new().into()
            },
            if let Some(processed_by) = &self.processed_by {
                format!("{:?}", processed_by).into()
            } else {
                String::new().into()
            },
            if let Some(processed_by_name) = &self.processed_by_name {
                format!("{:?}", processed_by_name).into()
            } else {
                String::new().into()
            },
            if let Some(role_timezone) = &self.role_timezone {
                format!("{:?}", role_timezone).into()
            } else {
                String::new().into()
            },
            if let Some(dates) = &self.dates {
                format!("{:?}", dates).into()
            } else {
                String::new().into()
            },
            if let Some(is_paid) = &self.is_paid {
                format!("{:?}", is_paid).into()
            } else {
                String::new().into()
            },
            if let Some(managed_by) = &self.managed_by {
                format!("{:?}", managed_by).into()
            } else {
                String::new().into()
            },
            if let Some(partial_days) = &self.partial_days {
                format!("{:?}", partial_days).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "role".into(),
            "role_name".into(),
            "requested_by".into(),
            "requested_by_name".into(),
            "status".into(),
            "start_date".into(),
            "end_date".into(),
            "start_date_start_time".into(),
            "end_date_end_time".into(),
            "start_date_custom_hours".into(),
            "end_date_custom_hours".into(),
            "comments".into(),
            "num_hours".into(),
            "num_minutes".into(),
            "leave_policy".into(),
            "leave_type_unique_id".into(),
            "policy_display_name".into(),
            "reason_for_leave".into(),
            "processed_at".into(),
            "processed_by".into(),
            "processed_by_name".into(),
            "role_timezone".into(),
            "dates".into(),
            "is_paid".into(),
            "managed_by".into(),
            "partial_days".into(),
        ]
    }
}

#[doc = "Leave balances object"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LeaveBalances {
    #[doc = "This is the unique role ID of the company leave types. Corresponds to the ids in \
             response of GET Company Leave Types"]
    #[serde(
        rename = "companyLeaveType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub company_leave_type: Option<String>,
    #[doc = "true if employee's balance corresponding to the company leave type is unlimited, \
             else false"]
    #[serde(
        rename = "isBalanceUnlimited",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_balance_unlimited: Option<bool>,
    #[doc = "The remaining balance in minutes for the employee corresponding to the company leave \
             type with future leave requests considered."]
    #[serde(
        rename = "balanceWithFutureRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_with_future_requests: Option<f64>,
    #[doc = "The remaining balance in minutes for the employee corresponding to the company leave \
             type with future leave requests not considered."]
    #[serde(
        rename = "balanceWithoutFutureRequests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub balance_without_future_requests: Option<f64>,
}

impl std::fmt::Display for LeaveBalances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LeaveBalances {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company_leave_type) = &self.company_leave_type {
                format!("{:?}", company_leave_type).into()
            } else {
                String::new().into()
            },
            if let Some(is_balance_unlimited) = &self.is_balance_unlimited {
                format!("{:?}", is_balance_unlimited).into()
            } else {
                String::new().into()
            },
            if let Some(balance_with_future_requests) = &self.balance_with_future_requests {
                format!("{:?}", balance_with_future_requests).into()
            } else {
                String::new().into()
            },
            if let Some(balance_without_future_requests) = &self.balance_without_future_requests {
                format!("{:?}", balance_without_future_requests).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company_leave_type".into(),
            "is_balance_unlimited".into(),
            "balance_with_future_requests".into(),
            "balance_without_future_requests".into(),
        ]
    }
}

#[doc = "Company leave request object"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyLeaveType {
    #[doc = "Unique identifier of the company leave request"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Company leave type key"]
    #[serde(rename = "leaveType", default, skip_serializing_if = "Option::is_none")]
    pub leave_type: Option<String>,
    #[doc = "Company leave type name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Is leave type unpaid"]
    #[serde(rename = "isUnpaid", default, skip_serializing_if = "Option::is_none")]
    pub is_unpaid: Option<bool>,
}

impl std::fmt::Display for CompanyLeaveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyLeaveType {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type) = &self.leave_type {
                format!("{:?}", leave_type).into()
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
            if let Some(is_unpaid) = &self.is_unpaid {
                format!("{:?}", is_unpaid).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "leave_type".into(),
            "name".into(),
            "description".into(),
            "is_unpaid".into(),
        ]
    }
}

#[doc = "A team is a self-defined group of employees within Rippling."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Team {
    #[doc = "The identifier of the team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the team."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The parent team (if this team is a subteam within a larger team)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}

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
    const LENGTH: usize = 3;
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
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into(), "parent".into()]
    }
}

#[doc = "Levels enable for self-defined,company-wide position levels, such as Manager, Engineering \
         Manager, Executive, etc."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Level {
    #[doc = "Unique identifier of the level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Name of the level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unique identifier of the parent level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Level {
    const LENGTH: usize = 3;
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
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "name".into(), "parent".into()]
    }
}

#[doc = "Information about the Rippling user whose token is being used to access Rippling's API."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AuthenticatedUserMe {
    #[doc = "Unied identifier of the user (likely an admin)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Work email of the user."]
    #[serde(rename = "workEmail", default, skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
    #[doc = "Unique identifier of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
}

impl std::fmt::Display for AuthenticatedUserMe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AuthenticatedUserMe {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(work_email) = &self.work_email {
                format!("{:?}", work_email).into()
            } else {
                String::new().into()
            },
            if let Some(company) = &self.company {
                format!("{:?}", company).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "work_email".into(), "company".into()]
    }
}

#[doc = "This payload should be used when updating existing groups."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GroupUpdatePayload {
    #[doc = "The name of the Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The external identifier of the Group."]
    #[serde(rename = "spokeId", default, skip_serializing_if = "Option::is_none")]
    pub spoke_id: Option<String>,
    #[doc = "The array of users within the Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<serde_json::Value>>,
    #[doc = "The version identifier of the Group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl std::fmt::Display for GroupUpdatePayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GroupUpdatePayload {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(spoke_id) = &self.spoke_id {
                format!("{:?}", spoke_id).into()
            } else {
                String::new().into()
            },
            if let Some(users) = &self.users {
                format!("{:?}", users).into()
            } else {
                String::new().into()
            },
            if let Some(version) = &self.version {
                format!("{:?}", version).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "spoke_id".into(),
            "users".into(),
            "version".into(),
        ]
    }
}

#[doc = "An ENUM string value, denoting the frequency at which the candidate should be paid once \
         the role begins. Note, the PAY_PERIOD ENUM implies the candidate is paid as per a custom \
         pay period."]
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
pub enum SalaryUnit {
    #[serde(rename = "HOUR")]
    #[display("HOUR")]
    Hour,
    #[serde(rename = "DAY")]
    #[display("DAY")]
    Day,
    #[serde(rename = "WEEK")]
    #[display("WEEK")]
    Week,
    #[serde(rename = "MONTH")]
    #[display("MONTH")]
    Month,
    #[serde(rename = "PAY_PERIOD")]
    #[display("PAY_PERIOD")]
    PayPeriod,
}

#[doc = "The ENUM type of employment the user will have within Rippling."]
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
pub enum CandidateEmploymentType {
    #[serde(rename = "CONTRACTOR")]
    #[display("CONTRACTOR")]
    Contractor,
    #[serde(rename = "SALARIED_PT")]
    #[display("SALARIED_PT")]
    SalariedPt,
    #[serde(rename = "SALARIED_FT")]
    #[display("SALARIED_FT")]
    SalariedFt,
    #[serde(rename = "HOURLY_FT")]
    #[display("HOURLY_FT")]
    HourlyFt,
    #[serde(rename = "HOURLY_PT")]
    #[display("HOURLY_PT")]
    HourlyPt,
    #[serde(rename = "TEMP")]
    #[display("TEMP")]
    Temp,
}

#[doc = "An array of json objects containing file names and public file URLs containing documents \
         pertaining to the candidate."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Attachments {
    #[doc = "The file name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[doc = "The public URL and name of a pdf/docx/doc/odt file containing documents pertaining \
             to the candidate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
}

impl std::fmt::Display for Attachments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Attachments {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(file_name) = &self.file_name {
                format!("{:?}", file_name).into()
            } else {
                String::new().into()
            },
            if let Some(file_url) = &self.file_url {
                format!("{:?}", file_url).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["file_name".into(), "file_url".into()]
    }
}

#[doc = "The Rippling candidate model."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Candidate {
    #[doc = "The candidate's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The candidate's email."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The candidate's job title."]
    #[serde(rename = "jobTitle", default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<String>,
    #[doc = "The candidate's phone number."]
    #[serde(
        rename = "phoneNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number: Option<String>,
    #[doc = "The unique identifier of the candidate from the ATS."]
    #[serde(
        rename = "candidateId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub candidate_id: Option<String>,
    #[doc = "The would-be start date of the candidate."]
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[doc = "An ENUM string value, denoting the frequency at which the candidate should be paid \
             once the role begins. Note, the PAY_PERIOD ENUM implies the candidate is paid as per \
             a custom pay period."]
    #[serde(
        rename = "salaryUnit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub salary_unit: Option<SalaryUnit>,
    #[doc = "The decimal value that the candidate gets paid every salaryUnit time period."]
    #[serde(
        rename = "salaryPerUnit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub salary_per_unit: Option<f64>,
    #[doc = "The bonus cash given to the candidate as a part of a one time payment, with two \
             decimal digit precision."]
    #[serde(
        rename = "signingBonus",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub signing_bonus: Option<f64>,
    #[doc = "A string field of the official currency as listed in ISO 4217."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[doc = "The number of shares that will be given to the candidate."]
    #[serde(
        rename = "equityShares",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub equity_shares: Option<i64>,
    #[doc = "This is the id of the department from GET/departments."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    #[doc = "The ENUM type of employment the user will have within Rippling."]
    #[serde(
        rename = "employmentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub employment_type: Option<CandidateEmploymentType>,
    #[doc = "This is the id of the worklocation from GET/work_locations."]
    #[serde(
        rename = "workLocation",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub work_location: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachments>>,
}

impl std::fmt::Display for Candidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Candidate {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(email) = &self.email {
                format!("{:?}", email).into()
            } else {
                String::new().into()
            },
            if let Some(job_title) = &self.job_title {
                format!("{:?}", job_title).into()
            } else {
                String::new().into()
            },
            if let Some(phone_number) = &self.phone_number {
                format!("{:?}", phone_number).into()
            } else {
                String::new().into()
            },
            if let Some(candidate_id) = &self.candidate_id {
                format!("{:?}", candidate_id).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(salary_unit) = &self.salary_unit {
                format!("{:?}", salary_unit).into()
            } else {
                String::new().into()
            },
            if let Some(salary_per_unit) = &self.salary_per_unit {
                format!("{:?}", salary_per_unit).into()
            } else {
                String::new().into()
            },
            if let Some(signing_bonus) = &self.signing_bonus {
                format!("{:?}", signing_bonus).into()
            } else {
                String::new().into()
            },
            if let Some(currency) = &self.currency {
                format!("{:?}", currency).into()
            } else {
                String::new().into()
            },
            if let Some(equity_shares) = &self.equity_shares {
                format!("{:?}", equity_shares).into()
            } else {
                String::new().into()
            },
            if let Some(department) = &self.department {
                format!("{:?}", department).into()
            } else {
                String::new().into()
            },
            if let Some(employment_type) = &self.employment_type {
                format!("{:?}", employment_type).into()
            } else {
                String::new().into()
            },
            if let Some(work_location) = &self.work_location {
                format!("{:?}", work_location).into()
            } else {
                String::new().into()
            },
            if let Some(attachments) = &self.attachments {
                format!("{:?}", attachments).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "email".into(),
            "job_title".into(),
            "phone_number".into(),
            "candidate_id".into(),
            "start_date".into(),
            "salary_unit".into(),
            "salary_per_unit".into(),
            "signing_bonus".into(),
            "currency".into(),
            "equity_shares".into(),
            "department".into(),
            "employment_type".into(),
            "work_location".into(),
            "attachments".into(),
        ]
    }
}

#[doc = "Geographic details from where the event was recorded."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RequestData {
    #[doc = "Event IP addresss."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "City the event was triggered from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "Country the event was triggered from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "Latitude the event was triggered from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<String>,
    #[doc = "Longitude the event was triggered from."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<String>,
}

impl std::fmt::Display for RequestData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RequestData {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(ip) = &self.ip {
                format!("{:?}", ip).into()
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
            if let Some(latitude) = &self.latitude {
                format!("{:?}", latitude).into()
            } else {
                String::new().into()
            },
            if let Some(longitude) = &self.longitude {
                format!("{:?}", longitude).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "ip".into(),
            "city".into(),
            "country".into(),
            "latitude".into(),
            "longitude".into(),
        ]
    }
}

#[doc = "An ENUM value for the type of object."]
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
pub enum SubjectsType {
    #[serde(rename = "ROLE")]
    #[display("ROLE")]
    Role,
    #[serde(rename = "SPOKE")]
    #[display("SPOKE")]
    Spoke,
    #[serde(rename = "RPASS_ITEM")]
    #[display("RPASS_ITEM")]
    RpassItem,
    #[serde(rename = "SPOKE_USER")]
    #[display("SPOKE_USER")]
    SpokeUser,
    #[serde(rename = "GROUP")]
    #[display("GROUP")]
    Group,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Subjects {
    #[doc = "Unique key for the event object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[doc = "An ENUM value for the type of object."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<SubjectsType>,
    #[doc = "Name used within Rippling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "Icon used within Rippling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
}

impl std::fmt::Display for Subjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Subjects {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(instance) = &self.instance {
                format!("{:?}", instance).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "instance".into(),
            "type_".into(),
            "display_name".into(),
            "icon".into(),
        ]
    }
}

#[doc = "An ENUM value for the type of the event."]
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
pub enum EventType {
    #[serde(rename = "EXTERNAL_ACCOUNT_CREATE")]
    #[display("EXTERNAL_ACCOUNT_CREATE")]
    ExternalAccountCreate,
    #[serde(rename = "EXTERNAL_ACCOUNT_INVITE")]
    #[display("EXTERNAL_ACCOUNT_INVITE")]
    ExternalAccountInvite,
    #[serde(rename = "EXTERNAL_ACCOUNT_DELETE")]
    #[display("EXTERNAL_ACCOUNT_DELETE")]
    ExternalAccountDelete,
    #[serde(rename = "EXTERNAL_ACCOUNT_SUSPEND")]
    #[display("EXTERNAL_ACCOUNT_SUSPEND")]
    ExternalAccountSuspend,
    #[serde(rename = "EXTERNAL_ACCOUNT_PASSWORD_RESET")]
    #[display("EXTERNAL_ACCOUNT_PASSWORD_RESET")]
    ExternalAccountPasswordReset,
    #[serde(rename = "EXTERNAL_GROUP_ADD")]
    #[display("EXTERNAL_GROUP_ADD")]
    ExternalGroupAdd,
    #[serde(rename = "EXTERNAL_GROUP_REMOVE")]
    #[display("EXTERNAL_GROUP_REMOVE")]
    ExternalGroupRemove,
    #[serde(rename = "EXTERNAL_SSO_GRANT")]
    #[display("EXTERNAL_SSO_GRANT")]
    ExternalSsoGrant,
    #[serde(rename = "EXTERNAL_SSO_REVOKE")]
    #[display("EXTERNAL_SSO_REVOKE")]
    ExternalSsoRevoke,
    #[serde(rename = "EXTERNAL_SSO_SIGNIN")]
    #[display("EXTERNAL_SSO_SIGNIN")]
    ExternalSsoSignin,
    #[serde(rename = "RPASS_ITEM_SHARED")]
    #[display("RPASS_ITEM_SHARED")]
    RpassItemShared,
    #[serde(rename = "RPASS_ITEM_UNSHARED")]
    #[display("RPASS_ITEM_UNSHARED")]
    RpassItemUnshared,
    #[serde(rename = "RPASS_ITEM_USED")]
    #[display("RPASS_ITEM_USED")]
    RpassItemUsed,
    #[serde(rename = "USER_LOGIN_SUCCESS")]
    #[display("USER_LOGIN_SUCCESS")]
    UserLoginSuccess,
    #[serde(rename = "USER_LOGIN_FAILED")]
    #[display("USER_LOGIN_FAILED")]
    UserLoginFailed,
    #[serde(rename = "ACCOUNT_PASSWORD_RESET")]
    #[display("ACCOUNT_PASSWORD_RESET")]
    AccountPasswordReset,
    #[serde(rename = "ACCOUNT_PASSWORD_CHANGED")]
    #[display("ACCOUNT_PASSWORD_CHANGED")]
    AccountPasswordChanged,
    #[serde(rename = "TWO_FACTOR_DEVICE_RESET")]
    #[display("TWO_FACTOR_DEVICE_RESET")]
    TwoFactorDeviceReset,
    #[serde(rename = "EXTERNAL_GROUP_MEMBER_REMOVE")]
    #[display("EXTERNAL_GROUP_MEMBER_REMOVE")]
    ExternalGroupMemberRemove,
}

#[doc = "ENUM value for the type of actor."]
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
pub enum InitiatorType {
    #[serde(rename = "ROLE")]
    #[display("ROLE")]
    Role,
    #[serde(rename = "SYSTEM")]
    #[display("SYSTEM")]
    System,
    #[serde(rename = "EXTERNAL")]
    #[display("EXTERNAL")]
    External,
}

#[doc = "The actor of the event."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Initiator {
    #[doc = "ENUM value for the type of actor."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<InitiatorType>,
    #[doc = "A unique identifier for the employee that initiated the action, if the type is ROLE."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "The name used within Rippling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The icon used within Rippling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
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
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "type_".into(),
            "role".into(),
            "display_name".into(),
            "icon".into(),
        ]
    }
}

#[doc = "Reason for the event, tied to the type of eveent."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EventReason {
    #[doc = "Reason for the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Message of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl std::fmt::Display for EventReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EventReason {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(reason) = &self.reason {
                format!("{:?}", reason).into()
            } else {
                String::new().into()
            },
            if let Some(message) = &self.message {
                format!("{:?}", message).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["reason".into(), "message".into()]
    }
}

#[doc = "The event model for company activity.\n\nPlease note, the event type can be one of the \
         following:\n\n- EXTERNAL_ACCONT_CREATE\n- EXTERNAL_ACCOUNT_INVITE\n- \
         EXTERNAL_ACCOUNT_DELETE\n- EXTERNAL_ACCOUNT_SUSPEND\n- EXTERNAL_ACCOUNT_PASSWORD_RESET\n- \
         EXTERNAL_GROUP_ADD\n- EXTERNAL_GROUP_REMOVE\n- EXTERNAL_GROUP_MEMBER_REMOVE\n- \
         EXTERNAL_GROUP_MEMBER_ADD\n- EXTERNAL_SSO_GRANT\n- EXTERNAL_SSO_REVOKE\n- \
         EXTERNAL_SSO_SIGNIN\n- RPASS_ITEM_SHARED\n- RPASS_ITEM_UNSHARED\n- RPASS_ITEM_USED\n- \
         USER_LOGIN_SUCCESS\n- USER_LOGIN_FAILED\n- ACCOUNT_PASSWORD_RESET\n- \
         ACCOUNT_PASSWORD_CHANGED\n- TWO_FACTOR_DEVICE_RESET\n"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Event {
    #[doc = "Unique identifier of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Geographic details from where the event was recorded."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_data: Option<RequestData>,
    #[doc = "An array of event identifiers that are linked to the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_events: Option<Vec<String>>,
    #[doc = "The list of objects of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subjects: Option<Vec<Option<Subjects>>>,
    #[doc = "An ENUM value for the type of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,
    #[doc = "Timestamp at which the event was recorded."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Unique identifier for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "Unique identifier for the external application for which the event was recorded. \
             This will be Null for events that don't correspond to an external appliction (e.g. \
             Rippling system and RPass events)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spoke: Option<String>,
    #[doc = "The actor of the event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[doc = "Reason for the event, tied to the type of eveent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_reason: Option<EventReason>,
    #[doc = "Display name for the event, tied to the type of event."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Event {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(request_data) = &self.request_data {
                format!("{:?}", request_data).into()
            } else {
                String::new().into()
            },
            if let Some(linked_events) = &self.linked_events {
                format!("{:?}", linked_events).into()
            } else {
                String::new().into()
            },
            if let Some(subjects) = &self.subjects {
                format!("{:?}", subjects).into()
            } else {
                String::new().into()
            },
            if let Some(event_type) = &self.event_type {
                format!("{:?}", event_type).into()
            } else {
                String::new().into()
            },
            if let Some(timestamp) = &self.timestamp {
                format!("{:?}", timestamp).into()
            } else {
                String::new().into()
            },
            if let Some(company) = &self.company {
                format!("{:?}", company).into()
            } else {
                String::new().into()
            },
            if let Some(spoke) = &self.spoke {
                format!("{:?}", spoke).into()
            } else {
                String::new().into()
            },
            if let Some(initiator) = &self.initiator {
                format!("{:?}", initiator).into()
            } else {
                String::new().into()
            },
            if let Some(event_reason) = &self.event_reason {
                format!("{:?}", event_reason).into()
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
        vec![
            "id".into(),
            "request_data".into(),
            "linked_events".into(),
            "subjects".into(),
            "event_type".into(),
            "timestamp".into(),
            "company".into(),
            "spoke".into(),
            "initiator".into(),
            "event_reason".into(),
            "name".into(),
        ]
    }
}

#[doc = "CustomField."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomField {
    #[serde(
        rename = "customFieldTitle1",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_field_title_1: Option<String>,
    #[serde(
        rename = "customFieldTitleN",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_field_title_n: Option<String>,
}

impl std::fmt::Display for CustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomField {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(custom_field_title_1) = &self.custom_field_title_1 {
                format!("{:?}", custom_field_title_1).into()
            } else {
                String::new().into()
            },
            if let Some(custom_field_title_n) = &self.custom_field_title_n {
                format!("{:?}", custom_field_title_n).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["custom_field_title_1".into(), "custom_field_title_n".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EntityInfo {
    #[doc = "The legal name of the entity"]
    #[serde(rename = "legalName", default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[doc = "The Canada Business Number"]
    #[serde(
        rename = "businessNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub business_number: Option<String>,
}

impl std::fmt::Display for EntityInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EntityInfo {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
            if let Some(business_number) = &self.business_number {
                format!("{:?}", business_number).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["legal_name".into(), "business_number".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Ca {
    #[doc = "The unique Rippling ID of the legal entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(
        rename = "entityInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub entity_info: Option<EntityInfo>,
    #[doc = "If set to true, the legal entity is Rippling's EOR. If set to false, the legal \
             entity is not on Rippling's EOR."]
    #[serde(rename = "isEor", default, skip_serializing_if = "Option::is_none")]
    pub is_eor: Option<bool>,
}

impl std::fmt::Display for Ca {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Ca {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(id) = &self.id {
                format!("{:?}", id).into()
            } else {
                String::new().into()
            },
            if let Some(entity_info) = &self.entity_info {
                format!("{:?}", entity_info).into()
            } else {
                String::new().into()
            },
            if let Some(is_eor) = &self.is_eor {
                format!("{:?}", is_eor).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "entity_info".into(), "is_eor".into()]
    }
}

#[doc = "This model represents the legal entities inside of a given company. Legal entities based \
         in Canada (CA) are currently supported at this time."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Entities {
    #[doc = "CA represents Canada."]
    #[serde(rename = "CA", default, skip_serializing_if = "Option::is_none")]
    pub ca: Option<Vec<Ca>>,
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
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(ca) = &self.ca {
            format!("{:?}", ca).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["ca".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostGroupsRequestBody {
    #[doc = "User-readable name of the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The unique ID for the group, this can be the unique identifier for the group entity \
             object within your application."]
    #[serde(rename = "spokeId", default, skip_serializing_if = "Option::is_none")]
    pub spoke_id: Option<String>,
    #[doc = "An array of Rippling IDs that will be in the group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

impl std::fmt::Display for PostGroupsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostGroupsRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(spoke_id) = &self.spoke_id {
                format!("{:?}", spoke_id).into()
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
        vec!["name".into(), "spoke_id".into(), "users".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostLeaveRequestsRequestBody {
    #[doc = "Unique identifier of the employee who is taking leave."]
    pub role: String,
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<String>,
    #[doc = "The status to create the leave request in. Only TILT managed requests can take a \
             status other than PENDING."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    #[serde(
        rename = "startDateStartTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_start_time: Option<String>,
    #[serde(
        rename = "endDateEndTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date_end_time: Option<String>,
    #[serde(
        rename = "startDateCustomHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_custom_hours: Option<String>,
    #[serde(
        rename = "endDateCustomHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date_custom_hours: Option<String>,
    #[doc = "Unique identifier of the company leave type"]
    #[serde(rename = "companyLeaveType")]
    pub company_leave_type: String,
    #[doc = "Unique identifier of the leave policy. Required if request is not managed by TILT"]
    #[serde(rename = "leavePolicy")]
    pub leave_policy: String,
    #[serde(
        rename = "reasonForLeave",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reason_for_leave: Option<String>,
    #[doc = "String identifier for third party that manages this leave request. This may be null."]
    #[serde(rename = "managedBy", default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[doc = "Object id for corresponding leave obejct in third party system. This may be null."]
    #[serde(
        rename = "externalId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub external_id: Option<String>,
}

impl std::fmt::Display for PostLeaveRequestsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostLeaveRequestsRequestBody {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.role.clone().into(),
            if let Some(requested_by) = &self.requested_by {
                format!("{:?}", requested_by).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            self.start_date.clone().into(),
            self.end_date.clone().into(),
            if let Some(start_date_start_time) = &self.start_date_start_time {
                format!("{:?}", start_date_start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_date_end_time) = &self.end_date_end_time {
                format!("{:?}", end_date_end_time).into()
            } else {
                String::new().into()
            },
            if let Some(start_date_custom_hours) = &self.start_date_custom_hours {
                format!("{:?}", start_date_custom_hours).into()
            } else {
                String::new().into()
            },
            if let Some(end_date_custom_hours) = &self.end_date_custom_hours {
                format!("{:?}", end_date_custom_hours).into()
            } else {
                String::new().into()
            },
            self.company_leave_type.clone().into(),
            self.leave_policy.clone().into(),
            if let Some(reason_for_leave) = &self.reason_for_leave {
                format!("{:?}", reason_for_leave).into()
            } else {
                String::new().into()
            },
            if let Some(managed_by) = &self.managed_by {
                format!("{:?}", managed_by).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "role".into(),
            "requested_by".into(),
            "status".into(),
            "start_date".into(),
            "end_date".into(),
            "start_date_start_time".into(),
            "end_date_end_time".into(),
            "start_date_custom_hours".into(),
            "end_date_custom_hours".into(),
            "company_leave_type".into(),
            "leave_policy".into(),
            "reason_for_leave".into(),
            "managed_by".into(),
            "external_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetLeaveBalancesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "Leave balances object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balances: Option<LeaveBalances>,
}

impl std::fmt::Display for GetLeaveBalancesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetLeaveBalancesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(balances) = &self.balances {
                format!("{:?}", balances).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["role".into(), "balances".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetLeaveBalanceResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "Leave balances object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balances: Option<LeaveBalances>,
}

impl std::fmt::Display for GetLeaveBalanceResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetLeaveBalanceResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(balances) = &self.balances {
                format!("{:?}", balances).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["role".into(), "balances".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Data {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
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
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(events) = &self.events {
                format!("{:?}", events).into()
            } else {
                String::new().into()
            },
            if let Some(next) = &self.next {
                format!("{:?}", next).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["events".into(), "next".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCompanyActivityResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl std::fmt::Display for GetCompanyActivityResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCompanyActivityResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(data) = &self.data {
                format!("{:?}", data).into()
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
        vec!["data".into(), "error".into()]
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
pub enum Action {
    #[serde(rename = "approve")]
    #[display("approve")]
    Approve,
    #[serde(rename = "decline")]
    #[display("decline")]
    Decline,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PatchLeaveRequestsLeaveRequestIdRequestBody {
    #[serde(
        rename = "requestedBy",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_by: Option<String>,
    #[doc = "Change the status of a request. This is only possible for TILT managed requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "startDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(
        rename = "startDateStartTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_start_time: Option<String>,
    #[serde(
        rename = "endDateEndTime",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date_end_time: Option<String>,
    #[serde(
        rename = "startDateCustomHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub start_date_custom_hours: Option<String>,
    #[serde(
        rename = "endDateCustomHours",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub end_date_custom_hours: Option<String>,
    #[doc = "Updated reason for leave request. This may be updated even for an APPROVED request."]
    #[serde(
        rename = "reasonForLeave",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub reason_for_leave: Option<String>,
}

impl std::fmt::Display for PatchLeaveRequestsLeaveRequestIdRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PatchLeaveRequestsLeaveRequestIdRequestBody {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(requested_by) = &self.requested_by {
                format!("{:?}", requested_by).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
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
            if let Some(start_date_start_time) = &self.start_date_start_time {
                format!("{:?}", start_date_start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_date_end_time) = &self.end_date_end_time {
                format!("{:?}", end_date_end_time).into()
            } else {
                String::new().into()
            },
            if let Some(start_date_custom_hours) = &self.start_date_custom_hours {
                format!("{:?}", start_date_custom_hours).into()
            } else {
                String::new().into()
            },
            if let Some(end_date_custom_hours) = &self.end_date_custom_hours {
                format!("{:?}", end_date_custom_hours).into()
            } else {
                String::new().into()
            },
            if let Some(reason_for_leave) = &self.reason_for_leave {
                format!("{:?}", reason_for_leave).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "requested_by".into(),
            "status".into(),
            "start_date".into(),
            "end_date".into(),
            "start_date_start_time".into(),
            "end_date_end_time".into(),
            "start_date_custom_hours".into(),
            "end_date_custom_hours".into(),
            "reason_for_leave".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AppHandleId {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_user_name: Option<String>,
}

impl std::fmt::Display for AppHandleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AppHandleId {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(app_user_name) = &self.app_user_name {
            format!("{:?}", app_user_name).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["app_user_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Results {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rippling_employee_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_handle_id: Option<AppHandleId>,
}

impl std::fmt::Display for Results {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Results {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(rippling_employee_id) = &self.rippling_employee_id {
                format!("{:?}", rippling_employee_id).into()
            } else {
                String::new().into()
            },
            if let Some(app_handle_id) = &self.app_handle_id {
                format!("{:?}", app_handle_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["rippling_employee_id".into(), "app_handle_id".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetAppAppMatchingUsersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Results>>,
}

impl std::fmt::Display for GetAppAppMatchingUsersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetAppAppMatchingUsersResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(results) = &self.results {
            format!("{:?}", results).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into()]
    }
}

#[doc = "PostMarkAppInstalledResponse."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PostMarkAppInstalledResponse {
    pub ok: bool,
}

impl std::fmt::Display for PostMarkAppInstalledResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PostMarkAppInstalledResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.ok).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["ok".into()]
    }
}
