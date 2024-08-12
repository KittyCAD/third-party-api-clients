//! Types for our Pagerduty API client.

use chrono::{DateTime, Utc};
use parse_display::{Display, FromStr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A list of services.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ServiceListResponse {
    /// The list of services.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub services: Vec<Service>,
    /// Echoes limit pagination property.
    #[serde(default)]
    pub limit: i64,
    /// Echoes offset pagination property.
    #[serde(default)]
    pub offset: i64,
    /// Indicates if there are additional records to return.
    #[serde(default)]
    pub more: bool,
}

/// A service object.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ServiceObject {
    /// The service.
    #[serde(default)]
    pub service: Service,
}

/// A service.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Service {
    /// The ID.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub id: String,
    /// The name of the service.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub name: String,
    /// The description of the service.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub description: String,
    /// Time in seconds that an incident is automatically resolved if left open for that long. Value is null if the feature is disabled. Value must not be negative. Setting this field to 0, null (or unset in POST request) will disable the feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_resolve_timeout: Option<i64>,
    /// Time in seconds that an incident changes to the Triggered State after being Acknowledged. Value is null if the feature is disabled. Value must not be negative. Setting this field to 0, null (or unset in POST request) will disable the feature.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledgement_timeout: Option<i64>,
    /// The current state of the Service. Valid statuses are:
    ///
    /// - `active`: The service is enabled and has no open incidents. This is the only status a service can be created with.
    /// - `warning`: The service is enabled and has one or more acknowledged incidents.
    /// - `critical`: The service is enabled and has one or more triggered incidents.
    /// - `maintenance`: The service is under maintenance, no new incidents will be triggered during maintenance mode.
    /// - `disabled`: The service is disabled and will not have any new triggered incidents.
    #[serde(default)]
    pub status: Status,
    /// Escalation policy.
    #[serde(default)]
    pub escalation_policy: EscalationPolicy,
    /// Incident urgency rule.
    #[serde(default)]
    pub incident_urgency_rule: IncidentUrgencyRule,
    /// Support Hours.
    #[serde(default)]
    pub support_hours: Option<SupportHours>,
    /// An array containing scheduled actions for the service.
    #[serde(default)]
    pub scheduled_actions: Vec<ScheduledAction>,
    /// Whether a service creates only incidents, or both alerts and incidents. A service must create alerts in order to enable incident merging.
    ///
    /// - `create_incidents` - The service will create one incident and zero alerts for each incoming event.
    /// - `create_alerts_and_incidents` - The service will create one incident and one associated alert for each incoming event.
    #[serde(default)]
    pub alert_creation: AlertCreation,
    /// Defines how alerts on this service will be automatically grouped into incidents. Note that the alert grouping features are available only on certain plans.
    #[serde(default)]
    pub alert_grouping_parameters: AlertGroupingParameters,
    /// Defines how alerts on this service are automatically suspended for a period of time before triggering, when identified as likely being transient. Note that automatically pausing notifications is only available on certain plans.
    #[serde(default)]
    pub auto_pause_notifications_parameters: AutoPauseNotificationsParameters,

    /// An API link to itself.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
    /// An HTML link to itself.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub html_url: String,
    /// The date/time when this service was created.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<DateTime<Utc>>,
    /// The date/time when the most recent incident was created for this service.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_incident_timestamp: Option<DateTime<Utc>>,
}

/// The state of the service.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Status {
    /// Active.
    #[default]
    Active,
    /// Warning.
    Warning,
    /// Critical.
    Critical,
    /// Maintenance.
    Maintenance,
    /// Disabled.
    Disabled,
}

/// How a service creates incidents.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AlertCreation {
    /// Create incidents.
    CreateIncidents,
    /// Create alerts and incidents.
    #[default]
    CreateAlertsAndIncidents,
}

/// An escalation policy.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EscalationPolicy {
    /// The ID.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub id: String,
    /// The type.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<EscalationPolicyType>,
    /// The summary.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub summary: String,
    /// The name.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub name: String,
    /// The description.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub description: String,
    /// An API link to itself.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize",
        rename = "self"
    )]
    pub self_: String,
    /// An HTML link to itself.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub html_url: String,
}

/// The type of escalation_policy.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EscalationPolicyType {
    /// A reference to an escalation policy.
    #[default]
    EscalationPolicyReference,
    /// An escalation policy object.
    EscalationPolicy,
}

/// An incident urgency rule.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IncidentUrgencyRule {
    /// The type.
    #[serde(default, rename = "type")]
    pub type_: IncidentUrgencyRuleType,
    /// The urgency.
    #[serde(default)]
    pub urgency: Option<Urgency>,
    /// The rule to use during support hours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub during_support_hours: Option<Box<IncidentUrgencyRule>>,
    /// The rule to use outside support hours.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outside_support_hours: Option<Box<IncidentUrgencyRule>>,
}

