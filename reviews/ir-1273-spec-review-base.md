---
id: SR-605001
title: 'Spec review of PR #605: checked-package V2 SourceRef and source-byte integrity (IR-1273, QSL-1501)'
type: SpecReview
analysis: base
scope: 'agent-ix/quire-specification@5f483b1; git diff origin/main(aaf8c65)...5f483b1 (PR #605): proposals/checked-package-v2/{schema.json,fixtures,source-documents*,source-integrity-vectors.json}, FR-322 "Source-byte integrity", FR-322-AC-319..327, FR-272 and native-diagnostics invalid_package row, FR-272-AC-26, TC-233; IR PR #837; QSL FR-518 row 28m; tickets IR-1273, QSL-1501'
review_set: subset
date: '2026-11-02'
relationships:
- target: ix://agent-ix/quire-specification/FR-322
  type: reviews
- target: ix://agent-ix/quire-specification/FR-272
  type: reviews
- target: ix://agent-ix/quire-specification/TC-233
  type: reviews
---

## Summary

Ticket: IR-1273, QSL-1501. PR: quire-specification#605, head 5f483b1. Mergeable after FND-001 and FND-002 are decided; neither blocks IR from starting. Independent checks: all seven source-documents digests recomputed with SHA-256 over the file bytes match (CRLF/LF/empty included); a walk of every published JSON file finds no two-member source reference left outside conformance-corpus-v1; no ID collision (AC-319..327 and FR-272-AC-26 appear only in #605 among open PRs); cargo test passes 281 across 18 targets (checked_package_v2 166); quire validate on changed spec files gives the same errors as main (missing id on FR-272/FR-322, record-invalid). Mutation: removing the refinement_step_rows site from the test decider turns tc_233_source_integrity_vectors_refuse_at_the_reference_pointer red at SI-R10. IR #837 omits that fourth site (IR reader CheckedSourceRegion in refinement_step_rows is validated at /refinement_step_rows/{i}/region/source), so IR #837 must change. Field name, SHA-256 and preimage, caller-supplied bytes keyed by pair, pointer-at-reference, and order lock/source_map/diagnostics match. QSL FR-518 row 28m (invalid_package/source-integrity at the CheckedSourceRef pointer) is served.

## Scope

| Unit | Path | Role |
| --- | --- | --- |
| Source-byte integrity section | spec/objects/interfaces/FR-322-checked-package-artifact.md:3191 | examined |
| FR-322-AC-319..327 | spec/objects/interfaces/FR-322-checked-package-artifact.md:4201 | examined |
| FR-322 lock bullet | spec/objects/interfaces/FR-322-checked-package-artifact.md:127 | examined |
| FR-322-AC-52 | spec/objects/interfaces/FR-322-checked-package-artifact.md:4016 | context_only |
| FR-272 invalid_package row and AC-26 | spec/objects/foundation/FR-272-typed-diagnostic-cause.md:73 | examined |
| native-diagnostics invalid_package row | proposals/quire-v1/definitions/native-diagnostics.md | examined |
| schema SourceRef, Diagnostic pairing | proposals/checked-package-v2/schema.json | examined |
| conformance-corpus-v1 SourceRef | proposals/conformance-corpus-v1/schema.json:1733 | examined |
| source-integrity-vectors SI-P1..S2 | proposals/checked-package-v2/source-integrity-vectors.json | examined |
| TC-233 relationships | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-322 phase-order sentence says integrity runs after closed-schema checks, but SI-R1..R5 and SI-R13 are schema-invalid and still refuse source-integrity | FR-322:3228 |
| FND-002 | medium | conformance-corpus-v1 SourceRef left two-member with no statement of scope; IR #837 names it as a gate | proposals/conformance-corpus-v1/schema.json:1733 |
| FND-003 | low | CRLF source documents have no .gitattributes; an autocrlf checkout changes bytes and breaks every digest | proposals/checked-package-v2/source-documents/ |
| FND-004 | low | FR-322-AC-319, AC-322 and AC-323 each carry more than one claim; AC-324 says the reader supplies no filesystem bytes, which no vector can fail | FR-322:4201 |

## Verdict

Mergeable once FND-001 and FND-002 are resolved (wording and a one-line scope ruling). The digests, pointers, order, schema closure and refusal vectors are correct and fail when mutated. IR #837 must change: add the refinement_step_rows[*].region.source site (between source-map and diagnostics) and align its within-reference order or drop it as unobservable.
