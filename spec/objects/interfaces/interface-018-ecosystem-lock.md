---
id: interface-018
title: "I18 exact ecosystem lock"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-335
    type: references
---

# [interface-018] I18 exact ecosystem lock

## Contract

```yaml
name: EcosystemLock
associated_types: [RepositoryRevision, PackageLock, SemanticDefinition, ProviderPin, ToolchainPin, OptionSet]
operations:
  - { name: resolve_lock, inputs: [declared ecosystem selection], output: canonical immutable lock or conflict refusal, semantics: resolves every repository package profile provider tool build and deployment selection }
invariants:
  - absent pins are not ambient defaults
  - changing a meaning-affecting pin creates a distinct run identity
```
