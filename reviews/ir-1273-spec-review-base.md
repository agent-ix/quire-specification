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

## Dispositions

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-001 | fixed | Head bfe3d2a: FR-322 now states the verification runs after strict JSON and canonical-byte checks and before every other check, closed-schema included; SI-R16 and FR-322-AC-341 pin it; mutating SI-R16's expected locus turns the vectors test red. |
| FND-002 | fixed | Head bfe3d2a: conformance-corpus-v1 SourceRef requires source_integrity (DependencyEntry.sources), ObjectEnvironmentRef.identity moved to closed two-member ObjectEnvironmentIdentity (judgement accepted: it names an environment, not a source), FR-336-AC-9/10, FR-323 sources check and AC-29..33, RP vectors. No two-member source reference remains in any fixture. |
| FND-003 | fixed | Head bfe3d2a: .gitattributes marks proposals/checked-package-v2/source-documents/** -text; digests recomputed and equal. |
| FND-004 | fixed | Head bfe3d2a: ACs split into single-claim FR-322-AC-319..341; AC-332 and SI-R7 make no-filesystem-lookup failable (file present beside fixtures, bytes unsupplied, still refused). |

## New findings (disposition pass 1)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-005 | medium | FR-323 replay sources check order is unspecified between entries and between failure kinds; the test decider checks every claim's syntax first, then missing-bytes and mismatch interleaved per entry, and no vector pins it | spec/objects/interfaces/FR-323-native-runtime-envelope.md:39-49 |
| FND-006 | low | TC-969 (relationships, Scope, trace line) does not list FR-336-AC-9/10 although the new test is tagged to it; FR-323-AC-29..33 are verified by TC-233 not TC-277 (the FR-323 replay TC) | spec/test-cases/TC-969-conformance-corpus-v1-schema-admits-the-fixture-and-refuses-out-of-domain-shapes.md:17,36 |

## Dispositions (round 2)

| FND | Outcome | Reason |
| --- | --- | --- |
| FND-005 | fixed | Head 24a6bb6: FR-323 states one total order (claim syntax of all entries first, then per entry bytes then digest, first failing entry decides); FR-323-AC-34/35 and vectors RP-R5/R6/R7 pin it. Mutations: dropping the syntax-first pass turns RP-R7 red; separate all-entries absent then digest passes turn RP-R5 red. |
| FND-006 | fixed | Head 24a6bb6: TC-969 Scope and relationships list FR-336-AC-9/10 with a tagged test; FR-323-AC-29..35 are verified by TC-277 (renamed test tc_277_replay_sources_are_checked_against_byte_provision, rows BP-20..BP-23) and removed from TC-233. |

## New findings (disposition pass 2)

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-007 | low | TC-277 rows BP-20..BP-23 reuse row labels that TC-431 already uses for BP-20 and BP-21; labels are scoped per document so nothing fails, but a reader citing BP-20 is ambiguous | spec/test-cases/TC-277-replay-byte-provision.md:71-74 |

## Verdict (disposition pass 1)

FND-001..004 fixed. FND-005 and FND-006 are new and open; neither changes any shipped vector, both are spec wording or trace fixes. Mergeable once FND-005 and FND-006 are fixed or the owner defers them. Checks at bfe3d2a: cargo test all 18 targets 283 passed; FR-323 and FR-336 quire validate diagnostics identical to main; digests recomputed equal; no ID collisions with open PRs (AC-319..341, FR-323-AC-29..33, FR-336-AC-9/10); mutations of SI-R16 and RP-R4 expectations turn the tests red.

## Verdict (disposition pass 2)

FND-005 and FND-006 fixed; one new low (FND-007) remains. Mergeable with lows. Checks at 24a6bb6: cargo test all 18 targets 283 passed; quire validate of FR-323, TC-277 and TC-969 identical to main; no ID collisions with open PRs (#600 adds TC-980 only; #602 nothing in range); both order mutations turn the replay test red.
