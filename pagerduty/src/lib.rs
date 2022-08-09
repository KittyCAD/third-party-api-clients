//! A fully generated & opinionated API client for the PagerDuty API.
//!
//! [![docs.rs](https://docs.rs/pagerduty-api/badge.svg)](https://docs.rs/pagerduty-api)
//!
//! ## API Details
//!
//! This document describes the PagerDuty REST APIs.
//! 
//! For guides and examples please visit our [Documentation.](https://developer.pagerduty.com/docs/get-started/getting-started/)
//! 
//! Our REST APIs are defined in OpenAPI v3.x. You can view the schema at [github.com/PagerDuty/api-schema](https://github.com/PagerDuty/api-schema).
//! 
//! Note that properties in some schemas have fields not shown by default such as `readOnly`, `format`, and `default`. Hover your cursor over the right column that looks like `optional+1` to see the full list of fields.
//! 
//!
//! 
//!
//! ### Contact
//!
//! 
//! | name | url | email |
//! |----|----|----|
//! | PagerDuty Support | <http://www.pagerduty.com/support> | support@pagerduty.com |
//! 
//! 
//!
//! ## Client Details
//!
//! 
//!
//! The documentation for the crate is generated
//! along with the code to make this library easy to use.
//! 
//!
//! To install the library, add the following to your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! pagerduty-api = "0.0.1"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use pagerduty_api::Client;
//!
//! let client = Client::new(
//!     String::from("api-key"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `PAGERDUTY_API_TOKEN`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use pagerduty_api::Client;
//!
//! let client = Client::new_from_env();
//! ```
//!
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
mod tests;
pub mod types;
#[doc(hidden)]
/// This describes your account's abilities by feature name. For example `"teams"`.
/// An ability may be available to your account based on things like your pricing plan or account state.
/// .
pub mod abilities;
/// Developers can write their own functionality to insert into PagerDuty's UI.
/// .
pub mod add_ons;
/// Provides enriched incident data.
/// .
pub mod analytics;
/// Provides audit record data.
/// .
pub mod audit;
/// Provides paused Incident reporting data on services and accounts that have paused Alerts.
/// .
pub mod paused_incident_reports;
/// Business services model capabilities that span multiple technical services and that may be owned by several different teams.
/// .
pub mod business_services;
/// Change Events enable you to send informational events about recent changes such as code deploys and system config changes from any system that can make an outbound HTTP connection. These events do not create incidents and do not send notifications; they are shown in context with incidents on the same PagerDuty service.
/// .
pub mod change_events;
/// Escalation policies define which user should be alerted at which time.
/// .
pub mod escalation_policies;
/// Event Orchestrations allow you to route events to an endpoint and create collections of Event Orchestrations, which define sets of actions to take based on event content.
/// .
pub mod event_orchestrations;
/// A PagerDuty extension vendor represents a specific type of outbound extension such as Generic Webhook, Slack, ServiceNow.
/// .
pub mod extension_schemas;
/// Extensions are representations of Extension Schema objects that are attached to Services.
/// .
pub mod extensions;
/// An incident represents a problem or an issue that needs to be addressed and resolved. Incidents trigger on a service, which prompts notifications to go out to on-call responders per the service's escalation policy.
/// .
pub mod incidents;
/// A log of all the events that happen to an Incident, and these are exposed as Log Entries.
/// .
pub mod log_entries;
/// A Maintenance Window is used to temporarily disable one or more Services for a set period of time.
/// .
pub mod maintenance_windows;
/// A Notification is created when an Incident is triggered or escalated.
/// .
pub mod notifications;
/// An on-call represents a contiguous unit of time for which a User will be on call for a given Escalation Policy and Escalation Rules
/// .
pub mod on_calls;
/// A priority is a label representing the importance and impact of an incident. This feature is only available on Standard and Enterprise plans.
/// .
pub mod priorities;
/// Response Plays are a package of Incident Actions that can be applied during an Incident's life cycle.
/// .
pub mod response_plays;
/// Rulesets allow you to route events to an endpoint and create collections of Event Rules, which define sets of actions to take based on event content.
/// .
pub mod rulesets;
/// A Schedule determines the time periods that users are On-Call.
/// .
pub mod schedules;
/// Services are categorized into technical and business services. Dependencies can be created via any combination of these services.
/// .
pub mod service_dependencies;
/// A Service may represent an application, component, or team you wish to open incidents against.
/// .
pub mod services;
/// A webhook is a way to receive events that occur on the PagerDuty platform via an HTTP POST request.
/// V3 webhooks are set up by creating a webhook subscription.
/// .
pub mod webhooks;
/// Status Dashboards represent user-defined views for the Status Dashboard product that are limited to specific Business Services rather than the whole set of top-level Business Services (those with no dependent Services).
/// .
pub mod status_dashboards;
/// A Tag is applied to Escalation Policies, Teams or Users and can be used to filter them.
/// .
pub mod tags;
/// A team is a collection of Users and Escalation Policies that represent a group of people within an organization.
/// .
pub mod teams;
/// Users are members of a PagerDuty account that have the ability to interact with Incidents and other data on the account.
/// .
pub mod users;
/// A PagerDuty Vendor represents a specific type of integration. AWS Cloudwatch, Splunk, Datadog are all examples of vendors
/// .
pub mod vendors;


