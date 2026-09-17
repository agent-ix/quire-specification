---
id: interface_009
title: "I09 protocol assessment"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-327
    type: references
---

# [interface_009] I09 protocol assessment

## Contract

```yaml
name: ProtocolAssessment
associated_types: [ProtocolRequest, ProtocolClaim, ProtocolResult, StrategyWitness, Counterexample]
operations:
  - { name: assess, inputs: [CheckedProtocol, claim, observations or model, premises, limits], output: typed claim result, semantics: dispatches conformance projection realizability composition or refinement without conflation }
invariants:
  - every result retains exact claim premises and lineage
  - no analyzer weakens the graph or drops branches
```
