mod errors;

use errors::MKSError;
use http::{HeaderMap, StatusCode};
use std::time::Duration;
use tokio::time::timeout;
use url::Url;

// Hyper imports.
use hyper::body::Buf;
use hyper::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use hyper::{Client, Method, Request};
#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

// Environment variables from Cargo.
static PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
static PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// MKS struct is used to make calls to the MKS API.
pub struct MKS {
    client: Client<HttpsConnector>,
    token: String,
    base_endpoint: url::Url,
    user_agent: String,
    timeout: Duration,
}

impl MKS {
    /// Construct the new MKS struct with default configuration.
    ///
    /// Use `Builder` to configure the client.
    pub fn new(base_endpoint: &str, token: &str) -> Result<MKS, MKSError> {
        MKS::with_builder(base_endpoint, token, MKS::builder())
    }

    fn with_builder(base_endpoint: &str, token: &str, builder: Builder) -> Result<MKS, MKSError> {
        // Check token.
        if token.is_empty() {
            return Err(MKSError::EmptyTokenError);
        }
        let token = String::from(token);

        // Check base endpoint.
        let base_endpoint = Url::parse(base_endpoint).map_err(|_| MKSError::EndpointError)?;

        // Use the provided Hyper client or configure a new one.
        let client = match builder.client {
            Some(client) => client,
            None => {
                #[cfg(feature = "rustls")]
                let client = Client::builder().build(HttpsConnector::new());
                #[cfg(feature = "rust-native-tls")]
                let client = Client::builder().build(HttpsConnector::new()?);

                client
            }
        };

        Ok(MKS {
            client,
            token,
            base_endpoint,
            user_agent: MKS::user_agent(),
            timeout: builder.timeout,
        })
    }

    fn user_agent() -> String {
        format!("{}/{}", PKG_NAME, PKG_VERSION)
    }

    /// Get a default builder.
    pub fn builder() -> Builder {
        Builder::default()
    }

    // Prepare a new request.
    fn new_request(
        &mut self,
        method: Method,
        path: &str,
        body: Option<String>,
    ) -> Result<Request<hyper::Body>, MKSError> {
        // Build a final Hyper URI.
        let uri = self.make_uri(path)?;

        // Prepare a new Hyper request.
        let mut req = Request::new(hyper::Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = uri.clone();

        // Add user-agent header.
        req.headers_mut().insert(
            USER_AGENT,
            HeaderValue::from_str(self.user_agent.as_str()).map_err(|_| MKSError::RequestError)?,
        );

        // Add x-auth-token header.
        req.headers_mut().insert(
            "x-auth-token",
            HeaderValue::from_str(self.token.as_str()).map_err(|_| MKSError::RequestError)?,
        );

        // Add body into the new request if it's provided.
        if let Some(body) = body {
            // Add content-length header if body is provided.
            let len = HeaderValue::from_str(&body.len().to_string())
                .map_err(|_| MKSError::RequestError)?;
            req.headers_mut().insert(CONTENT_LENGTH, len);

            // Add content-type header if body is provided.
            req.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_str("application/json").map_err(|_| MKSError::RequestError)?,
            );

            *req.body_mut() = hyper::Body::from(body);
        }

        Ok(req)
    }

    #[tokio::main]
    async fn do_request(&self, req: hyper::Request<hyper::Body>) -> Result<Response, MKSError> {
        let mut headers = HeaderMap::new();
        let duration = self.timeout.clone();
        let handle = async {
            let raw_resp = self.client.request(req).await?;

            headers = raw_resp.headers().clone();
            let status = raw_resp.status();
            let body = hyper::body::aggregate(raw_resp).await?.to_bytes();
            let body = String::from_utf8_lossy(&body);

            Ok::<_, hyper::Error>((body.to_string(), status))
        };

        let raw_resp = timeout(duration, handle).await??;

        let (body, status) = raw_resp;

        if !status.is_success() {
            return Err(MKSError::HttpError(status.as_u16(), body.to_string()));
        }

        Ok(Response {
            status,
            headers,
            body,
        })
    }

    fn make_uri(&self, path: &str) -> Result<hyper::Uri, MKSError> {
        let url = self
            .base_endpoint
            .clone()
            .join(path)
            .map_err(|_| MKSError::UrlError)?;

        url.as_str()
            .parse::<hyper::Uri>()
            .map_err(|_| MKSError::UrlError)
    }
}

/// Builder for `MKS`.
pub struct Builder {
    /// Hyper client to use for the connection.
    client: Option<Client<HttpsConnector>>,

    /// Request timeout.
    timeout: Duration,
}

// Default timeout for requests.
const DEFAULT_TIMEOUT: u64 = 30;

impl Default for Builder {
    fn default() -> Self {
        Self {
            client: None,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT),
        }
    }
}

impl Builder {
    /// Set Hyper client.
    ///
    /// By default this library will instantiate a new HttpsConnector.
    /// It will use hyper_rustls or hyper_tls depending on selected library features.
    pub fn client(mut self, client: Client<HttpsConnector>) -> Self {
        self.client = Some(client);
        self
    }

    /// Set request timeout.
    ///
    /// Default is 30 seconds.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Create `MKS` with the configuration in this builder.
    pub fn build(self, base_endpoint: &str, token: &str) -> Result<MKS, MKSError> {
        MKS::with_builder(base_endpoint, token, self)
    }
}

/// Response represents a result of an HTTP request.
struct Response {
    status: StatusCode,
    headers: HeaderMap,
    body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_mks_default_builder() {
        let mks = MKS::new("https://example.org", "token_a").unwrap();

        assert_eq!(
            mks.base_endpoint,
            Url::parse("https://example.org").unwrap()
        );
        assert_eq!(mks.token, String::from("token_a"));
        assert_eq!(mks.user_agent, format!("{}/{}", PKG_NAME, PKG_VERSION));
        assert_eq!(mks.timeout, Duration::from_secs(DEFAULT_TIMEOUT));
    }

    #[test]
    fn new_mks_with_builder() {
        let mks = MKS::builder()
            .timeout(Duration::from_secs(10))
            .build("https://example.com", "token_b")
            .unwrap();

        assert_eq!(
            mks.base_endpoint,
            Url::parse("https://example.com").unwrap()
        );
        assert_eq!(mks.token, String::from("token_b"));
        assert_eq!(mks.user_agent, format!("{}/{}", PKG_NAME, PKG_VERSION));
        assert_eq!(mks.timeout, Duration::from_secs(10));
    }
}
