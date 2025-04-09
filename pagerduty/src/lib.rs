//! A library for working with the Pagerduty API.
//! This library is an implementation of: <https://developer.pagerduty.com/api-reference/>
#![deny(missing_docs)]

#[cfg(test)]
mod tests;
pub mod types;

use anyhow::Result;
use reqwest::{Method, Request, StatusCode, Url};
use serde::{Deserialize, Serialize};

use crate::types::{
    EscalationPolicy, EscalationPolicyListResponse, Service, ServiceListResponse, ServiceObject,
};

/// Entrypoint for interacting with the Pagerduty API.
#[derive(Debug, Clone)]
pub struct Client {
    client: reqwest_middleware::ClientWithMiddleware,
    token: String,
}

/// The default host for the API.
pub const DEFAULT_HOST: &str = "https://api.pagerduty.com";
/// The limit for paginating.
pub const LIMIT: i64 = 30;

impl Client {
    /// Create a new Pagerduty client struct.
    #[tracing::instrument]
    pub fn new<T>(token: T) -> Self
    where
        T: ToString + std::fmt::Debug,
    {
        let http = reqwest::Client::builder().build();
        match http {
            Ok(c) => {
                let retry_policy = reqwest_retry::policies::ExponentialBackoff::builder()
                    .build_with_max_retries(3);
                let client = reqwest_middleware::ClientBuilder::new(c)
                    // Trace HTTP requests. See the tracing crate to make use of these traces.
                    .with(reqwest_tracing::TracingMiddleware::default())
                    // Retry failed requests.
                    .with(reqwest_retry::RetryTransientMiddleware::new_with_policy(
                        retry_policy,
                    ))
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
        let token = std::env::var("PAGERDUTY_TOKEN").expect("must set PAGERDUTY_TOKEN");

        Client::new(token)
    }

    #[tracing::instrument(skip(self, body))]
    fn request<P, B>(
        &self,
        method: Method,
        path: P,
        body: &B,
        query: Option<Vec<(&str, &str)>>,
    ) -> Result<Request>
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
        rb = rb.header("Authorization", &format!("Token token={}", self.token));

        // Add our user agent.
        rb = rb.header("User-Agent", "kittycad/pagerduty-rust-api");

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

    /// Create a service.
    #[tracing::instrument(skip(self))]
    pub async fn create_service(&self, service: &Service) -> Result<Service> {
        // Build the request.
        let request = self.request(
            Method::POST,
            "/services",
            &ServiceObject {
                service: service.clone(),
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
        let data: ServiceObject = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data.service)
    }

    /// List all services.
    #[tracing::instrument(skip(self))]
    pub async fn list_services(&self) -> Result<Vec<Service>> {
        let mut services: Vec<Service> = Default::default();
        let mut resp = self.list_services_internal(0).await?;

        services.append(&mut resp.services);

        while resp.more {
            let offset = resp.offset + LIMIT;
            resp = self.list_services_internal(offset).await?;

            services.append(&mut resp.services);
        }

        Ok(services)
    }

    #[tracing::instrument(skip(self))]
    async fn list_services_internal(&self, offset: i64) -> Result<ServiceListResponse> {
        let limit_str = format!("{}", LIMIT);
        let mut query: Vec<(&str, &str)> = vec![("limit", &limit_str)];

        let offset_str = format!("{}", offset);
        if offset > 0 {
            query.push(("offset", &offset_str));
        }

        // Build the request.
        let request = self.request(Method::GET, "/services", &(), Some(query))?;

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
        let data: ServiceListResponse = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data)
    }

    /// List all escalation policies.
    #[tracing::instrument(skip(self))]
    pub async fn list_escalation_policies(&self) -> Result<Vec<EscalationPolicy>> {
        let mut escalation_policies: Vec<EscalationPolicy> = Default::default();
        let mut resp = self.list_escalation_policies_internal(0).await?;

        escalation_policies.append(&mut resp.escalation_policies);

        while resp.more {
            let offset = resp.offset + LIMIT;
            resp = self.list_escalation_policies_internal(offset).await?;

            escalation_policies.append(&mut resp.escalation_policies);
        }

        Ok(escalation_policies)
    }

    #[tracing::instrument(skip(self))]
    async fn list_escalation_policies_internal(
        &self,
        offset: i64,
    ) -> Result<EscalationPolicyListResponse> {
        let limit_str = format!("{}", LIMIT);
        let mut query: Vec<(&str, &str)> = vec![("limit", &limit_str)];

        let offset_str = format!("{}", offset);
        if offset > 0 {
            query.push(("offset", &offset_str));
        }

        // Build the request.
        let request = self.request(Method::GET, "/escalation_policies", &(), Some(query))?;

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
        let data: EscalationPolicyListResponse = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data)
    }

    /// Get a service.
    #[tracing::instrument(skip(self))]
    pub async fn get_service(&self, id: &str) -> Result<Service> {
        // Build the request.
        let request = self.request(Method::GET, &format!("/services/{}", id), &(), None)?;

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
        let data: ServiceObject = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data.service)
    }

    /// Update a service.
    #[tracing::instrument(skip(self))]
    pub async fn update_service(&self, service: &Service) -> Result<Service> {
        // Build the request.
        let request = self.request(
            Method::PUT,
            &format!("/services/{}", service.id),
            &ServiceObject {
                service: service.clone(),
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
        let data: ServiceObject = serde_json::from_str(&text).map_err(|err| {
            // If deserialization failed, return the raw response.
            Error::Json {
                body: text,
                message: err.to_string(),
            }
        })?;

        Ok(data.service)
    }

    /// Delete a service.
    #[tracing::instrument(skip(self))]
    pub async fn delete_service(&self, id: &str) -> Result<()> {
        // Build the request.
        let request = self.request(Method::DELETE, &format!("/services/{}", id), &(), None)?;

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
    /// An error from Pagerduty.
    #[error("{error}: {description} {error_uri}")]
    Pagerduty {
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
