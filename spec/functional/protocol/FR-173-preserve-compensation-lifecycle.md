---
id: FR-173
title: "Preserve the complete compensation lifecycle"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-056
    type: references
  - target: ix://agent-ix/quire-specification/FR-058
    type: references
---

# FR-173: Preserve the complete compensation lifecycle

## Description

When assessing compensation, the protocol runtime SHALL apply the closed
registration, eligibility, attempt, retry, settlement and terminal-state
transition system.

## Inputs

Effect, registration, commit/recovery relation, attempts/outcomes, retry policy,
progress/completeness and observation revisions.

## Outputs

Independent execution, truth, activation, adequacy and closure axes with complete
lifecycle provenance.

## Behavior

The lifecycle states are unregistered, registered, eligible, attempting,
retry-wait, settled-success, settled-failure, cancelled and incomplete. Only an
observed matching successful effect permits registration. Incomplete input may
resume through an explicit input revision; terminal states change only through
an authored supersession relation. Every consumer preserves the full outcome
partition.

## Lifecycle transition system

Registration moves `unregistered -> registered` after a matching successful
effect. A declared recovery trigger moves `registered -> eligible`; dispatch
moves `eligible -> attempting`; a retryable failed attempt moves
`attempting -> retry-wait -> eligible`; and a final observed outcome moves an
attempt to `settled-success` or `settled-failure`. Cancellation is permitted
only before an attempt begins. Missing evidence yields `incomplete` while
retaining the prior state. Receipt acceptance, business-effect observation and
recovery adequacy are three separately evidenced decisions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-173-AC-1 | Every permitted lifecycle transition succeeds and every undeclared transition refuses with current/attempted states. | Test (TC-204) |
| FR-173-AC-2 | Attempt, receipt, business effect and achieved recovery remain distinct through success, retry and partial failure. | Test (TC-204) |
| FR-173-AC-3 | An explicit new input revision resumes incomplete assessment while a terminal result remains immutable absent supersession. | Test (TC-204) |
| FR-173-AC-4 | Retry eligibility follows the declared attempt count, delay and retryable-outcome predicate exactly. | Test (TC-204) |
| FR-173-AC-5 | A successful transport receipt cannot by itself establish either business recovery or compensation adequacy. | Test (TC-204) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
