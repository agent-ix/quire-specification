---
id: FR-377
title: "Decide refinement between two model subjects"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-375
    type: references
  - target: ix://agent-ix/quire-specification/FR-383
    type: references
  - target: ix://agent-ix/quire-specification/FR-376
    type: references
  - target: ix://agent-ix/quire-specification/FR-378
    type: references
  - target: ix://agent-ix/quire-specification/FR-379
    type: references
  - target: ix://agent-ix/quire-specification/FR-181
    type: references
  - target: ix://agent-ix/quire-specification/FR-161
    type: references
  - target: ix://agent-ix/quire-specification/FR-013
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-468
    type: references
---

# FR-377: Decide refinement between two model subjects

## Description

When a refinement item is checked, the analyzer SHALL decide whether every
fair behaviour of the concrete model subject maps, step by step, to a
behaviour of the abstract model subject up to stuttering, and SHALL require
abstract progress only through the abstract fairness set.

## Inputs

An admitted refinement declaration ([FR-375](FR-375-declare-model-refinement.md)),
the concrete subject (its checked package, initial states and universes) and
the abstract subject (its checked package and initial states, with each
abstract universe equal to its source population's concrete universe).

## Outputs

A verdict on the refinement item, with a counterexample for a refutation
([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)).

## Behavior

**Model subjects and behaviours.** A model subject's successor relation is
the one [FR-181](../runtime/FR-181-simulate-finite-executions.md)'s
exploration contract defines. A behaviour is a maximal path from an initial
state whose every position carries the transition identity that reached it;
a terminal state is extended by a terminal stutter step. Fairness
constraints, their `whole` and `each` granularities, and when a transition
identity is enabled or taken are the model-subject temporal semantics'.

**The mapping.** For a concrete state `s` with history values `h`
([FR-378](FR-378-check-refinement-auxiliary-state.md)), `map(s, h)` is the
abstract state with:

- for each abstract population, exactly the objects whose keys are the keys
  of the source population's objects in `s`;
- for each such object, most-specific type the abstract type whose object
  row names the concrete object's most-specific type;
- for each mapped field, the value of its row's expression with the concrete
  object as `self`, a concrete reference to a mapped type read as the
  abstract reference with the same key.

Each row is evaluated as a value-typed clause body over `s`'s observation
with a fresh meter. A row that is refused or incomplete at `s` makes `map`
**undetermined** there. A row that evaluates undefined at `s` makes the
refinement claim evaluate undefined there, which refutes it
([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)).

A mapped abstract field declared with `?` is set to `v` when its row is
`Some(v)` and left absent when it is `none`; a field declared `Option<T>` takes
the row's option value unchanged ([FR-375](FR-375-declare-model-refinement.md),
Option rows). The mapped state holds no relationship extent. A concrete object
whose most-specific type no object row names, and a reference to such an
object, make `map` undetermined at that state; `map` never drops an object it
cannot map.

**Failure precedence.** The rows of one `map(s, h)` are read in a fixed order
that does not depend on the order the rows were written in: the populations in
the node's order, within each population its objects in ascending key order,
and per object the object row of its type and then that type's field rows in
ascending field name as UTF-8 bytes ([FR-375](FR-375-declare-model-refinement.md),
Checked-package node). When several rows fail at one state:

- an undefined row outranks every undetermined row, so the claim is refuted at
  that state even when an earlier row is refused, incomplete or unmapped, and
  the first undefined row in the order is the one reported;
- otherwise the first undetermined row in the order is the one reported;
- an internal fault while evaluating a row, such as a broken evaluator
  invariant, is not an outcome of the row: the item settles `failed`, never a
  value, and the reading takes no part in the order
  ([FR-383](FR-383-explore-under-state-space-reduction.md)).

The reported row, object and cause are the same in a verdict and its replay.

**Failure precedence across a step.** A concrete step reads, at its
position, the receiver of its row, each argument of its row, the post history
`h'` and the rows of `map(s', h')`, in that order: the arguments in the order
the row writes them, the post-state rows in the order above. When several
reads do not return a value:

- an internal fault in any read settles the item `failed` whatever the other
  reads return, and takes no part in the order
  ([FR-383](FR-383-explore-under-state-space-reduction.md));
- otherwise an undefined read outranks every undetermined read, so the claim
  is refuted even when an earlier read is refused, incomplete, out of its
  declared type or of an unmapped type, and the first undefined read in the
  order is the one reported;
- otherwise the first undetermined read in the order is the one reported:
  refused, incomplete, a history update whose result leaves its declared
  type, or a read of an unmapped type.

The reported read and cause are the same in a verdict and its replay.

