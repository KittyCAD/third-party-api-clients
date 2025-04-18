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

#[doc = "The classification of the address."]
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
    #[serde(rename = "HOME")]
    #[display("HOME")]
    Home,
    #[serde(rename = "WORK")]
    #[display("WORK")]
    Work,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
}

#[doc = "The country component, pursuant to SCIM RFC 7643 4.1.2."]
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
pub enum AddressCountry {
    #[serde(rename = "AF")]
    #[display("AF")]
    Af,
    #[serde(rename = "AX")]
    #[display("AX")]
    Ax,
    #[serde(rename = "AL")]
    #[display("AL")]
    Al,
    #[serde(rename = "DZ")]
    #[display("DZ")]
    Dz,
    #[serde(rename = "AS")]
    #[display("AS")]
    As,
    #[serde(rename = "AD")]
    #[display("AD")]
    Ad,
    #[serde(rename = "AO")]
    #[display("AO")]
    Ao,
    #[serde(rename = "AI")]
    #[display("AI")]
    Ai,
    #[serde(rename = "AQ")]
    #[display("AQ")]
    Aq,
    #[serde(rename = "AG")]
    #[display("AG")]
    Ag,
    #[serde(rename = "AR")]
    #[display("AR")]
    Ar,
    #[serde(rename = "AM")]
    #[display("AM")]
    Am,
    #[serde(rename = "AW")]
    #[display("AW")]
    Aw,
    #[serde(rename = "AU")]
    #[display("AU")]
    Au,
    #[serde(rename = "AT")]
    #[display("AT")]
    At,
    #[serde(rename = "AZ")]
    #[display("AZ")]
    Az,
    #[serde(rename = "BS")]
    #[display("BS")]
    Bs,
    #[serde(rename = "BH")]
    #[display("BH")]
    Bh,
    #[serde(rename = "BD")]
    #[display("BD")]
    Bd,
    #[serde(rename = "BB")]
    #[display("BB")]
    Bb,
    #[serde(rename = "BY")]
    #[display("BY")]
    By,
    #[serde(rename = "BE")]
    #[display("BE")]
    Be,
    #[serde(rename = "BZ")]
    #[display("BZ")]
    Bz,
    #[serde(rename = "BJ")]
    #[display("BJ")]
    Bj,
    #[serde(rename = "BM")]
    #[display("BM")]
    Bm,
    #[serde(rename = "BT")]
    #[display("BT")]
    Bt,
    #[serde(rename = "BO")]
    #[display("BO")]
    Bo,
    #[serde(rename = "BQ")]
    #[display("BQ")]
    Bq,
    #[serde(rename = "BA")]
    #[display("BA")]
    Ba,
    #[serde(rename = "BW")]
    #[display("BW")]
    Bw,
    #[serde(rename = "BV")]
    #[display("BV")]
    Bv,
    #[serde(rename = "BR")]
    #[display("BR")]
    Br,
    #[serde(rename = "IO")]
    #[display("IO")]
    Io,
    #[serde(rename = "BN")]
    #[display("BN")]
    Bn,
    #[serde(rename = "BG")]
    #[display("BG")]
    Bg,
    #[serde(rename = "BF")]
    #[display("BF")]
    Bf,
    #[serde(rename = "BI")]
    #[display("BI")]
    Bi,
    #[serde(rename = "CV")]
    #[display("CV")]
    Cv,
    #[serde(rename = "KH")]
    #[display("KH")]
    Kh,
    #[serde(rename = "CM")]
    #[display("CM")]
    Cm,
    #[serde(rename = "CA")]
    #[display("CA")]
    Ca,
    #[serde(rename = "KY")]
    #[display("KY")]
    Ky,
    #[serde(rename = "CF")]
    #[display("CF")]
    Cf,
    #[serde(rename = "TD")]
    #[display("TD")]
    Td,
    #[serde(rename = "CL")]
    #[display("CL")]
    Cl,
    #[serde(rename = "CN")]
    #[display("CN")]
    Cn,
    #[serde(rename = "CX")]
    #[display("CX")]
    Cx,
    #[serde(rename = "CC")]
    #[display("CC")]
    Cc,
    #[serde(rename = "CO")]
    #[display("CO")]
    Co,
    #[serde(rename = "KM")]
    #[display("KM")]
    Km,
    #[serde(rename = "CG")]
    #[display("CG")]
    Cg,
    #[serde(rename = "CD")]
    #[display("CD")]
    Cd,
    #[serde(rename = "CK")]
    #[display("CK")]
    Ck,
    #[serde(rename = "CR")]
    #[display("CR")]
    Cr,
    #[serde(rename = "CI")]
    #[display("CI")]
    Ci,
    #[serde(rename = "HR")]
    #[display("HR")]
    Hr,
    #[serde(rename = "CW")]
    #[display("CW")]
    Cw,
    #[serde(rename = "CY")]
    #[display("CY")]
    Cy,
    #[serde(rename = "CZ")]
    #[display("CZ")]
    Cz,
    #[serde(rename = "DK")]
    #[display("DK")]
    Dk,
    #[serde(rename = "DJ")]
    #[display("DJ")]
    Dj,
    #[serde(rename = "DM")]
    #[display("DM")]
    Dm,
    #[serde(rename = "DO")]
    #[display("DO")]
    Do,
    #[serde(rename = "EC")]
    #[display("EC")]
    Ec,
    #[serde(rename = "EG")]
    #[display("EG")]
    Eg,
    #[serde(rename = "SV")]
    #[display("SV")]
    Sv,
    #[serde(rename = "GQ")]
    #[display("GQ")]
    Gq,
    #[serde(rename = "ER")]
    #[display("ER")]
    Er,
    #[serde(rename = "EE")]
    #[display("EE")]
    Ee,
    #[serde(rename = "SZ")]
    #[display("SZ")]
    Sz,
    #[serde(rename = "ET")]
    #[display("ET")]
    Et,
    #[serde(rename = "FK")]
    #[display("FK")]
    Fk,
    #[serde(rename = "FO")]
    #[display("FO")]
    Fo,
    #[serde(rename = "FJ")]
    #[display("FJ")]
    Fj,
    #[serde(rename = "FI")]
    #[display("FI")]
    Fi,
    #[serde(rename = "FR")]
    #[display("FR")]
    Fr,
    #[serde(rename = "GF")]
    #[display("GF")]
    Gf,
    #[serde(rename = "PF")]
    #[display("PF")]
    Pf,
    #[serde(rename = "TF")]
    #[display("TF")]
    Tf,
    #[serde(rename = "GA")]
    #[display("GA")]
    Ga,
    #[serde(rename = "GM")]
    #[display("GM")]
    Gm,
    #[serde(rename = "GE")]
    #[display("GE")]
    Ge,
    #[serde(rename = "DE")]
    #[display("DE")]
    De,
    #[serde(rename = "GH")]
    #[display("GH")]
    Gh,
    #[serde(rename = "GI")]
    #[display("GI")]
    Gi,
    #[serde(rename = "GR")]
    #[display("GR")]
    Gr,
    #[serde(rename = "GL")]
    #[display("GL")]
    Gl,
    #[serde(rename = "GD")]
    #[display("GD")]
    Gd,
    #[serde(rename = "GP")]
    #[display("GP")]
    Gp,
    #[serde(rename = "GU")]
    #[display("GU")]
    Gu,
    #[serde(rename = "GT")]
    #[display("GT")]
    Gt,
    #[serde(rename = "GG")]
    #[display("GG")]
    Gg,
    #[serde(rename = "GN")]
    #[display("GN")]
    Gn,
    #[serde(rename = "GW")]
    #[display("GW")]
    Gw,
    #[serde(rename = "GY")]
    #[display("GY")]
    Gy,
    #[serde(rename = "HT")]
    #[display("HT")]
    Ht,
    #[serde(rename = "HM")]
    #[display("HM")]
    Hm,
    #[serde(rename = "VA")]
    #[display("VA")]
    Va,
    #[serde(rename = "HN")]
    #[display("HN")]
    Hn,
    #[serde(rename = "HK")]
    #[display("HK")]
    Hk,
    #[serde(rename = "HU")]
    #[display("HU")]
    Hu,
    #[serde(rename = "IS")]
    #[display("IS")]
    Is,
    #[serde(rename = "IN")]
    #[display("IN")]
    In,
    #[serde(rename = "ID")]
    #[display("ID")]
    Id,
    #[serde(rename = "IQ")]
    #[display("IQ")]
    Iq,
    #[serde(rename = "IE")]
    #[display("IE")]
    Ie,
    #[serde(rename = "IM")]
    #[display("IM")]
    Im,
    #[serde(rename = "IL")]
    #[display("IL")]
    Il,
    #[serde(rename = "IT")]
    #[display("IT")]
    It,
    #[serde(rename = "JM")]
    #[display("JM")]
    Jm,
    #[serde(rename = "JP")]
    #[display("JP")]
    Jp,
    #[serde(rename = "JE")]
    #[display("JE")]
    Je,
    #[serde(rename = "JO")]
    #[display("JO")]
    Jo,
    #[serde(rename = "KZ")]
    #[display("KZ")]
    Kz,
    #[serde(rename = "KE")]
    #[display("KE")]
    Ke,
    #[serde(rename = "KI")]
    #[display("KI")]
    Ki,
    #[serde(rename = "KR")]
    #[display("KR")]
    Kr,
    #[serde(rename = "XK")]
    #[display("XK")]
    Xk,
    #[serde(rename = "KW")]
    #[display("KW")]
    Kw,
    #[serde(rename = "KG")]
    #[display("KG")]
    Kg,
    #[serde(rename = "LA")]
    #[display("LA")]
    La,
    #[serde(rename = "LV")]
    #[display("LV")]
    Lv,
    #[serde(rename = "LB")]
    #[display("LB")]
    Lb,
    #[serde(rename = "LS")]
    #[display("LS")]
    Ls,
    #[serde(rename = "LR")]
    #[display("LR")]
    Lr,
    #[serde(rename = "LY")]
    #[display("LY")]
    Ly,
    #[serde(rename = "LI")]
    #[display("LI")]
    Li,
    #[serde(rename = "LT")]
    #[display("LT")]
    Lt,
    #[serde(rename = "LU")]
    #[display("LU")]
    Lu,
    #[serde(rename = "MO")]
    #[display("MO")]
    Mo,
    #[serde(rename = "MG")]
    #[display("MG")]
    Mg,
    #[serde(rename = "MW")]
    #[display("MW")]
    Mw,
    #[serde(rename = "MY")]
    #[display("MY")]
    My,
    #[serde(rename = "MV")]
    #[display("MV")]
    Mv,
    #[serde(rename = "ML")]
    #[display("ML")]
    Ml,
    #[serde(rename = "MT")]
    #[display("MT")]
    Mt,
    #[serde(rename = "MH")]
    #[display("MH")]
    Mh,
    #[serde(rename = "MQ")]
    #[display("MQ")]
    Mq,
    #[serde(rename = "MR")]
    #[display("MR")]
    Mr,
    #[serde(rename = "MU")]
    #[display("MU")]
    Mu,
    #[serde(rename = "YT")]
    #[display("YT")]
    Yt,
    #[serde(rename = "MX")]
    #[display("MX")]
    Mx,
    #[serde(rename = "FM")]
    #[display("FM")]
    Fm,
    #[serde(rename = "MD")]
    #[display("MD")]
    Md,
    #[serde(rename = "MC")]
    #[display("MC")]
    Mc,
    #[serde(rename = "MN")]
    #[display("MN")]
    Mn,
    #[serde(rename = "ME")]
    #[display("ME")]
    Me,
    #[serde(rename = "MS")]
    #[display("MS")]
    Ms,
    #[serde(rename = "MA")]
    #[display("MA")]
    Ma,
    #[serde(rename = "MZ")]
    #[display("MZ")]
    Mz,
    #[serde(rename = "MM")]
    #[display("MM")]
    Mm,
    #[serde(rename = "NA")]
    #[display("NA")]
    Na,
    #[serde(rename = "NR")]
    #[display("NR")]
    Nr,
    #[serde(rename = "NP")]
    #[display("NP")]
    Np,
    #[serde(rename = "NL")]
    #[display("NL")]
    Nl,
    #[serde(rename = "AN")]
    #[display("AN")]
    An,
    #[serde(rename = "NC")]
    #[display("NC")]
    Nc,
    #[serde(rename = "NZ")]
    #[display("NZ")]
    Nz,
    #[serde(rename = "NI")]
    #[display("NI")]
    Ni,
    #[serde(rename = "NE")]
    #[display("NE")]
    Ne,
    #[serde(rename = "NG")]
    #[display("NG")]
    Ng,
    #[serde(rename = "NU")]
    #[display("NU")]
    Nu,
    #[serde(rename = "NF")]
    #[display("NF")]
    Nf,
    #[serde(rename = "MK")]
    #[display("MK")]
    Mk,
    #[serde(rename = "MP")]
    #[display("MP")]
    Mp,
    #[serde(rename = "NO")]
    #[display("NO")]
    No,
    #[serde(rename = "OM")]
    #[display("OM")]
    Om,
    #[serde(rename = "PK")]
    #[display("PK")]
    Pk,
    #[serde(rename = "PW")]
    #[display("PW")]
    Pw,
    #[serde(rename = "PS")]
    #[display("PS")]
    Ps,
    #[serde(rename = "PA")]
    #[display("PA")]
    Pa,
    #[serde(rename = "PG")]
    #[display("PG")]
    Pg,
    #[serde(rename = "PY")]
    #[display("PY")]
    Py,
    #[serde(rename = "PE")]
    #[display("PE")]
    Pe,
    #[serde(rename = "PH")]
    #[display("PH")]
    Ph,
    #[serde(rename = "PN")]
    #[display("PN")]
    Pn,
    #[serde(rename = "PL")]
    #[display("PL")]
    Pl,
    #[serde(rename = "PT")]
    #[display("PT")]
    Pt,
    #[serde(rename = "PR")]
    #[display("PR")]
    Pr,
    #[serde(rename = "QA")]
    #[display("QA")]
    Qa,
    #[serde(rename = "RO")]
    #[display("RO")]
    Ro,
    #[serde(rename = "RU")]
    #[display("RU")]
    Ru,
    #[serde(rename = "RW")]
    #[display("RW")]
    Rw,
    #[serde(rename = "RE")]
    #[display("RE")]
    Re,
    #[serde(rename = "BL")]
    #[display("BL")]
    Bl,
    #[serde(rename = "SH")]
    #[display("SH")]
    Sh,
    #[serde(rename = "KN")]
    #[display("KN")]
    Kn,
    #[serde(rename = "LC")]
    #[display("LC")]
    Lc,
    #[serde(rename = "MF")]
    #[display("MF")]
    Mf,
    #[serde(rename = "PM")]
    #[display("PM")]
    Pm,
    #[serde(rename = "VC")]
    #[display("VC")]
    Vc,
    #[serde(rename = "WS")]
    #[display("WS")]
    Ws,
    #[serde(rename = "SM")]
    #[display("SM")]
    Sm,
    #[serde(rename = "ST")]
    #[display("ST")]
    St,
    #[serde(rename = "SA")]
    #[display("SA")]
    Sa,
    #[serde(rename = "SN")]
    #[display("SN")]
    Sn,
    #[serde(rename = "RS")]
    #[display("RS")]
    Rs,
    #[serde(rename = "SC")]
    #[display("SC")]
    Sc,
    #[serde(rename = "SL")]
    #[display("SL")]
    Sl,
    #[serde(rename = "SG")]
    #[display("SG")]
    Sg,
    #[serde(rename = "SX")]
    #[display("SX")]
    Sx,
    #[serde(rename = "SK")]
    #[display("SK")]
    Sk,
    #[serde(rename = "SI")]
    #[display("SI")]
    Si,
    #[serde(rename = "SB")]
    #[display("SB")]
    Sb,
    #[serde(rename = "SO")]
    #[display("SO")]
    So,
    #[serde(rename = "ZA")]
    #[display("ZA")]
    Za,
    #[serde(rename = "GS")]
    #[display("GS")]
    Gs,
    #[serde(rename = "SS")]
    #[display("SS")]
    Ss,
    #[serde(rename = "ES")]
    #[display("ES")]
    Es,
    #[serde(rename = "LK")]
    #[display("LK")]
    Lk,
    #[serde(rename = "SD")]
    #[display("SD")]
    Sd,
    #[serde(rename = "SR")]
    #[display("SR")]
    Sr,
    #[serde(rename = "SJ")]
    #[display("SJ")]
    Sj,
    #[serde(rename = "SE")]
    #[display("SE")]
    Se,
    #[serde(rename = "CH")]
    #[display("CH")]
    Ch,
    #[serde(rename = "TW")]
    #[display("TW")]
    Tw,
    #[serde(rename = "TJ")]
    #[display("TJ")]
    Tj,
    #[serde(rename = "TZ")]
    #[display("TZ")]
    Tz,
    #[serde(rename = "TH")]
    #[display("TH")]
    Th,
    #[serde(rename = "TL")]
    #[display("TL")]
    Tl,
    #[serde(rename = "TG")]
    #[display("TG")]
    Tg,
    #[serde(rename = "TK")]
    #[display("TK")]
    Tk,
    #[serde(rename = "TO")]
    #[display("TO")]
    To,
    #[serde(rename = "TT")]
    #[display("TT")]
    Tt,
    #[serde(rename = "TN")]
    #[display("TN")]
    Tn,
    #[serde(rename = "TR")]
    #[display("TR")]
    Tr,
    #[serde(rename = "TM")]
    #[display("TM")]
    Tm,
    #[serde(rename = "TC")]
    #[display("TC")]
    Tc,
    #[serde(rename = "TV")]
    #[display("TV")]
    Tv,
    #[serde(rename = "UG")]
    #[display("UG")]
    Ug,
    #[serde(rename = "UA")]
    #[display("UA")]
    Ua,
    #[serde(rename = "AE")]
    #[display("AE")]
    Ae,
    #[serde(rename = "GB")]
    #[display("GB")]
    Gb,
    #[serde(rename = "US")]
    #[display("US")]
    Us,
    #[serde(rename = "UM")]
    #[display("UM")]
    Um,
    #[serde(rename = "UY")]
    #[display("UY")]
    Uy,
    #[serde(rename = "UZ")]
    #[display("UZ")]
    Uz,
    #[serde(rename = "VU")]
    #[display("VU")]
    Vu,
    #[serde(rename = "VE")]
    #[display("VE")]
    Ve,
    #[serde(rename = "VN")]
    #[display("VN")]
    Vn,
    #[serde(rename = "VG")]
    #[display("VG")]
    Vg,
    #[serde(rename = "VI")]
    #[display("VI")]
    Vi,
    #[serde(rename = "WF")]
    #[display("WF")]
    Wf,
    #[serde(rename = "EH")]
    #[display("EH")]
    Eh,
    #[serde(rename = "YE")]
    #[display("YE")]
    Ye,
    #[serde(rename = "ZM")]
    #[display("ZM")]
    Zm,
    #[serde(rename = "ZW")]
    #[display("ZW")]
    Zw,
}

#[doc = "Address."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Address {
    #[doc = "The classification of the address."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[doc = "The formatted mailing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[doc = "The full street address component, which may include house number, street name, P.O. \
             box, and multi-line extended street address information, pursuant to SCIM RFC 7643 \
             4.1.2.."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[doc = "The city or locality component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[doc = "The state or region component, pursuant to SCIM RFC 7643 4.1.2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The zip code or postal code component, pursuant to SCIM RFC 7643 4.1.2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "The country component, pursuant to SCIM RFC 7643 4.1.2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<AddressCountry>,
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
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(formatted) = &self.formatted {
                format!("{:?}", formatted).into()
            } else {
                String::new().into()
            },
            if let Some(street_address) = &self.street_address {
                format!("{:?}", street_address).into()
            } else {
                String::new().into()
            },
            if let Some(locality) = &self.locality {
                format!("{:?}", locality).into()
            } else {
                String::new().into()
            },
            if let Some(region) = &self.region {
                format!("{:?}", region).into()
            } else {
                String::new().into()
            },
            if let Some(postal_code) = &self.postal_code {
                format!("{:?}", postal_code).into()
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
            "type_".into(),
            "formatted".into(),
            "street_address".into(),
            "locality".into(),
            "region".into(),
            "postal_code".into(),
            "country".into(),
        ]
    }
}

#[doc = "Application status"]
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
    #[serde(rename = "ACTIVE")]
    #[display("ACTIVE")]
    Active,
    #[serde(rename = "REJECTED")]
    #[display("REJECTED")]
    Rejected,
    #[serde(rename = "HIRED")]
    #[display("HIRED")]
    Hired,
    #[serde(rename = "ARCHIVED")]
    #[display("ARCHIVED")]
    Archived,
}

#[doc = "Application."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Application {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "Application status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[doc = "Application stage"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[doc = "Application creation date"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub applied_at: Option<String>,
    #[doc = "Job requisition ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[doc = "Job requisition\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job: Option<JobRequisition>,
    #[doc = "Application url"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl std::fmt::Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Application {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(stage) = &self.stage {
                format!("{:?}", stage).into()
            } else {
                String::new().into()
            },
            if let Some(applied_at) = &self.applied_at {
                format!("{:?}", applied_at).into()
            } else {
                String::new().into()
            },
            if let Some(job_id) = &self.job_id {
                format!("{:?}", job_id).into()
            } else {
                String::new().into()
            },
            if let Some(job) = &self.job {
                format!("{:?}", job).into()
            } else {
                String::new().into()
            },
            if let Some(url) = &self.url {
                format!("{:?}", url).into()
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
            "status".into(),
            "stage".into(),
            "applied_at".into(),
            "job_id".into(),
            "job".into(),
            "url".into(),
        ]
    }
}

#[doc = "Break."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Break {
    #[doc = "The start time of the break."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the break."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The original start time of the break. If the startTime field has been rounded then \
             this contain the start time before the rounding occured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_time: Option<String>,
    #[doc = "The original end time of the break. If the endTime field has been rounded then this \
             contain the end time before the rounding occured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_end_time: Option<String>,
    #[doc = "The ID of the break type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub break_type_id: Option<String>,
}

impl std::fmt::Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Break {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(original_start_time) = &self.original_start_time {
                format!("{:?}", original_start_time).into()
            } else {
                String::new().into()
            },
            if let Some(original_end_time) = &self.original_end_time {
                format!("{:?}", original_end_time).into()
            } else {
                String::new().into()
            },
            if let Some(break_type_id) = &self.break_type_id {
                format!("{:?}", break_type_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_time".into(),
            "end_time".into(),
            "original_start_time".into(),
            "original_end_time".into(),
            "break_type_id".into(),
        ]
    }
}

#[doc = "BreakRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BreakRequest {
    #[doc = "The start time of the break."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the break."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The ID of the break type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub break_type_id: Option<String>,
}

impl std::fmt::Display for BreakRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BreakRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(break_type_id) = &self.break_type_id {
                format!("{:?}", break_type_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_time".into(),
            "end_time".into(),
            "break_type_id".into(),
        ]
    }
}

#[doc = "Candidate."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Candidate {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "Candidate first name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[doc = "Candidate last name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[doc = "Candidate email"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Candidate phone number"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[doc = "Candidate timezone"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
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
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
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
            if let Some(phone_number) = &self.phone_number {
                format!("{:?}", phone_number).into()
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
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "first_name".into(),
            "last_name".into(),
            "email".into(),
            "phone_number".into(),
            "timezone".into(),
        ]
    }
}

#[doc = "Company."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Company {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The company's ultimate holding entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_legal_entity_id: Option<String>,
    #[doc = "A list of the company's entities."]
    pub legal_entities_id: Vec<String>,
    #[doc = "The physical address of the holding entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_address: Option<Address>,
    #[doc = "The email address used when registering this company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
    #[doc = "The legal name of the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[doc = "The doing business as name for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doing_business_as_name: Option<String>,
    #[doc = "The phone number for the company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[doc = "The name of the company."]
    pub name: String,
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
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(parent_legal_entity_id) = &self.parent_legal_entity_id {
                format!("{:?}", parent_legal_entity_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.legal_entities_id).into(),
            if let Some(physical_address) = &self.physical_address {
                format!("{:?}", physical_address).into()
            } else {
                String::new().into()
            },
            if let Some(primary_email) = &self.primary_email {
                format!("{:?}", primary_email).into()
            } else {
                String::new().into()
            },
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
            if let Some(doing_business_as_name) = &self.doing_business_as_name {
                format!("{:?}", doing_business_as_name).into()
            } else {
                String::new().into()
            },
            if let Some(phone) = &self.phone {
                format!("{:?}", phone).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "parent_legal_entity_id".into(),
            "legal_entities_id".into(),
            "physical_address".into(),
            "primary_email".into(),
            "legal_name".into(),
            "doing_business_as_name".into(),
            "phone".into(),
            "name".into(),
        ]
    }
}

#[doc = "The classification of the worker by the company. * `CONTRACTOR`: Contractors are \
         self-employed workers who provide services on a short-term or per-project basis and are \
         not eligible for tax-withholding or benefits. * `EMPLOYEE`: Employees are hired and \
         managed by an employer, work under the employer's direct supervision and control, and are \
         protected by law for wages and employment rights."]
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
pub enum CompanyEmploymentTypeType {
    #[serde(rename = "CONTRACTOR")]
    #[display("CONTRACTOR")]
    Contractor,
    #[serde(rename = "EMPLOYEE")]
    #[display("EMPLOYEE")]
    Employee,
}

#[doc = "The compensation period for the employment type. * `SALARIED`: Employees that are paid a \
         fixed amount per year. * `HOURLY`: Employees that are paid a wage per hour worked."]
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
pub enum CompensationTimePeriod {
    #[serde(rename = "HOURLY")]
    #[display("HOURLY")]
    Hourly,
    #[serde(rename = "SALARIED")]
    #[display("SALARIED")]
    Salaried,
}

#[doc = "The amount worked for the employment type. * `FULL-TIME`: Full-time is at least 30 hours \
         per week. Full-time workers will typically be eligible for benefits. * `PART-TIME`: \
         Part-time is less than 30 hours per week. These workers may be eligible for benefits, \
         depending on company settings and hours worked. * `TEMPORARY`: These workers are hired on \
         a temporary basis. You can specify how each worker with this employment type will be paid \
         individually."]
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
pub enum AmountWorked {
    #[serde(rename = "PART-TIME")]
    #[display("PART-TIME")]
    PartTime,
    #[serde(rename = "FULL-TIME")]
    #[display("FULL-TIME")]
    FullTime,
    #[serde(rename = "TEMPORARY")]
    #[display("TEMPORARY")]
    Temporary,
}

#[doc = "CompanyEmploymentType."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CompanyEmploymentType {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The display label of the employment type."]
    pub label: String,
    #[doc = "The name of the employment type for non-custom employment types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The classification of the worker by the company. * `CONTRACTOR`: Contractors are \
             self-employed workers who provide services on a short-term or per-project basis and \
             are not eligible for tax-withholding or benefits. * `EMPLOYEE`: Employees are hired \
             and managed by an employer, work under the employer's direct supervision and \
             control, and are protected by law for wages and employment rights."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<CompanyEmploymentTypeType>,
    #[doc = "The compensation period for the employment type. * `SALARIED`: Employees that are \
             paid a fixed amount per year. * `HOURLY`: Employees that are paid a wage per hour \
             worked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_time_period: Option<CompensationTimePeriod>,
    #[doc = "The amount worked for the employment type. * `FULL-TIME`: Full-time is at least 30 \
             hours per week. Full-time workers will typically be eligible for benefits. * \
             `PART-TIME`: Part-time is less than 30 hours per week. These workers may be eligible \
             for benefits, depending on company settings and hours worked. * `TEMPORARY`: These \
             workers are hired on a temporary basis. You can specify how each worker with this \
             employment type will be paid individually."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_worked: Option<AmountWorked>,
}

