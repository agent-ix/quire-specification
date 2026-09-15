---
id: interface-013
title: "I13 backend capability, request and result"
type: interface
---

# [interface-013] I13 backend capability, request and result

## Contract

```yaml
name: BackendProvider
associated_types: [ProviderManifest, CapabilityDisposition, BackendRequest, BackendResult]
operations:
  - { name: negotiate, inputs: [ContractPackage, requested claims, domains, options], output: per-item supported requires-bound unsupported or invalid-request, semantics: completes before artifact emission }
  - { name: execute, inputs: [negotiated request, exact tool lock], output: artifacts results and accounting, semantics: preserves all item dispositions and provenance }
invariants:
  - no approximate artifact is emitted for unsupported meaning
  - every requested item has exactly one disposition
```
