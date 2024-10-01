//! A fully generated & opinionated API client for the Rippling API.
//!
//! [![docs.rs](https://docs.rs/rippling-api/badge.svg)](https://docs.rs/rippling-api)
//!
//! ## API Details
//!
//! Documentation for the Rippling Platform API.
//!
//!
//!
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
//! rippling-api = "0.1.7"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use rippling_api::Client;
//!
//! let client = Client::new(String::from("api-key"));
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `RIPPLING_API_TOKEN`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use rippling_api::Client;
//!
//! let client = Client::new_from_env();
//! ```
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![cfg_attr(docsrs, feature(doc_cfg))]

/// An application by a candidate to a specific job requisition.
#[cfg(feature = "requests")]
pub mod candidate_applications;
/// Someone who applies to a job requisition opened by the company.
#[cfg(feature = "requests")]
pub mod candidates;
/// Companies on Rippling.
#[cfg(feature = "requests")]
pub mod companies;
/// Compensation associated with workers.
#[cfg(feature = "requests")]
pub mod compensations;
/// Custom fields defined by the company.
#[cfg(feature = "requests")]
pub mod custom_fields;
/// Custom object fields defined by the company.
#[cfg(feature = "requests")]
pub mod custom_object_fields;
/// Custom object datarows defined by the company.
#[cfg(feature = "requests")]
pub mod custom_object_records;
/// Custom objects defined by the company.
#[cfg(feature = "requests")]
pub mod custom_objects;
/// Departments used by the company.
#[cfg(feature = "requests")]
pub mod departments;
/// Employment types used by the company.
#[cfg(feature = "requests")]
pub mod employment_types;
/// Availability of API features to the company or Partners.
#[cfg(feature = "requests")]
pub mod entitlements;
/// Job related information for the company.
#[cfg(feature = "requests")]
pub mod job;
/// A request for a job to be filled by a candidate.
#[cfg(feature = "requests")]
pub mod job_requisitions;
/// Leave balances for workers.
#[cfg(feature = "requests")]
pub mod leave_balances;
/// Leave requests submitted by workers.
#[cfg(feature = "requests")]
pub mod leave_requests;
/// Leave types used by the company.
#[cfg(feature = "requests")]
pub mod leave_types;
/// Legal entities registered by the company.
#[cfg(feature = "requests")]
pub mod legal_entities;
/// Provides the user's SSO information.
#[cfg(feature = "requests")]
pub mod me;
mod methods;
/// Object Categories defined by the company.
#[cfg(feature = "requests")]
pub mod object_categories;
/// Shift inputs used by the company.
#[cfg(feature = "requests")]
pub mod shift_inputs;
/// Teams at the company.
#[cfg(feature = "requests")]
pub mod teams;
#[cfg(test)]
mod tests;
/// Time entries submitted by workers.
#[cfg(feature = "requests")]
pub mod time_entries;
/// Levels and tracks used by the company for workers.
#[cfg(feature = "requests")]
pub mod tracks_and_levels;
pub mod types;
/// Users of the company.
#[cfg(feature = "requests")]
pub mod users;
pub mod utils;
/// Work locations used by the company.
#[cfg(feature = "requests")]
pub mod work_locations;
/// Workers who work or have worked at the company.
#[cfg(feature = "requests")]
pub mod workers;

#[cfg(feature = "requests")]
use std::env;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "requests")]
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
#[cfg(feature = "requests")]
pub struct Client {
    token: String,
    base_url: String,

    #[cfg(feature = "retry")]
    client: reqwest_middleware::ClientWithMiddleware,
    #[cfg(feature = "retry")]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    client_http1_only: reqwest_middleware::ClientWithMiddleware,

    #[cfg(not(feature = "retry"))]
    client: reqwest::Client,
    #[cfg(not(feature = "retry"))]
    #[cfg(not(target_arch = "wasm32"))]
    #[allow(dead_code)]
    client_http1_only: reqwest::Client,
}

/// A request builder.
#[cfg(feature = "retry")]
#[cfg(feature = "requests")]
pub struct RequestBuilder(pub reqwest_middleware::RequestBuilder);
#[cfg(not(feature = "retry"))]
#[cfg(feature = "requests")]
pub struct RequestBuilder(pub reqwest::RequestBuilder);

