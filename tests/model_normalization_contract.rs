// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (C) 2026 Agent-IX

//! Contract-artifact audits for the proposed model normalization replacement.
//! These assert that the authored obligations and procedures stay explicit;
//! they do not execute or qualify QSL's normalizer or strict reader.

use ix_trace_rs::trace;

const FR_150: &str =
    include_str!("../spec/functional/type-model/FR-150-preserve-model-normalization-provenance.md");
const TC_195: &str = include_str!("../spec/test-cases/TC-195-model-normalization-provenance.md");

#[trace("TC-195", "FR-150-AC-9")]
#[test]
fn tc_195_contract_keeps_unqualified_original_inventory() {
    assert!(FR_150.contains("without such a rule SHALL NOT receive an invented qualify fact"));
    assert!(
        FR_150.contains("existing selection data, not an added effective-identity preimage member")
    );
}

#[trace("TC-195", "FR-150-AC-10")]
#[test]
fn tc_195_contract_preserves_atomic_denial_semantics() {
    assert!(FR_150.contains("Denial performs none of its named work"));
    assert!(FR_150.contains("no effective view"));
}

#[trace("TC-195", "FR-150-AC-11")]
#[test]
fn tc_195_contract_orders_ancestor_and_work_denial() {
    assert!(
        FR_150.contains("unavailable owning work charge wins before a simultaneous edge denial")
    );
    assert!(FR_150.contains("previously admitted axis work is never refunded"));
}

#[trace("TC-195", "FR-150-AC-12")]
#[test]
fn tc_195_contract_requires_exact_prefix_expansion() {
    assert!(FR_150.contains("every original ordered fact, input key, rule and zero-based ordinal"));
    assert!(
        FR_150.contains("phase-3 inheritance ladder SHALL be admitted as a canonical prefix run")
    );
    assert!(TC_195.contains(
        "Independently expand and compare smaller ladders including shared tails/diamonds"
    ));
}

#[trace("TC-195", "FR-150-AC-13")]
#[test]
fn tc_195_contract_requires_complete_strict_reader_transport() {
    assert!(FR_150.contains("complete retained provenance transport"));
    assert!(FR_150.contains("strict reader SHALL verify admitted selection"));
    assert!(TC_195.contains("feed those exact bytes to an independent strict reader"));
    assert!(TC_195.contains("root-only/requested-membership proof"));
}

#[trace("TC-195", "FR-150-AC-14")]
#[test]
fn tc_195_contract_requires_writer_price_and_prework_denial() {
    assert!(FR_150
        .contains("analytically priced sole-writer JCS byte length before encoding or hashing"));
    assert!(TC_195.contains("analytical price equals the sole writer's returned length"));
    assert!(TC_195.contains("encodes/hashes no denied node"));
}

#[trace("TC-195", "FR-150-AC-16")]
#[test]
fn tc_195_contract_states_full_size_deterministic_and_memory_bounds() {
    assert!(FR_150.contains("hashed_bytes ≤ 268435456"));
    assert!(FR_150.contains("work_units ≤ 16777216"));
    assert!(FR_150.contains("peak resident memory ≤ 21474836480 bytes"));
    assert!(FR_150.contains("pending owner decision e11"));
    assert!(TC_195.contains("50,015,001 facts"));
    assert!(TC_195.contains("do not infer elapsed time from an extrapolation"));
}
