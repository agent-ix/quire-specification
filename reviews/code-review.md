---
id: SR-2026103004
title: "code-review review of STD-617 nested equality identity"
type: SpecReview
analysis: code-review
scope: "agent-ix/quire-specification; PR code and tests; tests/checked_package_v2.rs::tc_233_checked_term_data_vectors_cover_the_shared_carrier"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: reviews }
---

## Summary

Ticket: STD-617. PR: agent-ix/quire-specification#558. Verification: `cargo test --locked` passed all test targets (147 checked-package tests). Full and focused clippy checks fail on existing warnings in unchanged test code; full and focused rustfmt checks show baseline drift before the changed block. The equality key shape and term_path rules are explicit, and nested and/or positive cases and duplicate-key checks are present. The EQ-R6 negative vector does not provide a distinct graph context: it uses the same application identity, occurrence and [0] path as positive EQ-08, where that path is an equality. The test only checks the row against the JSON schema and its path value, not that traversal ends at a non-equality node. This leaves the reader-refusal evidence internally contradictory. #557’s target-union change is separate and compatible; STD-614 protocol term_ordinal remains distinct.

## Verdict

Review found one medium evidence/semantic defect. `EQ-R6-path-to-non-equality` is keyed to the same `and-equality-root` application and `[0]` path as positive `EQ-08-nested-and-first`. The authored positive case identifies that path as equality, so the negative case cannot prove a path-to-non-equality refusal without separate graph identity/context. The Rust test performs schema checks only for this refusal vector.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The non-equality path refusal reuses the same application, occurrence, and path as a positive nested equality, so the vector supplies contradictory graph meaning and cannot establish the claimed reader refusal. | proposals/checked-package-v2/checked-term-data-vectors.json:2022; tests/checked_package_v2.rs:19172; spec/objects/interfaces/FR-322-checked-package-artifact.md:3586 |

## Scope

| Unit | Path | Role | Excerpt |
| --- | --- | --- | --- |
| checked-term-data vectors | proposals/checked-package-v2/checked-term-data-vectors.json | examined | `term_path` differentiates nested equality entries and the EQ-R6 mutation claims a non-equality target. |
| TC-233 carrier test | tests/checked_package_v2.rs | examined | The EQ-R6 branch asserts schema validity, operator `equal`, and path `[0]`; it does not evaluate graph traversal. |

## Re-review at a2a3d2f977594396491805eb4e1c93becfdafabe

The typed `argument`, `member`, and `binding_value` path selectors now align with IR `CheckedTerm` traversal; positive vectors cover all three. FND-001 remains unresolved because EQ-R6 still has the same identity/path as positive EQ-08. `cargo test --locked` passed all targets (147 checked-package tests). Clippy still fails on existing warnings in unchanged test code. Full rustfmt reports broad baseline drift; focused rustfmt also reports the newly added assertion at `tests/checked_package_v2.rs:19025`.

## Additional findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-002 | low | The new binding-value assertion is not rustfmt-formatted; focused rustfmt reports a diff at the changed line. | tests/checked_package_v2.rs:19025 |

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | 1b1b7fac: EQ-R6 now has a distinct root application and checked-term context whose argument[0] is an `or` application; TC-233 asserts that root/path, the non-equality terminal, and the exact refusal code, cause, and pointer. |
| FND-002 | fixed | 1b1b7fac: rustfmt reformatted the binding-value assertion across three lines. |