impl std::fmt::Display for CompanyEmploymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CompanyEmploymentType {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.label.clone().into(),
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
            if let Some(compensation_time_period) = &self.compensation_time_period {
                format!("{:?}", compensation_time_period).into()
            } else {
                String::new().into()
            },
            if let Some(amount_worked) = &self.amount_worked {
                format!("{:?}", amount_worked).into()
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
            "label".into(),
            "name".into(),
            "type_".into(),
            "compensation_time_period".into(),
            "amount_worked".into(),
        ]
    }
}

#[doc = "Compensation."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Compensation {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The worker's ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[doc = "The worker's annual compensation. This calculation assumes 40-hour work weeks for \
             workers with an hourly wage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_compensation: Option<Currency>,
    #[doc = "The worker's annual salary equivalent, for insurance purposes. It will be equal to \
             the worker's annual compensation, except for owners that are receiving no \
             cashcompensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_salary_equivalent: Option<Currency>,
    #[doc = "The worker's hourly wage. This calculation assumes 40-hour work weeks for workers \
             with fixed compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_wage: Option<Currency>,
    #[doc = "The worker's monthly compensation. This calculation assumes 40-hour work weeks for \
             workers with an hourly wage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_compensation: Option<Currency>,
    #[doc = "The worker's on-target commission."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_target_commission: Option<Currency>,
    #[doc = "The worker's hourly wage. This calculation assumes 40-hour work weeks for workers \
             with fixed compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relocation_reimbursement: Option<Currency>,
    #[doc = "The worker's signing bonus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_bonus: Option<Currency>,
    #[doc = "The worker's target annual bonus amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_annual_bonus: Option<Currency>,
    #[doc = "The worker's weekly compensation. This calculation assumes 40-hour work weeks for \
             workers with an hourly wage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_compensation: Option<Currency>,
    #[doc = "The worker's target annual bonus as a percent of annual compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_annual_bonus_percent: Option<f64>,
    #[doc = "The worker's bonus schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus_schedule: Option<String>,
    #[doc = "The payment type for an worker's compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[doc = "The payment terms for an worker's compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<String>,
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
    const LENGTH: usize = 17;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(worker_id) = &self.worker_id {
                format!("{:?}", worker_id).into()
            } else {
                String::new().into()
            },
            if let Some(annual_compensation) = &self.annual_compensation {
                format!("{:?}", annual_compensation).into()
            } else {
                String::new().into()
            },
            if let Some(annual_salary_equivalent) = &self.annual_salary_equivalent {
                format!("{:?}", annual_salary_equivalent).into()
            } else {
                String::new().into()
            },
            if let Some(hourly_wage) = &self.hourly_wage {
                format!("{:?}", hourly_wage).into()
            } else {
                String::new().into()
            },
            if let Some(monthly_compensation) = &self.monthly_compensation {
                format!("{:?}", monthly_compensation).into()
            } else {
                String::new().into()
            },
            if let Some(on_target_commission) = &self.on_target_commission {
                format!("{:?}", on_target_commission).into()
            } else {
                String::new().into()
            },
            if let Some(relocation_reimbursement) = &self.relocation_reimbursement {
                format!("{:?}", relocation_reimbursement).into()
            } else {
                String::new().into()
            },
            if let Some(signing_bonus) = &self.signing_bonus {
                format!("{:?}", signing_bonus).into()
            } else {
                String::new().into()
            },
            if let Some(target_annual_bonus) = &self.target_annual_bonus {
                format!("{:?}", target_annual_bonus).into()
            } else {
                String::new().into()
            },
            if let Some(weekly_compensation) = &self.weekly_compensation {
                format!("{:?}", weekly_compensation).into()
            } else {
                String::new().into()
            },
            if let Some(target_annual_bonus_percent) = &self.target_annual_bonus_percent {
                format!("{:?}", target_annual_bonus_percent).into()
            } else {
                String::new().into()
            },
            if let Some(bonus_schedule) = &self.bonus_schedule {
                format!("{:?}", bonus_schedule).into()
            } else {
                String::new().into()
            },
            if let Some(payment_type) = &self.payment_type {
                format!("{:?}", payment_type).into()
            } else {
                String::new().into()
            },
            if let Some(payment_terms) = &self.payment_terms {
                format!("{:?}", payment_terms).into()
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
            "worker_id".into(),
            "annual_compensation".into(),
            "annual_salary_equivalent".into(),
            "hourly_wage".into(),
            "monthly_compensation".into(),
            "on_target_commission".into(),
            "relocation_reimbursement".into(),
            "signing_bonus".into(),
            "target_annual_bonus".into(),
            "weekly_compensation".into(),
            "target_annual_bonus_percent".into(),
            "bonus_schedule".into(),
            "payment_type".into(),
            "payment_terms".into(),
        ]
    }
}

#[doc = "The code of the country."]
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
pub enum Code {
    #[serde(rename = "AF")]
    #[display("AF")]
    Af,
    #[serde(rename = "AX")]
    #[display("AX")]
    Ax,
    #[serde(rename = "AL")]
    #[display("AL")]
    Al,
    #[serde(rename = "DZ")]
    #[display("DZ")]
    Dz,
    #[serde(rename = "AS")]
    #[display("AS")]
    As,
    #[serde(rename = "AD")]
    #[display("AD")]
    Ad,
    #[serde(rename = "AO")]
    #[display("AO")]
    Ao,
    #[serde(rename = "AI")]
    #[display("AI")]
    Ai,
    #[serde(rename = "AQ")]
    #[display("AQ")]
    Aq,
    #[serde(rename = "AG")]
    #[display("AG")]
    Ag,
    #[serde(rename = "AR")]
    #[display("AR")]
    Ar,
    #[serde(rename = "AM")]
    #[display("AM")]
    Am,
    #[serde(rename = "AW")]
    #[display("AW")]
    Aw,
    #[serde(rename = "AU")]
    #[display("AU")]
    Au,
    #[serde(rename = "AT")]
    #[display("AT")]
    At,
    #[serde(rename = "AZ")]
    #[display("AZ")]
    Az,
    #[serde(rename = "BS")]
    #[display("BS")]
    Bs,
    #[serde(rename = "BH")]
    #[display("BH")]
    Bh,
    #[serde(rename = "BD")]
    #[display("BD")]
    Bd,
    #[serde(rename = "BB")]
    #[display("BB")]
    Bb,
    #[serde(rename = "BY")]
    #[display("BY")]
    By,
    #[serde(rename = "BE")]
    #[display("BE")]
    Be,
    #[serde(rename = "BZ")]
    #[display("BZ")]
    Bz,
    #[serde(rename = "BJ")]
    #[display("BJ")]
    Bj,
    #[serde(rename = "BM")]
    #[display("BM")]
    Bm,
    #[serde(rename = "BT")]
    #[display("BT")]
    Bt,
    #[serde(rename = "BO")]
    #[display("BO")]
    Bo,
    #[serde(rename = "BQ")]
    #[display("BQ")]
    Bq,
    #[serde(rename = "BA")]
    #[display("BA")]
    Ba,
    #[serde(rename = "BW")]
    #[display("BW")]
    Bw,
    #[serde(rename = "BV")]
    #[display("BV")]
    Bv,
    #[serde(rename = "BR")]
    #[display("BR")]
    Br,
    #[serde(rename = "IO")]
    #[display("IO")]
    Io,
    #[serde(rename = "BN")]
    #[display("BN")]
    Bn,
    #[serde(rename = "BG")]
    #[display("BG")]
    Bg,
    #[serde(rename = "BF")]
    #[display("BF")]
    Bf,
    #[serde(rename = "BI")]
    #[display("BI")]
    Bi,
    #[serde(rename = "CV")]
    #[display("CV")]
    Cv,
    #[serde(rename = "KH")]
    #[display("KH")]
    Kh,
    #[serde(rename = "CM")]
    #[display("CM")]
    Cm,
    #[serde(rename = "CA")]
    #[display("CA")]
    Ca,
    #[serde(rename = "KY")]
    #[display("KY")]
    Ky,
    #[serde(rename = "CF")]
    #[display("CF")]
    Cf,
    #[serde(rename = "TD")]
    #[display("TD")]
    Td,
    #[serde(rename = "CL")]
    #[display("CL")]
    Cl,
    #[serde(rename = "CN")]
    #[display("CN")]
    Cn,
    #[serde(rename = "CX")]
    #[display("CX")]
    Cx,
    #[serde(rename = "CC")]
    #[display("CC")]
    Cc,
    #[serde(rename = "CO")]
    #[display("CO")]
    Co,
    #[serde(rename = "KM")]
    #[display("KM")]
    Km,
    #[serde(rename = "CG")]
    #[display("CG")]
    Cg,
    #[serde(rename = "CD")]
    #[display("CD")]
    Cd,
    #[serde(rename = "CK")]
    #[display("CK")]
    Ck,
    #[serde(rename = "CR")]
    #[display("CR")]
    Cr,
    #[serde(rename = "CI")]
    #[display("CI")]
    Ci,
    #[serde(rename = "HR")]
    #[display("HR")]
    Hr,
    #[serde(rename = "CW")]
    #[display("CW")]
    Cw,
    #[serde(rename = "CY")]
    #[display("CY")]
    Cy,
    #[serde(rename = "CZ")]
    #[display("CZ")]
    Cz,
    #[serde(rename = "DK")]
    #[display("DK")]
    Dk,
    #[serde(rename = "DJ")]
    #[display("DJ")]
    Dj,
    #[serde(rename = "DM")]
    #[display("DM")]
    Dm,
    #[serde(rename = "DO")]
    #[display("DO")]
    Do,
    #[serde(rename = "EC")]
    #[display("EC")]
    Ec,
    #[serde(rename = "EG")]
    #[display("EG")]
    Eg,
    #[serde(rename = "SV")]
    #[display("SV")]
    Sv,
    #[serde(rename = "GQ")]
    #[display("GQ")]
    Gq,
    #[serde(rename = "ER")]
    #[display("ER")]
    Er,
    #[serde(rename = "EE")]
    #[display("EE")]
    Ee,
    #[serde(rename = "SZ")]
    #[display("SZ")]
    Sz,
    #[serde(rename = "ET")]
    #[display("ET")]
    Et,
    #[serde(rename = "FK")]
    #[display("FK")]
    Fk,
    #[serde(rename = "FO")]
    #[display("FO")]
    Fo,
    #[serde(rename = "FJ")]
    #[display("FJ")]
    Fj,
    #[serde(rename = "FI")]
    #[display("FI")]
    Fi,
    #[serde(rename = "FR")]
    #[display("FR")]
    Fr,
    #[serde(rename = "GF")]
    #[display("GF")]
    Gf,
    #[serde(rename = "PF")]
    #[display("PF")]
    Pf,
    #[serde(rename = "TF")]
    #[display("TF")]
    Tf,
    #[serde(rename = "GA")]
    #[display("GA")]
    Ga,
    #[serde(rename = "GM")]
    #[display("GM")]
    Gm,
    #[serde(rename = "GE")]
    #[display("GE")]
    Ge,
    #[serde(rename = "DE")]
    #[display("DE")]
    De,
    #[serde(rename = "GH")]
    #[display("GH")]
    Gh,
    #[serde(rename = "GI")]
    #[display("GI")]
    Gi,
    #[serde(rename = "GR")]
    #[display("GR")]
    Gr,
    #[serde(rename = "GL")]
    #[display("GL")]
    Gl,
    #[serde(rename = "GD")]
    #[display("GD")]
    Gd,
    #[serde(rename = "GP")]
    #[display("GP")]
    Gp,
    #[serde(rename = "GU")]
    #[display("GU")]
    Gu,
    #[serde(rename = "GT")]
    #[display("GT")]
    Gt,
    #[serde(rename = "GG")]
    #[display("GG")]
    Gg,
    #[serde(rename = "GN")]
    #[display("GN")]
    Gn,
    #[serde(rename = "GW")]
    #[display("GW")]
    Gw,
    #[serde(rename = "GY")]
    #[display("GY")]
    Gy,
    #[serde(rename = "HT")]
    #[display("HT")]
    Ht,
    #[serde(rename = "HM")]
    #[display("HM")]
    Hm,
    #[serde(rename = "VA")]
    #[display("VA")]
    Va,
    #[serde(rename = "HN")]
    #[display("HN")]
    Hn,
    #[serde(rename = "HK")]
    #[display("HK")]
    Hk,
    #[serde(rename = "HU")]
    #[display("HU")]
    Hu,
    #[serde(rename = "IS")]
    #[display("IS")]
    Is,
    #[serde(rename = "IN")]
    #[display("IN")]
    In,
    #[serde(rename = "ID")]
    #[display("ID")]
    Id,
    #[serde(rename = "IQ")]
    #[display("IQ")]
    Iq,
    #[serde(rename = "IE")]
    #[display("IE")]
    Ie,
    #[serde(rename = "IM")]
    #[display("IM")]
    Im,
    #[serde(rename = "IL")]
    #[display("IL")]
    Il,
    #[serde(rename = "IT")]
    #[display("IT")]
    It,
    #[serde(rename = "JM")]
    #[display("JM")]
    Jm,
    #[serde(rename = "JP")]
    #[display("JP")]
    Jp,
    #[serde(rename = "JE")]
    #[display("JE")]
    Je,
    #[serde(rename = "JO")]
    #[display("JO")]
    Jo,
    #[serde(rename = "KZ")]
    #[display("KZ")]
    Kz,
    #[serde(rename = "KE")]
    #[display("KE")]
    Ke,
    #[serde(rename = "KI")]
    #[display("KI")]
    Ki,
    #[serde(rename = "KR")]
    #[display("KR")]
    Kr,
    #[serde(rename = "XK")]
    #[display("XK")]
    Xk,
    #[serde(rename = "KW")]
    #[display("KW")]
    Kw,
    #[serde(rename = "KG")]
    #[display("KG")]
    Kg,
    #[serde(rename = "LA")]
    #[display("LA")]
    La,
    #[serde(rename = "LV")]
    #[display("LV")]
    Lv,
    #[serde(rename = "LB")]
    #[display("LB")]
    Lb,
    #[serde(rename = "LS")]
    #[display("LS")]
    Ls,
    #[serde(rename = "LR")]
    #[display("LR")]
    Lr,
    #[serde(rename = "LY")]
    #[display("LY")]
    Ly,
    #[serde(rename = "LI")]
    #[display("LI")]
    Li,
    #[serde(rename = "LT")]
    #[display("LT")]
    Lt,
    #[serde(rename = "LU")]
    #[display("LU")]
    Lu,
    #[serde(rename = "MO")]
    #[display("MO")]
    Mo,
    #[serde(rename = "MG")]
    #[display("MG")]
    Mg,
    #[serde(rename = "MW")]
    #[display("MW")]
    Mw,
    #[serde(rename = "MY")]
    #[display("MY")]
    My,
    #[serde(rename = "MV")]
    #[display("MV")]
    Mv,
    #[serde(rename = "ML")]
    #[display("ML")]
    Ml,
    #[serde(rename = "MT")]
    #[display("MT")]
    Mt,
    #[serde(rename = "MH")]
    #[display("MH")]
    Mh,
    #[serde(rename = "MQ")]
    #[display("MQ")]
    Mq,
    #[serde(rename = "MR")]
    #[display("MR")]
    Mr,
    #[serde(rename = "MU")]
    #[display("MU")]
    Mu,
    #[serde(rename = "YT")]
    #[display("YT")]
    Yt,
    #[serde(rename = "MX")]
    #[display("MX")]
    Mx,
    #[serde(rename = "FM")]
    #[display("FM")]
    Fm,
    #[serde(rename = "MD")]
    #[display("MD")]
    Md,
    #[serde(rename = "MC")]
    #[display("MC")]
    Mc,
    #[serde(rename = "MN")]
    #[display("MN")]
    Mn,
    #[serde(rename = "ME")]
    #[display("ME")]
    Me,
    #[serde(rename = "MS")]
    #[display("MS")]
    Ms,
    #[serde(rename = "MA")]
    #[display("MA")]
    Ma,
    #[serde(rename = "MZ")]
    #[display("MZ")]
    Mz,
    #[serde(rename = "MM")]
    #[display("MM")]
    Mm,
    #[serde(rename = "NA")]
    #[display("NA")]
    Na,
    #[serde(rename = "NR")]
    #[display("NR")]
    Nr,
    #[serde(rename = "NP")]
    #[display("NP")]
    Np,
    #[serde(rename = "NL")]
    #[display("NL")]
    Nl,
    #[serde(rename = "AN")]
    #[display("AN")]
    An,
    #[serde(rename = "NC")]
    #[display("NC")]
    Nc,
    #[serde(rename = "NZ")]
    #[display("NZ")]
    Nz,
    #[serde(rename = "NI")]
    #[display("NI")]
    Ni,
    #[serde(rename = "NE")]
    #[display("NE")]
    Ne,
    #[serde(rename = "NG")]
    #[display("NG")]
    Ng,
    #[serde(rename = "NU")]
    #[display("NU")]
    Nu,
    #[serde(rename = "NF")]
    #[display("NF")]
    Nf,
    #[serde(rename = "MK")]
    #[display("MK")]
    Mk,
    #[serde(rename = "MP")]
    #[display("MP")]
    Mp,
    #[serde(rename = "NO")]
    #[display("NO")]
    No,
    #[serde(rename = "OM")]
    #[display("OM")]
    Om,
    #[serde(rename = "PK")]
    #[display("PK")]
    Pk,
    #[serde(rename = "PW")]
    #[display("PW")]
    Pw,
    #[serde(rename = "PS")]
    #[display("PS")]
    Ps,
    #[serde(rename = "PA")]
    #[display("PA")]
    Pa,
    #[serde(rename = "PG")]
    #[display("PG")]
    Pg,
    #[serde(rename = "PY")]
    #[display("PY")]
    Py,
    #[serde(rename = "PE")]
    #[display("PE")]
    Pe,
    #[serde(rename = "PH")]
    #[display("PH")]
    Ph,
    #[serde(rename = "PN")]
    #[display("PN")]
    Pn,
    #[serde(rename = "PL")]
    #[display("PL")]
    Pl,
    #[serde(rename = "PT")]
    #[display("PT")]
    Pt,
    #[serde(rename = "PR")]
    #[display("PR")]
    Pr,
    #[serde(rename = "QA")]
    #[display("QA")]
    Qa,
    #[serde(rename = "RO")]
    #[display("RO")]
    Ro,
    #[serde(rename = "RU")]
    #[display("RU")]
    Ru,
    #[serde(rename = "RW")]
    #[display("RW")]
    Rw,
    #[serde(rename = "RE")]
    #[display("RE")]
    Re,
    #[serde(rename = "BL")]
    #[display("BL")]
    Bl,
    #[serde(rename = "SH")]
    #[display("SH")]
    Sh,
    #[serde(rename = "KN")]
    #[display("KN")]
    Kn,
    #[serde(rename = "LC")]
    #[display("LC")]
    Lc,
    #[serde(rename = "MF")]
    #[display("MF")]
    Mf,
    #[serde(rename = "PM")]
    #[display("PM")]
    Pm,
    #[serde(rename = "VC")]
    #[display("VC")]
    Vc,
    #[serde(rename = "WS")]
    #[display("WS")]
    Ws,
    #[serde(rename = "SM")]
    #[display("SM")]
    Sm,
    #[serde(rename = "ST")]
    #[display("ST")]
    St,
    #[serde(rename = "SA")]
    #[display("SA")]
    Sa,
    #[serde(rename = "SN")]
    #[display("SN")]
    Sn,
    #[serde(rename = "RS")]
    #[display("RS")]
    Rs,
    #[serde(rename = "SC")]
    #[display("SC")]
    Sc,
    #[serde(rename = "SL")]
    #[display("SL")]
    Sl,
    #[serde(rename = "SG")]
    #[display("SG")]
    Sg,
    #[serde(rename = "SX")]
    #[display("SX")]
    Sx,
    #[serde(rename = "SK")]
    #[display("SK")]
    Sk,
    #[serde(rename = "SI")]
    #[display("SI")]
    Si,
    #[serde(rename = "SB")]
    #[display("SB")]
    Sb,
    #[serde(rename = "SO")]
    #[display("SO")]
    So,
    #[serde(rename = "ZA")]
    #[display("ZA")]
    Za,
    #[serde(rename = "GS")]
    #[display("GS")]
    Gs,
    #[serde(rename = "SS")]
    #[display("SS")]
    Ss,
    #[serde(rename = "ES")]
    #[display("ES")]
    Es,
    #[serde(rename = "LK")]
    #[display("LK")]
    Lk,
    #[serde(rename = "SD")]
    #[display("SD")]
    Sd,
    #[serde(rename = "SR")]
    #[display("SR")]
    Sr,
    #[serde(rename = "SJ")]
    #[display("SJ")]
    Sj,
    #[serde(rename = "SE")]
    #[display("SE")]
    Se,
    #[serde(rename = "CH")]
    #[display("CH")]
    Ch,
    #[serde(rename = "TW")]
    #[display("TW")]
    Tw,
    #[serde(rename = "TJ")]
    #[display("TJ")]
    Tj,
    #[serde(rename = "TZ")]
    #[display("TZ")]
    Tz,
    #[serde(rename = "TH")]
    #[display("TH")]
    Th,
    #[serde(rename = "TL")]
    #[display("TL")]
    Tl,
    #[serde(rename = "TG")]
    #[display("TG")]
    Tg,
    #[serde(rename = "TK")]
    #[display("TK")]
    Tk,
    #[serde(rename = "TO")]
    #[display("TO")]
    To,
    #[serde(rename = "TT")]
    #[display("TT")]
    Tt,
    #[serde(rename = "TN")]
    #[display("TN")]
    Tn,
    #[serde(rename = "TR")]
    #[display("TR")]
    Tr,
    #[serde(rename = "TM")]
    #[display("TM")]
    Tm,
    #[serde(rename = "TC")]
    #[display("TC")]
    Tc,
    #[serde(rename = "TV")]
    #[display("TV")]
    Tv,
    #[serde(rename = "UG")]
    #[display("UG")]
    Ug,
    #[serde(rename = "UA")]
    #[display("UA")]
    Ua,
    #[serde(rename = "AE")]
    #[display("AE")]
    Ae,
    #[serde(rename = "GB")]
    #[display("GB")]
    Gb,
    #[serde(rename = "US")]
    #[display("US")]
    Us,
    #[serde(rename = "UM")]
    #[display("UM")]
    Um,
    #[serde(rename = "UY")]
    #[display("UY")]
    Uy,
    #[serde(rename = "UZ")]
    #[display("UZ")]
    Uz,
    #[serde(rename = "VU")]
    #[display("VU")]
    Vu,
    #[serde(rename = "VE")]
    #[display("VE")]
    Ve,
    #[serde(rename = "VN")]
    #[display("VN")]
    Vn,
    #[serde(rename = "VG")]
    #[display("VG")]
    Vg,
    #[serde(rename = "VI")]
    #[display("VI")]
    Vi,
    #[serde(rename = "WF")]
    #[display("WF")]
    Wf,
    #[serde(rename = "EH")]
    #[display("EH")]
    Eh,
    #[serde(rename = "YE")]
    #[display("YE")]
    Ye,
    #[serde(rename = "ZM")]
    #[display("ZM")]
    Zm,
    #[serde(rename = "ZW")]
    #[display("ZW")]
    Zw,
}

#[doc = "Country."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Country {
    #[doc = "The code of the country."]
    pub code: Code,
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
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.code).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["code".into()]
    }
}

#[doc = "Currency."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Currency {
    #[doc = "The currency type, ex: USD, EUR, etc."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    #[doc = "The decimal amount for the currency."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Currency {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(currency_type) = &self.currency_type {
                format!("{:?}", currency_type).into()
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
        vec!["currency_type".into(), "value".into()]
    }
}

#[doc = "The data type of the custom field."]
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
    #[serde(rename = "RANGE")]
    #[display("RANGE")]
    Range,
    #[serde(rename = "REFERENCE_ID")]
    #[display("REFERENCE_ID")]
    ReferenceId,
    #[serde(rename = "BOOLEAN")]
    #[display("BOOLEAN")]
    Boolean,
    #[serde(rename = "ADDRESS")]
    #[display("ADDRESS")]
    Address,
    #[serde(rename = "OG_REFERENCE_FIELD")]
    #[display("OG_REFERENCE_FIELD")]
    OgReferenceField,
    #[serde(rename = "NATIVE_EDGE")]
    #[display("NATIVE_EDGE")]
    NativeEdge,
    #[serde(rename = "DATETIME")]
    #[display("DATETIME")]
    Datetime,
    #[serde(rename = "EMAIL")]
    #[display("EMAIL")]
    Email,
    #[serde(rename = "URL")]
    #[display("URL")]
    Url,
}

#[doc = "CustomField."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomField {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the custom field."]
    pub name: String,
    #[doc = "The description of the custom field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the custom field is required."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "The data type of the custom field."]
    #[serde(rename = "type")]
    pub type_: CustomFieldType,
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
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(required) = &self.required {
                format!("{:?}", required).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.type_).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "description".into(),
            "required".into(),
            "type_".into(),
        ]
    }
}

#[doc = "CustomObject."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomObject {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the custom object"]
    pub name: String,
    #[doc = "The description of the custom object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The api name of the custom object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    #[doc = "The plural label of the custom object"]
    pub plural_label: String,
    #[doc = "The category of the custom object"]
    pub category_id: String,
    #[doc = "The native category of the custom object if belongs to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_category_id: Option<String>,
    #[doc = "The id of the package which the custom object belongs to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_package_install_id: Option<String>,
    #[doc = "Whether to record the history of the custom object"]
    pub enable_history: bool,
    #[doc = "The id of the owner for the custom object"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

impl std::fmt::Display for CustomObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomObject {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(api_name) = &self.api_name {
                format!("{:?}", api_name).into()
            } else {
                String::new().into()
            },
            self.plural_label.clone().into(),
            self.category_id.clone().into(),
            if let Some(native_category_id) = &self.native_category_id {
                format!("{:?}", native_category_id).into()
            } else {
                String::new().into()
            },
            if let Some(managed_package_install_id) = &self.managed_package_install_id {
                format!("{:?}", managed_package_install_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.enable_history).into(),
            if let Some(owner_id) = &self.owner_id {
                format!("{:?}", owner_id).into()
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
            "name".into(),
            "description".into(),
            "api_name".into(),
            "plural_label".into(),
            "category_id".into(),
            "native_category_id".into(),
            "managed_package_install_id".into(),
            "enable_history".into(),
            "owner_id".into(),
        ]
    }
}

#[doc = "CustomObjectDataRow."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomObjectDataRow {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the custom object datarow"]
    pub name: String,
    #[doc = "The external id of the custom object datarow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "The owner id of the custom object datarow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role_id: Option<String>,
    #[doc = "The id of the role who created the custom object datarow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[doc = "The id of the role who made changes to the custom object datarow lastly"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
}

