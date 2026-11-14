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
member that changes the clause's meaning, and the reader SHALL refuse a node
whose rules below do not hold, with the first refusal in the reader order
below.

## Properties

- **body** — `{term: "aggregate", members: [binding("domain", …),
  binding("subjects", …), binding("parameters", …), binding("quantifiers", …),
  binding("match", …), binding("align_skip", …), binding("body", …)]}`, in
  this order:
  - `domain` — a `literal` text term `behaviours`, or a `reference` to the
    declared finite trace population with its bound for an FR-191 finite
    domain;
  - `subjects` — an aggregate of one `binding(alias, reference(R))` per
    distinct model alias the quantifiers name, in order of first naming.
    `R` is the `model`/`model_root` node
    ([FR-322](FR-322-checked-package-artifact.md) "Owner member schema")
    of the domain package the alias selects: one node per selected package
    identity, whatever the alias's spelling and whatever object types its
    object parameters name, so an alias that binds no object parameter has a
    subject too. Empty for a finite domain;
  - `parameters` — an aggregate of one `binding(name, reference(T))` per
    object parameter, in declaration order, each referencing the `model`
    declaration node of the object type whose population it binds;
  - `quantifiers` — a Tuple of FR-322 "Body grammar": an aggregate of one
    **quantifier Group** per quantifier, in source order. A quantifier Group
    is an aggregate whose members are, in this order:
    1. `binding("variable", …)`, a `literal` text term, the trace variable's
       name;
    2. `binding("quantifier", …)`, a `literal` text term `forall` or
       `exists`;
    3. `binding("alias", …)`, a `literal` text term naming the alias the
       variable ranges over; absent for a finite domain;
    4. one `reference` per constraint of the variable's resolved fairness
       set, in fairness-set order, to the `temporal`/`fairness` node whose
       body root is that constraint's `quire.op.temporal.fair` application
       ([FR-370](FR-370-checked-package-temporal-clause-body.md)); none for
       the empty set and none for a finite domain.
  - `match` — an aggregate of one `reference` per conjunct of the match, in
    source order, to the conjunct's Boolean `expression` node; a constant
    conjunct such as `true` is a `reference` to its Boolean
    `value`/`literal` node, never an inline literal. Empty with no `match` block;
  - `align_skip` — an aggregate of references to the `state`/`operation_anchor`
    nodes of the listed operations, in source order; empty with no list;
  - `body` — for a `behaviours` domain, a `reference` to the body's
    **body wrapper** ("Body wrapper"), a `temporal`/`temporal_clause` node;
    for a finite domain, a `reference` to the body's Boolean `expression`
    node.
- **dependencies** — every node the body references.

Every member of a quantifier Group is a Leaf or a `binding` whose value is a
Leaf, so the deepest path in the body is a Member `binding`, a Tuple, a
Group, a `binding` and a Leaf, the longest chain FR-322's grammar admits. A
constraint is never an inline aggregate: its kind, granularity and target
are the members of its `temporal`/`fairness` node, resolved and checked by
FR-370 "Fairness resolution" as for a temporal clause.

## Body wrapper

The body wrapper is the `temporal`/`temporal_clause` node of FR-370 that
carries a `behaviours`-domain body:

- its `temporal_profile` law is the clause's profile;
- its `formula` argument references the `temporal`/`formula` node of the
  body's root, so every formula node of the body is referenced from a clause
  formula argument or from another formula, as FR-370 places it;
- its `fairness` argument is the empty aggregate: each variable's fairness
  is in its quantifier Group, never in the wrapper;
- its `over` argument references one of the body's slot parameter nodes
  ("Slot parameters"); a consumer reads the body's per-trace state from the
  slot parameters, never from `over`; its clock and activation are the
  producer's fixed `model-steps` and `origin`, and its captures are empty.

