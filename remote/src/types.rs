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

#[doc = "The status of the task"]
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
    #[serde(rename = "completed")]
    #[display("completed")]
    Completed,
    #[serde(rename = "pending")]
    #[display("pending")]
    Pending,
}

#[doc = "Description and status of an onboarding task."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TaskDescription {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The status of the task"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl std::fmt::Display for TaskDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TaskDescription {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
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
        vec!["description".into(), "status".into()]
    }
}

#[doc = "A supported file"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct File {
    pub id: String,
    #[serde(deserialize_with = "crate::utils::date_time_format::deserialize")]
    pub inserted_at: chrono::DateTime<chrono::Utc>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for File {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            format!("{:?}", self.inserted_at).into(),
            self.name.clone().into(),
            if let Some(sub_type) = &self.sub_type {
                format!("{:?}", sub_type).into()
            } else {
                String::new().into()
            },
            self.type_.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "inserted_at".into(),
            "name".into(),
            "sub_type".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ConflictResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl std::fmt::Display for ConflictResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ConflictResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(message) = &self.message {
            format!("{:?}", message).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
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
pub enum TimeoffType {
    #[serde(rename = "paid_time_off")]
    #[display("paid_time_off")]
    PaidTimeOff,
    #[serde(rename = "sick_leave")]
    #[display("sick_leave")]
    SickLeave,
    #[serde(rename = "public_holiday")]
    #[display("public_holiday")]
    PublicHoliday,
    #[serde(rename = "unpaid_leave")]
    #[display("unpaid_leave")]
    UnpaidLeave,
    #[serde(rename = "extended_leave")]
    #[display("extended_leave")]
    ExtendedLeave,
    #[serde(rename = "in_lieu_time")]
    #[display("in_lieu_time")]
    InLieuTime,
    #[serde(rename = "maternity_leave")]
    #[display("maternity_leave")]
    MaternityLeave,
    #[serde(rename = "paternity_leave")]
    #[display("paternity_leave")]
    PaternityLeave,
    #[serde(rename = "parental_leave")]
    #[display("parental_leave")]
    ParentalLeave,
    #[serde(rename = "bereavement")]
    #[display("bereavement")]
    Bereavement,
    #[serde(rename = "military_leave")]
    #[display("military_leave")]
    MilitaryLeave,
    #[serde(rename = "other")]
    #[display("other")]
    Other,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Creator {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
pub struct LastEditor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl std::fmt::Display for LastEditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LastEditor {
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

#[doc = "Payroll run product type"]
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
pub enum ProductType {
    #[serde(rename = "eor")]
    #[display("eor")]
    Eor,
    #[serde(rename = "global_payroll")]
    #[display("global_payroll")]
    GlobalPayroll,
}

#[doc = "Status of the payroll"]
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
pub enum PayrollRunStatus {
    #[serde(rename = "preparing")]
    #[display("preparing")]
    Preparing,
    #[serde(rename = "processing")]
    #[display("processing")]
    Processing,
    #[serde(rename = "waiting_for_customer_approval")]
    #[display("waiting_for_customer_approval")]
    WaitingForCustomerApproval,
    #[serde(rename = "completed")]
    #[display("completed")]
    Completed,
    #[serde(rename = "finalized")]
    #[display("finalized")]
    Finalized,
    #[serde(rename = "rejected")]
    #[display("rejected")]
    Rejected,
}

#[doc = "Payroll Run type"]
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
    #[serde(rename = "main")]
    #[display("main")]
    Main,
    #[serde(rename = "one_off")]
    #[display("one_off")]
    OneOff,
    #[serde(rename = "pro_forma")]
    #[display("pro_forma")]
    ProForma,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayrollRun {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,
    #[doc = "Indicates if an Employer has completed the Payroll Inputs review flow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer_inputs_reviewed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_for_employment_matching: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inserted_at: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_editor: Option<LastEditor>,
    pub legal_entity: RemoteEntity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping_rules: Option<Vec<String>>,
    #[doc = "Name of the payroll_run to be displayed for users"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub net_pay_extraction_expression: Option<String>,
    #[doc = "The end date the payroll run is for"]
    pub period_end: chrono::NaiveDate,
    #[doc = "The start date the payroll run is for"]
    pub period_start: chrono::NaiveDate,
    #[doc = "Payroll run product type"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_type: Option<ProductType>,
    pub slug: String,
    #[doc = "Status of the payroll"]
    pub status: PayrollRunStatus,
    pub summarize_automatically: bool,
    #[doc = "Payroll Run type"]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validations: Option<serde_json::Value>,
}

impl std::fmt::Display for PayrollRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PayrollRun {
    const LENGTH: usize = 17;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(creator) = &self.creator {
                format!("{:?}", creator).into()
            } else {
                String::new().into()
            },
            if let Some(customer_inputs_reviewed) = &self.customer_inputs_reviewed {
                format!("{:?}", customer_inputs_reviewed).into()
            } else {
                String::new().into()
            },
            if let Some(field_for_employment_matching) = &self.field_for_employment_matching {
                format!("{:?}", field_for_employment_matching).into()
            } else {
                String::new().into()
            },
            if let Some(inserted_at) = &self.inserted_at {
                format!("{:?}", inserted_at).into()
            } else {
                String::new().into()
            },
            if let Some(last_editor) = &self.last_editor {
                format!("{:?}", last_editor).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.legal_entity).into(),
            if let Some(mapping_rules) = &self.mapping_rules {
                format!("{:?}", mapping_rules).into()
            } else {
                String::new().into()
            },
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(net_pay_extraction_expression) = &self.net_pay_extraction_expression {
                format!("{:?}", net_pay_extraction_expression).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.period_end).into(),
            format!("{:?}", self.period_start).into(),
            if let Some(product_type) = &self.product_type {
                format!("{:?}", product_type).into()
            } else {
                String::new().into()
            },
            self.slug.clone().into(),
            format!("{:?}", self.status).into(),
            format!("{:?}", self.summarize_automatically).into(),
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(validations) = &self.validations {
                format!("{:?}", validations).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "creator".into(),
            "customer_inputs_reviewed".into(),
            "field_for_employment_matching".into(),
            "inserted_at".into(),
            "last_editor".into(),
            "legal_entity".into(),
            "mapping_rules".into(),
            "name".into(),
            "net_pay_extraction_expression".into(),
            "period_end".into(),
            "period_start".into(),
            "product_type".into(),
            "slug".into(),
            "status".into(),
            "summarize_automatically".into(),
            "type_".into(),
            "validations".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Address {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<AddressCountry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_details: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
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
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
            if let Some(address_line_2) = &self.address_line_2 {
                format!("{:?}", address_line_2).into()
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
            if let Some(local_details) = &self.local_details {
                format!("{:?}", local_details).into()
            } else {
                String::new().into()
            },
            if let Some(postal_code) = &self.postal_code {
                format!("{:?}", postal_code).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
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
            "address".into(),
            "address_line_2".into(),
            "city".into(),
            "country".into(),
            "local_details".into(),
            "postal_code".into(),
            "slug".into(),
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
pub enum EmploymentType {
    #[serde(rename = "employee")]
    #[display("employee")]
    Employee,
    #[serde(rename = "contractor")]
    #[display("contractor")]
    Contractor,
}

#[doc = "Complete information of an employment"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Employment {
    #[doc = "Home address information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `address_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_details: Option<serde_json::Value>,
    #[doc = "Administrative information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `administrative_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_details: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_details: Option<Vec<serde_json::Value>>,
    #[doc = "Billing address information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `billing_address_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address_details: Option<serde_json::Value>,
    pub company_id: String,
    #[doc = "Contract information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `contract_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_details: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    pub created_at: String,
    #[doc = "Emergency contact information. Its properties may vary depending on the country."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact_details: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<File>>,
    pub full_name: String,
    pub id: String,
    pub job_title: String,
    #[doc = "All tasks that need to be completed before marking the employment as ready"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub onboarding_tasks: Option<OnboardingTasks>,
    #[doc = "Personal details information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `personal_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<serde_json::Value>,
    pub personal_email: String,
    #[doc = "Selected type of payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_plan_details: Option<PricingPlanDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisional_start_date: Option<chrono::NaiveDate>,
    #[doc = "The status of employment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EmploymentStatus>,
    #[serde(rename = "type")]
    pub type_: EmploymentType,
    #[serde(deserialize_with = "crate::utils::date_time_format::deserialize")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for Employment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Employment {
    const LENGTH: usize = 21;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.address_details).into(),
            format!("{:?}", self.administrative_details).into(),
            format!("{:?}", self.bank_account_details).into(),
            format!("{:?}", self.billing_address_details).into(),
            self.company_id.clone().into(),
            format!("{:?}", self.contract_details).into(),
            if let Some(country_code) = &self.country_code {
                format!("{:?}", country_code).into()
            } else {
                String::new().into()
            },
            self.created_at.clone().into(),
            format!("{:?}", self.emergency_contact_details).into(),
            if let Some(files) = &self.files {
                format!("{:?}", files).into()
            } else {
                String::new().into()
            },
            self.full_name.clone().into(),
            self.id.clone().into(),
            self.job_title.clone().into(),
            if let Some(onboarding_tasks) = &self.onboarding_tasks {
                format!("{:?}", onboarding_tasks).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.personal_details).into(),
            self.personal_email.clone().into(),
            format!("{:?}", self.pricing_plan_details).into(),
            if let Some(provisional_start_date) = &self.provisional_start_date {
                format!("{:?}", provisional_start_date).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.type_).into(),
            format!("{:?}", self.updated_at).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address_details".into(),
            "administrative_details".into(),
            "bank_account_details".into(),
            "billing_address_details".into(),
            "company_id".into(),
            "contract_details".into(),
            "country_code".into(),
            "created_at".into(),
            "emergency_contact_details".into(),
            "files".into(),
            "full_name".into(),
            "id".into(),
            "job_title".into(),
            "onboarding_tasks".into(),
            "personal_details".into(),
            "personal_email".into(),
            "pricing_plan_details".into(),
            "provisional_start_date".into(),
            "status".into(),
            "type_".into(),
            "updated_at".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Data {
    #[doc = "The current page among all of the total_pages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employments: Option<Vec<MinimalEmployment>>,
    #[doc = "The total number of records in the result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The total number of pages the user can go through"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
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
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(current_page) = &self.current_page {
                format!("{:?}", current_page).into()
            } else {
                String::new().into()
            },
            if let Some(employments) = &self.employments {
                format!("{:?}", employments).into()
            } else {
                String::new().into()
            },
            if let Some(total_count) = &self.total_count {
                format!("{:?}", total_count).into()
            } else {
                String::new().into()
            },
            if let Some(total_pages) = &self.total_pages {
                format!("{:?}", total_pages).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "current_page".into(),
            "employments".into(),
            "total_count".into(),
            "total_pages".into(),
        ]
    }
}

#[doc = "Response schema listing many employments"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListEmploymentsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl std::fmt::Display for ListEmploymentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListEmploymentsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeoffResponseData {
    pub timeoff: Timeoff,
}

impl std::fmt::Display for TimeoffResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeoffResponseData {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.timeoff).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["timeoff".into()]
    }
}

#[doc = "Timeoff response"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeoffResponse {
    pub data: TimeoffResponseData,
}

impl std::fmt::Display for TimeoffResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeoffResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.data).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTimeoffTypesResponseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<TimeoffType>,
}

impl std::fmt::Display for ListTimeoffTypesResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTimeoffTypesResponseData {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
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
        vec!["description".into(), "name".into()]
    }
}

#[doc = "Time off types response"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTimeoffTypesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTimeoffTypesResponseData>,
}

