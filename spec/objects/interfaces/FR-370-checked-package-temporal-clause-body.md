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
  - target: ix://agent-ix/quire-specification/FR-250
    type: references
  - target: ix://agent-ix/quire-specification/FR-255
    type: references
  - target: ix://agent-ix/quire-specification/FR-272
    type: references
  - target: ix://agent-ix/quire-specification/FR-342
    type: references
  - target: ix://agent-ix/quire-specification/FR-481
    type: references
---

# FR-370: Checked-package temporal clause body and temporal operation identities

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "temporal"` and
`semantic_form: "temporal_clause"`, the producer SHALL encode its `body` as one
application of `quire.op.temporal.clause` whose formula argument references a
tree of `temporal`/`formula` nodes, each the application of one temporal
operation catalogued below with its own operation identity. The reader SHALL refuse a node whose rules below do
not hold, with the first refusal in the reader order below.

## Properties

- **body** — an `application` term with `operator: "temporal"` and
  `operation.identity: "quire.op.temporal.clause"`, `laws` exactly one
  `temporal_profile` law equal to the lock's `profile_selections` row of that
  role, `member` `null`, `mode` `null` and `leaves` empty. Its `arguments` are
  exactly, in this order:
  1. **over** — a `reference` to the `value`/`parameter` node of the clause's
     `over` parameter. It is always present: the grammar admits no temporal
     clause without an `over` parameter, so the wire has no absent or `null`
     form for this argument;
  2. **clock** — a `text` literal, the clause's clock role;
  3. **activation** — an `aggregate` of the bindings `kind`, a `text` literal
     `origin` or `each`; `trigger`, a `reference` to the trigger's
     `value`/`parameter` node for `each` and absent for `origin`; and `guard`,
     the Boolean guard term for `each` with a guard, absent otherwise;
  4. **captures** — an `aggregate` with one binding per capture in source
     order, named by the capture and holding its initializer term;
  5. **fairness** — an `aggregate` of `reference`s to `temporal`/`fairness`
     nodes, each the body root of one `quire.op.temporal.fair` application,
     one per member of the resolved fairness set
     ([FR-362](../../functional/temporal/FR-362-apply-fairness-constraints.md)),
     in fairness-set order; empty for a clause with none;
  6. **formula** — a `reference` to the `temporal`/`formula` node whose body
     root is the clause's temporal term, an application of family `temporal`.
     An operand of a `temporal_formula` application that is itself a temporal
     term is a `reference` to its own `temporal`/`formula` node, so every body
     follows FR-322's body grammar whatever the formula's depth.
- **semantic_type** — the `scalar_type`/`boolean` node; the application's
  `result_type` names the same node.
- **occurrences** — every occurrence has role `claim`.
- **formula and fairness nodes** — a `temporal`/`formula` or
  `temporal`/`fairness` node has the clause's `semantic_type`, occurrences of
  role `expression`, and FR-322's application-node identity and dependencies.
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
where `interval` is one of:

- `null` for an unbounded operator
  ([FR-367](../../functional/temporal/FR-367-evaluate-interval-operators-under-infinite-trace.md));
- the **integer form** `{lower, upper}`, two non-negative decimal integer
  strings with `lower <= upper`, for a closed interval `[a,b]`, or
  `{lower, upper: null}` for a lower-bounded operator `[a,*]`; both ends are
  closed;
- the **timed form** `{lower, upper, lower_end, upper_end}`, used only under
  `quire.temporal.timed/v1`: `lower` and `upper` are exact non-negative
  rationals `{numerator, denominator}` in the clock binding's time unit,
  in lowest terms with a positive denominator as every other checked-package
  rational is, and `lower_end` and `upper_end` are each `closed` or `open`,
  so `[a, b]`, `(a, b]`, `[a, b)` and `(a, b)` are four distinct members
  ([FR-255](../temporal/FR-255-bounded-temporal-interval.md)). The upper
  bound is present, a timed interval being finite, except in a hybrid claim
  ([FR-481](FR-481-checked-package-hybrid-model-nodes.md) "Hybrid claims")
  whose profile admits `[a,*]`, where `upper` `null` with `upper_end` `open`
  is the interval `[q, *]`. `lower <= upper`,
  and `lower < upper` when either end is `open`.

An unbounded operator carries `{kind: "temporal_interval", interval: null}`.
An interval-capable operator whose `member` is `null` refuses
`invalid_package`/`operation-member-mismatch` at the application.

A bound outside its form's pattern, including a negative integer bound such
as `{lower: "-1", upper: null}` or a negative numerator, fails the published
schema and refuses `invalid_package`/`invalid-value` at that bound's pointer
during strict wire validation, before any step of this requirement runs, so
it is `invalid-value` under every profile and never
`operation-member-mismatch`. A timed bound not in lowest terms refuses
`invalid_semantic_graph`, as every non-normalized rational does
([FR-322](FR-322-checked-package-artifact.md)).

The `fairness` member is `{kind: "fairness", fairness_kind, granularity,
declaration, name}`: `fairness_kind` is `weak` or `strong`, `granularity` is
`whole` or `each`, and `declaration` and `name` name the operation's declaring
`model`/`object_type` node and the operation. The member holds the resolved form,
so `fair Op` and `fair weak whole Op` encode identically.

## Fairness resolution

A hybrid claim's fairness members name hybrid transitions and resolve by
[FR-481](FR-481-checked-package-hybrid-model-nodes.md) "Hybrid claims"; every
other fairness member's `name` resolves by [FR-322](FR-322-checked-package-artifact.md)
"Model-owned members" steps 2 and 3 among the **effective** operation
members of its `declaration` node, the set
[FR-362](../../functional/temporal/FR-362-apply-fairness-constraints.md)
resolves a fairness constraint against: own and inherited, with
redefinitions applied. An operation the declaring type only inherits is
admitted and resolves to its most derived redefinition.
[FR-342](FR-342-checked-package-operation-anchor-body.md)'s refusal of an
inherited-only operation belongs to an operation anchor and does not apply
to a fairness member. For one member, in this order:

1. **Declaring node.** The `declaration` target is a `model` node with
   `semantic_form` `object_type`, else `invalid_model_binding`/
   `malformed-declaration`, located at the target.
2. **Owner.** FR-322 step 2 looks up the declaration the node's
   `ModelOwner` names. A node that is not its own preimage, or whose owner
   names no selected declaration of its kind, was refused earlier by
   FR-322's derived-shape key stage (`missing_declaration`/
   `missing-selection` or `invalid_package`/`stale-node-key`).
3. **Name.** Exactly one effective operation member matches `name`; two or
   more refuse `ambiguous_declaration`/`ambiguous-name`, and none refuses
   `missing_declaration`/`missing-name`.

Strong previous is `quire.op.temporal.once` with interval `{lower: "1",
upper: "1"}`; it has no identity of its own.

## Placement and profile fit

- A `quire.op.temporal.clause` application is the body root of a
  `temporal`/`temporal_clause` node and stands nowhere else.
- A `temporal_formula` application stands only as the body root of a
  `temporal`/`formula` node, and that node is referenced only from a clause's
  formula argument or as an operand of another `temporal_formula` application.
  A `quire.op.temporal.fair` application stands only as the body root of a
  `temporal`/`fairness` node, referenced only from a clause's fairness
  argument.
- A reference to a `temporal`/`formula` or `temporal`/`fairness` node
  stands only where the rule above places it. A misplaced reference is
  located at the node whose body holds the reference, not at the formula or
  fairness node it names.
- A diagnostic's `details` never reference a `temporal`/`formula`,
  `temporal`/`fairness` or `expression`/`case` node, and never hold an
  application of the `temporal_formula`, `temporal_fairness` or `case`
  operator class. A `details` term may reference a `composite_type`/`union`
  or `value`/`union_value` node.

Each interval-capable operator's interval and the clause's fairness
argument fit the clause's `temporal_profile` law as this table states:

| Profile ([FR-250](../temporal/FR-250-temporal-semantic-profile.md)) | Admitted `interval` | Fairness argument |
| --- | --- | --- |
| `quire.temporal.infinite-trace/v1` | `null`, integer `[a,*]`, integer `[a,b]` | empty or non-empty |
| `quire.temporal.timed/v1` | `null`, timed `[a,b]` with either end open or closed | empty or non-empty |
| `quire.temporal.event-position.false-extension/v1`, `quire.temporal.fixed-sample.false-extension/v1`, `quire.temporal.timestamped-event.finite-window/v1` | integer `[a,b]` | empty |

A hybrid claim (FR-481) takes its intervals by FR-481 "Hybrid claims": the
timed form only under `quire.temporal.timed/v1`, as this table states, with
the timed `[q, *]` where that profile admits it; under any other profile
the integer form where the bounds are whole and closed and the timed form in
exact rational seconds otherwise; and the integer form under
`model-steps`. Its fairness argument fits this table. Under `quire.temporal.timed/v1` a lower-bounded
`[a,*]` interval and an integer-form interval are not admitted: a timed interval has a finite upper
bound (FR-255) and its bounds are exact rationals. The timed form is
admitted under no other profile.

An application at the body root of a node the rules above do not place it
in, and a misplaced reference to a formula or fairness node, refuses
`ill_typed`/`operator-ineligible` at the node whose body holds it. A
`temporal_formula`, `temporal_fairness` or `temporal` application nested
inside a node body or inside another term is outside
[FR-322](FR-322-checked-package-artifact.md)'s body grammar and refuses
`malformed_wire` at strict wire validation, like any other nested
application; only a nested `case` application is FR-322's named exception
([FR-440](FR-440-checked-package-union-and-case-nodes.md) "Case
placement"). Temporal nesting is expressed through references to
`temporal`/`formula` nodes. A reference from a diagnostic's `details` to
one of the three node forms above refuses `ill_typed`/`operator-ineligible` at that `details`
entry, `/diagnostics/entries/{e}/details/{d}`. An application of one of the
three operator classes above inside a `details` term refuses
`ill_typed`/`operator-ineligible` at that application's `operator` member,
`/diagnostics/entries/{e}/details/{d}/operator` when the term is the
application; strict wire validation reports it so, not as `malformed_wire`
([FR-322](FR-322-checked-package-artifact.md) "Body grammar"). When one
`details` term holds several such constructs, the reader reports the first
in document pre-order, outermost first: a `temporal_formula` application
with a nested `case` argument refuses at
`/diagnostics/entries/{e}/details/{d}/operator`, and an aggregate whose
members are a `temporal_formula` application and then a `case` application
refuses at `/diagnostics/entries/{e}/details/{d}/members/0/operator`. An
interval shape, or a non-empty fairness argument, that the clause's profile does not admit
refuses `invalid_package`/`operation-member-mismatch` at the application.
An interval with `lower > upper`, or a timed interval with an open end and
`lower = upper`, refuses `invalid_package`/`invalid-value` at the
application.

## Profile check

A clause's `temporal_profile` law is checked first among that clause's
checks. The law's definition names a known profile exactly when its
`identity` equals one of FR-250's five values byte for byte (FR-250
"Identity and comparison"); the reader needs no other catalog to decide it.
A law naming no known profile refuses `unknown_profile`, located at
`/semantic_graph/nodes/{n}/body/operation/laws/0/definition` for the
clause node `n`, with cause:

- `wrong-selection-role` when the same `DefinitionRef` is the `definition`
  of a `Selection` of the package's own lock (its `edition` or a
  `profile_selections` row) whose `role` is not `temporal_profile`;
- `unsupported-selection` otherwise.

Both causes are [FR-272](../foundation/FR-272-typed-diagnostic-cause.md)'s
`unknown_profile` variants. A clause whose `laws` is not exactly one law of
role `temporal_profile` has no profile to check: its law defect is the
operation step's (`operation-law-missing`, `operation-law-mismatch`), and the
temporal step skips that clause's profile fit.

## Reader order

The temporal step runs after the state step
([FR-341](FR-341-checked-package-state-clause-body.md) "Reader order") and
before any operation refusal ([FR-322](FR-322-checked-package-artifact.md)).
Strict wire validation, which refuses a negative or malformed bound,
precedes it. Within it, the reader first checks the placement of every
temporal application and of every reference to a `temporal`/`formula` or
`temporal`/`fairness` node, in ascending node-id digest order; then every
diagnostic `details` term, in entry order and then `details` order; then,
for each `temporal`/`temporal_clause` node in ascending node-id digest
order: the profile check ("Profile check"); then the `over`
reference targets a declared `value`/`parameter` dependency, else
`missing_declaration`/`missing-name` or `invalid_model_binding`/
`malformed-declaration`; then each fairness member, in fairness-argument
order, by the three steps of "Fairness resolution"; then the profile fit
of every interval and of the fairness argument; then each interval's bounds.
It reports the first defect. Operand families and `result_type` are the
applications' catalog checks at the operation step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-370-AC-1 | A `temporal`/`temporal_clause` node's body is a `quire.op.temporal.clause` application with one `temporal_profile` law, a `null` member and the six arguments over, clock, activation, captures, fairness and formula in that order; a body with arguments reordered, missing or of another term shape fails the published schema. | Test (TC-303) |
| FR-370-AC-2 | Each of the eight interval-capable operators, `holds`, the two temporal constants and the four connectives is encoded with its own `quire.op.temporal.*` identity, both V2 schemas enumerate those identities, the `temporal_formula` and `temporal_fairness` operator classes and the `temporal_interval` and `fairness` member kinds, and the operation catalog lists the `temporal` family. | Test (TC-303) |
| FR-370-AC-3 | An interval operator carries `{lower, upper}`; a lower-bounded operator carries `{lower, upper: null}` and an unbounded operator `{kind: "temporal_interval", interval: null}`; under `quire.temporal.infinite-trace/v1` all three are admitted; under `quire.temporal.timed/v1` `null` and a timed `[a,b]` are admitted and `[a,*]` and an integer-form `[a,b]` refuse `operation-member-mismatch`; under each of the three bounded profiles only an integer `[a,b]` is admitted and `null` or `[a,*]` refuses `operation-member-mismatch`; a `null` member on an interval-capable operator refuses `operation-member-mismatch`; and `lower > upper` refuses `invalid-value` after the profile fit. | Test (TC-303) |
| FR-370-AC-4 | `fair Op` and `fair weak whole Op` encode to byte-identical `quire.op.temporal.fair` applications, and a fairness member naming an operation absent from its declaring node's effective operations refuses `missing_declaration`/`missing-name`; a non-empty fairness argument is admitted under `quire.temporal.timed/v1` and `quire.temporal.infinite-trace/v1` and refuses `operation-member-mismatch` under each of the three bounded profiles. | Test (TC-303) |
| FR-370-AC-5 | A `temporal_formula` application at the body root of a node other than a `temporal`/`formula` node, a `quire.op.temporal.fair` application at the body root of a node other than a `temporal`/`fairness` node, and a `quire.op.temporal.clause` application at the body root of a node other than a `temporal`/`temporal_clause` node refuse `ill_typed`/`operator-ineligible` at that node; each of the three nested inside a node body or inside another term fails the V2 schema and refuses `malformed_wire` at the nested application. | Test (TC-303) |
| FR-370-AC-6 | Strong previous encodes as `quire.op.temporal.once` with interval `{lower: "1", upper: "1"}`. | Test (TC-303) |
| FR-370-AC-7 | The temporal step reports placement defects before clause defects, runs after the state step and before the operation step, and a `holds` operand of a non-Boolean family refuses `ill_typed`/`operator-ineligible` at the operation step. | Test (TC-303) |
| FR-370-AC-8 | Under `quire.temporal.timed/v1`, `[0, 3]`, `[0, 3)`, `(0, 3]` and `(0, 3)` in the timed form are four distinct admitted members, `{numerator: "1", denominator: "2"}` is an admitted bound, a timed interval `(3, 3]` and a timed `lower > upper` refuse `invalid_package`/`invalid-value`, a bound `{numerator: "2", denominator: "4"}` refuses `invalid_semantic_graph`, and the timed form under any other profile refuses `operation-member-mismatch`. | Test (TC-303) |
| FR-370-AC-9 | A negative interval bound, `{lower: "-1", upper: null}` or `{lower: "0", upper: "-1"}`, or a negative timed numerator fails the published schema and refuses `invalid_package`/`invalid-value` at the bound under every profile, before any temporal-step check, so it is never `operation-member-mismatch`. | Test (TC-303) |
| FR-370-AC-10 | A clause's profile check runs before its other checks: a law naming `quire.temporal.timed/v1` is known; a law naming the package's own `protocol_profile` selection refuses `unknown_profile`/`wrong-selection-role` and a law naming `quire.temporal.unknown/v1`, selected under no role, `unknown_profile`/`unsupported-selection`, each at `/semantic_graph/nodes/{n}/body/operation/laws/0/definition` and each before a defect in the same clause's `over` reference or intervals. | Test (TC-303) |
| FR-370-AC-11 | A fairness member naming an operation its `object_type` declaring node only inherits is admitted and resolves to the most derived redefinition; a name matching two effective operations refuses `ambiguous_declaration`/`ambiguous-name`; a declaring node no selected owner matches refuses `missing_declaration`/`missing-selection`; and a `declaration` naming a `systems_interface` model node or a non-model node refuses `invalid_model_binding`/`malformed-declaration` at the target. | Test (TC-303) |
| FR-370-AC-12 | A reference to a `temporal`/`formula` node from a state clause's condition, and a reference to a `temporal`/`fairness` node from a clause's formula argument, refuse `ill_typed`/`operator-ineligible` located at the referencing node; a diagnostic whose `details` reference a `temporal`/`formula`, `temporal`/`fairness` or `expression`/`case` node refuses `ill_typed`/`operator-ineligible` at `/diagnostics/entries/{e}/details/{d}`; a `details` term that is an application of the `temporal_formula`, `temporal_fairness` or `case` operator class refuses `ill_typed`/`operator-ineligible` at `/diagnostics/entries/{e}/details/{d}/operator`; a `details` term holding several such applications refuses at the first in document pre-order, so a `temporal_formula` application with a nested `case` argument refuses at `/diagnostics/entries/{e}/details/{d}/operator` and an aggregate of a `temporal_formula` application and then a `case` application at `/diagnostics/entries/{e}/details/{d}/members/0/operator`; and `details` referencing a `composite_type`/`union` or `value`/`union_value` node are admitted. | Test (TC-303) |
| FR-370-AC-13 | A `temporal` clause written with no `over` parameter refuses `invalid_syntax`/`unexpected-token` at the token standing where `over` is required, and produces no node; a `quire.op.temporal.clause` application whose argument 1 is absent, `null` or a non-reference term (such as the clause's clock text literal) fails the published schema. | Test (TC-180, TC-303) |

## Published artifacts and the operation catalog

`proposals/checked-package-v2/schema.json` and
`node-identity-preimage.schema.json` enumerate the `quire.op.temporal.*`
identities, the `temporal_formula` and `temporal_fairness` operator classes and
the `temporal_interval` and `fairness` member kinds, with the `null`, integer
and timed interval forms and their non-negative bound patterns, and fix the
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
- [FR-250](../temporal/FR-250-temporal-semantic-profile.md) owns the five
  profile labels the profile check matches, and
  [FR-255](../temporal/FR-255-bounded-temporal-interval.md) the integer and
  timed interval bounds and end openness.
- [FR-272](../foundation/FR-272-typed-diagnostic-cause.md) owns the
  `unknown_profile` causes `unsupported-selection` and
  `wrong-selection-role`.

## References

- quire-spec-language ADR-018 QS-11 and ADR-014 §13; Linear STD-99 and STD-131.
