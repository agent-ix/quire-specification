---
id: IT-050
title: "Integrate admitted observations with temporal and protocol assessment"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-113
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-114
    type: verifies
---
## Objective

Verify the planned boundary from F-owned admitted observations/progress to E-owned temporal meaning and B-owned protocol assessment without reconstructing bindings or invoking a production monitor.

## Target Integration

The integration joins the [observation binding contract](../../proposals/quire-v1/observation-contract.md), immutable captures, declared clock/history/closure inputs, and protocol obligation identities to selected temporal and protocol consumers.

## Preconditions

The consumed A/B/E interfaces are selected by exact identity. The O1/O2 fixture
declares finite bounds, relationships, clock authority, and expected outcomes.

## Inputs

Satisfied refund, missed deadline, quiet watermark, late refund, missing provider record, and ambiguous relationship controls for concurrent orders and split shipments.

## Test Procedure

1. Bind each record under its declared identity and correlation.
   - IT-050-SC-01: wrong/ambiguous binding refuses or remains incomplete.
2. Supply the same admitted records to batch and incremental assessment.
   - IT-050-SC-02: settled outcomes agree and non-settled causes remain distinct.
3. Remove clock/history/closure authority independently.
   - IT-050-SC-03: no missing authority becomes a deadline violation or success.

## Expected Results

The integration retains provenance and separates satisfied, violating, pending, incomplete, and refused results. It remains a planned contract test, not evidence that a production monitor executed.
