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
  - target: ix://agent-ix/quire-specification/FR-207
    type: verifies
---

# TC-254: Retain a family-neutral separating witness or leave the result witness-free

## Description

Exercises the FR-351 separating witness record across the collection, temporal
and protocol families, and over the dispositions that retain no witness.
Planned.

## Test Procedure

Decide a bounded `forall`/`exists` query, a temporal obligation and a protocol
claim each to a `decisive-witness` and a `decisive-counterexample` basis, and
inspect each retained record's deciding quantifier, deciding element, index,
value path and trace position. Refute a collection `forall` at its third
element; refute two occurrences of one quantifier node at equal elements and
compare the two records; decide a quantifier over a `filter` domain and over a
`map` domain; and decide a quantifier over a collection built inside the
claim (FR-351-AC-7). Compare the collection, temporal and protocol records for identical
shape. Separately drive one obligation to a `closed-scope` basis, one to
`unsettled`, one to resource exhaustion, one to cancellation and one to an
unsupported requested backend capability. Attempt to read a record missing its
deciding element, a record carrying an extra foreign member, and a default
value substituted for an absent index, value path or trace position.
Refute a `realizability` request item carrying an `assessed_root`; repeat
with only its `assessed_root` changed; then submit the item with
`assessed_root`, its `observation` and its `subject` each absent
(FR-351-AC-8).

## Expected Results

Every decisive basis retains one record with its exact deciding element and
the deciding quantifier, index, value path and trace position its family
assigns; the third-element refutation records the quantifier's occurrence key,
that element, index 2 and the path to the stored collection; the two
occurrence records compare unequal; the `filter` and `map` records name the
stored source collection and source index; the built-collection record is
rooted at the building expression's occurrence key; the collection,
temporal and protocol records share one shape. `closed-scope` and `unsettled`
results carry no witness. The exhausted, cancelled and unsupported results
retain their disposition and checked subject and emit no witness or substitute
witness. Every malformed, foreign-shaped or defaulted-absence record is
refused rather than accepted as the witness. The realizability record's
`value_path` root equals the item's `assessed_root` with an `object` subject
and empty `steps`; the changed-root run has the same key and a record that
differs only in `value_path.root`; each absent root member is refused
`invalid_runtime_input`/`missing-member` naming it.
