---
id: SR-202611071706002
title: "base review of QSL-1706 protocol-location vectors PL-04/PL-05 (FR-322-AC-388, AC-292, TC-233)"
type: SpecReview
analysis: base
scope: "agent-ix/qspec; branch spec/qsl-1706-vectors diff origin/main...HEAD; checked-term-data-vectors.json protocol_locations and read_work; FR-322-AC-292, FR-322-AC-388; TC-233; tests/checked_package_v2.rs tc_233_checked_term_data_vectors_cover_the_shared_carrier"
review_set: base
---

## Summary

The new positive rows are correct. The array is in ascending application-path order (`protocol-case-guard-term` < `protocol-declaration-term` < `protocol-related-end-term` < `shared-protocol-node`). Within each new group `term_ordinal` runs 0, 1 in origin UTF-8 order (`case no guard` < `case yes guard`; `related 0 from` < `related 1 from`). The work arithmetic holds: 12 equality + 7 protocol-location + 2 dispatch + 2 probabilistic = 23 rows; 23 + 3 units + 2 enums + 2 candidates + 0 term_path segments = 30, one-less 29. The final charged row is still `probabilistic_claims/1`. AC-292 and TC-233 prose match. Two low findings.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | The new loop over `run` rows panics on any positive `run` origin that is not `case <label> guard` or `related <k> from/to`. That includes a legitimate `run <path> body` or `run <path> decision <start>..<end>` row, which FR-322-AC-388 allows (`only a visible entry is named decision`). FR-322-AC-388 itself says only "no positive `run` row's origin name holds a span". Fix: limit the shape match to rows whose slot words contain `case` or `related`, and keep the `..` check for those rows only; or reword AC-388 to say that positive `run` rows are exactly case-guard and related-end rows. | tests/checked_package_v2.rs:21040-21056; FR-322-AC-388 (FR-322:4670) |
| FND-002 | low | The `shared_terms` check (`windows(2).find`) now matches PL-04b/PL-04 before PL-01/PL-02. The assertion still holds, but the shared-occurrence check no longer exercises PL-01/PL-02, which is the AC-291 root/child pair with one origin and different paths. Fix: assert the PL-01-root/PL-02-child pair by id, as the new FR-322-AC-388 loop does for its pairs, instead of taking the first adjacent match. | tests/checked_package_v2.rs:20979-20990; FR-322-AC-291 |

## Scope and evidence

No applicable AssuranceProfile was found. This is a direct base review of the diff only. Counts were measured with `jq` over `positive` in checked-term-data-vectors.json (equality_decisions 12, protocol_locations 7, dispatch_tables 2, probabilistic_claims 2, units 3, enum_members 2, candidates 2). The test was not run (no target dir in the worktree; spec-only review). Application nodes in these vectors are symbolic, and no package graph defines `protocol-case-guard-term` or `protocol-related-end-term`. That matches the existing PL-03 row. PL-05/PL-05b model two `related` ends at one application occurrence. QSL's compiler records one occurrence per position, so its compiled form uses separate occurrences, but the shared-occurrence form is wire-legal and is what AC-291 term-ordinal coverage needs.
