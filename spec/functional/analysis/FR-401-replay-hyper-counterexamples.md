---
id: FR-401
title: "Replay hyper counterexamples"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-400
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-399
    type: references
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/FR-364
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
---

# FR-401: Replay hyper counterexamples

## Description

When a hyper counterexample reaches the replay boundary, the replay
boundary SHALL settle the item only from a fresh replay of every trace through its
subject's native transition relation by the rule of its kind.

## Inputs

The `HyperCounterexample` ([FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md)),
the package bytes, one subject per alias, the checked clause and its instance,
and the request's limits.

## Outputs

`reproduced-with-evaluated-witness` and the item `refuted`, or `inconclusive`
with `replay-parity` or `replay-refused`, or FR-331 result `incomplete` with
cause `limit-reached{limit, value, setting}` or `cancelled`, or a refusal
`invalid_runtime_input`/`invalid-value`.

## Trace re-execution

Replay recompiles the package, re-admits each subject, checks the match and
each trace's fairness, and runs with no engine or solver present. Each trace re-executes from the initial state its `initial` index names
through its subject's FR-181 transition relation. A step is refused when its
transition is not enabled, or when a terminal-stutter step appears at a
state that is not terminal; a `lockstep` or `witness-exhausted` trace is refused when its
loop does not close at `loop_entry`. A refused step settles the item
`inconclusive` with `replay-parity`. A terminal-stutter step followed by a
non-stutter step, or one in a `step-tuple` trace, is malformed and never
reaches replay: [FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md)
refuses it before replay. Only a well-formed terminal-stutter step whose
pre-state proves not to be terminal is a refused step here.

## Replay rules

| Kind | Checks after re-execution | Reproduces when |
| --- | --- | --- |
| `lockstep` | `μ_U` at every joint step, the loop's closing step included; each trace's fairness set on its own loop | the body, evaluated over the tuple by tuple evaluation, is false |
| `witness-exhausted` | as `lockstep`, without evaluating the body; then recompute `X_0` to `X_position` along the universal traces from the existential subjects and the safety automaton, under the request's `max_witness_set` | `X_position` is empty and every earlier `X` is non-empty |
| `projected` | each `skipped` mark names a listed operation and each `visible` mark an unlisted one; the visible counts are equal; `μ_U` at every joint visible step | the body, evaluated three-valued over the joint projected prefix, is false |
| `step-tuple` | the last step of each trace is an enabled transition of its variable's operation with the recorded post-state | the body, evaluated over the tuple of executions, is false |
| `undefined` | `μ_U` at every joint step before `trace_position`; each component's state key at `trace_position` equals the carried `where.state_keys` | the expression the carried `where` names, evaluated at `trace_position` (the body or a match over every component; or, over the one component `where.variables` names, the guard of the transition `where.transition` names, else the `terminal when` predicate or state constraint at `where.locus`), is the undefined value with the carried locus and catalogued reason ([FR-399](FR-399-settle-hyperproperty-results.md)) |

- When an unmatched joint step or an unfair trace is found, the replay
  boundary SHALL refuse the counterexample as
  `invalid_runtime_input`/`invalid-value`, naming the joint position or the
  trace, and SHALL settle the item `inconclusive` with `replay-refused`.
- When the reproducing condition fails after every check passes (the body
  true, a recomputed `X_position` non-empty, a projected prefix not false, or
  an `undefined` evaluation that is defined or differs from the carried value
  in state, locus or reason),
  the replay boundary SHALL settle the item `inconclusive` with
  `replay-parity`.
- An `undefined` counterexample whose paths differ in step count (that of an
  `align skip` clause, which holds its skipped steps unmarked and whose
  position counts visible steps, and that of an HP-3 clause whose undefined
  `μ_U` leaves the existential paths one step short) has a path with no state at
  `trace_position`, so the row's state-key check cannot read every path at one
  position. The replay boundary SHALL refuse it as
  `unsupported_construct`/`not-yet-implemented`, SHALL settle no result from it
  and SHALL NOT read it in lockstep, which would pair a skipped step of one path
  with a visible step of another.
