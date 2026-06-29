# sovri-sdk-rust

Shared Rust types and agent/cloud integration contracts for the Sovri
project-level compliance platform. Placeholder crate scaffolded by MAT-81; the
real schemas land with MAT-83.

## Status

Foundational scaffold. No production contracts yet — the crate exposes a
version helper and builds offline, so dependent repositories (`sovri-agent`,
`sovri-frameworks`) start on a real foundation rather than a placeholder.

## Development

This crate builds, tests, and lints with the standard Rust toolchain. No
network access and no secrets are required.

- Build: `cargo build`
- Test: `cargo test`
- Lint: `cargo fmt --check && cargo clippy --all-targets -- -D warnings`

The same gates run in CI (`.github/workflows/ci.yml`) on every pull request.
Local Git hooks mirroring them are declared in `lefthook.yml`.

## Community and Open Core

Sovri follows an open-core model: an Apache-2.0 Community edition plus a
proprietary managed Cloud edition.

- This repository is **Community**, licensed under **Apache-2.0** (see
  `LICENSE`). Every source file carries an `SPDX-License-Identifier: Apache-2.0`
  header.
- Proprietary Cloud code lives in separate private repositories and never ships
  here. Cloud may depend on these public contracts; this repository never
  depends on Cloud.

## Air-gap and offline execution

Sovri compliance runs in regulated, frequently air-gapped environments.

- `cargo build` and `cargo test` succeed with no network connectivity and no
  configured secrets or credentials.
- The crate performs no I/O and opens no network connection at runtime.
- No environment variables are required beyond the operating-system defaults.

## License

Apache-2.0. See `LICENSE` and `NOTICE`.
