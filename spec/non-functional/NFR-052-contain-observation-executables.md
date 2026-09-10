---
id: NFR-052
title: "Contain observation executable dependencies"
type: NFR
quality_attribute: maintainability
relationships:
  - target: ix://agent-ix/quire-specification/FR-116
    type: constrains
---
## Statement

The observation implementation and qualification paths SHALL be Rust 1.98.1 and SHALL NOT require Java, JVM, Maven, Eclipse, Electron, or a foreign monitoring runtime to bind, replay, incrementally assess, or map observations.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Forbidden runtime dependencies in owned paths | 0 | 0 | Dependency inventory |
| First-party executable paths outside Rust 1.98.1 | 0 | 0 | Build/configuration inspection |

## Verification

Inspect owned build, test, adapter, fixture-audit, and CI definitions. Emitting a foreign textual mapping is recorded separately from taking a runtime dependency.