impl std::fmt::Display for ListTimeoffTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTimeoffTypesResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[doc = "Approved timeoff creation params"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateApprovedTimeoffParams {}

impl std::fmt::Display for CreateApprovedTimeoffParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateApprovedTimeoffParams {
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
pub enum EmploymentBasicParamsType {
    #[serde(rename = "employee")]
    #[display("employee")]
    Employee,
    #[serde(rename = "contractor")]
    #[display("contractor")]
    Contractor,
}

#[doc = "Description of the required params to create an employment."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmploymentBasicParams {
    pub company_id: String,
    pub country_code: String,
    pub full_name: String,
    pub job_title: String,
    pub personal_email: String,
    #[doc = "Required for employees, optional for contractors"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisional_start_date: Option<chrono::NaiveDate>,
    #[serde(rename = "type")]
    pub type_: EmploymentBasicParamsType,
}

impl std::fmt::Display for EmploymentBasicParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmploymentBasicParams {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.company_id.clone().into(),
            self.country_code.clone().into(),
            self.full_name.clone().into(),
            self.job_title.clone().into(),
            self.personal_email.clone().into(),
            if let Some(provisional_start_date) = &self.provisional_start_date {
                format!("{:?}", provisional_start_date).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.type_).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company_id".into(),
            "country_code".into(),
            "full_name".into(),
            "job_title".into(),
            "personal_email".into(),
            "provisional_start_date".into(),
            "type_".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayrollRunWithLegalEntity {}

impl std::fmt::Display for PayrollRunWithLegalEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PayrollRunWithLegalEntity {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[doc = "Complete information of an employment"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmploymentResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<EmploymentData>,
}

impl std::fmt::Display for EmploymentResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmploymentResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmploymentData {
    #[doc = "Complete information of an employment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment: Option<Employment>,
}

impl std::fmt::Display for EmploymentData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmploymentData {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(employment) = &self.employment {
            format!("{:?}", employment).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["employment".into()]
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
pub enum UpdateApprovedTimeoffParamsStatus {
    #[serde(rename = "approved")]
    #[display("approved")]
    Approved,
    #[serde(rename = "cancelled")]
    #[display("cancelled")]
    Cancelled,
}

#[doc = "Update timeoff params"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateApprovedTimeoffParams {
    #[doc = "UTC date time in YYYY-MM-DDTHH:mm:ss format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[doc = "The reason for cancelling a time off. Required when updating to status `cancelled`."]
    pub cancel_reason: String,
    #[doc = "Timeoff document params"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<TimeoffDocumentParams>,
    #[doc = "The reason for the update. Required when updating the time off data but not changing \
             the status."]
    pub edit_reason: String,
    #[doc = "UTC date in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "UTC date in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateApprovedTimeoffParamsStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoff_days: Option<Vec<TimeoffDaysParams>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoff_type: Option<TimeoffType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl std::fmt::Display for UpdateApprovedTimeoffParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateApprovedTimeoffParams {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(approved_at) = &self.approved_at {
                format!("{:?}", approved_at).into()
            } else {
                String::new().into()
            },
            if let Some(approver_id) = &self.approver_id {
                format!("{:?}", approver_id).into()
            } else {
                String::new().into()
            },
            self.cancel_reason.clone().into(),
            if let Some(document) = &self.document {
                format!("{:?}", document).into()
            } else {
                String::new().into()
            },
            self.edit_reason.clone().into(),
            if let Some(end_date) = &self.end_date {
                format!("{:?}", end_date).into()
            } else {
                String::new().into()
            },
            if let Some(notes) = &self.notes {
                format!("{:?}", notes).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(timeoff_days) = &self.timeoff_days {
                format!("{:?}", timeoff_days).into()
            } else {
                String::new().into()
            },
            if let Some(timeoff_type) = &self.timeoff_type {
                format!("{:?}", timeoff_type).into()
            } else {
                String::new().into()
            },
            if let Some(timezone) = &self.timezone {
                format!("{:?}", timezone).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "approved_at".into(),
            "approver_id".into(),
            "cancel_reason".into(),
            "document".into(),
            "edit_reason".into(),
            "end_date".into(),
            "notes".into(),
            "start_date".into(),
            "status".into(),
            "timeoff_days".into(),
            "timeoff_type".into(),
            "timezone".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct AddressCountry {
    pub code: String,
    pub features: Vec<String>,
    pub name: String,
    pub slug: String,
}

impl std::fmt::Display for AddressCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for AddressCountry {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.code.clone().into(),
            format!("{:?}", self.features).into(),
            self.name.clone().into(),
            self.slug.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "code".into(),
            "features".into(),
            "name".into(),
            "slug".into(),
        ]
    }
}

#[doc = "The status of employment"]
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
pub enum EmploymentStatus {
    #[serde(rename = "active")]
    #[display("active")]
    Active,
    #[serde(rename = "created")]
    #[display("created")]
    Created,
    #[serde(rename = "initiated")]
    #[display("initiated")]
    Initiated,
    #[serde(rename = "invited")]
    #[display("invited")]
    Invited,
    #[serde(rename = "pending")]
    #[display("pending")]
    Pending,
    #[serde(rename = "review")]
    #[display("review")]
    Review,
    #[serde(rename = "archived")]
    #[display("archived")]
    Archived,
    #[serde(rename = "deleted")]
    #[display("deleted")]
    Deleted,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnprocessableEntityErrorResponse {
    pub errors: std::collections::HashMap<String, Vec<String>>,
}

impl std::fmt::Display for UnprocessableEntityErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnprocessableEntityErrorResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.errors).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["errors".into()]
    }
}

#[doc = "TimeoffDay schema"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeoffDay {
    #[doc = "UTC date in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format"]
    pub day: chrono::NaiveDate,
    pub hours: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payroll_run: Option<PayrollRunWithLegalEntity>,
}

impl std::fmt::Display for TimeoffDay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeoffDay {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.day).into(),
            format!("{:?}", self.hours).into(),
            if let Some(payroll_run) = &self.payroll_run {
                format!("{:?}", payroll_run).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["day".into(), "hours".into(), "payroll_run".into()]
    }
}

