// SPDX-FileCopyrightText: 2026 Piotr Szpetkowski and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    levianaut::run().await
}