impl std::fmt::Display for CustomObjectDataRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomObjectDataRow {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role_id) = &self.owner_role_id {
                format!("{:?}", owner_role_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
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
            "name".into(),
            "external_id".into(),
            "owner_role_id".into(),
            "created_by".into(),
            "last_modified_by".into(),
        ]
    }
}

#[doc = "CustomObjectField."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CustomObjectField {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the custom object field"]
    pub name: String,
    #[doc = "The custom object which the field belongs to"]
    pub custom_object: String,
    #[doc = "The description of the custom object field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The api name of the custom object field"]
    pub api_name: String,
    #[doc = "This field specifies whether a particular column value has unique values"]
    pub is_unique: bool,
    #[doc = "whether the field is imuatable"]
    pub is_immutable: bool,
    #[doc = "whether the field is standard field"]
    pub is_standard: bool,
    #[doc = "The id of the package which the custom object field belongs to"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_package_install_id: Option<String>,
    #[doc = "whether the history is enable for the field"]
    pub enable_history: bool,
}

impl std::fmt::Display for CustomObjectField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CustomObjectField {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            self.custom_object.clone().into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            self.api_name.clone().into(),
            format!("{:?}", self.is_unique).into(),
            format!("{:?}", self.is_immutable).into(),
            format!("{:?}", self.is_standard).into(),
            if let Some(managed_package_install_id) = &self.managed_package_install_id {
                format!("{:?}", managed_package_install_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.enable_history).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "custom_object".into(),
            "description".into(),
            "api_name".into(),
            "is_unique".into(),
            "is_immutable".into(),
            "is_standard".into(),
            "managed_package_install_id".into(),
            "enable_history".into(),
        ]
    }
}

#[doc = "DayOff."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct DayOff {
    #[doc = "The date of the day off."]
    pub date: String,
    #[doc = "The number of minutes taken off for the day."]
    pub number_of_minutes_taken_off: f64,
}

impl std::fmt::Display for DayOff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for DayOff {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.date.clone().into(),
            format!("{:?}", self.number_of_minutes_taken_off).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["date".into(), "number_of_minutes_taken_off".into()]
    }
}

#[doc = "Department."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Department {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the department."]
    pub name: String,
    #[doc = "The parent department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent department.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<Department>>,
    #[doc = "Reference code of the department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_code: Option<String>,
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
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            if let Some(reference_code) = &self.reference_code {
                format!("{:?}", reference_code).into()
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
            "name".into(),
            "parent_id".into(),
            "parent".into(),
            "reference_code".into(),
        ]
    }
}

#[doc = "The classification of the email."]
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
pub enum EmailType {
    #[serde(rename = "HOME")]
    #[display("HOME")]
    Home,
    #[serde(rename = "WORK")]
    #[display("WORK")]
    Work,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
}

#[doc = "Email."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Email {
    #[doc = "A valid email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The classification of the email."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<EmailType>,
    #[doc = "The display value of the email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

impl std::fmt::Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Email {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(value) = &self.value {
                format!("{:?}", value).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(display) = &self.display {
                format!("{:?}", display).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["value".into(), "type_".into(), "display".into()]
    }
}

#[doc = "EntitlementModel."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct EntitlementModel {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Description of the entitlement"]
    pub description: String,
    #[doc = "Display name of the entitlement"]
    pub display_name: String,
}

impl std::fmt::Display for EntitlementModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for EntitlementModel {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.description.clone().into(),
            self.display_name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["id".into(), "description".into(), "display_name".into()]
    }
}

#[doc = "JobCode."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobCode {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the job dimension."]
    pub name: String,
    #[doc = "The ID of the job dimension this job code belongs to."]
    pub job_dimension_id: String,
    #[doc = "The job dimension this job code belongs to.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_dimension: Option<JobDimension>,
    #[doc = "The unique identifier of the job code in an outside system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "The ID of the job roster group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl std::fmt::Display for JobCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobCode {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            self.job_dimension_id.clone().into(),
            if let Some(job_dimension) = &self.job_dimension {
                format!("{:?}", job_dimension).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
            if let Some(group_id) = &self.group_id {
                format!("{:?}", group_id).into()
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
            "name".into(),
            "job_dimension_id".into(),
            "job_dimension".into(),
            "external_id".into(),
            "group_id".into(),
        ]
    }
}

#[doc = "JobCodeRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobCodeRequest {
    #[doc = "The name of the job dimension."]
    pub name: String,
    #[doc = "The ID of the job dimension this job code belongs to."]
    pub job_dimension_id: String,
    #[doc = "The unique identifier of the job code in an outside system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl std::fmt::Display for JobCodeRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobCodeRequest {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            self.job_dimension_id.clone().into(),
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "job_dimension_id".into(),
            "external_id".into(),
        ]
    }
}

#[doc = "JobCodeSummary."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobCodeSummary {
    #[doc = "List of job code ids that this summary is tracking hours for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_codes_id: Option<Vec<String>>,
    #[doc = "The total hours worked for the job codes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours_worked: Option<f64>,
}

impl std::fmt::Display for JobCodeSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobCodeSummary {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(job_codes_id) = &self.job_codes_id {
                format!("{:?}", job_codes_id).into()
            } else {
                String::new().into()
            },
            if let Some(hours_worked) = &self.hours_worked {
                format!("{:?}", hours_worked).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["job_codes_id".into(), "hours_worked".into()]
    }
}

#[doc = "JobDimension."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobDimension {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the job dimension"]
    pub name: String,
    #[doc = "The unique identifier of the job dimension in a third party system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl std::fmt::Display for JobDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobDimension {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
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
            "name".into(),
            "external_id".into(),
        ]
    }
}

#[doc = "JobDimensionRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobDimensionRequest {
    #[doc = "The name of the job dimension"]
    pub name: String,
    #[doc = "The unique identifier of the job dimension in a third party system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl std::fmt::Display for JobDimensionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobDimensionRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.name.clone().into(),
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "external_id".into()]
    }
}

#[doc = "Job requisition status"]
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
pub enum JobRequisitionStatus {
    #[serde(rename = "OPEN")]
    #[display("OPEN")]
    Open,
    #[serde(rename = "CLOSED")]
    #[display("CLOSED")]
    Closed,
    #[serde(rename = "PUBLISHED")]
    #[display("PUBLISHED")]
    Published,
    #[serde(rename = "DRAFT")]
    #[display("DRAFT")]
    Draft,
    #[serde(rename = "ARCHIVED")]
    #[display("ARCHIVED")]
    Archived,
}

#[doc = "JobRequisition."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobRequisition {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "Job requisition name"]
    pub name: String,
    #[doc = "Job requisition status"]
    pub status: JobRequisitionStatus,
}

impl std::fmt::Display for JobRequisition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobRequisition {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            format!("{:?}", self.status).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "status".into(),
        ]
    }
}

#[doc = "JobShift."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobShift {
    #[doc = "The start time of the job shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the job shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The original start time of the job shift. If the startTime field has been rounded \
             then this contain the start time before the rounding occured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_time: Option<String>,
    #[doc = "The original end time of the job shift. If the endTime field has been rounded then \
             this contain the end time before the rounding occured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_end_time: Option<String>,
    #[doc = "The IDs of the job codes associated with the job shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_codes_id: Option<Vec<String>>,
    #[doc = "Whether the job shift was entered as a duration in hours table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hours_only_input: Option<bool>,
}

impl std::fmt::Display for JobShift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobShift {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(original_start_time) = &self.original_start_time {
                format!("{:?}", original_start_time).into()
            } else {
                String::new().into()
            },
            if let Some(original_end_time) = &self.original_end_time {
                format!("{:?}", original_end_time).into()
            } else {
                String::new().into()
            },
            if let Some(job_codes_id) = &self.job_codes_id {
                format!("{:?}", job_codes_id).into()
            } else {
                String::new().into()
            },
            if let Some(is_hours_only_input) = &self.is_hours_only_input {
                format!("{:?}", is_hours_only_input).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_time".into(),
            "end_time".into(),
            "original_start_time".into(),
            "original_end_time".into(),
            "job_codes_id".into(),
            "is_hours_only_input".into(),
        ]
    }
}

#[doc = "JobShiftRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct JobShiftRequest {
    #[doc = "The start time of the job shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the job shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The duration of the job shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[doc = "The date of the job shift if using duration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "The original start time of the job shift. If the startTime field has been rounded \
             then this contain the start time before the rounding occured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_start_time: Option<String>,
    #[doc = "The original end time of the job shift. If the endTime field has been rounded then \
             this contain the end time before the rounding occured."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_end_time: Option<String>,
    #[doc = "The IDs of the job codes associated with the job shift."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_codes_id: Option<Vec<String>>,
    #[doc = "Whether the job shift was entered as a duration in hours table"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hours_only_input: Option<bool>,
}

impl std::fmt::Display for JobShiftRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for JobShiftRequest {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(duration) = &self.duration {
                format!("{:?}", duration).into()
            } else {
                String::new().into()
            },
            if let Some(start_date) = &self.start_date {
                format!("{:?}", start_date).into()
            } else {
                String::new().into()
            },
            if let Some(original_start_time) = &self.original_start_time {
                format!("{:?}", original_start_time).into()
            } else {
                String::new().into()
            },
            if let Some(original_end_time) = &self.original_end_time {
                format!("{:?}", original_end_time).into()
            } else {
                String::new().into()
            },
            if let Some(job_codes_id) = &self.job_codes_id {
                format!("{:?}", job_codes_id).into()
            } else {
                String::new().into()
            },
            if let Some(is_hours_only_input) = &self.is_hours_only_input {
                format!("{:?}", is_hours_only_input).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_time".into(),
            "end_time".into(),
            "duration".into(),
            "start_date".into(),
            "original_start_time".into(),
            "original_end_time".into(),
            "job_codes_id".into(),
            "is_hours_only_input".into(),
        ]
    }
}

#[doc = "LeaveBalance."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LeaveBalance {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the leave balance."]
    pub worker_id: String,
    #[doc = "The worker associated with the leave balance.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The ID of the leave type associated with the leave balance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[doc = "The leave type associated with the leave balance.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type: Option<LeaveType>,
    #[doc = "Indicates if the leave balance is unlimited."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_balance_unlimited: Option<bool>,
    #[doc = "The worker's leave balance including future leave requests. If the leave balance is \
             unlimited, this field will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_including_future_requests: Option<f64>,
    #[doc = "The worker's leave balance excluding future leave requests. If the leave balance is \
             unlimited, this field will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_excluding_future_requests: Option<f64>,
}

impl std::fmt::Display for LeaveBalance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LeaveBalance {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type_id) = &self.leave_type_id {
                format!("{:?}", leave_type_id).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type) = &self.leave_type {
                format!("{:?}", leave_type).into()
            } else {
                String::new().into()
            },
            if let Some(is_balance_unlimited) = &self.is_balance_unlimited {
                format!("{:?}", is_balance_unlimited).into()
            } else {
                String::new().into()
            },
            if let Some(balance_including_future_requests) = &self.balance_including_future_requests
            {
                format!("{:?}", balance_including_future_requests).into()
            } else {
                String::new().into()
            },
            if let Some(balance_excluding_future_requests) = &self.balance_excluding_future_requests
            {
                format!("{:?}", balance_excluding_future_requests).into()
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
            "worker_id".into(),
            "worker".into(),
            "leave_type_id".into(),
            "leave_type".into(),
            "is_balance_unlimited".into(),
            "balance_including_future_requests".into(),
            "balance_excluding_future_requests".into(),
        ]
    }
}

#[doc = "The status of the leave request."]
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
pub enum LeaveRequestStatus {
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

#[doc = "LeaveRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LeaveRequest {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the leave request."]
    pub worker_id: String,
    #[doc = "The worker associated with the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The ID of the worker who requested the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester_id: Option<String>,
    #[doc = "The worker who requested the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester: Option<Worker>,
    #[doc = "The status of the leave request."]
    pub status: LeaveRequestStatus,
    #[doc = "The start date of the leave request."]
    pub start_date: String,
    #[doc = "The start time of the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end date of the leave request."]
    pub end_date: String,
    #[doc = "The end time of the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The comments associated with the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "The number of minutes requested for the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_minutes_requested: Option<f64>,
    #[doc = "The ID of the leave policy associated with the leave request."]
    pub leave_policy_id: String,
    #[doc = "The ID of the leave type associated with the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[doc = "The leave type associated with the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type: Option<LeaveType>,
    #[doc = "The reason for the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_leave: Option<String>,
    #[doc = "The ID of the worker who reviewed the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<String>,
    #[doc = "The worker who reviewed the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<Worker>,
    #[doc = "The timestamp the leave request was reviewed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[doc = "The specific dates taken off and the amount of time taken off for each one."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_take_off: Option<Vec<DayOff>>,
    #[doc = "Whether the leave request is managed by an external system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_managed_by_external_system: Option<bool>,
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
    const LENGTH: usize = 23;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(requester_id) = &self.requester_id {
                format!("{:?}", requester_id).into()
            } else {
                String::new().into()
            },
            if let Some(requester) = &self.requester {
                format!("{:?}", requester).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            self.start_date.clone().into(),
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            self.end_date.clone().into(),
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(comments) = &self.comments {
                format!("{:?}", comments).into()
            } else {
                String::new().into()
            },
            if let Some(number_of_minutes_requested) = &self.number_of_minutes_requested {
                format!("{:?}", number_of_minutes_requested).into()
            } else {
                String::new().into()
            },
            self.leave_policy_id.clone().into(),
            if let Some(leave_type_id) = &self.leave_type_id {
                format!("{:?}", leave_type_id).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type) = &self.leave_type {
                format!("{:?}", leave_type).into()
            } else {
                String::new().into()
            },
            if let Some(reason_for_leave) = &self.reason_for_leave {
                format!("{:?}", reason_for_leave).into()
            } else {
                String::new().into()
            },
            if let Some(reviewer_id) = &self.reviewer_id {
                format!("{:?}", reviewer_id).into()
            } else {
                String::new().into()
            },
            if let Some(reviewer) = &self.reviewer {
                format!("{:?}", reviewer).into()
            } else {
                String::new().into()
            },
            if let Some(reviewed_at) = &self.reviewed_at {
                format!("{:?}", reviewed_at).into()
            } else {
                String::new().into()
            },
            if let Some(days_take_off) = &self.days_take_off {
                format!("{:?}", days_take_off).into()
            } else {
                String::new().into()
            },
            if let Some(is_managed_by_external_system) = &self.is_managed_by_external_system {
                format!("{:?}", is_managed_by_external_system).into()
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
            "worker_id".into(),
            "worker".into(),
            "requester_id".into(),
            "requester".into(),
            "status".into(),
            "start_date".into(),
            "start_time".into(),
            "end_date".into(),
            "end_time".into(),
            "comments".into(),
            "number_of_minutes_requested".into(),
            "leave_policy_id".into(),
            "leave_type_id".into(),
            "leave_type".into(),
            "reason_for_leave".into(),
            "reviewer_id".into(),
            "reviewer".into(),
            "reviewed_at".into(),
            "days_take_off".into(),
            "is_managed_by_external_system".into(),
        ]
    }
}

#[doc = "The status of the leave request."]
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
pub enum LeaveRequestRequestStatus {
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

#[doc = "LeaveRequestRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LeaveRequestRequest {
    #[doc = "The ID of the worker associated with the leave request."]
    pub worker_id: String,
    #[doc = "The ID of the worker who requested the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester_id: Option<String>,
    #[doc = "The status of the leave request."]
    pub status: LeaveRequestRequestStatus,
    #[doc = "The start date of the leave request."]
    pub start_date: String,
    #[doc = "The start time of the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end date of the leave request."]
    pub end_date: String,
    #[doc = "The end time of the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The number of hours to take off on the start date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date_custom_hours: Option<f64>,
    #[doc = "The number of hours to take off on the end date."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date_custom_hours: Option<f64>,
    #[doc = "The comments associated with the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "The ID of the leave policy associated with the leave request."]
    pub leave_policy_id: String,
    #[doc = "The ID of the leave type associated with the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[doc = "The reason for the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_leave: Option<String>,
    #[doc = "The ID of the worker who reviewed the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<String>,
    #[doc = "The timestamp the leave request was reviewed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
}

impl std::fmt::Display for LeaveRequestRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LeaveRequestRequest {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.worker_id.clone().into(),
            if let Some(requester_id) = &self.requester_id {
                format!("{:?}", requester_id).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            self.start_date.clone().into(),
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            self.end_date.clone().into(),
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
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
            self.leave_policy_id.clone().into(),
            if let Some(leave_type_id) = &self.leave_type_id {
                format!("{:?}", leave_type_id).into()
            } else {
                String::new().into()
            },
            if let Some(reason_for_leave) = &self.reason_for_leave {
                format!("{:?}", reason_for_leave).into()
            } else {
                String::new().into()
            },
            if let Some(reviewer_id) = &self.reviewer_id {
                format!("{:?}", reviewer_id).into()
            } else {
                String::new().into()
            },
            if let Some(reviewed_at) = &self.reviewed_at {
                format!("{:?}", reviewed_at).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "worker_id".into(),
            "requester_id".into(),
            "status".into(),
            "start_date".into(),
            "start_time".into(),
            "end_date".into(),
            "end_time".into(),
            "start_date_custom_hours".into(),
            "end_date_custom_hours".into(),
            "comments".into(),
            "leave_policy_id".into(),
            "leave_type_id".into(),
            "reason_for_leave".into(),
            "reviewer_id".into(),
            "reviewed_at".into(),
        ]
    }
}

#[doc = "LeaveType."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LeaveType {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The type of leave."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The name of the leave type."]
    pub name: String,
    #[doc = "The description of the leave type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the leave is paid."]
    pub is_paid: bool,
    #[doc = "Whether the leave is managed by an external system."]
    pub is_managed_by_external_system: bool,
}

impl std::fmt::Display for LeaveType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LeaveType {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.type_.clone().into(),
            self.name.clone().into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_paid).into(),
            format!("{:?}", self.is_managed_by_external_system).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "type_".into(),
            "name".into(),
            "description".into(),
            "is_paid".into(),
            "is_managed_by_external_system".into(),
        ]
    }
}

#[doc = "The legal entity's level in a hierarchy. * `PARENT`: The legal entity is considered the \
         ultimate holding entity. * `SUBSIDIARY`: The legal entity is considered a subsidiary, \
         fully or partially held by another. * `BRANCH`: The legal entity is considered a branch, \
         associated with a parent legal entity."]
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
pub enum EntityLevel {
    #[serde(rename = "PARENT")]
    #[display("PARENT")]
    Parent,
    #[serde(rename = "SUBSIDIARY")]
    #[display("SUBSIDIARY")]
    Subsidiary,
    #[serde(rename = "BRANCH")]
    #[display("BRANCH")]
    Branch,
}

#[doc = "The legal entity management type in the case of an employer of record (EOR) or \
         professional employment organization (PEO). * `PEO`: The legal entity is considered a \
         Professional Employment Organization (PEO). * `EOR`: The legal entity is considered an \
         Employer of Record (EOR)."]
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
pub enum ManagementType {
    #[serde(rename = "PEO")]
    #[display("PEO")]
    Peo,
    #[serde(rename = "EOR")]
    #[display("EOR")]
    Eor,
}

#[doc = "LegalEntity."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct LegalEntity {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The tax identifier for the legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_identifier: Option<String>,
    #[doc = "The country the legal entity is based in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[doc = "The legal name of the legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[doc = "The legal entity's level in a hierarchy. * `PARENT`: The legal entity is considered \
             the ultimate holding entity. * `SUBSIDIARY`: The legal entity is considered a \
             subsidiary, fully or partially held by another. * `BRANCH`: The legal entity is \
             considered a branch, associated with a parent legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_level: Option<EntityLevel>,
    #[doc = "The registration date of the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[doc = "The mailing address of the legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<Address>,
    #[doc = "The physical address of the legal entity, if it differs from the mailing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_address: Option<Address>,
    #[doc = "The parent legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent legal entity.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<LegalEntity>>,
    #[doc = "The legal entity management type in the case of an employer of record (EOR) or \
             professional employment organization (PEO). * `PEO`: The legal entity is considered \
             a Professional Employment Organization (PEO). * `EOR`: The legal entity is \
             considered an Employer of Record (EOR)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management_type: Option<ManagementType>,
    #[doc = "The company or organization associated with the legal entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[doc = "The company or organization associated with the legal entity\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
}

impl std::fmt::Display for LegalEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for LegalEntity {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(tax_identifier) = &self.tax_identifier {
                format!("{:?}", tax_identifier).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
            if let Some(entity_level) = &self.entity_level {
                format!("{:?}", entity_level).into()
            } else {
                String::new().into()
            },
            if let Some(registration_date) = &self.registration_date {
                format!("{:?}", registration_date).into()
            } else {
                String::new().into()
            },
            if let Some(mailing_address) = &self.mailing_address {
                format!("{:?}", mailing_address).into()
            } else {
                String::new().into()
            },
            if let Some(physical_address) = &self.physical_address {
                format!("{:?}", physical_address).into()
            } else {
                String::new().into()
            },
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            if let Some(management_type) = &self.management_type {
                format!("{:?}", management_type).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
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
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "tax_identifier".into(),
            "country".into(),
            "legal_name".into(),
            "entity_level".into(),
            "registration_date".into(),
            "mailing_address".into(),
            "physical_address".into(),
            "parent_id".into(),
            "parent".into(),
            "management_type".into(),
            "company_id".into(),
            "company".into(),
        ]
    }
}

#[doc = "Level."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Level {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the level. Must be unique within the company or organization."]
    pub name: String,
    #[doc = "The parent level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent level.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<Level>>,
    #[doc = "Global level is used to track the seniority of levels. The higher up a level is \
             placed on the page, the more senior and higher-ranked the level. Global level is \
             used in workflows, policies, and reports that use the level attribute (e.g., you can \
             use Level Lookup to set up a workflow that notifies the nearest person in an \
             worker's management chain at or above the specified level)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_level: Option<i64>,
    #[doc = "The description of the level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The rank of the level within its track."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    #[doc = "The track associated with the level, if it's not a global level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_id: Option<String>,
    #[doc = "The track associated with the level, if it's not a global level.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track: Option<Track>,
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
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            if let Some(global_level) = &self.global_level {
                format!("{:?}", global_level).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(rank) = &self.rank {
                format!("{:?}", rank).into()
            } else {
                String::new().into()
            },
            if let Some(track_id) = &self.track_id {
                format!("{:?}", track_id).into()
            } else {
                String::new().into()
            },
            if let Some(track) = &self.track {
                format!("{:?}", track).into()
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
            "name".into(),
            "parent_id".into(),
            "parent".into(),
            "global_level".into(),
            "description".into(),
            "rank".into(),
            "track_id".into(),
            "track".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Meta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redacted_fields: Option<Vec<RedactedField>>,
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
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(redacted_fields) = &self.redacted_fields {
            format!("{:?}", redacted_fields).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["redacted_fields".into()]
    }
}

#[doc = "Meta information for the response."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct MetaResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

impl std::fmt::Display for MetaResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for MetaResponse {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(meta) = &self.meta {
            format!("{:?}", meta).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into()]
    }
}

#[doc = "ObjectCategory."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ObjectCategory {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the Custom Category"]
    pub name: String,
    #[doc = "The description of the Custom Category"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl std::fmt::Display for ObjectCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ObjectCategory {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
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
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "description".into(),
        ]
    }
}

#[doc = "PayPeriod."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayPeriod {
    #[doc = "The start date of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "The end date of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The ID of the pay schedule associated with the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_schedule_id: Option<String>,
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
    const LENGTH: usize = 3;
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_date".into(),
            "end_date".into(),
            "pay_schedule_id".into(),
        ]
    }
}

#[doc = "PayPeriodRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PayPeriodRequest {
    #[doc = "The start date of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "The end date of the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The ID of the pay schedule associated with the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_schedule_id: Option<String>,
}

impl std::fmt::Display for PayPeriodRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PayPeriodRequest {
    const LENGTH: usize = 3;
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_date".into(),
            "end_date".into(),
            "pay_schedule_id".into(),
        ]
    }
}

#[doc = "PieceRatePremiums."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PieceRatePremiums {
    #[doc = "The pay rate for this piece rate premium."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_rate: Option<f64>,
    #[doc = "The total units produced at the premium rate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_units: Option<f64>,
}

impl std::fmt::Display for PieceRatePremiums {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PieceRatePremiums {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(premium_rate) = &self.premium_rate {
                format!("{:?}", premium_rate).into()
            } else {
                String::new().into()
            },
            if let Some(premium_units) = &self.premium_units {
                format!("{:?}", premium_units).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["premium_rate".into(), "premium_units".into()]
    }
}

#[doc = "Premiums."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Premiums {
    #[doc = "The pay rate for this premium."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_rate: Option<f64>,
    #[doc = "The total hours worked for at the premium rate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_hours: Option<f64>,
}

impl std::fmt::Display for Premiums {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Premiums {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(premium_rate) = &self.premium_rate {
                format!("{:?}", premium_rate).into()
            } else {
                String::new().into()
            },
            if let Some(premium_hours) = &self.premium_hours {
                format!("{:?}", premium_hours).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["premium_rate".into(), "premium_hours".into()]
    }
}

#[doc = "Prototype."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Prototype {}

impl std::fmt::Display for Prototype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Prototype {
    const LENGTH: usize = 0;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![]
    }
}

#[doc = "PrototypeJob."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PrototypeJob {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The worker's ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prototype_id: Option<String>,
    #[doc = "Job title"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "Work location for the job\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location: Option<PrototypeWorkLocation>,
}

impl std::fmt::Display for PrototypeJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PrototypeJob {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(prototype_id) = &self.prototype_id {
                format!("{:?}", prototype_id).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(work_location) = &self.work_location {
                format!("{:?}", work_location).into()
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
            "prototype_id".into(),
            "title".into(),
            "work_location".into(),
        ]
    }
}

#[doc = "PrototypeWorkLocation."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct PrototypeWorkLocation {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "Address for the work location"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "Whether the work location is remote"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_remote: Option<bool>,
}

impl std::fmt::Display for PrototypeWorkLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for PrototypeWorkLocation {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(address) = &self.address {
                format!("{:?}", address).into()
            } else {
                String::new().into()
            },
            if let Some(is_remote) = &self.is_remote {
                format!("{:?}", is_remote).into()
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
            "address".into(),
            "is_remote".into(),
        ]
    }
}

#[doc = "Info about the redacted fields."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RedactedField {
    #[doc = "The name for the redacted field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reason for the redaction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl std::fmt::Display for RedactedField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RedactedField {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
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
        vec!["name".into(), "reason".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RedactedFieldsRedactedFields {
    #[doc = "The name for the redacted field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The reason for the redaction"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl std::fmt::Display for RedactedFieldsRedactedFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RedactedFieldsRedactedFields {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
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
        vec!["name".into(), "reason".into()]
    }
}

#[doc = "A list of redacted fields."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RedactedFields {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub redacted_fields: Option<Vec<RedactedFieldsRedactedFields>>,
}

impl std::fmt::Display for RedactedFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RedactedFields {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![if let Some(redacted_fields) = &self.redacted_fields {
            format!("{:?}", redacted_fields).into()
        } else {
            String::new().into()
        }]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["redacted_fields".into()]
    }
}

#[doc = "Ssome."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Ssome {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The user's work email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
    #[doc = "The company ID of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[doc = "The company of the user.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
}

