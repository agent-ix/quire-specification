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
  - target: ix://agent-ix/quire-specification/FR-360
    type: references
  - target: ix://agent-ix/quire-specification/FR-243
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
  - target: ix://agent-ix/quire-specification/FR-177
    type: references
  - target: ix://agent-ix/quire-specification/FR-366
    type: references
  - target: ix://agent-ix/quire-specification/FR-429
    type: references
  - target: ix://agent-ix/quire-specification/FR-432
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

An [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
result with its settlement basis and cause, a counterexample, and a replay
result.

## Behavior

**Verdicts.**

| Outcome | FR-360 label | FR-243 basis |
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
| A budget stopped the run before it completed its method | `failed`, execution `resource-incomplete` (FR-331 result `incomplete`), cause `limit-reached{limit, value, setting}` | `unavailable` |

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

**Protocol relation failures.** A refinement whose abstract subject is a
protocol clause ([FR-432](../protocol/FR-432-map-every-protocol-step-in-refinements.md))
adds two kinds, for the refusal-set and terminal-success checks of
[FR-177](../protocol/FR-177-check-protocol-refinement.md). A model abstract
subject never yields them, and the seven kinds above keep their members and
rules unchanged.

| Kind | Members | Rule failed |
| --- | --- | --- |
| `RefusalNotPermitted` | `position`, `label` (an `ObservationLabel`: `step`, the FR-432 class spelling of the step with no alias, and `binder`, the step's complete binder record in effective field order, each entry `{field, value}` with a typed value) | the concrete state at `position` refuses `label`, which no stable member of the candidate set refuses |
| `TerminalSuccessMismatch` | `position`, `direction` (`concrete-only` or `abstract-only`) | concrete and abstract terminal success disagree at `position` |

Both are safety failures: the prefix is the `position` concrete steps that
reach the failing product state, the empty prefix at position 0, and there is
no loop. `position` follows the convention of the step kinds above: it is the index of
the post-state of the failing step, which equals the prefix's step count (0
for the initial state). The checks run on the product of FR-377, with no
fairness filter, at every reachable product state whose incoming step rule
passed (position 0 when the initial-state rule passed), so they read no `F_A`
and run with hidden abstract fields. At a product state, `c` is its concrete
state and `S` its candidate set, closed under the refinement's internal steps.

- `c` is **stable** when no enabled concrete step has a `stutter` or `any`
  row. A member of `S` is stable when no enabled abstract step is on the
  internal list.
- `c` **refuses** a label `l` when it is stable and no enabled concrete step
  is visible with label `l`. A member **refuses** `l` when it is stable and
  no enabled abstract step carries `l`.
- `c` is **terminal success** when no concrete step of any kind is enabled
  and the concrete model marks the state intended
  ([FR-366](../temporal/FR-366-check-deadlock-freedom.md)). A member of `S`
  is terminal success when it is a terminal protocol state that is intended
  ([FR-429](../protocol/FR-429-report-protocol-deadlocks.md)). These two
  `terminal` members are read here, for a protocol abstract subject, and
  are still not read by a model refinement (FR-377). An undefined `terminal
  when` predicate settles by the FR-366 and FR-429 rules unchanged.

`RefusalNotPermitted{position, label}` fails when `c` is stable, `S` has a
stable member, every stable member of `S` enables an abstract step carrying
`label`, and `c` refuses `label`. When several labels qualify the reported
`label` is the least by the bytes of its canonical (RFC 8785) encoding, so the
finder is deterministic; replay accepts any qualifying `label`.
`TerminalSuccessMismatch{position, direction}` fails with `concrete-only` when
`c` is terminal success and no member of `S` is, and with `abstract-only` when
no concrete step is enabled at `c`, `c` is not intended, and every member of
`S` is terminal success. A concrete state that still enables a step while
every member is terminal success is decided by the step rules and divergence
check, not here. `abstract-only` also requires `S` non-empty (an emptied `S`
is `NoAbstractMatch`).

**Order.** Product states are scanned in the canonical breadth-first order of
the explored product. At each state the checks run in this order and the
first failure is reported: the step rules of FR-377 on every discovered edge
into the state (a state is checked by the relation checks only when every such
edge passed), then `RefusalNotPermitted`, then `TerminalSuccessMismatch`.
Safety failures, these kinds included, are reported before the liveness kinds
`Divergence` and `AbstractUnfair`.

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
those at its entry. **Replay of the protocol relation kinds.** For `RefusalNotPermitted` and
`TerminalSuccessMismatch`, replay recomputes `S` over the prefix with the
protocol system and, at the prefix's last state, the stable, refuses and
terminal-success classifications above. It agrees when the recorded failure
holds there: for `RefusalNotPermitted`, `S` has a stable member, every stable
member enables an abstract step carrying the recorded `label`, and `c`
refuses it; for `TerminalSuccessMismatch`, the recorded `direction`
condition holds. Holding is checked per recorded kind, so a relation kind that
holds is accepted even where the other kind also fails, and any qualifying
`label` is accepted. Replay finds no earlier failure of any kind (by the Order
rule). These cases settle the item `inconclusive`, cause `replay-parity`: a
recorded `position` that is not the prefix's step count, a recorded failure
that does not hold at the last state (including a `label` that is not
enabled by every stable member or that `c` does not refuse, a `direction`
whose condition fails, and a state with no failure), and an earlier failure.
A counterexample of these kinds that carries a loop is refused
`invalid_runtime_input`/`invalid-value`.

Agreement settles the replay
`reproduced-with-evaluated-witness` and the item `refuted`. A recomputed
failure of a different kind or at a different position settles the item
`inconclusive`, cause `replay-parity`. The replay source names the
refinement node's identity, and the replay result names the refinement
failure it reproduced. Every other model-trace replay refusal applies
unchanged.

**Wire.** The [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
`counterexamples` entry of a refinement item is this counterexample, and its
`results` entry is the label above with its basis and cause.

**Not covered.** Assumption weakening (FR-177 fourth check) has no failure
kind here; it is specified separately (QSL-1441) and amends this requirement
when its relation is defined.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-379-AC-1 | Each outcome of the verdict table settles exactly its FR-360 label, FR-243 basis and cause, and a per-step simulation failure never settles `refuted`. | Test (TC-312, TC-315) |
| FR-379-AC-2 | A refutation carries exactly one `refinement_failure` member of the seven kinds with their listed members, or on a protocol abstract subject of the nine kinds of both tables; a safety failure is a finite prefix ending at its failing step and a liveness failure is a lasso fair under `F_C`; the counterexample carries concrete steps only. | Test (TC-312, TC-313) |
| FR-379-AC-3 | Replay re-executes the concrete steps, recomputes history values, `map` and candidate sets, reruns the step function at the failing position, and settles `reproduced-with-evaluated-witness` and the item `refuted` when it agrees. | Test (TC-315) |
| FR-379-AC-4 | A lasso whose history values or candidate set at the loop's last position differ from those at its entry is refused `invalid_runtime_input`/`invalid-value`. | Test (TC-315) |
| FR-379-AC-5 | A replay that recomputes a different failure kind, or the same kind at a different position, settles the item `inconclusive`, cause `replay-parity`. | Test (TC-315) |
| FR-379-AC-6 | A mapping row undefined at a reachable state settles `refuted`, cause `undefined-evaluation`, naming the row, the state, the expression locus and the catalogued reason, with an `Undefined` counterexample ending at that state that replays to the same undefined value; a row refused or incomplete there settles `inconclusive`, cause `mapping-undetermined`, naming the row, the state and the reaching prefix. | Test (TC-315, TC-316, TC-311) |
| FR-379-AC-7 | The lost-update counter refutes with `AbstractStepRejected{position: 4, transition: inc(c), cause: Postcondition}` after the prefix `beginA`, `beginB`, `commitA`, `commitB`, and replay recomputes the mapped values 0, 0, 0, 1, 1 and the same failure. | Test (TC-312) |
| FR-379-AC-8 | Over a protocol abstract subject, a stable concrete state that refuses a label every stable member of the candidate set enables fails `RefusalNotPermitted{position, label}` with `position` the prefix's step count, and the reported `label` is the least qualifying label by canonical encoding. | Test (TC-318, TC-431) |
| FR-379-AC-9 | `RefusalNotPermitted` is not reported when some stable member of the candidate set refuses the label, when the candidate set has no stable member, when the concrete state enables a `stutter` or `any` step, or when a concrete step visible with the label is enabled. | Test (TC-318) |
| FR-379-AC-10 | A terminal-success concrete state with no terminal-success member of the candidate set fails `TerminalSuccessMismatch{position, direction: concrete-only}`, and a terminal non-intended concrete state with every member terminal success fails it with `abstract-only`. | Test (TC-318, TC-431) |
| FR-379-AC-11 | `TerminalSuccessMismatch` is not reported when some member of the candidate set is terminal success (`concrete-only`), when some member is not (`abstract-only`), or when the concrete state enables a step. | Test (TC-318) |
| FR-379-AC-12 | A failing step rule on the incoming step is reported instead of a protocol relation failure at its post-state, and where both relation kinds fail at one state `RefusalNotPermitted` is reported; a model abstract subject never yields either kind. | Test (TC-318, TC-431) |
| FR-379-AC-13 | Replay of `RefusalNotPermitted` or `TerminalSuccessMismatch` recomputes the candidate set and the classifications at the prefix's last state and settles `reproduced-with-evaluated-witness` and the item `refuted` when the recorded failure holds; a recorded `position` that differs from the prefix's step count, a label no stable member enables or the concrete state does not refuse, a `direction` that does not hold, or no recomputed failure settles `inconclusive`, cause `replay-parity`; a loop on these kinds is refused `invalid_runtime_input`/`invalid-value`. | Test (TC-318, TC-431) |
| FR-379-AC-14 | The `RefinementFailure` wire definition admits exactly the nine kinds with their members, `ObservationLabel` is closed, and the seven original kinds keep their wire shapes. | Test (TC-431) |

## Dependencies

- [FR-377](FR-377-decide-model-refinement.md) and
  [FR-378](FR-378-check-refinement-auxiliary-state.md) decide the item.
- [FR-360](../../objects/temporal/FR-360-infinite-trace-result-disposition.md)
  and [FR-243](../../objects/protocol/FR-243-settlement-basis.md) supply the
  result labels and settlement bases.
- [FR-331](../../objects/interfaces/FR-331-backend-provider-envelope.md)
  carries the result and counterexample.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  successor relation replay re-executes.
- [FR-177](../protocol/FR-177-check-protocol-refinement.md) names the
  refusal-set and terminal-success checks, and
  [FR-432](../protocol/FR-432-map-every-protocol-step-in-refinements.md),
  [FR-366](../temporal/FR-366-check-deadlock-freedom.md) and
  [FR-429](../protocol/FR-429-report-protocol-deadlocks.md) supply the labels,
  internal list and intended-terminal classification they read.
- The model-subject temporal requirement (its counterexample, replay and
  inconclusive-cause vocabulary) is a normative prerequisite.

## References

- Design: quire-spec-language ADR-020 (RC-1 to RC-3, RE-4, QS-6) and the
  "Strength" paragraph of its §5.
- Tickets: Linear STD-133; paired QSL-367; STD-631 for the protocol relation
  failures, asked by QSL-1442 for QSL-1439 and QSL FR-148. The model-subject temporal
  requirement is Linear STD-131.
