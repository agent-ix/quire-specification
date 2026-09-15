---
id: interface-016
title: "I16 output mapping package"
type: interface
---

# [interface-016] I16 output mapping package

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