impl std::fmt::Display for Ssome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Ssome {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(work_email) = &self.work_email {
                format!("{:?}", work_email).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
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
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "work_email".into(),
            "company_id".into(),
            "company".into(),
        ]
    }
}

#[doc = "Segments."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Segments {
    #[doc = "The start time of the segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The IDs of the job codes associated with the segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_codes_id: Option<Vec<String>>,
    #[doc = "The multiplier for overtime hours in this segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ot_multiplier: Option<f64>,
    #[doc = "Name of the final earning for the segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The ID of the break type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub break_type_id: Option<String>,
    #[doc = "The pay rate for this segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_rate: Option<f64>,
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

#[cfg(feature = "tabled")]
impl tabled::Tabled for Segments {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(job_codes_id) = &self.job_codes_id {
                format!("{:?}", job_codes_id).into()
            } else {
                String::new().into()
            },
            if let Some(ot_multiplier) = &self.ot_multiplier {
                format!("{:?}", ot_multiplier).into()
            } else {
                String::new().into()
            },
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(break_type_id) = &self.break_type_id {
                format!("{:?}", break_type_id).into()
            } else {
                String::new().into()
            },
            if let Some(pay_rate) = &self.pay_rate {
                format!("{:?}", pay_rate).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "start_time".into(),
            "end_time".into(),
            "job_codes_id".into(),
            "ot_multiplier".into(),
            "display_name".into(),
            "break_type_id".into(),
            "pay_rate".into(),
        ]
    }
}

#[doc = "ShiftInput."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftInput {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The creator id associated with the shift input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[doc = "The creator associated with the shift input.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Worker>,
    #[doc = "Name of the shift unit."]
    pub name: String,
    #[doc = "Prompt for the shift unit."]
    pub prompt: String,
    #[doc = "Type of shift unit."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Two letter string designating country code which the shift input is associated."]
    pub country_code: String,
    #[doc = "The party that manages this shift input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
}

impl std::fmt::Display for ShiftInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ShiftInput {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(creator_id) = &self.creator_id {
                format!("{:?}", creator_id).into()
            } else {
                String::new().into()
            },
            if let Some(creator) = &self.creator {
                format!("{:?}", creator).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
            self.prompt.clone().into(),
            self.type_.clone().into(),
            self.country_code.clone().into(),
            if let Some(managed_by) = &self.managed_by {
                format!("{:?}", managed_by).into()
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
            "creator_id".into(),
            "creator".into(),
            "name".into(),
            "prompt".into(),
            "type_".into(),
            "country_code".into(),
            "managed_by".into(),
        ]
    }
}

#[doc = "ShiftInputRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftInputRequest {
    #[doc = "The creator id associated with the shift input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[doc = "Name of the shift unit."]
    pub name: String,
    #[doc = "Prompt for the shift unit."]
    pub prompt: String,
    #[doc = "Type of shift unit."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Two letter string designating country code which the shift input is associated."]
    pub country_code: String,
}

impl std::fmt::Display for ShiftInputRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ShiftInputRequest {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(creator_id) = &self.creator_id {
                format!("{:?}", creator_id).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
            self.prompt.clone().into(),
            self.type_.clone().into(),
            self.country_code.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "creator_id".into(),
            "name".into(),
            "prompt".into(),
            "type_".into(),
            "country_code".into(),
        ]
    }
}

#[doc = "ShiftInputValue."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftInputValue {
    #[doc = "The id of the relevant shift input"]
    pub shift_input_id: String,
    #[doc = "The id of the role that last added/updated this input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
}

impl std::fmt::Display for ShiftInputValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ShiftInputValue {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.shift_input_id.clone().into(),
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["shift_input_id".into(), "author_id".into()]
    }
}

#[doc = "ShiftInputValueRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ShiftInputValueRequest {
    #[doc = "The id of the relevant shift input"]
    pub shift_input_id: String,
    #[doc = "The id of the role that last added/updated this input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
}

impl std::fmt::Display for ShiftInputValueRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ShiftInputValueRequest {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.shift_input_id.clone().into(),
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["shift_input_id".into(), "author_id".into()]
    }
}

#[doc = "Team."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Team {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The parent team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent team\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<Team>>,
    #[doc = "The name of the team."]
    pub name: String,
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
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "parent_id".into(),
            "parent".into(),
            "name".into(),
        ]
    }
}

#[doc = "The termination type indicates whether the termination was voluntary or involuntary."]
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
pub enum TerminationDetailsType {
    #[serde(rename = "VOLUNTARY")]
    #[display("VOLUNTARY")]
    Voluntary,
    #[serde(rename = "INVOLUNTARY")]
    #[display("INVOLUNTARY")]
    Involuntary,
    #[serde(rename = "RETIREMENT")]
    #[display("RETIREMENT")]
    Retirement,
    #[serde(rename = "DEATH")]
    #[display("DEATH")]
    Death,
    #[serde(rename = "ABANDONMENT")]
    #[display("ABANDONMENT")]
    Abandonment,
    #[serde(rename = "OFFER_DECLINED")]
    #[display("OFFER_DECLINED")]
    OfferDeclined,
    #[serde(rename = "RESCIND")]
    #[display("RESCIND")]
    Rescind,
    #[serde(rename = "RENEGE")]
    #[display("RENEGE")]
    Renege,
}

#[doc = "TerminationDetails."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TerminationDetails {
    #[doc = "The termination type indicates whether the termination was voluntary or involuntary."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<TerminationDetailsType>,
    #[doc = "This is a description that will be custom to each Rippling company."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl std::fmt::Display for TerminationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TerminationDetails {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
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
        vec!["type_".into(), "reason".into()]
    }
}

#[doc = "TimeCard."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeCard {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the time card."]
    pub worker_id: String,
    #[doc = "The worker associated with the time card.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The pay period associated with the time card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period: Option<PayPeriod>,
    #[doc = "The summary of the time card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<TimeCardSummary>,
}

impl std::fmt::Display for TimeCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeCard {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period) = &self.pay_period {
                format!("{:?}", pay_period).into()
            } else {
                String::new().into()
            },
            if let Some(summary) = &self.summary {
                format!("{:?}", summary).into()
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
            "worker_id".into(),
            "worker".into(),
            "pay_period".into(),
            "summary".into(),
        ]
    }
}

#[doc = "TimeCardSummary."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeCardSummary {
    #[doc = "The earnings for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub earnings: Option<f64>,
    #[doc = "The amount of hours worked for each job code for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hours_worked_by_job_code: Option<Vec<JobCodeSummary>>,
    #[doc = "The premiums for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premiums: Option<f64>,
    #[doc = "The approved hours for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_hours: Option<f64>,
    #[doc = "The paid hours for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paid_hours: Option<f64>,
    #[doc = "The total hours for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_hours: Option<f64>,
    #[doc = "The total paid time off hours for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_paid_time_off_hours: Option<f64>,
    #[doc = "The total holiday hours for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_holiday_hours: Option<f64>,
    #[doc = "The total unpaid time off hours for the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_unpaid_time_off_hours: Option<f64>,
    #[doc = "The total number of regular hours worked during the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_hours: Option<f64>,
    #[doc = "The total number of overtime hours worked during the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overtime_hours: Option<f64>,
    #[doc = "The total number of doubletime hours worked during the pay period."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_overtime_hours: Option<f64>,
    #[doc = "The map of time entry to unpaidBreakHours in seconds"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unpaid_break_hours_by_entry: Option<f64>,
}

impl std::fmt::Display for TimeCardSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeCardSummary {
    const LENGTH: usize = 13;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(earnings) = &self.earnings {
                format!("{:?}", earnings).into()
            } else {
                String::new().into()
            },
            if let Some(hours_worked_by_job_code) = &self.hours_worked_by_job_code {
                format!("{:?}", hours_worked_by_job_code).into()
            } else {
                String::new().into()
            },
            if let Some(premiums) = &self.premiums {
                format!("{:?}", premiums).into()
            } else {
                String::new().into()
            },
            if let Some(approved_hours) = &self.approved_hours {
                format!("{:?}", approved_hours).into()
            } else {
                String::new().into()
            },
            if let Some(paid_hours) = &self.paid_hours {
                format!("{:?}", paid_hours).into()
            } else {
                String::new().into()
            },
            if let Some(total_hours) = &self.total_hours {
                format!("{:?}", total_hours).into()
            } else {
                String::new().into()
            },
            if let Some(total_paid_time_off_hours) = &self.total_paid_time_off_hours {
                format!("{:?}", total_paid_time_off_hours).into()
            } else {
                String::new().into()
            },
            if let Some(total_holiday_hours) = &self.total_holiday_hours {
                format!("{:?}", total_holiday_hours).into()
            } else {
                String::new().into()
            },
            if let Some(total_unpaid_time_off_hours) = &self.total_unpaid_time_off_hours {
                format!("{:?}", total_unpaid_time_off_hours).into()
            } else {
                String::new().into()
            },
            if let Some(regular_hours) = &self.regular_hours {
                format!("{:?}", regular_hours).into()
            } else {
                String::new().into()
            },
            if let Some(overtime_hours) = &self.overtime_hours {
                format!("{:?}", overtime_hours).into()
            } else {
                String::new().into()
            },
            if let Some(double_overtime_hours) = &self.double_overtime_hours {
                format!("{:?}", double_overtime_hours).into()
            } else {
                String::new().into()
            },
            if let Some(unpaid_break_hours_by_entry) = &self.unpaid_break_hours_by_entry {
                format!("{:?}", unpaid_break_hours_by_entry).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "earnings".into(),
            "hours_worked_by_job_code".into(),
            "premiums".into(),
            "approved_hours".into(),
            "paid_hours".into(),
            "total_hours".into(),
            "total_paid_time_off_hours".into(),
            "total_holiday_hours".into(),
            "total_unpaid_time_off_hours".into(),
            "regular_hours".into(),
            "overtime_hours".into(),
            "double_overtime_hours".into(),
            "unpaid_break_hours_by_entry".into(),
        ]
    }
}

#[doc = "The status of the time entry."]
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
pub enum TimeEntryStatus {
    #[serde(rename = "DRAFT")]
    #[display("DRAFT")]
    Draft,
    #[serde(rename = "APPROVED")]
    #[display("APPROVED")]
    Approved,
    #[serde(rename = "PAID")]
    #[display("PAID")]
    Paid,
    #[serde(rename = "FINALIZED")]
    #[display("FINALIZED")]
    Finalized,
}

#[doc = "TimeEntry."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeEntry {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the time entry."]
    pub worker_id: String,
    #[doc = "The worker associated with the time entry.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The start time of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The comments associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<TimeEntryComment>>,
    #[doc = "The job shifts worked during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_shifts: Option<Vec<JobShift>>,
    #[doc = "The breaks taken during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breaks: Option<Vec<Break>>,
    #[doc = "The premiums earned during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premiums: Option<Vec<Premiums>>,
    #[doc = "The piece-rate premiums earned during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub piece_rate_premiums: Option<Vec<PieceRatePremiums>>,
    #[doc = "The pay rates for each segment of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segments>>,
    #[doc = "A summary of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_entry_summary: Option<TimeEntrySummary>,
    #[doc = "The ID of the time card associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_card_id: Option<String>,
    #[doc = "The time card associated with the time entry.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_card: Option<TimeCard>,
    #[doc = "The tags associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[doc = "The unique key of the time entry in an outside system. If set, no other time entry \
             with the same key can be created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[doc = "Whether the time entry should create an extra hours run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_extra_hours_run: Option<bool>,
    #[doc = "The status of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TimeEntryStatus>,
    #[doc = "The pay period associated with the time card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period: Option<PayPeriod>,
    #[doc = "Arbitrary shift inputs collected on the time entry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_input_values: Option<Vec<ShiftInputValue>>,
}

impl std::fmt::Display for TimeEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeEntry {
    const LENGTH: usize = 22;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(comments) = &self.comments {
                format!("{:?}", comments).into()
            } else {
                String::new().into()
            },
            if let Some(job_shifts) = &self.job_shifts {
                format!("{:?}", job_shifts).into()
            } else {
                String::new().into()
            },
            if let Some(breaks) = &self.breaks {
                format!("{:?}", breaks).into()
            } else {
                String::new().into()
            },
            if let Some(premiums) = &self.premiums {
                format!("{:?}", premiums).into()
            } else {
                String::new().into()
            },
            if let Some(piece_rate_premiums) = &self.piece_rate_premiums {
                format!("{:?}", piece_rate_premiums).into()
            } else {
                String::new().into()
            },
            if let Some(segments) = &self.segments {
                format!("{:?}", segments).into()
            } else {
                String::new().into()
            },
            if let Some(time_entry_summary) = &self.time_entry_summary {
                format!("{:?}", time_entry_summary).into()
            } else {
                String::new().into()
            },
            if let Some(time_card_id) = &self.time_card_id {
                format!("{:?}", time_card_id).into()
            } else {
                String::new().into()
            },
            if let Some(time_card) = &self.time_card {
                format!("{:?}", time_card).into()
            } else {
                String::new().into()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags).into()
            } else {
                String::new().into()
            },
            if let Some(idempotency_key) = &self.idempotency_key {
                format!("{:?}", idempotency_key).into()
            } else {
                String::new().into()
            },
            if let Some(create_extra_hours_run) = &self.create_extra_hours_run {
                format!("{:?}", create_extra_hours_run).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period) = &self.pay_period {
                format!("{:?}", pay_period).into()
            } else {
                String::new().into()
            },
            if let Some(shift_input_values) = &self.shift_input_values {
                format!("{:?}", shift_input_values).into()
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
            "worker_id".into(),
            "worker".into(),
            "start_time".into(),
            "end_time".into(),
            "comments".into(),
            "job_shifts".into(),
            "breaks".into(),
            "premiums".into(),
            "piece_rate_premiums".into(),
            "segments".into(),
            "time_entry_summary".into(),
            "time_card_id".into(),
            "time_card".into(),
            "tags".into(),
            "idempotency_key".into(),
            "create_extra_hours_run".into(),
            "status".into(),
            "pay_period".into(),
            "shift_input_values".into(),
        ]
    }
}

#[doc = "TimeEntryComment."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeEntryComment {
    #[doc = "The time the comment was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[doc = "The ID of the worker who made of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_id: Option<String>,
    #[doc = "The text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl std::fmt::Display for TimeEntryComment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeEntryComment {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(author_id) = &self.author_id {
                format!("{:?}", author_id).into()
            } else {
                String::new().into()
            },
            if let Some(text) = &self.text {
                format!("{:?}", text).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["created_at".into(), "author_id".into(), "text".into()]
    }
}

#[doc = "TimeEntryCommentRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeEntryCommentRequest {
    #[doc = "The text of the comment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl std::fmt::Display for TimeEntryCommentRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeEntryCommentRequest {
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

#[doc = "The status of the time entry."]
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
pub enum TimeEntryRequestStatus {
    #[serde(rename = "DRAFT")]
    #[display("DRAFT")]
    Draft,
    #[serde(rename = "APPROVED")]
    #[display("APPROVED")]
    Approved,
    #[serde(rename = "PAID")]
    #[display("PAID")]
    Paid,
    #[serde(rename = "FINALIZED")]
    #[display("FINALIZED")]
    Finalized,
}

#[doc = "TimeEntryRequest."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeEntryRequest {
    #[doc = "The ID of the worker associated with the time entry."]
    pub worker_id: String,
    #[doc = "The duration of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    #[doc = "The comments associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<TimeEntryCommentRequest>>,
    #[doc = "The job shifts worked during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_shifts: Option<Vec<JobShiftRequest>>,
    #[doc = "The breaks taken during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breaks: Option<Vec<BreakRequest>>,
    #[doc = "The tags associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[doc = "The unique key of the time entry in an outside system. If set, no other time entry \
             with the same key can be created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[doc = "Whether the time entry should create an extra hours run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_extra_hours_run: Option<bool>,
    #[doc = "The status of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<TimeEntryRequestStatus>,
    #[doc = "The pay period associated with the time card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period: Option<PayPeriodRequest>,
    #[doc = "Arbitrary shift inputs collected on the time entry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_input_values: Option<Vec<ShiftInputValueRequest>>,
}

impl std::fmt::Display for TimeEntryRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeEntryRequest {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.worker_id.clone().into(),
            if let Some(duration) = &self.duration {
                format!("{:?}", duration).into()
            } else {
                String::new().into()
            },
            if let Some(comments) = &self.comments {
                format!("{:?}", comments).into()
            } else {
                String::new().into()
            },
            if let Some(job_shifts) = &self.job_shifts {
                format!("{:?}", job_shifts).into()
            } else {
                String::new().into()
            },
            if let Some(breaks) = &self.breaks {
                format!("{:?}", breaks).into()
            } else {
                String::new().into()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags).into()
            } else {
                String::new().into()
            },
            if let Some(idempotency_key) = &self.idempotency_key {
                format!("{:?}", idempotency_key).into()
            } else {
                String::new().into()
            },
            if let Some(create_extra_hours_run) = &self.create_extra_hours_run {
                format!("{:?}", create_extra_hours_run).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period) = &self.pay_period {
                format!("{:?}", pay_period).into()
            } else {
                String::new().into()
            },
            if let Some(shift_input_values) = &self.shift_input_values {
                format!("{:?}", shift_input_values).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "worker_id".into(),
            "duration".into(),
            "comments".into(),
            "job_shifts".into(),
            "breaks".into(),
            "tags".into(),
            "idempotency_key".into(),
            "create_extra_hours_run".into(),
            "status".into(),
            "pay_period".into(),
            "shift_input_values".into(),
        ]
    }
}

#[doc = "\nDTO used to store the summary of a TimeEntry\n"]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct TimeEntrySummary {
    #[doc = "The number of overtime hours worked during this time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub over_time_hours: Option<f64>,
    #[doc = "The number of double overtime hours worked during this time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub double_over_time_hours: Option<f64>,
    #[doc = "The number of regular hours worked during this time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regular_hours: Option<f64>,
    #[doc = "The duration of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

impl std::fmt::Display for TimeEntrySummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for TimeEntrySummary {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(over_time_hours) = &self.over_time_hours {
                format!("{:?}", over_time_hours).into()
            } else {
                String::new().into()
            },
            if let Some(double_over_time_hours) = &self.double_over_time_hours {
                format!("{:?}", double_over_time_hours).into()
            } else {
                String::new().into()
            },
            if let Some(regular_hours) = &self.regular_hours {
                format!("{:?}", regular_hours).into()
            } else {
                String::new().into()
            },
            if let Some(duration) = &self.duration {
                format!("{:?}", duration).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "over_time_hours".into(),
            "double_over_time_hours".into(),
            "regular_hours".into(),
            "duration".into(),
        ]
    }
}

#[doc = "Track."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Track {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the track. Must be unique within the company or organization."]
    pub name: String,
}

impl std::fmt::Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Track {
    const LENGTH: usize = 4;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
        ]
    }
}

#[doc = "User."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct User {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "Whether the user is able to access company resources, typically when they are in \
             actively engaged with the company and not after off-boarding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "The unique identifier across Rippling used by the User for direct authentication \
             into their associated company. Globally unique."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The user's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UserName>,
    #[doc = "The display name of the user using either the concatenated preferred given and \
             family name or username depending on availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The user's email addresses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Email>>,
    #[doc = "The user's phone numbers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<UserPhoneNumber>>,
    #[doc = "The user's addresses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<UserAddress>>,
    #[doc = "The user's photos."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<UserPhoto>>,
    #[doc = "The User's preferred written or spoken language in the same format of the HTTP \
             Accept-Language header, pursuant to Section 5.3.5 of RFC7231."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[doc = "The User's default location for purposes of localization of currency, date time \
             format, or numerical representations pursuant to RFC5646."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[doc = "The User's current time zone in IANA database Olson format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
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
    const LENGTH: usize = 14;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
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
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(emails) = &self.emails {
                format!("{:?}", emails).into()
            } else {
                String::new().into()
            },
            if let Some(phone_numbers) = &self.phone_numbers {
                format!("{:?}", phone_numbers).into()
            } else {
                String::new().into()
            },
            if let Some(addresses) = &self.addresses {
                format!("{:?}", addresses).into()
            } else {
                String::new().into()
            },
            if let Some(photos) = &self.photos {
                format!("{:?}", photos).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_language) = &self.preferred_language {
                format!("{:?}", preferred_language).into()
            } else {
                String::new().into()
            },
            if let Some(locale) = &self.locale {
                format!("{:?}", locale).into()
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
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "active".into(),
            "username".into(),
            "name".into(),
            "display_name".into(),
            "emails".into(),
            "phone_numbers".into(),
            "addresses".into(),
            "photos".into(),
            "preferred_language".into(),
            "locale".into(),
            "timezone".into(),
        ]
    }
}

