---
id: SR-514
title: "Scope-boundary review of the scope progress and closure vocabulary"
type: SpecReview
analysis: scope-boundary
scope: "Branch objects/closure-axes-vocabulary at 7a050c9: what the change deliberately did not touch"
review_set: subset
review_date: "2026-09-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-210, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-271, type: reviews }
---
## Summary

The handoff named three items and forbade the rest. The change addresses exactly
those three and leaves every neighbouring gap where it was recorded.

## The three items

| Item | What the change does | Shape |
| --- | --- | --- |
| 1. Four unenumerated axes | FR-061 declares `open`/`closed` on all four; FR-284 types it; FR-240 and FR-258 drop the gap items. | A declaration, as asked. |
| 2. Eight typed-but-unlabelled entities | FR-210 declares all eight non-members with a citation each. | A decision; no label minted. |
| 3. FR-271's convention scope | FR-271 declares its families producer-minted and a consumer's own codes a separate namespace. | A scope declaration; no convention imposed. |

Agent B asked for two specific decisions and both are made in the specification,
not in a message: the axes **share one vocabulary** (question 1), and **absence
is not a member** (question 2).

## Deliberately untouched

| Neighbouring gap | Left where it is |
| --- | --- |
| Input completeness has no member list | FR-258 Open gap, sharpened; SR-512 FND-004. |
| Participation is unenumerated at the source | FR-240 Open gap item 3, unchanged. |
| Result-identity derivation and identity-versus-bytes | FR-240 Open gap items 1 and 2, unchanged. |
| Settlement basis: FR-091 names four members, FR-061 and FR-094 name five | FR-258 Open gap item 1, unchanged. |
| Two code-spelling conventions in the producer's own namespace | FR-271 Open gap, unchanged; the new scope section says explicitly that resolving it would not settle a consumer's convention. |
| Seven catalog codes retained by meaning with no spelling | FR-271 Open gap, unchanged. |
| Four divergent protocol subject lists; receipt versus delivery | FR-210 Open gap, unchanged. |
| Identities for delivery, attempt, effect, commit boundary, recovery relation | `quire-specification#28`, not re-filed. |
| Producer stages, recomputed-digest refusal code, settlement basis | `quire-specification#29`, not re-filed. |
| Fourteen further identity/membership/ordering gaps | `quire-specification#31`, not re-filed. |

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | Item 2's answer is a decision about membership, and a decision can be wrong in a way a gap cannot. | FR-210, FR-050 |
| FND-002 | low | Item 3 declares a scope boundary without imposing a convention, so three spellings still coexist in B's repository. | FR-271 |
| FND-003 | low | No consumer repository was touched, and none was inspected. | — |

### FND-001 — a decision, not a gap

Agent B offered the alternative itself: "If the answer is that four of them are
not subjects, say so and B drops them rather than relabelling." The answer is
that none of the eight is a subject, and the basis is that FR-051, FR-049-AC-2,
FR-111 and FR-035 are the four lists that state subject membership and none names
any of them. FR-050 names components, endpoints, channels and participants only
as identities a role or role instance must not be substituted by, and "static
definitions" as what an instance is held distinct from.

If a later requirement names one of the eight as a subject, FR-210's non-member
table is wrong and must be corrected — which is a visible, citable claim rather
than a silence. That is the intended trade.

### FND-002 — a boundary, not a convention

FR-271 now states that its families are producer-minted and that a consumer's
refusal codes are a different namespace that never resolves to a member here, and
that resolution is by selected catalog and exact spelling together. It does not
state which convention a consumer should use, because no requirement establishes
one and inventing a house convention would bind every consumer repository from an
object artifact. B's question — "is a consumer free to choose, provided it
collides with no published family" — is answered yes on the namespace question
and left open on the style question, with the collision hazard stated.

### FND-003 — consumers stated, not inspected

`src/closure.rs` and `src/ids.rs` are described from agent B's handoff. Nothing
in either was read or changed, and no consumer repository was touched. The
required consumer changes are recorded in SR-513 FND-002.
