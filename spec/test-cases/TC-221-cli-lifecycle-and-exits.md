---
id: TC-221
title: "CLI lifecycle and exits"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-301
    type: verifies
---

# TC-221: CLI lifecycle and exits

## Description

Invokes every CLI verb and checks machine output, argument refusals and the
exit code of every outcome category. Planned.

## Test Procedure

| Row | Input | Expected | AC |
| --- | --- | --- | --- |
| CL-01 | Every lifecycle verb in machine-output mode | Each round-trips its typed library request and outcome | FR-301-AC-1 |
| CL-02 | Runs settling success, violation, a claim whose evaluation is undefined, a value run whose value is undefined, a pending temporal clause, an indeterminate temporal clause, refusal, unsupported, incomplete, inconclusive and internal failure, and a multi-item run mixing them; `version` | Exits `0`, `10`, `10`, `10`, `0`, `11`, `20`, `21`, `22`, `22` and `30`; the undefined claim is a violation with cause `UndefinedEvaluation`; the undefined value run keeps the label `undefined` in machine output; the pending clause keeps the label `pending` and the inconclusive proof still exits `22`; the mixed run exits with the most severe present code in the order `30`, `20`, `21`, `22`, `10`, `11`, `0` (so {10, 11} exits `10`, {11, 22} exits `22` and {0, 11} exits `11`); no exit depends on message text; `version` prints the application version | FR-301-AC-2 |
| CL-03 | An unknown verb, option, profile and provider | Each refuses before any stage runs, exits `20` and names the argument | FR-301-AC-3 |

## Expected Results

Every outcome category has exactly one exit code, undefined exits 10 and stays
labelled `undefined`, and argument errors never reach a stage.
