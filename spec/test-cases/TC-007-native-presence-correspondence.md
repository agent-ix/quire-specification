---
id: TC-007
title: "Preserve authored presence and null distinctions"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: verifies
---
## Description

Check the selected shared-model presence contract without inventing native clause
syntax. Planned; FCD #95 owns the unresolved source representation.

## Test Procedure

Use the selected producer's distinct required-empty 0..* and optional-nonempty
1..* declarations. Offer an absent member, present empty sequence, present nonempty
sequence and explicit null separately. Check source-derived declarations and
runtime values at the real consumer boundary. Where the selected representation
cannot express a distinction, exercise its explicit declaration refusal. Keep
optional value absence and nullable invocation-result selection as separate cases.

## Expected Results

Each authored distinction is preserved or refused before execution; no adapter
invents member presence from cardinality, an option, current data or a legacy IR
field shape. No accepted outcome is assumed before #95 selects the contract.
