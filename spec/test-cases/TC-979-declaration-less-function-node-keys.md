---
id: TC-979
title: "Declaration-less function node keys"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-322
    type: verifies
---

# TC-979: Declaration-less function node keys

## Description

Verify that a declaration-less `function` node has the derived key of FR-322
"Declaration-less function nodes": its parameters in order, its body root and its
measure, so equal content has an equal path and any change of content a different
one, that a duplicate `node_id` is refused, and that a model-owned accessor keeps
its owner-chosen label.

## Test Procedure

Read `proposals/checked-package-v2/entry-key-vectors.json`. For each case with
`parameters`, derive the path `["function", semantic_form, ...parameter index
segments, body root index segment, measure index segment if any]` and compare it
with `expected_path`; a case naming `equal_to` must equal that case's path and a
case naming `differs_from` must differ from it. EK-07 gives two nodes with equal
`package`, `path` and `ordinal`, which the reader refuses as a duplicate
`node_id`. EK-08 is an accessor with an owner-chosen label, admitted unchanged.

## Expected Results

EK-01 to EK-06 derive their `expected_path`; EK-03 equals EK-02; EK-04 to EK-06 differ
from EK-02. EK-07 refuses `invalid_semantic_graph` at the second node's `node_id`.
EK-08 is admitted.
