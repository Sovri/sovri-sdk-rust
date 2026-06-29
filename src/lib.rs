// Copyright 2026 Sovri contributors
// SPDX-License-Identifier: Apache-2.0

//! Sovri SDK — shared types and agent/cloud integration contracts.
//!
//! Placeholder crate scaffolded by MAT-81. The real shared contracts
//! (framework, control, rule, evidence, gap) land with MAT-83. This crate
//! must keep building offline with no secrets — see the repository README.

/// SDK contract version, surfaced so the agent and Cloud can report a
/// consistent integration-contract version.
pub const SDK_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the SDK contract version string.
#[must_use]
pub fn version() -> &'static str {
    SDK_VERSION
}

#[cfg(test)]
mod tests {
    use super::version;

    #[test]
    fn version_matches_crate_version() {
        assert_eq!(version(), env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn version_is_non_empty() {
        assert!(!version().is_empty());
    }
}
