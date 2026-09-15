---
id: interface-003
title: "I03 authoritative model selection"
type: interface
---

# [interface-003] I03 authoritative model selection

## Contract

```yaml
name: ModelSelection
associated_types: [ProducerInterfaceId, ModelExportId, OriginalDeclarationId, EffectiveDeclarationId]
operations:
  - { name: select_export, inputs: [producer authority, interface version, revision, digest], output: immutable typed export or refusal, semantics: validates exact producer bytes and closure }
  - { name: normalize, inputs: [export, semantic profile], output: original-to-effective correspondence, semantics: preserves ordered derivation provenance }
invariants:
  - language consumers never mint producer identities
  - missing closure is explicit
```
