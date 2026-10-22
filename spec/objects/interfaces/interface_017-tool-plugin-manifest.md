---
id: interface_017
title: "I17 tool and plugin manifest"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-334
    type: references
---

# [interface_017] I17 tool and plugin manifest

## Features

| Feature | Kind |
| --- | --- |
| validate_manifest | operation |

## Contract

```yaml
name: ToolPluginManifest
associated_types: [ToolId, CapabilitySet, EffectSet, RightsSet]
operations:
  - { name: validate_manifest, inputs: [canonical manifest, host policy], output: admitted provider selection or refusal, semantics: checks capabilities purity effects rights and platform }
invariants:
  - discovery is explicit
  - manifests cannot grant semantic admission or authority
```
