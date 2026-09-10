---
id: US-008
title: "Assess bounded temporal obligations without losing time meaning"
type: US
relationships:
  - target: ix://agent-ix/quire-specification/StR-002
    type: traces_to
  - target: ix://agent-ix/quire-specification/FR-090
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-091
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-092
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-093
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-094
    type: exercises
  - target: ix://agent-ix/quire-specification/FR-095
    type: exercises
---
## Story

**As a** specification author
**I want** bounded temporal obligations to retain their selected clock, trigger, capture, history, progress and closure meaning
**So that** an assessment or generated TL artifact cannot silently answer a different temporal question.

## Context

An order can fail after a payment effect and activate a refund obligation that
captures that effect and amount. The deadline can count later business events,
fixed samples, or elapsed event time; those interpretations are not
interchangeable. A silent interval, a closed execution and an incomplete file
also carry different evidence.

## Acceptance Examples (Illustrative)

### US-008-EX-1: Captured refund deadline

Given a failed fulfillment after a charge, the author can assess whether the
matching refund occurred within an inclusive bounded interval using the clock
selected by the clause and the amount captured at activation.

### US-008-EX-2: Silence advances only the selected clocks

Given no later business event, authoritative sample or event-time progress can
settle a due deadline, while an event-position deadline remains open until a
position arrives or the execution closes.

### US-008-EX-3: Missing history is not false

Given a past-time clause whose required earlier observations were not supplied,
the author sees incomplete history rather than a Boolean result inferred from
the missing prefix.

### US-008-EX-4: A bridge preserves or refuses

Given a native temporal clause, a generated TL subject either names an exact
reviewed correspondence or reports the unsupported clock/operator/boundary
dimension; it never changes the native profile to fit the available backend.

## Dependencies (Contextual)

- The composed native language and profile identity contract in
  [FR-020](../functional/FR-020-compose-native-profiles.md).
- Observation bindings and completeness/progress assertions supplied by Agent
  F's independently owned contract.
- The [typed predicate projection ticket](https://github.com/agent-ix/quire-contract-ir/issues/63).
