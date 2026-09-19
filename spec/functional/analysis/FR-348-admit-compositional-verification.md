---
id: FR-348
title: "Admit compositional verification obligations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-001
    type: references
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/AD-013
    type: references
---

# FR-348: Admit compositional verification obligations

## Description

When a system-level verification obligation decomposes into independently
provable sub-obligations over components connected only through their
declared interface contracts, the verification planner SHALL admit
compositional discharge: each sub-obligation is verified independently and
the system obligation follows from their composition under the declared
interface contract.

## Inputs

A system obligation over two or more composed components (per AD-001), each
component's own already-verified sub-obligation, the component's declared
interface contract (its typed pre/post obligations or protocol role
obligations), and the composition rule connecting the components.

## Outputs

Proved, refuted with a component-level counterexample or an interface-mismatch
counterexample, inconclusive, `unsupported`, or failed.

## Behavior

A system obligation is not re-verified as one undecomposed whole when its
components are already independently verified and connect only through
declared interface contracts. The planner checks that each component's
verified sub-obligation matches what the composition rule requires of it at
its interface, and, when every interface obligation matches, composes the
independently established results into the system-level verdict without
re-discharging the composed whole. An interface mismatch — a component's
proved obligation weaker than what the composition requires of it, or an
undeclared interface — refutes with the mismatching interface and component
identified. Composition is lowered as a target-neutral obligation per
AD-009/AD-010 and negotiated with a registered composition-capable backend. A
claim no registered composition-capable backend can discharge settles
`unsupported` with a warning naming the required capability.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-348-AC-1 | A system obligation over components already independently verified at their declared interfaces is discharged by composing those results, without re-verifying the composed whole as one obligation. | Test (TC-251) |
| FR-348-AC-2 | An interface mismatch between a component's proved obligation and what the composition rule requires refutes, naming the mismatching interface and component. | Test (TC-251) |
| FR-348-AC-3 | Composition is lowered as one target-neutral obligation and negotiated with a registered composition-capable backend rather than re-verified from scratch. | Test (TC-251) |
| FR-348-AC-4 | A claim no registered composition-capable backend can discharge settles `unsupported` with a warning naming the required capability. | Test (TC-251) |

## Dependencies

- [AD-001](../../assurance/AD-001-composed-native-language.md) supplies the
  composed-definition architecture whose components this requirement verifies
  compositionally.
- [AD-009](../../assurance/AD-009-verification-method-plans.md) supplies the
  verification-method and plan architecture this obligation is planned under.
- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  per-item provider capability negotiation this requirement uses.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- [AD-013](../../assurance/AD-013-unbounded-declaration-architecture.md)
  supplies the unbounded-declaration and quantifier-negotiation
  architecture this requirement is allocated to.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
