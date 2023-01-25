//! A fully generated & opinionated API client for the ramp API.
//!
//! [![docs.rs](https://docs.rs/ramp-api/badge.svg)](https://docs.rs/ramp-api)
//!
//! ## API Details
//!
//!
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
//! ramp-api = "0.0.2"
//! ```
//!
//! ## Basic example
//!
//! Typical use will require intializing a `Client`. This requires
//! a user agent string and set of credentials.
//!
//! ```rust,no_run
//! use ramp_api::Client;
//!
//! let client = Client::new(
//!     String::from("client-id"),
//!     String::from("client-secret"),
//!     String::from("redirect-uri"),
//!     String::from("token"),
//!     String::from("refresh-token"),
//! );
//! ```
//!
//! Alternatively, the library can search for most of the variables required for
//! the client in the environment:
//!
//! - `RAMP_CLIENT_ID`
//! - `RAMP_CLIENT_SECRET`
//! - `RAMP_REDIRECT_URI`
//!
//! And then you can create a client from the environment.
//!
//! ```rust,no_run
//! use ramp_api::Client;
//!
//! let client = Client::new_from_env(String::from("token"), String::from("refresh-token"));
//! ```
#![allow(missing_docs)]
#![allow(clippy::needless_lifetimes)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[doc(hidden)]
pub mod business;
pub mod card;
pub mod card_program;
pub mod custom_id_provider;
pub mod department;
pub mod location;
pub mod memo;
pub mod merchant;
pub mod receipt;
pub mod receipt_integrations;
pub mod reimbursement;
pub mod sales_lead;
#[cfg(test)]
mod tests;
pub mod token;
pub mod transaction;
pub mod types;
pub mod user;

use std::{
    convert::TryInto,
    env,
    ops::Add,
    sync::Arc,
    time::{Duration, Instant},
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), ".rs/", env!("CARGO_PKG_VERSION"),);

/// Entrypoint for interacting with the API client.
#[derive(Clone, Debug)]
pub struct Client {
    base_url: String,
    token: Arc<tokio::sync::RwLock<InnerToken>>,
    client_id: String,
    client_secret: String,
    redirect_uri: String,

    auto_refresh: bool,

    client: reqwest_middleware::ClientWithMiddleware,
}

/// An access token.
#[derive(Debug, JsonSchema, Clone, Default, Serialize, Deserialize)]
pub struct AccessToken {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub token_type: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub access_token: String,
    #[serde(default)]
    pub expires_in: i64,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub refresh_token: String,
    #[serde(default, alias = "x_refresh_token_expires_in")]
    pub refresh_token_expires_in: i64,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub scope: String,
}

/// Time in seconds before the access token expiration point that a refresh should
/// be performed. This value is subtracted from the `expires_in` value returned by
/// the provider prior to storing
const REFRESH_THRESHOLD: Duration = Duration::from_secs(60);

#[derive(Debug, Clone)]
struct InnerToken {
    access_token: String,
    refresh_token: String,
    expires_at: Option<Instant>,
}

impl Client {
    /// Create a new Client struct. It takes a type that can convert into
    /// an &str (`String` or `Vec<u8>` for example). As long as the function is
    /// given a valid API credentials your requests will work.
    #[tracing::instrument]
    pub fn new<I, K, R, T, Q>(
        client_id: I,
        client_secret: K,
        redirect_uri: R,
        token: T,
        refresh_token: Q,
    ) -> Self
    where
        I: ToString + std::fmt::Debug,
        K: ToString + std::fmt::Debug,
        R: ToString + std::fmt::Debug,
        T: ToString + std::fmt::Debug,
        Q: ToString + std::fmt::Debug,
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
                    base_url: "https://api.ramp.com/developer/v1".to_string(),
                    client_id: client_id.to_string(),
                    client_secret: client_secret.to_string(),
                    redirect_uri: redirect_uri.to_string(),
                    token: Arc::new(tokio::sync::RwLock::new(InnerToken {
                        access_token: token.to_string(),
                        refresh_token: refresh_token.to_string(),
                        expires_at: None,
                    })),

