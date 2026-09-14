---
id: TC-052
title: "Preserve bounded causal control"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-052
    type: verifies
---
## Description

Verify sequence, parallel branches, join, bounded iteration and termination as a finite causal graph.

## Test Procedure

Enumerate permitted interleavings of S1 and S2 around their join. Permute
equal-time inputs, then independently remove a join input, add a causal cycle,
remove the iteration maximum and create a zero-progress loop. Mutate each typed
semantic graph member: node/control kind, edge, guard or constraint graph and
dependency, event kind/binding, channel/FIFO rule, await rule, compensation
reference, finish rule, branch, join and maximum. Reuse the old qualified
protocol-definition identity for each mutated graph, then issue the correct new
identity. Independently change only external path/locus/rendering metadata.

## Expected Results

All and only causally valid interleavings admit. Timestamp/input order adds no
edge. Every semantic mutation requires a new qualified definition identity;
reusing the old identity refuses as a contradiction instead of approximating.
External path/locus/rendering metadata does not independently extend the graph
key, while an authored source-byte change still changes the raw-source digest
already carried by the qualified definition.
