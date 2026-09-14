---
id: NFR-061
title: "Contain output-mapping dependencies and rights"
type: NFR
quality_attribute: maintainability
relationships:
  - target: ix://agent-ix/quire-specification/FR-122
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-123
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-124
    type: constrains
---
## Statement

The first-party output-mapping production and qualification paths SHALL use Rust
1.98.1 with `publish = false`; an external parser/tool SHALL be optional,
downstream and selected by exact version/invocation/dependency/license identity,
and unavailable or ambiguous rights SHALL refuse that observation without
weakening the pure Rust mapping result.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| New first-party executable paths outside Rust 1.98.1 | 0 | 0 | inspection |
| Mandatory foreign-runtime mapping dependencies | 0 | 0 | dependency audit |
| Adopted external artifacts without exact rights/version record | 0 | 0 | rights inventory |
| Observer results used as preservation/native truth | 0 | 0 | adverse testing |

## Verification

Inspect build, test, adapter, fixture-audit and CI paths; inventory every adopted
artifact and transitive dependency; remove the optional observer and reproduce
identical mapping packages; inject unavailable/ambiguous observer rights and
verify a typed observation refusal with the package unchanged.

## Status

Accepted by FS06. QSpec references upstream material but copies no grammar,
runtime, library or corpus.
