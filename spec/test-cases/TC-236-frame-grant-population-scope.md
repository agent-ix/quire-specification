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
ranges over. This TC does not supply any missing implementation/conformance
evidence itself.

## Test Procedure

Author a model with a supertype and a subtype, an operation whose frame
grants `modifies` on a supertype field and `creates`/`deletes` on the
supertype, and an invocation whose pre/post populations include an object of
the subtype that is not reachable from any operation parameter.

Vector 1 (`modifies` scope): write the granted field on the unreachable
subtype object. Expect the write permitted, because the grant ranges over
every object of the invocation's pre and post populations whose effective
type has the field as an effective member, not only objects reachable from a
parameter. Then write a field the frame does not grant. Expect
`frame_violation`.

Vector 2 (`creates`/`deletes` scope): create a subtype instance under a
`creates` entry naming its supertype. Expect the creation permitted, because
the covered population is every object whose most-specific type conforms to
the named type (FR-151). Then create an instance of a type unrelated to any
`creates` entry. Expect `frame_violation`.

## Expected Results

A `modifies` grant's write permission and a `creates`/`deletes` grant's
identity-set permission each range over the conformance-inclusive population
FR-013 and FR-151 state, independent of parameter reachability; a write or
identity change outside that population receives `frame_violation`.
