---
id: TC-043
title: "Evaluate identity and reachability in explicit finite graphs"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-043
    type: verifies
---
## Description

Accepted portable controls for [FR-043](../functional/foundation/FR-043-evaluate-finite-graph-relations.md).
The matrix remains Planned; no executable test or observed coverage is claimed.

## Test Procedure

Construct explicitly finite isolated, self-loop, two-node cycle, branching/duplicate-edge and long-chain environments. Compare pre/post identities and same labels in another universe/type. Independently introduce duplicate object keys, a dangling edge, a snapshot mismatch, unsupported edge shape, missing completeness and insufficient graph-step budget.

## Expected Results

Results distinguish positive-length reachability from reflexive equality, preserve exact graph identity and terminate by finite visited identities. Refusals/incompleteness remain typed; no cutoff, missing node or unsupported representation produces a false unreachable result.
