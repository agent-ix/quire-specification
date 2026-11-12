---
id: FR-172
title: "Bind choice evidence, obligations and captures"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-053
    type: references
  - target: ix://agent-ix/quire-specification/FR-055
    type: references
  - target: ix://agent-ix/quire-specification/FR-227
    type: references
  - target: ix://agent-ix/quire-specification/FR-479
    type: references
  - target: ix://agent-ix/quire-specification/FR-288
    type: references
---

# FR-172: Bind choice evidence, obligations and captures

## Description

When activating a protocol obligation, the binder SHALL preserve its
whole-execution origin, owned-choice visibility evidence and ordered immutable
capture set.

## Inputs

Protocol/workflow instance, control occurrence, guard, choice owner/evidence,
activation boundary and capture expressions.

## Outputs

A uniquely keyed obligation activation with ordered captures, or typed
visibility/identity/definedness refusal.

## Behavior

Choice visibility is discharged only by an explicit communication or declared
shared fact observable by the role before its dependent action. Obligation keys
include the full originating execution/control identity. Capture expressions are
evaluated in authored order at the inclusive activation boundary; duplicates
remain separate occurrences with one value each.

## Choice visibility carrier

The owned-choice visibility evidence the binder preserves is the choice's
checked visibility evidence record ([FR-227](../../objects/protocol/FR-227-owned-choice.md)
"Visibility evidence contract", `visibility_premises`), which belongs to the
exact FR-288 qualified protocol-definition the
[FR-479](../../objects/protocol/FR-479-obligation-activation-binding.md)
activation binding names. The activation binding adds no visibility-evidence
member of its own: the evidence is reached only through the binding's qualified
definition and the choice's occurrence in the activation relation row, so
replacing the definition or row replaces the evidence (FR-479-AC-3), and a
binding naming a definition whose choice lacks the record refuses. The evidence
is a static checked fact of the definition; a consumer reads it from the
admitted package and never supplies or reconstructs it from a runtime message,
name, payload or timestamp.

## Activation ordering

At one control occurrence the runtime first commits the choice, then records the
evidence visible to each role, activates resulting obligations, and finally
evaluates captures from left to right against the inclusive activation snapshot.
The activation key is `(package, protocol instance, workflow instance, control
occurrence, obligation declaration, activation ordinal)`. A capture key appends
its authored ordinal, so repeated expressions never collapse. Capture failure
refuses the activation atomically and emits no partial capture list.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-172-AC-1 | Explicit pre-action choice evidence discharges visibility for exactly the roles and branch it identifies. | Test (TC-203) |
| FR-172-AC-2 | Equal local obligation names in different execution/control origins produce distinct identities. | Test (TC-203) |
| FR-172-AC-3 | Duplicate capture expressions retain authored order and separate occurrence identities with inclusive-boundary values. | Test (TC-203) |
| FR-172-AC-4 | Choice commitment, visibility recording, activation and capture evaluation occur in the specified order. | Test (TC-203) |
| FR-172-AC-5 | One undefined capture refuses activation atomically without exposing preceding capture values. | Test (TC-203) |
| FR-172-AC-6 | The owned-choice visibility evidence an activation preserves is the checked FR-227 record of the choice in the binding's FR-288 qualified definition; the FR-479 binding carries no separate visibility member, a binding whose definition lacks the choice's record refuses, and a caller-supplied or runtime-reconstructed substitute does not stand in for it. | Test (TC-203) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
