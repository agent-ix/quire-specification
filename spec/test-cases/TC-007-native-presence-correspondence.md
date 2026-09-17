---
id: TC-007
title: "Preserve authored presence and null distinctions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: verifies
---
## Description

Check the authored presence contract without inventing native clause syntax.
Presence is declared in the artifact `## Properties` `Presence` column. It
reaches Quire as IR `Field.presence` in the domain package
([AD-006](../assurance/AD-006-model-graph-state-contracts.md)). The extraction,
intake and runtime-value portions remain planned.

## Test Procedure

Use domain package declarations with distinct required-empty 0..* and
optional-nonempty 1..* fields. Offer an absent member, present empty sequence, present nonempty
sequence and explicit null separately. Check source-derived declarations and
runtime values at the real consumer boundary. Where the selected representation
cannot express a distinction, exercise its explicit declaration refusal. Keep
optional value absence and nullable invocation-result selection as separate cases.

## Expected Results

Each authored distinction is preserved or refused before execution; no adapter
invents member presence from cardinality, an option, current data or an IR
field shape that lacks `presence`. No executable coverage is claimed for the
extraction, intake or runtime-value cases.
