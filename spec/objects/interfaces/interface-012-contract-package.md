---
id: interface-012
title: "I12 target-neutral contract package"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: references
---

# [interface-012] I12 target-neutral contract package

## Contract

```yaml
name: ContractPackage
associated_types: [ContractPackageId, TypeGraph, ExpressionGraph, ClauseGraph, ProtocolReference, SourceMap]
operations:
  - { name: lower, inputs: [CheckedPackage, lowering profile, requested items], output: cycle-free contract package and accounting, semantics: exact representation or per-item refusal }
invariants:
  - the representation is not authored or semantic authority
  - unsupported meaning produces no substitute node
```
