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

When a native import is linked, the linker shall resolve its domain type names through the selected domain package.

## Inputs

Package identity, exact version, domain package identity and `sha256-jcs` digest, manifest, lock, qualified domain type names and declaration keys.

## Outputs

Unique resolved declaration handles or import diagnostics.

## Behavior

The spec's artifacts declare the domain model. The domain package lifted from them is the only source of domain declarations. Import aliases are lexical names. Package identity uses the owner/name contract; a declaration key (domain package identity, IR node identity, `sha256-jcs`) is separate from it. The linker checks manifest, lock and transitive closure independently of the domain package digest.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-002-AC-1 | An exact import resolves a qualified domain type name to its stable declaration key. | Test |
| FR-002-AC-2 | An absent package receives missing_import. | Test |
| FR-002-AC-3 | Two declarations with the same selected qualified name receive ambiguous_declaration. | Test |
| FR-002-AC-4 | A changed selected dependency receives stale_dependency. | Test |
| FR-002-AC-5 | An undeclared authority receives foreign_reference. | Test |

## Dependencies

- [US-002](../../usecase/US-002-exchange-exact-subjects.md) supplies the user need.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) defines the domain package and its declaration keys.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
