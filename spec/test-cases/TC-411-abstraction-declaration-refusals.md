---
id: TC-411
title: "Abstraction relation declaration refusals"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-450
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-353
    type: verifies
---

# TC-411: Abstraction relation declaration refusals

## Description

Mutate TC-410's AD-01 declaration and confirm each inadmissible binding
refuses with its catalogued code and cause, naming its key, entry or Rust
segment.

## Test Procedure

Use TC-410's domain package, extended with object type `Sub` that inherits
`ConfigVersion` and declares no operation, and a value type `Label`. Compile
each vector through the real local Rust boundary and read every refusal's
code, cause, named key or entry and source span.

## Vectors

| Vector | Mutation of AD-01 | Expected refusal |
| --- | --- | --- |
| AR-01 | `type M::Missing` | `missing_declaration`/`missing-name`, naming `M::Missing` and the domain package identity |
| AR-02 | `population M::Nowhere` | `missing_declaration`/`missing-name`, naming `M::Nowhere` |
| AR-03 | `operation M::Sub::attemptUpdate` | `missing_declaration`/`missing-name`, naming the key (`Sub`, `attemptUpdate`); AD-01's own key (`ConfigVersion`, `attemptUpdate`) checks |
| AR-04 | `type M::Label` | `invalid_model_binding`/`malformed-declaration` |
| AR-05 | Rust path `"type"`, `"9lives"`, `"a::"`, `"r#crate"`, `"a-b"` and an identifier in NFD form, one per run | `invalid_model_binding`/`malformed-declaration`, naming the first malformed segment and its source span |
| AR-06 | Field index `"01"` | `invalid_model_binding`/`malformed-declaration`, naming `01` and its span |
| AR-07 | Rust path `"r#type"`, path `"config_store::Store"`, field index `"0"`, receiver `"self"` | Admitted |
| AR-08 | Field binding `missing to "x"` | `invalid_model_binding`/`malformed-declaration`, naming the entry |
| AR-09 | `version` bound twice in one object binding | `invalid_model_binding`/`conflicting-binding`, naming both field bindings and `version` |
| AR-10 | `Sub` bound with a field inherited from `ConfigVersion` | Admitted |
| AR-11 | Operation binding with `()`; with `(next to "a", other to "b")`; with a parameter mapped twice; with `next` mapped to `"self"` | Each `invalid_model_binding`/`malformed-declaration`, naming the entry |
| AR-12 | The object binding repeated unchanged in the same declaration | `invalid_model_binding`/`conflicting-binding`, naming both bindings and the key |
| AR-13 | A second declaration binding `ConfigVersion` to `"other::Config"` | `invalid_model_binding`/`conflicting-binding`, naming both bindings and the key |

## Expected Results

Each vector reports exactly its listed refusal and no checked relation, or
is admitted as listed. Every refusal names its key, entry or segment, and a
segment refusal carries the segment's source span.
