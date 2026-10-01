---
id: TC-410
title: "Abstraction relation declaration syntax and binding forms"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-450
    type: verifies
---

# TC-410: Abstraction relation declaration syntax and binding forms

## Description

Parse and check `abstraction-decl` declarations over the ConfigVersion
domain package, and confirm each binding form yields its FR-353 key and
value, and that the declaration is recognized only under its profile.

## Test Procedure

Use a domain package declaring object type `ConfigVersion` with fields
`version` and `parent`, a population `Configs` of `ConfigVersion`, and an
operation `attemptUpdate(next)` declared by `ConfigVersion`. Compile each
vector's unit through the real local Rust boundary. For an admitted unit,
read the checked relation's bindings with their keys and values; for a
refused unit, read the stage, code, cause and source span.

## Vectors

| Vector | Unit | Expected result |
| --- | --- | --- |
| AD-01 | `abstraction Store using v { type M::ConfigVersion to "store::Config" { version to "ver"; parent to "0"; } population M::Configs to "configs"; operation M::ConfigVersion::attemptUpdate to "store::Config::attempt_update" receiver "self" (next to "next_version"); }` under `quire.model.complete/v1` | Admitted; exactly three bindings: the object key of `ConfigVersion` to `rust_type` `store::Config` with fields `parent` to `0` and `version` to `ver`; the population key of `Configs` to `collection` `configs`; the operation key (`ConfigVersion`, `attemptUpdate`) to `function` `store::Config::attempt_update`, `receiver` `self`, `parameters` `next` to `next_version` |
| AD-02 | `abstraction Store using v { type M::ConfigVersion to "store::Config"; }` | Admitted; one object binding with an empty field map |
| AD-03 | AD-02 in a package that selects only `quire.value.complete/v1` | `unsupported_construct` at the declaration's span |
| AD-04 | `abstraction Store using v { }` | `invalid_syntax` at `}` |
| AD-05 | AD-02 with `to store::Config` unquoted | `invalid_syntax` at `store` |
| AD-06 | AD-01 with each comment and whitespace run changed | Lossless rendering reproduces the source bytes; the bindings equal AD-01's |

## Expected Results

Each admitted vector yields exactly the listed bindings, keyed by the
effective-declaration key or operation key, with the Rust strings as their
byte content. Each refused vector yields exactly its listed refusal at its
span and no checked relation.
