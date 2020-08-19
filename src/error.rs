/// MKS error return type.
#[derive(Debug)]
pub enum Error {
    /// Failed to deserialize response body.
    DeserializeError(serde_json::Error, String),

    /// Failed to serialize a struct into request body.
    SerializeError(serde_json::Error),

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

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            Error::DeserializeError(err, body) => {
                format!("failed to deserialize body: {}, error: {}", err, body).fmt(f)
            }
            Error::SerializeError(err) => {
                format!("failed to serialize a struct, error: {}", err).fmt(f)
            }
            Error::EndpointError => "failed to parse base endpoint URL".fmt(f),
            Error::EmptyTokenError => "token cannot be empty".fmt(f),
            Error::HttpError(status, err) => {
                format!("bad status code: {}, error body: {}", status, err).fmt(f)
            }
            Error::HyperError(err) => {
                format!("failed to make the request due to Hyper error: {}", err).fmt(f)
            }
            Error::RequestError => "failed to build a new request".fmt(f),
            Error::TimeoutError => "request timed out".fmt(f),
            Error::UrlError => "failed to parse URL for request".fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl std::convert::From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Error::HyperError(e)
    }
}

impl std::convert::From<tokio::time::Elapsed> for Error {
    fn from(_e: tokio::time::Elapsed) -> Self {
        Error::TimeoutError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impl_error() {
        #[derive(Debug)]
        struct B(Option<Box<dyn std::error::Error + 'static>>);

        impl std::fmt::Display for B {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "B")
            }
        }

        impl std::error::Error for B {}

        let err = B(Some(Box::new(Error::RequestError)));

        let _err = &err as &(dyn std::error::Error);
    }
}