- When the recomputation of a witness set reaches `max_witness_set`, the
  replay boundary SHALL settle the item `incomplete` with cause
  `limit-reached{limit, value, setting}`, naming `max_witness_set`, its value
  and the setting `model_check.max_witness_set`, as
  [FR-394](FR-394-replay-state-graph-evidence.md) settles a stopped
  re-exploration; a cancelled replay settles `incomplete` with cause
  `cancelled`.
- A `witness-exhausted` refutation proves that a fair tuple of universal
  behaviours and a joint position exist by which every existential tuple
  matched with it has violated the body; replay re-establishes the absence of
  a witness by recomputing `X` exhaustively over the existential subjects
  along a finite prefix, which is why its basis is `decisive-counterexample`.

## Code-claim counterexamples

A step relation's code-claim counterexample gives, per execution, a pre-state,
an argument vector, a result and a post-state, and enters the replay boundary
by the backend-transcript arm of
[FR-197](../ir-proof/FR-197-replay-backend-counterexamples.md). Replay admits
each pre-state as a snapshot, checks that it satisfies its model's declared
invariants, and evaluates the body over the tuple. False reproduces; true
settles `inconclusive` with `replay-parity`; a pre-state that violates an
invariant is refused as `invalid_runtime_input`/`invalid-value`. No
reachability prefix is involved.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-401-AC-1 | The leaky-vault `lockstep` counterexample re-executes both traces, matches inputs at both joint steps, and reproduces with the body false at joint position 1. | Test (TC-338) |
| FR-401-AC-2 | The leaky-vault `witness-exhausted` counterexample reproduces by recomputing `X_0` as a singleton and `X_1` empty with no engine present. | Test (TC-339) |
| FR-401-AC-3 | A counterexample with `μ_U` false at a joint step, or with an unfair component, is refused as `invalid_runtime_input`/`invalid-value` and the item settles `inconclusive` with `replay-refused`. | Test (TC-345) |
| FR-401-AC-4 | A `witness-exhausted` counterexample whose recomputed `X_position` is non-empty settles `inconclusive` with `replay-parity`; one whose recomputation reaches `max_witness_set` settles `incomplete` with cause `limit-reached{limit, value, setting}` naming `max_witness_set` and `model_check.max_witness_set`. | Test (TC-345) |
| FR-401-AC-5 | A `projected` counterexample with a mismatched mark or unequal visible counts is refused, and a valid one reproduces with the three-valued prefix false. | Test (TC-344, TC-345) |
| FR-401-AC-6 | A `step-tuple` counterexample reproduces with the body false over the tuple of executions, and a code-claim counterexample reproduces from admitted pre-states with no reachability prefix. | Test (TC-343) |
| FR-401-AC-7 | A counterexample found under a reduction replays as an ordinary counterexample in quantifier order. | Test (TC-347) |
| FR-401-AC-8 | An `undefined` counterexample reproduces when the expression its `where` names (the body, a match, or a component subject's guard, `terminal when` predicate or state constraint) evaluated at `trace_position` is the counterexample's carried undefined value, with its state keys, locus and catalogued reason, settling `reproduced-with-evaluated-witness` and the item `refuted`; a defined value there, or another locus or reason, settles `inconclusive` with `replay-parity`. | Test (TC-348) |
| FR-401-AC-9 | A well-formed terminal-stutter step whose pre-state is not terminal settles `inconclusive` with `replay-parity`. | Test (TC-345) |
| FR-401-AC-10 | An `undefined` counterexample whose paths differ in step count, that of an `align skip` clause with unmarked paths that each hold a skipped step and a position that counts visible steps and that of an HP-3 `μ_U` with its existential paths one step short, is refused as `unsupported_construct`/`not-yet-implemented`, settles no result and is not replayed as a lockstep counterexample. | Test (TC-345) |

## Dependencies

- [FR-400](../../objects/interfaces/FR-400-hyper-counterexample.md) owns the
  wire.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the transition
  relation.
- [FR-399](FR-399-settle-hyperproperty-results.md) owns the witness set and
  settles the item from the replay.
- [FR-197](../ir-proof/FR-197-replay-backend-counterexamples.md) owns the
  replay boundary and the backend-transcript arm.
- [FR-364](../temporal/FR-364-replay-temporal-counterexamples.md) and [FR-362](../temporal/FR-362-apply-fairness-constraints.md)
  own model-trace re-execution and the fairness check on a loop.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
