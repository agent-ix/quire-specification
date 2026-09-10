---
id: SR-303
title: "Dependency review of native temporal lifecycle correspondence"
type: SpecReview
analysis: dependency
scope: "FR-090 through FR-095 dependencies across A, B, D, E, F, contract IR and TL"
review_set: all
evaluated_revision: "2f347b8"
review_date: "2026-09-10"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-095, type: reviews }
---
## Summary

The dependency graph remains one-way: A admits native source/profile
definitions; D and F provide typed model/observation inputs; E defines temporal
meaning; B owns the shared result relation; contract IR projects total Boolean
predicates; and exact TL profiles evaluate only the admitted bridge subset.

## Findings

| ID | Severity | Summary | Refs | Escape Cause |
| --- | --- | --- | --- | --- |
| FND-001 | high | Resolved: E's temporal handoff now supplies the exact lifecycle and support facts B's shared result consumes without copying B's result schema. | FR-091; FR-094; FR-061 | missing-requirement |
| FND-002 | medium | Resolved: TL truth remains downstream of native admission and total-Boolean projection; equal formula bytes never select a native profile or result lifecycle. | FR-095; IT-040/041 | wrong-requirement |
| FND-003 | high | Open F dependency: the observation consumer must preserve B/E dimensions before the composed assessment/output integration can pass. | FR-115/116; TC-145/146; IT-051 | missing-requirement |
| FND-004 | medium | Open A dependency: successor exact temporal definition/profile revisions and digests follow accepted E and F semantics; the earlier candidate stays historical. | definitions/README.md; checksums.sha256 | correct-requirement-no-evidence |
| FND-005 | medium | Open implementation dependency: contract-IR #63/#64 and reviewed exact TL profile revisions precede bridge implementation or qualification. | FR-095 Dependencies; IT-040 Preconditions | correct-requirement-no-evidence |

## Dependency order

```text
A native grammar/profile admission + D model identity + F observation authority
  -> E temporal truth/progress/closure semantics
  -> B shared orthogonal result relation
  -> F result-consumer preservation
  -> contract-IR total-Boolean projection and bridge interface
  -> exact supported tl-syntax/tl-mltl profiles
  -> Rust bridge implementation and differential evidence
  -> A successor definition selection and composed adoption review
```

Specification drafting may precede implementation dependencies. Missing
producer or consumer contracts block the affected integration/implementation
claim, not truthful review of the E semantic slice.