#[cfg(feature = "requests")]
impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    /// Also takes reqwest client builders, for customizing the client's behaviour.
    #[tracing::instrument]
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new_from_reqwest<T>(
        token: T,
        builder_http: reqwest::ClientBuilder,
        builder_websocket: reqwest::ClientBuilder,
    ) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match (builder_http.build(), builder_websocket.build()) {
                (Ok(c), Ok(c1)) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    let client_http1_only = reqwest_middleware::ClientBuilder::new(c1)
                        .with(reqwest_tracing::TracingMiddleware::default())
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    Client {
                        token: token.to_string(),
                        base_url: "https://rest.ripplingapis.com".to_string(),

                        client,
                        client_http1_only,
                    }
                }
                (Err(e), _) | (_, Err(e)) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            match (builder_http.build(), builder_websocket.build()) {
                (Ok(c), Ok(c1)) => Client {
                    token: token.to_string(),
                    base_url: "https://rest.ripplingapis.com".to_string(),

                    client: c,
                    client_http1_only: c1,
                },
                (Err(e), _) | (_, Err(e)) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    /// Also takes reqwest client builders, for customizing the client's behaviour.
    #[tracing::instrument]
    #[cfg(target_arch = "wasm32")]
    pub fn new_from_reqwest<T>(token: T, builder_http: reqwest::ClientBuilder) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match builder_http.build() {
                Ok(c) => {
                    let client = reqwest_middleware::ClientBuilder::new(c)
                        // Trace HTTP requests. See the tracing crate to make use of these traces.
                        .with(reqwest_tracing::TracingMiddleware::default())
                        // Retry failed requests.
                        .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                            reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                            |req: &reqwest::Request| req.try_clone().is_some(),
                        ))
                        .build();
                    Client {
                        token: token.to_string(),
                        base_url: "https://rest.ripplingapis.com".to_string(),

                        client,
                    }
                }
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            match builder_http.build() {
                Ok(c) => Client {
                    token: token.to_string(),
                    base_url: "https://rest.ripplingapis.com".to_string(),

                    client: c,
                },
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument]
    pub fn new<T>(token: T) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        #[cfg(not(target_arch = "wasm32"))]
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            // For file conversions we need this to be long.
            .timeout(std::time::Duration::from_secs(600))
            .connect_timeout(std::time::Duration::from_secs(60));
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        let client_http1 = reqwest::Client::builder()
            // For file conversions we need this to be long.
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(600))
            .connect_timeout(std::time::Duration::from_secs(60))
            .http1_only();
        #[cfg(not(target_arch = "wasm32"))]
        return Self::new_from_reqwest(token, client, client_http1);
        #[cfg(target_arch = "wasm32")]
        Self::new_from_reqwest(token, client)
    }

    /// Set the base URL for the client to something other than the default: <https://rest.ripplingapis.com>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `RIPPLING_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self {
        let token = env::var("RIPPLING_API_TOKEN").expect("must set RIPPLING_API_TOKEN");

        Client::new(token)
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(method, &u);

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

        Ok(RequestBuilder(req))
    }

    /// An application by a candidate to a specific job requisition.
    pub fn candidate_applications(&self) -> candidate_applications::CandidateApplications {
        candidate_applications::CandidateApplications::new(self.clone())
    }

    /// Someone who applies to a job requisition opened by the company.
    pub fn candidates(&self) -> candidates::Candidates {
        candidates::Candidates::new(self.clone())
    }

    /// Companies on Rippling.
    pub fn companies(&self) -> companies::Companies {
        companies::Companies::new(self.clone())
    }

    /// Compensation associated with workers.
    pub fn compensations(&self) -> compensations::Compensations {
        compensations::Compensations::new(self.clone())
    }

    /// Custom fields defined by the company.
    pub fn custom_fields(&self) -> custom_fields::CustomFields {
        custom_fields::CustomFields::new(self.clone())
    }

    /// Custom object fields defined by the company.
    pub fn custom_object_fields(&self) -> custom_object_fields::CustomObjectFields {
        custom_object_fields::CustomObjectFields::new(self.clone())
    }

    /// Custom object datarows defined by the company.
    pub fn custom_object_records(&self) -> custom_object_records::CustomObjectRecords {
        custom_object_records::CustomObjectRecords::new(self.clone())
    }

    /// Custom objects defined by the company.
    pub fn custom_objects(&self) -> custom_objects::CustomObjects {
        custom_objects::CustomObjects::new(self.clone())
    }

    /// Departments used by the company.
    pub fn departments(&self) -> departments::Departments {
        departments::Departments::new(self.clone())
    }

    /// Employment types used by the company.
    pub fn employment_types(&self) -> employment_types::EmploymentTypes {
        employment_types::EmploymentTypes::new(self.clone())
    }

    /// Availability of API features to the company or Partners.
    pub fn entitlements(&self) -> entitlements::Entitlements {
        entitlements::Entitlements::new(self.clone())
    }

    /// Job related information for the company.
    pub fn job(&self) -> job::Job {
        job::Job::new(self.clone())
    }

    /// A request for a job to be filled by a candidate.
    pub fn job_requisitions(&self) -> job_requisitions::JobRequisitions {
        job_requisitions::JobRequisitions::new(self.clone())
    }

    /// Leave balances for workers.
    pub fn leave_balances(&self) -> leave_balances::LeaveBalances {
        leave_balances::LeaveBalances::new(self.clone())
    }

    /// Leave requests submitted by workers.
    pub fn leave_requests(&self) -> leave_requests::LeaveRequests {
        leave_requests::LeaveRequests::new(self.clone())
    }

    /// Leave types used by the company.
    pub fn leave_types(&self) -> leave_types::LeaveTypes {
        leave_types::LeaveTypes::new(self.clone())
    }

    /// Legal entities registered by the company.
    pub fn legal_entities(&self) -> legal_entities::LegalEntities {
        legal_entities::LegalEntities::new(self.clone())
    }

    /// Provides the user's SSO information.
    pub fn me(&self) -> me::Me {
        me::Me::new(self.clone())
    }

    /// Object Categories defined by the company.
    pub fn object_categories(&self) -> object_categories::ObjectCategories {
        object_categories::ObjectCategories::new(self.clone())
    }

    /// Shift inputs used by the company.
    pub fn shift_inputs(&self) -> shift_inputs::ShiftInputs {
        shift_inputs::ShiftInputs::new(self.clone())
    }

    /// Teams at the company.
    pub fn teams(&self) -> teams::Teams {
        teams::Teams::new(self.clone())
    }

    /// Time entries submitted by workers.
    pub fn time_entries(&self) -> time_entries::TimeEntries {
        time_entries::TimeEntries::new(self.clone())
    }

    /// Levels and tracks used by the company for workers.
    pub fn tracks_and_levels(&self) -> tracks_and_levels::TracksAndLevels {
        tracks_and_levels::TracksAndLevels::new(self.clone())
    }

    /// Users of the company.
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }

    /// Work locations used by the company.
    pub fn work_locations(&self) -> work_locations::WorkLocations {
        work_locations::WorkLocations::new(self.clone())
    }

    /// Workers who work or have worked at the company.
    pub fn workers(&self) -> workers::Workers {
        workers::Workers::new(self.clone())
    }
}
