---
id: interface-003
title: "I03 authoritative model selection"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-321
    type: references
---

# [interface-003] I03 authoritative model selection

## Contract

```yaml
name: ModelSelection
associated_types: [ProducerInterfaceId, ModelExportId, OriginalDeclarationId, EffectiveDeclarationId]
operations:
  - { name: select_export, inputs: [producer authority, interface version, revision, digest], output: immutable typed export or refusal, semantics: validates exact producer bytes and closure }
  - { name: normalize, inputs: [export, selected quire.model.complete/v1 definition], output: original-to-effective correspondence (quire.model.effective-view/v1) or refusal, semantics: producer supplies originals and edges; QSL derives effective identities in its own domains and preserves ordered derivation provenance }
invariants:
  - language consumers never mint, rename or merge producer identities; effective identities are QSL-derived in QSL digest domains
  - missing closure is explicit
```
