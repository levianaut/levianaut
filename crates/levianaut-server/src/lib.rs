// SPDX-FileCopyrightText: 2026 Piotr Szpetkowski and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

mod error;
mod health;

use axum::Router;
use std::net::SocketAddr;

use error::{Error, Result};

pub fn router() -> Router {
    Router::new().merge(health::router())
}

pub async fn run(address: SocketAddr) -> Result<()> {
    let app = router();
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .map_err(|source| Error::Bind { address, source })?;
    let shutdown = shutdown_signal()?;

    println!("Levianaut is running at http://{address}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(Error::Serve)
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
        tokio::select! {
            _ = interrupt.recv() => {}
            _ = terminate.recv() => {}
        }
    })
}
