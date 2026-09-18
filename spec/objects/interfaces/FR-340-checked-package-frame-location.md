---
id: FR-340
title: "Checked-package frame location"
type: FR
object: value_object
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: specifies
  - target: ix://agent-ix/quire-specification/FR-207
    type: references
  - target: ix://agent-ix/quire-specification/FR-047
    type: references
---

# FR-340: Checked-package frame location

## Description

When a `quire.checked-semantic-graph/v2` node has `node_tag: "state"` and
`semantic_form: "frame"`, the producer SHALL encode its `body` as a closed,
ordered set of locations — each an exact declared root plus an ordered
traversal path — rather than an undifferentiated `SemanticTerm`, so a
consumer can enumerate every location an operation may modify without
inventing a vocabulary of its own.

## Properties

- **root** (`NodeRef`) — the exact declared node the location begins at,
  using the same node-key reference mechanism as every other reference in
  the checked semantic graph ([FR-322](FR-322-checked-package-artifact.md)).
  A location's root equals one of the frame node's own `dependencies`; a
  root naming a node the frame node does not declare as a dependency is
  refused.
- **steps** (ordered sequence of traversal step, possibly empty) — the
  closed vocabulary is exactly
  [FR-207](../foundation/FR-207-runtime-value-path.md)'s five kinds and
  their exact wire tags: `field` and `member` each carry the exact declared
  spelling as `name`; `index` carries an unsigned zero-based position as
  `index`; `map_key` carries a canonical typed key value as `key`; and
  `option_value` carries no further member. A frame location reuses this
  vocabulary exactly rather than defining a narrower or wider one: FR-207
  already closes "what a traversal step may be" for locating any value in
  the complete-V1 type system, and a modifiable location is one more way of
  locating a value. An empty step sequence denotes the root declaration
  itself, matching FR-207's "an empty step sequence denotes the subject
  itself".

## Wire encoding

A frame node's `body` is exactly `{"term": "aggregate", "members": [...]}`,
where every member is exactly `{"term": "location", "root": NodeRef,
"steps": [...]}`. Zero members is admitted and denotes an operation that
modifies nothing. Every member is a distinct bytewise value; two
bytewise-identical locations inside one frame body are refused. Every other
`SemanticTerm` shape — a bare `literal`, `reference`, `application` or an
`aggregate` whose members are not exactly `location` terms — is refused as a
`state`/`frame` body. Step tags are exactly `field`, `option_value`,
`member`, `index` and `map_key`; a step carrying an unrecognized tag, or
missing the exact further member its tag requires, is refused.

## Canonical ordering

A frame body's `members` array is in ascending canonical order, so an
identity preimage over a frame is stable without a separate sort step. The
order is exactly FR-207's key order, applied to a root-and-steps location in
place of FR-207's observation-and-subject-and-steps runtime path:

1. Order first by root node key (`NodeRef.digest`, compared as unsigned
   bytes).
2. Then order by step sequence: a proper prefix orders before every path
   that extends it; differing step kinds order `field` before
   `option_value` before `member` before `index` before `map_key`; steps of
   equal kind order by their exact name (UTF-8 bytes), by index
   (numerically), or by the map key's RFC 8785 (JCS) canonical JSON bytes.

A body whose members are not in this exact order is refused.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-340-AC-1 | A frame body of zero locations (`{"term":"aggregate","members":[]}`) validates against the published schema. | Test (TC-233) |
| FR-340-AC-2 | A frame body carrying one or more `location` members, each an exact `root` and an ordered (possibly empty) `steps` sequence restricted to `field`, `option_value`, `member`, `index` and `map_key`, validates against the published schema. | Test (TC-233) |
| FR-340-AC-3 | A `state`/`frame` node whose body is a bare `literal`, `reference`, `application`, or an `aggregate` whose members are not exactly `location` terms, fails the published schema. | Test (TC-233) |
| FR-340-AC-4 | A location step carrying an unrecognized `tag`, a `field`/`member` step without `name`, an `index` step without `index`, or a `map_key` step without `key`, fails the published schema. | Test (TC-233) |
| FR-340-AC-5 | Two bytewise-identical location members inside one frame body fail the published schema. | Test (TC-233) |
| FR-340-AC-6 | Recomputing the FR-207-derived canonical order of an admitted frame body's members reproduces the authored member sequence exactly; a body with an out-of-order pair is distinguishable from a canonically ordered one by that recomputation. | Test (TC-233) |
| FR-340-AC-7 | Every admitted location's `root` equals one of the frame node's own `dependencies`; a fixture whose location root is absent from `dependencies` is distinguishable by that join. | Test (TC-233) |

## Dependencies

- [FR-322](FR-322-checked-package-artifact.md) owns the `semantic_graph`
  property this artifact specifies the `state`/`frame` body of.
- [FR-207](../foundation/FR-207-runtime-value-path.md) and
  [FR-047](../foundation/FR-047-emit-typed-located-causes.md) supply the
  closed traversal-step vocabulary, its exact wire tags, and its canonical
  order, reused here without change.
- [interface_004](interface_004-checked-package.md) publishes the schema
  this artifact's shape is normative against, under
  [`proposals/checked-package-v2`](../../../proposals/checked-package-v2/).

## Status

Accepted complete-V1 boundary value contract for the `state`/`frame` node
body. Implementation and qualification remain tracked separately.
