---
id: FR-196
title: "Generate exact Kani contracts and harnesses"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/FR-195
    type: references
  - target: ix://agent-ix/quire-contract-ir/FR-036
    type: references
  - target: ix://agent-ix/quire-contract-codegen/FR-003
    type: references
  - target: ix://agent-ix/quire-contract-codegen/interface-001
    type: references
---

# FR-196: Generate exact Kani contracts and harnesses

## Description

When Kani generation is requested, the backend SHALL derive contract
preconditions, symbolic domains and harness bounds from the checked model and
record the Kani version and options it ran.

## Inputs

Target-neutral items, model domains, requested proof claims, Kani capability
profile and Kani options.

## Outputs

Rust contracts/harnesses with complete accounting, requires-bound/unsupported
dispositions, or generation failure.

## Behavior

Boolean, numeric, state, collection, graph, temporal and protocol items are
accepted only where the selected Kani profile encodes their exact semantics.
Finite bounds come from model domains or explicit proof requests. Unsupported
constructs emit no approximate harness. Generated crates are Rust-only and
`publish = false`.

## Kani mapping contract

Each lowered claim generates one Rust item and one proof harness. Preconditions
encode checked domain membership and definedness; postconditions invoke the
generated oracle over pre/post state and result; frame conditions assert
unchanged non-frame locations; invariants hold at admitted state boundaries.
Integer and collection ranges use model-derived inclusive bounds with checked
conversions. Graph, temporal or protocol state is generated only under a finite
exact encoding whose correspondence is recorded. Harness identity derives from
claim identity rather than source spelling. The evidence names the Kani version
it ran, its target, solver, unwind/loop bounds and command options.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-196-AC-1 | Generated preconditions and symbolic ranges equal the model's inclusive domains, including boundary values. | Test (TC-218) |
| FR-196-AC-2 | Missing bounds or unsupported exact semantics produce requires-bound/unsupported and no harness for that item. | Test (TC-218) |
| FR-196-AC-3 | Evidence accounts for every requested obligation with its disposition and the unwind/loop bounds it was checked under. | Inspection |
| FR-196-AC-4 | Preconditions, postconditions, invariants and frame clauses map separately to the generated contract and harness. | Test (TC-218) |
| FR-196-AC-5 | A finite encoding records its one-to-one semantic correspondence; absent correspondence emits unsupported and no Rust item. | Test (TC-218) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
