---
id: interface-001
title: "I01 exact source document"
type: interface
---

# [interface-001] I01 exact source document

## Contract

```yaml
name: SourceDocument
associated_types: [SourceUnitId, SourceRevision, SourceDigest, SourceRegion]
operations:
  - { name: read, inputs: [unit identity, revision, byte limit], output: exact bytes or typed non-result, semantics: bytes and revision are immutable and digest-bound }
invariants:
  - every region indexes the returned original bytes
  - no newline or Unicode normalization occurs
```
