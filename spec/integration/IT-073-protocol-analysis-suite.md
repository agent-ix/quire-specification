---
id: IT-073
title: "Protocol analyzers share one checked graph"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-174
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-177
    type: verifies
---

# IT-073: Protocol analyzers share one checked graph

## Objective

Verify that conformance, projection, realizability, assume-guarantee and
refinement consume the same real checked Protocol graph and keep separate claims.

## Target Integration

QSL emits I08 through its Rust API; `quire-protocol` analyzers consume it
and emit I09 results locally.

## Preconditions

Dynamic-role, queue, choice-evidence and loop contracts are enabled by exact
profiles and explicit finite analysis bounds.

## Inputs

Projectable/realizable and hidden-choice, incompatible-composition, extra-trace,
deadlock, bound-exhaustion and unsupported-construct graphs.

## Test Procedure

1. Check each source into one immutable graph.
   - IT-073-SC-01: every analyzer receives the identical graph identity.
2. Execute every requested claim.
   - IT-073-SC-02: each returns its own premises and witness/counterexample.
3. Mutate a visibility/queue/bound premise.
   - IT-073-SC-03: only dependent claims change and none drops graph behavior.

## Expected Results

Analyzer results remain claim-specific, source-correspondent and honest about
unsupported or inconclusive scope.
