---
id: TC-268
title: "Model-to-implementation abstraction relation binding and refusal"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-353
    type: verifies
---

# TC-268: Model-to-implementation abstraction relation binding and refusal

## Description

Bind model objects, populations and operation frames to implementation state
through an authored abstraction relation, and refuse emission onto
implementation code referencing an unbound model element.

## Test Procedure

Use the subject and inputs named by FR-353. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Vectors

| Vector | Mutation | Expected disposition |
| --- | --- | --- |
| AB-01 | Request emission onto implementation code referencing a model object, population or operation frame with no authored binding | Refuses before generation with `refused { code: missing_declaration, cause: missing-name }`, naming the unbound element and its owning model identity; the refusal is scoped to that item, and an independent sibling item with a resolved binding still reaches generation |
| AB-02 | Author a second binding for a key an existing binding already covers, once with an identical implementation representation and once with a conflicting one | Both refuse `refused { code: invalid_model_binding, cause: conflicting-binding }`, naming both candidate bindings and the shared key; neither binding is silently selected |

## Expected Results

Every positive and boundary result matches FR-353; each model object,
population and operation frame maps to exactly one implementation state
representation, a model object or population keyed by its AD-006
effective-declaration key and an operation frame by its operation key (the
declaring object type's effective-declaration key and the operation
identifier), with a frame binding admitted for an operation no clause names; Kani and Verus generation consume the bound relation through the
same AD-010 negotiation path; emission onto implementation code referencing a
model element with no bound abstraction relation refuses before generation
with `missing_declaration`/`missing-name`, naming the unbound element and its
owning model identity, scoped to that item alone, with no partial or
approximated relation emitted in its place; a duplicate or conflicting binding
on one key refuses `invalid_model_binding`/`conflicting-binding` naming both
candidate bindings and the shared key; rebinding an element creates an
explicit new relation revision; each mutation returns its exact typed
disposition with source/provenance and preserves independent sibling results.
