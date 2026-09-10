---
id: TC-020
title: "Report complete versus partial language capability scope"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-020
    type: verifies
---
## Description

Planned reporting controls for the versioned full-v1 inventory. This TC does
not supply any missing implementation/conformance evidence itself.

## Test Procedure

Construct report inputs with one family absent, all families present but a
required related-instance/population capability absent, and a fully referenced
synthetic inventory. Replace selected evidence with only planned cases or an
old profile's results. Separately request a deliberately partial profile report
while later realizability research is unfinished.

## Expected Results

Missing required capabilities and mismatched/absent conformance references stay
visible and prevent full-v1 claims. A complete synthetic report qualifies only
the reporting mechanism, not actual language support. An accurately scoped
partial release does not claim full v1 or wait for unrelated later research.
