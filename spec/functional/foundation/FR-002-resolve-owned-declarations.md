---
id: FR-002
title: "Resolve domain declarations through one authority"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-002"
    type: traces_to
---
# FR-002: Resolve domain declarations through one authority

## Description

When a native import is linked, the linker shall resolve its public exports through the selected shared model authority.

## Inputs

Package identity, exact version, compiled-IR byte digest, manifest, lock, export names and declaration handles.

## Outputs

Unique resolved declaration handles or import diagnostics.

## Behavior

Existing Filament contracts and the existing typed-model adapter retain domain authority. Import aliases are lexical names. Package identity uses the existing owner/name contract; semantic declaration identity is separate. The linker checks manifest, lock and transitive closure independently of the compiled-IR digest.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-002-AC-1 | An exact import resolves the exported stable type identity. | Test |
| FR-002-AC-2 | An absent package receives missing_import. | Test |
| FR-002-AC-3 | Two exported identities with the same selected name receive ambiguous_declaration. | Test |
| FR-002-AC-4 | A changed selected dependency receives stale_dependency. | Test |
| FR-002-AC-5 | An undeclared authority receives foreign_reference. | Test |

## Dependencies

- [US-002](../../usecase/US-002-exchange-exact-subjects.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/model-contract.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
