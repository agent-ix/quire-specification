---
id: TC-009
title: "Enforce sequence declaration bounds and occurrence order"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-019
    type: verifies
---
## Description

Check declared sequence bounds independently from runtime work budgets.
Planned composed state-core acceptance; merged compiler tests may supply reusable controls.

## Test Procedure

Through source intake and public model admission, declare sequence maxima in
direct, nested and unused declarations. Reject missing bounds and deferred
collection kinds/conversions. Evaluate ordered duplicate values and empty
forall/exists domains under a valid explicit maximum; offer missing sequence data.

## Expected Results

Each declared maximum admits and a missing bound refuses regardless of
reachability or runtime budgets. Duplicate occurrences retain order and count. Empty universal
is true, empty existential false; missing input cannot become an empty sequence.
