use error::Error;
use std::time::Duration;
use tokio::time::timeout;
use url::Url;

// Hyper imports.
use hyper::body::Buf;
use hyper::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, USER_AGENT};
use hyper::{Method, Request};
#[cfg(feature = "rustls")]
type HttpsConnector = hyper_rustls::HttpsConnector<hyper::client::HttpConnector>;
#[cfg(feature = "rust-native-tls")]
use hyper_tls;
#[cfg(feature = "rust-native-tls")]
type HttpsConnector = hyper_tls::HttpsConnector<hyper::client::HttpConnector>;

pub mod error;
pub mod resource_url;

pub mod cluster;
pub mod kubeversion;
pub mod node;
pub mod nodegroup;

// Environment variables from Cargo.
static PKG_NAME: &str = env!("CARGO_PKG_NAME");
static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// `Client` struct is used to make calls to the MKS API.
pub struct Client {
    client: hyper::Client<HttpsConnector>,
    token: String,
    base_endpoint: url::Url,
    user_agent: String,
    timeout: Duration,
}

impl Client {
    /// Construct the new Client struct with default configuration.
    ///
    /// Use `Builder` to configure the client.
    pub fn new(base_endpoint: &str, token: &str) -> Result<Client, Error> {
        Client::with_builder(base_endpoint, token, Client::builder())
    }

    fn with_builder(base_endpoint: &str, token: &str, builder: Builder) -> Result<Client, Error> {
        // Check token.
        if token.is_empty() {
            return Err(Error::EmptyTokenError);
        }
        let token = String::from(token);

        // Check base endpoint.
        let base_endpoint = Url::parse(base_endpoint).map_err(|_| Error::EndpointError)?;

        // Use the provided Hyper client or configure a new one.
        let client = match builder.client {
            Some(client) => client,
            None => {
                #[cfg(feature = "rustls")]
                let client = hyper::Client::builder().build(HttpsConnector::new());
                #[cfg(feature = "rust-native-tls")]
                let client = hyper::Client::builder().build(HttpsConnector::new()?);

                client
            }
        };

        Ok(Client {
            client,
            token,
            base_endpoint,
            user_agent: Client::user_agent(),
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
        &self,
        method: Method,
        path: &str,
        body: Option<String>,
    ) -> Result<Request<hyper::Body>, Error> {
        // Build a final Hyper URI.
        let uri = self.make_uri(path)?;

        // Prepare a new Hyper request.
        let mut req = Request::new(hyper::Body::empty());
        *req.method_mut() = method;
        *req.uri_mut() = uri;

        // Add user-agent header.
        req.headers_mut().insert(
            USER_AGENT,
            HeaderValue::from_str(self.user_agent.as_str()).map_err(|_| Error::RequestError)?,
        );

        // Add x-auth-token header.
        req.headers_mut().insert(
            "x-auth-token",
            HeaderValue::from_str(self.token.as_str()).map_err(|_| Error::RequestError)?,
        );

        // Add body into the new request if it's provided.
        if let Some(body) = body {
            // Add content-length header if body is provided.
            let len =
                HeaderValue::from_str(&body.len().to_string()).map_err(|_| Error::RequestError)?;
            req.headers_mut().insert(CONTENT_LENGTH, len);

            // Add content-type header if body is provided.
            req.headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_str("application/json").map_err(|_| Error::RequestError)?,
            );

            *req.body_mut() = hyper::Body::from(body);
        }

        Ok(req)
    }

    #[tokio::main]
    async fn do_request(&self, req: hyper::Request<hyper::Body>) -> Result<String, Error> {
        let duration = self.timeout;
        let handle = async {
            let raw_resp = self.client.request(req).await?;

            let status = raw_resp.status();
            let body = hyper::body::aggregate(raw_resp).await?.to_bytes();
            let body = String::from_utf8_lossy(&body);

            Ok::<_, hyper::Error>((body.to_string(), status))
        };

        let raw_resp = timeout(duration, handle).await??;

        let (body, status) = raw_resp;

        if !status.is_success() {
            return Err(Error::HttpError(status.as_u16(), body));
        }

        Ok(body)
    }

    fn make_uri(&self, path: &str) -> Result<hyper::Uri, Error> {
        let url = self
            .base_endpoint
            .clone()
            .join(path)
            .map_err(|_| Error::UrlError)?;

        url.as_str()
            .parse::<hyper::Uri>()
            .map_err(|_| Error::UrlError)
    }
}

/// Methods to work with Kubernetes versions.
impl Client {
    /// List all Kubernetes versions.
    pub fn list_kube_versions(&self) -> Result<Vec<kubeversion::schemas::KubeVersion>, Error> {
        kubeversion::api::list_kube_versions(self)
    }
}

/// Methods to work with nodegroups.
impl Client {
    /// Get a cluster nodegroup.
    pub fn get_nodegroup(
        &self,
        cluster_id: &str,
        nodegroup_id: &str,
    ) -> Result<nodegroup::schemas::Nodegroup, Error> {
        nodegroup::api::get_nodegroup(self, cluster_id, nodegroup_id)
    }

