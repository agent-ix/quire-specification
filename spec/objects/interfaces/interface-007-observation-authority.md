---
id: interface-007
title: "I07 observation authority bundle"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-325
    type: references
---

# [interface-007] I07 observation authority bundle

## Contract

```yaml
name: ObservationAuthority
associated_types: [ObservationId, Revision, Population, Position, ClockProgress, Completeness]
operations:
  - { name: select, inputs: [authority, scope, revision], output: immutable records and authority facts, semantics: preserves correlation population time progress and completeness }
  - { name: supersede, inputs: [prior revision, new revision, relation], output: revisioned authority view, semantics: never infers supersession from ingestion order }
invariants:
  - unknown population or progress remains unknown
  - duplicate and conflicting observations are distinguished
```
