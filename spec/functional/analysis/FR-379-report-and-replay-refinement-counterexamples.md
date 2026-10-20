---
id: FR-379
title: "Report and replay refinement verdicts and counterexamples"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-378
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-379: Report and replay refinement verdicts and counterexamples

## Description

When a refinement item settles, the analyzer SHALL report a verdict from the
infinite-trace result vocabulary and, for a refutation, a counterexample over
the concrete subject that names its refinement failure and replays without
the engine that found it.

## Inputs

A refinement item decided under [FR-377](FR-377-decide-model-refinement.md)
and [FR-378](FR-378-check-refinement-auxiliary-state.md), and for replay the
counterexample, the concrete package and the abstract subject's package.

## Outputs

An [FR-341](../../objects/temporal/FR-341-infinite-trace-result-disposition.md)
result with its settlement basis and cause, a counterexample, and a replay
result.

## Behavior

**Verdicts.**

| Outcome | FR-341 label | FR-243 basis |
| --- | --- | --- |
| Explicit-state run completed with no failure | `proved` | `closed-scope` |
| Per-step simulation: every step obligation and the initial-state obligation proved | `proved` | `decisive-witness` |
| Explicit-state or unrolling run found a failure that replays | `refuted` | `decisive-counterexample` |
| A step obligation fails from a pre-state that satisfies the concrete invariants | `inconclusive`, cause induction not closed | `unsettled` |
| A field row, a step-row receiver or argument or a history update evaluates undefined at a reachable state, and its `Undefined` counterexample replays | `refuted`, cause `undefined-evaluation` | `decisive-counterexample` |
| `map`, a receiver, an argument or a history update undetermined at a reachable state | `inconclusive`, cause `mapping-undetermined` | `unsettled` |
| An undecided contract conjunction of either subject during expansion or a step check | `inconclusive`, cause undecided successor | `unsettled` |
| Replay recomputes a different failure kind or position | `inconclusive`, cause `replay-parity` | `unsettled` |
| Liveness half with a hidden field (FR-378) | `unsupported` | `unavailable` |
| A budget stopped the run before it completed its method | `incomplete`, execution `resource-incomplete`, cause `limit-reached{limit, value, setting}` | `unavailable` |

An explicit-state `proved` holds for the concrete subject's initial states
and universes. A per-step `proved` holds for every state of the declared
types. A per-step failure is never `refuted`, because its pre-state may be
unreachable; every refutation comes from an explicit-state or unrolling run
and is replayed.

**Undefined evaluation.** The mapping is part of the refinement claim. A
claim, field row, step-row receiver or argument or history update that evaluates
undefined at a reachable concrete state settles the item `refuted` by the
FR-363 rule, with cause `undefined-evaluation` whose `where` names the row or
update, the position, the concrete state key and the locus of the expression
that has no value, and whose `cause` names the undefined value's catalogued
reason. Its counterexample carries `refinement_failure` `Undefined` and ends
at that state, and replay reproduces the undefined value there.

**`mapping-undetermined`.** A new infinite-trace inconclusive cause: a field
row, a step-row receiver or argument or a history update was refused or incomplete, a
history update left its declared type, or an object or reference of a type no object row names was mapped (an unmapped type), at a reachable state. It names the
row, the state and the concrete prefix reaching it.

**Counterexample.** A refinement counterexample is a model-subject temporal
counterexample over the concrete subject, as the model-subject temporal
semantics define it, with one added `refinement_failure` member, exactly one
of:

| Kind | Members | Rule failed |
| --- | --- | --- |
| `InitialNotAbstract` | `initial` | initial-state rule |
| `StutterChanged` | `position` | `stutter` step |
| `AbstractStepRejected` | `position`, `transition`, `cause` (`Precondition{clause}`, `Frame{code}` or `Postcondition`) | abstract-operation step |
| `NoAbstractMatch` | `position` | `any` step, or an emptied candidate set |
| `Divergence` | `constraint` | an `F_A` constraint violated by a lasso whose loop has only `stutter` steps, including the concrete terminal stutter |
| `Undefined` | `position`, `row` | a field row, step-row receiver or argument or history update evaluated undefined at `position`; `row` names it |
| `AbstractUnfair` | `constraint` | an `F_A` constraint violated by a lasso whose loop takes abstract steps |

