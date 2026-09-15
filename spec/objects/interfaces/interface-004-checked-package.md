---
id: interface-004
title: "I04 immutable checked package"
type: interface
---

# [interface-004] I04 immutable checked package

## Contract

```yaml
name: CheckedPackage
associated_types: [PackageId, SemanticGraph, PackageLock, FeatureReport, SourceMap]
operations:
  - { name: link_and_check, inputs: [ParsedSource, ModelSelection, profile bundle], output: checked package or located refusals, semantics: resolves exact definitions types dependencies and capabilities }
  - { name: inspect, inputs: [checked package], output: immutable declarations capabilities and diagnostics, semantics: does not execute or lower }
invariants:
  - runtime inputs do not participate in static package identity
  - backend support does not alter admission
```