#[doc = "The status of the task"]
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
pub enum TaskStatus {
    #[serde(rename = "completed")]
    #[display("completed")]
    Completed,
    #[serde(rename = "pending")]
    #[display("pending")]
    Pending,
}

#[doc = "Holidays response"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct HolidaysResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Holiday>>,
}

impl std::fmt::Display for HolidaysResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for HolidaysResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[doc = "Selected type of payment."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PricingPlanDetails {
    pub frequency: String,
}

impl std::fmt::Display for PricingPlanDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PricingPlanDetails {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.frequency.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["frequency".into()]
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
pub enum TimeoffStatus {
    #[serde(rename = "approved")]
    #[display("approved")]
    Approved,
    #[serde(rename = "cancelled")]
    #[display("cancelled")]
    Cancelled,
    #[serde(rename = "declined")]
    #[display("declined")]
    Declined,
    #[serde(rename = "requested")]
    #[display("requested")]
    Requested,
    #[serde(rename = "taken")]
    #[display("taken")]
    Taken,
    #[serde(rename = "cancel_requested")]
    #[display("cancel_requested")]
    CancelRequested,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Timeoff {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::utils::nullable_date_time_format::deserialize"
    )]
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_reason: Option<String>,
    #[doc = "Optional UTC date time in YYYY-MM-DDTHH:mm:ss format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_at: Option<serde_json::Value>,
    #[doc = "A supported file"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<File>,
    pub employment_id: String,
    pub end_date: chrono::NaiveDate,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub status: TimeoffStatus,
    pub timeoff_days: Vec<TimeoffDay>,
    pub timeoff_type: TimeoffType,
    pub timezone: String,
}

impl std::fmt::Display for Timeoff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Timeoff {
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(approved_at) = &self.approved_at {
                format!("{:?}", approved_at).into()
            } else {
                String::new().into()
            },
            if let Some(approver_id) = &self.approver_id {
                format!("{:?}", approver_id).into()
            } else {
                String::new().into()
            },
            if let Some(cancel_reason) = &self.cancel_reason {
                format!("{:?}", cancel_reason).into()
            } else {
                String::new().into()
            },
            if let Some(cancelled_at) = &self.cancelled_at {
                format!("{:?}", cancelled_at).into()
            } else {
                String::new().into()
            },
            if let Some(document) = &self.document {
                format!("{:?}", document).into()
            } else {
                String::new().into()
            },
            self.employment_id.clone().into(),
            format!("{:?}", self.end_date).into(),
            self.id.clone().into(),
            if let Some(notes) = &self.notes {
                format!("{:?}", notes).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.start_date).into(),
            format!("{:?}", self.status).into(),
            format!("{:?}", self.timeoff_days).into(),
            format!("{:?}", self.timeoff_type).into(),
            self.timezone.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "approved_at".into(),
            "approver_id".into(),
            "cancel_reason".into(),
            "cancelled_at".into(),
            "document".into(),
            "employment_id".into(),
            "end_date".into(),
            "id".into(),
            "notes".into(),
            "start_date".into(),
            "status".into(),
            "timeoff_days".into(),
            "timeoff_type".into(),
            "timezone".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyManager {
    #[doc = "Company ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[doc = "Company Manager role."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[doc = "User Email"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[doc = "User ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "User's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl std::fmt::Display for CompanyManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyManager {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            if let Some(role) = &self.role {
                format!("{:?}", role).into()
            } else {
                String::new().into()
            },
            if let Some(user_email) = &self.user_email {
                format!("{:?}", user_email).into()
            } else {
                String::new().into()
            },
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(user_name) = &self.user_name {
                format!("{:?}", user_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company_id".into(),
            "role".into(),
            "user_email".into(),
            "user_id".into(),
            "user_name".into(),
        ]
    }
}

#[doc = "Timeoff document params"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeoffDocumentParams {
    #[doc = "The binary content of the file encoded with base64"]
    pub content: String,
    #[doc = "The file name of the document"]
    pub name: String,
}