A safety failure is a finite prefix ending at its failing step. A liveness
failure is a lasso fair under `F_C`. The counterexample carries concrete
steps only: no mapped state, history value or candidate set.

**Replay.** Replay recompiles the concrete package and the abstract
subject's package, re-executes the concrete steps through the concrete
successor relation ([FR-181](../runtime/FR-181-simulate-finite-executions.md)),
recomputes history values, `map` and candidate sets along the trace, and
reruns the step function at the failing position; for `Undefined` it
re-evaluates `row` at `position` and agrees when it is the undefined value with
the item's locus and catalogued reason. For a lasso it checks the
loop fair under `F_C`, recomputes history values and candidate sets over the
prefix and one pass of the loop, and evaluates the violated `F_A` constraint
over the loop. Replay refuses `invalid_runtime_input`/`invalid-value` when
the history values or candidate set at the loop's last position differ from
those at its entry. Agreement settles the replay
`reproduced-with-evaluated-witness` and the item `refuted`. A recomputed
failure of a different kind or at a different position settles the item
`inconclusive`, cause `replay-parity`. The replay source names the
refinement node's identity, and the replay result names the refinement
failure it reproduced. Every other model-trace replay refusal applies
unchanged.

**Wire.** The [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
`counterexamples` entry of a refinement item is this counterexample, and its
`results` entry is the label above with its basis and cause.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-379-AC-1 | Each outcome of the verdict table settles exactly its FR-341 label, FR-243 basis and cause, and a per-step simulation failure never settles `refuted`. | Test (TC-312, TC-315) |
| FR-379-AC-2 | A refutation carries exactly one `refinement_failure` member of the seven kinds with its listed members; a safety failure is a finite prefix ending at its failing step and a liveness failure is a lasso fair under `F_C`; the counterexample carries concrete steps only. | Test (TC-312, TC-313) |
| FR-379-AC-3 | Replay re-executes the concrete steps, recomputes history values, `map` and candidate sets, reruns the step function at the failing position, and settles `reproduced-with-evaluated-witness` and the item `refuted` when it agrees. | Test (TC-315) |
| FR-379-AC-4 | A lasso whose history values or candidate set at the loop's last position differ from those at its entry is refused `invalid_runtime_input`/`invalid-value`. | Test (TC-315) |
| FR-379-AC-5 | A replay that recomputes a different failure kind, or the same kind at a different position, settles the item `inconclusive`, cause `replay-parity`. | Test (TC-315) |
| FR-379-AC-6 | A mapping row undefined at a reachable state settles `refuted`, cause `undefined-evaluation`, naming the row, the state, the expression locus and the catalogued reason, with an `Undefined` counterexample ending at that state that replays to the same undefined value; a row refused or incomplete there settles `inconclusive`, cause `mapping-undetermined`, naming the row, the state and the reaching prefix. | Test (TC-315, TC-316, TC-311) |
| FR-379-AC-7 | The lost-update counter refutes with `AbstractStepRejected{position: 4, transition: inc(c), cause: Postcondition}` after the prefix `beginA`, `beginB`, `commitA`, `commitB`, and replay recomputes the mapped values 0, 0, 0, 1, 1 and the same failure. | Test (TC-312) |

## Dependencies

- [FR-377](FR-377-decide-model-refinement.md) and
  [FR-378](FR-378-check-refinement-auxiliary-state.md) decide the item.
- [FR-341](../../objects/temporal/FR-341-infinite-trace-result-disposition.md)
  and [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supply the
  result labels and settlement bases.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the result and counterexample.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  successor relation replay re-executes.
- The model-subject temporal requirement (its counterexample, replay and
  inconclusive-cause vocabulary) is a normative prerequisite.

## References

- Design: quire-spec-language ADR-020 (RC-1 to RC-3, RE-4, QS-6) and the
  "Strength" paragraph of its §5.
- Tickets: Linear STD-133; paired QSL-367. The model-subject temporal
  requirement is Linear STD-131.
