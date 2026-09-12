---
id: FR-258
title: "Temporal decision scope and closure"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-091
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-256
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
---
# FR-258: Temporal decision scope and closure

## Description

When a temporal disposition is reported, the evaluator SHALL keep the decision scope and the surrounding execution as separately identified scopes whose progress and closure are independent facts.

FR-091 states the separation: "Closure of that decision scope authorizes
`closed-scope`; it does not imply closure of the surrounding execution." FR-094
states that "A complete closed decision scope shall authorize the selected
closed-boundary rule and settlement basis `closed-scope`. It shall not restamp
the surrounding execution as closed."

## Properties

| Property | Source | Note |
| --- | --- | --- |
| Scope role | FR-094 Inputs, FR-094 Behavior | Either the decision scope or the surrounding execution. The two are never merged. |
| Scope identity | FR-094 Inputs | "exact decision-scope and surrounding-execution identities". |
| Progress state | FR-094 Behavior, FR-094-AC-3 | Open or covered through a named point, supplied by a progress assertion ([FR-256](FR-256-temporal-progress-assertion.md)). |
| Closure state | FR-094 Behavior, FR-094-AC-3 | Open or closed, with its own closure authority. |
| Input completeness | FR-094 Behavior, FR-094-AC-3 | Complete or incomplete, held distinct from closure. |
| Exact decision-support set | FR-091 Behavior, FR-094 Behavior | "the exact valuation, capture, clock, progress and profile facts that form its decision-support set". |

FR-094-AC-3 states the independence as a checkable claim: "Open/closed decision
scope, open/closed surrounding execution, completed/failed assessment execution
and complete/incomplete input remain independently represented; closing one axis
cannot close or complete another."

A closed label is not a closed scope. FR-094 requires the evaluator to
"classify end-of-file, consumer shutdown or a file labelled closed without
complete required observations as an incomplete input", and states that the
evaluator "shall not apply false extension or finite-window empty truth to an
incomplete input."

## Identity and comparison

- **Identity kind** — *value*. A scope record has no identity of its own and is
  compared componentwise; the scope it names is identified by FR-094's exact
  scope identities.
- **Key** — (scope role, scope identity, progress state, closure state, input
  completeness) together with the exact decision-support set where one is
  retained. Equality is componentwise over exactly those components and nothing
  else. Display name, source path, span, message text, locale and rendering
  never participate
  ([FR-009](../../functional/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/FR-047-emit-typed-located-causes.md)). Scope role is
  a key component because FR-094 forbids substituting one axis for another and
  FR-061 requires that the producer "SHALL NOT use completeness, assessment
  execution or any other progress/closure fact as a substitute for any closure
  axis".
- **Key order** — lexicographic over the key components, for collection
  comparison only. It carries no causal meaning and no progression from open
  towards closed; a closed record does not sort as a successor of an open one.
- **Causal order** — none of its own. Progress advances under the admitted
  causal/sequence order of the named clock binding
  ([FR-252](FR-252-temporal-clock-binding.md)), never derived from key order,
  timestamp or ingestion order.
- **Absence** — an unreported progress, closure or completeness fact is absent
  and compares only to absence; it is never defaulted to open, to closed or to
  complete. FR-061 states that "A strict reader SHALL refuse an omitted required
  axis, cross-scope substitution, global premise under `not-required`, or global
  claim without its complete-global-conformance closure record."
- **Digest domain** — a scope or support identity carried as a digest retains
  the domain it was derived under
  ([FR-201](../foundation/FR-201-canonical-identity-domain.md)); an identity
  crossing domains is unequal regardless of byte equality.

## Early settlement on an open scope

FR-091 states the only admitted early-settlement rule: "The evaluator shall
return a Boolean for an open decision scope only when the selected profile
proves that every admitted continuation preserves it. Such a result shall
identify `decisive-witness` for true or `decisive-counterexample` for false and
shall retain the exact valuation, capture, clock, progress and profile facts
that form its decision-support set." FR-091 continues: "An undecided open scope
shall remain pending with settlement basis `unsettled`." FR-094 repeats the
scoping: "All other open futures remain `unsettled` and pending."

FR-094 also states the survival rule under a resource ceiling: "A sibling
obligation that already settled over complete exact decision support shall
retain its prior truth, settlement basis and support; the stop shall not erase
or restamp it."

The settlement-basis vocabulary itself is not enumerated here. It is authored as
[FR-243](../result/FR-243-settlement-basis.md) in the result object set; this
artifact names its members only where the temporal prose names them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-258-AC-1 | A closed decision scope and a closed surrounding execution compare as two distinct facts, and closing one does not close the other. | Analysis |
| FR-258-AC-2 | A file labelled closed without complete required observations compares as an incomplete input rather than as a closed scope. | Analysis |
| FR-258-AC-3 | An unreported closure or completeness fact compares only to absence and receives no default value. | Analysis |
| FR-258-AC-4 | The declared key order carries no progression from open towards closed. | Analysis |
| FR-258-AC-5 | A settled sibling result retains its truth, settlement basis and exact deciding-fact identities unchanged when a resource ceiling stops another obligation. | Analysis |

## Dependencies

- [FR-091](../../functional/FR-091-evaluate-bounded-future.md) establishes the
  decision-scope closure rule and the early-settlement condition.
- [FR-094](../../functional/FR-094-interpret-progress-history-and-closure.md)
  establishes the independence of the axes and the incomplete-input rule.
- [FR-061](../../functional/FR-061-report-orthogonal-results.md) establishes the
  shared result contract these facts are reported through.
- [FR-256](FR-256-temporal-progress-assertion.md) supplies the assertion that
  advances progress or stamps closure.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Open gap

Three requirements state the membership of the settlement-basis vocabulary and
they do not state the same membership. Two list five members, one lists four and
never names the fifth.

FR-061 Behavior:

> "Every truth result SHALL carry exactly one settlement basis from
> `closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled` or
> `unavailable`, plus the exact supporting fact and progress identities."

FR-094-AC-7:

> "`closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled`
> and `unavailable` are accepted only with their valid truth, scope and exact
> deciding-fact combinations; every one-axis substitution is refused."

[FR-115](../../functional/FR-115-report-activation-participation-and-adequacy.md)
Behavior, for a settled truth on an open subject, requires it to retain its
"`closed-scope`, `decisive-witness`, `decisive-counterexample`, `unsettled`, or
`unavailable`" basis.

FR-091 names only four, across three sentences, and never names `unavailable`:

> "Such a result shall identify `decisive-witness` for true or
> `decisive-counterexample` for false"

> "Closure of that decision scope authorizes `closed-scope`"

> "An undecided open scope shall remain pending with settlement basis
> `unsettled`."

FR-091 instead describes the missing-fact case without a basis label: "The
evaluator shall make truth unavailable when a fact is missing inside the exact
decision-support set." Whether `unavailable` is a settlement-basis member that
FR-091 omits, or a truth state that FR-061 and FR-094 additionally spell as a
basis, is not stated anywhere. This artifact records both readings and resolves
neither.

## Status

Draft. This artifact records existing meaning structurally. It establishes no
new obligation and claims no implementation coverage.
