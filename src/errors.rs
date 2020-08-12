/// MKS error return type.
#[derive(Debug)]
pub enum MKSError {
    /// Bad endpoint value.
    EndpointError,

    /// Empty token value.
    EmptyTokenError,

    /// HTTP response contains bad status code.
    HttpError(u16, String),

    /// Failed to perform HTTP request with Hyper.
    HyperError(hyper::Error),

    /// Error while building a new request.
    RequestError,

    /// Request timed out.
    TimeoutError,

    /// Bad URL for a new request.
    UrlError,
}

impl std::fmt::Display for MKSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            MKSError::EndpointError => "failed to parse base endpoint URL".fmt(f),
            MKSError::EmptyTokenError => "token cannot be empty".fmt(f),
            MKSError::HttpError(status, err) => {
                format!("bad status code: {}, error body: {}", status, err).fmt(f)
            }
            MKSError::HyperError(err) => {
                format!("failed to make the request due to Hyper error: {}", err).fmt(f)
            }
            MKSError::RequestError => "failed to build a new request".fmt(f),
            MKSError::TimeoutError => "request timed out".fmt(f),
            MKSError::UrlError => "failed to parse URL for request".fmt(f),
        }
    }
}

impl std::convert::From<hyper::Error> for MKSError {
    fn from(e: hyper::Error) -> Self {
        MKSError::HyperError(e)
    }
}

impl std::convert::From<tokio::time::Elapsed> for MKSError {
    fn from(_e: tokio::time::Elapsed) -> Self {
        MKSError::TimeoutError
    }
}
