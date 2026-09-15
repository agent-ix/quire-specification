---
id: interface-006
title: "I06 native temporal artifact and result"
type: interface
---

# [interface-006] I06 native temporal artifact and result

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
