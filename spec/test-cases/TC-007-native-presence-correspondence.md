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
syntax. FCD #95 / PR #99 accepts the static authored-axis contract and QSL #93 /
PR #94 consumes its exact admitted bundle. The Semantic IR/frontend and
runtime-value portion remains planned under FCD #93 and the relevant consumer.

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
field shape. The accepted static producer/native result does not claim that the
remaining source-IR or runtime-value cases have executed.
