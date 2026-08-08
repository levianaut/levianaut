use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use levianaut_server::router;

#[tokio::test]
async fn get_health_returns_ok() {
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    let response = router().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
