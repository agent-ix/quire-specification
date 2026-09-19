---
id: interface_005
title: "I05 native runtime input and result"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-323
    type: references
---

# [interface_005] I05 native runtime input and result

## Contract

```yaml
name: NativeRuntime
associated_types: [RuntimeRequest, StateEnvironment, RuntimeResult, ResourceLimit]
operations:
  - { name: execute, inputs: [CheckedPackage, typed request, explicit inputs, limits], output: ordered per-clause results, semantics: reference native value state and graph evaluation }
  - { name: replay, inputs: [CheckedPackage, canonical counterexample, limits], output: verdict and parity metadata, semantics: uses the same execute semantics }
invariants:
  - invalid unsupported incomplete undefined failed and false remain distinct
  - exhaustion never truncates meaning into a completed value
```
