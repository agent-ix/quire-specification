---
id: TC-944
title: "Scoped members of a model-document operation frame"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-013
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: verifies
---
# TC-944: Scoped members of a model-document operation frame

## Description

Verify the model contract's Frames rule for the optional members
`modifies_receiver`, `deletes_receiver` and `deletes_parameters`: the closed
member set, the member and entry refusals and their order, and the lift of the
members to the checked-package frame body. Scope: FR-013-AC-11 to
FR-013-AC-14.

## Test Procedure

Fixture: a domain package with object type `Counter` (fields `value` and
`step`), object type `Sub` specializing `Counter`, object type `Gauge` with
field `d`, a relationship `Supersedes` between `Counter` objects, and an
operation `t(n: Integer, a: Reference<Counter>)` of `Counter`, process `Flow` (identity field `id` and field `x`, both `Integer`, one step `begin` of kind `command` consuming and emitting nothing, as TC-235's `reserve_stock`) with an operation `go(a: Reference<Counter>)`, state machine `Lamp` (field `on: Boolean`, states `Off` and `On`, transition `[0]` `Off → On` on its operation `flip`, as TC-235's `OrderLifecycle`), where `go`, `flip` and `t` start with the empty frame `{"modifies":[],"creates":[],"deletes":[]}`, and the operations of step 2, `bump` and `merge(a: Reference<Counter>, b: Reference<Counter>)`, both of `Counter`. Each of `bump` and `merge` is named by its own postcondition clause (`DoneBump`, `DoneMerge`, each `{ true }`) so that the package holds its frame node; an operation no clause names has no frame node. Node identity
strings are written with the prefix `ix://test/counters/` as `Counter/value`,
`Counter/step`, `Gauge/d`, `Flow/x`, `Counter` and `Supersedes`.

1. Intake the package once per row, each row replacing the `frame` of `t`, or of the operation the row names (of
   `Sub` in R7, where `Sub` declares its own `t` with the same parameters and `Counter.t` stays as it is):

   | Row | `frame` |
   | --- | --- |
   | R1 | `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":["Counter/nope"]}` |
   | R2 | `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":["Gauge/d"]}` |
   | R3 | `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":["Supersedes"]}` |
   | R4 | `{"modifies":[],"creates":[],"deletes":[],"deletes_parameters":["missing"]}` |
   | R5 | `{"modifies":[],"creates":[],"deletes":[],"deletes_parameters":["n"]}` |
   | R6 | `{"modifies":[],"creates":[],"deletes":[],"deletes_parameters":["a"],"deletes_receiver":true}` |
   | R7 | `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":["Counter/value"]}` |
   | R8 | `{"modifies":["Counter/value"],"creates":[],"deletes":[],"modifies_receiver":["Counter/nope","Counter/value"]}` |
   | R9 | `{"modifies":["Counter/value"],"creates":[],"deletes":[],"modifies_receiver":["Counter/value"]}` |
   | R10 | `{"modifies":[],"creates":[],"deletes":[],"deletes_receiver":"yes"}` |
   | R11 | `{"modifies":[],"creates":[],"deletes":[],"modifies_self":[]}` |
   | R12 | `{"modifies":[],"creates":[]}` |
   | R13 | `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":[7]}` |
   | R14 | `{"modifies":[],"creates":[],"deletes":[]}` |
   | R15 | `go` of `Flow`: `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":["Flow/x"]}` |
   | R16 | `go` of `Flow`: `{"modifies":[],"creates":[],"deletes":[],"deletes_receiver":true}` |
   | R17 | `go` of `Flow`: `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":[],"deletes_receiver":false}` |
   | R18 | `go` of `Flow`: `{"modifies":[],"creates":[],"deletes":[],"deletes_parameters":["a"]}` |
   | R19 | `flip` of `Lamp`: `{"modifies":[],"creates":[],"deletes":[],"deletes_receiver":true}` |
   | R20 | `go` of `Flow`: `{"modifies":[],"creates":[],"deletes":[],"modifies_receiver":["Flow/x"],"deletes_receiver":true}` |

