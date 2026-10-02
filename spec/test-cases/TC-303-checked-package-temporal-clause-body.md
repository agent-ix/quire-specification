---
id: TC-303
title: "Checked-package temporal clause body and operation identities"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-370
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-303: Checked-package temporal clause body and operation identities

## Description

Schema and reader vectors for the `temporal`/`temporal_clause` node body and
the `quire.op.temporal.*` operation identities.

## Test Procedure

Encode as V2 packages:

1. TC-293's step 1 clause (infinite-trace, `fair weak each`) and the TC-290
   bounded claim (event-position false-extension, `on origin`).
2. A formula using each of the eight interval-capable operators with and
   without an interval, `holds`, `true`, `false`, `not`, `and`, `or`,
   `implies`, and `once[1,1]`.
3. Mutations: the clause's arguments reordered; the fairness argument missing;
   an unbounded `eventually` under the event-position profile; a fairness
   member under the event-position profile; an interval `{lower: "3", upper:
   "1"}`; a fairness member naming `attemptUpdat`; a `quire.op.temporal.always`
   application in a state clause's condition; a `quire.op.temporal.fair`
   application in a formula; a `quire.op.temporal.clause` nested in a formula;
   a `holds` whose operand is an integer term.
4. Encode `fair Op` and `fair weak whole Op` versions of one clause.
5. Feed a package with a state-step defect and a temporal placement defect,
   and one with a temporal placement defect and a temporal clause defect.

## Expected Results

1. Both are admitted: each body is a `quire.op.temporal.clause` application
   with one `temporal_profile` law, a `null` member and the arguments over,
   clock, activation, captures, fairness and formula (FR-370-AC-1).
2. Each operator carries its own identity; bounded operators carry
   `{lower, upper}` and unbounded ones `null`; `once[1,1]` is
   `quire.op.temporal.once` with `{lower: "1", upper: "1"}`; both V2 schemas
   enumerate every identity, class, family and member kind used
   (FR-370-AC-2, FR-370-AC-6).
3. Reordered or missing arguments fail the schema (FR-370-AC-1); the unbounded
   operator and the fairness member under the bounded profile refuse
   `operation-member-mismatch`, the reversed interval `invalid-value`
   (FR-370-AC-3, FR-370-AC-4); the unknown operation refuses
   `missing_declaration`/`missing-name` (FR-370-AC-4); the three misplaced
   applications refuse `ill_typed`/`operator-ineligible` at the node holding
   them (FR-370-AC-5); the integer `holds` operand refuses `ill_typed`/
   `operator-ineligible` at the operation step (FR-370-AC-7).
4. The two encodings are byte-identical (FR-370-AC-4).
5. The state-step defect is reported first; then the placement defect before
   the clause defect (FR-370-AC-7).
