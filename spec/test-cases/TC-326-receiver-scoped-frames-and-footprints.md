---
id: TC-326
title: "Receiver-scoped frames and footprints"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-013
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-383
    type: verifies
---

# TC-326: Receiver-scoped frames and footprints

## Description

Enforce receiver-scoped and every-object `modifies` entries, carry the scope
through the checked package, and derive and enforce the write and read
footprints partial-order reduction uses.

## Test Procedure

Subject: TC-323's counters, plus an operation `copy(p: Reference<Counter>)`
with frame `modifies self.v` and postcondition `self.v = p.v`, and an
operation `reset()` with frame `modifies self.v` whose postcondition reads
`deref(self.peer).v` through a field `peer: Option<Reference<Counter>>`.

1. Frame `modifies self.v` on `bump`: observe an invocation on `a` that
   changes `a.v`, then one that also changes `b.v`.
2. Frame `modifies v` on `bump`: repeat both observations.
3. Frame `modifies v, self.v` on `bump`.
4. Emit the checked package for frames 1 and 2 and read each frame node's
   field entry; remove `scope` from an entry; set it to `some-objects`.
5. Derive `W` and `R` for `bump(a)`, `copy(a, b)` and `reset(a)`, and decide
   independence of `bump(a)` with `bump(b)`, with `copy(c, a)`, and with
   `reset(c)` under frame 1, and of `bump(a)` with `bump(b)` under frame 2.
6. Evaluate `reset(a)`'s postcondition for the engine over an observation
   restricted to a read footprint that omits the `peer` target's `v`.

7. Add an operation `check()` with frame `modifies self.v` and pre
   `forall o in counters: o.v < 2`; derive its read footprint and decide its
   dependence with `bump(b)`, with `spawn()` (frame `creates Counter`) and
   with `drop()` (frame `deletes self`) on `c`.
8. Frame `deletes self` on `drop()`: observe an invocation on `a` that
   deletes `a`, then one that deletes `b`. Frame `deletes p` on
   `drop2(p: Reference<Counter>)` with argument `b`: delete `b`, then `a`.
   Frame `deletes Counter`: delete `a` and `b` together.
9. Emit the checked package for `drop`, `drop2` and the type-wide frame and
   read each frame node's `deletes_scoped`; remove the member; name a
   parameter `q` the operation does not declare; order a `parameter` entry
   before the `receiver` entry.

## Expected Results

Under frame 1 the first observation is frame-valid and the second receives
`frame_violation` at `b.v`; under frame 2 both are frame-valid; frame 3
refuses `invalid_model_binding`/`conflicting-binding` (FR-013-AC-8). In case
4 the entries carry `scope` `receiver` and `every-object`, the two
`package_id`s differ, and an entry with no `scope` or `some-objects` fails
the schema (FR-340-AC-16). In case 5 `W(bump(a)) = {(a, v)}` and
`R(bump(a)) = {(a, v), (counters, membership of a)}`, `R(copy(a, b))` holds
`(b, v)` and the membership of `a` and `b`, `W(copy(a, b)) = {(a, v)}`, and
`R(reset(a))` holds `(Counter, v, any object)`; `bump(a)` and `bump(b)` are
independent under frame 1 and dependent under frame 2, `bump(a)` and
`copy(c, a)` are dependent, and `bump(a)` and `reset(c)` are dependent
(FR-383-AC-3). Case 6 settles the item `failed`, never a value
(FR-383-AC-3).

In case 7 `check()` reads `AnyField{Counter, v}` and `AnyMembership{counters}`
beside its receiver's membership, so it is dependent with `bump(b)`,
`spawn()` and `drop()` on `c` (FR-383-AC-13). In case 8 deleting the
receiver under `deletes self` and the argument under `deletes p` are
frame-valid, deleting `b` under `deletes self` and `a` under `deletes p`
receive `frame_violation`, and deleting both under `deletes Counter` is
frame-valid (FR-013-AC-9). In case 9 the bodies carry `[{kind: "receiver"}]`,
`[{kind: "parameter", name: "p"}]` and `[]`; a body without the member fails
the schema, the unknown parameter refuses `missing_declaration`/
`missing-name`, and the misordered array refuses `invalid_semantic_graph`
(FR-340-AC-17).
