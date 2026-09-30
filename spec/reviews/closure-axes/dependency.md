---
id: SR-513
title: "Dependency review of the scope progress and closure vocabulary"
type: SpecReview
analysis: dependency
scope: "Branch objects/closure-axes-vocabulary at 7a050c9: the edges added, the artifacts that depend on the closed gap, and what each consumer must do"
review_set: subset
review_date: "2026-09-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-284, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-240, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-258, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-210, type: reviews }
---
## Summary

The change adds four frontmatter edges and one registration pair. Every edge is
admitted by the authoring module's `allowed_links`, which was checked rather than
assumed after one edge had to be withdrawn.

## Edges added

| From | Verb | To | Admitted because |
| --- | --- | --- | --- |
| FR-061 (behavioral) | references | FR-284 | Behavioral FRs are not object-typed; FR-061 already references FR-280, FR-281 and FR-282, all enumerations. |
| FR-284 (enumeration) | specifies | FR-061, FR-091, FR-094, FR-115 | The object → behavioral verb, matching FR-247's four `specifies` edges. |
| FR-284 (enumeration) | references | FR-247 | `enumeration.allowed_links.references` admits `enumeration`, and FR-247 is one. |
| FR-240 (entity) | references | FR-284 | `entity.allowed_links.references: [entity, aggregate_root, enumeration]`. |
| FR-258 (value_object) | references | FR-284 | `value_object.allowed_links.references: [enumeration]`. |
| FR-200 (domain) | contains | FR-284 | `domain.contains` admits every object type. |

`specifies` still declares no inverse in `spec-artifacts-iso`, so FR-061's
back-edge to FR-284 is authored explicitly rather than derived. That is the same
arrangement PR #26 established and quoin#369 tracks.

## Registration

`spec/spec.md` gains the `contains` edge and the `## Requirements` row;
`spec/protocol/tests.md` gains three criterion rows and extends TC-061's criteria
list. No new TC is minted: FR-284's criteria are `Inspection` and `Analysis`
only, and the three new FR-061 criteria bind to the existing TC-061.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | An `enumeration` cannot reference a `value_object` or `entity`, so FR-210's new non-member table links four object artifacts in prose only. | FR-210, quoin#369 |
| FND-002 | medium | Agent B must delete twenty labels and adopt two, and its five-axis product state becomes formable; that is a consumer change stated here, not made. | FR-028 |
| FND-003 | low | `origin/agent-b/v1-protocol-spec` is six commits ahead of main and touches the protocol family. | — |

### FND-001 — the edge that could not be authored

FR-210's `## Not members of this vocabulary` table names FR-215, FR-217, FR-233,
FR-237 and FR-238. Frontmatter `references` edges to all five were authored and
then withdrawn: `enumeration.allowed_links.references` admits `enumeration`
only, and those five are value objects and entities. The links survive as
relative markdown, so a reader follows them and the graph does not carry them.

This is the second of quoin#369's four vocabulary gaps observed in the wild —
the ticket already records that "an `entity` cannot `references` a
`value_object`" — and it is the reason the check was run against the manifest
rather than trusted.

### FND-002 — what agent B must do

Stated, not made; no consumer repository is touched by this change.

1. Delete the twenty invented labels in `src/closure.rs`, including
   `not-started`, `advanced`, `stalled`, `unobserved` and the `contradicted`
   carried across all four axes. FR-284's `## Non-members` names each.
2. Adopt `open` / `closed` on all four axes, with FR-284's per-axis meaning.
3. Keep the fifth axis on FR-247's five-member vocabulary and do not merge the
   two types (SR-512 FND-001).
4. Delete the eight subject tags in `src/ids.rs`: the eight that collide with
   FR-210 in favour of FR-210 by reference, and the eight in FR-210's new
   non-member table because none is a subject kind.
5. `FR-028-AC-1` becomes executable. The five-axis product state is 80
   combinations, each axis independently variable.

### FND-003 — branch coordination

`origin/agent-b/v1-protocol-spec` carries six unmerged commits in the protocol
family. Its FR-061 content is already on main, so a merge conflict with this
amendment is unlikely but not impossible; the amendment is additive and inserts
a paragraph pair into FR-061's Behavior section and three rows into its criteria
table.
