---
id: NFR-061
title: "Contain output-mapping dependencies"
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
with `publish = false`; an external parser/tool observer SHALL be optional and
downstream, the mapping result SHALL NOT depend on it, and an unavailable or
ambiguous observer SHALL refuse that observation without changing the pure Rust
mapping result.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Mandatory foreign-runtime mapping dependencies | 0 | 0 | Cargo.toml review |
| Observer results used as preservation/native truth | 0 | 0 | adverse testing |

## Verification

Confirm the mapping crates' Cargo.toml declares no mandatory foreign-runtime
dependency; remove the optional observer and reproduce identical mapping
packages; inject an unavailable/ambiguous observer and verify a typed observation
refusal with the package unchanged.

## Status

Accepted by FS06. QSpec references upstream material but copies no grammar,
runtime, library or corpus.
