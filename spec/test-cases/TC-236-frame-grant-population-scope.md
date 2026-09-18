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
match exactly the objects actually created. Expect the creation permitted,
because the covered population is every object whose most-specific type
conforms to the named type (FR-151), and expect no `population_delta_mismatch`
because the declared inventory is correct. Then, in the same invocation,
create an instance of a type unrelated to any `creates` entry, again
declaring the creation inventory to match exactly the objects actually
created. Expect `frame_violation` and no `population_delta_mismatch`.

Vector 4 (`deletes` scope): delete a subtype instance under a `deletes`
entry naming its supertype, declaring the invocation's deletion inventory to
match exactly the objects actually deleted. Expect the deletion permitted,
because the covered population is every object whose most-specific type
conforms to the named type (FR-151), and expect no `population_delta_mismatch`.

## Expected Results

A `modifies` grant's write permission and a `creates`/`deletes` grant's
identity-set permission each range over the conformance-inclusive population
FR-013 and FR-151 state, whether an object is reachable from a parameter or
not and whether its effective type carries the granted field directly or
reaches it through `redefines`; a write or identity change outside that
population receives `frame_violation`, and a correctly declared
creation/deletion inventory never itself produces `population_delta_mismatch`.
