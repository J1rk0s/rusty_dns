use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("Failed to resolve packet")]
    GenericResolveError,

    #[error("Failed to resolve IPv4 for packet")]
    Ipv4ResolveError,

    #[error("Failed to resolve IPv6 for packet")]
    Ipv6ResolveError,

    #[error("Failed to resolve PTR for packet")]
    PtrResolveError,
}