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
   application, a `quire.op.temporal.fair` application and a
   `quire.op.temporal.clause` application each at the body root of a
   `function` node; each of a `quire.op.temporal.always`,
   `quire.op.temporal.fair` and `quire.op.temporal.clause` application
   nested in a node body's arguments;
   a `holds` whose operand is an integer term.
4. Encode `fair Op` and `fair weak whole Op` versions of one clause.
5. Feed a package with a state-step defect and a temporal placement defect,
   and one with a temporal placement defect and a temporal clause defect.
6. Interval admission by profile: `null`, integer `[2,*]`, integer `[0,3]`
   and timed `[0,3]` under each of the five FR-250 profiles; under
   `quire.temporal.timed/v1` the timed `[0,3]`, `[0,3)`, `(0,3]`, `(0,3)`,
   `[1/2,3]`, `(3,3]`, `[3,1]` and `[2/4,3]`; an interval-capable operator
   whose `member` is `null`.
7. Negative bounds: `{lower: "-1", upper: null}` and `{lower: "0", upper:
   "-1"}` under each profile, and a timed bound with numerator `"-1"`.
8. Profile check: the clause law naming `quire.temporal.timed/v1`; naming
   the package's own `protocol_profile` selection; naming
   `quire.temporal.unknown/v1`, which no lock selection names; each of the
   last two also with an `over` reference to a non-parameter node.
9. Fairness: a member naming an operation the declaring `object_type` node
   only inherits and that a subtype redefines; a name matching two effective
   operations; a declaring node keyed under an unselected domain package; a
   `declaration` naming a `systems_interface` model node and one naming a
   `value` node; a non-empty fairness argument under each profile.
10. Placement: a reference to a `temporal`/`formula` node from a state
    clause's condition; a reference to a `temporal`/`fairness` node from a
    formula argument; a diagnostic whose `details` reference each of a
    `temporal`/`formula`, `temporal`/`fairness`, `expression`/`case`,
    `composite_type`/`union` and `value`/`union_value` node; a diagnostic
    whose `details` term is an application of each of the
    `temporal_formula`, `temporal_fairness` and `case` operator classes; a
    `details` term that is a `temporal_formula` application with a nested
    `case` argument; a `details` aggregate of a `temporal_formula`
    application and then a `case` application.

## Expected Results

1. Both are admitted: each body is a `quire.op.temporal.clause` application
   with one `temporal_profile` law, a `null` member and the arguments over,
   clock, activation, captures, fairness and formula; the formula argument and
   each fairness member reference their own `temporal`/`formula` and
   `temporal`/`fairness` nodes, and every body follows FR-322's body grammar
   (FR-370-AC-1).
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
   body-root applications refuse `ill_typed`/`operator-ineligible` at the
   node holding them and the three nested applications fail the schema and
   refuse `malformed_wire` at the nested application (FR-370-AC-5); the
   integer `holds` operand refuses `ill_typed`/`operator-ineligible` at the
   operation step (FR-370-AC-7).
4. The two encodings are byte-identical (FR-370-AC-4).
5. The state-step defect is reported first; then the placement defect before
   the clause defect (FR-370-AC-7).
6. `quire.temporal.infinite-trace/v1` admits `null`, `[2,*]` and integer
   `[0,3]`; `quire.temporal.timed/v1` admits `null` and timed `[0,3]` and
   refuses `[2,*]` and integer `[0,3]` `operation-member-mismatch`; each
   bounded profile admits only integer `[0,3]`; the timed form outside the
   timed profile and the `null` member refuse `operation-member-mismatch`
   (FR-370-AC-3). Under the timed profile the four end variants and
   `[1/2,3]` are distinct and admitted, `(3,3]` and `[3,1]` refuse
   `invalid-value` and `[2/4,3]` `invalid_semantic_graph` (FR-370-AC-8).
7. Each fails the published schema and refuses `invalid_package`/
   `invalid-value` at the bound under every profile (FR-370-AC-9).
8. The timed law is known; the protocol selection refuses
   `unknown_profile`/`wrong-selection-role` and the unknown label
   `unknown_profile`/`unsupported-selection` at
   `/semantic_graph/nodes/{n}/body/operation/laws/0/definition`, ahead of the
   `over` defect (FR-370-AC-10).
9. The inherited operation is admitted and resolves to the redefinition;
   the ambiguous name refuses `ambiguous_declaration`/`ambiguous-name`, the
   unselected owner `missing_declaration`/`missing-selection`, and both
   wrong declaring nodes `invalid_model_binding`/`malformed-declaration` at
   the target (FR-370-AC-11); the non-empty fairness argument is admitted
   under the timed and infinite-trace profiles and refuses
   `operation-member-mismatch` under the three bounded ones (FR-370-AC-4).
10. Both misplaced references refuse `ill_typed`/`operator-ineligible` at
    the referencing node; each `details` reference to a formula, fairness or
    `case` node refuses `ill_typed`/`operator-ineligible` at
    `/diagnostics/entries/{e}/details/{d}`, each `details` application at
    `/diagnostics/entries/{e}/details/{d}/operator`, the application with a
    nested `case` at `/diagnostics/entries/{e}/details/{d}/operator`, the
    aggregate at `/diagnostics/entries/{e}/details/{d}/members/0/operator`,
    and the union and union-value references are admitted (FR-370-AC-12).
