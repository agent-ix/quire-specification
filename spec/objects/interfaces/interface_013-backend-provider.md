---
id: interface_013
title: "I13 backend capability, request and result"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
---

# [interface_013] I13 backend capability, request and result

## Contract

```yaml
name: BackendProvider
associated_types: [ProviderManifest, CapabilityDisposition, BackendRequest, BackendResult]
operations:
  - { name: negotiate, inputs: [ContractPackage, requested claims, per-item candidate sets, domains, options], output: per-item supported requires-bound unsupported or invalid-request, semantics: completes before artifact emission }
  - { name: execute, inputs: [negotiated request], output: artifacts results and accounting, semantics: preserves all item dispositions and provenance }
invariants:
  - no approximate artifact is emitted for unsupported meaning
  - every requested item has exactly one disposition
  - negotiation settles over the candidate set it receives and never computes or reorders it
```
