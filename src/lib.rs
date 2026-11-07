// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Authored definition documents, exposed by reference.
//!
//! Consumers read QSpec's package-selection vectors and
//! the native diagnostics catalog through these constants instead of holding copies. The crate carries the
//! exact authored bytes, plus the typed [`QUALIFICATION_VECTORS`] table that
//! `build.rs` generates from the authored test-case rows.

/// `proposals/quire-v1/definitions/complete-value-selection-vectors.json`:
/// the accepted, refused and ordering vectors for the complete-value
/// package selection.
pub const COMPLETE_VALUE_SELECTION_VECTORS: &str =
    include_str!("../proposals/quire-v1/definitions/complete-value-selection-vectors.json");

/// `proposals/quire-v1/definitions/native-diagnostics.md`: the
/// `quire.native.diagnostics/v1` interpretation catalog document.
pub const NATIVE_DIAGNOSTICS: &str =
    include_str!("../proposals/quire-v1/definitions/native-diagnostics.md");

/// The complete `acme/colors` Semantic IR 2.0.0 model with the two nullary
/// `Color` cases, exposed for consumers' enum-comparison inputs.
pub const ACME_COLORS_DOMAIN_PACKAGE: &str =
    include_str!("../proposals/checked-package-v2/domain-package-acme-colors.json");

/// Native source selecting [`ACME_COLORS_DOMAIN_PACKAGE`] and declaring both
/// truth arms of equality and inequality over `Red` and `Green`. These are
/// authored input bytes, not a recorded runtime or generated-code agreement.
pub const ACME_COLORS_COMPARISON_SOURCE: &str =
    include_str!("../proposals/checked-package-v2/model-enum-comparison.native");

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

/// A test case whose authored vector table is exported.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TestCase {
    /// `spec/test-cases/TC-185-exact-decimal-semantics.md`.
    Tc185,
    /// `spec/test-cases/TC-186-text-and-enum-identity-semantics.md`.
    Tc186,
    /// `spec/test-cases/TC-187-quantity-and-unit-conversion.md`.
    Tc187,
    /// `spec/test-cases/TC-192-integer-division-profiles.md`.
    Tc192,
    /// `spec/test-cases/TC-193-ieee-exceptional-and-rounding-profiles.md`.
    Tc193,
}

impl TestCase {
    /// Every exported test case, in id order.
    pub const ALL: [TestCase; 5] = [
        TestCase::Tc185,
        TestCase::Tc186,
        TestCase::Tc187,
        TestCase::Tc192,
        TestCase::Tc193,
    ];

    /// The authored test-case id, for example `TC-185`.
    pub const fn id(self) -> &'static str {
        match self {
            TestCase::Tc185 => "TC-185",
            TestCase::Tc186 => "TC-186",
            TestCase::Tc187 => "TC-187",
            TestCase::Tc192 => "TC-192",
            TestCase::Tc193 => "TC-193",
        }
    }
}

/// One authored vector row of a test case's `Vector` table. The text fields
/// are the authored cells, trimmed; nothing is interpreted or re-spelled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QualificationVector {
    /// The test case the row belongs to.
    pub test_case: TestCase,
    /// The authored vector id, for example `D01` or `DIV-08`.
    pub id: &'static str,
    /// The authored operation, inputs, profile or domain cell.
    pub inputs: &'static str,
    /// The authored expected value, disposition, bits or result cell.
    pub expected: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/vectors.rs"));

/// The authored vectors of one test case, in authored order.
pub fn vectors_of(test_case: TestCase) -> impl Iterator<Item = &'static QualificationVector> {
    QUALIFICATION_VECTORS
        .iter()
        .filter(move |v| v.test_case == test_case)
}