#[doc = "The country component, pursuant to SCIM RFC 7643 4.1.2."]
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
pub enum UserAddressCountry {
    #[serde(rename = "AF")]
    #[display("AF")]
    Af,
    #[serde(rename = "AX")]
    #[display("AX")]
    Ax,
    #[serde(rename = "AL")]
    #[display("AL")]
    Al,
    #[serde(rename = "DZ")]
    #[display("DZ")]
    Dz,
    #[serde(rename = "AS")]
    #[display("AS")]
    As,
    #[serde(rename = "AD")]
    #[display("AD")]
    Ad,
    #[serde(rename = "AO")]
    #[display("AO")]
    Ao,
    #[serde(rename = "AI")]
    #[display("AI")]
    Ai,
    #[serde(rename = "AQ")]
    #[display("AQ")]
    Aq,
    #[serde(rename = "AG")]
    #[display("AG")]
    Ag,
    #[serde(rename = "AR")]
    #[display("AR")]
    Ar,
    #[serde(rename = "AM")]
    #[display("AM")]
    Am,
    #[serde(rename = "AW")]
    #[display("AW")]
    Aw,
    #[serde(rename = "AU")]
    #[display("AU")]
    Au,
    #[serde(rename = "AT")]
    #[display("AT")]
    At,
    #[serde(rename = "AZ")]
    #[display("AZ")]
    Az,
    #[serde(rename = "BS")]
    #[display("BS")]
    Bs,
    #[serde(rename = "BH")]
    #[display("BH")]
    Bh,
    #[serde(rename = "BD")]
    #[display("BD")]
    Bd,
    #[serde(rename = "BB")]
    #[display("BB")]
    Bb,
    #[serde(rename = "BY")]
    #[display("BY")]
    By,
    #[serde(rename = "BE")]
    #[display("BE")]
    Be,
    #[serde(rename = "BZ")]
    #[display("BZ")]
    Bz,
    #[serde(rename = "BJ")]
    #[display("BJ")]
    Bj,
    #[serde(rename = "BM")]
    #[display("BM")]
    Bm,
    #[serde(rename = "BT")]
    #[display("BT")]
    Bt,
    #[serde(rename = "BO")]
    #[display("BO")]
    Bo,
    #[serde(rename = "BQ")]
    #[display("BQ")]
    Bq,
    #[serde(rename = "BA")]
    #[display("BA")]
    Ba,
    #[serde(rename = "BW")]
    #[display("BW")]
    Bw,
    #[serde(rename = "BV")]
    #[display("BV")]
    Bv,
    #[serde(rename = "BR")]
    #[display("BR")]
    Br,
    #[serde(rename = "IO")]
    #[display("IO")]
    Io,
    #[serde(rename = "BN")]
    #[display("BN")]
    Bn,
    #[serde(rename = "BG")]
    #[display("BG")]
    Bg,
    #[serde(rename = "BF")]
    #[display("BF")]
    Bf,
    #[serde(rename = "BI")]
    #[display("BI")]
    Bi,
    #[serde(rename = "CV")]
    #[display("CV")]
    Cv,
    #[serde(rename = "KH")]
    #[display("KH")]
    Kh,
    #[serde(rename = "CM")]
    #[display("CM")]
    Cm,
    #[serde(rename = "CA")]
    #[display("CA")]
    Ca,
    #[serde(rename = "KY")]
    #[display("KY")]
    Ky,
    #[serde(rename = "CF")]
    #[display("CF")]
    Cf,
    #[serde(rename = "TD")]
    #[display("TD")]
    Td,
    #[serde(rename = "CL")]
    #[display("CL")]
    Cl,
    #[serde(rename = "CN")]
    #[display("CN")]
    Cn,
    #[serde(rename = "CX")]
    #[display("CX")]
    Cx,
    #[serde(rename = "CC")]
    #[display("CC")]
    Cc,
    #[serde(rename = "CO")]
    #[display("CO")]
    Co,
    #[serde(rename = "KM")]
    #[display("KM")]
    Km,
    #[serde(rename = "CG")]
    #[display("CG")]
    Cg,
    #[serde(rename = "CD")]
    #[display("CD")]
    Cd,
    #[serde(rename = "CK")]
    #[display("CK")]
    Ck,
    #[serde(rename = "CR")]
    #[display("CR")]
    Cr,
    #[serde(rename = "CI")]
    #[display("CI")]
    Ci,
    #[serde(rename = "HR")]
    #[display("HR")]
    Hr,
    #[serde(rename = "CW")]
    #[display("CW")]
    Cw,
    #[serde(rename = "CY")]
    #[display("CY")]
    Cy,
    #[serde(rename = "CZ")]
    #[display("CZ")]
    Cz,
    #[serde(rename = "DK")]
    #[display("DK")]
    Dk,
    #[serde(rename = "DJ")]
    #[display("DJ")]
    Dj,
    #[serde(rename = "DM")]
    #[display("DM")]
    Dm,
    #[serde(rename = "DO")]
    #[display("DO")]
    Do,
    #[serde(rename = "EC")]
    #[display("EC")]
    Ec,
    #[serde(rename = "EG")]
    #[display("EG")]
    Eg,
    #[serde(rename = "SV")]
    #[display("SV")]
    Sv,
    #[serde(rename = "GQ")]
    #[display("GQ")]
    Gq,
    #[serde(rename = "ER")]
    #[display("ER")]
    Er,
    #[serde(rename = "EE")]
    #[display("EE")]
    Ee,
    #[serde(rename = "SZ")]
    #[display("SZ")]
    Sz,
    #[serde(rename = "ET")]
    #[display("ET")]
    Et,
    #[serde(rename = "FK")]
    #[display("FK")]
    Fk,
    #[serde(rename = "FO")]
    #[display("FO")]
    Fo,
    #[serde(rename = "FJ")]
    #[display("FJ")]
    Fj,
    #[serde(rename = "FI")]
    #[display("FI")]
    Fi,
    #[serde(rename = "FR")]
    #[display("FR")]
    Fr,
    #[serde(rename = "GF")]
    #[display("GF")]
    Gf,
    #[serde(rename = "PF")]
    #[display("PF")]
    Pf,
    #[serde(rename = "TF")]
    #[display("TF")]
    Tf,
    #[serde(rename = "GA")]
    #[display("GA")]
    Ga,
    #[serde(rename = "GM")]
    #[display("GM")]
    Gm,
    #[serde(rename = "GE")]
    #[display("GE")]
    Ge,
    #[serde(rename = "DE")]
    #[display("DE")]
    De,
    #[serde(rename = "GH")]
    #[display("GH")]
    Gh,
    #[serde(rename = "GI")]
    #[display("GI")]
    Gi,
    #[serde(rename = "GR")]
    #[display("GR")]
    Gr,
    #[serde(rename = "GL")]
    #[display("GL")]
    Gl,
    #[serde(rename = "GD")]
    #[display("GD")]
    Gd,
    #[serde(rename = "GP")]
    #[display("GP")]
    Gp,
    #[serde(rename = "GU")]
    #[display("GU")]
    Gu,
    #[serde(rename = "GT")]
    #[display("GT")]
    Gt,
    #[serde(rename = "GG")]
    #[display("GG")]
    Gg,
    #[serde(rename = "GN")]
    #[display("GN")]
    Gn,
    #[serde(rename = "GW")]
    #[display("GW")]
    Gw,
    #[serde(rename = "GY")]
    #[display("GY")]
    Gy,
    #[serde(rename = "HT")]
    #[display("HT")]
    Ht,
    #[serde(rename = "HM")]
    #[display("HM")]
    Hm,
    #[serde(rename = "VA")]
    #[display("VA")]
    Va,
    #[serde(rename = "HN")]
    #[display("HN")]
    Hn,
    #[serde(rename = "HK")]
    #[display("HK")]
    Hk,
    #[serde(rename = "HU")]
    #[display("HU")]
    Hu,
    #[serde(rename = "IS")]
    #[display("IS")]
    Is,
    #[serde(rename = "IN")]
    #[display("IN")]
    In,
    #[serde(rename = "ID")]
    #[display("ID")]
    Id,
    #[serde(rename = "IQ")]
    #[display("IQ")]
    Iq,
    #[serde(rename = "IE")]
    #[display("IE")]
    Ie,
    #[serde(rename = "IM")]
    #[display("IM")]
    Im,
    #[serde(rename = "IL")]
    #[display("IL")]
    Il,
    #[serde(rename = "IT")]
    #[display("IT")]
    It,
    #[serde(rename = "JM")]
    #[display("JM")]
    Jm,
    #[serde(rename = "JP")]
    #[display("JP")]
    Jp,
    #[serde(rename = "JE")]
    #[display("JE")]
    Je,
    #[serde(rename = "JO")]
    #[display("JO")]
    Jo,
    #[serde(rename = "KZ")]
    #[display("KZ")]
    Kz,
    #[serde(rename = "KE")]
    #[display("KE")]
    Ke,
    #[serde(rename = "KI")]
    #[display("KI")]
    Ki,
    #[serde(rename = "KR")]
    #[display("KR")]
    Kr,
    #[serde(rename = "XK")]
    #[display("XK")]
    Xk,
    #[serde(rename = "KW")]
    #[display("KW")]
    Kw,
    #[serde(rename = "KG")]
    #[display("KG")]
    Kg,
    #[serde(rename = "LA")]
    #[display("LA")]
    La,
    #[serde(rename = "LV")]
    #[display("LV")]
    Lv,
    #[serde(rename = "LB")]
    #[display("LB")]
    Lb,
    #[serde(rename = "LS")]
    #[display("LS")]
    Ls,
    #[serde(rename = "LR")]
    #[display("LR")]
    Lr,
    #[serde(rename = "LY")]
    #[display("LY")]
    Ly,
    #[serde(rename = "LI")]
    #[display("LI")]
    Li,
    #[serde(rename = "LT")]
    #[display("LT")]
    Lt,
    #[serde(rename = "LU")]
    #[display("LU")]
    Lu,
    #[serde(rename = "MO")]
    #[display("MO")]
    Mo,
    #[serde(rename = "MG")]
    #[display("MG")]
    Mg,
    #[serde(rename = "MW")]
    #[display("MW")]
    Mw,
    #[serde(rename = "MY")]
    #[display("MY")]
    My,
    #[serde(rename = "MV")]
    #[display("MV")]
    Mv,
    #[serde(rename = "ML")]
    #[display("ML")]
    Ml,
    #[serde(rename = "MT")]
    #[display("MT")]
    Mt,
    #[serde(rename = "MH")]
    #[display("MH")]
    Mh,
    #[serde(rename = "MQ")]
    #[display("MQ")]
    Mq,
    #[serde(rename = "MR")]
    #[display("MR")]
    Mr,
    #[serde(rename = "MU")]
    #[display("MU")]
    Mu,
    #[serde(rename = "YT")]
    #[display("YT")]
    Yt,
    #[serde(rename = "MX")]
    #[display("MX")]
    Mx,
    #[serde(rename = "FM")]
    #[display("FM")]
    Fm,
    #[serde(rename = "MD")]
    #[display("MD")]
    Md,
    #[serde(rename = "MC")]
    #[display("MC")]
    Mc,
    #[serde(rename = "MN")]
    #[display("MN")]
    Mn,
    #[serde(rename = "ME")]
    #[display("ME")]
    Me,
    #[serde(rename = "MS")]
    #[display("MS")]
    Ms,
    #[serde(rename = "MA")]
    #[display("MA")]
    Ma,
    #[serde(rename = "MZ")]
    #[display("MZ")]
    Mz,
    #[serde(rename = "MM")]
    #[display("MM")]
    Mm,
    #[serde(rename = "NA")]
    #[display("NA")]
    Na,
    #[serde(rename = "NR")]
    #[display("NR")]
    Nr,
    #[serde(rename = "NP")]
    #[display("NP")]
    Np,
    #[serde(rename = "NL")]
    #[display("NL")]
    Nl,
    #[serde(rename = "AN")]
    #[display("AN")]
    An,
    #[serde(rename = "NC")]
    #[display("NC")]
    Nc,
    #[serde(rename = "NZ")]
    #[display("NZ")]
    Nz,
    #[serde(rename = "NI")]
    #[display("NI")]
    Ni,
    #[serde(rename = "NE")]
    #[display("NE")]
    Ne,
    #[serde(rename = "NG")]
    #[display("NG")]
    Ng,
    #[serde(rename = "NU")]
    #[display("NU")]
    Nu,
    #[serde(rename = "NF")]
    #[display("NF")]
    Nf,
    #[serde(rename = "MK")]
    #[display("MK")]
    Mk,
    #[serde(rename = "MP")]
    #[display("MP")]
    Mp,
    #[serde(rename = "NO")]
    #[display("NO")]
    No,
    #[serde(rename = "OM")]
    #[display("OM")]
    Om,
    #[serde(rename = "PK")]
    #[display("PK")]
    Pk,
    #[serde(rename = "PW")]
    #[display("PW")]
    Pw,
    #[serde(rename = "PS")]
    #[display("PS")]
    Ps,
    #[serde(rename = "PA")]
    #[display("PA")]
    Pa,
    #[serde(rename = "PG")]
    #[display("PG")]
    Pg,
    #[serde(rename = "PY")]
    #[display("PY")]
    Py,
    #[serde(rename = "PE")]
    #[display("PE")]
    Pe,
    #[serde(rename = "PH")]
    #[display("PH")]
    Ph,
    #[serde(rename = "PN")]
    #[display("PN")]
    Pn,
    #[serde(rename = "PL")]
    #[display("PL")]
    Pl,
    #[serde(rename = "PT")]
    #[display("PT")]
    Pt,
    #[serde(rename = "PR")]
    #[display("PR")]
    Pr,
    #[serde(rename = "QA")]
    #[display("QA")]
    Qa,
    #[serde(rename = "RO")]
    #[display("RO")]
    Ro,
    #[serde(rename = "RU")]
    #[display("RU")]
    Ru,
    #[serde(rename = "RW")]
    #[display("RW")]
    Rw,
    #[serde(rename = "RE")]
    #[display("RE")]
    Re,
    #[serde(rename = "BL")]
    #[display("BL")]
    Bl,
    #[serde(rename = "SH")]
    #[display("SH")]
    Sh,
    #[serde(rename = "KN")]
    #[display("KN")]
    Kn,
    #[serde(rename = "LC")]
    #[display("LC")]
    Lc,
    #[serde(rename = "MF")]
    #[display("MF")]
    Mf,
    #[serde(rename = "PM")]
    #[display("PM")]
    Pm,
    #[serde(rename = "VC")]
    #[display("VC")]
    Vc,
    #[serde(rename = "WS")]
    #[display("WS")]
    Ws,
    #[serde(rename = "SM")]
    #[display("SM")]
    Sm,
    #[serde(rename = "ST")]
    #[display("ST")]
    St,
    #[serde(rename = "SA")]
    #[display("SA")]
    Sa,
    #[serde(rename = "SN")]
    #[display("SN")]
    Sn,
    #[serde(rename = "RS")]
    #[display("RS")]
    Rs,
    #[serde(rename = "SC")]
    #[display("SC")]
    Sc,
    #[serde(rename = "SL")]
    #[display("SL")]
    Sl,
    #[serde(rename = "SG")]
    #[display("SG")]
    Sg,
    #[serde(rename = "SX")]
    #[display("SX")]
    Sx,
    #[serde(rename = "SK")]
    #[display("SK")]
    Sk,
    #[serde(rename = "SI")]
    #[display("SI")]
    Si,
    #[serde(rename = "SB")]
    #[display("SB")]
    Sb,
    #[serde(rename = "SO")]
    #[display("SO")]
    So,
    #[serde(rename = "ZA")]
    #[display("ZA")]
    Za,
    #[serde(rename = "GS")]
    #[display("GS")]
    Gs,
    #[serde(rename = "SS")]
    #[display("SS")]
    Ss,
    #[serde(rename = "ES")]
    #[display("ES")]
    Es,
    #[serde(rename = "LK")]
    #[display("LK")]
    Lk,
    #[serde(rename = "SD")]
    #[display("SD")]
    Sd,
    #[serde(rename = "SR")]
    #[display("SR")]
    Sr,
    #[serde(rename = "SJ")]
    #[display("SJ")]
    Sj,
    #[serde(rename = "SE")]
    #[display("SE")]
    Se,
    #[serde(rename = "CH")]
    #[display("CH")]
    Ch,
    #[serde(rename = "TW")]
    #[display("TW")]
    Tw,
    #[serde(rename = "TJ")]
    #[display("TJ")]
    Tj,
    #[serde(rename = "TZ")]
    #[display("TZ")]
    Tz,
    #[serde(rename = "TH")]
    #[display("TH")]
    Th,
    #[serde(rename = "TL")]
    #[display("TL")]
    Tl,
    #[serde(rename = "TG")]
    #[display("TG")]
    Tg,
    #[serde(rename = "TK")]
    #[display("TK")]
    Tk,
    #[serde(rename = "TO")]
    #[display("TO")]
    To,
    #[serde(rename = "TT")]
    #[display("TT")]
    Tt,
    #[serde(rename = "TN")]
    #[display("TN")]
    Tn,
    #[serde(rename = "TR")]
    #[display("TR")]
    Tr,
    #[serde(rename = "TM")]
    #[display("TM")]
    Tm,
    #[serde(rename = "TC")]
    #[display("TC")]
    Tc,
    #[serde(rename = "TV")]
    #[display("TV")]
    Tv,
    #[serde(rename = "UG")]
    #[display("UG")]
    Ug,
    #[serde(rename = "UA")]
    #[display("UA")]
    Ua,
    #[serde(rename = "AE")]
    #[display("AE")]
    Ae,
    #[serde(rename = "GB")]
    #[display("GB")]
    Gb,
    #[serde(rename = "US")]
    #[display("US")]
    Us,
    #[serde(rename = "UM")]
    #[display("UM")]
    Um,
    #[serde(rename = "UY")]
    #[display("UY")]
    Uy,
    #[serde(rename = "UZ")]
    #[display("UZ")]
    Uz,
    #[serde(rename = "VU")]
    #[display("VU")]
    Vu,
    #[serde(rename = "VE")]
    #[display("VE")]
    Ve,
    #[serde(rename = "VN")]
    #[display("VN")]
    Vn,
    #[serde(rename = "VG")]
    #[display("VG")]
    Vg,
    #[serde(rename = "VI")]
    #[display("VI")]
    Vi,
    #[serde(rename = "WF")]
    #[display("WF")]
    Wf,
    #[serde(rename = "EH")]
    #[display("EH")]
    Eh,
    #[serde(rename = "YE")]
    #[display("YE")]
    Ye,
    #[serde(rename = "ZM")]
    #[display("ZM")]
    Zm,
    #[serde(rename = "ZW")]
    #[display("ZW")]
    Zw,
}

#[doc = "UserAddress."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserAddress {
    #[doc = "The classification of the address."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<Type>,
    #[doc = "The formatted mailing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[doc = "The full street address component, which may include house number, street name, P.O. \
             box, and multi-line extended street address information, pursuant to SCIM RFC 7643 \
             4.1.2.."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[doc = "The city or locality component."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[doc = "The state or region component, pursuant to SCIM RFC 7643 4.1.2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "The zip code or postal code component, pursuant to SCIM RFC 7643 4.1.2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "The country component, pursuant to SCIM RFC 7643 4.1.2."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<UserAddressCountry>,
}

impl std::fmt::Display for UserAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserAddress {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(formatted) = &self.formatted {
                format!("{:?}", formatted).into()
            } else {
                String::new().into()
            },
            if let Some(street_address) = &self.street_address {
                format!("{:?}", street_address).into()
            } else {
                String::new().into()
            },
            if let Some(locality) = &self.locality {
                format!("{:?}", locality).into()
            } else {
                String::new().into()
            },
            if let Some(region) = &self.region {
                format!("{:?}", region).into()
            } else {
                String::new().into()
            },
            if let Some(postal_code) = &self.postal_code {
                format!("{:?}", postal_code).into()
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
            "type_".into(),
            "formatted".into(),
            "street_address".into(),
            "locality".into(),
            "region".into(),
            "postal_code".into(),
            "country".into(),
        ]
    }
}

#[doc = "UserName."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserName {
    #[doc = "The user's full name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[doc = "The given legal name of the user, or first name in most Western languages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[doc = "The middle name(s) of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[doc = "The legal family name of the user, or last name in most Western languages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[doc = "The preferred given name, or first name in most Western languages, by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_given_name: Option<String>,
    #[doc = "The preferred family name, or last name in most Western languages, by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_family_name: Option<String>,
}

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserName {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(formatted) = &self.formatted {
                format!("{:?}", formatted).into()
            } else {
                String::new().into()
            },
            if let Some(given_name) = &self.given_name {
                format!("{:?}", given_name).into()
            } else {
                String::new().into()
            },
            if let Some(middle_name) = &self.middle_name {
                format!("{:?}", middle_name).into()
            } else {
                String::new().into()
            },
            if let Some(family_name) = &self.family_name {
                format!("{:?}", family_name).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_given_name) = &self.preferred_given_name {
                format!("{:?}", preferred_given_name).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_family_name) = &self.preferred_family_name {
                format!("{:?}", preferred_family_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "formatted".into(),
            "given_name".into(),
            "middle_name".into(),
            "family_name".into(),
            "preferred_given_name".into(),
            "preferred_family_name".into(),
        ]
    }
}

#[doc = "The classification of the phone number, pursuant to SCIM RFC 7643 4.1.2."]
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
pub enum UserPhoneNumberType {
    #[serde(rename = "HOME")]
    #[display("HOME")]
    Home,
    #[serde(rename = "WORK")]
    #[display("WORK")]
    Work,
    #[serde(rename = "MOBILE")]
    #[display("MOBILE")]
    Mobile,
    #[serde(rename = "FAX")]
    #[display("FAX")]
    Fax,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
}

#[doc = "UserPhoneNumber."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserPhoneNumber {
    #[doc = "The canonical global phone number pursuant to RFC3966."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The classification of the phone number, pursuant to SCIM RFC 7643 4.1.2."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<UserPhoneNumberType>,
    #[doc = "The display value of the phone number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

impl std::fmt::Display for UserPhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserPhoneNumber {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(value) = &self.value {
                format!("{:?}", value).into()
            } else {
                String::new().into()
            },
            if let Some(type_) = &self.type_ {
                format!("{:?}", type_).into()
            } else {
                String::new().into()
            },
            if let Some(display) = &self.display {
                format!("{:?}", display).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["value".into(), "type_".into(), "display".into()]
    }
}

#[doc = "The classification of the photo."]
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
pub enum UserPhotoType {
    #[serde(rename = "PHOTO")]
    #[display("PHOTO")]
    Photo,
    #[serde(rename = "THUMBNAIL")]
    #[display("THUMBNAIL")]
    Thumbnail,
}

#[doc = "UserPhoto."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UserPhoto {
    #[doc = "The URL of the photo."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[doc = "The classification of the photo."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<UserPhotoType>,
}

impl std::fmt::Display for UserPhoto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UserPhoto {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(value) = &self.value {
                format!("{:?}", value).into()
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
        vec!["value".into(), "type_".into()]
    }
}

#[doc = "WorkLocation."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WorkLocation {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the work location."]
    pub name: String,
    #[doc = "The address for the work location."]
    pub address: Address,
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
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            format!("{:?}", self.address).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "address".into(),
        ]
    }
}

#[doc = "The worker's country."]
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
pub enum WorkerCountry {
    #[serde(rename = "AF")]
    #[display("AF")]
    Af,
    #[serde(rename = "AX")]
    #[display("AX")]
    Ax,
    #[serde(rename = "AL")]
    #[display("AL")]
    Al,
    #[serde(rename = "DZ")]
    #[display("DZ")]
    Dz,
    #[serde(rename = "AS")]
    #[display("AS")]
    As,
    #[serde(rename = "AD")]
    #[display("AD")]
    Ad,
    #[serde(rename = "AO")]
    #[display("AO")]
    Ao,
    #[serde(rename = "AI")]
    #[display("AI")]
    Ai,
    #[serde(rename = "AQ")]
    #[display("AQ")]
    Aq,
    #[serde(rename = "AG")]
    #[display("AG")]
    Ag,
    #[serde(rename = "AR")]
    #[display("AR")]
    Ar,
    #[serde(rename = "AM")]
    #[display("AM")]
    Am,
    #[serde(rename = "AW")]
    #[display("AW")]
    Aw,
    #[serde(rename = "AU")]
    #[display("AU")]
    Au,
    #[serde(rename = "AT")]
    #[display("AT")]
    At,
    #[serde(rename = "AZ")]
    #[display("AZ")]
    Az,
    #[serde(rename = "BS")]
    #[display("BS")]
    Bs,
    #[serde(rename = "BH")]
    #[display("BH")]
    Bh,
    #[serde(rename = "BD")]
    #[display("BD")]
    Bd,
    #[serde(rename = "BB")]
    #[display("BB")]
    Bb,
    #[serde(rename = "BY")]
    #[display("BY")]
    By,
    #[serde(rename = "BE")]
    #[display("BE")]
    Be,
    #[serde(rename = "BZ")]
    #[display("BZ")]
    Bz,
    #[serde(rename = "BJ")]
    #[display("BJ")]
    Bj,
    #[serde(rename = "BM")]
    #[display("BM")]
    Bm,
    #[serde(rename = "BT")]
    #[display("BT")]
    Bt,
    #[serde(rename = "BO")]
    #[display("BO")]
    Bo,
    #[serde(rename = "BQ")]
    #[display("BQ")]
    Bq,
    #[serde(rename = "BA")]
    #[display("BA")]
    Ba,
    #[serde(rename = "BW")]
    #[display("BW")]
    Bw,
    #[serde(rename = "BV")]
    #[display("BV")]
    Bv,
    #[serde(rename = "BR")]
    #[display("BR")]
    Br,
    #[serde(rename = "IO")]
    #[display("IO")]
    Io,
    #[serde(rename = "BN")]
    #[display("BN")]
    Bn,
    #[serde(rename = "BG")]
    #[display("BG")]
    Bg,
    #[serde(rename = "BF")]
    #[display("BF")]
    Bf,
    #[serde(rename = "BI")]
    #[display("BI")]
    Bi,
    #[serde(rename = "CV")]
    #[display("CV")]
    Cv,
    #[serde(rename = "KH")]
    #[display("KH")]
    Kh,
    #[serde(rename = "CM")]
    #[display("CM")]
    Cm,
    #[serde(rename = "CA")]
    #[display("CA")]
    Ca,
    #[serde(rename = "KY")]
    #[display("KY")]
    Ky,
    #[serde(rename = "CF")]
    #[display("CF")]
    Cf,
    #[serde(rename = "TD")]
    #[display("TD")]
    Td,
    #[serde(rename = "CL")]
    #[display("CL")]
    Cl,
    #[serde(rename = "CN")]
    #[display("CN")]
    Cn,
    #[serde(rename = "CX")]
    #[display("CX")]
    Cx,
    #[serde(rename = "CC")]
    #[display("CC")]
    Cc,
    #[serde(rename = "CO")]
    #[display("CO")]
    Co,
    #[serde(rename = "KM")]
    #[display("KM")]
    Km,
    #[serde(rename = "CG")]
    #[display("CG")]
    Cg,
    #[serde(rename = "CD")]
    #[display("CD")]
    Cd,
    #[serde(rename = "CK")]
    #[display("CK")]
    Ck,
    #[serde(rename = "CR")]
    #[display("CR")]
    Cr,
    #[serde(rename = "CI")]
    #[display("CI")]
    Ci,
    #[serde(rename = "HR")]
    #[display("HR")]
    Hr,
    #[serde(rename = "CW")]
    #[display("CW")]
    Cw,
    #[serde(rename = "CY")]
    #[display("CY")]
    Cy,
    #[serde(rename = "CZ")]
    #[display("CZ")]
    Cz,
    #[serde(rename = "DK")]
    #[display("DK")]
    Dk,
    #[serde(rename = "DJ")]
    #[display("DJ")]
    Dj,
    #[serde(rename = "DM")]
    #[display("DM")]
    Dm,
    #[serde(rename = "DO")]
    #[display("DO")]
    Do,
    #[serde(rename = "EC")]
    #[display("EC")]
    Ec,
    #[serde(rename = "EG")]
    #[display("EG")]
    Eg,
    #[serde(rename = "SV")]
    #[display("SV")]
    Sv,
    #[serde(rename = "GQ")]
    #[display("GQ")]
    Gq,
    #[serde(rename = "ER")]
    #[display("ER")]
    Er,
    #[serde(rename = "EE")]
    #[display("EE")]
    Ee,
    #[serde(rename = "SZ")]
    #[display("SZ")]
    Sz,
    #[serde(rename = "ET")]
    #[display("ET")]
    Et,
    #[serde(rename = "FK")]
    #[display("FK")]
    Fk,
    #[serde(rename = "FO")]
    #[display("FO")]
    Fo,
    #[serde(rename = "FJ")]
    #[display("FJ")]
    Fj,
    #[serde(rename = "FI")]
    #[display("FI")]
    Fi,
    #[serde(rename = "FR")]
    #[display("FR")]
    Fr,
    #[serde(rename = "GF")]
    #[display("GF")]
    Gf,
    #[serde(rename = "PF")]
    #[display("PF")]
    Pf,
    #[serde(rename = "TF")]
    #[display("TF")]
    Tf,
    #[serde(rename = "GA")]
    #[display("GA")]
    Ga,
    #[serde(rename = "GM")]
    #[display("GM")]
    Gm,
    #[serde(rename = "GE")]
    #[display("GE")]
    Ge,
    #[serde(rename = "DE")]
    #[display("DE")]
    De,
    #[serde(rename = "GH")]
    #[display("GH")]
    Gh,
    #[serde(rename = "GI")]
    #[display("GI")]
    Gi,
    #[serde(rename = "GR")]
    #[display("GR")]
    Gr,
    #[serde(rename = "GL")]
    #[display("GL")]
    Gl,
    #[serde(rename = "GD")]
    #[display("GD")]
    Gd,
    #[serde(rename = "GP")]
    #[display("GP")]
    Gp,
    #[serde(rename = "GU")]
    #[display("GU")]
    Gu,
    #[serde(rename = "GT")]
    #[display("GT")]
    Gt,
    #[serde(rename = "GG")]
    #[display("GG")]
    Gg,
    #[serde(rename = "GN")]
    #[display("GN")]
    Gn,
    #[serde(rename = "GW")]
    #[display("GW")]
    Gw,
    #[serde(rename = "GY")]
    #[display("GY")]
    Gy,
    #[serde(rename = "HT")]
    #[display("HT")]
    Ht,
    #[serde(rename = "HM")]
    #[display("HM")]
    Hm,
    #[serde(rename = "VA")]
    #[display("VA")]
    Va,
    #[serde(rename = "HN")]
    #[display("HN")]
    Hn,
    #[serde(rename = "HK")]
    #[display("HK")]
    Hk,
    #[serde(rename = "HU")]
    #[display("HU")]
    Hu,
    #[serde(rename = "IS")]
    #[display("IS")]
    Is,
    #[serde(rename = "IN")]
    #[display("IN")]
    In,
    #[serde(rename = "ID")]
    #[display("ID")]
    Id,
    #[serde(rename = "IQ")]
    #[display("IQ")]
    Iq,
    #[serde(rename = "IE")]
    #[display("IE")]
    Ie,
    #[serde(rename = "IM")]
    #[display("IM")]
    Im,
    #[serde(rename = "IL")]
    #[display("IL")]
    Il,
    #[serde(rename = "IT")]
    #[display("IT")]
    It,
    #[serde(rename = "JM")]
    #[display("JM")]
    Jm,
    #[serde(rename = "JP")]
    #[display("JP")]
    Jp,
    #[serde(rename = "JE")]
    #[display("JE")]
    Je,
    #[serde(rename = "JO")]
    #[display("JO")]
    Jo,
    #[serde(rename = "KZ")]
    #[display("KZ")]
    Kz,
    #[serde(rename = "KE")]
    #[display("KE")]
    Ke,
    #[serde(rename = "KI")]
    #[display("KI")]
    Ki,
    #[serde(rename = "KR")]
    #[display("KR")]
    Kr,
    #[serde(rename = "XK")]
    #[display("XK")]
    Xk,
    #[serde(rename = "KW")]
    #[display("KW")]
    Kw,
    #[serde(rename = "KG")]
    #[display("KG")]
    Kg,
    #[serde(rename = "LA")]
    #[display("LA")]
    La,
    #[serde(rename = "LV")]
    #[display("LV")]
    Lv,
    #[serde(rename = "LB")]
    #[display("LB")]
    Lb,
    #[serde(rename = "LS")]
    #[display("LS")]
    Ls,
    #[serde(rename = "LR")]
    #[display("LR")]
    Lr,
    #[serde(rename = "LY")]
    #[display("LY")]
    Ly,
    #[serde(rename = "LI")]
    #[display("LI")]
    Li,
    #[serde(rename = "LT")]
    #[display("LT")]
    Lt,
    #[serde(rename = "LU")]
    #[display("LU")]
    Lu,
    #[serde(rename = "MO")]
    #[display("MO")]
    Mo,
    #[serde(rename = "MG")]
    #[display("MG")]
    Mg,
    #[serde(rename = "MW")]
    #[display("MW")]
    Mw,
    #[serde(rename = "MY")]
    #[display("MY")]
    My,
    #[serde(rename = "MV")]
    #[display("MV")]
    Mv,
    #[serde(rename = "ML")]
    #[display("ML")]
    Ml,
    #[serde(rename = "MT")]
    #[display("MT")]
    Mt,
    #[serde(rename = "MH")]
    #[display("MH")]
    Mh,
    #[serde(rename = "MQ")]
    #[display("MQ")]
    Mq,
    #[serde(rename = "MR")]
    #[display("MR")]
    Mr,
    #[serde(rename = "MU")]
    #[display("MU")]
    Mu,
    #[serde(rename = "YT")]
    #[display("YT")]
    Yt,
    #[serde(rename = "MX")]
    #[display("MX")]
    Mx,
    #[serde(rename = "FM")]
    #[display("FM")]
    Fm,
    #[serde(rename = "MD")]
    #[display("MD")]
    Md,
    #[serde(rename = "MC")]
    #[display("MC")]
    Mc,
    #[serde(rename = "MN")]
    #[display("MN")]
    Mn,
    #[serde(rename = "ME")]
    #[display("ME")]
    Me,
    #[serde(rename = "MS")]
    #[display("MS")]
    Ms,
    #[serde(rename = "MA")]
    #[display("MA")]
    Ma,
    #[serde(rename = "MZ")]
    #[display("MZ")]
    Mz,
    #[serde(rename = "MM")]
    #[display("MM")]
    Mm,
    #[serde(rename = "NA")]
    #[display("NA")]
    Na,
    #[serde(rename = "NR")]
    #[display("NR")]
    Nr,
    #[serde(rename = "NP")]
    #[display("NP")]
    Np,
    #[serde(rename = "NL")]
    #[display("NL")]
    Nl,
    #[serde(rename = "AN")]
    #[display("AN")]
    An,
    #[serde(rename = "NC")]
    #[display("NC")]
    Nc,
    #[serde(rename = "NZ")]
    #[display("NZ")]
    Nz,
    #[serde(rename = "NI")]
    #[display("NI")]
    Ni,
    #[serde(rename = "NE")]
    #[display("NE")]
    Ne,
    #[serde(rename = "NG")]
    #[display("NG")]
    Ng,
    #[serde(rename = "NU")]
    #[display("NU")]
    Nu,
    #[serde(rename = "NF")]
    #[display("NF")]
    Nf,
    #[serde(rename = "MK")]
    #[display("MK")]
    Mk,
    #[serde(rename = "MP")]
    #[display("MP")]
    Mp,
    #[serde(rename = "NO")]
    #[display("NO")]
    No,
    #[serde(rename = "OM")]
    #[display("OM")]
    Om,
    #[serde(rename = "PK")]
    #[display("PK")]
    Pk,
    #[serde(rename = "PW")]
    #[display("PW")]
    Pw,
    #[serde(rename = "PS")]
    #[display("PS")]
    Ps,
    #[serde(rename = "PA")]
    #[display("PA")]
    Pa,
    #[serde(rename = "PG")]
    #[display("PG")]
    Pg,
    #[serde(rename = "PY")]
    #[display("PY")]
    Py,
    #[serde(rename = "PE")]
    #[display("PE")]
    Pe,
    #[serde(rename = "PH")]
    #[display("PH")]
    Ph,
    #[serde(rename = "PN")]
    #[display("PN")]
    Pn,
    #[serde(rename = "PL")]
    #[display("PL")]
    Pl,
    #[serde(rename = "PT")]
    #[display("PT")]
    Pt,
    #[serde(rename = "PR")]
    #[display("PR")]
    Pr,
    #[serde(rename = "QA")]
    #[display("QA")]
    Qa,
    #[serde(rename = "RO")]
    #[display("RO")]
    Ro,
    #[serde(rename = "RU")]
    #[display("RU")]
    Ru,
    #[serde(rename = "RW")]
    #[display("RW")]
    Rw,
    #[serde(rename = "RE")]
    #[display("RE")]
    Re,
    #[serde(rename = "BL")]
    #[display("BL")]
    Bl,
    #[serde(rename = "SH")]
    #[display("SH")]
    Sh,
    #[serde(rename = "KN")]
    #[display("KN")]
    Kn,
    #[serde(rename = "LC")]
    #[display("LC")]
    Lc,
    #[serde(rename = "MF")]
    #[display("MF")]
    Mf,
    #[serde(rename = "PM")]
    #[display("PM")]
    Pm,
    #[serde(rename = "VC")]
    #[display("VC")]
    Vc,
    #[serde(rename = "WS")]
    #[display("WS")]
    Ws,
    #[serde(rename = "SM")]
    #[display("SM")]
    Sm,
    #[serde(rename = "ST")]
    #[display("ST")]
    St,
    #[serde(rename = "SA")]
    #[display("SA")]
    Sa,
    #[serde(rename = "SN")]
    #[display("SN")]
    Sn,
    #[serde(rename = "RS")]
    #[display("RS")]
    Rs,
    #[serde(rename = "SC")]
    #[display("SC")]
    Sc,
    #[serde(rename = "SL")]
    #[display("SL")]
    Sl,
    #[serde(rename = "SG")]
    #[display("SG")]
    Sg,
    #[serde(rename = "SX")]
    #[display("SX")]
    Sx,
    #[serde(rename = "SK")]
    #[display("SK")]
    Sk,
    #[serde(rename = "SI")]
    #[display("SI")]
    Si,
    #[serde(rename = "SB")]
    #[display("SB")]
    Sb,
    #[serde(rename = "SO")]
    #[display("SO")]
    So,
    #[serde(rename = "ZA")]
    #[display("ZA")]
    Za,
    #[serde(rename = "GS")]
    #[display("GS")]
    Gs,
    #[serde(rename = "SS")]
    #[display("SS")]
    Ss,
    #[serde(rename = "ES")]
    #[display("ES")]
    Es,
    #[serde(rename = "LK")]
    #[display("LK")]
    Lk,
    #[serde(rename = "SD")]
    #[display("SD")]
    Sd,
    #[serde(rename = "SR")]
    #[display("SR")]
    Sr,
    #[serde(rename = "SJ")]
    #[display("SJ")]
    Sj,
    #[serde(rename = "SE")]
    #[display("SE")]
    Se,
    #[serde(rename = "CH")]
    #[display("CH")]
    Ch,
    #[serde(rename = "TW")]
    #[display("TW")]
    Tw,
    #[serde(rename = "TJ")]
    #[display("TJ")]
    Tj,
    #[serde(rename = "TZ")]
    #[display("TZ")]
    Tz,
    #[serde(rename = "TH")]
    #[display("TH")]
    Th,
    #[serde(rename = "TL")]
    #[display("TL")]
    Tl,
    #[serde(rename = "TG")]
    #[display("TG")]
    Tg,
    #[serde(rename = "TK")]
    #[display("TK")]
    Tk,
    #[serde(rename = "TO")]
    #[display("TO")]
    To,
    #[serde(rename = "TT")]
    #[display("TT")]
    Tt,
    #[serde(rename = "TN")]
    #[display("TN")]
    Tn,
    #[serde(rename = "TR")]
    #[display("TR")]
    Tr,
    #[serde(rename = "TM")]
    #[display("TM")]
    Tm,
    #[serde(rename = "TC")]
    #[display("TC")]
    Tc,
    #[serde(rename = "TV")]
    #[display("TV")]
    Tv,
    #[serde(rename = "UG")]
    #[display("UG")]
    Ug,
    #[serde(rename = "UA")]
    #[display("UA")]
    Ua,
    #[serde(rename = "AE")]
    #[display("AE")]
    Ae,
    #[serde(rename = "GB")]
    #[display("GB")]
    Gb,
    #[serde(rename = "US")]
    #[display("US")]
    Us,
    #[serde(rename = "UM")]
    #[display("UM")]
    Um,
    #[serde(rename = "UY")]
    #[display("UY")]
    Uy,
    #[serde(rename = "UZ")]
    #[display("UZ")]
    Uz,
    #[serde(rename = "VU")]
    #[display("VU")]
    Vu,
    #[serde(rename = "VE")]
    #[display("VE")]
    Ve,
    #[serde(rename = "VN")]
    #[display("VN")]
    Vn,
    #[serde(rename = "VG")]
    #[display("VG")]
    Vg,
    #[serde(rename = "VI")]
    #[display("VI")]
    Vi,
    #[serde(rename = "WF")]
    #[display("WF")]
    Wf,
    #[serde(rename = "EH")]
    #[display("EH")]
    Eh,
    #[serde(rename = "YE")]
    #[display("YE")]
    Ye,
    #[serde(rename = "ZM")]
    #[display("ZM")]
    Zm,
    #[serde(rename = "ZW")]
    #[display("ZW")]
    Zw,
}

