---
id: FR-301
title: "Expose the complete native CLI lifecycle"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-300
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
execution and tool failure. Version output reports semantic/wire/tool pins.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-301-AC-1 | Every lifecycle command round-trips its typed API request/result in machine-output mode. | Test (TC-221) |
| FR-301-AC-2 | Each outcome class produces its assigned stable exit status without parsing message text. | Test (TC-221) |
| FR-301-AC-3 | Unknown options/profiles/providers refuse before execution and identify the command argument locus. | Test (TC-221) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
