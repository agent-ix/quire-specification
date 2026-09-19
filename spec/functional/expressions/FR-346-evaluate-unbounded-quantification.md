---
id: FR-346
title: "Evaluate unbounded universal and existential quantification"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/FR-145
    type: references
  - target: ix://agent-ix/quire-specification/FR-144
    type: references
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/AD-013
    type: references
  - target: ix://agent-ix/quire-specification/FR-349
    type: references
---

# FR-346: Evaluate unbounded universal and existential quantification

## Description

When `forall` or `exists` quantifies over a collection whose type declares no
FR-144 bound, the evaluator SHALL negotiate discharge with a registered
quantifier-capable backend rather than visiting occurrences.

## Inputs

A quantified expression (`forall`/`exists`) over a collection-typed binding, the
collection's element type and bound state (present or absent per FR-144), and
the closed predicate expression.

## Outputs

Proved, refuted with a witness (`exists`) or counterexample (`forall`),
inconclusive, `unsupported`, or failed.

## Behavior

FR-145 decides `forall`/`exists` over a bounded collection by visiting each
retained occurrence in FR-144 canonical order and charging its schedule; that
path is unchanged by this requirement. A collection type with no declared
bound carries no promise that a finite occurrence set exists at evaluation
time, so the reference evaluator does not visit occurrences to decide the
quantifier. Instead the claim is lowered as a target-neutral IR obligation per
AD-010 and negotiated per item with a registered quantifier-capable backend
(an SMT-style solver or an equivalent registered provider; Kani is one
possible registrant, never the only one). A provider that establishes the
quantifier over the full declared unbounded domain returns proved, or refuted
with a replayable witness or counterexample; an unproved claim is inconclusive
rather than silently narrowed to a finite sample, and no provider may report
by exhaustive enumeration over an unbounded domain. A claim no registered
quantifier-capable backend can discharge settles `unsupported` with a warning
naming the required capability.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-346-AC-1 | `forall`/`exists` over a bounded collection type continues to be decided by FR-145 occurrence-visiting and its charge schedule, unaffected by this requirement. | Test (TC-249) |
| FR-346-AC-2 | `forall`/`exists` over an unbounded collection type is lowered as a target-neutral IR obligation and negotiated with a registered quantifier-capable backend rather than decided by visiting occurrences. | Test (TC-249) |
| FR-346-AC-3 | A provider that discharges the obligation over the declared unbounded domain returns proved, refuted with a witness or counterexample, or inconclusive, and never substitutes a finite sample for the unbounded domain. | Test (TC-249) |
| FR-346-AC-4 | A claim no registered quantifier-capable backend can discharge settles `unsupported` with a warning naming the required capability. | Test (TC-249) |
| FR-346-AC-5 | When a registered quantifier-capable SMT backend is present, the qualification corpus settles at least one `forall`/`exists` case `proved` and at least one `refuted` through that backend; the AC-4 `unsupported`-with-warning absence path is unchanged when no such backend is registered. | Test (TC-249, TC-266) |

## Dependencies

- [FR-145](FR-145-convert-and-query-collections.md) defines the bounded
  occurrence-visiting quantification this requirement extends to unbounded
  collection types.
- [FR-144](../type-model/FR-144-preserve-complete-collection-algebra.md)
  defines the optional cardinality bound whose absence is this requirement's
  trigger.
- [AD-010](../../assurance/AD-010-target-neutral-ir-backends.md) supplies the
  per-item provider capability negotiation this requirement uses.
- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- [AD-013](../../assurance/AD-013-unbounded-declaration-architecture.md)
  supplies the unbounded-declaration and quantifier-negotiation
  architecture this requirement is allocated to.
- [FR-349](../ir-proof/FR-349-register-smt-backend.md) supplies the
  registered SMT backend that discharges this requirement's `proved`/`refuted`
  cases when registered.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification
  acceptance.
