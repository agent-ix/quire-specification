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

Use the exact version-locked subject and inputs named by FR-353. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-353; each model object,
population and operation frame maps to exactly one implementation state
representation; Kani and Verus generation consume the bound relation through
the same AD-010 negotiation path; emission onto implementation code
referencing a model element with no bound abstraction relation refuses before
generation, naming the unbound element and its owning model identity, with no
partial or approximated relation emitted in its place; rebinding an element
creates an explicit new relation revision; each mutation returns its exact
typed disposition with source/provenance and preserves independent sibling
results.
