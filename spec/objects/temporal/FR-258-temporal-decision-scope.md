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
  - target: ix://agent-ix/quire-specification/FR-240
    type: references
  - target: ix://agent-ix/quire-specification/FR-261
    type: references
  - target: ix://agent-ix/quire-specification/FR-264
    type: references
  - target: ix://agent-ix/quire-specification/FR-284
    type: references
  - target: ix://agent-ix/quire-specification/FR-295
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

| Field | Type | Multiplicity | Constraints |
|---|---|---|---|
| scope_role | String | 1..1 |  |
| scope_identity | String | 1..1 |  |
| progress_state | TemporalProgressAssertion | 1..1 |  |
| closure_state | ScopeProgressAndClosureStateVocabulary | 1..1 |  |
| input_completeness | String | 1..1 |  |
| exact_decision_support_set | String | 1..1 |  |

## Field sources

| Property | Source | Note |
| --- | --- | --- |
| Scope role | FR-094 Inputs, FR-094 Behavior | Either the decision scope or the surrounding execution. The two are never merged. |
| Scope identity | FR-094 Inputs | "exact decision-scope and surrounding-execution identities". |
| Progress state | FR-094 Behavior, FR-094-AC-3 | An independent, mutable axis, advanced by a progress assertion ([FR-256](FR-256-temporal-progress-assertion.md)). It carries one state from [FR-284](../protocol/FR-284-scope-progress-and-closure-state.md): `closed` once the retained progress point covers the scope's required boundary, `open` otherwise. |
| Closure state | FR-094 Behavior, FR-094-AC-3 | An independent, mutable axis carrying its own closure authority. It carries one state from [FR-284](../protocol/FR-284-scope-progress-and-closure-state.md): `closed` once that authority has closed the scope, `open` otherwise. |
| Input completeness | FR-094 Behavior, FR-094-AC-3 | An independent, mutable axis carrying exactly one FR-295 `complete`, `incomplete` or `contradicted` state and its exact population/boundary/assertion facts. |
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
- **Key** — (scope role, scope identity). Equality is componentwise over exactly
  those two components and nothing else. Display name, source path, span,
  message text, locale and rendering never participate
  ([FR-009](../../functional/foundation/FR-009-preserve-object-identity.md),
  [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)). Scope role is
  a key component because FR-094 forbids substituting one axis for another and
  FR-061 requires that the producer "SHALL NOT use completeness, assessment
  execution or any other progress/closure fact as a substitute for any closure
  axis".

  **Progress state, closure state, input completeness and the exact
  decision-support set are outside the key.** They are the mutable facts
  observed *about* an identified scope, not components of the scope's identity.
  FR-094 requires that "Progress shall be monotonic within its identified
  binding" and FR-094-AC-8 requires that a "same-binding progress regression or
  conflicting assertion revision" be detected as a typed contradiction or
  refusal. Both rules presuppose that the same scope stays the same subject
  while its progress advances: if a mutable axis participated in the key, one
  scope observed at two progress points would compare as two unequal objects, a
  regression would be a different object rather than a detected contradiction,
  and monotonicity would have nothing to hold over. This follows the exclusion
  pattern of [FR-261](../observation/FR-261-observation-binding.md), whose
  concrete mapping and admitted record set stay outside the binding key, and of
  [FR-264](../observation/FR-264-membership-rule.md), whose admitted member set
  stays outside the rule key.
- **Key order** — lexicographic over (scope role, scope identity), for
  collection comparison only. It carries no causal meaning and no progression
  from open towards closed; because no progress or closure fact participates in
  the key, no key order over these records can express such a progression.
- **Causal order** — none of its own. Progress advances under the admitted
  causal/sequence order of the named clock binding
  ([FR-252](FR-252-temporal-clock-binding.md)), never derived from key order,
  timestamp or ingestion order.
- **Absence** — an unreported progress, closure or completeness fact is absent
  and compares only to absence; it is never defaulted to open, to closed or to
  complete. FR-061 states that "A strict reader SHALL refuse an omitted required
  axis, cross-scope substitution, global premise under `not-required`, or global
  claim without its complete-global-conformance closure record."

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
[FR-243](../protocol/FR-243-settlement-basis.md) in the result object set; this
artifact names its members only where the temporal prose names them.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-258-AC-1 | A closed decision scope and a closed surrounding execution compare as two distinct facts, and closing one does not close the other. | Test |
| FR-258-AC-2 | A file labelled closed without complete required observations compares as an incomplete input rather than as a closed scope. | Test |
| FR-258-AC-3 | An unreported closure or completeness fact compares only to absence and receives no default value. | Test |
| FR-258-AC-4 | The declared key order carries no progression from open towards closed. | Test |
| FR-258-AC-5 | A settled sibling result retains its truth, settlement basis and exact deciding-fact identities unchanged when a resource ceiling stops another obligation. | Test |
| FR-258-AC-6 | One scope observed at two different progress points compares equal on the key, so the later observation is evaluated for monotonicity against the earlier rather than treated as a different scope. | Test |
| FR-258-AC-7 | The progress and closure axes each carry exactly one FR-284 member, while input completeness carries exactly one independent FR-295 member; cross-vocabulary substitution is refused. | Test |

## Dependencies

- [FR-091](../../functional/temporal/FR-091-evaluate-bounded-future.md) establishes the
  decision-scope closure rule and the early-settlement condition.
- [FR-094](../../functional/temporal/FR-094-interpret-progress-history-and-closure.md)
  establishes the independence of the axes and the incomplete-input rule.
- [FR-061](../../functional/protocol/FR-061-report-orthogonal-results.md) establishes the
  shared result contract these facts are reported through.
- [FR-256](FR-256-temporal-progress-assertion.md) supplies the assertion that
  advances progress or stamps closure.
- [FR-240](../protocol/FR-240-canonical-protocol-result.md) carries the same
  four axes as result properties.
- [FR-284](../protocol/FR-284-scope-progress-and-closure-state.md) defines the
  progress and closure state vocabulary.
- [FR-295](../observation/FR-295-input-completeness-state.md) defines the
  independent completeness vocabulary and derivation.
- [FR-261](../observation/FR-261-observation-binding.md) and
  [FR-264](../observation/FR-264-membership-rule.md) supply the pattern for
  holding an observed, changing set outside a key.
- [FR-200](../foundation/FR-200-quire-assessment-domain.md) supplies the
  identity discipline.

## Resolved vocabulary history

FR-243 resolves the canonical settlement basis as the five-member vocabulary
and defines the explicit lower-boundary `unavailable` adaptation. FR-295 now
defines input completeness as the separate three-member vocabulary
`complete`, `incomplete`, `contradicted`, derived from its exact required fact
population. Equal label spelling between completeness and the fifth global
closure axis does not make the axes equal or substitutable.

## Status

Proposed completeness reconciliation for `quire-specification#40`; the
settlement-basis portion was previously accepted under FR-243.
