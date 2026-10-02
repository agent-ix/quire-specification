---
id: ADR-006
title: "Bounded control in the protocol profile layer"
type: ADR
status: accepted
relationships:
  - target: ix://agent-ix/quire-specification/AD-003
    type: depends_on
  - target: ix://agent-ix/quire-specification/AD-007
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-052
    type: depends_on
  - target: ix://agent-ix/quire-specification/FR-171
    type: depends_on
---
# ADR-006: Bounded control in the protocol profile layer

## Status

**Decided, 2026-10-01** (team-leader decision for the protocol
transition-system specification). `quire.protocol.complete/v1` is the one
protocol admission profile. A `repeat` needs only progress, every body cycle
passing through a step node; an
iteration maximum and an invariant with a variant are each optional model
meaning. The Context and Question sections record the contradiction as it
stood; the Decision and Consequences sections record the outcome.

## Context

[AD-003](../assurance/AD-003-native-quire-language-charter.md) fixes one
protocol-layer profile identity, `quire.protocol.complete/v1`, covering
"static/dynamic choreography, compensation, conformance, projection,
realizability, composition and refinement under AD-007". The tree carries no
second `quire.protocol.*` semantic profile; `quire.protocol.assessment/v1` and
the result identities name result and assessment contracts, not an admission
profile.

Four criteria refuse a repetition that carries no authored finite iteration
maximum:
[FR-052-AC-3](../functional/protocol/FR-052-represent-bounded-control.md),
[FR-049-AC-3](../functional/foundation/FR-049-bind-native-choreography-syntax.md),
[FR-220-AC-4](../objects/protocol/FR-220-causal-control-graph.md) and
[FR-228-AC-2](../objects/protocol/FR-228-bounded-control-construct.md). FR-052's
Behavior states the same rule: "Iteration SHALL carry an authored finite maximum
and progress edge. The linker SHALL refuse missing joins, cyclic causal
dependencies, unbounded repetition and termination conditions that cannot be
reached within the declared finite graph."

[FR-171](../functional/protocol/FR-171-evaluate-dynamic-control.md) states that
"Loops require invariant preservation and a well-founded variant for termination
claims", and FR-171-AC-2 admits "a loop with proved invariant/variant" while
returning inconclusive or refused for an unproved termination claim. It names no
finite iteration maximum as a premise.

Both statements apply to a package selecting `quire.protocol.complete/v1`. A
linker that admits an invariant-and-variant loop with no authored maximum
satisfies FR-171-AC-2 and violates the four refusal criteria; a linker that
refuses it satisfies them and violates FR-171-AC-2.

## Question

Does the protocol layer need a bounded-control profile identity distinct from
`quire.protocol.complete/v1`, under which an absent finite iteration maximum
refuses, or does `quire.protocol.complete/v1` admit an invariant-and-variant
loop outright, making the finite iteration maximum one of two alternative
termination premises?

Answering it fixes, in one place: how many `quire.protocol.*` admission
identities the hierarchy carries; which termination premises an admitted
iteration may carry; and whether FR-220 and FR-228 record a finite maximum, an
invariant-and-variant pair, or either.

## Decision

`quire.protocol.complete/v1` is the one protocol admission profile. A
`repeat` carries progress: every cycle in its body graph passes through a step
node.

- An authored iteration maximum is optional model meaning: the loop runs at
  most that many iterations, and its `exhausted` control runs at the maximum.
- An authored invariant and variant are optional model meaning. Only a
  termination claim reads them (FR-171-AC-2); admission does not require them
  to be proved.
- A loop with neither runs for as long as its guard admits. An explorer keeps
  no iteration counter for it and closes its cycles through the state key.
- Only a zero-progress loop refuses.

## Consequences

- FR-052-AC-3, FR-049-AC-3, FR-220-AC-4 and FR-228-AC-2 refuse only missing
  join inputs, causal cycles and zero-progress loops, and admit a repeat with
  or without a maximum and with or without an invariant and variant. FR-228-AC-5
  admits an authored invariant and variant without a proof. None of them
  contradicts FR-171-AC-2 any longer.
- The shared grammar's `repetition` production makes the maximum and the
  invariant and variant each optional; only a repetition with a maximum has an
  `exhausted` control.
- A protocol may model a loop meant to run for ever, such as a spinning reader
  or a store loop, and its liveness is decided over lassos. Such a loop's
  termination claim settles inconclusive or refused by FR-171-AC-2.
- A bound is either model meaning or a caller-set resource limit, never a
  fixed cap the language imposes.

## References

- Raised from the protocol transition-system design: quire-spec-language
  ADR-027 PS-6; Linear STD-140, paired QSL-396.
