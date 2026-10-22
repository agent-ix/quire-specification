---
id: SR-510
title: "Base review of the scope progress and closure vocabulary"
type: SpecReview
analysis: base
scope: "Branch objects/closure-axes-vocabulary at 7a050c9: the FR-061 amendment, the new FR-284 object artifact, the FR-240 and FR-258 gap closures, the FR-210 non-member declaration, the FR-271 scope declaration, and the FR-200, spec/spec.md and acceptance-matrix registrations"
review_set: subset
review_date: "2026-09-12"
relationships:
  - { target: ix://agent-ix/quire-specification/FR-061, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-284, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-240, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-258, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-210, type: reviews }
  - { target: ix://agent-ix/quire-specification/FR-271, type: reviews }
---
## Summary

The change answers three questions agent B put to this repository and closes the
one that blocks it. Four of five progress/closure axes had no published member
list, so B's crate invented twenty labels; `FR-028-AC-1` could not execute and
the five-axis product state could not be formed at all.

Like the runtime value path before it, this change is **not** purely structural.
FR-200 forbids an object artifact from supplying a vocabulary no behavioral
requirement establishes, and no requirement enumerated these axes. So the
declaration is made where declarations belong — in the owning behavioral
requirement, FR-061 — and FR-284 extracts it. FR-240 and FR-258, which had both
recorded the absence honestly, now cite FR-284 and drop the corresponding gap
items.

The two label strings are not invented. `open` and `closed` are used for the
decision scope and the surrounding execution by FR-059, FR-061, FR-091, FR-094
and FR-115, and FR-061 uses `open` on a progress axis directly ("decision-scope
progress is open"). What FR-061 now adds is the *closure* of the set, the
per-axis meaning, the independence rules and the absence rule — the parts a
consumer cannot infer.

Two questions did not need a new vocabulary and did not get one. FR-210 declares
eight nouns non-members with the citation for each; FR-271 declares its own
scope as producer-minted codes. Both are decisions B can implement against,
recorded in the specification rather than in a message.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | medium | FR-061 is amended by A while B owns the protocol family; the declaration is placed on the requirement that states the four-axis decomposition, but the placement is a cost. | FR-061 |
| FND-002 | low | `open` on a progress axis reads awkwardly in English; the alternative was minting `advancing`/`complete-through-boundary`, which no requirement spells. | FR-061, FR-284 |
| FND-003 | low | Input completeness remains unenumerated, so one axis of FR-094-AC-3's four is still not a closed set. | FR-258, FR-094 |

### FND-001 — the amendment sits on B's requirement

FR-061 belongs to the protocol family that agent B owns. A is amending it
because B asked A to publish the vocabulary and B declared it would not supply
one ("B is **not** supplying them"). Four candidate owners were weighed:

- **FR-094** states the axes' independence and the progress-coverage condition,
  but owns the *temporal interpretation* of assertions, not the result contract.
  Its own dependency note says so: "Agent B owns the shared result-dimension wire
  contract."
- **FR-115** and **FR-116** restate the axes at the observation and consumer
  boundaries; both defer to FR-061 explicitly.
- **FR-059** owns the fifth axis, not the four.
- **FR-061** states the decomposition verbatim — "Decision-scope progress and
  closure and surrounding-execution progress and closure are four independent
  facts" — and already carries the fifth axis's member list in the same
  paragraph. Putting the four axes' member list anywhere else would split one
  sentence's subject across two requirements.

FR-061 is therefore the smallest owner, and the amendment is additive: 25
inserted lines, zero deleted. The cost is that a future B edit to FR-061's
Behavior section must merge against it.

### FND-002 — the label choice

`decision-scope progress is closed` is not natural English. The alternative
considered was a distinct progress vocabulary — `advancing` and
`complete-through-boundary`, or B's deleted `not-started`/`advanced`/`stalled`.
Every such spelling is absent from the accepted prose, and FR-200 plus B's own
constraint ("B can mark a criterion unexecutable against a stated gap, but cannot
detect a guess") weigh against minting one. FR-061's own "decision-scope
progress is open" settles which label set the prose actually uses. The awkward
reading is accepted in exchange for zero invented spellings.

### FND-003 — the remaining axis

FR-094-AC-3 names four independently-represented axes: decision scope, surrounding
execution, assessment execution and input completeness. The first two are now
enumerated here, assessment execution is enumerated by FR-241, and input
completeness is not. FR-094 names it only as "complete/incomplete input" while
requiring that a file "labelled closed without complete required observations" be
classified as an incomplete input. Whether that is a two-member vocabulary or a
record carrying missing-dependency identities is not stated. FR-258 retains that
gap explicitly rather than extending FR-284 over it, because the two axes are held
distinct by the same acceptance criterion that names them.

## Checklist

| Gate | Result |
| --- | --- |
| ID formats (FR/AC/SR) | FR-284, FR-284-AC-1..AC-8, FR-061-AC-11..AC-13, SR-510..SR-514 all conform. |
| Duplicate or gap in ids | FR-284 is the first free id above FR-283 across all 17 local and 20 remote refs. No existing id changed. |
| Link integrity | `quire validate` reports zero `[missing]` and zero edge warnings. |
| EARS grammar | 355/355 documents grammar-clean, 0 grammar findings. |
| AC verification vocabulary | FR-284 uses `Inspection`/`Analysis` only and mints no TC. FR-061-AC-11..13 bind to the existing TC-061. |
| Coverage | The three new FR-061 criteria are registered in `spec/protocol/tests.md` against TC-061, both in the criterion rows and in TC-061's criteria list. |
