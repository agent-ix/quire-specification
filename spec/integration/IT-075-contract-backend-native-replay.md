---
id: IT-075
title: "Contract backends agree with native replay"
type: IT
relationships:
  - target: ix://agent-ix/quire-specification/FR-195
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-197
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-180
    type: verifies
---

# IT-075: Contract backends agree with native replay

## Objective

Verify QSL-to-Contract-IR lowering, generated oracle/property/Kani consumers and
the native QSL complete-V1 executor (FR-197, AD-016 arrow 7) agree on supported bounded subjects and explicit
refusals.

## Target Integration

Real pinned Rust APIs from QSL, Contract Model/IR, Codegen and Contract Runtime
run locally; Kani uses the recorded installed version/options.

## Preconditions

All crates use `publish = false` and the isolated Cargo target directory
`target-codex-backends`; no hosted CI is dispatched.

## Inputs

Complete supported corpus including ConfigVersion 0, 1000 and just outside,
plus unsupported/missing-bound and mutation cases.

## Test Procedure

1. Lower every requested item and inspect accounting.
   - IT-075-SC-01: every item is exact-supported, requires-bound or unsupported.
2. Execute oracle/property/Kani and native bounded cases.
   - IT-075-SC-02: all supported verdicts agree.
3. Replay every backend counterexample through native execution.
   - IT-075-SC-03: verdict agrees or parity failure is retained.

## Expected Results

No unsupported construct emits an approximating artifact; domain bounds and
tool pins are exact and replayable.
