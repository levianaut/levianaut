// SPDX-FileCopyrightText: 2026 Piotr Szpetkowski and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

mod error;
mod health;

use axum::Router;
use axum::serve::ListenerExt;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

use error::{Error, Result};

pub fn router() -> Router {
    Router::new().merge(health::router())
}

pub async fn run(address: SocketAddr) -> Result<()> {
    let app = router().layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|source| Error::Bind { address, source })?;
    let shutdown = shutdown_signal()?;

    // `address` may ask for port 0, in which case the OS picks the real port.
    let address = listener.local_addr().unwrap_or(address);

    tracing::info!("Levianaut is running at http://{address}");
    axum::serve(listener.limit_connections(512), app)
        .with_graceful_shutdown(shutdown)
        .await;

    tracing::info!("Levianaut has shut down");
    Ok(())
}

#[cfg(unix)]
fn shutdown_signal() -> Result<impl Future<Output = ()>> {
    use tokio::signal::unix::{SignalKind, signal};

    let mut interrupt = signal(SignalKind::interrupt()).map_err(|source| Error::Signal {
        signal: "SIGINT",
        source,
    })?;
    let mut terminate = signal(SignalKind::terminate()).map_err(|source| Error::Signal {
        signal: "SIGTERM",
        source,
    })?;

    Ok(async move {
        let signal = tokio::select! {
            _ = interrupt.recv() => "SIGINT",
            _ = terminate.recv() => "SIGTERM",
        };

        tracing::info!("received {signal}, waiting for open connections to finish");
    })
}