impl std::fmt::Display for TimeoffDocumentParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeoffDocumentParams {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.content.clone().into(), self.name.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["content".into(), "name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyManagerParams {
    #[doc = "The Company ID. Required if the access token can access multiple companies. Optional \
             otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[doc = "The work email of the company manager"]
    pub email: String,
    #[doc = "The name of the company manager"]
    pub name: String,
    #[doc = "The role assigned for the new manager. The value should be one of the \
             following:\n\n- `admin`: an Admin can manage most of the resources in remote.\n- \
             `onboarding_manager`: an Onboarding Manager can add, see and manage new hires.\n- \
             `people_manager`: an People Manager can view the employee profiles for the team \
             members they manage and approve and decline time off and expenses for their \
             employees.\n"]
    pub role: String,
}

impl std::fmt::Display for CompanyManagerParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyManagerParams {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
            } else {
                String::new().into()
            },
            self.email.clone().into(),
            self.name.clone().into(),
            self.role.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company_id".into(),
            "email".into(),
            "name".into(),
            "role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct NotFoundResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl std::fmt::Display for NotFoundResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for NotFoundResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(message) = &self.message {
            format!("{:?}", message).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
    }
}

#[doc = "A supported country on Remote"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Country {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country_subdivisions: Option<Vec<CountrySubdivision>>,
    pub name: String,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Country {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.code.clone().into(),
            if let Some(country_subdivisions) = &self.country_subdivisions {
                format!("{:?}", country_subdivisions).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["code".into(), "country_subdivisions".into(), "name".into()]
    }
}

