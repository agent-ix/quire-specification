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
versioned and separate from human diagnostics. Exit statuses distinguish success,
logical violation, invalid/refused input, unsupported capability, incomplete
execution and tool failure. Version output reports the app version.

## Command and exit contract

CLI verbs mirror the lifecycle API and accept an explicit profile plus
versioned input/output formats. Exit codes are: `0` completed without violation,
`10` logical violation, `20` invalid/refused input, `21` unsupported, `22`
incomplete and `30` tool failure. Multi-item machine output retains per-item
dispositions; the process code is the highest-severity present code under this
ordering: tool failure, invalid, unsupported, incomplete, violation, success.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-301-AC-1 | Every lifecycle command round-trips its typed API request/result in machine-output mode. | Test (TC-221) |
| FR-301-AC-2 | Each outcome class produces its assigned stable exit status without parsing message text. | Test (TC-221) |
| FR-301-AC-3 | Unknown options/profiles/providers refuse before execution and identify the command argument locus. | Test (TC-221) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
