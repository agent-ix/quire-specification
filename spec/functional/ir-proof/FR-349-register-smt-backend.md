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

When the target-neutral contract package is negotiated per AD-010, the SMT
backend (cvc5, with Z3 as an independent cross-check) SHALL register one
descriptor under the same exact provider contract as Kani, carrying its FR-290
(capability kind, mode) pairs and its declared obligation-form set.

If an item's obligation form is outside that declared set, then the SMT
negotiation arm SHALL settle the item `unsupported` with a warning naming the
form.

## Inputs

Target-neutral IR items carrying an FR-346 quantification obligation, an
FR-347 induction obligation, an FR-348 composition obligation or an FR-147
mathematical-integer obligation, plus the registered backend's advertised
FR-290 (capability kind, mode) pairs and solver options.

## Outputs

A per-item `supported`, `requires_bound`, `unsupported` or `invalid_input`
disposition before generation, and, for an accepted item, `proved`, `refuted`
with a replayable witness or counterexample, inconclusive, or failed.

## Behavior

The SMT backend registers under the one exact provider contract AD-010
defines; it is a registrant alongside Kani, never a second shape. Registration
advertises exact FR-290 (capability kind, mode) pairs; an item whose required
kind it does not advertise has it outside its candidate set. Unbounded
quantification (FR-346), induction (FR-347), compositional discharge (FR-348)
and mathematical-integer domains (FR-147) are proof techniques, not capability
kinds. The backend descriptor declares the subset of these four obligation
forms it discharges; its negotiation arm settles an item whose obligation form
is outside the declared set `unsupported` with a warning naming the form, cause
`unsupported_projection`/`unsupported-requested-capability`. Z3 is the cross-check inside the one SMT descriptor: it advertises no FR-290
pairs of its own and is never a candidate. Z3 runs as an independent
cross-check of cvc5's verdict on the same query; a cvc5/Z3 verdict disagreement settles the claim `inconclusive` and is
retained as an explicit parity failure naming both verdicts, rather than
silently preferring either solver. When Z3 is not configured in the SMT descriptor, no cross-check
runs: cvc5's verdict settles the claim on its own, and the settlement carries
a warning naming the absent cross-check; this is a warned outcome, never a
hold on cvc5's settlement. A configured Z3 whose tool probe finds it
missing, errors or exceeds its limit is the same warned absent
cross-check: cvc5's verdict settles the claim, and the item never records the
FR-290 tool-absence result `unsupported` on Z3's account. Counterexamples and witnesses retain the FR-036
original source spans and replay through the QSL complete-V1 executor per AD-010
(FR-197, AD-016 arrow 7). A
claim whose required capability kind no registered backend advertises settles
`unsupported` with a warning naming the required capability kind; this absence
path is unaffected by whether the SMT backend is registered for other
capability kinds.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-349-AC-1 | The SMT backend registers under the same exact provider contract as Kani, with no second contract shape. | Test (TC-266) |
| FR-349-AC-2 | Registration advertises exact FR-290 (capability kind, mode) pairs and a declared obligation-form set; an item whose kind is unadvertised is not a candidate for it, and an item whose obligation form is outside the declared set settles `unsupported` at its negotiation arm with a warning naming the form and cause `unsupported_projection`/`unsupported-requested-capability`. | Test (TC-266) |
| FR-349-AC-3 | A cvc5/Z3 verdict disagreement on the same query settles the claim `inconclusive`, is retained as an explicit parity failure naming both verdicts, and is never resolved to either solver's verdict alone. | Test (TC-266) |
| FR-349-AC-4 | A proved or refuted result retains a replayable witness or counterexample carrying its original FR-036 source spans. | Test (TC-266) |
| FR-349-AC-5 | A claim whose required capability kind no registered backend advertises settles `unsupported` with a warning naming the required capability kind. | Test (TC-266) |
| FR-349-AC-6 | Z3's absence from the SMT descriptor, or a missing Z3 tool at probe, does not block, hold or turn `unsupported` cvc5's settlement of a claim within its advertised (capability kind, mode) pairs; the settlement carries a warning naming the absent cross-check. | Test (TC-266) |

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
