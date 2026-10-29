---
id: FR-475
title: "Checked-package state-graph claim body and obligation identity"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-390
    type: references
  - target: ix://agent-ix/quire-specification/FR-391
    type: references
  - target: ix://agent-ix/quire-specification/FR-393
    type: references
  - target: ix://agent-ix/quire-specification/FR-331
    type: references
  - target: ix://agent-ix/quire-specification/FR-402
    type: references
  - target: ix://agent-ix/quire-specification/FR-370
    type: references
---

# FR-475: Checked-package state-graph claim body and obligation identity

## Description

When a checked package carries a `possible`, `always possible` or `unique
path` claim ([FR-390](../../functional/analysis/FR-390-analyze-state-graph-claims.md)),
the producer SHALL emit it as one `quire.checked-semantic-graph/v2` node with
`node_tag: "claim"` and `semantic_form: "state_graph"`, encode its `body` as
the aggregate below, and compute each of its items' obligation identity over
every member that changes the claim's meaning. The reader SHALL refuse a node
whose rules below do not hold, with the first refusal in the reader order
below.

## Properties

- **body** — `{term: "aggregate", members: [binding("kind", …),
  binding("over", …), binding("from", …), binding("target", …)]}`, in this
  order:
  - `kind` — a `literal` text term, `possible`, `always-possible` or
    `unique-path`;
  - `over` — a `reference` to the `value`/`parameter` node of the claim's
    `over` parameter, or the empty aggregate `{term: "aggregate", members:
    []}` for a claim written without one;
  - `from` — for `always possible`, a `reference` to the predicate node of
    `Q`; an `always possible` claim written without `from` references the
    node of the constant `true`, so it encodes identically to the same claim
    written `from (true)`. For `unique path`, a `reference` to the predicate
    node of `X`. For `possible`, the empty aggregate;
  - `target` — a `reference` to the predicate node of `P` for `possible`
    and `always possible`, and of `Y` for `unique path`.
- **predicate nodes** — each `from` and `target` target is exactly one of
  two node forms, with `semantic_type` the `scalar_type`/`boolean` node: an
  `expression` node, the predicate's Boolean term, or a `value`/`literal`
  node, a constant predicate such as `true`, as
  [FR-322](FR-322-checked-package-artifact.md)'s body grammar encodes it. No
  other node form is a predicate, a Boolean-typed `temporal`, `state` or
  `claim` node or a `function` node included. That a predicate reads no operation anchor, result or
  pre-state is the checker's admission rule (FR-390 "Admission"); the reader
  does not check it again.
- **semantic_type** — the `scalar_type`/`boolean` node.
- **declaration** — none. The node is content-addressed, as a
  `temporal`/`temporal_clause` node is: it carries no `declaration` and no
  `owner`, two claims with equal checked content share one node, and each
  declaration is one `claim` occurrence of it, told apart by its occurrence
  ordinal.
- **occurrences** — every occurrence has role `claim`, one per declaration
  that checks to this node, so a provider request names a state-graph item as
  `claim {node, occurrence}`
  ([FR-331](FR-331-backend-provider-envelope.md)).
- **dependencies** — exactly the unique, node-id-ascending set of the body's
  `reference` targets: the `over` parameter node when present, the `from`
  node when present and the `target` node.

The body carries no temporal profile, clock, activation, capture, fairness
or model subject member: a state-graph claim has none (FR-390), and its model
subject is the request item's subject.

## Obligation identity

The obligation identity of a state-graph item binds:

- the model subject: its initial states, universes and proof bounds;
- the claim's `kind`;
- the `from` and `target` predicates;
- the `over` parameter's instance: the parameter's universe key, or none for
  a claim without `over`.

Two claims that differ only in `kind`, such as `possible P` and `always
possible P` over the same `P`, have different node ids and different
obligation identities. Two claims that differ only in source spelling with
the same checked meaning share one node id and one obligation identity. One
claim node requested over two subjects that differ in initial states or proof
bounds keeps one node id and has two obligation identities. The
[FR-393](FR-393-state-graph-evidence.md) envelope binds this identity.

## Reader order

