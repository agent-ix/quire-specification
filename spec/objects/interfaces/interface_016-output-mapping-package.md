---
id: interface_016
title: "I16 output mapping package"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-299
    type: references
---

# [interface_016] I16 output mapping package

## Features

| Feature | Kind |
| --- | --- |
| map | operation |

## Contract

```yaml
name: OutputMappingPackage
associated_types: [TargetProfile, GeneratedBytes, ObligationMappingRecord, LossRecord]
operations:
  - { name: map, inputs: [CheckedPackage, obligation set, target profile, limits], output: immutable target package and complete records, semantics: preserved conditional unrepresented or refused per obligation }
invariants:
  - generated text is never native source authority
  - parser acceptance is not semantic equivalence
```