#[doc = "Description of the basic required and onboarding tasks params to create an \
         employment.\nYou do not need to include all onboarding tasks when creating or updating an \
         employment.\n"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmploymentFullParams {
    #[doc = "Home address information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `address_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_details: Option<serde_json::Value>,
    #[doc = "Administrative information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `administrative_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_details: Option<serde_json::Value>,
    #[doc = "Bank account information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `bank_account_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_details: Option<serde_json::Value>,
    #[doc = "Billing address information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `billing_address_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address_details: Option<serde_json::Value>,
    pub company_id: String,
    #[doc = "Contract information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `contract_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_details: Option<serde_json::Value>,
    #[doc = "A supported country on Remote"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[doc = "Emergency contact information. Its properties may vary depending on the country."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact_details: Option<serde_json::Value>,
    pub full_name: String,
    pub job_title: String,
    #[doc = "The user id of the manager, who should have an `admin`, `owner` or `people_manager` \
             role.\nYou can find these users by querying the [Company Managers \
             endpoint](#operation/get_index_company_manager).\n**Update of this field is only \
             available for active employments.**\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<String>,
    #[doc = "Personal details information. As its properties may vary depending on the country,\nyou must query the [Show form schema](https://gateway.remote.com/eor/v1/docs/openapi.html#tag/Countries/operation/get_show_form_country) endpoint\npassing the country code and `personal_details` as path parameters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<serde_json::Value>,
    pub personal_email: String,
    #[doc = "Selected type of payment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_plan_details: Option<PricingPlanDetails>,
    #[doc = "Required for employees, optional for contractors"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisional_start_date: Option<chrono::NaiveDate>,
}

