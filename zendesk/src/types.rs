//! Types for our Zendesk API client.

use chrono::{DateTime, Utc};
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A list of contacts returned from an API call.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ContactsListResponse {
    /// A list of contact data.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ContactData>,
}

/// A nested contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ContactData {
    /// The contact data.
    pub data: Contact,
    /// Metadata.
    #[serde(default, skip_serializing_if = "Meta::is_default")]
    pub meta: Meta,
}

/// Nested metadata.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Meta {
    /// The metadata type.
    #[serde(
        default,
        rename = "type",
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub type_: String,
}

impl Meta {
    /// Returns true if this is the default struct.
    pub fn is_default(&self) -> bool {
        self == &Meta::default()
    }
}

/// A contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Contact {
    /// The ID.
    #[serde(default)]
    pub id: i64,
    /// The ID of the creator.
    #[serde(default)]
    pub creator_id: i64,
    /// The ID of the owner.
    #[serde(default)]
    pub owner_id: i64,
    /// If this is an organization.
    #[serde(default)]
    pub is_organization: bool,
    /// The contact ID.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub contact_id: String,
    /// The parent organization ID.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_organization_id: Option<i64>,
    /// The name.
    /// This field will be set only if the contact is an organization. `is_organization` is set to true.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub name: String,
    /// The first name.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /// The last name.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /// The customer's status.
    #[serde(default)]
    pub customer_status: CustomerStatus,
    /// The prospect's status.
    #[serde(default)]
    pub prospect_status: ProspectStatus,
    /// The title.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub title: String,
    /// The description.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub description: String,
    /// The industry.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub industry: String,
    /// The website.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub website: String,
    /// The email address.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub email: String,
    /// The phone number.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /// The mobile number.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub mobile: String,
    /// The fax number.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub fax: String,
    /// The twitter handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub twitter: String,
    /// The facebook handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub facebook: String,
    /// The linkedin handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub linkedin: String,
    /// The skype handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub skype: String,
    /// The address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// The billing address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    /// The shipping address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    /// The tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// The custom fields.
    #[serde(default, skip_serializing_if = "CustomFields::is_default")]
    pub custom_fields: CustomFields,
    /// When the contact was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// When the contact was updated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

/// An address.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct Address {
    /// The address line 1.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub line1: String,
    /// The address city.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub city: String,
    /// The postal code.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub postal_code: String,
    /// The address state.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub state: String,
    /// The address country.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub country: String,
}

impl Address {
    /// Returns true if the address is the default struct.
    pub fn is_default(&self) -> bool {
        self == &Address::default()
    }
}

/// A list of custom fields.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct CustomFields {
    /// How this contact is known.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub known_via: String,
    /// The contact's Stripe ID.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub stripe_id: String,
    /// The contact's KittyCAD ID.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub kittycad_id: String,
}

impl CustomFields {
    /// Returns true if this is the default struct.
    pub fn is_default(&self) -> bool {
        self == &CustomFields::default()
    }
}

/// The status of the customer.
#[derive(Debug, Display, FromStr, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[display(style = "snake_case")]
pub enum CustomerStatus {
    /// None.
    #[serde(rename = "none")]
    #[display("none")]
    Empty,
    /// Current.
    Current,
    /// Past.
    Past,
}

impl Default for CustomerStatus {
    fn default() -> Self {
        Self::Empty
    }
}

/// The status of the prospect.
#[derive(Debug, Display, FromStr, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[display(style = "snake_case")]
pub enum ProspectStatus {
    /// None.
    #[serde(rename = "none")]
    #[display("none")]
    Empty,
    /// Current.
    Current,
    /// Lost.
    Lost,
}

impl Default for ProspectStatus {
    fn default() -> Self {
        Self::Empty
    }
}

/// A nested new contact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NewContactData {
    /// The new contact data.
    pub data: NewContact,
    /// Metadata.
    #[serde(default, skip_serializing_if = "Meta::is_default")]
    pub meta: Meta,
}

/// A new contact.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NewContact {
    /// The name.
    /// This field will be set only if the contact is an organization. `is_organization` is set to true.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub name: String,
    /// The first name.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub first_name: String,
    /// The last name.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub last_name: String,
    /// The customer's status.
    #[serde(default)]
    pub customer_status: CustomerStatus,
    /// The prospect's status.
    #[serde(default)]
    pub prospect_status: ProspectStatus,
    /// The title.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub title: String,
    /// The description.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub description: String,
    /// The industry.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub industry: String,
    /// The website.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub website: String,
    /// The email address.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub email: String,
    /// The phone number.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub phone: String,
    /// The mobile number.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub mobile: String,
    /// The fax number.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub fax: String,
    /// The twitter handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub twitter: String,
    /// The facebook handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub facebook: String,
    /// The linkedin handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub linkedin: String,
    /// The skype handle.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub skype: String,
    /// The address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    /// The billing address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<Address>,
    /// The shipping address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Address>,
    /// The tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// The custom fields.
    #[serde(default, skip_serializing_if = "CustomFields::is_default")]
    pub custom_fields: CustomFields,
}

/// A module for deserializing a null string.
pub mod deserialize_null_string {
    use serde::Deserialize;

    /// Deserialize a null string as an empty string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).unwrap_or_default();

        Ok(s)
    }
}
