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
Planned replacement-profile acceptance; compiler PR31 has reusable controls.

## Test Procedure

Through source intake and public model admission, compare maxima 10000 and 10001
in direct, nested and unused declarations. Reject missing bounds and deferred
collection kinds/conversions. Evaluate ordered duplicate values and empty
forall/exists domains under a valid explicit maximum; offer missing sequence data.

## Expected Results

The 10000 declaration admits and 10001 refuses regardless of reachability or
runtime budgets. Duplicate occurrences retain order and count. Empty universal
is true, empty existential false; missing input cannot become an empty sequence.
