---
id: FR-377
title: "Decide refinement between two model subjects"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-375
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
postcondition over `(s, s')` imply the row's rule over `(map(s), map(s'))`.
A per-step obligation reads one concrete step and cannot see history values,
so a declaration with a history field is decided by the explicit-state engine
only.

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
| FR-377-AC-7 | A per-step simulation obligation is one `operation-contract` item per step row and one for the initial-state rule; it is never routed for a declaration with a hidden field or a history field, or for an `any` row. | Test (TC-312) |
| FR-377-AC-8 | The compare-and-set counter refines the abstract counter (28 reachable concrete states, every edge passing the step function, no fair stutter SCC below `value` 2); the same refinement under a concrete `terminal when` member written from the abstract predicate has its concrete deadlock-freedom item proved. | Test (TC-312, TC-313) |

## Dependencies

- [FR-375](FR-375-declare-model-refinement.md) admits the declaration;
  [FR-376](FR-376-evaluate-population-valued-mapping-expressions.md) gives
  population-valued rows their values.
- [FR-378](FR-378-check-refinement-auxiliary-state.md) adds history values
  and the hidden-field candidate set to the product.
- [FR-379](FR-379-report-and-replay-refinement-counterexamples.md) owns the
  verdicts, counterexamples and replay of a refinement item.
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
  RE-2, QS-2, RU-2).
- Tickets: Linear STD-133; paired QSL-367. The model-subject temporal
  requirement is Linear STD-131.
