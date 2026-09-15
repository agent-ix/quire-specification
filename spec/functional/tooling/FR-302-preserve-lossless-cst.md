---
id: FR-302
title: "Preserve a lossless concrete syntax tree"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-134
    type: references
  - target: ix://agent-ix/quire-specification/AD-004
    type: references
---

# FR-302: Preserve a lossless concrete syntax tree

## Description

When parsing source for authoring tools, the parser SHALL retain every token,
trivia byte, recovery insertion/deletion and original source span in a lossless
concrete syntax tree.

## Inputs

Exact source bytes, source revision and edition.

## Outputs

Lossless CST with recoveries/diagnostics, or fatal encoding/resource refusal.

## Behavior

Serializing an unrecovered CST reproduces the original bytes exactly. Recovery
nodes are explicit, zero/fixed-width source-correspondent edits and never checked
semantic declarations. Incremental edits create a new source revision and retain
stable node identity only for unchanged byte-correspondent regions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-302-AC-1 | Arbitrary valid source including comments/whitespace round-trips byte-for-byte through the CST. | Test (TC-222) |
| FR-302-AC-2 | Invalid source exposes explicit recoveries and exact loci without publishing a checked package. | Test (TC-222) |
| FR-302-AC-3 | Incremental edits preserve identity only for unchanged mapped nodes under the new source revision. | Test (TC-222) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