#[doc = "The worker's status within the organization."]
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
pub enum WorkerStatus {
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

#[doc = "The gender of the worker, if specified."]
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
    #[serde(rename = "NONBINARY")]
    #[display("NONBINARY")]
    Nonbinary,
    #[serde(rename = "UNDETERMINED")]
    #[display("UNDETERMINED")]
    Undetermined,
    #[serde(rename = "DIVERSE")]
    #[display("DIVERSE")]
    Diverse,
    #[serde(rename = "DOES_NOT_APPLY")]
    #[display("DOES_NOT_APPLY")]
    DoesNotApply,
}

#[doc = "The identified race of the worker, if specified."]
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
pub enum Race {
    #[serde(rename = "BLACK")]
    #[display("BLACK")]
    Black,
    #[serde(rename = "BROWN")]
    #[display("BROWN")]
    Brown,
    #[serde(rename = "CHINESE")]
    #[display("CHINESE")]
    Chinese,
    #[serde(rename = "EURASIAN")]
    #[display("EURASIAN")]
    Eurasian,
    #[serde(rename = "INDIAN")]
    #[display("INDIAN")]
    Indian,
    #[serde(rename = "INDIGENOUS")]
    #[display("INDIGENOUS")]
    Indigenous,
    #[serde(rename = "WHITE")]
    #[display("WHITE")]
    White,
    #[serde(rename = "YELLOW")]
    #[display("YELLOW")]
    Yellow,
    #[serde(rename = "NOT_INFORMED")]
    #[display("NOT_INFORMED")]
    NotInformed,
    #[serde(rename = "OTHER")]
    #[display("OTHER")]
    Other,
}

#[doc = "The identified ethnicity of the worker, if specified."]
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
pub enum Ethnicity {
    #[serde(rename = "HISPANIC_OR_LATINO")]
    #[display("HISPANIC_OR_LATINO")]
    HispanicOrLatino,
    #[serde(rename = "WHITE")]
    #[display("WHITE")]
    White,
    #[serde(rename = "BLACK_OR_AFRICAN_AMERICAN")]
    #[display("BLACK_OR_AFRICAN_AMERICAN")]
    BlackOrAfricanAmerican,
    #[serde(rename = "NATIVE_HAWAIIAN_OR_OTHER_PACIFIC_ISLANDER")]
    #[display("NATIVE_HAWAIIAN_OR_OTHER_PACIFIC_ISLANDER")]
    NativeHawaiianOrOtherPacificIslander,
    #[serde(rename = "ASIAN")]
    #[display("ASIAN")]
    Asian,
    #[serde(rename = "AMERICAN_INDIAN_OR_ALASKA_NATIVE")]
    #[display("AMERICAN_INDIAN_OR_ALASKA_NATIVE")]
    AmericanIndianOrAlaskaNative,
    #[serde(rename = "TWO_OR_MORE_RACES")]
    #[display("TWO_OR_MORE_RACES")]
    TwoOrMoreRaces,
    #[serde(rename = "DECLINE_TO_SELF_IDENTIFY")]
    #[display("DECLINE_TO_SELF_IDENTIFY")]
    DeclineToSelfIdentify,
}

#[doc = "The countries that the worker has citizenship in."]
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
pub enum Citizenship {
    #[serde(rename = "AF")]
    #[display("AF")]
    Af,
    #[serde(rename = "AX")]
    #[display("AX")]
    Ax,
    #[serde(rename = "AL")]
    #[display("AL")]
    Al,
    #[serde(rename = "DZ")]
    #[display("DZ")]
    Dz,
    #[serde(rename = "AS")]
    #[display("AS")]
    As,
    #[serde(rename = "AD")]
    #[display("AD")]
    Ad,
    #[serde(rename = "AO")]
    #[display("AO")]
    Ao,
    #[serde(rename = "AI")]
    #[display("AI")]
    Ai,
    #[serde(rename = "AQ")]
    #[display("AQ")]
    Aq,
    #[serde(rename = "AG")]
    #[display("AG")]
    Ag,
    #[serde(rename = "AR")]
    #[display("AR")]
    Ar,
    #[serde(rename = "AM")]
    #[display("AM")]
    Am,
    #[serde(rename = "AW")]
    #[display("AW")]
    Aw,
    #[serde(rename = "AU")]
    #[display("AU")]
    Au,
    #[serde(rename = "AT")]
    #[display("AT")]
    At,
    #[serde(rename = "AZ")]
    #[display("AZ")]
    Az,
    #[serde(rename = "BS")]
    #[display("BS")]
    Bs,
    #[serde(rename = "BH")]
    #[display("BH")]
    Bh,
    #[serde(rename = "BD")]
    #[display("BD")]
    Bd,
    #[serde(rename = "BB")]
    #[display("BB")]
    Bb,
    #[serde(rename = "BY")]
    #[display("BY")]
    By,
    #[serde(rename = "BE")]
    #[display("BE")]
    Be,
    #[serde(rename = "BZ")]
    #[display("BZ")]
    Bz,
    #[serde(rename = "BJ")]
    #[display("BJ")]
    Bj,
    #[serde(rename = "BM")]
    #[display("BM")]
    Bm,
    #[serde(rename = "BT")]
    #[display("BT")]
    Bt,
    #[serde(rename = "BO")]
    #[display("BO")]
    Bo,
    #[serde(rename = "BQ")]
    #[display("BQ")]
    Bq,
    #[serde(rename = "BA")]
    #[display("BA")]
    Ba,
    #[serde(rename = "BW")]
    #[display("BW")]
    Bw,
    #[serde(rename = "BV")]
    #[display("BV")]
    Bv,
    #[serde(rename = "BR")]
    #[display("BR")]
    Br,
    #[serde(rename = "IO")]
    #[display("IO")]
    Io,
    #[serde(rename = "BN")]
    #[display("BN")]
    Bn,
    #[serde(rename = "BG")]
    #[display("BG")]
    Bg,
    #[serde(rename = "BF")]
    #[display("BF")]
    Bf,
    #[serde(rename = "BI")]
    #[display("BI")]
    Bi,
    #[serde(rename = "CV")]
    #[display("CV")]
    Cv,
    #[serde(rename = "KH")]
    #[display("KH")]
    Kh,
    #[serde(rename = "CM")]
    #[display("CM")]
    Cm,
    #[serde(rename = "CA")]
    #[display("CA")]
    Ca,
    #[serde(rename = "KY")]
    #[display("KY")]
    Ky,
    #[serde(rename = "CF")]
    #[display("CF")]
    Cf,
    #[serde(rename = "TD")]
    #[display("TD")]
    Td,
    #[serde(rename = "CL")]
    #[display("CL")]
    Cl,
    #[serde(rename = "CN")]
    #[display("CN")]
    Cn,
    #[serde(rename = "CX")]
    #[display("CX")]
    Cx,
    #[serde(rename = "CC")]
    #[display("CC")]
    Cc,
    #[serde(rename = "CO")]
    #[display("CO")]
    Co,
    #[serde(rename = "KM")]
    #[display("KM")]
    Km,
    #[serde(rename = "CG")]
    #[display("CG")]
    Cg,
    #[serde(rename = "CD")]
    #[display("CD")]
    Cd,
    #[serde(rename = "CK")]
    #[display("CK")]
    Ck,
    #[serde(rename = "CR")]
    #[display("CR")]
    Cr,
    #[serde(rename = "CI")]
    #[display("CI")]
    Ci,
    #[serde(rename = "HR")]
    #[display("HR")]
    Hr,
    #[serde(rename = "CW")]
    #[display("CW")]
    Cw,
    #[serde(rename = "CY")]
    #[display("CY")]
    Cy,
    #[serde(rename = "CZ")]
    #[display("CZ")]
    Cz,
    #[serde(rename = "DK")]
    #[display("DK")]
    Dk,
    #[serde(rename = "DJ")]
    #[display("DJ")]
    Dj,
    #[serde(rename = "DM")]
    #[display("DM")]
    Dm,
    #[serde(rename = "DO")]
    #[display("DO")]
    Do,
    #[serde(rename = "EC")]
    #[display("EC")]
    Ec,
    #[serde(rename = "EG")]
    #[display("EG")]
    Eg,
    #[serde(rename = "SV")]
    #[display("SV")]
    Sv,
    #[serde(rename = "GQ")]
    #[display("GQ")]
    Gq,
    #[serde(rename = "ER")]
    #[display("ER")]
    Er,
    #[serde(rename = "EE")]
    #[display("EE")]
    Ee,
    #[serde(rename = "SZ")]
    #[display("SZ")]
    Sz,
    #[serde(rename = "ET")]
    #[display("ET")]
    Et,
    #[serde(rename = "FK")]
    #[display("FK")]
    Fk,
    #[serde(rename = "FO")]
    #[display("FO")]
    Fo,
    #[serde(rename = "FJ")]
    #[display("FJ")]
    Fj,
    #[serde(rename = "FI")]
    #[display("FI")]
    Fi,
    #[serde(rename = "FR")]
    #[display("FR")]
    Fr,
    #[serde(rename = "GF")]
    #[display("GF")]
    Gf,
    #[serde(rename = "PF")]
    #[display("PF")]
    Pf,
    #[serde(rename = "TF")]
    #[display("TF")]
    Tf,
    #[serde(rename = "GA")]
    #[display("GA")]
    Ga,
    #[serde(rename = "GM")]
    #[display("GM")]
    Gm,
    #[serde(rename = "GE")]
    #[display("GE")]
    Ge,
    #[serde(rename = "DE")]
    #[display("DE")]
    De,
    #[serde(rename = "GH")]
    #[display("GH")]
    Gh,
    #[serde(rename = "GI")]
    #[display("GI")]
    Gi,
    #[serde(rename = "GR")]
    #[display("GR")]
    Gr,
    #[serde(rename = "GL")]
    #[display("GL")]
    Gl,
    #[serde(rename = "GD")]
    #[display("GD")]
    Gd,
    #[serde(rename = "GP")]
    #[display("GP")]
    Gp,
    #[serde(rename = "GU")]
    #[display("GU")]
    Gu,
    #[serde(rename = "GT")]
    #[display("GT")]
    Gt,
    #[serde(rename = "GG")]
    #[display("GG")]
    Gg,
    #[serde(rename = "GN")]
    #[display("GN")]
    Gn,
    #[serde(rename = "GW")]
    #[display("GW")]
    Gw,
    #[serde(rename = "GY")]
    #[display("GY")]
    Gy,
    #[serde(rename = "HT")]
    #[display("HT")]
    Ht,
    #[serde(rename = "HM")]
    #[display("HM")]
    Hm,
    #[serde(rename = "VA")]
    #[display("VA")]
    Va,
    #[serde(rename = "HN")]
    #[display("HN")]
    Hn,
    #[serde(rename = "HK")]
    #[display("HK")]
    Hk,
    #[serde(rename = "HU")]
    #[display("HU")]
    Hu,
    #[serde(rename = "IS")]
    #[display("IS")]
    Is,
    #[serde(rename = "IN")]
    #[display("IN")]
    In,
    #[serde(rename = "ID")]
    #[display("ID")]
    Id,
    #[serde(rename = "IQ")]
    #[display("IQ")]
    Iq,
    #[serde(rename = "IE")]
    #[display("IE")]
    Ie,
    #[serde(rename = "IM")]
    #[display("IM")]
    Im,
    #[serde(rename = "IL")]
    #[display("IL")]
    Il,
    #[serde(rename = "IT")]
    #[display("IT")]
    It,
    #[serde(rename = "JM")]
    #[display("JM")]
    Jm,
    #[serde(rename = "JP")]
    #[display("JP")]
    Jp,
    #[serde(rename = "JE")]
    #[display("JE")]
    Je,
    #[serde(rename = "JO")]
    #[display("JO")]
    Jo,
    #[serde(rename = "KZ")]
    #[display("KZ")]
    Kz,
    #[serde(rename = "KE")]
    #[display("KE")]
    Ke,
    #[serde(rename = "KI")]
    #[display("KI")]
    Ki,
    #[serde(rename = "KR")]
    #[display("KR")]
    Kr,
    #[serde(rename = "XK")]
    #[display("XK")]
    Xk,
    #[serde(rename = "KW")]
    #[display("KW")]
    Kw,
    #[serde(rename = "KG")]
    #[display("KG")]
    Kg,
    #[serde(rename = "LA")]
    #[display("LA")]
    La,
    #[serde(rename = "LV")]
    #[display("LV")]
    Lv,
    #[serde(rename = "LB")]
    #[display("LB")]
    Lb,
    #[serde(rename = "LS")]
    #[display("LS")]
    Ls,
    #[serde(rename = "LR")]
    #[display("LR")]
    Lr,
    #[serde(rename = "LY")]
    #[display("LY")]
    Ly,
    #[serde(rename = "LI")]
    #[display("LI")]
    Li,
    #[serde(rename = "LT")]
    #[display("LT")]
    Lt,
    #[serde(rename = "LU")]
    #[display("LU")]
    Lu,
    #[serde(rename = "MO")]
    #[display("MO")]
    Mo,
    #[serde(rename = "MG")]
    #[display("MG")]
    Mg,
    #[serde(rename = "MW")]
    #[display("MW")]
    Mw,
    #[serde(rename = "MY")]
    #[display("MY")]
    My,
    #[serde(rename = "MV")]
    #[display("MV")]
    Mv,
    #[serde(rename = "ML")]
    #[display("ML")]
    Ml,
    #[serde(rename = "MT")]
    #[display("MT")]
    Mt,
    #[serde(rename = "MH")]
    #[display("MH")]
    Mh,
    #[serde(rename = "MQ")]
    #[display("MQ")]
    Mq,
    #[serde(rename = "MR")]
    #[display("MR")]
    Mr,
    #[serde(rename = "MU")]
    #[display("MU")]
    Mu,
    #[serde(rename = "YT")]
    #[display("YT")]
    Yt,
    #[serde(rename = "MX")]
    #[display("MX")]
    Mx,
    #[serde(rename = "FM")]
    #[display("FM")]
    Fm,
    #[serde(rename = "MD")]
    #[display("MD")]
    Md,
    #[serde(rename = "MC")]
    #[display("MC")]
    Mc,
    #[serde(rename = "MN")]
    #[display("MN")]
    Mn,
    #[serde(rename = "ME")]
    #[display("ME")]
    Me,
    #[serde(rename = "MS")]
    #[display("MS")]
    Ms,
    #[serde(rename = "MA")]
    #[display("MA")]
    Ma,
    #[serde(rename = "MZ")]
    #[display("MZ")]
    Mz,
    #[serde(rename = "MM")]
    #[display("MM")]
    Mm,
    #[serde(rename = "NA")]
    #[display("NA")]
    Na,
    #[serde(rename = "NR")]
    #[display("NR")]
    Nr,
    #[serde(rename = "NP")]
    #[display("NP")]
    Np,
    #[serde(rename = "NL")]
    #[display("NL")]
    Nl,
    #[serde(rename = "AN")]
    #[display("AN")]
    An,
    #[serde(rename = "NC")]
    #[display("NC")]
    Nc,
    #[serde(rename = "NZ")]
    #[display("NZ")]
    Nz,
    #[serde(rename = "NI")]
    #[display("NI")]
    Ni,
    #[serde(rename = "NE")]
    #[display("NE")]
    Ne,
    #[serde(rename = "NG")]
    #[display("NG")]
    Ng,
    #[serde(rename = "NU")]
    #[display("NU")]
    Nu,
    #[serde(rename = "NF")]
    #[display("NF")]
    Nf,
    #[serde(rename = "MK")]
    #[display("MK")]
    Mk,
    #[serde(rename = "MP")]
    #[display("MP")]
    Mp,
    #[serde(rename = "NO")]
    #[display("NO")]
    No,
    #[serde(rename = "OM")]
    #[display("OM")]
    Om,
    #[serde(rename = "PK")]
    #[display("PK")]
    Pk,
    #[serde(rename = "PW")]
    #[display("PW")]
    Pw,
    #[serde(rename = "PS")]
    #[display("PS")]
    Ps,
    #[serde(rename = "PA")]
    #[display("PA")]
    Pa,
    #[serde(rename = "PG")]
    #[display("PG")]
    Pg,
    #[serde(rename = "PY")]
    #[display("PY")]
    Py,
    #[serde(rename = "PE")]
    #[display("PE")]
    Pe,
    #[serde(rename = "PH")]
    #[display("PH")]
    Ph,
    #[serde(rename = "PN")]
    #[display("PN")]
    Pn,
    #[serde(rename = "PL")]
    #[display("PL")]
    Pl,
    #[serde(rename = "PT")]
    #[display("PT")]
    Pt,
    #[serde(rename = "PR")]
    #[display("PR")]
    Pr,
    #[serde(rename = "QA")]
    #[display("QA")]
    Qa,
    #[serde(rename = "RO")]
    #[display("RO")]
    Ro,
    #[serde(rename = "RU")]
    #[display("RU")]
    Ru,
    #[serde(rename = "RW")]
    #[display("RW")]
    Rw,
    #[serde(rename = "RE")]
    #[display("RE")]
    Re,
    #[serde(rename = "BL")]
    #[display("BL")]
    Bl,
    #[serde(rename = "SH")]
    #[display("SH")]
    Sh,
    #[serde(rename = "KN")]
    #[display("KN")]
    Kn,
    #[serde(rename = "LC")]
    #[display("LC")]
    Lc,
    #[serde(rename = "MF")]
    #[display("MF")]
    Mf,
    #[serde(rename = "PM")]
    #[display("PM")]
    Pm,
    #[serde(rename = "VC")]
    #[display("VC")]
    Vc,
    #[serde(rename = "WS")]
    #[display("WS")]
    Ws,
    #[serde(rename = "SM")]
    #[display("SM")]
    Sm,
    #[serde(rename = "ST")]
    #[display("ST")]
    St,
    #[serde(rename = "SA")]
    #[display("SA")]
    Sa,
    #[serde(rename = "SN")]
    #[display("SN")]
    Sn,
    #[serde(rename = "RS")]
    #[display("RS")]
    Rs,
    #[serde(rename = "SC")]
    #[display("SC")]
    Sc,
    #[serde(rename = "SL")]
    #[display("SL")]
    Sl,
    #[serde(rename = "SG")]
    #[display("SG")]
    Sg,
    #[serde(rename = "SX")]
    #[display("SX")]
    Sx,
    #[serde(rename = "SK")]
    #[display("SK")]
    Sk,
    #[serde(rename = "SI")]
    #[display("SI")]
    Si,
    #[serde(rename = "SB")]
    #[display("SB")]
    Sb,
    #[serde(rename = "SO")]
    #[display("SO")]
    So,
    #[serde(rename = "ZA")]
    #[display("ZA")]
    Za,
    #[serde(rename = "GS")]
    #[display("GS")]
    Gs,
    #[serde(rename = "SS")]
    #[display("SS")]
    Ss,
    #[serde(rename = "ES")]
    #[display("ES")]
    Es,
    #[serde(rename = "LK")]
    #[display("LK")]
    Lk,
    #[serde(rename = "SD")]
    #[display("SD")]
    Sd,
    #[serde(rename = "SR")]
    #[display("SR")]
    Sr,
    #[serde(rename = "SJ")]
    #[display("SJ")]
    Sj,
    #[serde(rename = "SE")]
    #[display("SE")]
    Se,
    #[serde(rename = "CH")]
    #[display("CH")]
    Ch,
    #[serde(rename = "TW")]
    #[display("TW")]
    Tw,
    #[serde(rename = "TJ")]
    #[display("TJ")]
    Tj,
    #[serde(rename = "TZ")]
    #[display("TZ")]
    Tz,
    #[serde(rename = "TH")]
    #[display("TH")]
    Th,
    #[serde(rename = "TL")]
    #[display("TL")]
    Tl,
    #[serde(rename = "TG")]
    #[display("TG")]
    Tg,
    #[serde(rename = "TK")]
    #[display("TK")]
    Tk,
    #[serde(rename = "TO")]
    #[display("TO")]
    To,
    #[serde(rename = "TT")]
    #[display("TT")]
    Tt,
    #[serde(rename = "TN")]
    #[display("TN")]
    Tn,
    #[serde(rename = "TR")]
    #[display("TR")]
    Tr,
    #[serde(rename = "TM")]
    #[display("TM")]
    Tm,
    #[serde(rename = "TC")]
    #[display("TC")]
    Tc,
    #[serde(rename = "TV")]
    #[display("TV")]
    Tv,
    #[serde(rename = "UG")]
    #[display("UG")]
    Ug,
    #[serde(rename = "UA")]
    #[display("UA")]
    Ua,
    #[serde(rename = "AE")]
    #[display("AE")]
    Ae,
    #[serde(rename = "GB")]
    #[display("GB")]
    Gb,
    #[serde(rename = "US")]
    #[display("US")]
    Us,
    #[serde(rename = "UM")]
    #[display("UM")]
    Um,
    #[serde(rename = "UY")]
    #[display("UY")]
    Uy,
    #[serde(rename = "UZ")]
    #[display("UZ")]
    Uz,
    #[serde(rename = "VU")]
    #[display("VU")]
    Vu,
    #[serde(rename = "VE")]
    #[display("VE")]
    Ve,
    #[serde(rename = "VN")]
    #[display("VN")]
    Vn,
    #[serde(rename = "VG")]
    #[display("VG")]
    Vg,
    #[serde(rename = "VI")]
    #[display("VI")]
    Vi,
    #[serde(rename = "WF")]
    #[display("WF")]
    Wf,
    #[serde(rename = "EH")]
    #[display("EH")]
    Eh,
    #[serde(rename = "YE")]
    #[display("YE")]
    Ye,
    #[serde(rename = "ZM")]
    #[display("ZM")]
    Zm,
    #[serde(rename = "ZW")]
    #[display("ZW")]
    Zw,
}

