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

Use every ID in the [51-row capability crosswalk](../../proposals/quire-v1/capabilities.md#initial-specification-crosswalk),
preserving its V1/Later/Refuse decision. A required positive V1 capability cannot
disappear because another row shares its family. Later capabilities are not
silently promoted to mandatory positive scope; selected-profile refusal rules
still require their corresponding negative conformance controls. References to
D's reserved but absent requirement IDs do not count as authored or verified
requirements. An outer package feature list is discovery metadata, not evidence
that any listed family or required capability is implemented.

## Expected Results

Missing required capabilities and mismatched/absent conformance references stay
visible and prevent full-v1 claims. A complete synthetic report qualifies only
the reporting mechanism, not actual language support. An accurately scoped
partial release does not claim full v1 or wait for unrelated later research.
