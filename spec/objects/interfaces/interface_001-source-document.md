---
id: interface_001
title: "I01 exact source document"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-202
    type: references
  - target: ix://agent-ix/quire-specification/FR-203
    type: references
---

# [interface_001] I01 exact source document

## Contract

```yaml
name: SourceDocument
associated_types: [SourceUnitId, SourceDigest, SourceRegion]
operations:
  - { name: read, inputs: [unit identity, byte limit], output: exact bytes or typed non-result, semantics: bytes are immutable and digest-bound }
invariants:
  - every region indexes the returned original bytes
  - no newline or Unicode normalization occurs
```
