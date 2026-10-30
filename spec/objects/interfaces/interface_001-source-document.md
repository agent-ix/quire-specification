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

## Features

| Feature | Kind |
| --- | --- |
| read | operation |

## Contract

```yaml
name: SourceDocument
associated_types: [SourceUnitId, SourceRegion]
operations:
  - { name: read, inputs: [unit identity, byte limit], output: exact bytes or typed non-result, semantics: bytes are immutable }
invariants:
  - every region indexes the returned original bytes
  - no newline or Unicode normalization occurs
```
