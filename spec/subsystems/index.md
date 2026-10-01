---
id: complete-v1-subsystem-index
title: "Complete V1 subsystem specification index"
type: index
---

# Complete V1 subsystem specification index

This index is navigation, not a second source of requirements. The complete V1
scope is the atomic inventory in
[`docs/v1-capability-inventory.md`](../../docs/v1-capability-inventory.md).
Every capability remains required; dependency order does not defer scope.

## Contents

| Subsystem | Architecture | Normative families | Primary implementation owner |
| --- | --- | --- | --- |
| S1 language, packages and evolution | AD-001, AD-003 | `functional/foundation`, `functional/source-package` | `quire-spec-language` |
| S2 values, types, expressions and collections | AD-005 | `functional/type-model`, `functional/expressions` | `quire-spec-language`, `quire-contract-model` |
| S3 model, graph and state contracts | AD-006 | `functional/type-model`, `objects/interfaces` | `quire-spec-language` QSL model intake; `quire-rs` semantic extraction and FCD `extraction-frontend` |
| S4 temporal and observation | AD-002 | `functional/temporal`, `functional/observation` | QSL, `tl-*`, `quire-observation` |
| S5 protocol and choreography | AD-007 | `functional/protocol`, `objects/protocol` | QSL, `quire-protocol` |
| S6 composed execution, monitoring and workflow | AD-008 | `functional/runtime`, `functional/workflow` | `quire-integration` plus domain evaluators |
| S7 verification methods and plans | AD-009 | `functional/analysis` | `quire-verification`, method providers |
| S8 semantic IR and exact backends | AD-010 | `functional/ir-proof` | Contract IR and codegen repositories |
| S9 mappings, authoring and tooling | AD-004 | `functional/output-mapping`, `functional/tooling` | QSL and Rust tools/mappers |
| S10 integration, evidence and qualification | AD-011 | `functional/ecosystem`, `integration` | `quire-integration`, `quire-verification`, Quoin intake |

The normative architecture descriptions are in `spec/assurance`. Existing
accepted requirements remain in place; new files are grouped by subsystem so
the expansion does not extend the historical flat foundation directory.
