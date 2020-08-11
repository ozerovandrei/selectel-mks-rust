mod error;

use error::Error;
use std::time::Duration;
use url::Url;

// Hyper imports.
use hyper::Client;
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
    pub fn new(base_endpoint: &str, token: &str) -> Result<MKS, Error> {
        MKS::with_builder(base_endpoint, token, MKS::builder())
    }

    fn with_builder(base_endpoint: &str, token: &str, builder: Builder) -> Result<MKS, Error> {
        // Check token.
        if token.is_empty() {
            return Err(Error::TokenError);
        }
        let token = String::from(token);

        // Check base endpoint.
        let base_endpoint = Url::parse(base_endpoint).map_err(|_| Error::EndpointError)?;

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
    pub fn build(self, base_endpoint: &str, token: &str) -> Result<MKS, Error> {
        MKS::with_builder(base_endpoint, token, self)
    }
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
