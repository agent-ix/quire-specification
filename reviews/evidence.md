---
id: SR-2026103009
title: "evidence review of STD-617 nested equality identity"
type: SpecReview
analysis: evidence
scope: "agent-ix/quire-specification; FR-322-AC-288, FR-322-AC-289, FR-322-AC-292; TC-233 and checked_package_v2.rs"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: reviews }
---

## Summary

Ticket: STD-617. PR: agent-ix/quire-specification#558. The equality key shape and term_path rules are explicit, and nested and/or positive cases and duplicate-key checks are present. The EQ-R6 negative vector does not provide a distinct graph context: it uses the same application identity, occurrence and [0] path as positive EQ-08, where that path is an equality. The test only checks the row against the JSON schema and its path value, not that traversal ends at a non-equality node. This leaves the reader-refusal evidence internally contradictory. #557’s target-union change is separate and compatible; STD-614 protocol term_ordinal remains distinct.

## Verdict

Review found one medium evidence/semantic defect. `EQ-R6-path-to-non-equality` is keyed to the same `and-equality-root` application and `[0]` path as positive `EQ-08-nested-and-first`. The authored positive case identifies that path as equality, so the negative case cannot prove a path-to-non-equality refusal without separate graph identity/context. The Rust test performs schema checks only for this refusal vector.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The non-equality path refusal reuses the same application, occurrence, and path as a positive nested equality, so the vector supplies contradictory graph meaning and cannot establish the claimed reader refusal. | proposals/checked-package-v2/checked-term-data-vectors.json:2022; tests/checked_package_v2.rs:19172; spec/objects/interfaces/FR-322-checked-package-artifact.md:3586 |

## Scope

| Unit | Path | Role | Excerpt |
| --- | --- | --- | --- |
| FR-322-AC-288 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined | `Equality records ... preserve the exact root application/occurrence, term_path ... Positive equality vectors include nested equalities under and roots ...` |
| FR-322-AC-289 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined | `The reader joins each equality row to one local root expression/application node ... follows term_path by zero-based application-argument indexes ... path ending at a non-equality application ... refuses ...` |
| FR-322-AC-292 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined | `Schema-valid reader mutations cover ... a path ending at a non-equality application ... Reader-level mutations have the expected refusal code, cause and exact JSON pointer.` |
| TC-233 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined | `A path to a non-equality application refuses ... duplicate (...) rows refuse ...` |

## Re-review at a2a3d2f977594396491805eb4e1c93becfdafabe

The new path forms have positive schema-vector coverage. FND-001 remains unresolved: EQ-R6's application/occurrence/path matches positive EQ-08, and `tc_233_checked_term_data_vectors_cover_the_shared_carrier` still asserts only schema validity, operator, and path for the purported refusal.
