//! A fully generated & opinionated API client for the Discourse API.
//!
//! [![docs.rs](https://docs.rs/discourse-api/badge.svg)](https://docs.rs/discourse-api)
//!
//! ## API Details
//!
//! This page contains the documentation on how to use Discourse through API calls.
//!
//! > Note: For any endpoints not listed you can follow the
//! [reverse engineer the Discourse API](https://meta.discourse.org/t/-/20576)
//! guide to figure out how to use an API endpoint.
//!
//! ### Request Content-Type
//!
//! The Content-Type for POST and PUT requests can be set to `application/x-www-form-urlencoded`,
//! `multipart/form-data`, or `application/json`.
//!
//! ### Endpoint Names and Response Content-Type
//!
//! Most API endpoints provide the same content as their HTML counterparts. For example
//! the URL `/categories` serves a list of categories, the `/categories.json` API provides the
//! same information in JSON format.
//!
//! Instead of sending API requests to `/categories.json` you may also send them to `/categories`
//! and add an `Accept: application/json` header to the request to get the JSON response.
//! Sending requests with the `Accept` header is necessary if you want to use URLs
//! for related endpoints returned by the API, such as pagination URLs.
//! These URLs are returned without the `.json` prefix so you need to add the header in
//! order to get the correct response format.
//!
//! ### Authentication
//!
//! Some endpoints do not require any authentication, pretty much anything else will
//! require you to be authenticated.
//!
//! To become authenticated you will need to create an API Key from the admin panel.
//!
//! Once you have your API Key you can pass it in along with your API Username
//! as an HTTP header like this:
//!
//! ```text
//! curl -X GET "http://127.0.0.1:3000/admin/users/list/active.json" \
//! -H "Api-Key: 714552c6148e1617aeab526d0606184b94a80ec048fc09894ff1a72b740c5f19" \
//! -H "Api-Username: system"
//! ```
//!
//! and this is how POST requests will look:
//!
//! ```text
//! curl -X POST "http://127.0.0.1:3000/categories" \
//! -H "Content-Type: multipart/form-data;" \
//! -H "Api-Key: 714552c6148e1617aeab526d0606184b94a80ec048fc09894ff1a72b740c5f19" \
//! -H "Api-Username: system" \
//! -F "name=89853c20-4409-e91a-a8ea-f6cdff96aaaa" \
//! -F "color=49d9e9" \
//! -F "text_color=f0fcfd"
//! ```
//!
//! ### Boolean values
//!
//! If an endpoint accepts a boolean be sure to specify it as a lowercase
//! `true` or `false` value unless noted otherwise.
//!
//!
//!
//!
//!
//! ### License
//!
//!
//! | name | url |
//! |----|----|
//! | MIT | <https://docs.discourse.org/LICENSE.txt> |
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
//! discourse-api = "0.1.0"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use discourse_api::Client;
//!
//! let client = Client::new(String::from("api-key"));
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `DISCOURSE_API_TOKEN`
//!
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use discourse_api::Client;
//!
//! let client = Client::new_from_env();
//! ```
#![allow(elided_named_lifetimes)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::too_many_arguments)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "requests")]
pub mod backups;
#[cfg(feature = "requests")]
pub mod badges;
#[cfg(feature = "requests")]
pub mod categories;
#[cfg(feature = "requests")]
pub mod discourse_calendar_events;
#[cfg(feature = "requests")]
pub mod groups;
#[cfg(feature = "requests")]
pub mod invites;
mod methods;
#[cfg(feature = "requests")]
pub mod notifications;
#[cfg(feature = "requests")]
pub mod posts;
#[cfg(feature = "requests")]
pub mod private_messages;
#[cfg(feature = "requests")]
pub mod search;
#[cfg(feature = "requests")]
pub mod site;
#[cfg(feature = "requests")]
pub mod tags;
#[cfg(test)]
mod tests;
#[cfg(feature = "requests")]
pub mod topics;
pub mod types;
#[cfg(feature = "requests")]
pub mod uploads;
#[cfg(feature = "requests")]
pub mod users;

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
    #[tracing::instrument(skip(token))]
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
                        base_url: "https://discourse.example.com".to_string(),

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
                    base_url: "https://discourse.example.com".to_string(),

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
    #[tracing::instrument(skip(token))]
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
                        base_url: "https://discourse.example.com".to_string(),

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
                    base_url: "https://discourse.example.com".to_string(),

                    client: c,
                },
                Err(e) => panic!("creating reqwest client failed: {:?}", e),
            }
        }
    }

    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API key your requests will work.
    #[tracing::instrument(skip(token))]
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

    /// Set the base URL for the client to something other than the default: <https://discourse.example.com>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `ENV_VARIABLE_PREFIX_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self {
        let token = env::var("DISCOURSE_API_TOKEN").expect("must set DISCOURSE_API_TOKEN");
        let base_url =
            env::var("DISCOURSE_HOST").unwrap_or("https://discourse.example.com".to_string());

        let mut c = Client::new(token);
        c.set_base_url(base_url);
        c
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

    /// Return a reference to an interface that provides access to Discourse Calendar - Events operations.
    pub fn discourse_calendar_events(&self) -> discourse_calendar_events::DiscourseCalendarEvents {
        discourse_calendar_events::DiscourseCalendarEvents::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Backups operations.
    pub fn backups(&self) -> backups::Backups {
        backups::Backups::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Badges operations.
    pub fn badges(&self) -> badges::Badges {
        badges::Badges::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Categories operations.
    pub fn categories(&self) -> categories::Categories {
        categories::Categories::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Groups operations.
    pub fn groups(&self) -> groups::Groups {
        groups::Groups::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Invites operations.
    pub fn invites(&self) -> invites::Invites {
        invites::Invites::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Notifications operations.
    pub fn notifications(&self) -> notifications::Notifications {
        notifications::Notifications::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Posts operations.
    pub fn posts(&self) -> posts::Posts {
        posts::Posts::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Private Messages operations.
    pub fn private_messages(&self) -> private_messages::PrivateMessages {
        private_messages::PrivateMessages::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Search operations.
    pub fn search(&self) -> search::Search {
        search::Search::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Site operations.
    pub fn site(&self) -> site::Site {
        site::Site::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Tags operations.
    pub fn tags(&self) -> tags::Tags {
        tags::Tags::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Topics operations.
    pub fn topics(&self) -> topics::Topics {
        topics::Topics::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Uploads operations.
    pub fn uploads(&self) -> uploads::Uploads {
        uploads::Uploads::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Users operations.
    pub fn users(&self) -> users::Users {
        users::Users::new(self.clone())
    }
}
