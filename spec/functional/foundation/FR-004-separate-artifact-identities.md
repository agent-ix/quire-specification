---
id: FR-004
title: "Preserve independent artifact identities"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-002"
    type: traces_to
---
# FR-004: Preserve independent artifact identities

## Description

When a reference is bound, the consumer shall preserve its artifact kind, authority, identity and content binding.

## Inputs

Source, model, property, frame, snapshot, invocation, run, environment and generated-artifact references.

## Outputs

Exact role-specific bindings or identity-conflict diagnostics.

## Behavior

The immutable key is kind, authority and identity. A path or display label is a locator. Reusing the same key with a changed wire is a conflict. Runtime snapshots, invocations and runs do not adopt authored requirement identity. The earlier operation fixtures retain their established draft encoding.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-004-AC-1 | Changed content under an existing immutable key receives identity_content_conflict. | Test |
| FR-004-AC-2 | A snapshot reference cannot substitute for an invocation reference. | Test |
| FR-004-AC-3 | Each independent operation case selects its own invocation identity. | Test |
| FR-004-AC-4 | A frame change invalidates the previous invocation byte binding. | Test |

## Dependencies

- [US-002](../../usecase/US-002-exchange-exact-subjects.md) supplies the user need.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) supplies the model and state context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
