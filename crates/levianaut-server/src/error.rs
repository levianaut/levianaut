use std::net::SocketAddr;

/// An error that prevented the server from starting or running to completion.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not listen on {address}")]
    Bind {
        address: SocketAddr,
        source: std::io::Error,
    },

    #[error("could not install the {signal} signal handler")]
    Signal {
        signal: &'static str,
        source: std::io::Error,
    },

    #[error("the server stopped unexpectedly")]
    Serve(#[source] std::io::Error),
}

/// A [`Result`](std::result::Result) with the server's [`Error`] type.
pub type Result<T> = std::result::Result<T, Error>;
