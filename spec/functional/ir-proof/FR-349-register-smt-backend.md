---
id: FR-349
title: "Register an SMT proof backend"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/FR-036
    type: references
  - target: ix://agent-ix/quire-specification/FR-290
    type: references
  - target: ix://agent-ix/quire-specification/FR-346
    type: references
  - target: ix://agent-ix/quire-specification/FR-347
    type: references
  - target: ix://agent-ix/quire-specification/FR-348
    type: references
  - target: ix://agent-ix/quire-specification/FR-147
    type: references
---

# FR-349: Register an SMT proof backend

## Description

When the target-neutral contract package is negotiated per AD-010, an SMT
backend (cvc5, with Z3 as an independent cross-check) SHALL register under the
same exact provider contract as Kani and advertise which of unbounded
quantification, induction, composition and mathematical-integer capabilities it
discharges.

## Inputs

Target-neutral IR items carrying an FR-346 quantification obligation, an
FR-347 induction obligation, an FR-348 composition obligation or an FR-147
mathematical-integer obligation, plus the registered backend's advertised
capability set and toolchain/options pins.

## Outputs

A per-item `supported`, `requires_bound`, `unsupported` or `invalid_input`
disposition before generation, and, for an accepted item, `proved`, `refuted`
with a replayable witness or counterexample, inconclusive, or failed.

## Behavior

The SMT backend registers under the one exact provider contract AD-010
defines; it is a registrant alongside Kani, never a second shape. Registration
advertises exactly which capabilities among unbounded quantification (FR-346),
induction (FR-347), composition (FR-348) and mathematical-integer domains
(FR-147) the backend discharges; an unadvertised capability is not negotiated
to it. Z3 runs as an independent cross-check of cvc5's verdict on the same
query; a cvc5/Z3 verdict disagreement is retained as an explicit parity
failure rather than silently preferring either solver. Counterexamples and
witnesses retain the FR-036 original source spans and replay through
`runtime::execute` per AD-010. A claim whose required capability no
registered backend advertises settles `unsupported` with a warning naming the
required capability; this absence path is unaffected by whether the SMT
backend is registered for other capabilities.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-349-AC-1 | The SMT backend registers under the same exact provider contract as Kani, with no second contract shape. | Test (TC-266) |
| FR-349-AC-2 | Registration advertises an exact capability subset among quantification, induction, composition and mathematical integers; a claim outside the advertised subset is not dispatched to it. | Test (TC-266) |
| FR-349-AC-3 | A cvc5/Z3 verdict disagreement on the same query is retained as an explicit parity failure and never silently resolved to either solver's verdict. | Test (TC-266) |
| FR-349-AC-4 | A proved or refuted result retains a replayable witness or counterexample carrying its original FR-036 source spans. | Test (TC-266) |
| FR-349-AC-5 | A claim whose required capability no registered backend advertises settles `unsupported` with a warning naming the required capability. | Test (TC-266) |

## Dependencies

- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  one exact provider contract this backend registers under.
- [FR-290](../../objects/protocol/FR-290-protocol-claim-kind.md) supplies the
  backend-absence policy and capability-advertisement precedent this
  requirement follows.
- [FR-036](../foundation/FR-036-retain-lexical-source-locations.md) supplies
  the original source spans this requirement's counterexamples and witnesses
  retain.
- [FR-346](../expressions/FR-346-evaluate-unbounded-quantification.md),
  [FR-347](../analysis/FR-347-admit-inductive-verification.md) and
  [FR-348](../analysis/FR-348-admit-compositional-verification.md) define the
  obligations this backend may register to discharge.
- [FR-147](../expressions/FR-147-evaluate-integer-division-domains.md) defines
  the mathematical-integer domain this backend may register to discharge.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