                    auto_refresh: false,
                    client,
                }
            }
            Err(e) => panic!("creating reqwest client failed: {:?}", e),
        }
    }

    /// Set the base URL for the client to something other than the default: <https://api.ramp.com/developer/v1>.
    #[tracing::instrument]
    pub fn set_base_url<H>(&mut self, base_url: H)
    where
        H: Into<String> + std::fmt::Display + std::fmt::Debug,
    {
        self.base_url = base_url.to_string().trim_end_matches('/').to_string();
    }

    /// Enables or disables the automatic refreshing of access tokens upon expiration
    #[tracing::instrument]
    pub fn set_auto_access_token_refresh(&mut self, enabled: bool) -> &mut Self {
        self.auto_refresh = enabled;
        self
    }

    /// Sets a specific `Instant` at which the access token should be considered expired.
    /// The expiration value will only be used when automatic access token refreshing is
    /// also enabled. `None` may be passed in if the expiration is unknown. In this case
    /// automatic refreshes will be attempted when encountering an UNAUTHENTICATED status
    /// code on a response.
    #[tracing::instrument]
    pub async fn set_expires_at(&self, expires_at: Option<Instant>) -> &Self {
        self.token.write().await.expires_at = expires_at;
        self
    }

    /// Gets the `Instant` at which the access token used by this client is set to expire
    /// if one is known
    #[tracing::instrument]
    pub async fn expires_at(&self) -> Option<Instant> {
        self.token.read().await.expires_at
    }

    /// Sets the number of seconds in which the current access token should be considered
    /// expired
    #[tracing::instrument]
    pub async fn set_expires_in(&self, expires_in: i64) -> &Self {
        self.token.write().await.expires_at = Self::compute_expires_at(expires_in);
        self
    }

    /// Gets the number of seconds from now in which the current access token will be
    /// considered expired if one is known
    #[tracing::instrument]
    pub async fn expires_in(&self) -> Option<Duration> {
        self.token
            .read()
            .await
            .expires_at
            .map(|i| i.duration_since(Instant::now()))
    }

    /// Determines if the access token currently stored in the client is expired. If the
    /// expiration can not be determined, None is returned
    #[tracing::instrument]
    pub async fn is_expired(&self) -> Option<bool> {
        self.token
            .read()
            .await
            .expires_at
            .map(|expiration| expiration <= Instant::now())
    }

    #[tracing::instrument]
    fn compute_expires_at(expires_in: i64) -> Option<Instant> {
        let seconds_valid = expires_in
            .try_into()
            .ok()
            .map(Duration::from_secs)
            .and_then(|dur| dur.checked_sub(REFRESH_THRESHOLD))
            .or_else(|| Some(Duration::from_secs(0)));

        seconds_valid.map(|seconds_valid| Instant::now().add(seconds_valid))
    }

    /// Create a new Client struct from the environment variables:
    ///     - `RAMP_CLIENT_ID`
    ///     - `RAMP_CLIENT_SECRET`
    ///     - `RAMP_REDIRECT_URI`
    #[tracing::instrument]
    pub fn new_from_env<T, R>(token: T, refresh_token: R) -> Self
    where
        T: ToString + std::fmt::Debug,
        R: ToString + std::fmt::Debug,
    {
        let client_id = env::var("RAMP_CLIENT_ID").expect("must set RAMP_CLIENT_ID");
        let client_secret = env::var("RAMP_CLIENT_SECRET").expect("must set RAMP_CLIENT_SECRET");
        let redirect_uri = env::var("RAMP_REDIRECT_URI").expect("must set RAMP_REDIRECT_URI");

        Client::new(client_id, client_secret, redirect_uri, token, refresh_token)
    }

    /// Return a user consent url with an optional set of scopes.
    /// If no scopes are provided, they will not be passed in the url.
    pub fn user_consent_url(&self, scopes: &[String]) -> String {
        let state = uuid::Uuid::new_v4();

        let url = format!(
            "https://app.ramp.com/v1/authorize?client_id={}&response_type=code&redirect_uri={}&state={}",
             self.client_id, self.redirect_uri, state
        );

        if scopes.is_empty() {
            return url;
        }

        // Add the scopes.
        format!("{}&scope={}", url, scopes.join(" "))
    }

    /// Refresh an access token from a refresh token. Client must have a refresh token
    /// for this to work.
    pub async fn refresh_access_token(&self) -> anyhow::Result<AccessToken> {
        let response = {
            let refresh_token = &self.token.read().await.refresh_token;

            if refresh_token.is_empty() {
                anyhow::bail!("refresh token cannot be empty");
            }

            let mut headers = reqwest::header::HeaderMap::new();
            headers.append(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            );

            let params = [
                ("grant_type", "refresh_token"),
                ("refresh_token", refresh_token),
                ("client_id", &self.client_id),
                ("client_secret", &self.client_secret),
                ("redirect_uri", &self.redirect_uri),
            ];
            let client = reqwest::Client::new();
            client
                .post("https://api.ramp.com/v1/public/customer/token")
                .headers(headers)
                .form(&params)
                .basic_auth(&self.client_id, Some(&self.client_secret))
                .send()
                .await?
        };

        // Unwrap the response.
        let t: AccessToken = response.json().await?;

        let refresh_token = self.token.read().await.refresh_token.clone();

        *self.token.write().await = InnerToken {
            access_token: t.access_token.clone(),
            refresh_token,
            expires_at: Self::compute_expires_at(t.expires_in),
        };

        Ok(t)
    }

    /// Get an access token from the code returned by the URL paramter sent to the
    /// redirect URL.
    pub async fn get_access_token(
        &mut self,
        code: &str,
        state: &str,
    ) -> anyhow::Result<AccessToken> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.append(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("redirect_uri", &self.redirect_uri),
            ("state", state),
        ];
        let client = reqwest::Client::new();
        let resp = client
            .post("https://api.ramp.com/v1/public/customer/token")
            .headers(headers)
            .form(&params)
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .send()
            .await?;

        // Unwrap the response.
        let t: AccessToken = resp.json().await?;

        *self.token.write().await = InnerToken {
            access_token: t.access_token.clone(),
            refresh_token: t.refresh_token.clone(),
            expires_at: Self::compute_expires_at(t.expires_in),
        };

        Ok(t)
    }

    /// Create a raw request to our API.
    #[tracing::instrument]
    pub async fn request_raw(
        &self,
        method: reqwest::Method,
        uri: &str,
        body: Option<reqwest::Body>,
    ) -> anyhow::Result<reqwest_middleware::RequestBuilder> {
        if self.auto_refresh {
            let expired = self.is_expired().await;

            match expired {
                // We have a known expired token, we know we need to perform a refresh prior to
                // attempting to make a request
                Some(true) => {
                    self.refresh_access_token().await?;
                }

                // We have a (theoretically) known good token available. We make an optimistic
                // attempting at the request. If the token is no longer good, then something other
                // than the expiration is triggering the failure. We defer handling of these errors
                // to the caller
                Some(false) => (),

                // We do not know what state we are in. We could have a valid or expired token.
                // Generally this means we are in one of two cases:
                //   1. We have not yet performed a token refresh, nor has the user provided
                //      expiration data, and therefore do not know the expiration of the user
                //      provided token
                //   2. The provider is returning unusable expiration times, at which point we
                //      choose to ignore them
                None => (),
            }
        }

        let u = if uri.starts_with("https://") || uri.starts_with("http://") {
            uri.to_string()
        } else {
            format!("{}/{}", self.base_url, uri.trim_start_matches('/'))
        };

        let mut req = self.client.request(method, &u);

        // Add in our authentication.
        req = req.bearer_auth(&self.token.read().await.access_token);

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

    /// Return a reference to an interface that provides access to Business operations.
    pub fn business(&self) -> business::Business {
        business::Business::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Card operations.
    pub fn card(&self) -> card::Card {
        card::Card::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Card Program operations.
    pub fn card_program(&self) -> card_program::CardProgram {
        card_program::CardProgram::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Custom Id Provider operations.
    pub fn custom_id_provider(&self) -> custom_id_provider::CustomIdProvider {
        custom_id_provider::CustomIdProvider::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Department operations.
    pub fn department(&self) -> department::Department {
        department::Department::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Location operations.
    pub fn location(&self) -> location::Location {
        location::Location::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Receipt operations.
    pub fn receipt(&self) -> receipt::Receipt {
        receipt::Receipt::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Reimbursement operations.
    pub fn reimbursement(&self) -> reimbursement::Reimbursement {
        reimbursement::Reimbursement::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Token operations.
    pub fn token(&self) -> token::Token {
        token::Token::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Transaction operations.
    pub fn transaction(&self) -> transaction::Transaction {
        transaction::Transaction::new(self.clone())
    }

    /// Return a reference to an interface that provides access to User operations.
    pub fn user(&self) -> user::User {
        user::User::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Memo operations.
    pub fn memo(&self) -> memo::Memo {
        memo::Memo::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Merchant operations.
    pub fn merchant(&self) -> merchant::Merchant {
        merchant::Merchant::new(self.clone())
    }

    /// Return a reference to an interface that provides access to SalesLead operations.
    pub fn sales_lead(&self) -> sales_lead::SalesLead {
        sales_lead::SalesLead::new(self.clone())
    }

    /// Return a reference to an interface that provides access to Receipt Integrations operations.
    pub fn receipt_integrations(&self) -> receipt_integrations::ReceiptIntegrations {
        receipt_integrations::ReceiptIntegrations::new(self.clone())
    }
}
