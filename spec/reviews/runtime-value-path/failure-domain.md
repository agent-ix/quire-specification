---
id: SR-507
title: "Failure-domain review of the runtime value path model"
type: SpecReview
analysis: failure-domain
scope: "Identity confusion, absence handling, order soundness and traversal edge cases for FR-047 and FR-207 at a9368c1"
review_set: subset
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-047, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-207, type: reviews }
---
## Summary

The model was attacked on the four failure modes that defeated the nouns in the
object layer: a key that cannot discriminate two distinct things, an absence
that is representable as a value, an order that is claimed but not established,
and a component that changes without changing identity. Three hold. The order
was the one that failed, and it now carries its restriction explicitly rather
than a false claim.

## Discrimination

The model discriminates the three cases the state evaluator must separate.

| Case | Distinguished by | Sound |
| --- | --- | --- |
| `o.field` missing vs `o.member` missing, same name | Step kind; FR-047 "never equal to one another even when they carry the same name" | Yes |
| An option that is absent vs its contained value that is absent | The option-value step is present in the second path and absent in the first; the step carries no name so it cannot collide with a field | Yes |
| The same nested value under pre and under post | Root observation differs, so roots are unequal; FR-042 keeps presence facts from crossing observation identity | Yes |
| The subject itself unavailable vs a nested value unavailable | Empty step sequence vs non-empty; FR-047 "an empty step sequence denotes the subject itself and is never an absent path" | Yes |
| Two objects with identical field values | Root subjects differ by declared object identity; FR-009 "Equal fields do not imply equal objects" | Yes |

This is the distinction agent A reported as blocking. It resolves.

## Absence

Absence has three representations in the model and each compares only to
absence: an absent path, an absent observation, and an absent subject. The
prohibited masquerades are named explicitly — an empty step sequence, byte zero,
and a name-search match are each stated not to be an absent path. FR-047's
"An absent observation or an absent subject yields no path at all" closes the
remaining hole, which is a rooted path carrying an absent component; that shape
does not exist.

FR-046-AC-4 requires that a missing input "remains unavailable/incomplete,
without fabricating a delta, violation or Boolean". A path reports where a value
was sought, not that it was found; FR-207 states this directly. Nothing in the
model lets a located path stand as evidence that the located value existed.

## Mutability

No component of the key is mutable. The observation identity is a selection, the
subject is a declared identity or a declared binding, and the step names are
declared spellings retained exactly by FR-037. A runtime *value* changes without
changing the path that names it — which is correct: the path is a locus, not a
reading. FR-207 carries this as an explicit sentence rather than leaving it to
inference.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | The comparison is **not a total order over all runtime value paths**, and the specification now says so. It is total over any set of paths sharing one selected observation identity. Across observations it resolves only if the observation identity supplies a key order, and FR-265 records that a selected snapshot's key components are not fixed, so none is available today. A consumer that sorts a mixed-observation collection of paths has no defined result. | FR-047 Behavior; FR-207 "Key order"; FR-265 "Key", "Key order" |
| FND-002 | low | The practical exposure of FND-001 is narrow. A single diagnostic reports loci under one selected observation, so the ordering a producer actually performs falls inside the total region. The exposure is a cross-observation report that sorts paths globally, for which no requirement states a need. | FR-047 Outputs; FR-046 Inputs |
| FND-003 | medium | Closing FND-001 requires fixing the selected observation's key, which belongs to workstream F (FR-110 to FR-116, FR-260 to FR-269), not to A. It cannot be closed inside FR-047 without A declaring a key over F's noun. Stated, not made. Deriving the order from a capture timestamp is independently forbidden: FR-113 states that "Arbitrary timestamp order SHALL NOT establish causality", and FR-265 records that ordering snapshots by capture time would supply an order the prose does not state. | FR-265; FR-113; docs/functional-capability-ledger.md |
| FND-004 | medium | Two producers may emit paths that are structurally equal but serialize differently, because FR-047 fixes no wire spelling, discriminator tag or separator. Comparison across producers is therefore defined at the model level and undefined at the byte level. This is the same unresolved axis as SR-502 FND-006, and it is disclosed in both FR-047 and FR-207 rather than closed. | FR-047 Behavior; FR-207 "Serialization is out of scope"; FR-207 "Open gap" |
| FND-005 | low | The JSON field/index path remains structurally undefined, so a producer that must report an `invalid_package` or `invalid_runtime_input` locus still has no model for it. The change deliberately does not extend to it, because the proposed catalog is its only source and that catalog is not adopted. | FR-274 "Open gap"; proposed catalog `invalid_package` and `invalid_runtime_input` rows |
