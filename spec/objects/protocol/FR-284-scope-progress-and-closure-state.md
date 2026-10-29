---
id: FR-284
title: "Scope progress and closure state vocabulary"
type: FR
object: enumeration
relationships:
  - target: ix://agent-ix/quire-specification/FR-061
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-091
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-115
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-247
    type: references
---
# FR-284: Scope progress and closure state vocabulary

## Description

When decision-scope progress, decision-scope closure, surrounding-execution progress or surrounding-execution closure is reported, the result producer SHALL retain exactly one state from this closed two-member vocabulary on each of those four axes independently.

## Values

| Value | Description |
| --- | --- |
| `open` | On a progress axis: the scope's retained progress point does not yet cover every required source, subject and valuation through the scope's required boundary (FR-061, FR-094). On a closure axis: the scope's exact closure authority has not closed it (FR-061, FR-091). FR-061 uses both readings verbatim — "decision-scope progress is open" and "An open decision scope" — and FR-094-AC-3 pairs the label across both scopes. |
| `closed` | On a progress axis: the retained progress point covers every required source, subject and valuation through the scope's required boundary. FR-094 fixes that condition: a watermark at `W` settles a deadline `D` "only when `W >= D` and completeness covers every required source, subject and valuation through `D`". On a closure axis: the scope's exact closure authority has closed it (FR-061 "A closed decision scope", "closed surrounding execution"; FR-091 "Closure of that decision scope authorizes `closed-scope`"). |

Both label strings are used by the accepted requirements for both scopes and for
progress. This artifact mints no new spelling.

## The four axes this vocabulary governs

FR-061 states the decomposition: "Decision-scope progress and closure and
surrounding-execution progress and closure are four independent facts with exact
scope, authority and boundary identities." Each of the four carries one state
from the table above, together with its own exact scope, authority and boundary
identities, which are not part of this vocabulary.

| Axis | `closed` means |
| --- | --- |
| decision-scope progress | The retained progress point covers the decision scope's required boundary. |
| decision-scope closure | The decision scope's exact closure authority has closed it. |
| surrounding-execution progress | The retained progress point covers the surrounding execution's required boundary. |
| surrounding-execution closure | The surrounding execution's exact closure authority has closed it. |

The four remain independent. FR-061 forbids substitution outright: "The result
producer SHALL NOT use completeness, assessment execution or any other
progress/closure fact as a substitute for any closure axis." FR-094 forbids the
specific carry: a complete closed decision scope "shall not restamp the
surrounding execution as closed". FR-091 states the same separation: "Closure of
that decision scope authorizes `closed-scope`; it does not imply closure of the
surrounding execution." A `closed` progress fact therefore never closes its own
scope's closure fact, and a `closed` closure fact on one scope never closes the
other.

## Not the fifth axis

Complete-global-conformance closure is a fifth independent fact carrying a
different, five-member vocabulary defined by
[FR-247](FR-247-complete-global-conformance-closure-state.md): `closed`, `open`,
`incomplete`, `contradicted`, `not-required`. Its extra three members belong to
that axis alone. FR-061 refuses the cross-wire directly — "another closure axis
relabelled `not-required` is refused" — and FR-059 refuses the substitution in
the other direction: "a closed decision scope, settled per-obligation truth,
completed assessment or closed surrounding execution cannot substitute for a
missing or non-closed global premise".

A label equal to `closed` or `open` on one of the four axes and on the fifth is
one label string in two distinct vocabularies. Equality of the label does not
make the axes equal or substitutable.

## Non-members

No other label is a member of any of the four axes:

- `incomplete`, `contradicted`, `not-required` — members of the fifth axis only
  (FR-247). A retained premise that is later contradicted is reported by
  supersession, not by a closure label: FR-061-AC-9 requires a superseding or
  invalidating result that preserves the earlier bytes.
- `complete`, `incomplete` as an *input* state — a separate axis. FR-094-AC-3
  holds "complete/incomplete input" distinct from "Open/closed decision scope,
  open/closed surrounding execution", and FR-094 classifies "a file labelled
  closed without complete required observations as an incomplete input".
- `completed`, `failed` — the assessment-execution axis
  ([FR-241](FR-241-assessment-execution-disposition.md)), which FR-061 forbids
  using as a substitute for any closure axis.
- `not-started`, `advanced`, `stalled`, `unobserved` — named by no requirement.
  They are not members here and are not to be reintroduced by a consumer.

## Identity and comparison

- **Identity kind** — *value*. A state label has no identity of its own and is
  compared componentwise as an exact string. The *fact* it belongs to
  additionally carries the exact scope, authority and boundary identities
  required by FR-061, and those identities are compared under
  [FR-258](../temporal/FR-258-temporal-decision-scope.md)'s key.
- **Key** — the label itself. Matching is exact. Display name, source path, span,
  message text, locale and rendering never participate; FR-061 states a strict
  reader "SHALL NOT infer independence from distinct display names or
  implementations".
- **Key order** — lexicographic over the label, for collection comparison only.
  `closed` sorting before `open` is an artefact of that ordering and carries no
  progression from `open` towards `closed`, no precedence and no causal meaning.
- **Causal order** — none over the labels. A progress axis advances monotonically
  within its identified binding (FR-094), and that monotone advance is the
  authored progression of the named clock binding
  ([FR-252](../temporal/FR-252-temporal-clock-binding.md)). It is never derived
  from key order, timestamp or source position.
- **Absence** — an absent axis compares only to absence. It is never defaulted to
  `open` or to `closed`: FR-061 requires that "A strict reader SHALL refuse an
  omitted required axis". Absence is not a member of this vocabulary.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-284-AC-1 | Each member's value equals the exact label string in the table, and each is used by the requirement cited in its description. | Inspection |
| FR-284-AC-2 | Each of the four axes carries exactly one member, and a label outside this vocabulary on any of the four is refused. | Analysis |
| FR-284-AC-3 | `not-required`, `incomplete` and `contradicted` are accepted on the complete-global-conformance axis and refused on each of the four. | Analysis |
| FR-284-AC-4 | A `closed` progress fact leaves its own scope's closure fact independently readable and does not close it. | Analysis |
| FR-284-AC-5 | A `closed` closure fact on one scope leaves the other scope's progress and closure facts unchanged. | Analysis |
| FR-284-AC-6 | An omitted axis compares only to absence and receives no default member. | Analysis |
| FR-284-AC-7 | The declared key order carries no progression from `open` towards `closed`. | Inspection |
| FR-284-AC-8 | The four axes and the fifth axis are varied independently across their own vocabularies without any combination being derived from another. | Analysis |

## Dependencies

- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md)
  establishes the four axes, their two-member vocabulary, the per-axis meaning,
  the non-members and the absence rule.
- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) and
  [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establish the progress-coverage condition, monotonicity and the no-restamp rule.
- [FR-115](../../functional/observation/FR-115-report-activation-participation-and-adequacy.md)
  restates the four axes at the observation-result boundary.
- [FR-247](FR-247-complete-global-conformance-closure-state.md) defines the
  distinct fifth-axis vocabulary.
- [FR-240](FR-240-canonical-protocol-result.md) carries the four axes as result
  properties; [FR-258](../temporal/FR-258-temporal-decision-scope.md) carries them
  as observed facts about an identified scope.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the identity
  discipline.

## Status

Draft. This artifact types a vocabulary declared by
[FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) and
claims no implementation coverage.
