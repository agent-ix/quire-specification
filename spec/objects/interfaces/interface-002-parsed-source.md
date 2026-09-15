---
id: interface-002
title: "I02 lossless parsed source"
type: interface
---

# [interface-002] I02 lossless parsed source

## Contract

```yaml
name: ParsedSource
associated_types: [LosslessCst, Token, Recovery, Diagnostic]
operations:
  - { name: parse, inputs: [SourceDocument, edition, limits], output: CST and diagnostics or fatal non-result, semantics: retains all bytes tokens trivia spans and recoveries }
  - { name: apply_edit, inputs: [CST, source edit], output: new SourceDocument and CST, semantics: creates a new revision and preserves only mapped unchanged-node identity }
invariants:
  - an unrecovered CST serializes to exact input bytes
  - a CST is not a checked semantic package
```
