// SPDX-FileCopyrightText: 2026 Piotr Szpetkowski and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{Router, http::StatusCode, routing::get};

pub(crate) fn router() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> StatusCode {
    StatusCode::OK
}
