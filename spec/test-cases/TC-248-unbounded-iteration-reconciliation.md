---
id: TC-248
title: "Unbounded iteration admitted by proved invariant/variant"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-228
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-171
    type: verifies
---

# TC-248: Unbounded iteration admitted by proved invariant/variant

## Description

Admit a `bounded-iteration` construct that carries a proved invariant and
well-founded variant and no authored finite maximum, and refuse one that
carries neither.

## Test Procedure

Use the exact version-locked subject and inputs named by FR-228 and FR-171.
Execute its positive case, each declared boundary, and each explicit
refusal/non-conclusive mutation through the real local Rust boundary; compare
the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-228-AC-5 and FR-171-AC-2: a loop
with a proved invariant and well-founded variant is admitted without an
authored finite maximum; a loop with neither refuses; an unproved termination
claim is inconclusive or refused rather than silently bounded; each mutation
returns its exact typed disposition with source/provenance and preserves
independent sibling results.
