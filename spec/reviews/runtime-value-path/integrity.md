---
id: SR-506
title: "Integrity review of the runtime value path model"
type: SpecReview
analysis: integrity
scope: "Internal consistency of FR-047, FR-207 and FR-274 at a9368c1: grounding of every normative clause, agreement between the amendment and the extraction, and registration completeness"
review_set: subset
review_date: "2026-09-11"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-047, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-207, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-274, type: reviews }
---
## Summary

Every normative clause the amendment adds was traced to an accepted requirement
or is declared as an authored convention of FR-047 itself. Two defects were
found during authoring and corrected before the commit; both are recorded below
because each would have been a silent soundness hole.

## Grounding of each clause

| Clause | Grounded in | Status |
| --- | --- | --- |
| Observation is part of the root | FR-042 "Presence facts never cross observation identity"; FR-045-AC-3 | Accepted requirement |
| Declared object identity as a subject | FR-009 universe/type/object identity; FR-204 | Accepted requirement |
| Invocation parameter/result as a subject | FR-046 "Parameters come from one declared invocation", "result is post-only" | Accepted requirement |
| Field selection retains exact spelling | FR-037-AC-6 "retain their exact spelling even when the member is a keyword" | Accepted requirement |
| Option-value traversal is its own kind | FR-005 unwrap requires a presence proof; FR-045 "the same immutable optional value/field/observation" | Accepted requirement |
| Member selection is distinct from a field | FR-019-AC-2 member presence/multiplicity; FR-037-AC-6 | Accepted requirement |
| Rendering never participates | FR-047-AC-1; FR-047 "formatting/localization cannot change classification" | Accepted requirement |
| Empty path is not an absent path | FR-047-AC-4; proposed catalog "an empty path ... cannot masquerade as a located failure" | Accepted requirement, catalog concurring |
| Order carries no causal meaning | FR-052 "without deriving causal edges from source order"; FR-113 | Accepted requirement |
| Subject-kind order; step-kind order | Neither is derivable | **Authored convention, declared as such in FR-047** |
| Serialization out of scope | Declared by FR-047; FR-047 "creates no competing result envelope" | Authored, consistent |

The two authored conventions are the only content in the change that no prior
requirement establishes. They are deliberate: a total order needs a tie-break
between kinds and none can be derived. They live in FR-047, which is entitled to
establish them, and FR-207 cites them rather than asserting them — "The
subject-kind and step-kind orders are fixed by FR-047 and are not asserted
here." This is exactly the split FR-200 requires.

## Findings

Findings FND-001 and FND-002 were detected and corrected before the commit; each
would otherwise have been a silent soundness hole. FND-003 and FND-004 are open.

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | high | The first draft rooted a path at "the named State root of one selected observation". `State root` appears in **no accepted requirement**; it is wording from `proposals/quire-v1/definitions/native-diagnostics.md`, which FR-274 itself labels "proposed, not adopted". The amendment would have imported a non-adopted term into a normative requirement. Corrected before commit: the root is redefined as (selected observation identity, subject) with the subject grounded in FR-009/FR-204 and FR-046, and the catalog wording is not used. | FR-047 Behavior; FR-274 "proposed, not adopted"; proposals/quire-v1/definitions/native-diagnostics.md |
| FND-002 | high | The first draft declared the key order "a total order ... by root key first". It is not total. The root contains a selected observation identity, and FR-265 records that "the prose does not fix the key components" of a selected snapshot and that no key order over snapshots is stated. Two paths under different observations would have had no defined order while the artifact claimed one. Corrected before commit: FR-047 now states the order is total over paths sharing one observation and extends across observations only where the selected observation identity itself supplies a key order, FR-207 records the residual gap, and FR-047-AC-7 and FR-207-AC-6 test the restriction. | FR-047 Behavior; FR-207 "Key order"; FR-265 "Key", "Key order" |
| FND-003 | medium | FR-204 declares a key order lexicographic over (`universe`, `type`, `object_identity`) while its own "Cross-universe comparison" section states that comparison "is defined only within one declared universe" and that coincident labels from different universes "do not compare equal and do not compare unequal-but-comparable". FR-207's root order inherits that tension: it orders subjects across universes using an order FR-204 elsewhere says is not available across universes. This predates the change and is not resolved by it. | FR-204 "Key order"; FR-204 "Cross-universe comparison"; FR-009-AC-3 |
| FND-004 | low | FR-274 keys on the three location-kind labels while its own open gap states the catalog gives them "no discriminator tags or identifier spellings", and FR-047 now explicitly fixes none either. This is SR-502 FND-006 restated for this artifact; the change narrows FR-274's structural gap and leaves its spelling gap exactly where it was. | FR-274 "Key"; FR-274 "Open gap"; SR-502 FND-006 |

## Registration completeness

`FR-207` is registered in FR-200's `contains` list, in `spec/spec.md`'s master
`contains` list and Requirements table, and its two behavioral criteria
(FR-047-AC-6, FR-047-AC-7) are registered in `spec/composed-foundation/tests.md`
both as criterion rows and in the TC-047 criteria list. Every cross-repository
reference in the change uses `ix://agent-ix/quire-specification/FR-NNN`, which
is path-independent. Validation reports 349/349 documents grammar-clean, zero
findings, zero edge warnings.
