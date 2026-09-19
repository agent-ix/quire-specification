---
id: FR-353
title: "Declare a model-to-implementation abstraction relation"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-350
    type: references
---

# FR-353: Declare a model-to-implementation abstraction relation

## Description

When a proof backend emits onto or verifies implementation code, the checked
package SHALL carry an explicit abstraction relation from model objects,
populations and operation frames to implementation state, and emission onto
implementation code that references a model element with no bound abstraction
relation SHALL refuse, naming the element.

## Inputs

Checked model objects, populations and FR-052-style operation frames; the
implementation Rust state (types, fields, functions) an abstraction relation
binds them to; a requested Kani or Verus emission or verification target.

## Outputs

A resolved abstraction binding usable by FR-196/FR-350 generation, or a
located refusal naming the unbound model element.

## Behavior

The abstraction relation is authored, never inferred: a model object maps to
exactly one implementation state representation, a population maps to exactly
one implementation collection representation, and an operation frame maps to
exactly one implementation function's parameters, result and framed state.
The relation is consumed by the same target-neutral negotiation path AD-010
defines for Kani (FR-196) and Verus (FR-350) generation; a claim referencing a
model element carries the element's bound implementation representation
forward into the generated contract. Emission onto implementation code that
references a model object, population or operation frame with no bound
abstraction relation refuses before generation, naming the unbound element and
its owning model identity; no partial or approximated relation is emitted in
its place. Rebinding an element to a different implementation representation
is an explicit new relation revision; it does not silently reinterpret an
already-generated contract.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-353-AC-1 | An authored abstraction relation maps each model object, population and operation frame to exactly one implementation state representation. | Test (TC-268) |
| FR-353-AC-2 | Kani and Verus generation consume the bound abstraction relation through the same AD-010 negotiation path used for their other capability advertisements. | Test (TC-268) |
| FR-353-AC-3 | Emission onto implementation code referencing a model element with no bound abstraction relation refuses before generation, naming the unbound element and its owning model identity. | Test (TC-268) |
| FR-353-AC-4 | No partial or approximated abstraction relation is emitted for an unbound element; the refusal is exact and located. | Test (TC-268) |
| FR-353-AC-5 | Rebinding a model element to a different implementation representation creates an explicit new relation revision rather than reinterpreting an already-generated contract. | Test (TC-268) |

## Dependencies

- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  per-item provider capability negotiation this requirement's binding is
  consumed through.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) defines the
  model objects, populations and state contracts this relation maps from.
- [FR-196](FR-196-generate-kani-contracts.md) and
  [FR-350](FR-350-emit-verus-contracts.md) are the two generation paths that
  consume this relation and refuse in its absence.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
