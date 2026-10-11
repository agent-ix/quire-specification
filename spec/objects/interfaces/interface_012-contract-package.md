---
id: interface_012
title: "I12 target-neutral contract package"
type: interface
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: references
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-019
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-035
    type: references
---

# [interface_012] I12 target-neutral contract package

## Features

| Feature | Kind |
| --- | --- |
| lower | operation |

## Contract

```yaml
name: ContractPackage
associated_types: [ContractPackageId, SourceMap, LoweringProfileRef]
operations:
  - { name: lower, inputs: [checked_package, lowering_profile, requested_items, limits], output: ContractPackage and per-item accounting, semantics: exact representation or per-item refusal }
invariants:
  - the representation is not authored or semantic authority
  - unsupported meaning produces no substitute node
  - one key-ordered semantic graph is the sole owner of semantic nodes
  - semantic node references use stable node identity, never array position or display name
  - dependency references retain FR-322 dependency-selection semantics and are not rewritten as local nodes
```

## Operations

### lower

Lower a checked package for the selected profile and requested items, preserving
exact semantic meaning or a typed per-item refusal.

| Param | Type | Multiplicity | Constraints |
|---|---|---|---|
| checked_package | CheckedPackage | 1..1 | admitted immutable package |
| lowering_profile | LoweringProfileRef | 1..1 | explicitly selected; closed `{authority, identity}` value |
| requested_items | RequestedItem | 0..* | each retains its own disposition |
| limits | caller-supplied limits (FR-195) | 1..1 | bounds lowering work and output |

Returns: ContractPackage and per-item accounting

`LoweringProfileRef` has the same closed `{authority, identity}` value form as
FR-322 `DefinitionRef`. It identifies the caller's explicit lowering-profile
choice. I12 defines no profile registry and no mapping from this reference to
required source semantic selections; the reference is carried as identity, not
used to infer a source-selection compatibility result.