#[doc = "Worker."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Worker {
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The worker's associated user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[doc = "The worker's associated user.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[doc = "The worker's manager."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager_id: Option<String>,
    #[doc = "The worker's manager.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manager: Option<Box<Worker>>,
    #[doc = "The worker's associated legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_entity_id: Option<String>,
    #[doc = "The worker's associated legal entity.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_entity: Option<LegalEntity>,
    #[doc = "The worker's country."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<WorkerCountry>,
    #[doc = "The start date of the worker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[doc = "The end date of the worker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[doc = "The worker's number within the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
    #[doc = "The worker's associated work email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_email: Option<String>,
    #[doc = "The worker's associated personal email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub personal_email: Option<String>,
    #[doc = "The worker's status within the organization."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<WorkerStatus>,
    #[doc = "The location that the worker is mapped to for tax purposes. In the case that a \
             worker is remote, the location's type is remote."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<WorkerLocation>,
    #[doc = "The worker's employment type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type_id: Option<String>,
    #[doc = "The worker's employment type.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<CompanyEmploymentType>,
    #[doc = "The gender of the worker, if specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[doc = "The worker's date of birth."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[doc = "The identified race of the worker, if specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub race: Option<Race>,
    #[doc = "The identified ethnicity of the worker, if specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ethnicity: Option<Ethnicity>,
    #[doc = "The countries that the worker has citizenship in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<Citizenship>,
    #[doc = "The compensation package for the worker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_id: Option<String>,
    #[doc = "The compensation package for the worker.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation: Option<Compensation>,
    #[doc = "The worker's assigned department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    #[doc = "The worker's assigned department.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department: Option<Department>,
    #[doc = "The worker's assigned teams."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams_id: Option<Vec<String>>,
    #[doc = "The worker's assigned teams.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<Team>>,
    #[doc = "The worker's title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The level of the worker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_id: Option<String>,
    #[doc = "The level of the worker.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[doc = "The details of the worker's termination, if applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub termination_details: Option<TerminationDetails>,
    #[doc = "Custom fields for the worker\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
}

impl std::fmt::Display for Worker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for Worker {
    const LENGTH: usize = 35;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(user_id) = &self.user_id {
                format!("{:?}", user_id).into()
            } else {
                String::new().into()
            },
            if let Some(user) = &self.user {
                format!("{:?}", user).into()
            } else {
                String::new().into()
            },
            if let Some(manager_id) = &self.manager_id {
                format!("{:?}", manager_id).into()
            } else {
                String::new().into()
            },
            if let Some(manager) = &self.manager {
                format!("{:?}", manager).into()
            } else {
                String::new().into()
            },
            if let Some(legal_entity_id) = &self.legal_entity_id {
                format!("{:?}", legal_entity_id).into()
            } else {
                String::new().into()
            },
            if let Some(legal_entity) = &self.legal_entity {
                format!("{:?}", legal_entity).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
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
            if let Some(number) = &self.number {
                format!("{:?}", number).into()
            } else {
                String::new().into()
            },
            if let Some(work_email) = &self.work_email {
                format!("{:?}", work_email).into()
            } else {
                String::new().into()
            },
            if let Some(personal_email) = &self.personal_email {
                format!("{:?}", personal_email).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(location) = &self.location {
                format!("{:?}", location).into()
            } else {
                String::new().into()
            },
            if let Some(employment_type_id) = &self.employment_type_id {
                format!("{:?}", employment_type_id).into()
            } else {
                String::new().into()
            },
            if let Some(employment_type) = &self.employment_type {
                format!("{:?}", employment_type).into()
            } else {
                String::new().into()
            },
            if let Some(gender) = &self.gender {
                format!("{:?}", gender).into()
            } else {
                String::new().into()
            },
            if let Some(date_of_birth) = &self.date_of_birth {
                format!("{:?}", date_of_birth).into()
            } else {
                String::new().into()
            },
            if let Some(race) = &self.race {
                format!("{:?}", race).into()
            } else {
                String::new().into()
            },
            if let Some(ethnicity) = &self.ethnicity {
                format!("{:?}", ethnicity).into()
            } else {
                String::new().into()
            },
            if let Some(citizenship) = &self.citizenship {
                format!("{:?}", citizenship).into()
            } else {
                String::new().into()
            },
            if let Some(compensation_id) = &self.compensation_id {
                format!("{:?}", compensation_id).into()
            } else {
                String::new().into()
            },
            if let Some(compensation) = &self.compensation {
                format!("{:?}", compensation).into()
            } else {
                String::new().into()
            },
            if let Some(department_id) = &self.department_id {
                format!("{:?}", department_id).into()
            } else {
                String::new().into()
            },
            if let Some(department) = &self.department {
                format!("{:?}", department).into()
            } else {
                String::new().into()
            },
            if let Some(teams_id) = &self.teams_id {
                format!("{:?}", teams_id).into()
            } else {
                String::new().into()
            },
            if let Some(teams) = &self.teams {
                format!("{:?}", teams).into()
            } else {
                String::new().into()
            },
            if let Some(title) = &self.title {
                format!("{:?}", title).into()
            } else {
                String::new().into()
            },
            if let Some(level_id) = &self.level_id {
                format!("{:?}", level_id).into()
            } else {
                String::new().into()
            },
            if let Some(level) = &self.level {
                format!("{:?}", level).into()
            } else {
                String::new().into()
            },
            if let Some(termination_details) = &self.termination_details {
                format!("{:?}", termination_details).into()
            } else {
                String::new().into()
            },
            if let Some(custom_fields) = &self.custom_fields {
                format!("{:?}", custom_fields).into()
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
            "user_id".into(),
            "user".into(),
            "manager_id".into(),
            "manager".into(),
            "legal_entity_id".into(),
            "legal_entity".into(),
            "country".into(),
            "start_date".into(),
            "end_date".into(),
            "number".into(),
            "work_email".into(),
            "personal_email".into(),
            "status".into(),
            "location".into(),
            "employment_type_id".into(),
            "employment_type".into(),
            "gender".into(),
            "date_of_birth".into(),
            "race".into(),
            "ethnicity".into(),
            "citizenship".into(),
            "compensation_id".into(),
            "compensation".into(),
            "department_id".into(),
            "department".into(),
            "teams_id".into(),
            "teams".into(),
            "title".into(),
            "level_id".into(),
            "level".into(),
            "termination_details".into(),
            "custom_fields".into(),
        ]
    }
}

#[doc = "The type of location."]
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
pub enum WorkerLocationType {
    #[serde(rename = "REMOTE")]
    #[display("REMOTE")]
    Remote,
    #[serde(rename = "WORK")]
    #[display("WORK")]
    Work,
}

#[doc = "WorkerLocation."]
#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct WorkerLocation {
    #[doc = "The type of location."]
    #[serde(rename = "type")]
    pub type_: WorkerLocationType,
}

