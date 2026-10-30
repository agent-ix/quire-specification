---
id: FR-312
title: "Preserve canonical counterexample lineage"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
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

## Revision and equivalence contract

Each counterexample revision has a revision identity, a parent revision identity,
transformation identity/options and replay result. A transformation may shrink,
normalize, translate or redact; it cannot overwrite its input. Two examples are
equivalent only relative to a named claim when both are domain-valid and replay
to the same scoped failure under an explicit value/execution correspondence.
Redaction lists removed fields and affected claims; if replay-required material
is removed, the revision is retained as non-replayable evidence rather than a
counterexample substitute.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-312-AC-1 | Original, shrunk and replayed counterexamples retain ordered immutable lineage and exact typed values. | Test (TC-232) |
| FR-312-AC-2 | A minimization that changes domain validity or verdict is rejected as lineage failure. | Test (TC-232) |
| FR-312-AC-3 | Redacted/lost replay fields remain visible limitations and cannot satisfy replay qualification. | Test (TC-232) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
