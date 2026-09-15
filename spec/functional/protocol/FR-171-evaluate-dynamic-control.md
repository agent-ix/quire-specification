---
id: FR-171
title: "Evaluate joins, proved loops and dynamic roles"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-052
    type: references
  - target: ix://agent-ix/quire-specification/AD-007
    type: references
---

# FR-171: Evaluate joins, proved loops and dynamic roles

## Description

When checking dynamic protocol control, the checker SHALL validate each join,
loop and role-spawn against a closed control and population contract.

## Inputs

Control graph, join policy, branch states, loop invariant/variant, dynamic-role
population/lifetime/closure and resource bounds.

## Outputs

Checked control semantics or typed ambiguity, termination, closure or bound
refusal/incomplete outcome.

## Behavior

Join policies are `all`, `any`, `quorum(n)` or a named pure predicate and define
outstanding-branch cancellation/continuation. Loops require invariant preservation
and a well-founded variant for termination claims. Dynamic roles bind unique
instances to declared populations and lifetimes; finite execution requires a
discharged population bound.

## Control and population rules

`all` settles after every branch settles; `any` settles after the first branch;
`quorum(n)` settles after `n` distinct branches; and a predicate join settles
only when its pure predicate is true over the immutable branch-result map. Each
join declares whether unsettled branches continue or receive cancellation.
A loop invariant holds before the first iteration and after every body step; a
lexicographic well-founded variant decreases on every back edge.

A spawn allocates a fresh `(protocol_instance, role_declaration, ordinal)`
identity within its closed population. Lifetime is lexical, explicit-until or
protocol-instance; retirement forbids later actions but preserves history.
Population closure and a finite upper bound are separate premises.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-171-AC-1 | Every join policy settles only under its exact branch condition and applies its declared outstanding-branch behavior. | Test (TC-202) |
| FR-171-AC-2 | A loop with proved invariant/variant is admitted; an unproved termination claim is inconclusive/refused rather than silently bounded. | Test (TC-202) |
| FR-171-AC-3 | Spawn/lifetime/population closure creates and retires exact role instances; missing symbolic bounds return requires-bound for finite execution. | Test (TC-202) |
| FR-171-AC-4 | Each join policy is tested at zero, below-threshold, threshold and post-threshold branch states. | Test (TC-202) |
| FR-171-AC-5 | A retired dynamic role cannot act or be silently replaced by a new role with the same display name. | Test (TC-202) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