    /// List cluster nodegroups.
    pub fn list_nodegroups(
        &self,
        cluster_id: &str,
    ) -> Result<Vec<nodegroup::schemas::Nodegroup>, Error> {
        nodegroup::api::list_nodegroups(self, cluster_id)
    }

    /// Create a cluster nodegroup.
    pub fn create_nodegroup(
        &self,
        cluster_id: &str,
        opts: &nodegroup::schemas::CreateOpts,
    ) -> Result<(), Error> {
        nodegroup::api::create_nodegroup(self, cluster_id, opts)
    }

    /// Delete a cluster nodegroup.
    pub fn delete_nodegroup(&self, cluster_id: &str, nodegroup_id: &str) -> Result<(), Error> {
        nodegroup::api::delete_nodegroup(self, cluster_id, nodegroup_id)
    }

    /// Resize a cluster nodegroup.
    pub fn resize_nodegroup(
        &self,
        cluster_id: &str,
        nodegroup_id: &str,
        opts: &nodegroup::schemas::ResizeOpts,
    ) -> Result<(), Error> {
        nodegroup::api::resize_nodegroup(self, cluster_id, nodegroup_id, opts)
    }

    /// Update a cluster nodegroup.
    pub fn update_nodegroup(
        &self,
        cluster_id: &str,
        nodegroup_id: &str,
        opts: &nodegroup::schemas::UpdateOpts,
    ) -> Result<(), Error> {
        nodegroup::api::update_nodegroup(self, cluster_id, nodegroup_id, opts)
    }
}

/// Methods to work with clusters.
impl Client {
    /// Get a cluster.
    pub fn get_cluster(&self, cluster_id: &str) -> Result<cluster::schemas::Cluster, Error> {
        cluster::api::get_cluster(self, cluster_id)
    }

    /// List clusters.
    pub fn list_clusters(&self) -> Result<Vec<cluster::schemas::Cluster>, Error> {
        cluster::api::list_clusters(self)
    }

    /// Create a cluster.
    pub fn create_cluster(
        &self,
        opts: &cluster::schemas::CreateOpts,
    ) -> Result<cluster::schemas::Cluster, Error> {
        cluster::api::create_cluster(self, opts)
    }

    /// Delete a cluster.
    pub fn delete_cluster(&self, cluster_id: &str) -> Result<(), Error> {
        cluster::api::delete_cluster(self, cluster_id)
    }
}

/// Builder for `Client`.
pub struct Builder {
    /// Hyper client to use for the connection.
    client: Option<hyper::Client<HttpsConnector>>,

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
    pub fn client(mut self, client: hyper::Client<HttpsConnector>) -> Self {
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

    /// Create `Client` with the configuration in this builder.
    pub fn build(self, base_endpoint: &str, token: &str) -> Result<Client, Error> {
        Client::with_builder(base_endpoint, token, self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client_default_builder() {
        let client = Client::new("https://example.org", "token_a").unwrap();

        assert_eq!(
            client.base_endpoint,
            Url::parse("https://example.org").unwrap()
        );
        assert_eq!(client.token, String::from("token_a"));
        assert_eq!(client.user_agent, format!("{}/{}", PKG_NAME, PKG_VERSION));
        assert_eq!(client.timeout, Duration::from_secs(DEFAULT_TIMEOUT));
    }

    #[test]
    fn new_client_with_builder() {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build("https://example.com", "token_b")
            .unwrap();

        assert_eq!(
            client.base_endpoint,
            Url::parse("https://example.com").unwrap()
        );
        assert_eq!(client.token, String::from("token_b"));
        assert_eq!(client.user_agent, format!("{}/{}", PKG_NAME, PKG_VERSION));
        assert_eq!(client.timeout, Duration::from_secs(10));
    }
}
