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
an aggregate of exactly three bindings, in this order: `context`, the
object type declaring the operation; `operation`, the operation's name; and
`frame`, the operation's `state`/`frame` node. The reader SHALL refuse a
node whose joins below do not hold with the first refusal in the order
below.

## Properties

- **body** — `{term: "aggregate", members: [binding("context",
  reference(C)), binding("operation", literal), binding("frame",
  reference(F))]}`:
  - `context` — a `reference` to the `model`/`object_type` node `C` that
    declares the operation;
  - `operation` — a `literal` term with `value_kind: "text"`, whose `type`
    is the `scalar_type`/`text` node and whose `value` is the operation's
    name, an identifier;
  - `frame` — a `reference` to the operation's `state`/`frame` node `F`
    ([FR-340](FR-340-checked-package-frame-location.md)).
- **semantic_type** — `C`, the context object type's node. A reference to
  the anchor therefore has family `object`, which is the anchor position of
  `quire.op.state.clause`
  ([FR-341](FR-341-checked-package-state-clause-body.md)).
- **occurrences** — every occurrence has role `anchor`, one per clause that
  names the operation.
- **dependencies** — `C` and `F`, the two nodes the body references.

One `operation_anchor` node and one `frame` node stand for one (declaring
object type, operation name). An operation a subtype inherits is anchored
at the object type that declares it.

## Operation resolution

The `operation` name resolves among the operation members of `C` by
[FR-322](FR-322-checked-package-artifact.md)'s "Model-owned members": step
2 looks up the declaration `C`'s `ModelOwner` names, and step 3 matches the name against the
exposed effective operation members of `C`, own and inherited, with
redefinitions applied. Field members are not candidates. The resolved
operation is declared by `C` itself: an operation `C` only inherits is
anchored at its declaring type, not at `C`. The resolved operation record
is the signature every clause anchored here binds (FR-341).

## Reader order

The anchor checks run in the state step (FR-341 "Reader order"), after the
frame step, over every `state`/`operation_anchor` node in ascending node-id
digest order and before any `state`/`state_clause` node. For one anchor,
the checks run in this order and the first failing check is its defect:

1. **Context.** The `context` target is a declared dependency of the anchor,
   else `missing_declaration`/`missing-name`, that is a
   `model`/`object_type` node, else `invalid_model_binding`/
   `malformed-declaration`. The locus is the target.
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

Once every anchor's own joins (1-3) hold, the reader checks the package-wide
uniqueness this artifact's "One `operation_anchor` node and one `frame`
node..." rule states, over every anchor in the same ascending node-id
digest order: no two anchors join to the same (`context`, `operation`)
pair, and no `frame` target is the `frame` of two different anchors. The
second anchor to claim a pair or a `frame` target refuses
`ambiguous_declaration`/`ambiguous-name` at that anchor.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-342-AC-1 | A `state`/`operation_anchor` node's body is exactly the three bindings `context` (a `reference`), `operation` (a `text` literal whose value is an identifier) and `frame` (a `reference`), in that order, and every occurrence has role `anchor`; a body with the bindings reordered, a non-text `operation`, a missing binding, and an occurrence of another role each fail the published schema. | Test (TC-233) |
| FR-342-AC-2 | The `context` target is a declared `model`/`object_type` dependency and the `frame` target a declared `state`/`frame` dependency; a `context` naming a frame, a `frame` naming an object type, and a `frame` target missing from `dependencies` refuse `invalid_model_binding`/`malformed-declaration`, `invalid_model_binding`/`malformed-declaration` and `missing_declaration`/`missing-name` respectively, at the target. | Test (TC-233) |
| FR-342-AC-3 | The anchor's `semantic_type` is its `context` object type; an anchor whose `semantic_type` names another node refuses `invalid_model_binding`/`malformed-declaration` at the anchor. | Test (TC-233) |
| FR-342-AC-4 | The redefinition case `(Sub, "size")` is not qualified until AGE-2226 restores operation redefinition in the document (AN-02 is dropped from the vectors; the requirement stands). The `operation` name resolves among the context's exposed operation members to one the context declares: `(Order, "scaled")` resolves to `Order`'s operation and `(Sub, "size")` to `Sub`'s redefinition; `(Sub, "reset")`, which `Sub` only inherits, and an operation on a `systems_interface` node refuse `invalid_model_binding`/`malformed-declaration`; an undeclared name and a field's name refuse `missing_declaration`/`missing-name`; a context keyed under an unselected domain package version refuses `missing_declaration`/`missing-selection`. | Test (TC-280) |
| FR-342-AC-5 | The state step checks every `operation_anchor` node before any `state_clause` node: a package carrying an anchor defect and a clause defect reports the anchor defect. | Test (TC-233) |
| FR-342-AC-6 | No two `state`/`operation_anchor` nodes join to the same (`context`, `operation`) pair, and no `state`/`frame` node is the `frame` target of two different anchors; a duplicate pair and a shared frame each refuse `ambiguous_declaration`/`ambiguous-name` at the second anchor in ascending node-id digest order. | Test (TC-233) |

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
`state`/`operation_anchor` node body. Implementation and qualification
remain tracked separately.
