mod health;

use axum::Router;

pub fn router() -> Router {
    Router::new().merge(health::router())
}

pub async fn run(address: &str) -> std::io::Result<()> {
    let app = router();
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("Levianaut is running at http://{address}");
    axum::serve(listener, app).await
}