The state-graph step is the first of the body steps that follow the state
step, in the one order
[FR-322](FR-322-checked-package-artifact.md) "Body steps" fixes. It visits
every `claim`/`state_graph` node in ascending node-id order. For one
node, the checks run in this order and the first failing check is its
defect:

1. **Shape.** The body is an aggregate of exactly the four bindings in order,
   else `invalid_semantic_graph` at `/semantic_graph/nodes/{n}/body`.
2. **Kind.** `kind` is one of the three text literals, else
   `invalid_package`/`invalid-value` at
   `/semantic_graph/nodes/{n}/body/members/0`.
3. **Form fit.** `over` is a `reference` or the empty aggregate; `from` is
   the empty aggregate when `kind` is `possible` and a `reference`
   otherwise; `target` is a `reference`. Else `invalid_semantic_graph` at
   the binding that breaks the rule.
4. **Targets**, in body order (`over`, `from`, `target`): an `over` target
   that is not a `value`/`parameter` node refuses `invalid_model_binding`/
   `malformed-declaration`, as [FR-370](FR-370-checked-package-temporal-clause-body.md)
   refuses a temporal clause's `over`; a `from` or `target` target that is
   neither an `expression` node nor a `value`/`literal` node, or whose
   `semantic_type` is not the `scalar_type`/`boolean` node, refuses
   `ill_typed`/`operator-ineligible`.
   The locus is the target.

A `dependencies` list that differs from the body's targets is FR-322's
graph-shape refusal, `invalid_semantic_graph` at the node's `/dependencies`,
which precedes this step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-475-AC-1 | A `possible`, an `always possible` and a `unique path` claim each produce one `claim`/`state_graph` node whose body has exactly the bindings `kind`, `over`, `from` and `target` in that order, with `kind` `possible`, `always-possible` and `unique-path` respectively, `from` the empty aggregate for `possible` only, `semantic_type` the `scalar_type`/`boolean` node, no `declaration` and no `owner`, every occurrence of role `claim`, and `dependencies` exactly the body's targets in node-id order. | Test (TC-960) |
| FR-475-AC-2 | A claim written without `over` encodes `over` as the empty aggregate; `possible Anything using v { true }` references the Boolean `value`/`literal` node of `true` as its `target` and is admitted; an `always possible` claim written without `from` and the same claim written `from (true)` have one node id, `from` referencing the `true` literal node; two declarations of one claim with different names share one node and carry two `claim` occurrences with ordinals 0 and 1. | Test (TC-960) |
| FR-475-AC-3 | `possible P` and `always possible P` over one `P` have different node ids and different obligation identities; two instances of one `over` claim with different universe keys have different obligation identities; one claim requested over two subjects with different initial states keeps one node id and has two obligation identities; a claim that differs only in whitespace and comments has the same node id and obligation identity. | Test (TC-960) |
| FR-475-AC-4 | A body with `from` and `target` swapped in order refuses `invalid_semantic_graph`; `kind` `eventually-possible` refuses `invalid_package`/`invalid-value` at `/semantic_graph/nodes/{n}/body/members/0`; a `possible` node whose `from` is a reference, a `unique-path` node whose `from` is the empty aggregate, and an `over` that is a text literal each refuse `invalid_semantic_graph`; with `dependencies` rewritten to the body's targets, a `target` naming a `value`/`parameter` node, and a `target` naming a `temporal`/`temporal_clause` node, each refuse `ill_typed`/`operator-ineligible` at the target; a node whose `dependencies` omit its `target` refuses `invalid_semantic_graph` at `/dependencies`. | Test (TC-960) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `state_graph` among the
`claim` node's `semantic_form` values.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the checked-package node
  form, node ids, `dependencies` and the reader's refusal order.
- [FR-390](../../functional/analysis/FR-390-analyze-state-graph-claims.md)
  states the three forms and their predicates; [FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md)
  settles each item; [FR-393](FR-393-state-graph-evidence.md) carries the
  evidence whose envelope binds this identity.
- [FR-402](FR-402-hyperproperty-obligation-identity.md) is the sibling body of
  the `claim`/`hyperproperty` node.

## References

- STD-267 (this requirement); quire-spec-language ADR-022 SG-1 to SG-3.
