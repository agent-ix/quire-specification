// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Authored definition documents, exposed by reference.
//!
//! Consumers read QSpec's definition lock, its package-selection vectors and
//! the native diagnostics catalog through these constants instead of holding copies. The crate carries the
//! exact authored bytes and nothing else: no parsing.

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

/// `proposals/quire-v1/definitions/value-accounting-charge-point-vectors.json`:
/// the complete charge-point spelling list of `quire.value.accounting/v1`'s
/// ordered table, with spellings the table does not declare. A kernel compares
/// its own `ChargePoint` set with this list.
pub const VALUE_ACCOUNTING_CHARGE_POINT_VECTORS: &str =
    include_str!("../proposals/quire-v1/definitions/value-accounting-charge-point-vectors.json");

/// `proposals/quire-v1/definitions/value-accounting-limit-kind-vectors.json`:
/// the complete `ScalarLimitsV1` counter spelling list of
/// `quire.value.accounting/v1`, in field order, with spellings that are not
/// `ScalarLimitsV1` counters. It does not cover the `limit_kind` values of
/// `PopulationAdmissionLimitsV1` or `ModelNormalizationLimitsV1`. A kernel compares its own
/// `LimitKind` set with this list.
pub const VALUE_ACCOUNTING_LIMIT_KIND_VECTORS: &str =
    include_str!("../proposals/quire-v1/definitions/value-accounting-limit-kind-vectors.json");
