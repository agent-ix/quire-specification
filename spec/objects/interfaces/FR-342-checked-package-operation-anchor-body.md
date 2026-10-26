---
id: FR-342
title: "Checked-package operation anchor body"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-340
    type: references
  - target: ix://agent-ix/quire-specification/FR-341
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-342: Checked-package operation anchor body

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "state"` and
`semantic_form: "operation_anchor"`, the producer SHALL encode its `body` as
an aggregate of three or four bindings, in this order: `context`, the
object type, process or state machine declaring the operation; `operation`,
the operation's name; and `frame`, the operation's `state`/`frame` node. The reader SHALL refuse a
node whose joins below do not hold with the first refusal in the order
below.

## Properties

- **body** — `{term: "aggregate", members: [binding("context",
  reference(C)), binding("operation", literal), binding("frame",
  reference(F))]}`:
  - `context` — a `reference` to the `model`/`object_type`,
    `model`/`process` or `model`/`state_machine` node `C` that declares the
    operation;
  - `operation` — a `literal` term with `value_kind: "text"`, whose `type`
    is the `scalar_type`/`text` node and whose `value` is the operation's
    name, an identifier;
  - `frame` — a `reference` to the operation's `state`/`frame` node `F`
    ([FR-340](FR-340-checked-package-frame-location.md)).
- **execution point** — an optional fourth `binding` named
  `execution_point`, following `frame`, whose value is FR-322's closed
  `ExecutionPoint`. A clause-bound anchor carries the same accepted point as
  its `state_clause`; a protocol-only anchor omits it. The point is part of
  the body, so it participates in the structural-node preimage and identity.
- **semantic_type** — `C`, the context's node. A reference to the anchor
  therefore has family `object`, which is the anchor position of
  `quire.op.state.clause`
  ([FR-341](FR-341-checked-package-state-clause-body.md)).
- **occurrences** — every occurrence has role `anchor`, one per clause or
  protocol attempt that names the operation and resolves to this exact
  point-qualified anchor.
- **dependencies** — `C` and `F`, the two nodes the body references.

One `operation_anchor` node stands for one (declaring type, operation name,
execution point) tuple, where absence is a distinct value for an anchor with
no clause binding. The declaring type is an object type, a process or a state
machine, and its `frame` target is the node of that operation's frame. A frame node's key
hashes its body and its `semantic_type`, so operations of one declaring type
whose frames are equal share one frame node, each through its own anchor; an
anchor of another declaring type cannot share it, because the frame's
`semantic_type` equals the anchor's `context`. An operation a subtype inherits
is anchored at the type that declares it. An operation a process declares
gets its anchor, its frame node and its operation-contract record as an object
type's operation does, anchored at the `model`/`process` node, and an operation
a state machine declares the same at the `model`/`state_machine` node. The
`self` an anchored clause binds is the declaring type's `Reference<C>`
([FR-341](FR-341-checked-package-state-clause-body.md)): the run for a
process, the one context for a state machine.

## Operation resolution

The `operation` name resolves among the operation members of `C` by
[FR-322](FR-322-checked-package-artifact.md)'s "Model-owned members": step
2 looks up the declaration `C`'s `ModelOwner` names (an object type, a process
or a state machine), and step 3 matches the name against the exposed effective
operation members of `C`, own and inherited along its supertypes of the same
meaning, with redefinitions applied. Field members are not candidates. The resolved
operation is declared by `C` itself: an operation `C` only inherits is
anchored at its declaring type, not at `C`. The resolved operation record
is the signature every clause anchored here binds (FR-341).

## Reader order

The anchor checks run in the state step (FR-341 "Reader order"), after the
frame step, over every `state`/`operation_anchor` node in ascending node-id
digest order and before any `state`/`state_clause` node. For one anchor,
the checks run in this order and the first failing check is its defect:

1. **Context.** The `context` target is a declared dependency of the anchor,
   else `missing_declaration`/`missing-name`, that is a `model`/`object_type`,
   `model`/`process` or `model`/`state_machine` node, else
   `invalid_model_binding`/`malformed-declaration`. A record value type, an
   event type, a persistence interface, a systems interface and every other
   node are not contexts. The locus is the target.
2. **Frame.** The `frame` target is a declared dependency, else
   `missing_declaration`/`missing-name`, that is a `state`/`frame` node,
   else `invalid_model_binding`/`malformed-declaration`. The locus is the
   target.
3. **Types.** The anchor's `semantic_type` and the frame's `semantic_type`
   both equal the `context` target, else `invalid_model_binding`/
   `malformed-declaration` at the anchor node.
4. **Operation.** The `operation` name resolves as above: a context whose
   owner names no declaration was refused earlier by FR-322's owner join
   (`missing_declaration`/`missing-selection`); no operation of that name refuses
   `missing_declaration`/`missing-name`; two or more refuse
   `ambiguous_declaration`/`ambiguous-name`; an operation `C` does not
   itself declare refuses `invalid_model_binding`/`malformed-declaration`.
5. **Scoped deletes.** Every `parameter` entry of the `frame` target's
   `deletes_scoped` names a reference parameter of the resolved operation,
   else `missing_declaration`/`missing-name` with the frame node as the
   locus ([FR-340](FR-340-checked-package-frame-location.md)). A frame shared
   by several anchors is checked against each anchor's operation.

Once every anchor's own joins (1-3) hold, the reader checks the package-wide
uniqueness this artifact's "One `operation_anchor` node stands for..." rule
states, over every anchor in the same ascending node-id digest order: no two
anchors join to the same (`context`, `operation`, execution point) tuple. The
second anchor to claim a tuple refuses
`ambiguous_declaration`/`ambiguous-name` at that anchor.
Two anchors of one `context` that reference one `frame` node are admitted.
An operation-anchor point is `pre{operation}` or `post{operation}` and its
name equals the `operation` binding; it must equal the point carried by each
clause that references the anchor. A malformed or inapplicable point refuses
`invalid_model_binding`/`malformed-declaration` at the anchor.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-342-AC-1 | A `state`/`operation_anchor` node's body has exactly three bindings `context` (a `reference`), `operation` (a `text` literal whose value is an identifier) and `frame` (a `reference`), or those same bindings followed by `execution_point`; every occurrence has role `anchor`. A body with reordered bindings, non-text `operation`, missing binding or another occurrence role fails the published schema. | Test (TC-233) |
| FR-342-AC-2 | The `context` target is a declared `model`/`object_type`, `model`/`process` or `model`/`state_machine` dependency and the `frame` target a declared `state`/`frame` dependency; a `context` naming a frame, a `frame` naming an object type, and a `frame` target missing from `dependencies` refuse `invalid_model_binding`/`malformed-declaration`, `invalid_model_binding`/`malformed-declaration` and `missing_declaration`/`missing-name` respectively, at the target. | Test (TC-233) |
| FR-342-AC-3 | The anchor's `semantic_type` is its `context` node, whichever of the three it is; an anchor whose `semantic_type` names another node refuses `invalid_model_binding`/`malformed-declaration` at the anchor. | Test (TC-233) |
| FR-342-AC-4 | The `operation` name resolves among the context's exposed operation members to one the context declares: `(Order, "scaled")` resolves to `Order`'s operation and `(Sub, "size")` to `Sub`'s redefinition; `(Sub, "reset")`, which `Sub` only inherits, and an operation on a `systems_interface` node refuse `invalid_model_binding`/`malformed-declaration`; an undeclared name and a field's name refuse `missing_declaration`/`missing-name`; a context owned by a domain package identity the lock does not select refuses `missing_declaration`/`missing-selection`. | Test (TC-280) |
| FR-342-AC-5 | The state step checks every `operation_anchor` node before any `state_clause` node: a package carrying an anchor defect and a clause defect reports the anchor defect. | Test (TC-233) |
| FR-342-AC-6 | No two `state`/`operation_anchor` nodes join to the same (`context`, `operation`, execution point) tuple; a duplicate tuple refuses `ambiguous_declaration`/`ambiguous-name` at the second anchor in ascending node-id digest order. Two anchors of one `context` whose operations have equal frames reference one `state`/`frame` node and are both admitted. | Test (TC-233) |
| FR-342-AC-7 | A `state`/`operation_anchor` node whose `context` is a `model`/`process` node is admitted: `(Flow, "go")` resolves to the process's own operation, its `frame` is a `state`/`frame` node whose `semantic_type` is the process node, the anchor's `semantic_type` is the process node, and a reference to the anchor has family `object`. An operation a process inherits from its supertype process is anchored at the declaring process, and `(SubFlow, "go")` with `go` only inherited refuses `invalid_model_binding`/`malformed-declaration`. | Test (TC-233, TC-280) |
| FR-342-AC-8 | A `state`/`operation_anchor` node whose `context` is a `model`/`state_machine` node is admitted on the same terms: `(Gate, "toggle")` resolves to the machine's own operation and the frame and anchor `semantic_type` are the state-machine node. Two anchors of one process or state-machine `context` whose operations have equal frames reference one `state`/`frame` node, and a frame never joins the anchors of two different declaring types. | Test (TC-233, TC-280) |
| FR-342-AC-9 | A `context` that is a `model`/`record_value_type`, `model`/`event_type`, `model`/`persistence_interface` or `model`/`systems_interface` node refuses `invalid_model_binding`/`malformed-declaration` at the `context` target, and a process-owned or state-machine-owned anchor with a missing operation name refuses `missing_declaration`/`missing-name`, an ambiguous one `ambiguous_declaration`/`ambiguous-name`, and one owned by a domain package identity the lock does not select `missing_declaration`/`missing-selection`. | Test (TC-233, TC-280) |
| FR-342-AC-10 | An operation anchor admits an optional closed execution point after `frame`; `pre{operation: "go"}` and `post{operation: "go"}` are distinct identities for the same context and operation, while a protocol-only anchor may omit the point. An unknown variant, extra or missing member, initialization/handler point, mismatched operation name, duplicate context/operation/point tuple, or clause whose point differs from its anchor is refused at the first applicable reader step. FR-322 pins the closed point values; TC-233 verifies the anchor identity contribution and joins. | Test (TC-233) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the model-owned member
  resolution the operation name uses and the reader's refusal order.
- [FR-340](FR-340-checked-package-frame-location.md) owns the frame node the
  anchor binds.
- [FR-341](FR-341-checked-package-state-clause-body.md) owns the clauses
  anchored here and the state step.
- [AD-006](../../assurance/AD-006-model-graph-state-contracts.md) owns
  operation contracts as a model feature.
- [FR-047](../../functional/foundation/FR-047-emit-typed-located-causes.md)
  requires each refusal to carry a stable catalogued cause and an exact
  locus.

## Status

Accepted complete-V1 boundary value contract for the
`state`/`operation_anchor` node body.
