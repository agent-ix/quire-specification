---
id: TC-200
title: "Infinite-trace provider boundary"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-161
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-341
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-095
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-090
    type: verifies
---

# TC-200: Infinite-trace provider boundary

## Description

Sound infinite-trace analysis differential and typed-boundary controls for
`always`, `eventually`, `until`, `release` and the past duals `previous`,
`historically`, `once` and `since`, selected under
`quire.temporal.infinite-trace/v1`. Verifies the FR-161 result vocabulary maps
exactly onto FR-341/FR-241/FR-242/FR-243, that infinite-trace requests never
reach a TL target, and that a missing liveness backend settles `unsupported`
rather than holding.

## Test Procedure

For each of `always`, `eventually`, `until`, `release`, `previous`,
`historically`, `once` and `since`, author the interval-less form admitted only
under the selected `quire.temporal.infinite-trace/v1` profile
([FR-090](../functional/temporal/FR-090-select-temporal-profile-and-clock.md),
[AD-012](../assurance/AD-012-unbounded-temporal-operator-grammar.md)). Exercise
each operator against:

1. A conformance lasso (finite prefix plus closed loop) that satisfies it under
   the inductive infinite-trace definitions in FR-161's "Infinite-trace
   semantics", under an explicit fairness assumption.
2. A conformance lasso, and a bare finite prefix with a complete bad prefix,
   that refute it.
3. A finite prefix with no complete lasso and no complete bad prefix, where
   liveness remains open.
4. A request naming an operator or model/fairness closure the selected
   provider declares unsupported.
5. A request sized past the declared resource ceiling for the selected
   provider.
6. A request submitted while no backend has registered the infinite-trace
   capability.

Independently mutate the fairness assumption on one otherwise-proving case and
confirm the lasso no longer proves the property. For every result, retrieve
premises, and for proved/refuted results, retrieve the witness or counterexample
and replay it against an independent evaluator of the same inductive
definitions. In parallel, submit one event-position bounded-future request and
one infinite-trace request from the same source population to the FR-095
bridge and inspect its support-table disposition for each.

## Expected Results

The conformance-lasso case returns `proved`; the refuting cases return
`refuted` with a replayable witness or counterexample whose fairness identity
is included in its provenance (FR-161-AC-1, FR-161-AC-4, FR-161-AC-5). The
open-liveness case returns `inconclusive`, never `proved` (FR-161-AC-2). The
unsupported-operator/closure case and the resource-ceiling case each return a
distinct non-conclusive disposition — `unsupported` and `failed` respectively —
and neither is interchangeable with `inconclusive` (FR-161-AC-3). The
no-registered-backend case settles `unsupported` with a warning naming the
missing liveness capability; it never holds, blocks or returns a refusal error
(FR-161-AC-7). The mutated-fairness case stops proving and returns
`inconclusive` or `unsupported`, never a stale `proved`.

Every returned result label matches exactly one [FR-341](../objects/temporal/FR-341-infinite-trace-result-disposition.md)
member with its required FR-241 execution, FR-242 truth and FR-243 settlement-basis
combination: `proved`/`refuted` carry execution `completed` and basis
`closed-scope`, `decisive-witness` or `decisive-counterexample`; `inconclusive`
carries execution `completed`, truth `pending` and basis `unsettled`;
`unsupported` and `failed` carry truth and basis `unavailable` and are
distinguished only by their own FR-241 execution disposition (FR-161-AC-6).

The infinite-trace bridge request in the parallel FR-095 check returns a typed
unsupported mapping naming an absent TL target and emits no substitute formula,
while the event-position request from the same population resolves normally;
[FR-251](../objects/temporal/FR-251-tl-target-profile.md) records the
infinite-trace target as absent rather than as either of its two members
(FR-095-AC-9, FR-251-AC-3).
