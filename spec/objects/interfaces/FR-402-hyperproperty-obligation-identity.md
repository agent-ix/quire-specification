---
id: FR-402
title: "Hyperproperty claim body and obligation identity"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-395
    type: references
  - target: ix://agent-ix/quire-specification/FR-397
    type: references
  - target: ix://agent-ix/quire-specification/FR-191
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
---

# FR-402: Hyperproperty claim body and obligation identity

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "claim"` and
`semantic_form: "hyperproperty"`, the producer SHALL encode its `body` as the
aggregate below and compute each of its items' obligation identity over every
member that changes the clause's meaning.

## Properties

- **body** — `{term: "aggregate", members: [binding("domain", …),
  binding("subjects", …), binding("parameters", …), binding("quantifiers", …),
  binding("match", …), binding("align_skip", …), binding("body", …)]}`, in
  this order:
  - `domain` — a `literal` text term `behaviours`, or a `reference` to the
    declared finite trace population with its bound for an FR-191 finite
    domain;
  - `subjects` — an aggregate of one `binding(alias, reference(M))` per
    distinct model alias the quantifiers name, in order of first naming. Each
    target `M` is the alias's model declaration node: `node_tag: "model"`,
    with its selected `ModelOwner` as
    [FR-322](FR-322-checked-package-artifact.md) defines; empty for a finite
    domain;
  - `parameters` — an aggregate of one `binding(name, reference(T))` per
    object parameter, in declaration order, each referencing the `model` /
    `object_type` declaration node whose population it binds;
  - `quantifiers` — an aggregate of one `binding(variable, aggregate)` per
    quantifier, in source order, whose aggregate holds a `literal` text term
    `forall` or `exists`, a `literal` text term naming the alias (absent for a
    finite domain), and an aggregate of references to its fairness
    constraints in source order (empty for the empty set). Each reference
    targets that constraint's `temporal`/`fairness` node under
    [FR-370](FR-370-checked-package-temporal-clause-body.md);
  - `match` — an aggregate of references to the match's conjuncts in source
    order; empty with no `match` block;
  - `align_skip` — an aggregate of references to the `state`/`operation_anchor`
    nodes of the listed operations, in source order; empty with no list;
  - `body` — for a `behaviours` domain, a reference to the body's
    `temporal`/`formula` node; for a finite trace domain, a reference to the
    body's Boolean `expression` node. A model-domain body is not a reference
    to a `temporal`/`temporal_clause` node.
- **dependencies** — every node the body references.

## Obligation identity

The obligation identity of a hyper item binds:

- every subject in alias order, with its initial states, universes and proof
  bounds;
- the quantifier prefix, each variable with its quantifier and its alias, in
  order;
- each variable's fairness set;
- the match, as its conjuncts;
- the `align skip` list;
- the body;
- the object parameters' instance: each parameter's universe key.

For a `behaviours` domain, each `subjects` reference targets the `model` node
selected for its alias: its `ModelOwner` resolves to the model selection bound
to that alias under [FR-322](FR-322-checked-package-artifact.md). A target
that is not a `model` node for that alias refuses
`invalid_model_binding`/`malformed-declaration` at the target. The
`body` reference targets a `temporal`/`formula` node, and each quantifier's
fairness references target `temporal`/`fairness` nodes.
[FR-370](FR-370-checked-package-temporal-clause-body.md) admits those
references only in the slots defined here. For a finite trace domain,
`subjects` is empty, every quantifier has an empty fairness set, and the
`body` reference targets an `expression` node.

Reversing two quantifiers, moving a fairness constraint from one variable to
another, renaming which alias a variable ranges over, or changing the
instance gives a different obligation identity (FR-191-AC-2, FR-191-AC-4).
A form and a compiler-detected copy-swap symmetry are functions of the checked
clause, so they add no identity member.

The model claim of a step relation binds every subject, the execution
variables in order with their operations, and the body; its node is the
[FR-477](FR-477-checked-package-step-relation-claim-body.md)
`claim`/`step_relation` node. Its code claim binds
the same members plus each operation's implementation binding, and differs
from the model claim by its capability kind, so the two never share an
identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-402-AC-1 | A `claim`/`hyperproperty` node's body has exactly the seven bindings in order, with subjects in order of first naming and quantifiers in source order. | Test (TC-346) |
| FR-402-AC-2 | Reversing two quantifiers, moving a fairness constraint between variables, changing a variable's alias, changing the match or the `align skip` list, and changing the instance key each change the obligation identity. | Test (TC-346) |
| FR-402-AC-3 | Two clauses that differ only in source spelling with the same checked meaning share one obligation identity. | Test (TC-346) |
| FR-402-AC-4 | A step relation's model claim and code claim have different obligation identities. | Test (TC-343, TC-346) |
| FR-402-AC-5 | For a `behaviours` domain, each subject reference targets its alias's selected `model` node, the body reference targets a `temporal`/`formula` node, and each quantifier's fairness references target `temporal`/`fairness` nodes; these placements are admitted by FR-370. A subject reference that targets no model node selected for its alias refuses `invalid_model_binding`/`malformed-declaration` at the target. For a finite trace domain, `subjects` and each fairness set are empty and the body reference targets a Boolean `expression` node. | Test (TC-346) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the checked-package node
  form and its identity rules.
- [FR-395](../../functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md)
  and [FR-397](../../functional/analysis/FR-397-check-step-relations-over-model-operations.md)
  state the clause meanings this identity binds.
- [FR-191](../../functional/analysis/FR-191-analyze-relational-hyperproperties.md)
  requires quantifier order to be preserved in identity.
- [FR-370](FR-370-checked-package-temporal-clause-body.md) defines the
  temporal formula and fairness nodes referenced by model-domain claims.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design).
