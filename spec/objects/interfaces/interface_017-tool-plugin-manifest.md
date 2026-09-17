---
id: interface_017
title: "I17 tool and plugin manifest"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-334
    type: references
---

# [interface_017] I17 tool and plugin manifest

## Contract

```yaml
name: ToolPluginManifest
associated_types: [ToolId, ContractVersion, CapabilitySet, EffectSet, RightsSet, ConformanceId]
operations:
  - { name: validate_manifest, inputs: [canonical manifest, host policy], output: admitted provider selection or refusal, semantics: checks version capabilities purity effects rights platform provenance and conformance }
invariants:
  - discovery is explicit and versioned
  - manifests cannot grant semantic admission or authority
```
