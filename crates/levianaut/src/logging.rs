// SPDX-FileCopyrightText: 2026 Piotr Szpetkowski and contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

use tracing_subscriber::{EnvFilter, filter::LevelFilter};

/// The log directives used when `RUST_LOG` is not set.
const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::INFO;

/// Installs the global log subscriber for the process.
///
/// Log verbosity is taken from the `RUST_LOG` environment variable, falling
/// back to [`DEFAULT_LOG_LEVEL`] when it is not set.
pub(crate) fn init() {
    let filter = EnvFilter::builder()
        .with_default_directive(DEFAULT_LOG_LEVEL.into())
        .from_env_lossy();

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
