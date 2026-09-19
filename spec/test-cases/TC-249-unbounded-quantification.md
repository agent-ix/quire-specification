---
id: TC-249
title: "Unbounded quantification negotiation"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-346
    type: verifies
---

# TC-249: Unbounded quantification negotiation

## Description

Evaluate `forall`/`exists` over a bounded collection by occurrence-visiting
and over an unbounded collection by backend negotiation.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-346. Execute its
positive case, each declared boundary, and each explicit refusal/non-conclusive
mutation through the real local Rust boundary; compare the complete typed
result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-346; `forall`/`exists` over a
bounded collection is decided by FR-145 occurrence-visiting; over an unbounded
collection it is lowered to a target-neutral obligation and negotiated with a
registered quantifier-capable backend; a claim no registered backend can
discharge settles `unsupported` with a warning naming the required capability;
with the FR-349 SMT backend registered, at least one case settles `proved` and
at least one settles `refuted` through it; each mutation returns its exact
typed disposition with source/provenance and preserves independent sibling
results.