2. Emit the checked package for `bump` of `Counter`, whose frame is
   `{"modifies":["Counter/value"],"creates":[],"deletes":[],"modifies_receiver":["Counter/step"],"deletes_receiver":true}`, and for `merge`, whose frame is
   `{"modifies":[],"creates":[],"deletes":["Counter"],"deletes_parameters":["b","a"],"deletes_receiver":true}`,
   and read each `state`/`frame` node body. `<Counter>` stands for the NodeRef
   object of the `Counter` object type node.

The filament-core-data semantic-IR frame schema validates the model document
before intake and must admit the scoped members. Rows R1 to R9, R15 to R20 and
step 2 use a scoped member, so they depend on that schema change. Rows R10 to
R13 are member-shape rows: the schema, before and after that change, refuses a
wrong-typed member, an unnamed member and a non-string element at the member,
and refuses a frame with no `deletes` (required before and after), each with
its own schema diagnostic before intake; the `malformed-declaration` stated
for them is the intake reader's contract where the schema has not run. Only
R14, which uses the three arrays alone, is reachable today.

## Expected Results

- R1 refuses `missing_declaration`/`missing-name` at the `modifies_receiver`
  entry (no node of that identity exists).
- R2 refuses `invalid_model_binding`/`malformed-declaration` at the entry
  (`d` is a field of `Gauge`, to which `Counter` does not conform); R3 refuses
  the same (`Supersedes` is a relationship).
- R4 refuses `missing_declaration`/`missing-name` at the entry (`missing` is no
  parameter); R5 refuses the same (`n` is an `Integer`).
- R6 admits (`a` is a `Reference<Counter>`; `deletes_receiver` is admitted for
  an operation of an object type). R7 admits (`Sub` inherits `value`).
- R8 refuses only `missing_declaration`/`missing-name` at `Counter/nope`; the
  conflict on `Counter/value` is checked once every entry resolves
  (FR-013-AC-12).
- R9 refuses `invalid_model_binding`/`conflicting-binding` naming both entries
  and `t`, located at the `modifies_receiver` entry, the later in document
  order.
- R10 refuses `invalid_model_binding`/`malformed-declaration` at
  `deletes_receiver` (a string is not a boolean), R11 at `modifies_self` (a
  member the rule does not name), R12 at the absent `deletes`, and R13 at
  `modifies_receiver` (a number is not a string), each before any entry
  is read (FR-013-AC-11).
- R14, a frame with only the three required arrays, admits: the three optional
  members are absent, so empty or false.
- R15 refuses `invalid_model_binding`/`malformed-declaration` at
  `modifies_receiver` (the receiver of `go` is a run, not an object of an
  object type; the entry `Flow/x` is not read). R16 refuses the same at
  `deletes_receiver`, and R19 refuses the same at `deletes_receiver` for the
  state machine `Lamp`. R20 refuses at `modifies_receiver` alone, the first of
  the two members in the checking order (FR-013-AC-14).
- R17 and R18 admit: an empty `modifies_receiver`, `deletes_receiver: false`
  and a `deletes_parameters` name of a reference parameter read as for an
  object type.
- Step 2: the body of `bump` is
  `{"creates":[],"deletes":[],"deletes_scoped":[{"kind":"receiver"}],"modifies":[{"declaration":<Counter>,"kind":"field","name":"step","scope":"receiver"},{"declaration":<Counter>,"kind":"field","name":"value","scope":"every-object"}],"term":"frame"}`
  (`step` before `value`, one declaration ordering by name); the body of
  `merge` is
  `{"creates":[],"deletes":[<Counter>],"deletes_scoped":[{"kind":"receiver"},{"kind":"parameter","name":"a"},{"kind":"parameter","name":"b"}],"modifies":[],"term":"frame"}`
  (the authored order `b`, `a` is not the emitted order; FR-340 sorts the
  receiver first, then parameters by name) (FR-013-AC-13).
