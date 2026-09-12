---
id: FR-243
title: "Settlement basis vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
---
# FR-243: Settlement basis vocabulary

## Description

When a truth result is retained, the result producer SHALL carry exactly one settlement basis from this closed vocabulary plus the exact supporting fact and progress identities.

## Values

| Value | Description |
| --- | --- |
| `closed-scope` | The decision scope is complete and closed, authorizing the selected closed-boundary rule (FR-094). Valid with truth `satisfied` or `violated`; closure of the decision scope does not imply closure of the surrounding execution (FR-061, FR-091). |
| `decisive-witness` | An open decision scope settled `satisfied` because the selected profile proves the retained witness preserves that truth under every admitted continuation (FR-061, FR-091, FR-094). |
| `decisive-counterexample` | An open decision scope settled `violated` because the selected profile proves the retained counterexample preserves that truth under every admitted continuation (FR-061, FR-091, FR-094). |
| `unsettled` | Assessment execution is `completed`, decision-scope progress is open and admitted continuations do not yet preserve one Boolean truth; the truth is `pending` (FR-061, FR-094). |
| `unavailable` | The basis is unavailable: assessment execution is `refused`, `unsupported`, `resource-incomplete` or `failed`, or a required decision-support fact is missing (FR-061, FR-094-AC-7, FR-115). |

## Identity and comparison

- **Identity kind** — *value*. A basis label has no identity of its own and is
  compared componentwise as an exact string.
- **Key** — the label itself. Matching is exact. Display name, source path, span,
  message text, locale and rendering never participate; FR-061 states that a
  strict reader "SHALL NOT infer independence from distinct display names or
  implementations".
- **Key order** — lexicographic over the label, for collection comparison only. It
  carries no strength ranking between bases: it does not make `closed-scope`
  stronger evidence than `decisive-witness`, and it carries no causal meaning.
- **Causal order** — none. The basis is not a causal node. The deciding witness or
  counterexample it names has its own supporting fact and progress identities,
  which are never derived from key order, timestamp or source position.
- **Absence** — an absent settlement basis compares only to absence. It is
  distinct from the supplied value `unavailable` and is never defaulted. FR-061
  requires "exactly one settlement basis" with every truth result.
- **Digest domain** — not applicable; this key contains no digest. See
  [`ix://agent-ix/quire-specification/FR-201`](../foundation/FR-201-canonical-identity-domain.md).

## Recorded divergence: membership is not stated uniformly

Four requirements state this vocabulary and one of them lists only four members.

- [FR-061](../../functional/FR-061-report-orthogonal-results.md): "Every truth
  result SHALL carry exactly one settlement basis from `closed-scope`,
  `decisive-witness`, `decisive-counterexample`, `unsettled` or `unavailable`".
- [FR-094](../../functional/FR-094-interpret-progress-history-and-closure.md)-AC-7:
  "`closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled` and
  `unavailable` are accepted only with their valid truth, scope and exact
  deciding-fact combinations".
- [FR-115](../../functional/FR-115-report-activation-participation-and-adequacy.md):
  "it SHALL retain its `closed-scope`, `decisive-witness`,
  `decisive-counterexample`, `unsettled`, or `unavailable` basis and every exact
  supporting fact/progress identity".
- [FR-091](../../functional/FR-091-evaluate-bounded-future.md): "Such a result
  shall identify `decisive-witness` for true or `decisive-counterexample` for
  false ... Closure of that decision scope authorizes `closed-scope`; it does not
  imply closure of the surrounding execution. An undecided open scope shall remain
  pending with settlement basis `unsettled`."

FR-091 names four members and never names `unavailable`, although FR-091 supplies
the settlement basis "needed by the shared result contract" that FR-061 owns.
This artifact lists the five members FR-061, FR-094-AC-7 and FR-115 each state,
and records that FR-091 does not name the fifth. It adds nothing to FR-091.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-243-AC-1 | Each member's value equals the exact label string in the table. | Inspection |
| FR-243-AC-2 | Exactly one basis accompanies every truth result, with its exact supporting fact and progress identities. | Analysis |
| FR-243-AC-3 | An absent basis compares only to absence and is never defaulted to `unavailable` or `unsettled`. | Analysis |
| FR-243-AC-4 | `closed-scope` on a decision scope is not read as closure of the surrounding execution. | Analysis |

## Dependencies

- [FR-061](../../functional/FR-061-report-orthogonal-results.md) establishes the
  five members and the truth/basis combination rules.
- [FR-091](../../functional/FR-091-evaluate-bounded-future.md) and
  [FR-094](../../functional/FR-094-interpret-progress-history-and-closure.md)
  establish the decisive-settlement and closed-boundary conditions.
- [FR-115](../../functional/FR-115-report-activation-participation-and-adequacy.md)
  restates the vocabulary at the observation-result boundary.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Open gap

FR-091 lists four members where FR-061, FR-094-AC-7 and FR-115 list five. No
requirement states whether FR-091's evaluator may emit `unavailable`. Both lists
are quoted above; this artifact does not merge them.
