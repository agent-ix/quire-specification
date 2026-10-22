---
id: interface_006
title: "I06 native temporal artifact and result"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-324
    type: references
---

# [interface_006] I06 native temporal artifact and result

## Features

| Feature | Kind |
| --- | --- |
| evaluate | operation |
| repair | operation |

## Contract

```yaml
name: NativeTemporal
associated_types: [TemporalArtifact, ClockBinding, HistoryBoundary, PartialValuation, TemporalResult]
operations:
  - { name: evaluate, inputs: [artifact, observation authority, progress, limits], output: multidimensional temporal result, semantics: bounded or infinite-trace profile meaning }
  - { name: repair, inputs: [prior result, observation revision], output: superseded and replacement results, semantics: recomputes affected suffix or window }
invariants:
  - finite evidence cannot prove unresolved infinite satisfaction
  - missing or uncertain inputs are never coerced to false
```
