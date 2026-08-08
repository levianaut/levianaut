#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = axum::Router::new();

    let address = "127.0.0.1:8096";
    let listener = tokio::net::TcpListener::bind(address).await?;

    println!("Levianaut is running at http://{address}");
    axum::serve(listener, app).await
}