impl std::fmt::Display for EmploymentFullParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmploymentFullParams {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(address_details) = &self.address_details {
                format!("{:?}", address_details).into()
            } else {
                String::new().into()
            },
            if let Some(administrative_details) = &self.administrative_details {
                format!("{:?}", administrative_details).into()
            } else {
                String::new().into()
            },
            if let Some(bank_account_details) = &self.bank_account_details {
                format!("{:?}", bank_account_details).into()
            } else {
                String::new().into()
            },
            if let Some(billing_address_details) = &self.billing_address_details {
                format!("{:?}", billing_address_details).into()
            } else {
                String::new().into()
            },
            self.company_id.clone().into(),
            if let Some(contract_details) = &self.contract_details {
                format!("{:?}", contract_details).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(emergency_contact_details) = &self.emergency_contact_details {
                format!("{:?}", emergency_contact_details).into()
            } else {
                String::new().into()
            },
            self.full_name.clone().into(),
            self.job_title.clone().into(),
            if let Some(manager_id) = &self.manager_id {
                format!("{:?}", manager_id).into()
            } else {
                String::new().into()
            },
            if let Some(personal_details) = &self.personal_details {
                format!("{:?}", personal_details).into()
            } else {
                String::new().into()
            },
            self.personal_email.clone().into(),
            if let Some(pricing_plan_details) = &self.pricing_plan_details {
                format!("{:?}", pricing_plan_details).into()
            } else {
                String::new().into()
            },
            if let Some(provisional_start_date) = &self.provisional_start_date {
                format!("{:?}", provisional_start_date).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address_details".into(),
            "administrative_details".into(),
            "bank_account_details".into(),
            "billing_address_details".into(),
            "company_id".into(),
            "contract_details".into(),
            "country".into(),
            "emergency_contact_details".into(),
            "full_name".into(),
            "job_title".into(),
            "manager_id".into(),
            "personal_details".into(),
            "personal_email".into(),
            "pricing_plan_details".into(),
            "provisional_start_date".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MaybeMinimalCompany {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl std::fmt::Display for MaybeMinimalCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MaybeMinimalCompany {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(slug) = &self.slug {
                format!("{:?}", slug).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "slug".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyManagersResponseData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_managers: Option<Vec<CompanyManager>>,
    #[doc = "The current page among all of the total_pages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
    #[doc = "The total number of records in the result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The total number of pages the user can go through"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
}

impl std::fmt::Display for CompanyManagersResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyManagersResponseData {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(company_managers) = &self.company_managers {
                format!("{:?}", company_managers).into()
            } else {
                String::new().into()
            },
            if let Some(current_page) = &self.current_page {
                format!("{:?}", current_page).into()
            } else {
                String::new().into()
            },
            if let Some(total_count) = &self.total_count {
                format!("{:?}", total_count).into()
            } else {
                String::new().into()
            },
            if let Some(total_pages) = &self.total_pages {
                format!("{:?}", total_pages).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "company_managers".into(),
            "current_page".into(),
            "total_count".into(),
            "total_pages".into(),
        ]
    }
}

#[doc = "Response schema listing many company_managers"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyManagersResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<CompanyManagersResponseData>,
}

impl std::fmt::Display for CompanyManagersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyManagersResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[doc = "List of countries supported by Remote API"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CountriesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<Country>>,
}

impl std::fmt::Display for CountriesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CountriesResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyManagerCreatedResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_manager: Option<CompanyManager>,
}

impl std::fmt::Display for CompanyManagerCreatedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyManagerCreatedResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(company_manager) = &self.company_manager {
            format!("{:?}", company_manager).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["company_manager".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RemoteEntity {
    pub address: Address,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<MaybeMinimalCompany>,
    #[doc = "Identifies if the legal entity is owned by Remote"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_internal: Option<bool>,
    pub name: String,
    pub slug: String,
}

impl std::fmt::Display for RemoteEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RemoteEntity {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.address).into(),
            format!("{:?}", self.company).into(),
            if let Some(is_internal) = &self.is_internal {
                format!("{:?}", is_internal).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
            self.slug.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address".into(),
            "company".into(),
            "is_internal".into(),
            "name".into(),
            "slug".into(),
        ]
    }
}

#[doc = "Object with required and optional fields, its descriptions and suggested presentation"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CountryFormResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Data>,
}

impl std::fmt::Display for CountryFormResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CountryFormResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MinimalCompany {
    pub name: String,
    pub slug: String,
}

impl std::fmt::Display for MinimalCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MinimalCompany {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.name.clone().into(), self.slug.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "slug".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnprocessableEntityResponse {}

impl std::fmt::Display for UnprocessableEntityResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnprocessableEntityResponse {
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
pub struct TooManyRequestsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl std::fmt::Display for TooManyRequestsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TooManyRequestsResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(message) = &self.message {
            format!("{:?}", message).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Holiday {
    pub day: chrono::NaiveDate,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl std::fmt::Display for Holiday {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Holiday {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.day).into(),
            self.name.clone().into(),
            if let Some(note) = &self.note {
                format!("{:?}", note).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["day".into(), "name".into(), "note".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BadRequestResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl std::fmt::Display for BadRequestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BadRequestResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(message) = &self.message {
            format!("{:?}", message).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UnauthorizedResponse {
    pub message: String,
}

impl std::fmt::Display for UnauthorizedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UnauthorizedResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![self.message.clone().into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["message".into()]
    }
}

#[doc = "A subdivision of a supported country on Remote"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CountrySubdivision {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subdivision_type: Option<String>,
}

impl std::fmt::Display for CountrySubdivision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CountrySubdivision {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(code) = &self.code {
                format!("{:?}", code).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
            if let Some(subdivision_type) = &self.subdivision_type {
                format!("{:?}", subdivision_type).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["code".into(), "name".into(), "subdivision_type".into()]
    }
}

#[doc = "Timeoff creation params"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateTimeoffParams {
    #[doc = "Timeoff document params"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<TimeoffDocumentParams>,
    pub employment_id: String,
    #[doc = "UTC date in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format"]
    pub end_date: chrono::NaiveDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[doc = "UTC date in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format"]
    pub start_date: chrono::NaiveDate,
    pub timeoff_days: Vec<TimeoffDaysParams>,
    pub timeoff_type: TimeoffType,
    pub timezone: String,
}

impl std::fmt::Display for CreateTimeoffParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateTimeoffParams {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(document) = &self.document {
                format!("{:?}", document).into()
            } else {
                String::new().into()
            },
            self.employment_id.clone().into(),
            format!("{:?}", self.end_date).into(),
            if let Some(notes) = &self.notes {
                format!("{:?}", notes).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.start_date).into(),
            format!("{:?}", self.timeoff_days).into(),
            format!("{:?}", self.timeoff_type).into(),
            self.timezone.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "document".into(),
            "employment_id".into(),
            "end_date".into(),
            "notes".into(),
            "start_date".into(),
            "timeoff_days".into(),
            "timeoff_type".into(),
            "timezone".into(),
        ]
    }
}

#[doc = "Timeoff days params"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeoffDaysParams {
    #[doc = "UTC date in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day: Option<chrono::NaiveDate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours: Option<i64>,
}

impl std::fmt::Display for TimeoffDaysParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeoffDaysParams {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(day) = &self.day {
                format!("{:?}", day).into()
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
        vec!["day".into(), "hours".into()]
    }
}

#[doc = "All tasks that need to be completed before marking the employment as ready"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct OnboardingTasks {
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub administrative_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bank_account_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emergency_contact_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_document_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_details: Option<TaskDescription>,
    #[doc = "Description and status of an onboarding task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_plan_details: Option<TaskDescription>,
}

impl std::fmt::Display for OnboardingTasks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for OnboardingTasks {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.address_details).into(),
            format!("{:?}", self.administrative_details).into(),
            format!("{:?}", self.bank_account_details).into(),
            format!("{:?}", self.billing_address_details).into(),
            format!("{:?}", self.contract_details).into(),
            format!("{:?}", self.emergency_contact_details).into(),
            format!("{:?}", self.employment_document_details).into(),
            format!("{:?}", self.personal_details).into(),
            format!("{:?}", self.pricing_plan_details).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "address_details".into(),
            "administrative_details".into(),
            "bank_account_details".into(),
            "billing_address_details".into(),
            "contract_details".into(),
            "emergency_contact_details".into(),
            "employment_document_details".into(),
            "personal_details".into(),
            "pricing_plan_details".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTimeoffResponseData {
    #[doc = "The current page among all of the total_pages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeoffs: Option<Vec<Timeoff>>,
    #[doc = "The total number of records in the result"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    #[doc = "The total number of pages the user can go through"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
}

impl std::fmt::Display for ListTimeoffResponseData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTimeoffResponseData {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(current_page) = &self.current_page {
                format!("{:?}", current_page).into()
            } else {
                String::new().into()
            },
            if let Some(timeoffs) = &self.timeoffs {
                format!("{:?}", timeoffs).into()
            } else {
                String::new().into()
            },
            if let Some(total_count) = &self.total_count {
                format!("{:?}", total_count).into()
            } else {
                String::new().into()
            },
            if let Some(total_pages) = &self.total_pages {
                format!("{:?}", total_pages).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "current_page".into(),
            "timeoffs".into(),
            "total_count".into(),
            "total_pages".into(),
        ]
    }
}

#[doc = "Response schema listing many timeoffs"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTimeoffResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<ListTimeoffResponseData>,
}

impl std::fmt::Display for ListTimeoffResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTimeoffResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(data) = &self.data {
            format!("{:?}", data).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["data".into()]
    }
}

