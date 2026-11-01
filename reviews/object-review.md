---
id: SR-2026103013
title: "object-review review of STD-617 nested equality identity"
type: SpecReview
analysis: base
scope: "agent-ix/quire-specification; FR-322 EqualityDecision key, term_path, ProtocolLocation term_ordinal, and checked-term-data vector objects"
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
| EqualityDecision and ProtocolLocation | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined | Equality key is application, occurrence, term_path; protocol location uses per-term ordinal. |
| EQ-R6 and EQ-08 | proposals/checked-package-v2/checked-term-data-vectors.json | examined | Both identify `and-equality-root`, occurrence expression 0, path `[0]`; positive EQ-08 is an equality. |

## Re-review at a2a3d2f977594396491805eb4e1c93becfdafabe

The equality identity object is now application + occurrence + typed term path, with argument/member/binding-value selectors; protocol identity remains separate through `term_ordinal`. FND-001 remains unresolved because EQ-R6's tuple is identical to the positive EQ-08 tuple while claiming a non-equality terminal.
