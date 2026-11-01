---
id: SR-11771
title: "Rust review of STD-616 / PR #557"
type: SpecReview
analysis: code-review
scope: "spec/objects/interfaces/FR-322-checked-package-artifact.md; spec/objects/interfaces/FR-370-checked-package-temporal-clause-body.md; spec/objects/interfaces/FR-481-checked-package-hybrid-model-nodes.md; spec/functional/protocol/FR-430-apply-protocol-fairness.md; proposals/checked-package-v2/schema.json; proposals/checked-package-v2/temporal-fairness-target-vectors.json; tests/checked_package_v2.rs"
review_set: all
---

## Summary

`cargo test --locked` passes (148 checked-package tests). Clippy reports 16 existing errors in unchanged code. Repository-wide rustfmt fails on broad baseline drift; focused rustfmt also identifies unformatted lines in the new test block.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The newly added TC-303 test block is not rustfmt-formatted. | tests/checked_package_v2.rs |
| FND-002 | medium | The reader-case assertions do not compare expected locus with the exact target or branch-label path. | tests/checked_package_v2.rs, FR-370-AC-15 |
| FND-003 | medium | The new reader test omits the FR-370-AC-15 trace tag. | tests/checked_package_v2.rs, FR-370-AC-15 |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-004 | medium | FR-370-AC-15 now requires wrong-family refusals for local targets, but reader vectors exercise a wrong operator only for a parallel target, leaving protocol-root, attempt, and compensation wrong-kind behavior unverified. | FR-370-AC-15, TC-303 |


## Dispositions

| FND | outcome | reason |
| --- | --- | --- |
| FND-001 | fixed | Verified against rebased head `bb14012f`; after excerpt: `The reviewed tc_233 test block is rustfmt-formatted; rustfmt output for the exact function is byte-identical to the checked-in block.` |
| FND-002 | fixed | Verified against rebased head `bb14012f`; after excerpt: `        assert_eq!(row["expected"]["locus"], expected_locus, "{id}");` |
| FND-003 | fixed | Verified against rebased head `bb14012f`; after excerpt: `#[trace("TC-303", "FR-322-AC-293", "FR-370-AC-14", "FR-370-AC-15")]` |
| FND-004 | still-open | The added wrong-family rule covers all local target variants, but the reader vectors only exercise a wrong-kind parallel. Add schema-valid wrong-family cases for protocol root, attempt and compensation targets with exact refusal code/cause/locus assertions. |