**Abstract budget stop.** The step rules are decided only when every read
returned a value. Every clause evaluation of either subject gets a fresh
meter of one budget. If an evaluation that the abstract subject runs to
decide the step (a precondition, a frame decision or a postcondition)
exhausts its meter, the run stops: it settles `incomplete`, execution
`resource-incomplete`, cause `limit-reached{limit, value, setting}` with the
limit that bounds one clause evaluation, its configured bound and its
FR-461 `model_check.*` setting
([FR-379](FR-379-report-and-replay-refinement-counterexamples.md)), never
`failed`, `refuted`, `mapping-undetermined` or `undecided-successor`. A fault,
an undefined read or an undetermined read in the same step settles as above
instead, because the step rules are not reached. Which side ran out decides
the outcome: a concrete row, argument or history update that exhausts its
meter makes that read incomplete, hence undetermined, since it is a property
of that one expression at that state, whereas the abstract subject's
exhaustion limits the run. An abstract contract that the clause evaluator
refuses or leaves incomplete without exhausting its meter is an undecided
successor
([FR-363](../temporal/FR-363-settle-every-behaviour-temporal-verdicts.md)).

**Step rules.** Each concrete step `(s, h) -t-> (s', h')` passes or fails by
the row of `t`'s operation, with `a = map(s, h)` and `a' = map(s', h')`:

- **`stutter`** passes when `a'` equals `a` by state key.
- **An abstract operation** passes when the abstract transition identity the
  row names, with receiver and arguments evaluated at `(s, h)` and each `_`
  ranging over its parameter domain, takes `a` to `a'` in the abstract
  subject: its effective precondition holds at `a`, `(a, a')` is within its
  frame ([FR-013](../foundation/FR-013-validate-operation-frames.md)), and its
  effective postcondition holds over `(a, a')`, with the abstract result bound
  to the concrete step's result when both operations declare one, and
  ranging over the result domain otherwise.
- **`any`** passes when it passes the `stutter` rule, or the abstract-operation
  rule for some abstract transition identity enabled at `a`.
- The concrete terminal stutter step is a `stutter` step.

One function decides each step, and every engine and replay answers to it.

**Initial states.** For each initial state `c0` of the concrete subject with
initial history values `h0`, `map(c0, h0)` equals, by state key, an initial
state of the abstract subject.

**Abstract progress.** With `F_A` empty, a mapped behaviour may stutter
forever at any abstract state; the declaration is a safety refinement and
holds even when the concrete model spins or halts where the abstract model
could move. With `F_A` non-empty, `F_A` holds on a mapped behaviour when each
of its constraints holds read through `map`: an abstract transition identity
is enabled at a position when it is enabled at that position's mapped state,
and taken at a step that is an abstract-operation step for it, or an `any`
step matched to it. A stutter tail violates the refinement exactly when it
violates an `F_A` constraint: the constraint is enabled at the tail's mapped
state and no step of the tail takes it.

**Refinement.** The concrete subject refines the abstract subject under the
declaration exactly when the initial-state rule holds, and every behaviour of
the concrete subject that is fair under `F_C` passes the step rules at every
step and satisfies `F_A` on its mapped behaviour. The **safety half** is the
initial-state and step rules; it is a safety property of the concrete
subject, and every violation is a finite prefix ending at its first failing
step. The **liveness half** is `F_A`, present only when `F_A` is non-empty;
it is a liveness property of the concrete subject under `F_C`.

**Terminal states and deadlocks.** A concrete terminal state is read as a
stutter tail at its mapped state. When the mapped state is abstract-terminal
the refinement holds there. When an abstract transition is enabled there, the
refinement holds when `F_A` is empty, and is refuted with failure
`Divergence` when an `F_A` constraint is enabled there. A concrete state where
only `stutter`-mapped operations are enabled is not terminal. The refinement
item reads neither model's `terminal` member, and its verdict is the same
under every combination of them. The deadlock-freedom items of the two
models are separate items that the model-subject temporal semantics define;
a request with a refinement item over the concrete subject carries the
concrete model's deadlock-freedom item. A concrete `terminal when P_A'`
member, with `P_A'` the abstract predicate with each abstract field replaced
by its field row, can be written when the abstract predicate reads only
fields mapped by rows over concrete fields.

