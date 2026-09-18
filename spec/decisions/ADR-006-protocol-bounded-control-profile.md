---
id: ADR-006
title: "Bounded control in the protocol profile layer"
type: ADR
status: proposed
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

**Open question for the architect.** Two normative statements bind the same
protocol loop to opposite dispositions, and the profile hierarchy offers no
identity that separates them.

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

## Consequences

Until the question is answered, FR-052-AC-3, FR-049-AC-3, FR-220-AC-4 and
FR-228-AC-2 bind the same loop opposite to FR-171-AC-2, and an implementation
can satisfy one side only. The contradiction is recorded here rather than scoped
away, because scoping it would require naming a profile no accepted hierarchy
supplies, and deleting either side would remove a normative refusal or a
normative admission without a decision.
