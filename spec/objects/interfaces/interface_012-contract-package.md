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
associated_types: [SourceMap, LoweringProfileRef, CompleteLoweringProfileV2]
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
| lowering_profile | CompleteLoweringProfileV2 | 1..1 | explicitly selected resolved profile; owns `reference` and typed `required_source_selections` |
| requested_items | RequestedItem | 0..* | each retains its own disposition |
| limits | caller-supplied limits (FR-195) | 1..1 | bounds lowering work and output |

Returns: ContractPackage and per-item accounting

`CompleteLoweringProfileV2` is the resolved lowering-operation input owned by
Contract IR FR-035. Its `reference: LoweringProfileRef` identifies the explicitly
selected profile; its `required_source_selections: Vec<CheckedSelection>` carries
that profile's authoritative source-selection requirements. Each row reuses
FR-322's `Selection` value `{role, definition}` and `DefinitionRef`
`{authority, identity}` without another representation. The profile producer
supplies the requirements when constructing the resolved profile, together with
its existing supported tags, bounds requirement and work limit; a reference alone
is insufficient input. There is no ambient registry or identifier parsing.

The lowerer SHALL apply FR-330's exact row-membership predicate to those typed
requirements and the admitted checked package's `lock.profile_selections` before
emitting semantic nodes. The output's `package.lowering_profile` remains only
`lowering_profile.reference`; the requirements are operation input, not additional
wire members or an amendment to source meaning.
