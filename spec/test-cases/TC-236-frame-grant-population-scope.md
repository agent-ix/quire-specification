---
id: TC-236
title: "Frame grant population scope"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-013
    type: verifies
---
## Description

Planned test of a `modifies`/`creates`/`deletes` frame grant's population
scope: which objects a `modifies` entry's write permission ranges over, and
which population a `creates`/`deletes` entry's identity-set permission
ranges over.

## Test Procedure

Author a model with a supertype and a subtype, where the subtype redefines
one of the supertype's fields, an operation whose frame grants `modifies` on
a supertype field and `creates`/`deletes` on the supertype, and an invocation
whose pre/post populations include an object of the subtype that is not
reachable from any operation parameter.

Vector 1 (`modifies` scope, direct member): write the granted field on the
unreachable subtype object. Expect the write permitted, because the grant
ranges over every object of the invocation's pre and post populations whose
effective type has the field as an effective member. Then write a field the
frame does not grant. Expect `frame_violation`.

Vector 2 (`modifies` scope, `redefines`): write the granted field on an
unreachable object of the subtype that reaches the field only through its
`redefines` declaration. Expect the write permitted, because the grant's
population includes an object whose effective type reaches the granted
field through `redefines`.

Vector 3 (`creates` scope): create a subtype instance under a `creates`
entry naming its supertype, declaring the invocation's creation inventory to
match exactly the objects actually created. Expect exactly the creation
permitted, with no diagnostic, because the declared inventory is correct and
the covered population is every object whose most-specific type conforms to
the named type (FR-151). Then, in the same invocation, create an instance of
a type unrelated to any `creates` entry, again declaring the creation
inventory to match exactly the objects actually created. Expect exactly
`frame_violation`.

Vector 4 (`deletes` scope): delete a subtype instance under a `deletes`
entry naming its supertype, declaring the invocation's deletion inventory to
match exactly the objects actually deleted. Expect exactly the deletion
permitted, with no diagnostic, because the covered population is every
object whose most-specific type conforms to the named type (FR-151). Then,
in the same invocation, delete an instance of a type unrelated to any
`deletes` entry, again declaring the deletion inventory to match exactly the
objects actually deleted. Expect exactly `frame_violation`.

Vector 5 (`frame.redefines-link` metering): extend the model to a chain
`T0`, `T1 ≤ T0`, `T2 ≤ T1`, `T3 ≤ T2`, where field `T0/f` is redefined by
`T1/f1`, `T1/f1` by `T2/f2` and `T2/f2` by `T3/f3`, and an operation whose
frame grants only `modifies: [T0/f]`. Run one invocation whose sole change is
a write of `f3` on one `T3` object, so the walk follows `k = 3` `redefines`
links (`f3` to `f2`, `f2` to `f1`, `f1` to `f`). With `work_units` limited to
4, expect the write permitted, with exactly one `frame.write` and three
`frame.redefines-link` charges and `work_units` consumed 4. With `work_units`
limited to 3, expect `incomplete { limit_kind: work_units, limit: 3,
consumed: 3, next_charge: 1, charge_point: frame.redefines-link }` and no
frame verdict.

## Expected Results

A `modifies` grant's write permission and a `creates`/`deletes` grant's
identity-set permission each range over the conformance-inclusive population
FR-013 and FR-151 state, whether an object is reachable from a parameter or
not and whether its effective type carries the granted field directly or
reaches it through `redefines`; a write or identity change inside that
population, declared in a matching creation/deletion inventory, succeeds with
no diagnostic, and a write or identity change outside that population
receives exactly `frame_violation`. Each field write charges one
`frame.write` and one `frame.redefines-link` per `redefines` link followed,
so Vector 5 consumes exactly four work units and stops at its third
`frame.redefines-link` under a limit of three (FR-013-AC-7).
