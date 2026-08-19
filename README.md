# Levianaut

A free, open-source and modern video media server.

> [!WARNING]
> Levianaut is in very early development. APIs, configuration,
> project structure and other things may change without notice.
> It is not currently intended for usage except for development and testing.

## Development

Levianaut requires Rust 1.97 or later.

Build Levianaut from source with:
```sh
cargo build -p levianaut
```

Start the server with:
```sh
cargo run -p levianaut -- server
```

By default, the server listens on `127.0.0.1:8096`.
To use a different address, pass it with `--addr`:
```sh
cargo run -p levianaut -- server --addr 127.0.0.1:18096
```

The server exposes a health check at `/health` which responds with HTTP `200 OK`:
```sh
curl -i http://127.0.0.1:8096/health
```

## License

Copyright (C) 2026 Piotr Szpetkowski and contributors

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

See [LICENSE](LICENSE) for the full license text.
