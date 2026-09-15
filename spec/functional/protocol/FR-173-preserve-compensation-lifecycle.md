---
id: FR-173
title: "Preserve the complete compensation lifecycle"
type: FR
relationships:
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

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-173-AC-1 | Every permitted lifecycle transition succeeds and every undeclared transition refuses with current/attempted states. | Test (TC-204) |
| FR-173-AC-2 | Attempt, receipt, business effect and achieved recovery remain distinct through success, retry and partial failure. | Test (TC-204) |
| FR-173-AC-3 | An explicit new input revision resumes incomplete assessment while a terminal result remains immutable absent supersession. | Test (TC-204) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
