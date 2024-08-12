//! A fully generated & opinionated API client for the Twilio API.
//!
//! [![docs.rs](https://docs.rs/twilio-api/badge.svg)](https://docs.rs/twilio-api)
//!
//! ## API Details
//!
//! This is the public Twilio REST API.
//!
//! [API Terms of Service](https://www.twilio.com/legal/tos)
//!
//! ### Contact
//!
//!
//! | name | url | email |
//! |----|----|----|
//! | Twilio Support | <https://support.twilio.com> | support@twilio.com |
//!
//! ### License
//!
//!
//! | name | url |
//! |----|----|
//! | Apache 2.0 | <https://www.apache.org/licenses/LICENSE-2.0.html> |
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
//! twilio-api = "0.1.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use twilio_api::Client;
//!
//! let client = Client::new(String::from("username"), String::from("password"));
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `TWILIO_USERNAME`
//! - `TWILIO_PASSWORD`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use twilio_api::Client;
//!
//! let client = Client::new_from_env();
//! ```
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "requests")]
pub mod default;
mod methods;
#[cfg(test)]
mod tests;
pub mod types;
pub mod utils;

#[cfg(feature = "requests")]
use std::env;

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "requests")]
static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
#[cfg(feature = "requests")]
pub struct Client {
    username: String,
    password: String,
    base_url: String,

    #[cfg(feature = "retry")]
    client: reqwest_middleware::ClientWithMiddleware,
    #[cfg(not(feature = "retry"))]
    client: reqwest::Client,
}

#[cfg(feature = "requests")]
impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument]
    pub fn new<T>(username: T, password: T) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        let client = reqwest::Client::builder()
            .user_agent(APP_USER_AGENT)
            .timeout(std::time::Duration::from_secs(60))
            .connect_timeout(std::time::Duration::from_secs(60))
            .build();
        #[cfg(feature = "retry")]
        {
            // Retry up to 3 times with increasing intervals between attempts.
            let retry_policy =
                reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
            match client {
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
                        username: username.to_string(),
                        password: password.to_string(),
                        base_url: "https://api.twilio.com".to_string(),

                        client,
                    }
                }
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
        #[cfg(not(feature = "retry"))]
        {
            Client {
                username: username.to_string(),
                password: password.to_string(),
                base_url: "https://api.twilio.com".to_string(),

                client,
            }
        }
    }

    /// Set the base URL for the client to something other than the default: <https://api.twilio.com>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `TWILIO_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self {
        let username = env::var("TWILIO_USERNAME").expect("must set TWILIO_USERNAME");
        let password = env::var("TWILIO_PASSWORD").expect("must set TWILIO_PASSWORD");

        Client::new(username, password)
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

        let mut req = self.client.request(method, &u);

        // Add in our authentication.
        req = req.basic_auth(&self.username, Some(&self.password));

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

    /// Return a reference to an interface that provides access to default operations.
    pub fn default(&self) -> default::Default {
        default::Default::new(self.clone())
    }
}
