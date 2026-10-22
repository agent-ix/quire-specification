---
id: interface_019
title: "I19 independent conformance corpus and result"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-336
    type: references
---

# [interface_019] I19 independent conformance corpus and result

## Features

| Feature | Kind |
| --- | --- |
| execute_corpus | operation |

## Contract

```yaml
name: ConformanceCorpus
associated_types: [InventoryCapabilityId, CorpusVector, ExpectedDisposition, QualificationResult]
operations:
  - { name: execute_corpus, inputs: [corpus, qualification subject], output: per-vector and per-capability results, semantics: covers every inventory capability with positive boundary and refusal vectors }
invariants:
  - every inventory capability has exactly one primary coverage entry
  - missing or unexecuted vectors remain visible gaps
```
