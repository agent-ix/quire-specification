---
id: TC-052
title: "Preserve bounded causal control"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-052
    type: verifies
---
## Description

Verify sequence, parallel branches, join, iteration and termination as a finite causal graph.

## Test Procedure

Enumerate permitted interleavings of S1 and S2 around their join. Permute
equal-time inputs, then independently remove a join input, add a causal cycle,
create a zero-progress loop, both as an empty repeat body and as a repeat whose
body is a `choice` with one case holding an event and another case holding an
empty sequence. Admit a repeat whose body holds a step with a
maximum only, an invariant and variant only, both, and neither. Mutate each typed
semantic graph member: node/control kind, edge, guard or constraint graph and
dependency, event kind/binding, channel/FIFO rule, await rule, compensation
reference, finish rule, branch, join and maximum. Reuse the old qualified
protocol-definition identity for each mutated graph, then issue the correct new
identity. Independently change only external path/locus/rendering metadata.

## Vectors

| Vector | Repeat body | Expected result |
| --- | --- | --- |
| ZP-01 | `repeat R by w visible() while { true } choice C by w visible() { case go when { true } event E by w as (e: M::Tick) { true }; case idle when { false } sequence S { } }` | refuses `unsupported_construct`/`declaration-form` at `R`, naming the repeat node and the selected protocol profile: the cycle through case `idle` passes through no step node, though the body holds a step |
| ZP-02 | the same repeat with case `idle` holding `event I by w as (i: M::Tick) { true };` | admitted: every cycle passes through a step node |
| ZP-03 | `repeat P by w visible() while { true } await W after Start using p clock "c" within [0, 5] match receive R via h of Start … then sequence T { } timeout sequence U { }` | admitted: the cycle through `timeout` passes through the `await`, whose `timeout` traversal is a step |

## Expected Results

All and only causally valid interleavings admit. Timestamp/input order adds no
edge. Every semantic mutation requires a new qualified definition identity;
reusing the old identity refuses as a contradiction instead of approximating.
External path/locus/rendering metadata does not independently extend the graph
key, while an authored source-byte change still changes the raw-source digest
already carried by the qualified definition.
