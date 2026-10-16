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
FR-013-AC-13.

## Test Procedure

Fixture: a domain package with object type `Counter` (fields `value` and
`step`), object type `Sub` specializing `Counter`, object type `Gauge` with
field `d`, a relationship `Supersedes` between `Counter` objects, and an
operation `t(n: Integer, a: Reference<Counter>)` of `Counter`, and the operations of step 2, `bump` and `merge(a: Reference<Counter>, b: Reference<Counter>)`, both of `Counter`. Each of `bump` and `merge` is named by its own postcondition clause (`DoneBump`, `DoneMerge`, each `{ true }`) so that the package holds its frame node; an operation no clause names has no frame node. Node identity
strings are written with the prefix `ix://test/counters/` as `Counter/value`,
`Counter/step`, `Gauge/d`, `Counter` and `Supersedes`.

1. Intake the package once per row, each row replacing the `frame` of `t` (of
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

2. Emit the checked package for `bump` of `Counter`, whose frame is
   `{"modifies":["Counter/value"],"creates":[],"deletes":[],"modifies_receiver":["Counter/step"],"deletes_receiver":true}`, and for `merge`, whose frame is
   `{"modifies":[],"creates":[],"deletes":["Counter"],"deletes_parameters":["b","a"],"deletes_receiver":true}`,
   and read each `state`/`frame` node body. `<Counter>` stands for the NodeRef
   object of the `Counter` object type node.

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
- Step 2: the body of `bump` is
  `{"creates":[],"deletes":[],"deletes_scoped":[{"kind":"receiver"}],"modifies":[{"declaration":<Counter>,"kind":"field","name":"step","scope":"receiver"},{"declaration":<Counter>,"kind":"field","name":"value","scope":"every-object"}],"term":"frame"}`
  (`step` before `value`, one declaration ordering by name); the body of
  `merge` is
  `{"creates":[],"deletes":[<Counter>],"deletes_scoped":[{"kind":"receiver"},{"kind":"parameter","name":"a"},{"kind":"parameter","name":"b"}],"modifies":[],"term":"frame"}`
  (the authored order `b`, `a` is not the emitted order; FR-340 sorts the
  receiver first, then parameters by name) (FR-013-AC-13).
