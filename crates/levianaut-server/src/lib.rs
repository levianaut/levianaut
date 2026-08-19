mod health;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(health::router())
}

pub async fn run(address: &str) -> std::io::Result<()> {
    let app = router();
    let listener = tokio::net::TcpListener::bind(address).await?;
    let shutdown = shutdown_signal()?;

    println!("Levianaut is running at http://{address}");
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown)
        .await
}

#[cfg(unix)]
fn shutdown_signal() -> std::io::Result<impl Future<Output = ()>> {
    use tokio::signal::unix::{SignalKind, signal};

    let mut interrupt = signal(SignalKind::interrupt())?;
    let mut terminate = signal(SignalKind::terminate())?;

    Ok(async move {
        tokio::select! {
            _ = interrupt.recv() => {}
            _ = terminate.recv() => {}
        }
    })
}
