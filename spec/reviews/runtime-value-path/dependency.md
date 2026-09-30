---
id: SR-508
title: "Dependency review of the runtime value path model"
type: SpecReview
analysis: dependency
scope: "Edge direction, edge legality, cross-workstream dependence and identifier allocation for the change at a9368c1"
review_set: subset
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-207, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-274, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-200, type: reviews }
---
## Summary

The change adds eight edges and changes none. Every edge validates, the
`specifies` back-edge that has no declared inverse is authored explicitly, and
no existing identifier moved. One structural dependence crosses a workstream
boundary for the first time in the object layer, and it is load-bearing.

## Edges added

| From | Verb | To | Legality |
| --- | --- | --- | --- |
| FR-207 (value_object) | `specifies` | FR-047 (behavioral FR) | Registered in `spec-artifacts-iso` as "Object FR specifies a behavioral FR" |
| FR-207 (value_object) | `composes` | FR-204 (value_object) | `value_object.allowed_links.composes: [value_object, enumeration]` |
| FR-207 (value_object) | `composes` | FR-265 (value_object) | Same rule |
| FR-207 (value_object) | `references` | FR-274 (enumeration) | `value_object.allowed_links.references: [enumeration]` |
| FR-047 (behavioral FR) | `references` | FR-207 | Generic back-edge, per the object layer's D2 convention |
| FR-274 (enumeration) | `references` | FR-207 (value_object) | See DEP-001 |
| FR-200 (domain) | `contains` | FR-207 | `domain.allowed_links.contains` admits every object type |
| spec.md | `contains` | FR-207 | Master registration |

`specifies` declares no inverse in `spec-artifacts-iso`, so the FR-047 back-edge
is authored explicitly rather than derived. It is present.

Validation emits no `disallowed-edge-type` warning for any edge in the change.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | `enumeration.allowed_links.references` lists only `enumeration`, so FR-274 referencing the value_object FR-207 is outside the declared vocabulary, as is its pre-existing reference to FR-203. Neither produces a warning in this environment, and the edge is semantically correct: an enumeration of location kinds must point at the artifacts that type its members. This is the object vocabulary being too narrow, already filed as agent-ix/quoin#369, not a defect in the change. | FR-274 frontmatter; `spec-objects-business@0.2.0` manifest; quoin#369 |
| FND-002 | medium | FR-207 (workstream A) composes FR-265 (workstream F) and its order completeness depends on a key that F has not fixed. This is the first A-to-F structural dependence in the object layer, and it is load-bearing rather than decorative: FD-001 is unresolvable inside A. F should be told that fixing the selected observation's key would complete FR-207's order as a side effect. **Stated, not made** — no F artifact was modified. | FR-207 "Key order", "Open gap"; FR-265; SR-507 FD-003 |
| FND-003 | low | FR-207 cites FR-005, FR-009, FR-037, FR-042, FR-045 and FR-046 in prose and dependency links but declares `specifies` edges to none of them, only to FR-047. This is correct after the amendment — FR-047 now restates the model normatively, so FR-047 is the single owning requirement — but it means the six grounding requirements gain no back-edge and a reader traversing edges from FR-009 will not find FR-207. Adding six more `specifies` edges would instead imply FR-207 extracts each of them, which it does not. Recorded as an accepted trade-off. | FR-207 frontmatter; FR-207 "Dependencies" |

## Identifier allocation

FR-207 was free. Verified: no reference to `FR-207` existed anywhere in `spec/`
before this change, and the foundation object block FR-200..FR-206 was
contiguous with headroom to FR-209. SR-505..SR-509 were free; the prior
allocation ended at SR-504. No existing identifier was changed, and the single
`id:` line in the diff is the added `id: FR-207`.

## Consumer impact

None required, and none made. The change alters no existing requirement's
meaning and removes no statement, so a downstream `ix://` edge that resolved
before still resolves to the same requirement with the same content plus two
additional acceptance criteria. A consumer that wishes to rely on the runtime
value path model may now cite FR-047 or FR-207 directly; nothing obliges it to.
`quire-protocol`, `quire-observation`, `quire-verification` and
`quire-spec-language` are untouched.
