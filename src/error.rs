/// MKS error return type.
#[derive(Debug)]
pub enum Error {
    /// Bad endpoint value.
    EndpointError,

    /// Bad token value.
    TokenError,
}
