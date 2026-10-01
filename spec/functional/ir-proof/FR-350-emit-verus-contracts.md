---
id: FR-350
title: "Emit Verus contracts from QSL state contracts"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/FR-196
    type: references
  - target: ix://agent-ix/quire-specification/FR-353
    type: references
  - target: ix://agent-ix/quire-specification/FR-354
    type: references
---

# FR-350: Emit Verus contracts from QSL state contracts

## Description

When Verus generation is requested for a named implementation Rust function,
the backend SHALL derive its `requires`, `ensures` and loop invariant clauses
from the function's bound QSL state contract and record the Verus
version and options it ran.

## Inputs

Target-neutral items carrying a QSL state contract (pre/post obligations and
loop invariants), the named implementation Rust function the contract is
bound to through the FR-353 abstraction relation, and the Verus options.

## Outputs

Rust source annotated with Verus `requires`/`ensures`/`invariant` clauses and
complete accounting, a `requires_bound`/`unsupported` disposition, or
generation failure.

## Behavior

Verus registers under the same AD-010 provider contract as Kani and the SMT
backend. Emission targets a named implementation Rust function only; an
unnamed or synthesized target is never emitted onto. Each `requires` clause
encodes the state contract's precondition over the function's parameters
under the FR-353 abstraction relation; each `ensures` clause encodes the
postcondition over parameters, result and any framed state; each loop
invariant encodes the contract's declared invariant at the loop's admitted
state boundary. A state element the abstraction relation does not bind is
never emitted onto; that case is FR-353's refusal, not this requirement's
concern. Verus verification of an emitted function is negotiated per item
through the same capability-negotiation path as Kani and SMT: `supported`,
`requires_bound`, `unsupported` or `invalid_input` before generation, and
proved/refuted/inconclusive/failed after. A refuted clause returns with a
replayable counterexample; the evidence names the Verus version and options
it ran for every emitted function. When Verus itself is not
registered, emission is not blocked as `unsupported` on that account alone: a
claim whose model-level obligation the registered SMT backend already
settled `proved` keeps that settlement's FR-354 trust basis, recorded as
`solver-asserted` with a warning naming the absent Verus verification
capability, distinct from a claim whose required capability no registered
backend advertises at all, which remains `unsupported`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-350-AC-1 | Emitted `requires`/`ensures`/loop-invariant clauses correspond exactly to the bound QSL state contract's precondition, postcondition and invariant. | Test (TC-267) |
| FR-350-AC-2 | Emission targets only a named implementation Rust function; no clause is emitted onto an unnamed or synthesized target. | Test (TC-267) |
| FR-350-AC-3 | Verus registers under the same AD-010 provider contract as Kani and the SMT backend, with no second contract shape. | Test (TC-267) |
| FR-350-AC-4 | A refuted clause returns a replayable counterexample for the emitted function. | Test (TC-267) |
| FR-350-AC-5 | A claim Verus cannot discharge settles `unsupported` with a warning naming the required capability, or `requires_bound` when a finite bound is missing. | Test (TC-267) |
| FR-350-AC-6 | When Verus is not registered, a claim the registered SMT backend already settled `proved` keeps its FR-354 `solver-asserted` trust basis and records a warning naming the absent Verus verification capability, rather than settling `unsupported`. | Test (TC-267) |

## Dependencies

- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  one exact provider contract this backend registers under.
- [FR-196](FR-196-generate-kani-contracts.md) supplies the precedent
  contract-generation shape (preconditions, postconditions, invariants, frame
  conditions, evidence accounting) this requirement follows for Verus.
- [FR-353](FR-353-declare-abstraction-relation.md) supplies the
  model-to-implementation binding this requirement's emission targets and
  whose absence refuses emission.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
