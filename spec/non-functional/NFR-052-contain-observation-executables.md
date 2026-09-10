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

The observation implementation and qualification paths SHALL use Rust 1.98.1; Java, JVM, Maven, Eclipse, Electron, and foreign monitoring runtimes are prohibited for binding, replay, incremental assessment, and observation mapping.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Forbidden runtime dependencies in owned paths | 0 | 0 | Dependency inventory |
| First-party executable paths outside Rust 1.98.1 | 0 | 0 | Build/configuration inspection |

## Verification

Inspect owned build, test, adapter, fixture-audit, and CI definitions. Emitting a foreign textual mapping is recorded separately from taking a runtime dependency.
