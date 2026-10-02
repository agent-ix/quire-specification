---
id: FR-365
title: "Build the canonical counterexample of an explicit-state analysis"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-181
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-362
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-364
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-363
    type: references
---

# FR-365: Build the canonical counterexample of an explicit-state analysis

## Description

When an exhaustive explicit-state analysis refutes a temporal item, the
analyzer SHALL return the canonical counterexample defined below, so that the
same subject, clause and limits always yield the same counterexample.

## Inputs

The product of the subject's state graph with the item's property automaton,
explored in [FR-181](../runtime/FR-181-simulate-finite-executions.md)
canonical breadth-first order; its retained edges; the enabled transition
identities of each model state; and the item's fairness set.

## Outputs

One [FR-364](FR-364-replay-temporal-counterexamples.md) counterexample, a
function of the subject, the clause and the limits.

## Behavior

### Components and order

A product state is (model state, automaton state). The analyzer SHALL
enumerate product states, strongly connected components (SCCs) and their
sub-components in discovery order under FR-181 canonical breadth-first order,
never in hash-map iteration order.

A non-trivial SCC is a **passing component** when it holds a state of every
acceptance set of the automaton and meets every fairness constraint:

- for a weak constraint, it has an edge that takes the constraint or a state
  that does not enable it;
- for a strong constraint, it has an edge that takes the constraint or no
  state that enables it.

An SCC that meets the acceptance and weak conditions and fails the strong
condition for a set `B` of strong constraints is refined: its states that
enable some member of `B` are removed and the remainder is decomposed and
tested again. Enabledness is read from the model state's enabled transition
identities, never from the product's edges. The first passing component is the
first in discovery order, depth-first through refinement.

### Stem

The stem is the first path in FR-181 canonical breadth-first order from an
initial product state to the first passing component. Its last state is the
component's **entry state** `e`.

### Loop: the greedy walk

The obligations of the passing component `C` are: one state of each acceptance
set; for each weak constraint, an edge of `C` that takes it or a state of `C`
that does not enable it; and for each strong constraint that some state of `C`
enables, an edge of `C` that takes it. A strong constraint that no state of `C`
enables has no obligation.

Starting at `e`, with the obligations `e` itself discharges removed, the
analyzer SHALL repeat:

1. Search breadth-first inside `C` from the current state for the nearest state
   or edge that discharges an open obligation, nearest by edge count, ties
   broken by canonical transition order and then by obligation order.
2. Append that path, ending with the discharging edge when the obligation is an
   edge.
3. Remove every obligation the appended path discharges.

When no obligation is open, the analyzer appends the shortest path inside `C`
back to `e`, ties broken by canonical transition order. When that leaves the
loop empty, it appends the shortest cycle through `e`. The walk runs at most
one search per obligation plus one, each linear in the edges of `C`.

The loop discharges every obligation, so the lasso is fair under every
constraint of the item by construction.

### Deadlock and safety counterexamples

The canonical counterexample of a deadlock is the canonical breadth-first path
to the first deadlocked state. The canonical counterexample of a safety or
bounded refutation is the canonical breadth-first path to the first product
state at which the property's monitor rejects.

### Undefined evaluation

A product state at which the claim evaluates undefined
([FR-363](FR-363-settle-every-behaviour-temporal-verdicts.md)) is a
first-phase violation. The first phase explores the product in FR-181
canonical breadth-first order and finds undefined states, rejecting states and
the deadlocked states of a deadlock item. Among those it found, the canonical
counterexample is the one with the shortest prefix; at equal length, the state
discovered first in FR-181 canonical breadth-first order wins. When an
undefined state wins, the counterexample is the canonical breadth-first path
to it, with an `undefined_evaluation` kind (FR-364).

The second phase, which searches for a passing component of a liveness item,
runs only when the first phase found no undefined state. An undefined state
therefore never competes with a liveness stem: a liveness item with a
reachable undefined state within the explored depth is refuted by the
undefined state.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-365-AC-1 | Two runs over the same subject, clause and limits return byte-identical counterexamples, under every registration order and every hash seed. | Test (TC-296) |
| FR-365-AC-2 | The returned lasso's stem is the first canonical breadth-first path to the first passing component, and its loop visits a state of every acceptance set and discharges every weak and strong obligation of that component. | Test (TC-296) |
| FR-365-AC-3 | The greedy walk performs at most one breadth-first search per obligation plus one, and on the TC-293 ConfigVersion subject under `fair weak whole` its loop is three `upd(a)` steps from the initial state with an empty stem. | Test (TC-296) |
| FR-365-AC-4 | An SCC that fails a strong constraint is refined by removing every state at which it is enabled, and a fair accepting cycle in the remainder is found, past more than one level of refinement. | Test (TC-300) |
| FR-365-AC-5 | The canonical deadlock counterexample is the canonical breadth-first path to the first deadlocked state. | Test (TC-297) |
| FR-365-AC-6 | An undefined state ranks as a first-phase violation: the canonical counterexample is the shortest prefix to an undefined, rejecting or deadlocked state, ties broken by canonical breadth-first discovery order, and a winning undefined state yields the canonical breadth-first path to it with an `undefined_evaluation` kind; a liveness item with a reachable undefined state is refuted by that state and its liveness phase never runs, even when a passing component's stem is shorter. | Test (TC-305) |

## Dependencies

- [FR-181](../runtime/FR-181-simulate-finite-executions.md) owns canonical
  breadth-first order and canonical transition order.
- [FR-362](FR-362-apply-fairness-constraints.md) owns enabledness and the
  fairness kinds.
- [FR-364](FR-364-replay-temporal-counterexamples.md) owns the wire form and
  replay that the canonical counterexample satisfies.

## References

- quire-spec-language ADR-018 §3 Determinism, FA-4 and CX-5; ADR-019 SR-2,
  SR-6, SR-7 and RU-3; Linear STD-131 and STD-132.
