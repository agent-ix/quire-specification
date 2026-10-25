---
id: SR-13615
title: 'Code review of QSpec PR #304: closed stage-limit cause set test line'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-specification@90d6c3f; git diff origin/main...90d6c3f (PR #304, branch qspec-hyper-prob-followups): tests/checked_package_v2.rs; tickets QSL-553, QSL-554, QSL-555, QSL-573, QSL-883'
review_set: subset
date: '2026-10-21'
relationships:
- target: ix://agent-ix/quire-specification/FR-272
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Tickets: QSL-553, QSL-554, QSL-555, QSL-573, QSL-883. PR: quire-specification#304 (private), head 90d6c3fd3c85644fd43799464ed7400b9ea83a8a, branch qspec-hyper-prob-followups, scoped to `git diff origin/main...HEAD` (one commit, 16 files, +113/-31). Verified against merged QSL main (6eceb158b #1125 and 133e7f9f7 #1124): ADR-023 HM-7, PH-4/PH-5, 'What EN-1 admits', HV-8, PA-3, PA-6, CS-1 to CS-4; FR-174, FR-177, FR-178, FR-183, FR-184, FR-185 'Exact values', FR-096 'A stage limit names its kind'; and QSL code qsl-analyze/src/model_check/hyper.rs (NoPartnerPath, Halt::WitnessSet count), qsl-replay/src/model_check/hyper_replay.rs and settle.rs, qsl-semantics/src/check/hyper_clause.rs (copy group on HP-2 and HP-6), qsl-foundation/src/diagnostic/stage.rs and setting.rs (RationalBits, s3.rational_bits), qsl-semantics/src/check/check/typing.rs (quantity magnitude measured). The only code change is one line in tests/checked_package_v2.rs: `"rational-bits-exceeded"` joins the expected `stage_limit_exceeded` cause set of `tc_233_stage_limit_and_duplicate_selection_cause_sets_are_closed` (`#[trace("TC-233", "FR-272-AC-10")]`). The test compares that set exactly against both the native-diagnostics row and the FR-272 row, so the line is load-bearing. Mutation check: removing `rational-bits-exceeded` from the FR-272 row alone fails the test at tests/checked_package_v2.rs:8456 ('FR-272 `stage_limit_exceeded` row lists exactly its causes'); reverted. The trace tag is correct. Gates at HEAD (own clone, own CARGO_TARGET_DIR): `quire validate --scope . 'spec/**/*.md'` reports 134 failed documents, byte-identical to origin/main's output after path normalisation; of the 15 touched documents only FR-272 appears, with its one pre-existing failure, so the PR adds none. `cargo test --test checked_package_v2 --test backend_provider_v1`: 112 and 11 passed. quoin 0.28.3; quire 0.36.2 (engine 0.50.2), newer than the 0.36.1 the rollout note names, with no behaviour difference observed.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| tc_233_stage_limit_and_duplicate_selection_cause_sets_are_closed | tests/checked_package_v2.rs | examined |
| FR-272-AC-10 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md | context_only |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Verdict

Clean. The added expected cause matches both edited rows and FR-272-AC-10, and the test fails when either row drops it.