The wrapper is not a single-trace temporal claim. A
`temporal`/`temporal_clause` node that is the `body` target of a
`claim`/`hyperproperty` node is a body wrapper: its `claim` occurrences and
its `temporal_clause_names` and `temporal_bindings` rows (FR-322) are the
hyper clause's, and a consumer that lists or resolves single-trace temporal
claims SHALL exclude it: a producer writes no
[FR-368](../../functional/temporal/FR-368-negotiate-temporal-model-check-items.md)
`formula` item whose `claim.node` is a body wrapper, since the wrapper is
claimed only by the hyper clause's own items.

## Slot parameters

A `behaviours`-domain body and a match conjunct read the state and the step
labels of each trace through `value`/`parameter` nodes, its **slot
parameters**. A slot parameter's `name` is an ASCII identifier, as FR-322's
`value`/`parameter` body requires. The producer SHALL write it by the
encoding below, and a consumer SHALL decode a slot's role and fields from
the name by the same encoding:

- `esc(x)` maps each byte of the UTF-8 text `x`: an ASCII letter or digit is
  itself, `_` is `__`, and every other byte is `_x` followed by its two
  uppercase hexadecimal digits.
- A name is a tag, then `_s` and `esc(field)` for each field, in order:
  - the state of object parameter `p` in trace `t`: `slot_s<esc(p)>_s<esc(t)>`,
    so `v @ a` is `slot_sv_sa`;
  - the operation label `t.step.op`: `op_s<esc(t)>`;
  - the receiver `t.step.receiver`: `rcv_s<esc(t)>`;
  - the argument `p` of operation `T::n` in `t.step.T::n.p`:
    `arg_s<esc(t)>_s<esc(T)>_s<esc(n)>_s<esc(p)>`, so `a.step.Vault::step.i`
    is `arg_sa_sVault_sstep_si`.

The tags `slot`, `op`, `rcv` and `arg` hold no `_`, so a name's tag, and
with it its field count, is the text before its first `_`, and every name
begins with a letter. In the rest of
the name every `_` starts exactly one of `__`, `_x` plus two hexadecimal
digits, or `_s`, none a prefix of another, so the name splits into its
fields uniquely and each field decodes uniquely. Two distinct tags or field
lists therefore never give one name, whatever underscores or non-ASCII
characters the source names hold. A slot parameter's node identity is its
name, level and type, as for every `value`/`parameter` node (FR-322).

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

