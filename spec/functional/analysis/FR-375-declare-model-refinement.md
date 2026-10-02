---
id: FR-375
title: "Declare a refinement between two state models"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-151
    type: references
  - target: ix://agent-ix/quire-specification/FR-376
    type: references
  - target: ix://agent-ix/quire-specification/FR-377
    type: references
  - target: ix://agent-ix/quire-specification/FR-378
    type: references
---

# FR-375: Declare a refinement between two state models

## Description

When a unit declares a refinement between an abstract and a concrete state
model, the checker SHALL admit the declaration only when every abstract
population, every mapped abstract field and every concrete operation has
exactly one well-typed row, and SHALL emit the admitted declaration as one
node of the checked package.

## Inputs

A composed unit with two `model` imports (or one `model` import and one
dependency package that holds the abstract model), an infinite-trace profile
alias, and one `refinement` declaration.

## Outputs

An admitted refinement node in the checked package, or a located refusal
naming the offending row.

## Surface syntax

The declaration extends the shared grammar's `declaration` production. The
productions below use the shared grammar's EBNF conventions and its `ident`,
`member-name`, `type-name`, `operation-name`, `model-name`, `expr` and
`literal` productions.

```ebnf
declaration    = … | refinement ;
refinement     = 'refinement', ident, 'using', ident,
                 'abstract', ident, 'concrete', ident,
                 '{', { refinement-row }, '}' ;
refinement-row = population-row | object-row | step-row
               | history-row | fairness-row ;
population-row = 'population', model-name, 'from', model-name, ';' ;
object-row     = 'object', type-name, 'from', type-name,
                 '{', { field-row }, '}' ;
field-row      = member-name, '=', expr, ';' ;
step-row       = 'step', operation-name, '->', step-target, ';' ;
step-target    = operation-name, '(', expr, { ',', step-argument }, ')'
               | 'stutter'
               | 'any' ;
step-argument  = expr | '_' ;
history-row    = 'history', type-name, '.', member-name, ':',
                 type-name, '=', literal,
                 '{', { 'on', operation-name, ':', expr, ';' }, '}' ;
fairness-row   = ('assume' | 'ensure'), 'fair', 'weak',
                 [ 'each' | 'whole' ], operation-name, ';' ;
```

The first `ident` after `using` names an infinite-trace profile alias; the
idents after `abstract` and `concrete` name model aliases. In a step target
the first expression is the abstract receiver and each further argument is
one per declared abstract parameter, in parameter order. A `fairness-row`
with no granularity is `whole`. Inside a refinement declaration the
expression grammar also admits the population-valued forms of
[FR-376](FR-376-evaluate-population-valued-mapping-expressions.md).

## Behavior

**Selection.** The abstract model is a model alias of the unit or of a
dependency package selected in the unit's `dependency_selections`. The
**abstract subject** is the checked package holding the abstract model's
contract clauses; the **concrete subject** is the declaring package. A
profile alias that does not select `quire.temporal.infinite-trace/v1`
refuses `unknown_profile`/`wrong-selection-role` at the alias.

**Population rows.** Each abstract population has exactly one row naming one
concrete population as its source. An abstract population with no row
refuses `missing_declaration`/`missing-name`; one with two rows refuses
`invalid_model_binding`/`conflicting-binding`, naming both rows. A concrete
population that is no row's source is concrete-only state.

**Object rows.** Each abstract object type that an abstract population admits
is mapped from one concrete object type of that population's source. Each
`field-row` defines one abstract field. Its expression has the concrete
object as `self` and reads the concrete state through the forms a
state-clause body admits, history fields
([FR-378](FR-378-check-refinement-auxiliary-state.md)), pure function
application and the FR-376 forms. A field named in two rows refuses
`invalid_model_binding`/`conflicting-binding`, naming both rows. A field with
no row is a **hidden field** (FR-378).

**Typing and reference lifting.** A field row's expression conforms to the
abstract field's declared type under the conformance rule of
[FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md),
after reference lifting: a concrete reference to an object of a mapped type
denotes the abstract object with the same key, so `Reference<T_C>` conforms
to `Reference<T_A>` exactly when an object row maps `T_A` from `T_C`. A row
that does not conform refuses `ill_typed`/`type-mismatch` at the row's
expression.

**Step rows.** Each concrete operation has exactly one step row. An operation
with no row refuses `missing_declaration`/`missing-name`, naming the
operation; no row is supplied by default. An operation with two rows refuses
`invalid_model_binding`/`conflicting-binding`. An abstract-operation target
has one receiver expression and one argument per abstract parameter, each
typed against the abstract parameter type after reference lifting, over the
concrete receiver as `self`, the concrete arguments by parameter name and
the concrete pre-state; `_` leaves that argument open. A wrong argument
count refuses `ill_typed`/`operator-ineligible`. When both operations declare a
result, the concrete result type conforms to the abstract result type after
reference lifting, or the row refuses `ill_typed`/`type-mismatch`.

