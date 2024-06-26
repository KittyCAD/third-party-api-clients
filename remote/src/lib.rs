//! A fully generated & opinionated API client for the Remote API.
//!
//! [![docs.rs](https://docs.rs/remote-api/badge.svg)](https://docs.rs/remote-api)
//!
//! ## API Details
//!
//! # Authentication
//!
//! <!-- ReDoc-Inject: <security-definitions> -->
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
//! remote-api = "0.1.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use remote_api::Client;
//!
//! let client = Client::new(String::from("api-key"));
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `REMOTE_API_TOKEN`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use remote_api::Client;
//!
//! let client = Client::new_from_env();
//! ```
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod company_managers;
pub mod countries;
pub mod employments;
pub mod sandbox;
#[cfg(test)]
mod tests;
pub mod time_off;
pub mod types;
pub mod utils;

use std::env;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
pub struct Client {
    token: String,
    base_url: String,

    client: reqwest_middleware::ClientWithMiddleware,
    client_http1_only: reqwest_middleware::ClientWithMiddleware,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument]
    pub fn new<T>(token: T) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        // Retry up to 3 times with increasing intervals between attempts.
        let retry_policy =
            reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            // For file conversions we need this to be long.
            .timeout(std::time::Duration::from_secs(60))
            .connect_timeout(std::time::Duration::from_secs(60))
            .build();
        let client_http1 = reqwest::Client::builder()
            // For file conversions we need this to be long.
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(60))
            .connect_timeout(std::time::Duration::from_secs(60))
            .http1_only()
            .build();
        match (client, client_http1) {
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
                    base_url: "https://gateway.remote.com".to_string(),

                    client,
                    client_http1_only,
                }
            }
            (Err(e), _) | (_, Err(e)) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the base URL for the client to something other than the default: <https://gateway.remote.com>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `REMOTE_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self {
        let token = env::var("REMOTE_API_TOKEN").expect("must set REMOTE_API_TOKEN");

        Client::new(token)
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<reqwest_middleware::RequestBuilder> {
        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(method, u);

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

    /// Return a reference to an interface that provides access to Company Managers operations.
    pub fn company_managers(&self) -> company_managers::CompanyManagers {
        company_managers::CompanyManagers::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Countries operations.
    pub fn countries(&self) -> countries::Countries {
        countries::Countries::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Employments operations.
    pub fn employments(&self) -> employments::Employments {
        employments::Employments::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Sandbox operations.
    pub fn sandbox(&self) -> sandbox::Sandbox {
        sandbox::Sandbox::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Time Off operations.
    pub fn time_off(&self) -> time_off::TimeOff {
        time_off::TimeOff::new(self.clone())
    }
}