use std::env;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    ".rs/",
    env!("CARGO_PKG_VERSION"),
);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
pub struct Client {
    token: String,
    base_url: String,

    client: reqwest_middleware::ClientWithMiddleware,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument]
    pub fn new<T>(
        token: T,
    ) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        // Retry up to 3 times with increasing intervals between attempts.
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build();
        match client {
            Ok(c) => {
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware)
                    // Retry failed requests.
                    .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                        reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                        |req: &reqwest::Request| req.try_clone().is_some(),
                    ))
                    .build();

                Client {
                    token: token.to_string(),
                    base_url: "https://api.pagerduty.com".to_string(),

                    client,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the base URL for the client to something other than the default: <https://api.pagerduty.com>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `PAGERDUTY_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self
    {
        let token = env::var("PAGERDUTY_API_TOKEN").expect("must set PAGERDUTY_API_TOKEN");

        Client::new(
            token,
        )
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<reqwest_middleware::RequestBuilder>
    {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(
            method,
            &u,
        );

        // Add in our authentication.
        req = req.bearer_auth(&self.token);

        // Set the default headers.
        req = req.header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        req = req.header(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(body) = body {
            req = req.body(body);
        }

        Ok(req)
    }


/// This describes your account's abilities by feature name. For example `"teams"`.
/// An ability may be available to your account based on things like your pricing plan or account state.
/// .
               pub fn abilities(&self) -> abilities::Abilities {
                    abilities::Abilities::new(self.clone())
               }

/// Developers can write their own functionality to insert into PagerDuty's UI.
/// .
               pub fn add_ons(&self) -> add_ons::AddOns {
                    add_ons::AddOns::new(self.clone())
               }

/// Provides enriched incident data.
/// .
               pub fn analytics(&self) -> analytics::Analytics {
                    analytics::Analytics::new(self.clone())
               }

/// Provides audit record data.
/// .
               pub fn audit(&self) -> audit::Audit {
                    audit::Audit::new(self.clone())
               }

/// Provides paused Incident reporting data on services and accounts that have paused Alerts.
/// .
               pub fn paused_incident_reports(&self) -> paused_incident_reports::PausedIncidentReports {
                    paused_incident_reports::PausedIncidentReports::new(self.clone())
               }

/// Business services model capabilities that span multiple technical services and that may be owned by several different teams.
/// .
               pub fn business_services(&self) -> business_services::BusinessServices {
                    business_services::BusinessServices::new(self.clone())
               }

/// Change Events enable you to send informational events about recent changes such as code deploys and system config changes from any system that can make an outbound HTTP connection. These events do not create incidents and do not send notifications; they are shown in context with incidents on the same PagerDuty service.
/// .
               pub fn change_events(&self) -> change_events::ChangeEvents {
                    change_events::ChangeEvents::new(self.clone())
               }

/// Escalation policies define which user should be alerted at which time.
/// .
               pub fn escalation_policies(&self) -> escalation_policies::EscalationPolicies {
                    escalation_policies::EscalationPolicies::new(self.clone())
               }

/// Event Orchestrations allow you to route events to an endpoint and create collections of Event Orchestrations, which define sets of actions to take based on event content.
/// .
               pub fn event_orchestrations(&self) -> event_orchestrations::EventOrchestrations {
                    event_orchestrations::EventOrchestrations::new(self.clone())
               }

/// A PagerDuty extension vendor represents a specific type of outbound extension such as Generic Webhook, Slack, ServiceNow.
/// .
               pub fn extension_schemas(&self) -> extension_schemas::ExtensionSchemas {
                    extension_schemas::ExtensionSchemas::new(self.clone())
               }

/// Extensions are representations of Extension Schema objects that are attached to Services.
/// .
               pub fn extensions(&self) -> extensions::Extensions {
                    extensions::Extensions::new(self.clone())
               }

/// An incident represents a problem or an issue that needs to be addressed and resolved. Incidents trigger on a service, which prompts notifications to go out to on-call responders per the service's escalation policy.
/// .
               pub fn incidents(&self) -> incidents::Incidents {
                    incidents::Incidents::new(self.clone())
               }

/// A log of all the events that happen to an Incident, and these are exposed as Log Entries.
/// .
               pub fn log_entries(&self) -> log_entries::LogEntries {
                    log_entries::LogEntries::new(self.clone())
               }

/// A Maintenance Window is used to temporarily disable one or more Services for a set period of time.
/// .
               pub fn maintenance_windows(&self) -> maintenance_windows::MaintenanceWindows {
                    maintenance_windows::MaintenanceWindows::new(self.clone())
               }

/// A Notification is created when an Incident is triggered or escalated.
/// .
               pub fn notifications(&self) -> notifications::Notifications {
                    notifications::Notifications::new(self.clone())
               }

/// An on-call represents a contiguous unit of time for which a User will be on call for a given Escalation Policy and Escalation Rules
/// .
               pub fn on_calls(&self) -> on_calls::OnCalls {
                    on_calls::OnCalls::new(self.clone())
               }

/// A priority is a label representing the importance and impact of an incident. This feature is only available on Standard and Enterprise plans.
/// .
               pub fn priorities(&self) -> priorities::Priorities {
                    priorities::Priorities::new(self.clone())
               }

/// Response Plays are a package of Incident Actions that can be applied during an Incident's life cycle.
/// .
               pub fn response_plays(&self) -> response_plays::ResponsePlays {
                    response_plays::ResponsePlays::new(self.clone())
               }

/// Rulesets allow you to route events to an endpoint and create collections of Event Rules, which define sets of actions to take based on event content.
/// .
               pub fn rulesets(&self) -> rulesets::Rulesets {
                    rulesets::Rulesets::new(self.clone())
               }

/// A Schedule determines the time periods that users are On-Call.
/// .
               pub fn schedules(&self) -> schedules::Schedules {
                    schedules::Schedules::new(self.clone())
               }

/// Services are categorized into technical and business services. Dependencies can be created via any combination of these services.
/// .
               pub fn service_dependencies(&self) -> service_dependencies::ServiceDependencies {
                    service_dependencies::ServiceDependencies::new(self.clone())
               }

/// A Service may represent an application, component, or team you wish to open incidents against.
/// .
               pub fn services(&self) -> services::Services {
                    services::Services::new(self.clone())
               }

/// A webhook is a way to receive events that occur on the PagerDuty platform via an HTTP POST request.
/// V3 webhooks are set up by creating a webhook subscription.
/// .
               pub fn webhooks(&self) -> webhooks::Webhooks {
                    webhooks::Webhooks::new(self.clone())
               }

/// Status Dashboards represent user-defined views for the Status Dashboard product that are limited to specific Business Services rather than the whole set of top-level Business Services (those with no dependent Services).
/// .
               pub fn status_dashboards(&self) -> status_dashboards::StatusDashboards {
                    status_dashboards::StatusDashboards::new(self.clone())
               }

/// A Tag is applied to Escalation Policies, Teams or Users and can be used to filter them.
/// .
               pub fn tags(&self) -> tags::Tags {
                    tags::Tags::new(self.clone())
               }

/// A team is a collection of Users and Escalation Policies that represent a group of people within an organization.
/// .
               pub fn teams(&self) -> teams::Teams {
                    teams::Teams::new(self.clone())
               }

/// Users are members of a PagerDuty account that have the ability to interact with Incidents and other data on the account.
/// .
               pub fn users(&self) -> users::Users {
                    users::Users::new(self.clone())
               }

/// A PagerDuty Vendor represents a specific type of integration. AWS Cloudwatch, Splunk, Datadog are all examples of vendors
/// .
               pub fn vendors(&self) -> vendors::Vendors {
                    vendors::Vendors::new(self.clone())
               }

}