Reversing two quantifiers, moving a fairness constraint from one variable to
another, renaming which alias a variable ranges over, or changing the
instance gives a different obligation identity (FR-191-AC-2, FR-191-AC-4).
A form and a compiler-detected copy-swap symmetry are functions of the checked
clause, so they add no identity member. A trace variable's name is part of the
body (its Group's `variable` and its slot names), so renaming a trace variable
changes the node and the obligation identity: it is not a difference of
spelling under FR-402-AC-3.

The model claim of a step relation binds every subject, the execution
variables in order with their operations, and the body; its node is the
[FR-477](FR-477-checked-package-step-relation-claim-body.md)
`claim`/`step_relation` node. Its code claim binds
the same members plus each operation's implementation binding, and differs
from the model claim by its capability kind, so the two never share an
identity.

## Reader order

The hyperproperty step is a body step of
[FR-322](FR-322-checked-package-artifact.md) "Body steps": it runs after the
step-relation step and before the hybrid step. FR-370's temporal step, which
runs earlier, has already checked the placement of every reference to a
`temporal`/`fairness` or `temporal`/`formula` node, including those in a
quantifier Group, and every body wrapper as a temporal clause. The step
visits every `claim`/`hyperproperty` node in ascending node-id order. For one
node, with `B` = `/semantic_graph/nodes/{n}/body`, the checks run in this
order and the first failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the seven bindings in
   order, and each `quantifiers` member is a quantifier Group whose bindings
   are `variable`, `quantifier` and, for a `behaviours` domain, `alias`, in
   that order and followed by `reference`s only; else
   `invalid_semantic_graph` at the first member that breaks the layout. A
   `quantifier` value other than the text `forall` or `exists` refuses
   `invalid_package`/`invalid-value` at that binding's value. For a finite
   domain, a `subjects` member, an `alias` binding or a fairness reference
   refuses `invalid_semantic_graph` at that member.
2. **Subjects.** A `subjects` target that is not a `model`/`model_root`
   node refuses `invalid_model_binding`/`malformed-declaration` at the
   target. A quantifier Group's `alias` naming no `subjects` binding
   refuses `missing_declaration`/`missing-name` at that `alias` binding,
   and a `subjects` binding no Group names refuses `invalid_semantic_graph`
   at that binding.
3. **Body.** For a `behaviours` domain, a `body` target that is not a
   `temporal`/`temporal_clause` node refuses `ill_typed`/
   `operator-ineligible` at the hyperproperty node, and a body wrapper whose
   `fairness` argument is not empty refuses `invalid_semantic_graph` at the
   wrapper's `/semantic_graph/nodes/{w}/body/arguments/4`. For a finite
   domain, a `body` target that is not an `expression` node whose
   `semantic_type` is the `scalar_type`/`boolean` node refuses
   `ill_typed`/`non-boolean-root` at the target.
4. **Fairness**, Group by Group and reference by reference in body order.
   Every refusal of this step is located at the reference,
   `B/members/3/value/members/{g}/members/{m}`, and for one reference the
   checks run in this order:
   1. a reference whose target is not a `temporal`/`fairness` node refuses
      `ill_typed`/`operator-ineligible`, and the reader admits no Group
      reference to any other node form (a `temporal`/`temporal_clause`, an
      `expression`, a `value` or a `model` node among them); a reference to
      a `temporal`/`formula` node never reaches this check, since FR-370's
      temporal step has already refused it as misplaced;
   2. its fairness node's `target` that is not an `operation` target refuses
      `invalid_model_binding`/`malformed-declaration`;
   3. FR-370 "Fairness resolution" steps 1 to 3 run on the fairness node as
      on a clause's fairness member, with their codes and causes;
   4. a declaring node whose `ModelOwner` is not of the package whose
      `model_root` the Group's alias names refuses
      `invalid_model_binding`/`malformed-declaration`;
   5. under a body wrapper whose profile is one whose FR-370 profile fit
      admits only an empty fairness argument, the reference refuses
      `invalid_package`/`operation-member-mismatch`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-402-AC-1 | A hyperproperty claim node's body has exactly the seven bindings in order, with subjects in order of first naming and quantifiers in source order. | Test (TC-346) |
| FR-402-AC-2 | Reversing two quantifiers, moving a fairness constraint between variables, changing a variable's alias, changing the match or the `align skip` list, and changing the instance key each change the obligation identity. | Test (TC-346) |
| FR-402-AC-3 | Two clauses that differ only in source spelling with the same checked meaning share one obligation identity; renaming a trace variable is not spelling and gives a different node and obligation identity. | Test (TC-346) |
| FR-402-AC-4 | A step relation's model claim and code claim have different obligation identities. | Test (TC-343, TC-346) |
| FR-402-AC-5 | A `behaviours` clause's `quantifiers` binding is a Tuple of one quantifier Group per quantifier, each Group binding `variable`, `quantifier` and `alias` to text literals and then holding one `reference` per fairness constraint to its `temporal`/`fairness` node; with `fair weak V::Vault::step` on `a` of TC-337 HB-01, `a`'s Group holds one reference and `b`'s none; the body passes strict wire validation of FR-322's grammar with no `malformed_wire`, and the reader admits the package. A finite-domain clause's Groups bind `variable` and `quantifier` only and its `subjects` is empty. | Test (TC-346) |
| FR-402-AC-6 | Each `subjects` binding references the `model`/`model_root` node of the package its alias selects: two aliases' clauses over one object type of each package reference two roots, and a clause whose alias binds no object parameter has that alias's root as its subject; a `subjects` target that is a `model`/`object_type` node refuses `invalid_model_binding`/`malformed-declaration` at the target; a quantifier `alias` naming no subject refuses `missing_declaration`/`missing-name` at the `alias` binding; a `subjects` binding no Group names, and a body of six bindings, each refuse `invalid_semantic_graph`; and a finite-domain body with a `subjects` member, an `alias` binding or a fairness reference refuses `invalid_semantic_graph` at that member. | Test (TC-346) |
| FR-402-AC-7 | Each quantifier fairness refusal is located at the reference `/semantic_graph/nodes/{n}/body/members/3/value/members/{g}/members/{m}`: a fairness node whose target is a `protocol_root` target refuses `invalid_model_binding`/`malformed-declaration`; one whose `operation` target names no effective operation refuses `missing_declaration`/`missing-name`; one whose declaring node belongs to another alias's package refuses `invalid_model_binding`/`malformed-declaration`; and a non-empty quantifier fairness set under a body wrapper whose profile is `quire.temporal.event-position.false-extension/v1` refuses `invalid_package`/`operation-member-mismatch`. A `quantifier` value `some` refuses `invalid_package`/`invalid-value` and a Group with `quantifier` before `variable` refuses `invalid_semantic_graph`. | Test (TC-346) |
| FR-402-AC-8 | A `behaviours` clause's `body` references a body wrapper whose `formula` argument references the body's root formula node and whose `fairness` argument is empty; the request the producer writes for the clause holds no FR-368 `formula` item whose `claim.node` is the wrapper; a `body` referencing a `temporal`/`formula` node directly refuses `ill_typed`/`operator-ineligible` at the hyperproperty node; a wrapper with a non-empty `fairness` argument refuses `invalid_semantic_graph` at that argument; and a finite-domain `body` referencing an integer-typed `expression` node refuses `ill_typed`/`non-boolean-root` at the target. | Test (TC-346) |
| FR-402-AC-9 | Every slot parameter name matches `[A-Za-z_][A-Za-z0-9_]*` and decodes to its tag and fields: `v @ a` is `slot_sv_sa`, `a.step.op` is `op_sa` and `a.step.Vault::step.i` is `arg_sa_sVault_sstep_si`; the slots for object parameter `a_b` in trace `c` and for `a` in trace `b_c` are `slot_sa__b_sc` and `slot_sa_sb__c`, which differ; a byte outside ASCII letters, digits and `_` is `_x` and two uppercase hexadecimal digits; and names built from every pair of distinct tags and field lists over a set including `_`, `s`, `x` and digit-leading names are pairwise distinct. | Test (TC-346) |
| FR-402-AC-10 | A quantifier Group `reference` whose target is a node of the graph other than a `temporal`/`fairness` node is refused, never admitted: with `a`'s fairness reference of TC-346 HI-08 retargeted to the clause's body wrapper `temporal`/`temporal_clause` node, to a Boolean `expression` node, or to `Vault`'s `model`/`object_type` node, each refuses `ill_typed`/`operator-ineligible` at `/semantic_graph/nodes/{n}/body/members/3/value/members/0/members/3`, before any fairness-target or resolution check of that reference. | Test (TC-346) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the checked-package node
  form, its body grammar and its identity rules.
- [FR-370](FR-370-checked-package-temporal-clause-body.md) owns the temporal
  clause, formula and fairness nodes the body references and their
  placement.
- [FR-395](../../functional/analysis/FR-395-analyze-hyperproperties-over-behaviours.md)
  and [FR-397](../../functional/analysis/FR-397-check-step-relations-over-model-operations.md)
  state the clause meanings this identity binds.
- [FR-191](../../functional/analysis/FR-191-analyze-relational-hyperproperties.md)
  requires quantifier order to be preserved in identity.

## References

- STD-136 (this requirement); QSL-370 and its ADR-023 (design); QSL-1093
  (the grammar-legal quantifier shape, the subject node, the body wrapper
  and the slot names).
