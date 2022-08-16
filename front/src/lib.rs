//! A fully generated & opinionated API client for the Front API.
//!
//! [![docs.rs](https://docs.rs/front-api/badge.svg)](https://docs.rs/front-api)
//!
//! ## API Details
//!
//!
//!
//!
//!
//! ### Contact
//!
//!
//! | name | email |
//! |----|----|
//! | Front Platform | api@frontapp.com |
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
//! front-api = "0.0.2"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use front_api::Client;
//!
//! let client = Client::new(String::from("api-key"));
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `FRONT_API_TOKEN`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use front_api::Client;
//!
//! let client = Client::new_from_env();
//! ```
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
pub mod accounts;
pub mod analytics;
pub mod attachments;
pub mod channels;
pub mod comments;
pub mod contact_groups;
pub mod contact_handles;
pub mod contact_notes;
pub mod contacts;
pub mod conversations;
pub mod custom_fields;
pub mod drafts;
pub mod events;
pub mod inboxes;
pub mod links;
pub mod message_template_folders;
pub mod message_templates;
pub mod messages;
pub mod rules;
pub mod shifts;
pub mod signatures;
pub mod tags;
pub mod teammates;
pub mod teams;
#[cfg(test)]
mod tests;
pub mod token_identity;
pub mod types;

use std::env;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

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
    pub fn new<T>(token: T) -> Self
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
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(reqwest_conditional_middleware::ConditionalMiddleware::new(
                        reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy),
                        |req: &reqwest::Request| req.try_clone().is_some(),
                    ))
                    .build();

                Client {
                    token: token.to_string(),
                    base_url: "https://api2.frontapp.com".to_string(),

                    client,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the base URL for the client to something other than the default: <https://api2.frontapp.com>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Create a new Client struct from the environment variable: `FRONT_API_TOKEN`.
    #[tracing::instrument]
    pub fn new_from_env() -> Self {
        let token = env::var("FRONT_API_TOKEN").expect("must set FRONT_API_TOKEN");

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

        Ok(req)
    }

    /// Return a reference to an interface that provides access to Accounts operations.
    pub fn accounts(&self) -> accounts::Accounts {
        accounts::Accounts::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Events operations.
    pub fn events(&self) -> events::Events {
        events::Events::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Analytics operations.
    pub fn analytics(&self) -> analytics::Analytics {
        analytics::Analytics::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Attachments operations.
    pub fn attachments(&self) -> attachments::Attachments {
        attachments::Attachments::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Token Identity operations.
    pub fn token_identity(&self) -> token_identity::TokenIdentity {
        token_identity::TokenIdentity::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Message Template Folders operations.
    pub fn message_template_folders(&self) -> message_template_folders::MessageTemplateFolders {
        message_template_folders::MessageTemplateFolders::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Message Templates operations.
    pub fn message_templates(&self) -> message_templates::MessageTemplates {
        message_templates::MessageTemplates::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Contact Groups operations.
    pub fn contact_groups(&self) -> contact_groups::ContactGroups {
        contact_groups::ContactGroups::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Contacts operations.
    pub fn contacts(&self) -> contacts::Contacts {
        contacts::Contacts::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Contact Handles operations.
    pub fn contact_handles(&self) -> contact_handles::ContactHandles {
        contact_handles::ContactHandles::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Contact Notes operations.
    pub fn contact_notes(&self) -> contact_notes::ContactNotes {
        contact_notes::ContactNotes::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Channels operations.
    pub fn channels(&self) -> channels::Channels {
        channels::Channels::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Inboxes operations.
    pub fn inboxes(&self) -> inboxes::Inboxes {
        inboxes::Inboxes::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Comments operations.
    pub fn comments(&self) -> comments::Comments {
        comments::Comments::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Conversations operations.
    pub fn conversations(&self) -> conversations::Conversations {
        conversations::Conversations::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Messages operations.
    pub fn messages(&self) -> messages::Messages {
        messages::Messages::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Custom Fields operations.
    pub fn custom_fields(&self) -> custom_fields::CustomFields {
        custom_fields::CustomFields::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Drafts operations.
    pub fn drafts(&self) -> drafts::Drafts {
        drafts::Drafts::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Rules operations.
    pub fn rules(&self) -> rules::Rules {
        rules::Rules::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Shifts operations.
    pub fn shifts(&self) -> shifts::Shifts {
        shifts::Shifts::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Signatures operations.
    pub fn signatures(&self) -> signatures::Signatures {
        signatures::Signatures::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Tags operations.
    pub fn tags(&self) -> tags::Tags {
        tags::Tags::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Teams operations.
    pub fn teams(&self) -> teams::Teams {
        teams::Teams::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Teammates operations.
    pub fn teammates(&self) -> teammates::Teammates {
        teammates::Teammates::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Links operations.
    pub fn links(&self) -> links::Links {
        links::Links::new(self.clone())
    }
}
