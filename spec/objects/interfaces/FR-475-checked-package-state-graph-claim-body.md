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
  - `from` — for `always possible`, a `reference` to the `expression` node of
    `Q`; an `always possible` claim written without `from` references the
    `expression` node of the constant `true`, so it encodes identically to the
    same claim written `from (true)`. For `unique path`, a `reference` to the
    `expression` node of `X`. For `possible`, the empty aggregate;
  - `target` — a `reference` to the `expression` node of `P` for `possible`
    and `always possible`, and of `Y` for `unique path`.
- **predicate nodes** — each `from` and `target` target is an `expression`
  node whose `semantic_type` is the `scalar_type`/`boolean` node and whose
  body is the predicate's Boolean term under
  [FR-322](FR-322-checked-package-artifact.md)'s body grammar. A predicate is
  a state predicate: it reads no operation anchor, result or pre-state.
- **semantic_type** — the `scalar_type`/`boolean` node.
- **declaration** — `{qualified_name}`, the claim's declared name.
- **occurrences** — every occurrence has role `claim`, one per declaration
  that checks to this node, so a provider request names a state-graph item as
  `claim {node, occurrence}`
  ([FR-331](FR-331-backend-provider-envelope.md)).
- **dependencies** — exactly the unique, digest-ascending set of the body's
  `reference` targets: the `over` parameter node when present, the `from`
  node when present and the `target` node.

The body carries no temporal profile, clock, activation, capture or fairness
member: a state-graph claim has none (FR-390).

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
the same checked meaning share one node id and one obligation identity. The
[FR-393](FR-393-state-graph-evidence.md) envelope binds this identity.

## Reader order

The state-graph step runs after the state step
([FR-341](FR-341-checked-package-state-clause-body.md) "Reader order") and
before the operation step, over every `claim`/`state_graph` node in ascending
node-id digest order. For one node, the checks run in this order and the first
failing check is its defect:

1. **Shape.** The body is an aggregate of exactly the four bindings in order,
   else `invalid_semantic_graph` at `/semantic_graph/nodes/{n}/body`.
2. **Kind.** `kind` is one of the three text literals, else
   `invalid_package`/`invalid-value` at the `kind` binding.
3. **Form fit.** `from` is the empty aggregate exactly when `kind` is
   `possible`, and `target` is a `reference`, else
   `invalid_semantic_graph` at the binding that breaks the rule.
4. **Targets**, in body order (`over`, `from`, `target`): an
   `over` target that is not a `value`/`parameter` node, or a `from` or
   `target` target that is not an `expression` node of `semantic_type`
   `scalar_type`/`boolean`, refuses `ill_typed`/`operator-ineligible`. The
   locus is the target.

A `dependencies` list that differs from the body's targets is FR-322's
graph-shape refusal, `invalid_semantic_graph` at the node's `/dependencies`,
which precedes this step.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-475-AC-1 | A `possible`, an `always possible` and a `unique path` claim each produce one `claim`/`state_graph` node whose body has exactly the bindings `kind`, `over`, `from` and `target` in that order, with `kind` `possible`, `always-possible` and `unique-path` respectively, `from` the empty aggregate for `possible` only, `semantic_type` the `scalar_type`/`boolean` node, every occurrence of role `claim`, and `dependencies` exactly the body's targets in digest order. | Test (TC-960) |
| FR-475-AC-2 | A claim written without `over` encodes `over` as the empty aggregate; an `always possible` claim written without `from` and the same claim written `from (true)` have one node id. | Test (TC-960) |
| FR-475-AC-3 | `possible P` and `always possible P` over one `P` have different node ids and different obligation identities; two instances of one `over` claim with different universe keys have different obligation identities; a claim that differs only in whitespace and comments has the same node id and obligation identity. | Test (TC-960) |
| FR-475-AC-4 | A body with `from` and `target` swapped in order refuses `invalid_semantic_graph`; `kind` `eventually-possible` refuses `invalid_package`/`invalid-value`; a `possible` node whose `from` is a reference refuses `invalid_semantic_graph`; a `target` naming a `value`/`parameter` node refuses `ill_typed`/`operator-ineligible` at the target; a node whose `dependencies` omit its `target` refuses `invalid_semantic_graph` at `/dependencies`. | Test (TC-960) |

## Published artifacts

`proposals/checked-package-v2/schema.json` admits `state_graph` among the
`claim` node's `semantic_form` values.

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the checked-package node
  form, node keys, `dependencies` and the reader's refusal order.
- [FR-390](../../functional/analysis/FR-390-analyze-state-graph-claims.md)
  states the three forms and their predicates; [FR-391](../../functional/analysis/FR-391-settle-state-graph-claims.md)
  settles each item; [FR-393](FR-393-state-graph-evidence.md) carries the
  evidence whose envelope binds this identity.
- [FR-402](FR-402-hyperproperty-obligation-identity.md) is the sibling body of
  the `claim`/`hyperproperty` node.

## References

- STD-267 (this requirement); quire-spec-language ADR-022 SG-1 to SG-3.
