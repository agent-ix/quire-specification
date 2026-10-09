---
id: FR-385
title: "Apply reductions by the preservation table and report reduced verdicts"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-382
    type: references
  - target: ix://agent-ix/quire-specification/FR-383
    type: references
  - target: ix://agent-ix/quire-specification/FR-384
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-241
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-385: Apply reductions by the preservation table and report reduced verdicts

## Description

When a model-check item selects a reduction, the engine SHALL apply it only
where the preservation table reads "yes" for the claim's form and fairness
set, SHALL settle every other selection before expansion with a named cause,
and SHALL report a reduced proof as its own basis and every refutation as a
concrete, replayed counterexample that names no reduction.

## Inputs

A model-check item with its claim form, fairness set and reduction selection
([FR-382](FR-382-select-state-space-reductions.md)), and the run's outcome
([FR-383](FR-383-explore-under-state-space-reduction.md),
[FR-384](FR-384-decide-each-fairness-on-annotated-quotient.md)).

## Outputs

An [FR-341](../../objects/temporal/FR-341-infinite-trace-result-disposition.md)
result with its [FR-243](../../objects/protocol/FR-243-settlement-basis.md)
basis, proof basis or cause, in the item's
[FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
terminal record.

## Preservation table

A reduction applies to a run exactly when its row reads "yes" for the claim's
form and fairness set. "Whole" and "each" are fairness granularities;
"weak" and "strong" are fairness kinds.

| Form | Symmetry (admitted) | Partial-order reduction | State constraint |
| --- | --- | --- | --- |
| Reachable-state invariant | yes | yes | refutation real; pass only when no boundary state is reached |
| Bounded-profile formula, `on origin` or `on each` | yes | no: counts positions | as the invariant row |
| Infinite-trace safety, no interval operator | yes | yes | as the invariant row |
| Infinite-trace safety with an interval operator | yes | no: counts positions | as the invariant row |
| Liveness, empty fairness set, no interval operator | yes | yes | lasso real (only expanded states form cycles); pass only when no boundary state is reached |
| Liveness with an interval operator under an unbounded one | yes, subject to the fairness rows | no: counts positions | as the liveness row |
| Liveness, weak or strong, every constraint `whole`, including a protocol's default scheduler constraints | yes | yes, with fairness visibility | as the liveness row |
| Liveness, weak or strong, any constraint `each` | yes, on the annotated quotient (FR-384) | yes, with fairness visibility | as the liveness row |
| Deadlock-freedom item | yes | yes | a deadlock found is real; boundary states are never deadlocks; pass only when no boundary state is reached |
| Model refinement, safety half | yes | no: reads step labels | as the invariant row |
| Model refinement, liveness half | yes; `each` rows on the annotated quotient | no | as the liveness row |

Symmetry keeps path lengths, so it preserves position-counting forms.
Partial-order reduction keeps a stuttering-equivalent representative of every
behaviour, which preserves exactly the forms invariant under stuttering
([FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md)). A
combination applies only when every selected reduction's row reads "yes".

## Verdicts

| Outcome | FR-341 label | FR-241 / FR-243 | Proof basis or cause |
| --- | --- | --- | --- |
| Run completed with symmetry or partial-order reduction applied, no counterexample | `proved` | `completed` / `closed-scope` | proof basis `reduced`, listing each applied reduction with its parameters and preservation row, with `certification` `uncertified` |
| Run with only a state constraint, completed, no boundary state reached, no counterexample | `proved` | `completed` / `closed-scope` | proof basis `exhaustive`, with `certification` `uncertified`; the constraint is in the terminal record's method |
| Counterexample found under any reduction | `refuted` | `completed` / `decisive-counterexample` | none; the record names no reduction |
| A selected reduction's row reads "no" | `inconclusive` | `completed` / `unsettled` | `reduction-not-preserving{reduction, form}`, settled before expansion |
| Initial states not closed under a generator | `inconclusive` | `completed` / `unsettled` | `symmetry-broken{population, initial, generator}`, settled before expansion |
| Run completed with no counterexample and at least one boundary state | `inconclusive` | `completed` / `unsettled` | `constraint-reached{boundary_states}`, taking precedence over `reduction-horizon` and a horizon bound when both apply |
| Partial-order run completed to its search horizon `max_depth` with no counterexample | `inconclusive` | `completed` / `unsettled` | `reduction-horizon{max_depth}` |
| Symmetry-only run completed to its search horizon `max_depth` with no counterexample | `inconclusive` | `completed` / `unsettled` | the cause an unreduced run to that horizon settles with |
| A budget stopped the run before it completed its method, under any reduction | `incomplete` | `resource-incomplete` / `unavailable` | `limit-reached{limit, value, setting}` |
| Selected reduction not advertised, or partial-order reduction over a system with no footprints | `unsupported` | `unsupported` / `unavailable` | `unsupported_projection`/`unsupported-requested-capability` naming the reduction |

A claim that evaluates undefined at a reached state settles `refuted` with
cause `undefined-evaluation` by the FR-363 rule, under any reduction, with a
concrete counterexample that names no reduction.

A symmetry- or partial-order-reduced proof's result, and a constraint-only
proof's, carries `certification` `uncertified`
([FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)): no
core checker re-checks the reduced graph, so a consumer can tell it from a
certified proof. A reduced proof is never an exhaustive proof: a consumer that counts proofs
counts `reduced` as its own basis, covering only the forms whose row reads
"yes", with the subject's universes, the symmetry groups and the footprint
scopes as its bounds. `max_depth` is the search horizon, a method parameter
and never a resource limit, so reaching it is a completed method and never a
stopped run. A partial-order run to horizon `k` does not establish that no
counterexample of length at most `k` exists, because a reduced graph does not
keep short counterexamples short, so it settles with its own cause,
`reduction-horizon`, and never with the cause an unreduced run to `k` settles
with. A refutation's counterexample is
concrete and replayed ([FR-383](FR-383-explore-under-state-space-reduction.md)),
and its basis is `decisive-counterexample` whatever reduction found it.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-385-AC-1 | Each cell of the preservation table is applied as stated: a "yes" cell runs the reduction, and a "no" cell settles `inconclusive`, cause `reduction-not-preserving` naming the reduction and form, before any expansion. | Test (TC-323, TC-327) |
| FR-385-AC-2 | A bounded-profile `always[0,2] holds(x.v = 0)` claim with partial-order reduction selected settles `inconclusive`, `reduction-not-preserving{partial-order, bounded}`, before expansion, and never a false pass. | Test (TC-323) |
| FR-385-AC-3 | A completed symmetry or partial-order run with no counterexample settles `proved`, basis `closed-scope`, proof basis `reduced` listing each applied reduction, its parameters and its row, with `certification` `uncertified`; a constraint-only run with no boundary state settles proof basis `exhaustive`, `certification` `uncertified`, with the constraint in the method. | Test (TC-321, TC-323, TC-324) |
| FR-385-AC-4 | A refutation under any reduction settles `refuted`, basis `decisive-counterexample`, with a concrete replayed counterexample, and its record names no reduction. | Test (TC-321, TC-323) |
| FR-385-AC-5 | A constrained run that completes with no counterexample and reaches a boundary state settles `inconclusive`, `constraint-reached` with the boundary-state count, ahead of `reduction-horizon` and a horizon bound; a lasso through a boundary state is never reported. | Test (TC-324) |
| FR-385-AC-6 | A partial-order run that completes to its search horizon `max_depth` with no counterexample settles `inconclusive`, FR-241 `completed`, FR-243 `unsettled`, cause `reduction-horizon` naming the horizon value, and never `failed` or `resource-incomplete`; a symmetry-only run to the same horizon settles `inconclusive` with the cause an unreduced run to that horizon settles with. | Test (TC-323) |
| FR-385-AC-7 | Each result label carries exactly its FR-241 execution and FR-243 basis from the verdict table. | Test (TC-327) |
| FR-385-AC-8 | The protocol `Mark` under its default scheduler constraints with partial-order reduction settles `proved`, proof basis `reduced`, and under `scheduling adversarial` settles `refuted` with the lasso `fork`, `send(S)`, then the loop `duplicate`, `lose`. | Test (TC-325) |
| FR-385-AC-9 | A run under any reduction that reaches `max_states` before completing its method settles `incomplete`, FR-241 `resource-incomplete`, cause `limit-reached{limit: max_states, value, setting}`, and never `failed`. | Test (TC-323) |

## Dependencies

- [FR-382](FR-382-select-state-space-reductions.md) owns the selection and
  pre-check; [FR-383](FR-383-explore-under-state-space-reduction.md) and
  [FR-384](FR-384-decide-each-fairness-on-annotated-quotient.md) own the
  reduced run.
- [FR-341](../../objects/temporal/FR-341-infinite-trace-result-disposition.md),
  [FR-241](../../objects/protocol/FR-241-assessment-execution-disposition.md)
  and [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supply the
  result axes.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the terminal record.
- [FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md) states
  which infinite-trace forms are invariant under stuttering.
- The model-subject temporal requirement (claim forms, the proof-basis and
  inconclusive-cause vocabulary, the search horizon) is a normative prerequisite.

## References

- Design: quire-spec-language ADR-021 (PT-1, PT-2, RV-1 to RV-7, TX-4,
  QS-5, QS-6).
- Tickets: Linear STD-134; paired QSL-368. The model-subject temporal
  requirement is Linear STD-131.