impl Default for IncidentUrgencyRule {
    fn default() -> Self {
        IncidentUrgencyRule {
            type_: IncidentUrgencyRuleType::Constant,
            urgency: Some(Urgency::High),
            during_support_hours: None,
            outside_support_hours: None,
        }
    }
}

/// The type of incident urgency rule.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum IncidentUrgencyRuleType {
    /// Constant.
    #[default]
    Constant,
    /// Use support hours.
    UseSupportHours,
}

/// The incidents' urgency, if type is constant.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum Urgency {
    /// Low.
    Low,
    /// High.
    #[default]
    High,
    /// Severity based.
    SeverityBased,
}

/// The support hours.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SupportHours {
    /// The type.
    #[serde(default, rename = "type")]
    pub type_: SupportHoursType,
    /// The time zone for the support hours.
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_null_string::deserialize"
    )]
    pub time_zone: String,
    /// The support hours' starting time of day (date portion is ignored).
    #[serde(
        deserialize_with = "serde_naive_time::deserialize",
        serialize_with = "serde_naive_time::serialize"
    )]
    pub start_time: chrono::NaiveTime,
    /// The support hours' ending time of day (date portion is ignored).
    #[serde(
        deserialize_with = "serde_naive_time::deserialize",
        serialize_with = "serde_naive_time::serialize"
    )]
    pub end_time: chrono::NaiveTime,
    /// The days of the week.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub days_of_week: Vec<i32>,
}

/// The type of support hours.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SupportHoursType {
    /// Fixed time per day.
    #[default]
    FixedTimePerDay,
}

/// A scheduled action.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScheduledAction {
    /// The type.
    #[serde(default, rename = "type")]
    pub type_: ScheduledActionType,
    /// Represents when scheduled action will occur.
    #[serde(default)]
    pub at: ScheduledActionAt,
    /// Urgency level. Must be set to high.
    #[serde(default)]
    pub to_urgency: Urgency,
}

/// The type of scheduled action.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ScheduledActionType {
    /// Urgency change.
    #[default]
    UrgencyChange,
}

/// Represents when scheduled action will occur.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScheduledActionAt {
    /// The type.
    #[serde(default, rename = "type")]
    pub type_: ScheduledActionAtType,
    /// Designates either the start or the end of support hours.
    #[serde(default)]
    pub name: ScheduledActionAtName,
}

/// The type of scheduled action at.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ScheduledActionAtType {
    /// Named time.
    #[default]
    NamedTime,
}

/// Designates either the start or the end of support hours.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ScheduledActionAtName {
    /// Support hours start.
    #[default]
    SupportHoursStart,
    /// Support hours end.
    SupportHoursEnd,
}

/// Defines how alerts on this service will be automatically grouped into incidents. Note that the alert grouping features are available only on certain plans.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AlertGroupingParameters {
    /// The type.
    #[serde(default, rename = "type")]
    pub type_: Option<AlertGroupingType>,
}

/// The type of alert grouping.
#[derive(
    Display, Default, FromStr, PartialEq, Debug, JsonSchema, Deserialize, Serialize, Clone,
)]
#[display(style = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum AlertGroupingType {
    /// Time.
    Time,
    /// Intelligent.
    #[default]
    Intelligent,
    /// Content based.
    ContentBased,
}

/// Defines how alerts on this service are automatically suspended for a period of time before triggering, when identified as likely being transient. Note that automatically pausing notifications is only available on certain plans.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AutoPauseNotificationsParameters {
    /// Indicates whether alerts should be automatically suspended when identified as transient.
    #[serde(default)]
    pub enabled: bool,
    /// Indicates in seconds how long alerts should be suspended before triggering.
    /// Allowed values: 120, 180, 300, 600, 900
    #[serde(default)]
    pub timeout: Option<i64>,
}

/// A list of escalation policies.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EscalationPolicyListResponse {
    /// The list of services.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub escalation_policies: Vec<EscalationPolicy>,
    /// Echoes limit pagination property.
    #[serde(default)]
    pub limit: i64,
    /// Echoes offset pagination property.
    #[serde(default)]
    pub offset: i64,
    /// Indicates if there are additional records to return.
    #[serde(default)]
    pub more: bool,
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

/// A module for deserializing a null string.
pub mod serde_naive_time {
    use chrono::NaiveTime;
    use serde::{Deserialize, Serializer};

    /// Deserialize a time to the right format.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = NaiveTime::deserialize(deserializer)?;

        Ok(s)
    }

    /// Serialize a time to the right format.
    pub fn serialize<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", time.format("%H:%M:%S"));
        serializer.serialize_str(&s)
    }
}
