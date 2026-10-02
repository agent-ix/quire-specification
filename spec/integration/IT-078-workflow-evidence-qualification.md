---
id: IT-078
title: "Authorized workflow results enter qualification evidence"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-182
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-311
    type: verifies
---

# IT-078: Authorized workflow results enter qualification evidence

## Objective

Verify formal decisions, authorized external adapter receipts, observation
feedback, counterexample lineage and qualification evidence retain distinct
identities through the real Rust boundaries.

## Target Integration

The `quire-integration` Rust facade and an isolated local adapter exchange I11;
QObs consumes the resulting observation; Verification emits I20 evidence intake.
No Quoin runtime dependency is required before its Rust port lands.

## Preconditions

Authority grants and adapter/evidence contracts are available locally.

## Inputs

Authorized, duplicate, unauthorized, conflicting-receipt and redacted-evidence
cases over one reference workflow.

## Test Procedure

1. Process the authorized decision and duplicate delivery.
   - IT-078-SC-01: one effect request and one reconciled receipt are retained.
2. Feed the observed effect back and build evidence.
   - IT-078-SC-02: decision, request, receipt, effect, result and evidence stay distinct.
3. Run refusal/redaction cases and qualification.
   - IT-078-SC-03: limitations prevent only claims requiring missing authority/replay.

## Expected Results

No evaluator performs the effect, no receipt fabricates business success, and
qualification preserves exact traceability and limitations.