#[doc = "Required params to update an employment in the Sandbox environment.\n\nCurrently only \
         supports setting the Employment Status to `active`.\n"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EmploymentUpdateParams {
    #[doc = "The status of employment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EmploymentStatus>,
}

impl std::fmt::Display for EmploymentUpdateParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EmploymentUpdateParams {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(status) = &self.status {
            format!("{:?}", status).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["status".into()]
    }
}

#[doc = "Minimal information of an employment."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MinimalEmployment {
    #[doc = "A supported country on Remote"]
    pub country: Country,
    pub full_name: String,
    pub id: String,
    pub job_title: String,
    #[doc = "The status of employment"]
    pub status: EmploymentStatus,
}

impl std::fmt::Display for MinimalEmployment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MinimalEmployment {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.country).into(),
            self.full_name.clone().into(),
            self.id.clone().into(),
            self.job_title.clone().into(),
            format!("{:?}", self.status).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "country".into(),
            "full_name".into(),
            "id".into(),
            "job_title".into(),
            "status".into(),
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
pub enum OrderBy {
    #[serde(rename = "asc")]
    #[display("asc")]
    Asc,
    #[serde(rename = "desc")]
    #[display("desc")]
    Desc,
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
pub enum SortBy {
    #[serde(rename = "timeoff_type")]
    #[display("timeoff_type")]
    TimeoffType,
    #[serde(rename = "status")]
    #[display("status")]
    Status,
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
pub enum GetIndexTimeoffStatus {
    #[serde(rename = "approved")]
    #[display("approved")]
    Approved,
    #[serde(rename = "cancelled")]
    #[display("cancelled")]
    Cancelled,
    #[serde(rename = "declined")]
    #[display("declined")]
    Declined,
    #[serde(rename = "requested")]
    #[display("requested")]
    Requested,
    #[serde(rename = "taken")]
    #[display("taken")]
    Taken,
    #[serde(rename = "cancel_requested")]
    #[display("cancel_requested")]
    CancelRequested,
}
