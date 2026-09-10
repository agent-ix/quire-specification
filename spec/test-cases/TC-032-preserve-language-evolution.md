---
id: TC-032
title: "Preserve selected meanings across language evolution"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-032
    type: verifies
---
## Description

Planned acceptance controls for [FR-032](../functional/FR-032-preserve-language-evolution.md).
No executable test or completed coverage is claimed by this artifact.

## Test Procedure

Retain one historical source and definition. Introduce independently a new operator, declaration and family, including a keyword colliding with a previously legal identifier. Read old and new selections with old/new support sets. Migrate one source explicitly and compare original artifacts before and after.

## Expected Results

The historical source retains its interpretation. New required meaning either has explicit support or refuses. Migration produces a new subject and source correspondence without mutating old bytes or restamping old evidence. Exact keyword/grammar vectors are supplied by L1 before implementation.

