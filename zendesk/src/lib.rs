//! A library for working with the Zendesk API.
//! This library is an implementation of: <https://developers.getbase.com/docs/rest/reference/contacts>
#![deny(missing_docs)]

#[cfg(test)]
mod tests;
pub mod types;

use anyhow::Result;
use reqwest::{Method, Request, StatusCode, Url};
use serde::{Deserialize, Serialize};

use crate::types::{Contact, ContactData, ContactsListResponse, NewContact, NewContactData};

/// Entrypoint for interacting with the Zendesk API.
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest_middleware::ClientWithMiddleware,
    token: String,
}

/// The default host for the API.
pub const DEFAULT_HOST: &str = "https://api.getbase.com";

impl Client {
    /// Create a new Zendesk client struct.
    #[tracing::instrument]
    pub fn new<T>(token: T) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        let http = reqwest::Client::builder().build();
        match http {
            Ok(c) => {
                let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder().build_with_max_retries(3);
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(retry_policy))
                    .build();

                Self {
                    client,
                    token: token.to_string(),
                }
            }
            Err(e) => panic!("creating client failed: {:?}", e),
        }
    }

    /// Create a new Client struct from environment variables. As long as the function is
    /// given a valid API key and your requests will work.
    pub fn new_from_env() -> Self {
        let token = std::env::var("ZENDESK_TOKEN").expect("must set ZENDESK_TOKEN");

        Client::new(token)
    }

    #[tracing::instrument(skip(self, body))]
    fn request<P, B>(&self, method: Method, path: P, body: &B, query: Option<Vec<(&str, &str)>>) -> Result<Request>
    where
        P: ToString + std::fmt::Debug,
        B: serde::Serialize,
    {
        let url = Url::parse(&format!(
            "{}/{}",
            DEFAULT_HOST,
            path.to_string().trim_start_matches('/')
        ))?;

        let mut rb = self.client.request(method.clone(), url);

        // Add our token to the request.
        rb = rb.bearer_auth(self.token.as_str());

        // Add our user agent.
        rb = rb.header("User-Agent", "kittycad/zendesk-rust-api");

        match query {
            None => (),
            Some(val) => {
                rb = rb.query(&val);
            }
        }

        // Add the body, this is to ensure our GET and DELETE calls succeed.
        if method != Method::GET && method != Method::DELETE {
            rb = rb.json(body);
        }

        // Build the request.
        Ok(rb.build()?)
    }

    /// Create a contact.
    #[tracing::instrument(skip(self))]
    pub async fn create_contact(&self, contact: &NewContact) -> Result<Contact> {
        // Build the request.
        let request = self.request(
            Method::POST,
            "/v2/contacts",
            &NewContactData {
                data: contact.clone(),
                meta: Default::default(),
            },
            None,
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            StatusCode::CREATED => (),
            StatusCode::ACCEPTED => (),
            s => {
                // Try to deserialize the response.
                let body = resp.text().await?;
                let err: Error = match serde_json::from_str(&body) {
                    Ok(j) => j,
                    Err(_) => {
                        // If deserialization failed, return the raw response.
                        Error::Http {
                            status: s.to_string(),
                            code: s.as_u16(),
                            message: body,
                        }
                    }
                };

                return Err(err.into());
            }
        };

        let text = resp.text().await?;

        // Try to deserialize the response.
        let data: ContactData = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data.data)
    }

    /// List all contacts.
    #[tracing::instrument(skip(self))]
    pub async fn list_contacts(&self, filter_by_email: Option<&str>) -> Result<Vec<Contact>> {
        let mut query = vec![("per_page", "100")];

        // Add the filter if it exists.
        if let Some(email) = filter_by_email {
            query.push(("email", email));
        }

        // Build the request.
        let request = self.request(Method::GET, "/v2/contacts", &(), Some(query))?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                // Try to deserialize the response.
                let body = resp.text().await?;
                let err: Error = match serde_json::from_str(&body) {
                    Ok(j) => j,
                    Err(_) => {
                        // If deserialization failed, return the raw response.
                        Error::Http {
                            status: s.to_string(),
                            code: s.as_u16(),
                            message: body,
                        }
                    }
                };

                return Err(err.into());
            }
        };

        // Try to deserialize the response.
        let text = resp.text().await?;

        // Try to deserialize the response.
        let data: ContactsListResponse = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        // TODO: pagination.

        Ok(data.items.iter().map(|c| c.data.clone()).collect())
    }

    /// Get a contact.
    #[tracing::instrument(skip(self))]
    pub async fn get_contact(&self, id: &str) -> Result<Contact> {
        // Build the request.
        let request = self.request(Method::GET, &format!("/v2/contacts/{}", id), &(), None)?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            s => {
                // Try to deserialize the response.
                let body = resp.text().await?;
                let err: Error = match serde_json::from_str(&body) {
                    Ok(j) => j,
                    Err(_) => {
                        // If deserialization failed, return the raw response.
                        Error::Http {
                            status: s.to_string(),
                            code: s.as_u16(),
                            message: body,
                        }
                    }
                };

                return Err(err.into());
            }
        };

        // Try to deserialize the response.
        let text = resp.text().await?;

        // Try to deserialize the response.
        let data: ContactData = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data.data)
    }

    /// Update a contact.
    #[tracing::instrument(skip(self))]
    pub async fn update_contact(&self, id: &str, contact: &NewContact) -> Result<Contact> {
        // Build the request.
        let request = self.request(
            Method::PUT,
            &format!("/v2/contacts/{}", id),
            &NewContactData {
                data: contact.clone(),
                meta: Default::default(),
            },
            None,
        )?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            StatusCode::CREATED => (),
            StatusCode::ACCEPTED => (),
            s => {
                // Try to deserialize the response.
                let body = resp.text().await?;
                let err: Error = match serde_json::from_str(&body) {
                    Ok(j) => j,
                    Err(_) => {
                        // If deserialization failed, return the raw response.
                        Error::Http {
                            status: s.to_string(),
                            code: s.as_u16(),
                            message: body,
                        }
                    }
                };

                return Err(err.into());
            }
        };

        // Try to deserialize the response.
        let text = resp.text().await?;

        // Try to deserialize the response.
        let data: ContactData = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data.data)
    }

    /// Delete a contact.
    #[tracing::instrument(skip(self))]
    pub async fn delete_contact(&self, id: &str) -> Result<()> {
        // Build the request.
        let request = self.request(Method::DELETE, &format!("/v2/contacts/{}", id), &(), None)?;

        let resp = self.client.execute(request).await?;
        match resp.status() {
            StatusCode::OK => (),
            StatusCode::CREATED => (),
            StatusCode::ACCEPTED => (),
            StatusCode::NO_CONTENT => (),
            s => {
                // Try to deserialize the response.
                let body = resp.text().await?;
                let err: Error = match serde_json::from_str(&body) {
                    Ok(j) => j,
                    Err(_) => {
                        // If deserialization failed, return the raw response.
                        Error::Http {
                            status: s.to_string(),
                            code: s.as_u16(),
                            message: body,
                        }
                    }
                };

                return Err(err.into());
            }
        };

        Ok(())
    }
}

/// An error for an HTTP request.
/// This comes from: <https://developers.getbase.com/docs/rest/articles/oauth2/errors>.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    /// An error from the HTTP client.
    #[error("{status} {code}: {message}")]
    Http {
        /// A status string.
        #[serde(rename = "error_code")]
        status: String,

        /// The HTTP status code.
        #[serde(default)]
        code: u16,

        /// A message string.
        message: String,
    },
    /// An error from Zendesk.
    #[error("{error}: {description} {error_uri}")]
    Zendesk {
        /// An error string.
        error: String,

        /// A description string.
        description: String,

        /// A URL with more details.
        error_uri: String,
    },
    /// An error from deserialization.
    #[error("{message}: {body}")]
    Json {
        /// The response body.
        body: String,

        /// The error message.
        message: String,
    },
}