**History rows.** A history row names a concrete object type, a new field
name that the type does not declare, a bounded declared type and an initial
literal of that type, and at most one update per concrete operation of that
type. An update expression reads the step's receiver as `self` (post-state
fields), `pre(...)` of pre-state fields and of the history field, and the
step's arguments; it is typed against the history field's type. A name the
concrete type already declares refuses `invalid_model_binding`/
`conflicting-binding`; an unbounded type refuses `ill_typed`/`type-mismatch`;
two updates for one operation refuse `invalid_model_binding`/
`conflicting-binding`.

**Fairness rows.** `assume` rows form the concrete fairness set `F_C` and
name concrete operations; `ensure` rows form the abstract fairness set `F_A`
and name abstract operations. A row naming an operation of the other model
refuses `missing_declaration`/`missing-name`. Either set may be empty.

**Checked-package node.** An admitted declaration is one
`quire.checked-semantic-graph/v2` node with `node_tag: "relation"` and
`semantic_form: "refinement"` ([FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md)).
Its `semantic_type` is the concrete model declaration node, and its
`dependencies` hold every node its rows name. Its body is
`{"term": "refinement", "abstract", "profile", "populations", "objects",
"steps", "histories", "assume", "ensure"}`: `abstract` is the abstract
model's `PackageNodeKey`; `populations`, `objects`, `steps` and `histories`
hold one entry per row, each carrying its names as node keys and its
expressions as `SemanticTerm`s; a step entry's `target` is
`{"kind": "operation", "operation", "receiver", "arguments"}` with a
`{"term": "open"}` term for each `_`, or `{"kind": "stutter"}`, or
`{"kind": "any"}`; `assume` and `ensure` hold `{operation, granularity}`
entries with `granularity` exactly `each` or `whole`. Every array is sorted
ascending by the node-key digest of its first named declaration, then by
member name as UTF-8 bytes. The node enters the identity projection, so it
enters the `package_id` and the obligation identity of every item that
cites it.

**Claim kinds.** A request to check the whole declaration is one item of
kind `temporal-satisfaction`; each per-step simulation obligation is one item
of kind `operation-contract` ([FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md)).

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-375-AC-1 | A declaration with one population row per abstract population, one well-typed object row per admitted abstract type, one step row per concrete operation and well-typed history and fairness rows is admitted and emitted as one `relation`/`refinement` node whose body carries every row. | Test (TC-310) |
| FR-375-AC-2 | A concrete operation with no step row refuses `missing_declaration`/`missing-name` naming the operation, and the declaration is not admitted with any default target; the same declaration with `step … -> any;` written for it is admitted. | Test (TC-310) |
| FR-375-AC-3 | An abstract population with no row refuses `missing_declaration`/`missing-name`; a second row for one population, a field named twice, a second step row for one operation and a second history update for one operation each refuse `invalid_model_binding`/`conflicting-binding` naming both rows. | Test (TC-310) |
| FR-375-AC-4 | A field-row or step-argument expression whose type does not conform to the abstract type after reference lifting refuses `ill_typed`/`type-mismatch` at that expression; a `Reference<T_C>` expression for a `Reference<T_A>` field is admitted exactly when an object row maps `T_A` from `T_C`. | Test (TC-310) |
| FR-375-AC-5 | A history row whose name the concrete type already declares, whose type is unbounded, or whose update is ill-typed refuses with its named cause; an admitted history field appears in no concrete clause, frame or successor relation. | Test (TC-310) |
| FR-375-AC-6 | A fairness row with no granularity is read as `whole`; an `assume` row naming an abstract operation, or an `ensure` row naming a concrete operation, refuses `missing_declaration`/`missing-name`. | Test (TC-310) |
| FR-375-AC-7 | Changing any row of an admitted declaration changes the refinement node's id, the `package_id` and the obligation identity of an item that cites it; reordering rows in source does not. | Test (TC-310) |
| FR-375-AC-8 | A request to check the whole declaration requires `temporal-satisfaction`, each per-step simulation obligation requires `operation-contract`, and neither requires `refinement`. | Test (TC-310) |

## Dependencies

- [FR-322](../../objects/interfaces/FR-322-checked-package-artifact.md) owns
  the checked-package graph that carries the refinement node.
- [FR-151](../type-model/FR-151-resolve-conformance-redefinition-dispatch.md)
  owns the conformance rule used for row typing.
- [FR-376](FR-376-evaluate-population-valued-mapping-expressions.md) owns
  the population-valued expression forms admitted inside the declaration.
- [FR-377](FR-377-decide-model-refinement.md) owns what an admitted
  declaration means; [FR-378](FR-378-check-refinement-auxiliary-state.md)
  owns history and hidden fields.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) owns the
  claim-kind vocabulary.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns
  the productions this requirement extends.

## References

- Design: quire-spec-language ADR-020 (RM-1 to RM-7, AX-1, QS-1, QS-4,
  QS-5, RU-1).
- Tickets: Linear STD-133; paired QSL-367.
