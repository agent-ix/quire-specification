---
id: TC-059
title: "Assess finite global conformance"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-059
    type: verifies
---
## Description

Verify global conformance across permitted concurrency and adverse finite histories.

## Test Procedure

Replay every causally valid S1/S2 interleaving with payment and refund. Inject unexpected, missing, misordered, duplicate-effect and cross-workflow events. Compare open/incomplete histories and settled batch versus incremental inputs.

## Expected Results

Valid interleavings conform. Every adverse event names its obligation; open/incomplete inputs never report complete success; settled batch and incremental semantic results agree exactly.

