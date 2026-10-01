// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Authored definition documents, exposed by reference.
//!
//! Consumers read QSpec's definition lock, its package-selection vectors and
//! the native diagnostics catalog through these constants instead of holding copies. The crate carries the
//! exact authored bytes and nothing else: no parsing, no recorded digests.
//! A consumer that needs a definition digest hashes the bytes itself, and the
//! lock's own digests are checked against the files they name by TC-233.

/// `proposals/quire-v1/definitions/complete-value-lock.json`: the
/// `quire.value.definition-lock/v1` lock for the complete value system.
pub const COMPLETE_VALUE_LOCK: &str =
    include_str!("../proposals/quire-v1/definitions/complete-value-lock.json");

/// `proposals/quire-v1/definitions/complete-value-selection-vectors.json`:
/// the accepted, refused and ordering vectors for the lock's package
/// selection.
pub const COMPLETE_VALUE_SELECTION_VECTORS: &str =
    include_str!("../proposals/quire-v1/definitions/complete-value-selection-vectors.json");

/// `proposals/quire-v1/definitions/native-diagnostics.md`: the
/// `quire.native.diagnostics/v1` interpretation catalog document.
pub const NATIVE_DIAGNOSTICS: &str =
    include_str!("../proposals/quire-v1/definitions/native-diagnostics.md");
