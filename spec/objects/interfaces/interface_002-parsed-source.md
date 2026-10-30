---
id: interface_002
title: "I02 lossless parsed source"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-320
    type: references
---

# [interface_002] I02 lossless parsed source

## Features

| Feature | Kind |
| --- | --- |
| parse | operation |
| apply_edit | operation |

## Contract

```yaml
name: ParsedSource
associated_types: [LosslessCst, Token, Recovery, Diagnostic]
operations:
  - { name: parse, inputs: [SourceDocument, edition, limits], output: CST and diagnostics or fatal non-result, semantics: retains all bytes tokens trivia spans and recoveries }
  - { name: apply_edit, inputs: [CST, source edit], output: new SourceDocument and CST, semantics: creates a new source document and preserves only mapped unchanged-node identity }
invariants:
  - an unrecovered CST serializes to exact input bytes
  - a CST is not a checked semantic package
```
