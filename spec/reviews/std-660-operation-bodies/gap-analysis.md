---
id: SR-202611101500008
title: "Gap analysis of the STD-660 operation_bodies carrier"
type: SpecReview
analysis: gap-analysis
scope: "agent-ix/quire-specification; PR #651 at 6ff1281f: FR-322-AC-397, TC-987, tests/checked_package_v2.rs read_operation_bodies and operation_body_carrier_vectors_match_the_closed_member_schema_and_reader"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: references }
---

## Summary

Ticket: STD-660. Planless gap analysis scoped to the PR diff, run with
`quire matrix --scope . --format tsv` (quire 0.41.0, engine 0.57.0) on the PR head. The new test is
real and discriminating, but the criterion it is tagged with does not exist in the computed matrix.

## Verdict

FAIL on FND-001. Evidence is not hollow. `cargo test --locked --test checked_package_v2` passes
168/168, and the new test passes. A local mutation run against the reference reader and the schema
killed all 12 mutants. Each was killed by the vector meant for it, or by a panic on the positive
cases:
- removed reader checks: candidate (OBR-13), key-set (OBR-18), coverage (OBR-16), shared node
  (OBR-12), function tag (OBR-11), key order (OBR-15), duplicate key (OBR-14), document owner
  (OBR-05), has_body agreement and declaring form (panics);
- schema changes: an open `no_body` arm (OBS-10) and `minItems: 1` (an explicit assertion).

The semantic review ran at reviewer level, using the mutation run and a probe vector. Its
spec-side result is in the failure-domain SpecReview. No reverse gap: the reader is test-only
code that implements the section.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-322-AC-397 is not a minted criterion, so the `#[trace("TC-987", "FR-322-AC-397")]` tag on `operation_body_carrier_vectors_match_the_closed_member_schema_and_reader` binds nothing. `quire matrix` has no FR-322-AC-397 row and warns `untracked-id-has-minted-children` at tests/checked_package_v2.rs:22660. The criterion goes unverified in the matrix even though the test passes. The cause is the blank line before the row (base SpecReview FND-001). | tests/checked_package_v2.rs:22660; spec/objects/interfaces/FR-322-checked-package-artifact.md:4776 |

## Coverage

- Criteria in scope: 1 (FR-322-AC-397), which is untagged-equivalent because it is not minted.
- Tests in scope: 1, tagged and passing.
- Semantic review: run at reviewer level by the mutation run.
- Plan completion: not assessed

## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | At 498a3593, `quire matrix` binds FR-322-AC-397 to `operation_body_carrier_vectors_match_the_closed_member_schema_and_reader` (status tagged), and the `untracked-id-has-minted-children` warning for 397 is gone. |