**Engines.** An explicit-state engine decides both halves on the product of
the concrete subject, its history values and, when hidden fields exist, the
FR-378 candidate set, running the step function on every retained edge and,
when `F_A` is non-empty, deciding `F_A` under the `F_C` filter on the
retained graph. A per-step simulation engine discharges the safety half of a
declaration with no hidden field, no history field and no `any` row as one `operation-contract`
obligation per step row and one for the initial-state rule
([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)): the
concrete invariants, the operation's effective precondition, frame and
postcondition over `(s, s')` imply the row's rule over `(map(s), map(s'))`. The step obligation is per application: each application of the row is its own (receiver, argument vector) instance, evaluated at `s0`, and the obligation is negated per application, never as one global negation ([FR-468](FR-468-certify-per-step-refinement-obligations.md) negates per pair `(s, s')`).
A per-step obligation reads one concrete step and cannot see history values,
so a declaration with a history field is decided by the explicit-state engine
only.

**Timed concrete subject.** A refinement whose concrete model is timed
([FR-415](../temporal/FR-415-declare-clocks-and-timed-models.md)) and whose
abstract model is untimed is admitted. The checker SHALL refuse:

- a refinement whose abstract model is timed, with
  `unsupported_construct`/`declaration-form` at the refinement's abstract
  model reference;
- a mapping row or step row that reads a clock field of the concrete model,
  with `unsupported_construct`/`expression-form` at the read, so the mapped
  abstract state is a function of the discrete state alone.

A delay of the concrete model maps to no abstract step: the step function
SHALL run only on discrete steps, and the abstract state is unchanged across
a delay, which is an abstract stutter. Over a `time dense` concrete subject,
the symbolic provider (FR-416) SHALL decide the safety half on the product
whose states are (discrete product state, zone), running the step function on
each discrete symbolic edge in canonical breadth-first order. A failing edge
SHALL refute only when its pre-state has a point with a time-divergent
continuation, and its counterexample carries exact rational delays chosen
canonically (FR-416) that replay by FR-181 timed replay. The liveness half
over a timed concrete subject SHALL read only time-divergent concrete
behaviours: an accepting cycle refutes only when its loop has positive total
delay.

**Stuttering-invariant reading.** With stutter positions removed, an infinite
mapped behaviour is a behaviour of the abstract subject, and a finite one is a
prefix of one followed by a stutter tail that `F_A` admits. Infinite-trace
until and since are non-strict ([FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-377-AC-1 | `map` builds each abstract population from its source population by key, each abstract object's most-specific type from its object row, and each mapped field from its row with references lifted by key; a row that is refused or incomplete makes `map` undetermined at that state, and a row that evaluates undefined makes the refinement claim evaluate undefined there. | Test (TC-311, TC-315) |
| FR-377-AC-2 | A `stutter` step passes exactly when the mapped state is unchanged by state key; an abstract-operation step passes exactly when the named abstract transition identity's precondition, frame and postcondition admit the mapped pair, with the abstract result bound to the concrete result; an `any` step passes when either rule admits the mapped pair. | Test (TC-312, TC-313, TC-316) |
| FR-377-AC-3 | A concrete initial state whose mapped state equals no abstract initial state by state key refutes the refinement at the initial state. | Test (TC-313) |
| FR-377-AC-4 | With `F_A` empty, a concrete loop of `stutter` steps at a mapped state where an abstract transition is enabled does not refute the refinement; with an `F_A` constraint enabled there and the loop fair under `F_C`, it refutes with failure `Divergence`. | Test (TC-312) |
| FR-377-AC-5 | A concrete terminal state whose mapped state is abstract-terminal holds; one whose mapped state enables an abstract transition holds with `F_A` empty and refutes `Divergence` when an `F_A` constraint is enabled there; each verdict is the same under all four combinations of the two models' `terminal` members. | Test (TC-313) |
| FR-377-AC-6 | A concrete state where only `stutter`-mapped operations are enabled is not a deadlock of the concrete model, and a request with a refinement item over the concrete subject also carries the concrete model's deadlock-freedom item as a separate item. | Test (TC-313) |
| FR-377-AC-7 | A per-step simulation obligation is one `operation-contract` item per step row and one for the initial-state rule; it is never routed for a declaration with a hidden field, a history field or an `any` row. | Test (TC-312) |
| FR-377-AC-8 | The compare-and-set counter refines the abstract counter (28 reachable concrete states, every edge passing the step function, no fair stutter SCC below `value` 2); the same refinement under a concrete `terminal when` member written from the abstract predicate has its concrete deadlock-freedom item proved. | Test (TC-312, TC-313) |
| FR-377-AC-9 | Among several rows that fail at one state, the first undefined row in the order (object key, then field name bytes) is reported and refutes the claim even when an earlier row is refused, incomplete or unmapped; otherwise the first undetermined row is reported; writing the rows in another source order changes neither the order nor the reported row; an internal fault settles the item `failed` and is never a row value. | Test (TC-315) |
| FR-377-AC-10 | A mapped field declared `?` is absent for the row `none` and set to `v` for `Some(v)`, and a field declared `Option<T>` keeps an explicit `none`. | Test (TC-315) |
| FR-377-AC-11 | A concrete object of a most-specific type no object row names makes `map` undetermined at that state and is never dropped from the mapped state; a reference to such an object makes `map` undetermined naming the row that holds it; the mapped state of a refinement holds no relationship extent. | Test (TC-315) |
| FR-377-AC-12 | Among the receiver, arguments, post history and post-state rows of one step, an internal fault settles the item `failed` whatever the other reads return; otherwise the first undefined read in the order receiver, arguments as written, post history, post-state rows is reported and refutes the claim even when an earlier read is refused, incomplete, out of type or of an unmapped type; otherwise the first undetermined read is reported; an undefined receiver is reported before an undefined argument, an undefined argument before an undefined post-state row, an undefined post history before an undefined post-state row, and an incomplete argument before an out-of-type post history; the reported read is the same in the verdict and its replay. | Test (TC-315) |
| FR-377-AC-13 | A meter the abstract subject exhausts while deciding a step whose reads all returned values settles `incomplete`, execution `resource-incomplete`, cause `limit-reached{limit, value, setting}` for the limit bounding one clause evaluation, never `failed`, `refuted`, `mapping-undetermined` or `undecided-successor`; an undefined read, an undetermined read or a fault in the same step settles by AC-12 instead of the stop; an abstract contract left incomplete without exhausting its meter is `undecided-successor`. | Test (TC-315) |
| FR-377-AC-14 | An untimed abstract model `Req` (object `r` with `sent: Bool`, initial `false`; `send` with postcondition `self.sent = true`) refined by a timed model `TReq` (`time dense`; `r` with `sent: Bool` and clock `x`, initial `sent = false`, `x = 0`; `send` with guard `x >= 1` and postcondition `self.sent = true`; step map `send -> send`; mapping `sent -> sent`) settles the safety half `proved`, with no step-function call for a delay; the variant whose mapping is `sent -> (x >= 1)` refuses `unsupported_construct`/`expression-form` at the read of `x`; a refinement whose abstract model is `TReq` and whose concrete model is a copy of `TReq` refuses `unsupported_construct`/`declaration-form` at the abstract model reference. | Test (TC-963) |
| FR-377-AC-15 | `TReq` extended with `trap: Bool` (initial `false`), `send` resetting `x`, `enter` (precondition `self.sent`, guard `x >= 1`, reset `x`, postcondition `self.trap = true`) and `undo` (precondition `self.trap and self.sent`, postcondition `self.sent = false`), with step map `send -> send`, `enter -> stutter`, `undo -> stutter`, settles the safety half `refuted` with the counterexample `send` after delay 1, `enter` after delay 1, `undo` after delay 0, failing at position 3, which replays; with the time invariant `when self.trap { x <= 0 }`, the pre-state points of `undo` reach no time-divergent continuation, so the failing edge does not refute and the safety half settles `proved`, while that model's time-lock-freedom item is refuted. | Test (TC-963) |
| FR-377-AC-16 | `Req` with `send` given the precondition `not self.sent` and `ensure fair weak Req::send`, refined by `TReq` with `send` given the precondition `not self.sent` and an operation `poll` (precondition `not self.sent`, reset `x`), step map `send -> send`, `poll -> stutter`, mapping `sent -> sent`: with the time invariant `when not self.sent { x <= 0 }`, the `poll` loop at `sent = false`, the only loop on which `send` is enabled and not taken, has total delay 0 and does not refute, so the refinement settles `proved`; without the time invariant the same loop admits a positive delay and the refinement settles `refuted` with failure `Divergence` naming `send`, its lasso an empty stem and a loop of one `poll` step whose delay is 1 (the loop's delay set is every positive rational, unbounded, so ADR-026 EZ-8 picks its lower end 0 plus one unit), which replays. | Test (TC-963) |

## Dependencies

- [FR-375](FR-375-declare-model-refinement.md) admits the declaration;
  [FR-376](FR-376-evaluate-population-valued-mapping-expressions.md) gives
  population-valued rows their values.
- [FR-378](FR-378-check-refinement-auxiliary-state.md) adds history values
  and the hidden-field candidate set to the product.
- [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) owns the
  verdicts, counterexamples and replay of a refinement item.
- [FR-468](FR-468-certify-per-step-refinement-obligations.md) owns the
  queries and certificate of the per-step simulation obligations.
- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns the
  successor relation and state keys.
- [FR-013](../foundation/FR-013-validate-operation-frames.md) owns frame
  admission of the abstract pair.
- [FR-161](../temporal/FR-161-analyze-infinite-trace-properties.md) owns the
  infinite-trace operators and fairness filtering.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the
  `temporal-satisfaction` and `operation-contract` kinds.
- The model-subject temporal requirement (behaviours, terminal stutter,
  fairness constraints and deadlock-freedom items) is a normative
  prerequisite.

## References

- Design: quire-spec-language ADR-020 (RS-1 to RS-13, CO-1, CO-2, RE-1,
  RE-2, QS-2, RU-2); ADR-026 §15 and QSL FR-430 (timed refinement).
- Tickets: Linear STD-133; paired QSL-367. The model-subject temporal
  requirement is Linear STD-131.
