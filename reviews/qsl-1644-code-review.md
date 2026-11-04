---
id: SR-617005
title: 'Code review of PR #618: tc_303 window aggregate oracle and vectors (QSL-1644)'
type: SpecReview
analysis: code-review
scope: 'agent-ix/quire-specification@9d8696de; PR #618: tests/checked_package_v2.rs (aggregate oracle and tc_303_window_aggregate_arguments_admit_each_kind_and_refuse_at_the_first_defect), proposals/checked-package-v2/temporal-aggregate-vectors.json, Cargo.lock'
review_set: subset
date: '2026-11-04'
relationships:
- target: ix://agent-ix/quire-specification/TC-303
  type: reviews
---

## Summary

Ticket: QSL-1644. Rust lane included (rust-review). The new test passes, as do `tc_233_operation_catalog_is_closed_and_bound_to_both_schemas` and `tc_233_every_v2_application_names_exactly_one_catalogued_operation_identity`, run with the locked build into a fresh target dir.

The oracle `aggregate_argument_refusal` runs FR-370 steps 1-5 in the stated order. Profile is checked first. Names, duplicates and order are checked per member, in member order. Kind follows, then excluded bindings in member order and missing bindings in table order. Values come last, in table order: `lower > upper` is checked after `upper`, and the unit check on `upper` comes before that comparison.

Each of the 23 reader cases differs from its `base` positive by one mutation, verified by diffing every case against its base. Each expected code, cause and pointer equals the first defect under that ladder (pointer indices re-derived by hand). The rational spellings `1` (AR-14) and `2` (AR-22) are canonical under FR-322 "Scalar literal values".

Lock move: quire-canonical e1e7826..35987f1 changes only AGENTS.md and CLAUDE.md, with no code. quire-verification-contracts 5cc7b34..5070338 brings the catalog entry (#23), and also #22 and #26. #26 removes `digest`/`sourceRevision` from the e02-draft-2 schema, which no test here reads. Its other consumers are `jcs_canonicalize` and the catalog, and the catalog only gains one entry. No side effects found beyond the catalog test.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| aggregate_argument_refusal and helpers | tests/checked_package_v2.rs:19906 | examined |
| tc_303_window_aggregate_arguments_admit_each_kind_and_refuse_at_the_first_defect | tests/checked_package_v2.rs:20062 | examined |
| temporal-aggregate-vectors.json (9 positive, 23 reader) | proposals/checked-package-v2/temporal-aggregate-vectors.json | examined |
| Cargo.lock quire-canonical, quire-verification-contracts | Cargo.lock:486 | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | TC-303 step 15 says each reader case is "a single mutation of a positive", and every reader row carries `base`, but the test never reads `base`. A future case could mutate two things, and the oracle would report only the first, masking the second. Fix: assert that the case's argument differs from `positive[base].argument` in exactly one member (or only in the profile) | tests/checked_package_v2.rs:20098 |

## Verdict

Mergeable for the Rust and vector logic. The code is idiomatic for this test file: `let-else`, borrowed `Value` families, and `BigInt` cross-multiplication for exact comparison. The one finding is a low-severity hardening.
