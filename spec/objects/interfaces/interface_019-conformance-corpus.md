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
  - name: execute_corpus
    inputs: [corpus, qualification subject]
    output: per-vector and per-capability results
    semantics: covers every inventory capability with positive boundary and refusal vectors
    corpus_wire: quire.conformance-corpus/v1
    capability_entries: coverage.primary
    vector_members: [id, capability_id, class, subject, applicability, input, expected]
    result_members: [subject, run_id, vector_id, outcome, artifacts]
invariants:
  - every inventory ID appears exactly once in coverage.primary
  - each vector names a primary capability and an applicable subject consumer
  - missing or unexecuted vectors remain visible gaps
  - execution does not rewrite the corpus results or coverage members
```
