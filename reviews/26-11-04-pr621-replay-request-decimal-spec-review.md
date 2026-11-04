---
id: SR-20261104200621
title: "Spec review of quire-specification PR #621 (replay request shape, Decimal mode, QSL-1599)"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; FR-323, FR-336, FR-140, FR-142, TC-185, TC-187, TC-194, TC-193, TC-277, FR-375, proposals/conformance-corpus-v1/schema.json, tests/qualification_vectors.rs"
review_set: subset
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-336
    type: reviews
  - target: ix://agent-ix/quire-specification/FR-140
    type: reviews
---

## Summary

Ticket QSL-1599, PR #621 head a2a25cda. FR-336 rows match $defs.ReplayRequest and DependencyEntry. FR-323 contradicts FR-336 on dependency refinement_internals and keeps request pointers that do not resolve in the schema's document. Decimal changes agree with qsl-cst grammar.rs (Decimal form requires the mode) but leave stale mode-free source examples. Counts 29 and 171 pass.

## Verdict

FAIL as written. Not mergeable until FND-001 and FND-002 are fixed in the PR.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | FR-323 and AC-17 give each dependency entry an optional refinement_internals; FR-336 and schema DependencyEntry are closed to {identity, sources}. Feature kept: add the member to the schema. | FR-323 lines 76-80; FR-323-AC-17; FR-336 DependencyEntryWire |
| FND-002 | high | Refusal loci /package/dependencies, /package/refinement_internals, /package/sources and /package/dependencies/n/... are unchanged though package is now the checked-package document and dependencies is a sibling; the pointers resolve to nothing. | FR-323 rules 1 and 3, AC-7, AC-17; FR-375 lines 183-184; TC-277 BP-08, BP-15 |
| FND-003 | medium | FR-140 calls Decimal[lo, hi; smin, smax; exact] the mode-free spelling; it names the mode. | FR-140 type paragraph; FR-140-AC-9 |
| FND-004 | medium | Mode-free Decimal source examples remain: TC-193 F20 (expects ill_typed, now a syntax refusal), FR-142-AC-28 and AC-29 (U-rows already changed), TC-187 U44 expected cell says naming no mode. | TC-193 F20; FR-142-AC-28, AC-29; TC-187 U44 |
| FND-005 | low | A non-replay request package reference has no definition or schema; FR-323 says only exact reference. | FR-323 package member |

## Coverage

Examined: FR-323 package, dependencies, AC-7, AC-17; FR-336 package, dependencies, DependencyEntryWire; FR-140 and AC-8, AC-9; FR-142 carrier text; TC-185 D10 to D29; TC-187 U41 to U44; TC-194; qualification_vectors counts; QSL replay request.rs and execute.rs; grammar.rs Decimal form.
