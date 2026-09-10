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

Enumerate permitted interleavings of S1 and S2 around their join. Permute equal-time inputs, then independently remove a join input, add a causal cycle, remove the iteration maximum and create a zero-progress loop.

## Expected Results

All and only causally valid interleavings admit. Timestamp/input order adds no edge. Every structural mutant refuses, and a semantic graph or bound change changes protocol identity.

