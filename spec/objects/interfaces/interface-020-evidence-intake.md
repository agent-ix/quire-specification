---
id: interface-020
title: "I20 immutable evidence intake"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-337
    type: references
---

# [interface-020] I20 immutable evidence intake

## Contract

```yaml
name: EvidenceIntake
associated_types: [EvidenceId, SubjectLink, ResultReference, ArtifactReference, RightsMetadata]
operations:
  - { name: ingest, inputs: [validated result references, artifacts, traceability, rights], output: immutable evidence record or refusal, semantics: retains producer bytes and typed relationships without executing producers }
invariants:
  - evidence identity never substitutes for semantic or run identity
  - lost or redacted replay material remains an explicit limitation
```
