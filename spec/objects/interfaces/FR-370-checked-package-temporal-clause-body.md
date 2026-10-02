---
id: FR-370
title: "Checked-package temporal clause body and temporal operation identities"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-048
    type: references
  - target: ix://agent-ix/quire-specification/FR-090
    type: references
  - target: ix://agent-ix/quire-specification/FR-362
    type: references
  - target: ix://agent-ix/quire-specification/FR-367
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-370: Checked-package temporal clause body and temporal operation identities

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "temporal"` and
`semantic_form: "temporal_clause"`, the producer SHALL encode its `body` as one
application of `quire.op.temporal.clause` whose formula argument is a tree of
the temporal operation applications catalogued below, each operator with its
own operation identity. The reader SHALL refuse a node whose rules below do
not hold, with the first refusal in the reader order below.

## Properties

- **body** — an `application` term with `operator: "temporal"` and
  `operation.identity: "quire.op.temporal.clause"`, `laws` exactly one
  `temporal_profile` law equal to the lock's `profile_selections` row of that
  role, `member` `null`, `mode` `null` and `leaves` empty. Its `arguments` are
  exactly, in this order:
  1. **over** — a `reference` to the `value`/`parameter` node of the clause's
     `over` parameter;
  2. **clock** — a `text` literal, the clause's clock role;
  3. **activation** — an `aggregate` of the bindings `kind`, a `text` literal
     `origin` or `each`; `trigger`, a `reference` to the trigger's
     `value`/`parameter` node for `each` and absent for `origin`; and `guard`,
     the Boolean guard term for `each` with a guard, absent otherwise;
  4. **captures** — an `aggregate` with one binding per capture in source
     order, named by the capture and holding its initializer term;
  5. **fairness** — an `aggregate` of `quire.op.temporal.fair` applications,
     one per member of the resolved fairness set
     ([FR-362](../../functional/temporal/FR-362-apply-fairness-constraints.md)),
     in fairness-set order; empty for a clause with none;
  6. **formula** — the clause's temporal term, an application of family
     `temporal`.
- **semantic_type** — the `scalar_type`/`boolean` node; the application's
  `result_type` names the same node.
- **occurrences** — every occurrence has role `claim`.
- **dependencies** — FR-322's application-node rule: the unique
  digest-ascending `reference` targets and member declarations of the body.

## Operation catalog entries

Each temporal operator is its own catalogued operation (FR-322 "Operation
families, groups and constraints"). `temporal` is the operand and result
family of a temporal formula, distinct from `boolean`
([FR-048](../../functional/foundation/FR-048-bind-native-temporal-syntax.md)).
None carries a law: the enclosing clause's `temporal_profile` law fixes the
meaning of every operator in its formula. The `temporal` result form, like the
`clause` result form, declares a Boolean truth: the application's
`result_type` names the package's `scalar_type`/`boolean` node, and the
catalog's `temporal` family keeps a formula term out of every `boolean`
operand position except a nested temporal operator's.

| Identity | Operator class | Operands | Result | Member |
| --- | --- | --- | --- | --- |
| `quire.op.temporal.clause` | `temporal` | `reference`, `text`, `aggregate`, `aggregate`, `aggregate`, `temporal` | `clause` (declared Boolean) | none |
| `quire.op.temporal.holds` | `temporal_formula` | `boolean` | `temporal` | none |
| `quire.op.temporal.true`, `quire.op.temporal.false` | `temporal_formula` | none | `temporal` | none |
| `quire.op.temporal.not` | `temporal_formula` | `temporal` | `temporal` | none |
| `quire.op.temporal.and`, `quire.op.temporal.or`, `quire.op.temporal.implies` | `temporal_formula` | `temporal`, `temporal` | `temporal` | none |
| `quire.op.temporal.eventually`, `quire.op.temporal.always`, `quire.op.temporal.once`, `quire.op.temporal.historically` | `temporal_formula` | `temporal` | `temporal` | `temporal_interval` |
| `quire.op.temporal.until`, `quire.op.temporal.release`, `quire.op.temporal.since`, `quire.op.temporal.triggered` | `temporal_formula` | `temporal`, `temporal` | `temporal` | `temporal_interval` |
| `quire.op.temporal.fair` | `temporal_fairness` | none | `temporal` | `fairness` |

The `temporal_interval` member is `{kind: "temporal_interval", interval}`,
where `interval` is `{lower, upper}`, two decimal strings with
`lower <= upper`, for an interval operator; `{lower, upper: null}` for a
lower-bounded operator `[a,*]`; and `null` for an unbounded operator ([FR-367](../../functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)).

The `fairness` member is `{kind: "fairness", fairness_kind, granularity,
declaration, name}`: `fairness_kind` is `weak` or `strong`, `granularity` is
`whole` or `each`, and `declaration` and `name` name the operation's declaring
`model` node and the operation, as FR-322's `operation` member does. The member holds the resolved form,
so `fair Op` and `fair weak whole Op` encode identically.

Strong previous is `quire.op.temporal.once` with interval `{lower: "1",
upper: "1"}`; it has no identity of its own.

## Placement and profile fit

- A `quire.op.temporal.clause` application is the body root of a
  `temporal`/`temporal_clause` node and stands nowhere else.
- A `temporal_formula` application stands only in a clause's formula argument
  or as an operand of another `temporal_formula` application. A
  `quire.op.temporal.fair` application stands only in a clause's fairness
  argument.
- Under a bounded `temporal_profile` law every interval-capable operator
  carries an interval with a non-null `upper` and the fairness argument is
  empty. Under `quire.temporal.infinite-trace/v1` an interval may be null,
  lower-bounded or closed.

A misplaced application refuses `ill_typed`/`operator-ineligible` at the node
that holds it. A null interval, a null `upper` or a non-empty fairness argument under a bounded
profile refuses `invalid_package`/`operation-member-mismatch` at the
application. An interval with `lower > upper` refuses
`invalid_package`/`invalid-value` at the application.

## Reader order

The temporal step runs after the state step
([FR-341](FR-341-checked-package-state-clause-body.md) "Reader order") and
before any operation refusal ([FR-322](FR-322-checked-package-artifact.md)).
Within it, the reader first checks the placement of every temporal
application, in ascending node-id digest order; then, for each
`temporal`/`temporal_clause` node in ascending node-id digest order: the `over`
reference targets a declared `value`/`parameter` dependency, else
`missing_declaration`/`missing-name` or `invalid_model_binding`/
`malformed-declaration`; then each fairness member's operation resolves on its
declaring node, else `missing_declaration`/`missing-name`; then the profile fit
of every interval and of the fairness argument; then each interval's bounds.
It reports the first defect. Operand families and `result_type` are the
applications' catalog checks at the operation step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-370-AC-1 | A `temporal`/`temporal_clause` node's body is a `quire.op.temporal.clause` application with one `temporal_profile` law, a `null` member and the six arguments over, clock, activation, captures, fairness and formula in that order; a body with arguments reordered, missing or of another term shape fails the published schema. | Test (TC-303) |
| FR-370-AC-2 | Each of the eight interval-capable operators, `holds`, the two temporal constants and the four connectives is encoded with its own `quire.op.temporal.*` identity, both V2 schemas enumerate those identities, the `temporal_formula` and `temporal_fairness` operator classes and the `temporal_interval` and `fairness` member kinds, and the operation catalog lists the `temporal` family. | Test (TC-303) |
| FR-370-AC-3 | An interval operator carries `{lower, upper}`; a lower-bounded operator carries `{lower, upper: null}` and an unbounded operator `null`, both admitted under `quire.temporal.infinite-trace/v1`; a `null` interval or a `null` `upper` under a bounded profile refuses `operation-member-mismatch`, and `lower > upper` refuses `invalid-value`. | Test (TC-303) |
| FR-370-AC-4 | `fair Op` and `fair weak whole Op` encode to byte-identical `quire.op.temporal.fair` applications, and a fairness member naming an operation absent from its declaring node refuses `missing_declaration`/`missing-name`; a non-empty fairness argument under a bounded profile refuses `operation-member-mismatch`. | Test (TC-303) |
| FR-370-AC-5 | A temporal operator application outside a clause's formula, a `quire.op.temporal.fair` application outside its fairness argument, and a `quire.op.temporal.clause` application nested in another term refuse `ill_typed`/`operator-ineligible` at the node that holds them. | Test (TC-303) |
| FR-370-AC-6 | Strong previous encodes as `quire.op.temporal.once` with interval `{lower: "1", upper: "1"}`. | Test (TC-303) |
| FR-370-AC-7 | The temporal step reports placement defects before clause defects, runs after the state step and before the operation step, and a `holds` operand of a non-Boolean family refuses `ill_typed`/`operator-ineligible` at the operation step. | Test (TC-303) |

## Published artifacts and the operation catalog

`proposals/checked-package-v2/schema.json` and
`node-identity-preimage.schema.json` enumerate the `quire.op.temporal.*`
identities, the `temporal_formula` and `temporal_fairness` operator classes and
the `temporal_interval` and `fairness` member kinds, and fix the
`temporal`/`temporal_clause` body. The published clause and all-families
packages encode their temporal clauses in this form.

The operation catalog `quire.checked-operation-catalog/v1`, owned by
quire-verification-contracts, SHALL hold the `temporal` family (a member of
the `any_term` group), the `temporal_formula` and `temporal_fairness`
operator classes' entries, the `temporal_interval` and `fairness` member
kinds, one entry per `quire.op.temporal.*` identity with the operands, result
form and member of the table above, and `quire.op.temporal.clause` with no
member.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`, the
  operation catalog, the application-node preimage and the reader's refusal
  order this artifact adds the temporal step to.
- [FR-362](../../functional/temporal/FR-362-apply-fairness-constraints.md) and
  [FR-367](../../functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md)
  own the fairness set and the interval rules this body encodes.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact locus.

## References

- quire-spec-language ADR-018 QS-11 and ADR-014 §13; Linear STD-99 and STD-131.
