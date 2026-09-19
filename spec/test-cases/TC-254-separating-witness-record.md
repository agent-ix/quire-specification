---
id: TC-254
title: "Retain a family-neutral separating witness or leave the result witness-free"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-041
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-061
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-094
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-243
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-351
    type: verifies
---
## Description

Exercises the FR-351 separating witness record across the collection, temporal
and protocol families, and over the dispositions that retain no witness.
Planned.

## Test Procedure

Decide a bounded `forall`/`exists` query, a temporal obligation and a protocol
claim each to a `decisive-witness` and a `decisive-counterexample` basis, and
inspect each retained record's deciding element, index, value path and trace
position. Compare the collection, temporal and protocol records for identical
shape. Separately drive one obligation to a `closed-scope` basis, one to
`unsettled`, one to resource exhaustion, one to cancellation and one to an
unsupported requested backend capability. Attempt to read a record missing its
deciding element, a record carrying an extra foreign member, and a default
value substituted for an absent index, value path or trace position.

## Expected Results

Every decisive basis retains one record with its exact deciding element and
the index, value path and trace position its family assigns; the collection,
temporal and protocol records share one shape. `closed-scope` and `unsettled`
results carry no witness. The exhausted, cancelled and unsupported results
retain their disposition and checked subject and emit no witness or substitute
witness. Every malformed, foreign-shaped or defaulted-absence record is
refused rather than accepted as the witness.
