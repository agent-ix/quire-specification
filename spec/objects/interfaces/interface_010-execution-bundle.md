---
id: interface_010
title: "I10 composed execution bundle"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-328
    type: references
---

# [interface_010] I10 composed execution bundle

## Features

| Feature | Kind |
| --- | --- |
| compose | operation |

## Contract

```yaml
name: ExecutionBundle
associated_types: [ExecutionRequest, DomainResult, BundleDisposition]
operations:
  - { name: compose, inputs: [state results, temporal results, protocol results], output: canonical ordered bundle, semantics: preserves each domain result and shared prerequisite relation }
invariants:
  - domain truth execution adequacy and provenance axes are not flattened
  - unsupported siblings remain present and independent
```
