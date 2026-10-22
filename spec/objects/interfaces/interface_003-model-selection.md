---
id: interface_003
title: "I03 authoritative model selection"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-321
    type: references
---

# [interface_003] I03 authoritative model selection

## Features

| Feature | Kind |
| --- | --- |
| select_package | operation |
| intake | operation |
| normalize | operation |

## Contract

```yaml
name: ModelSelection
associated_types: [DomainPackageId, DeclarationKey, EffectiveDeclarationId]
operations:
  - { name: select_package, inputs: [domain package identity, sha256-jcs digest], output: immutable admitted domain package or refusal, semantics: validates the exact package bytes against the sha256-jcs digest }
  - { name: intake, inputs: [admitted domain package, selected quire.model.complete/v1 definition], output: original declarations keyed by (package identity, IR node identity, sha256-jcs) or located refusal, semantics: builds one declaration per IR node and refuses retaining the IR node identity and its FCD `origin` }
  - { name: normalize, inputs: [original declarations, selected quire.model.complete/v1 definition], output: original-to-effective view (quire.model.effective-view/v2) or refusal, semantics: QSL derives effective identities in its own domains and preserves ordered derivation provenance }
invariants:
  - declaration keys come only from the domain package identity and IR node identities; effective identities are QSL-derived in QSL digest domains
  - a missing or dangling declaration reference is an explicit refusal
```