impl std::fmt::Display for WorkerLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for WorkerLocation {
    const LENGTH: usize = 1;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![format!("{:?}", self.type_).into()]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["type_".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCandidatesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Candidate>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCandidatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCandidatesResponse {
    type Item = Candidate;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCandidatesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCandidateApplicationsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Application>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCandidateApplicationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCandidateApplicationsResponse {
    type Item = Application;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCandidateApplicationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCompaniesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Company>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCompaniesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCompaniesResponse {
    type Item = Company;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCompaniesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCompensationsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Compensation>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCompensationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCompensationsResponse {
    type Item = Compensation;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCompensationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCompensationsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The worker's ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[doc = "The worker's annual compensation. This calculation assumes 40-hour work weeks for \
             workers with an hourly wage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_compensation: Option<Currency>,
    #[doc = "The worker's annual salary equivalent, for insurance purposes. It will be equal to \
             the worker's annual compensation, except for owners that are receiving no \
             cashcompensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annual_salary_equivalent: Option<Currency>,
    #[doc = "The worker's hourly wage. This calculation assumes 40-hour work weeks for workers \
             with fixed compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hourly_wage: Option<Currency>,
    #[doc = "The worker's monthly compensation. This calculation assumes 40-hour work weeks for \
             workers with an hourly wage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub monthly_compensation: Option<Currency>,
    #[doc = "The worker's on-target commission."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_target_commission: Option<Currency>,
    #[doc = "The worker's hourly wage. This calculation assumes 40-hour work weeks for workers \
             with fixed compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relocation_reimbursement: Option<Currency>,
    #[doc = "The worker's signing bonus."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_bonus: Option<Currency>,
    #[doc = "The worker's target annual bonus amount."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_annual_bonus: Option<Currency>,
    #[doc = "The worker's weekly compensation. This calculation assumes 40-hour work weeks for \
             workers with an hourly wage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weekly_compensation: Option<Currency>,
    #[doc = "The worker's target annual bonus as a percent of annual compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_annual_bonus_percent: Option<f64>,
    #[doc = "The worker's bonus schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bonus_schedule: Option<String>,
    #[doc = "The payment type for an worker's compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_type: Option<String>,
    #[doc = "The payment terms for an worker's compensation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<String>,
}

impl std::fmt::Display for GetCompensationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCompensationsResponse {
    const LENGTH: usize = 18;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(worker_id) = &self.worker_id {
                format!("{:?}", worker_id).into()
            } else {
                String::new().into()
            },
            if let Some(annual_compensation) = &self.annual_compensation {
                format!("{:?}", annual_compensation).into()
            } else {
                String::new().into()
            },
            if let Some(annual_salary_equivalent) = &self.annual_salary_equivalent {
                format!("{:?}", annual_salary_equivalent).into()
            } else {
                String::new().into()
            },
            if let Some(hourly_wage) = &self.hourly_wage {
                format!("{:?}", hourly_wage).into()
            } else {
                String::new().into()
            },
            if let Some(monthly_compensation) = &self.monthly_compensation {
                format!("{:?}", monthly_compensation).into()
            } else {
                String::new().into()
            },
            if let Some(on_target_commission) = &self.on_target_commission {
                format!("{:?}", on_target_commission).into()
            } else {
                String::new().into()
            },
            if let Some(relocation_reimbursement) = &self.relocation_reimbursement {
                format!("{:?}", relocation_reimbursement).into()
            } else {
                String::new().into()
            },
            if let Some(signing_bonus) = &self.signing_bonus {
                format!("{:?}", signing_bonus).into()
            } else {
                String::new().into()
            },
            if let Some(target_annual_bonus) = &self.target_annual_bonus {
                format!("{:?}", target_annual_bonus).into()
            } else {
                String::new().into()
            },
            if let Some(weekly_compensation) = &self.weekly_compensation {
                format!("{:?}", weekly_compensation).into()
            } else {
                String::new().into()
            },
            if let Some(target_annual_bonus_percent) = &self.target_annual_bonus_percent {
                format!("{:?}", target_annual_bonus_percent).into()
            } else {
                String::new().into()
            },
            if let Some(bonus_schedule) = &self.bonus_schedule {
                format!("{:?}", bonus_schedule).into()
            } else {
                String::new().into()
            },
            if let Some(payment_type) = &self.payment_type {
                format!("{:?}", payment_type).into()
            } else {
                String::new().into()
            },
            if let Some(payment_terms) = &self.payment_terms {
                format!("{:?}", payment_terms).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "worker_id".into(),
            "annual_compensation".into(),
            "annual_salary_equivalent".into(),
            "hourly_wage".into(),
            "monthly_compensation".into(),
            "on_target_commission".into(),
            "relocation_reimbursement".into(),
            "signing_bonus".into(),
            "target_annual_bonus".into(),
            "weekly_compensation".into(),
            "target_annual_bonus_percent".into(),
            "bonus_schedule".into(),
            "payment_type".into(),
            "payment_terms".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCustomFieldsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<CustomField>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCustomFieldsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCustomFieldsResponse {
    type Item = CustomField;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCustomFieldsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCustomObjectsResponse {
    pub results: Vec<CustomObject>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCustomObjectsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCustomObjectsResponse {
    type Item = CustomObject;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCustomObjectsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCustomObjectsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl std::fmt::Display for CreateCustomObjectsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateCustomObjectsRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(category) = &self.category {
                format!("{:?}", category).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "description".into(), "category".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCustomObjectsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plural_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
}

impl std::fmt::Display for UpdateCustomObjectsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCustomObjectsRequestBody {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(category) = &self.category {
                format!("{:?}", category).into()
            } else {
                String::new().into()
            },
            if let Some(plural_label) = &self.plural_label {
                format!("{:?}", plural_label).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "description".into(),
            "category".into(),
            "plural_label".into(),
            "owner_role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListDepartmentsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Department>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListDepartmentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListDepartmentsResponse {
    type Item = Department;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListDepartmentsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetDepartmentsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the department."]
    pub name: String,
    #[doc = "The parent department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent department.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Department>,
    #[doc = "Reference code of the department."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_code: Option<String>,
}

impl std::fmt::Display for GetDepartmentsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetDepartmentsResponse {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            if let Some(reference_code) = &self.reference_code {
                format!("{:?}", reference_code).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "parent_id".into(),
            "parent".into(),
            "reference_code".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListEmploymentTypesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<CompanyEmploymentType>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListEmploymentTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListEmploymentTypesResponse {
    type Item = CompanyEmploymentType;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListEmploymentTypesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[doc = "The classification of the worker by the company. * `CONTRACTOR`: Contractors are \
         self-employed workers who provide services on a short-term or per-project basis and are \
         not eligible for tax-withholding or benefits. * `EMPLOYEE`: Employees are hired and \
         managed by an employer, work under the employer's direct supervision and control, and are \
         protected by law for wages and employment rights."]
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
pub enum GetEmploymentTypesResponseType {
    #[serde(rename = "CONTRACTOR")]
    #[display("CONTRACTOR")]
    Contractor,
    #[serde(rename = "EMPLOYEE")]
    #[display("EMPLOYEE")]
    Employee,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetEmploymentTypesResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The display label of the employment type."]
    pub label: String,
    #[doc = "The name of the employment type for non-custom employment types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The classification of the worker by the company. * `CONTRACTOR`: Contractors are \
             self-employed workers who provide services on a short-term or per-project basis and \
             are not eligible for tax-withholding or benefits. * `EMPLOYEE`: Employees are hired \
             and managed by an employer, work under the employer's direct supervision and \
             control, and are protected by law for wages and employment rights."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<GetEmploymentTypesResponseType>,
    #[doc = "The compensation period for the employment type. * `SALARIED`: Employees that are \
             paid a fixed amount per year. * `HOURLY`: Employees that are paid a wage per hour \
             worked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensation_time_period: Option<CompensationTimePeriod>,
    #[doc = "The amount worked for the employment type. * `FULL-TIME`: Full-time is at least 30 \
             hours per week. Full-time workers will typically be eligible for benefits. * \
             `PART-TIME`: Part-time is less than 30 hours per week. These workers may be eligible \
             for benefits, depending on company settings and hours worked. * `TEMPORARY`: These \
             workers are hired on a temporary basis. You can specify how each worker with this \
             employment type will be paid individually."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub amount_worked: Option<AmountWorked>,
}

impl std::fmt::Display for GetEmploymentTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetEmploymentTypesResponse {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.label.clone().into(),
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
            if let Some(compensation_time_period) = &self.compensation_time_period {
                format!("{:?}", compensation_time_period).into()
            } else {
                String::new().into()
            },
            if let Some(amount_worked) = &self.amount_worked {
                format!("{:?}", amount_worked).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "label".into(),
            "name".into(),
            "type_".into(),
            "compensation_time_period".into(),
            "amount_worked".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListEntitlementsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<EntitlementModel>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListEntitlementsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListEntitlementsResponse {
    type Item = EntitlementModel;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListEntitlementsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListJobCodesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<JobCode>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListJobCodesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListJobCodesResponse {
    type Item = JobCode;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListJobCodesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetJobCodesResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the job dimension."]
    pub name: String,
    #[doc = "The ID of the job dimension this job code belongs to."]
    pub job_dimension_id: String,
    #[doc = "The job dimension this job code belongs to.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_dimension: Option<JobDimension>,
    #[doc = "The unique identifier of the job code in an outside system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[doc = "The ID of the job roster group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

impl std::fmt::Display for GetJobCodesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetJobCodesResponse {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            self.job_dimension_id.clone().into(),
            if let Some(job_dimension) = &self.job_dimension {
                format!("{:?}", job_dimension).into()
            } else {
                String::new().into()
            },
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
            if let Some(group_id) = &self.group_id {
                format!("{:?}", group_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "job_dimension_id".into(),
            "job_dimension".into(),
            "external_id".into(),
            "group_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListJobDimensionsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<JobDimension>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListJobDimensionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListJobDimensionsResponse {
    type Item = JobDimension;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListJobDimensionsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetJobDimensionsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the job dimension"]
    pub name: String,
    #[doc = "The unique identifier of the job dimension in a third party system"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl std::fmt::Display for GetJobDimensionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetJobDimensionsResponse {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(external_id) = &self.external_id {
                format!("{:?}", external_id).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "external_id".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListJobRequisitionsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<JobRequisition>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListJobRequisitionsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListJobRequisitionsResponse {
    type Item = JobRequisition;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListJobRequisitionsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLeaveBalancesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<LeaveBalance>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListLeaveBalancesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListLeaveBalancesResponse {
    type Item = LeaveBalance;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLeaveBalancesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetLeaveBalancesResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the leave balance."]
    pub worker_id: String,
    #[doc = "The worker associated with the leave balance.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The ID of the leave type associated with the leave balance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[doc = "The leave type associated with the leave balance.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type: Option<LeaveType>,
    #[doc = "Indicates if the leave balance is unlimited."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_balance_unlimited: Option<bool>,
    #[doc = "The worker's leave balance including future leave requests. If the leave balance is \
             unlimited, this field will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_including_future_requests: Option<f64>,
    #[doc = "The worker's leave balance excluding future leave requests. If the leave balance is \
             unlimited, this field will be null."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_excluding_future_requests: Option<f64>,
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
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type_id) = &self.leave_type_id {
                format!("{:?}", leave_type_id).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type) = &self.leave_type {
                format!("{:?}", leave_type).into()
            } else {
                String::new().into()
            },
            if let Some(is_balance_unlimited) = &self.is_balance_unlimited {
                format!("{:?}", is_balance_unlimited).into()
            } else {
                String::new().into()
            },
            if let Some(balance_including_future_requests) = &self.balance_including_future_requests
            {
                format!("{:?}", balance_including_future_requests).into()
            } else {
                String::new().into()
            },
            if let Some(balance_excluding_future_requests) = &self.balance_excluding_future_requests
            {
                format!("{:?}", balance_excluding_future_requests).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "worker_id".into(),
            "worker".into(),
            "leave_type_id".into(),
            "leave_type".into(),
            "is_balance_unlimited".into(),
            "balance_including_future_requests".into(),
            "balance_excluding_future_requests".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLeaveRequestsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<LeaveRequest>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListLeaveRequestsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListLeaveRequestsResponse {
    type Item = LeaveRequest;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLeaveRequestsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[doc = "The status of the leave request."]
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
pub enum GetLeaveRequestsResponseStatus {
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
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetLeaveRequestsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the leave request."]
    pub worker_id: String,
    #[doc = "The worker associated with the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The ID of the worker who requested the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester_id: Option<String>,
    #[doc = "The worker who requested the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requester: Option<Worker>,
    #[doc = "The status of the leave request."]
    pub status: GetLeaveRequestsResponseStatus,
    #[doc = "The start date of the leave request."]
    pub start_date: String,
    #[doc = "The start time of the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end date of the leave request."]
    pub end_date: String,
    #[doc = "The end time of the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The comments associated with the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[doc = "The number of minutes requested for the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_minutes_requested: Option<f64>,
    #[doc = "The ID of the leave policy associated with the leave request."]
    pub leave_policy_id: String,
    #[doc = "The ID of the leave type associated with the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    #[doc = "The leave type associated with the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type: Option<LeaveType>,
    #[doc = "The reason for the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason_for_leave: Option<String>,
    #[doc = "The ID of the worker who reviewed the leave request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_id: Option<String>,
    #[doc = "The worker who reviewed the leave request.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<Worker>,
    #[doc = "The timestamp the leave request was reviewed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewed_at: Option<String>,
    #[doc = "The specific dates taken off and the amount of time taken off for each one."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub days_take_off: Option<Vec<DayOff>>,
    #[doc = "Whether the leave request is managed by an external system."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_managed_by_external_system: Option<bool>,
}

impl std::fmt::Display for GetLeaveRequestsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetLeaveRequestsResponse {
    const LENGTH: usize = 24;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(requester_id) = &self.requester_id {
                format!("{:?}", requester_id).into()
            } else {
                String::new().into()
            },
            if let Some(requester) = &self.requester {
                format!("{:?}", requester).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.status).into(),
            self.start_date.clone().into(),
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            self.end_date.clone().into(),
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(comments) = &self.comments {
                format!("{:?}", comments).into()
            } else {
                String::new().into()
            },
            if let Some(number_of_minutes_requested) = &self.number_of_minutes_requested {
                format!("{:?}", number_of_minutes_requested).into()
            } else {
                String::new().into()
            },
            self.leave_policy_id.clone().into(),
            if let Some(leave_type_id) = &self.leave_type_id {
                format!("{:?}", leave_type_id).into()
            } else {
                String::new().into()
            },
            if let Some(leave_type) = &self.leave_type {
                format!("{:?}", leave_type).into()
            } else {
                String::new().into()
            },
            if let Some(reason_for_leave) = &self.reason_for_leave {
                format!("{:?}", reason_for_leave).into()
            } else {
                String::new().into()
            },
            if let Some(reviewer_id) = &self.reviewer_id {
                format!("{:?}", reviewer_id).into()
            } else {
                String::new().into()
            },
            if let Some(reviewer) = &self.reviewer {
                format!("{:?}", reviewer).into()
            } else {
                String::new().into()
            },
            if let Some(reviewed_at) = &self.reviewed_at {
                format!("{:?}", reviewed_at).into()
            } else {
                String::new().into()
            },
            if let Some(days_take_off) = &self.days_take_off {
                format!("{:?}", days_take_off).into()
            } else {
                String::new().into()
            },
            if let Some(is_managed_by_external_system) = &self.is_managed_by_external_system {
                format!("{:?}", is_managed_by_external_system).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "worker_id".into(),
            "worker".into(),
            "requester_id".into(),
            "requester".into(),
            "status".into(),
            "start_date".into(),
            "start_time".into(),
            "end_date".into(),
            "end_time".into(),
            "comments".into(),
            "number_of_minutes_requested".into(),
            "leave_policy_id".into(),
            "leave_type_id".into(),
            "leave_type".into(),
            "reason_for_leave".into(),
            "reviewer_id".into(),
            "reviewer".into(),
            "reviewed_at".into(),
            "days_take_off".into(),
            "is_managed_by_external_system".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLeaveTypesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<LeaveType>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListLeaveTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListLeaveTypesResponse {
    type Item = LeaveType;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLeaveTypesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetLeaveTypesResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The type of leave."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "The name of the leave type."]
    pub name: String,
    #[doc = "The description of the leave type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the leave is paid."]
    pub is_paid: bool,
    #[doc = "Whether the leave is managed by an external system."]
    pub is_managed_by_external_system: bool,
}

impl std::fmt::Display for GetLeaveTypesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetLeaveTypesResponse {
    const LENGTH: usize = 9;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.type_.clone().into(),
            self.name.clone().into(),
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.is_paid).into(),
            format!("{:?}", self.is_managed_by_external_system).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "type_".into(),
            "name".into(),
            "description".into(),
            "is_paid".into(),
            "is_managed_by_external_system".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLegalEntitiesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<LegalEntity>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListLegalEntitiesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListLegalEntitiesResponse {
    type Item = LegalEntity;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLegalEntitiesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetLegalEntitiesResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The tax identifier for the legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tax_identifier: Option<String>,
    #[doc = "The country the legal entity is based in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[doc = "The legal name of the legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    #[doc = "The legal entity's level in a hierarchy. * `PARENT`: The legal entity is considered \
             the ultimate holding entity. * `SUBSIDIARY`: The legal entity is considered a \
             subsidiary, fully or partially held by another. * `BRANCH`: The legal entity is \
             considered a branch, associated with a parent legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entity_level: Option<EntityLevel>,
    #[doc = "The registration date of the entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<String>,
    #[doc = "The mailing address of the legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<Address>,
    #[doc = "The physical address of the legal entity, if it differs from the mailing address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_address: Option<Address>,
    #[doc = "The parent legal entity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent legal entity.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<LegalEntity>,
    #[doc = "The legal entity management type in the case of an employer of record (EOR) or \
             professional employment organization (PEO). * `PEO`: The legal entity is considered \
             a Professional Employment Organization (PEO). * `EOR`: The legal entity is \
             considered an Employer of Record (EOR)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management_type: Option<ManagementType>,
    #[doc = "The company or organization associated with the legal entity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[doc = "The company or organization associated with the legal entity\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<Company>,
}

impl std::fmt::Display for GetLegalEntitiesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetLegalEntitiesResponse {
    const LENGTH: usize = 16;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(tax_identifier) = &self.tax_identifier {
                format!("{:?}", tax_identifier).into()
            } else {
                String::new().into()
            },
            if let Some(country) = &self.country {
                format!("{:?}", country).into()
            } else {
                String::new().into()
            },
            if let Some(legal_name) = &self.legal_name {
                format!("{:?}", legal_name).into()
            } else {
                String::new().into()
            },
            if let Some(entity_level) = &self.entity_level {
                format!("{:?}", entity_level).into()
            } else {
                String::new().into()
            },
            if let Some(registration_date) = &self.registration_date {
                format!("{:?}", registration_date).into()
            } else {
                String::new().into()
            },
            if let Some(mailing_address) = &self.mailing_address {
                format!("{:?}", mailing_address).into()
            } else {
                String::new().into()
            },
            if let Some(physical_address) = &self.physical_address {
                format!("{:?}", physical_address).into()
            } else {
                String::new().into()
            },
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            if let Some(management_type) = &self.management_type {
                format!("{:?}", management_type).into()
            } else {
                String::new().into()
            },
            if let Some(company_id) = &self.company_id {
                format!("{:?}", company_id).into()
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
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "tax_identifier".into(),
            "country".into(),
            "legal_name".into(),
            "entity_level".into(),
            "registration_date".into(),
            "mailing_address".into(),
            "physical_address".into(),
            "parent_id".into(),
            "parent".into(),
            "management_type".into(),
            "company_id".into(),
            "company".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListLevelsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Level>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListLevelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListLevelsResponse {
    type Item = Level;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListLevelsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetLevelsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the level. Must be unique within the company or organization."]
    pub name: String,
    #[doc = "The parent level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent level.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Level>,
    #[doc = "Global level is used to track the seniority of levels. The higher up a level is \
             placed on the page, the more senior and higher-ranked the level. Global level is \
             used in workflows, policies, and reports that use the level attribute (e.g., you can \
             use Level Lookup to set up a workflow that notifies the nearest person in an \
             worker's management chain at or above the specified level)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_level: Option<i64>,
    #[doc = "The description of the level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The rank of the level within its track."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<i64>,
    #[doc = "The track associated with the level, if it's not a global level."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_id: Option<String>,
    #[doc = "The track associated with the level, if it's not a global level.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track: Option<Track>,
}

impl std::fmt::Display for GetLevelsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetLevelsResponse {
    const LENGTH: usize = 12;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            if let Some(global_level) = &self.global_level {
                format!("{:?}", global_level).into()
            } else {
                String::new().into()
            },
            if let Some(description) = &self.description {
                format!("{:?}", description).into()
            } else {
                String::new().into()
            },
            if let Some(rank) = &self.rank {
                format!("{:?}", rank).into()
            } else {
                String::new().into()
            },
            if let Some(track_id) = &self.track_id {
                format!("{:?}", track_id).into()
            } else {
                String::new().into()
            },
            if let Some(track) = &self.track {
                format!("{:?}", track).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "parent_id".into(),
            "parent".into(),
            "global_level".into(),
            "description".into(),
            "rank".into(),
            "track_id".into(),
            "track".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListObjectCategoriesResponse {
    pub results: Vec<ObjectCategory>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListObjectCategoriesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListObjectCategoriesResponse {
    type Item = ObjectCategory;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListObjectCategoriesResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateObjectCategoriesRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl std::fmt::Display for CreateObjectCategoriesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateObjectCategoriesRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "description".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateObjectCategoriesRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl std::fmt::Display for UpdateObjectCategoriesRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateObjectCategoriesRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "description".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListShiftInputsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<ShiftInput>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListShiftInputsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListShiftInputsResponse {
    type Item = ShiftInput;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListShiftInputsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetShiftInputsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The creator id associated with the shift input."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<String>,
    #[doc = "The creator associated with the shift input.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Worker>,
    #[doc = "Name of the shift unit."]
    pub name: String,
    #[doc = "Prompt for the shift unit."]
    pub prompt: String,
    #[doc = "Type of shift unit."]
    #[serde(rename = "type")]
    pub type_: String,
    #[doc = "Two letter string designating country code which the shift input is associated."]
    pub country_code: String,
    #[doc = "The party that manages this shift input"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
}

impl std::fmt::Display for GetShiftInputsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetShiftInputsResponse {
    const LENGTH: usize = 11;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(creator_id) = &self.creator_id {
                format!("{:?}", creator_id).into()
            } else {
                String::new().into()
            },
            if let Some(creator) = &self.creator {
                format!("{:?}", creator).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
            self.prompt.clone().into(),
            self.type_.clone().into(),
            self.country_code.clone().into(),
            if let Some(managed_by) = &self.managed_by {
                format!("{:?}", managed_by).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "creator_id".into(),
            "creator".into(),
            "name".into(),
            "prompt".into(),
            "type_".into(),
            "country_code".into(),
            "managed_by".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTeamsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Team>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListTeamsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListTeamsResponse {
    type Item = Team;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTeamsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTeamsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The parent team"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[doc = "The parent team\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Team>,
    #[doc = "The name of the team."]
    pub name: String,
}

impl std::fmt::Display for GetTeamsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTeamsResponse {
    const LENGTH: usize = 7;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(parent_id) = &self.parent_id {
                format!("{:?}", parent_id).into()
            } else {
                String::new().into()
            },
            if let Some(parent) = &self.parent {
                format!("{:?}", parent).into()
            } else {
                String::new().into()
            },
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "parent_id".into(),
            "parent".into(),
            "name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTimeCardsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<TimeCard>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListTimeCardsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListTimeCardsResponse {
    type Item = TimeCard;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTimeCardsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTimeCardsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the time card."]
    pub worker_id: String,
    #[doc = "The worker associated with the time card.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The pay period associated with the time card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period: Option<PayPeriod>,
    #[doc = "The summary of the time card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub summary: Option<TimeCardSummary>,
}

impl std::fmt::Display for GetTimeCardsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTimeCardsResponse {
    const LENGTH: usize = 8;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period) = &self.pay_period {
                format!("{:?}", pay_period).into()
            } else {
                String::new().into()
            },
            if let Some(summary) = &self.summary {
                format!("{:?}", summary).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "worker_id".into(),
            "worker".into(),
            "pay_period".into(),
            "summary".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTimeEntriesResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<TimeEntry>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListTimeEntriesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListTimeEntriesResponse {
    type Item = TimeEntry;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTimeEntriesResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[doc = "The status of the time entry."]
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
pub enum GetTimeEntriesResponseStatus {
    #[serde(rename = "DRAFT")]
    #[display("DRAFT")]
    Draft,
    #[serde(rename = "APPROVED")]
    #[display("APPROVED")]
    Approved,
    #[serde(rename = "PAID")]
    #[display("PAID")]
    Paid,
    #[serde(rename = "FINALIZED")]
    #[display("FINALIZED")]
    Finalized,
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTimeEntriesResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The ID of the worker associated with the time entry."]
    pub worker_id: String,
    #[doc = "The worker associated with the time entry.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker: Option<Worker>,
    #[doc = "The start time of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[doc = "The end time of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[doc = "The comments associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<TimeEntryComment>>,
    #[doc = "The job shifts worked during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_shifts: Option<Vec<JobShift>>,
    #[doc = "The breaks taken during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breaks: Option<Vec<Break>>,
    #[doc = "The premiums earned during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premiums: Option<Vec<Premiums>>,
    #[doc = "The piece-rate premiums earned during the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub piece_rate_premiums: Option<Vec<PieceRatePremiums>>,
    #[doc = "The pay rates for each segment of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<Segments>>,
    #[doc = "A summary of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_entry_summary: Option<TimeEntrySummary>,
    #[doc = "The ID of the time card associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_card_id: Option<String>,
    #[doc = "The time card associated with the time entry.\n\nExpandable field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_card: Option<TimeCard>,
    #[doc = "The tags associated with the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[doc = "The unique key of the time entry in an outside system. If set, no other time entry \
             with the same key can be created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[doc = "Whether the time entry should create an extra hours run."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_extra_hours_run: Option<bool>,
    #[doc = "The status of the time entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetTimeEntriesResponseStatus>,
    #[doc = "The pay period associated with the time card."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pay_period: Option<PayPeriod>,
    #[doc = "Arbitrary shift inputs collected on the time entry"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shift_input_values: Option<Vec<ShiftInputValue>>,
}

impl std::fmt::Display for GetTimeEntriesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTimeEntriesResponse {
    const LENGTH: usize = 23;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.worker_id.clone().into(),
            if let Some(worker) = &self.worker {
                format!("{:?}", worker).into()
            } else {
                String::new().into()
            },
            if let Some(start_time) = &self.start_time {
                format!("{:?}", start_time).into()
            } else {
                String::new().into()
            },
            if let Some(end_time) = &self.end_time {
                format!("{:?}", end_time).into()
            } else {
                String::new().into()
            },
            if let Some(comments) = &self.comments {
                format!("{:?}", comments).into()
            } else {
                String::new().into()
            },
            if let Some(job_shifts) = &self.job_shifts {
                format!("{:?}", job_shifts).into()
            } else {
                String::new().into()
            },
            if let Some(breaks) = &self.breaks {
                format!("{:?}", breaks).into()
            } else {
                String::new().into()
            },
            if let Some(premiums) = &self.premiums {
                format!("{:?}", premiums).into()
            } else {
                String::new().into()
            },
            if let Some(piece_rate_premiums) = &self.piece_rate_premiums {
                format!("{:?}", piece_rate_premiums).into()
            } else {
                String::new().into()
            },
            if let Some(segments) = &self.segments {
                format!("{:?}", segments).into()
            } else {
                String::new().into()
            },
            if let Some(time_entry_summary) = &self.time_entry_summary {
                format!("{:?}", time_entry_summary).into()
            } else {
                String::new().into()
            },
            if let Some(time_card_id) = &self.time_card_id {
                format!("{:?}", time_card_id).into()
            } else {
                String::new().into()
            },
            if let Some(time_card) = &self.time_card {
                format!("{:?}", time_card).into()
            } else {
                String::new().into()
            },
            if let Some(tags) = &self.tags {
                format!("{:?}", tags).into()
            } else {
                String::new().into()
            },
            if let Some(idempotency_key) = &self.idempotency_key {
                format!("{:?}", idempotency_key).into()
            } else {
                String::new().into()
            },
            if let Some(create_extra_hours_run) = &self.create_extra_hours_run {
                format!("{:?}", create_extra_hours_run).into()
            } else {
                String::new().into()
            },
            if let Some(status) = &self.status {
                format!("{:?}", status).into()
            } else {
                String::new().into()
            },
            if let Some(pay_period) = &self.pay_period {
                format!("{:?}", pay_period).into()
            } else {
                String::new().into()
            },
            if let Some(shift_input_values) = &self.shift_input_values {
                format!("{:?}", shift_input_values).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "worker_id".into(),
            "worker".into(),
            "start_time".into(),
            "end_time".into(),
            "comments".into(),
            "job_shifts".into(),
            "breaks".into(),
            "premiums".into(),
            "piece_rate_premiums".into(),
            "segments".into(),
            "time_entry_summary".into(),
            "time_card_id".into(),
            "time_card".into(),
            "tags".into(),
            "idempotency_key".into(),
            "create_extra_hours_run".into(),
            "status".into(),
            "pay_period".into(),
            "shift_input_values".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListTracksResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Track>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListTracksResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListTracksResponse {
    type Item = Track;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListTracksResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetTracksResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the track. Must be unique within the company or organization."]
    pub name: String,
}

impl std::fmt::Display for GetTracksResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetTracksResponse {
    const LENGTH: usize = 5;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListUsersResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<User>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListUsersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListUsersResponse {
    type Item = User;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListUsersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetUsersResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "Whether the user is able to access company resources, typically when they are in \
             actively engaged with the company and not after off-boarding."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[doc = "The unique identifier across Rippling used by the User for direct authentication \
             into their associated company. Globally unique."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The user's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UserName>,
    #[doc = "The display name of the user using either the concatenated preferred given and \
             family name or username depending on availability."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[doc = "The user's email addresses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<Email>>,
    #[doc = "The user's phone numbers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<UserPhoneNumber>>,
    #[doc = "The user's addresses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<UserAddress>>,
    #[doc = "The user's photos."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<UserPhoto>>,
    #[doc = "The User's preferred written or spoken language in the same format of the HTTP \
             Accept-Language header, pursuant to Section 5.3.5 of RFC7231."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[doc = "The User's default location for purposes of localization of currency, date time \
             format, or numerical representations pursuant to RFC5646."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[doc = "The User's current time zone in IANA database Olson format"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl std::fmt::Display for GetUsersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetUsersResponse {
    const LENGTH: usize = 15;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            if let Some(active) = &self.active {
                format!("{:?}", active).into()
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
            if let Some(display_name) = &self.display_name {
                format!("{:?}", display_name).into()
            } else {
                String::new().into()
            },
            if let Some(emails) = &self.emails {
                format!("{:?}", emails).into()
            } else {
                String::new().into()
            },
            if let Some(phone_numbers) = &self.phone_numbers {
                format!("{:?}", phone_numbers).into()
            } else {
                String::new().into()
            },
            if let Some(addresses) = &self.addresses {
                format!("{:?}", addresses).into()
            } else {
                String::new().into()
            },
            if let Some(photos) = &self.photos {
                format!("{:?}", photos).into()
            } else {
                String::new().into()
            },
            if let Some(preferred_language) = &self.preferred_language {
                format!("{:?}", preferred_language).into()
            } else {
                String::new().into()
            },
            if let Some(locale) = &self.locale {
                format!("{:?}", locale).into()
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
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "active".into(),
            "username".into(),
            "name".into(),
            "display_name".into(),
            "emails".into(),
            "phone_numbers".into(),
            "addresses".into(),
            "photos".into(),
            "preferred_language".into(),
            "locale".into(),
            "timezone".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListWorkLocationsResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<WorkLocation>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListWorkLocationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListWorkLocationsResponse {
    type Item = WorkLocation;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListWorkLocationsResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetWorkLocationsResponse {
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
    #[doc = "Identifier field"]
    pub id: String,
    #[doc = "Record creation date"]
    pub created_at: String,
    #[doc = "Record update date"]
    pub updated_at: String,
    #[doc = "The name of the work location."]
    pub name: String,
    #[doc = "The address for the work location."]
    pub address: Address,
}

impl std::fmt::Display for GetWorkLocationsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetWorkLocationsResponse {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            self.id.clone().into(),
            self.created_at.clone().into(),
            self.updated_at.clone().into(),
            self.name.clone().into(),
            format!("{:?}", self.address).into(),
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "meta".into(),
            "id".into(),
            "created_at".into(),
            "updated_at".into(),
            "name".into(),
            "address".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListWorkersResponse {
    #[doc = "A list of redacted fields."]
    #[serde(rename = "__meta", default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<RedactedFields>,
    pub results: Vec<Worker>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListWorkersResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListWorkersResponse {
    type Item = Worker;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListWorkersResponse {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(meta) = &self.meta {
                format!("{:?}", meta).into()
            } else {
                String::new().into()
            },
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["meta".into(), "results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCustomObjectsCustomObjectApiNameFieldsResponse {
    pub results: Vec<CustomObjectField>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCustomObjectsCustomObjectApiNameFieldsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCustomObjectsCustomObjectApiNameFieldsResponse {
    type Item = CustomObjectField;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCustomObjectsCustomObjectApiNameFieldsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCustomObjectsCustomObjectApiNameFieldsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_history: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_field_formula: Option<String>,
}

impl std::fmt::Display for CreateCustomObjectsCustomObjectApiNameFieldsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateCustomObjectsCustomObjectApiNameFieldsRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(required) = &self.required {
                format!("{:?}", required).into()
            } else {
                String::new().into()
            },
            if let Some(is_unique) = &self.is_unique {
                format!("{:?}", is_unique).into()
            } else {
                String::new().into()
            },
            if let Some(enable_history) = &self.enable_history {
                format!("{:?}", enable_history).into()
            } else {
                String::new().into()
            },
            if let Some(derived_field_formula) = &self.derived_field_formula {
                format!("{:?}", derived_field_formula).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "description".into(),
            "required".into(),
            "is_unique".into(),
            "enable_history".into(),
            "derived_field_formula".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCustomObjectsCustomObjectApiNameFieldsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_unique: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_history: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derived_field_formula: Option<String>,
}

impl std::fmt::Display for UpdateCustomObjectsCustomObjectApiNameFieldsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCustomObjectsCustomObjectApiNameFieldsRequestBody {
    const LENGTH: usize = 6;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
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
            if let Some(required) = &self.required {
                format!("{:?}", required).into()
            } else {
                String::new().into()
            },
            if let Some(is_unique) = &self.is_unique {
                format!("{:?}", is_unique).into()
            } else {
                String::new().into()
            },
            if let Some(enable_history) = &self.enable_history {
                format!("{:?}", enable_history).into()
            } else {
                String::new().into()
            },
            if let Some(derived_field_formula) = &self.derived_field_formula {
                format!("{:?}", derived_field_formula).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "description".into(),
            "required".into(),
            "is_unique".into(),
            "enable_history".into(),
            "derived_field_formula".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct Results {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compnay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
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
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(compnay_id) = &self.compnay_id {
                format!("{:?}", compnay_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(custom_object) = &self.custom_object {
                format!("{:?}", custom_object).into()
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
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "compnay_id".into(),
            "created_at".into(),
            "created_by".into(),
            "custom_object".into(),
            "external_id".into(),
            "id".into(),
            "last_modified_by".into(),
            "updated_at".into(),
            "owner_role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListCustomObjectsCustomObjectApiNameRecordsResponse {
    pub results: Vec<Results>,
    #[doc = "A link to the next page of responses."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}

impl std::fmt::Display for ListCustomObjectsCustomObjectApiNameRecordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "requests")]
impl crate::types::paginate::Pagination for ListCustomObjectsCustomObjectApiNameRecordsResponse {
    type Item = Results;
    fn has_more_pages(&self) -> bool {
        self.next_link.is_some()
    }

    fn next_page_token(&self) -> Option<String> {
        self.next_link.clone()
    }

    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> anyhow::Result<reqwest::Request, crate::types::error::Error> {
        let mut req = req.try_clone().ok_or_else(|| {
            crate::types::error::Error::InvalidRequest(format!(
                "failed to clone request: {:?}",
                req
            ))
        })?;
        *req.url_mut() =
            url::Url::parse(self.next_link.as_deref().unwrap_or("")).map_err(|_| {
                crate::types::error::Error::InvalidRequest(format!(
                    "failed to parse url: {:?}",
                    self.next_link
                ))
            })?;
        Ok(req)
    }

    fn items(&self) -> Vec<Self::Item> {
        self.results.clone()
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListCustomObjectsCustomObjectApiNameRecordsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.results).into(),
            if let Some(next_link) = &self.next_link {
                format!("{:?}", next_link).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into(), "next_link".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_api_name: Option<String>,
}

impl std::fmt::Display for CreateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(field_api_name) = &self.field_api_name {
                format!("{:?}", field_api_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "field_api_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct CreateCustomObjectsCustomObjectApiNameRecordsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compnay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
}

impl std::fmt::Display for CreateCustomObjectsCustomObjectApiNameRecordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for CreateCustomObjectsCustomObjectApiNameRecordsResponse {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(compnay_id) = &self.compnay_id {
                format!("{:?}", compnay_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(custom_object) = &self.custom_object {
                format!("{:?}", custom_object).into()
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
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "compnay_id".into(),
            "created_at".into(),
            "created_by".into(),
            "custom_object".into(),
            "external_id".into(),
            "id".into(),
            "last_modified_by".into(),
            "updated_at".into(),
            "owner_role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListByQueryCustomObjectsCustomObjectApiNameRecordsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl std::fmt::Display for ListByQueryCustomObjectsCustomObjectApiNameRecordsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListByQueryCustomObjectsCustomObjectApiNameRecordsRequestBody {
    const LENGTH: usize = 3;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(query) = &self.query {
                format!("{:?}", query).into()
            } else {
                String::new().into()
            },
            if let Some(limit) = &self.limit {
                format!("{:?}", limit).into()
            } else {
                String::new().into()
            },
            if let Some(cursor) = &self.cursor {
                format!("{:?}", cursor).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["query".into(), "limit".into(), "cursor".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct ListByQueryCustomObjectsCustomObjectApiNameRecordsResponse {
    pub results: Vec<Results>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl std::fmt::Display for ListByQueryCustomObjectsCustomObjectApiNameRecordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for ListByQueryCustomObjectsCustomObjectApiNameRecordsResponse {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            format!("{:?}", self.results).into(),
            if let Some(cursor) = &self.cursor {
                format!("{:?}", cursor).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["results".into(), "cursor".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCustomObjectsCustomObjectApiNameRecordsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compnay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
}

impl std::fmt::Display for GetCustomObjectsCustomObjectApiNameRecordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCustomObjectsCustomObjectApiNameRecordsResponse {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(compnay_id) = &self.compnay_id {
                format!("{:?}", compnay_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(custom_object) = &self.custom_object {
                format!("{:?}", custom_object).into()
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
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "compnay_id".into(),
            "created_at".into(),
            "created_by".into(),
            "custom_object".into(),
            "external_id".into(),
            "id".into(),
            "last_modified_by".into(),
            "updated_at".into(),
            "owner_role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_api_name: Option<String>,
}

impl std::fmt::Display for UpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(field_api_name) = &self.field_api_name {
                format!("{:?}", field_api_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "field_api_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct UpdateCustomObjectsCustomObjectApiNameRecordsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compnay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
}

impl std::fmt::Display for UpdateCustomObjectsCustomObjectApiNameRecordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for UpdateCustomObjectsCustomObjectApiNameRecordsResponse {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(compnay_id) = &self.compnay_id {
                format!("{:?}", compnay_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(custom_object) = &self.custom_object {
                format!("{:?}", custom_object).into()
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
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "compnay_id".into(),
            "created_at".into(),
            "created_by".into(),
            "custom_object".into(),
            "external_id".into(),
            "id".into(),
            "last_modified_by".into(),
            "updated_at".into(),
            "owner_role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct GetCustomObjectsCustomObjectApiNameRecordsByExternalIdResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compnay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
}

impl std::fmt::Display for GetCustomObjectsCustomObjectApiNameRecordsByExternalIdResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for GetCustomObjectsCustomObjectApiNameRecordsByExternalIdResponse {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(compnay_id) = &self.compnay_id {
                format!("{:?}", compnay_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(custom_object) = &self.custom_object {
                format!("{:?}", custom_object).into()
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
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "compnay_id".into(),
            "created_at".into(),
            "created_by".into(),
            "custom_object".into(),
            "external_id".into(),
            "id".into(),
            "last_modified_by".into(),
            "updated_at".into(),
            "owner_role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RowsToWrite {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_api_name: Option<String>,
}

impl std::fmt::Display for RowsToWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RowsToWrite {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(field_api_name) = &self.field_api_name {
                format!("{:?}", field_api_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "field_api_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BulkCreateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_to_write: Option<Vec<RowsToWrite>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_or_nothing: Option<bool>,
}

impl std::fmt::Display for BulkCreateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkCreateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(rows_to_write) = &self.rows_to_write {
                format!("{:?}", rows_to_write).into()
            } else {
                String::new().into()
            },
            if let Some(all_or_nothing) = &self.all_or_nothing {
                format!("{:?}", all_or_nothing).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["rows_to_write".into(), "all_or_nothing".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BulkCreateCustomObjectsCustomObjectApiNameRecordsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compnay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
}

impl std::fmt::Display for BulkCreateCustomObjectsCustomObjectApiNameRecordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkCreateCustomObjectsCustomObjectApiNameRecordsResponse {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(compnay_id) = &self.compnay_id {
                format!("{:?}", compnay_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(custom_object) = &self.custom_object {
                format!("{:?}", custom_object).into()
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
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "compnay_id".into(),
            "created_at".into(),
            "created_by".into(),
            "custom_object".into(),
            "external_id".into(),
            "id".into(),
            "last_modified_by".into(),
            "updated_at".into(),
            "owner_role".into(),
        ]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BulkDeleteCustomObjectsCustomObjectApiNameRecordsRequestBody {
    #[doc = "a list of ids, e.g. [id_1, id_2]."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_to_delete: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_or_nothing: Option<bool>,
}

impl std::fmt::Display for BulkDeleteCustomObjectsCustomObjectApiNameRecordsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkDeleteCustomObjectsCustomObjectApiNameRecordsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(rows_to_delete) = &self.rows_to_delete {
                format!("{:?}", rows_to_delete).into()
            } else {
                String::new().into()
            },
            if let Some(all_or_nothing) = &self.all_or_nothing {
                format!("{:?}", all_or_nothing).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["rows_to_delete".into(), "all_or_nothing".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct RowsToUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_api_name: Option<String>,
}

impl std::fmt::Display for RowsToUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for RowsToUpdate {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(field_api_name) = &self.field_api_name {
                format!("{:?}", field_api_name).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["name".into(), "field_api_name".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BulkUpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rows_to_update: Option<Vec<RowsToUpdate>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_or_nothing: Option<bool>,
}

impl std::fmt::Display for BulkUpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkUpdateCustomObjectsCustomObjectApiNameRecordsRequestBody {
    const LENGTH: usize = 2;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(rows_to_update) = &self.rows_to_update {
                format!("{:?}", rows_to_update).into()
            } else {
                String::new().into()
            },
            if let Some(all_or_nothing) = &self.all_or_nothing {
                format!("{:?}", all_or_nothing).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec!["rows_to_update".into(), "all_or_nothing".into()]
    }
}

#[derive(
    serde :: Serialize, serde :: Deserialize, PartialEq, Debug, Clone, schemars :: JsonSchema,
)]
pub struct BulkUpdateCustomObjectsCustomObjectApiNameRecordsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compnay_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_object: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_role: Option<String>,
}

impl std::fmt::Display for BulkUpdateCustomObjectsCustomObjectApiNameRecordsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self).map_err(|_| std::fmt::Error)?
        )
    }
}

#[cfg(feature = "tabled")]
impl tabled::Tabled for BulkUpdateCustomObjectsCustomObjectApiNameRecordsResponse {
    const LENGTH: usize = 10;
    fn fields(&self) -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            if let Some(name) = &self.name {
                format!("{:?}", name).into()
            } else {
                String::new().into()
            },
            if let Some(compnay_id) = &self.compnay_id {
                format!("{:?}", compnay_id).into()
            } else {
                String::new().into()
            },
            if let Some(created_at) = &self.created_at {
                format!("{:?}", created_at).into()
            } else {
                String::new().into()
            },
            if let Some(created_by) = &self.created_by {
                format!("{:?}", created_by).into()
            } else {
                String::new().into()
            },
            if let Some(custom_object) = &self.custom_object {
                format!("{:?}", custom_object).into()
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
            if let Some(last_modified_by) = &self.last_modified_by {
                format!("{:?}", last_modified_by).into()
            } else {
                String::new().into()
            },
            if let Some(updated_at) = &self.updated_at {
                format!("{:?}", updated_at).into()
            } else {
                String::new().into()
            },
            if let Some(owner_role) = &self.owner_role {
                format!("{:?}", owner_role).into()
            } else {
                String::new().into()
            },
        ]
    }

    fn headers() -> Vec<std::borrow::Cow<'static, str>> {
        vec![
            "name".into(),
            "compnay_id".into(),
            "created_at".into(),
            "created_by".into(),
            "custom_object".into(),
            "external_id".into(),
            "id".into(),
            "last_modified_by".into(),
            "updated_at".into(),
            "owner_role".into(),
        ]
    }
}
