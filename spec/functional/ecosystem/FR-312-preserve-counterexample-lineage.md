---
id: FR-312
title: "Preserve canonical counterexample lineage"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-197
    type: references
  - target: ix://agent-ix/quire-specification/AD-011
    type: references
---

# FR-312: Preserve canonical counterexample lineage

## Description

When a method emits or reduces a counterexample, the evidence boundary SHALL retain its typed canonical inputs, semantic/run identity, derivation, reduction and native replay lineage.

## Inputs

Original and minimized counterexamples, method/provider run, package/domain and
replay results.

## Outputs

Immutable lineage graph and canonical artifacts, or typed validation/refusal.

## Behavior

Each revision links to its predecessor and transformation. A minimized example
must remain domain-valid and reproduce the same scoped failure. Different
methods may relate equivalent examples without merging identities. Redaction or
loss is explicit and prevents claims requiring removed replay material.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-312-AC-1 | Original, shrunk and replayed counterexamples retain ordered immutable lineage and exact typed values. | Test (TC-232) |
| FR-312-AC-2 | A minimization that changes domain validity or verdict is rejected as lineage failure. | Test (TC-232) |
| FR-312-AC-3 | Redacted/lost replay fields remain visible limitations and cannot satisfy replay qualification. | Test (TC-232) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
