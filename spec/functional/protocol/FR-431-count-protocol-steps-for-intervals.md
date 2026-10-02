---
id: FR-431
title: "Observe protocol positions and measure intervals in counted steps"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-008
    type: implements
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-091
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-092
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-161
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-253
    type: references
  - target: ix://agent-ix/quire-specification/FR-426
    type: depends_on
---

# FR-431: Observe protocol positions and measure intervals in counted steps

## Description

When a temporal clause is evaluated over a behaviour of a protocol subject,
the evaluator SHALL make every step a position with the anchor stated below,
and SHALL measure every interval bound in counted steps, where only
operation, event, send and receive steps count.

## Inputs

A temporal clause and a behaviour of a protocol subject: a maximal path of
its steps ([FR-426](FR-426-apply-protocol-steps.md)) from an initial protocol
state.

## Outputs

The behaviour's positions with their anchors and observations, and the
counted distance between any two positions.

## Behavior

### Positions and anchors

Position 0 is the initial protocol state, observed with an `initialization`
anchor. Position `i > 0` is the post-state of the behaviour's `i`-th step. An
`attempt` or `cattempt` step is observed with its operation's anchor. Every
other step is observed with a `protocol` anchor,
`{"type":"protocol","step":"<kind>","node":"<node>"|null,"name":"<qualified-name>"|null}`,
naming the step kind and its node, or its channel, template, role or trigger
by qualified name. A memory step is observed with the anchor its memory model
gives. A `holds` atom evaluates over the model observation at the position,
each shared location read through the memory model's atom values. Every step
is a position for atoms and for unbounded operators. Rest points, binders,
registrations, role instances and queues are read by replay and
counterexample rendering, not by atoms.

### Counted steps

A step is **counted** when it is an `attempt`, a `cattempt`, an `event`, an
`activate`, a `send` or a `receive`. A `fork`, `join`, `finish`, `timeout`,
`cend`, `duplicate`, `lose`, `spawn`, `retire`, `fence` or memory step is
**uncounted**. The terminal stutter step is counted, as over a model subject.

The **distance** from position `i` to position `j >= i` is the number of
counted steps among steps `i + 1` through `j`. An interval operator measures
distance, not position count: `eventually[a, b] p` holds at `i` when `p` holds
at some `j >= i` whose distance from `i` lies in `[a, b]`; `always`, `until`
and `release` with an interval read distance the same way, and the past
operators with an interval read it backwards ([FR-091](FR-091-evaluate-bounded-future.md),
[FR-092](FR-092-evaluate-bounded-past.md)). Several positions can share one
distance. A bounded profile's horizon is in counted steps.

Over a model subject every step is counted, so distance equals position
count there. A loop with no counted step keeps every distance from a position
before it constant, so an interval that needs a counted step after the loop
is entered is not met on that lasso. A formula with no interval operator reads
the same truth whether or not a step counts.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-431-AC-1 | Along the `Fill` behaviour through `B` then `A`, the counted steps are the two attempts and the `fork`, `join` and `finish` steps are uncounted, so `eventually[0,2] holds(k.v = 3)` holds at position 0. | Test (TC-385) |
| FR-431-AC-2 | Along the `Pay` failure path, `eventually[0,2] holds(k.bal = 0)` holds at position 2 and fails at position 1, and on the success path it fails at every position from 1. | Test (TC-380, TC-385) |
| FR-431-AC-3 | Each step kind is classified counted or uncounted exactly as listed; a past interval operator measures backwards in counted steps; the terminal stutter step counts. | Test (TC-385) |
| FR-431-AC-4 | An `attempt` position carries its operation's anchor, a `fork`, `join`, `send` or `finish` position carries a `protocol` anchor naming its kind and node, and position 0 carries `initialization`. | Test (TC-385) |
| FR-431-AC-5 | A claim with no interval operator receives the same verdict when the uncounted steps of a behaviour are read as counted. | Test (TC-385) |

## Dependencies

- [FR-091](../temporal/FR-091-evaluate-bounded-future.md) and
  [FR-092](../temporal/FR-092-evaluate-bounded-past.md) own the interval
  operators whose distance this requirement fixes over a protocol subject.
- [FR-426](FR-426-apply-protocol-steps.md) owns the step kinds.

## References

- Design: quire-spec-language ADR-027 PB-1 to PB-3, SE-3 and QS-12.
- Tickets: Linear STD-140; paired QSL-396.
