---
id: TC-248
title: "Iteration admitted without a maximum or a proved invariant/variant"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-228
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-171
    type: verifies
---

# TC-248: Iteration admitted without a maximum or a proved invariant/variant

## Description

Admit a `bounded-iteration` construct with a progress edge whether it carries
an authored maximum, an unproved authored invariant and variant, both or
neither, and refuse a zero-progress loop.

## Test Procedure

Use the subject and inputs named by FR-228 and FR-171.
Execute its positive case, each declared boundary, and each explicit
refusal/non-conclusive mutation through the real local Rust boundary; compare
the complete typed result and artifact accounting with the criterion.

## Expected Results

Every positive and boundary result matches FR-228-AC-5 and FR-171-AC-2: a loop
with an invariant and variant and no maximum is admitted without its
invariant or variant being proved; a loop with neither is admitted; a
zero-progress loop refuses; an unproved termination claim is inconclusive or
refused rather than silently bounded; each mutation
returns its exact typed disposition with source/provenance and preserves
independent sibling results.
