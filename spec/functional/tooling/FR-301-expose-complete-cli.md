---
id: FR-301
title: "Expose the complete native CLI lifecycle"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-300
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-010
    type: references
---

# FR-301: Expose the complete native CLI lifecycle

## Description

When a user invokes the Quire CLI, the command SHALL map parse, check, lower,
execute, replay, monitor, inspect, format and version requests to the exact Rust
lifecycle contracts.

## Inputs

Command, files/stdin, explicit profile/provider/options and output format.

## Outputs

Deterministic stdout artifact/result, structured stderr diagnostics and stable
exit status.

## Behavior

Commands never infer a wider profile or fallback target. Machine output is
separate from human diagnostics. Exit statuses distinguish success, logical
violation, undefined evaluation, invalid/refused input, unsupported capability,
incomplete execution, inconclusive results and tool failure. Version output reports the app version.

## Command and exit contract

CLI verbs mirror the lifecycle API and accept an explicit profile plus
explicit input/output formats. One total function maps each outcome
category to its exit code:

| Outcome category | Exit code |
| --- | --- |
| success: completed without violation | `0` |
| violation: a logical violation, including a claim whose evaluation is undefined | `10` |
| undefined: a value run (`run` of a function) whose value is undefined | `10` |
| pending: a temporal clause that declared progress could still resolve, including a supplied-trace clause pending when the trace ends | `0` |
| indeterminate: a temporal clause whose closed evidence leaves its verdict unresolved ([FR-091](../temporal/FR-091-evaluate-bounded-future.md)) | `11` |
| refusal: invalid or refused input | `20` |
| unsupported | `21` |
| incomplete: a limit, cancellation, deadline or budget | `22` |
| inconclusive | `22` |
| internal failure, including tool failure | `30` |

A claim whose evaluation is undefined settles violation (refuted) with cause
`UndefinedEvaluation{where, cause}` and exits `10`. A value run whose value is
undefined exits `10` and its machine output keeps the label `undefined`. Multi-item machine output retains
per-item dispositions; the process code is the highest-severity present code
under this ordering: `30`, `20`, `21`, `22`, `10`, `11`, `0`.

Exit `11` is the next free code in the evaluation-result decade beside `10`: an
indeterminate item is neither a violation nor a success, so it takes its own code
rather than sharing `10` or `22`. It ranks below `10` so that an indeterminate item
never hides a violation, and above `0` so that it is never reported as success.
A `pending` result is its own category and exits `0`: nothing has failed and the
clause awaits progress. `inconclusive` is the proof and analysis category and
keeps `22`; `pending` is not inconclusive, and neither is indeterminate.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-301-AC-1 | Every lifecycle command round-trips its typed API request/result in machine-output mode. | Test (TC-221) |
| FR-301-AC-2 | Each outcome category produces its assigned stable exit status without parsing message text; a claim whose evaluation is undefined exits `10` as a violation with cause `UndefinedEvaluation`, a value run whose value is undefined exits `10` with the label `undefined`, a pending clause exits `0` with the category label `pending`, an indeterminate clause exits `11` with the category label `indeterminate`, a multi-item run with item codes {10, 11} exits `10`, {11, 22} exits `22` and {0, 11} exits `11`, and `version` reports the application version. | Test (TC-221) |
| FR-301-AC-3 | Unknown options/profiles/providers refuse before execution and identify the command argument locus. | Test (TC-221) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
