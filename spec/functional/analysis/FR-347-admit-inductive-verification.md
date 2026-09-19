---
id: FR-347
title: "Admit inductive verification obligations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/FR-144
    type: references
  - target: ix://agent-ix/quire-specification/FR-143
    type: references
  - target: ix://agent-ix/quire-specification/FR-171
    type: references
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
  - target: ix://agent-ix/quire-specification/AD-010
    type: references
  - target: ix://agent-ix/quire-specification/AD-013
    type: references
---

# FR-347: Admit inductive verification obligations

## Description

When a verification obligation ranges over an unbounded collection, a
recursive value type or another well-founded, unboundedly-sized domain, the
verification planner SHALL admit an inductive proof: an authored base case
and an inductive step preserving the property across the domain's
well-founded construction.

## Inputs

An obligation stated over an FR-144 unbounded collection type, an FR-143
recursive value type, or another domain with an authored well-founded order;
an authored base case; an authored inductive step; the well-founded order the
step decreases under.

## Outputs

Proved, refuted with a base-case or inductive-step counterexample,
inconclusive, `unsupported`, or failed.

## Behavior

FR-171 already admits a loop with no authored finite maximum when it carries a
proved invariant and well-founded variant instead; this requirement
generalizes that precedent from one control construct to every domain that is
unbounded or recursively defined. An obligation over such a domain is not
discharged by exhaustive enumeration, because no finite enumeration is
promised to exist. Instead the planner requires the domain's well-founded
order, an authored base case covering its minimal elements and an authored
inductive step showing the property is preserved from every smaller element
to the next; both are lowered as one target-neutral induction obligation per
AD-009/AD-010 and negotiated with a registered induction-capable backend. A
provider that discharges both parts returns proved; a failing base case or
step returns refuted with its counterexample; an unproved obligation is
inconclusive rather than silently narrowed to finitely many checked
instances. A claim no registered induction-capable backend can discharge
settles `unsupported` with a warning naming the required capability.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-347-AC-1 | An obligation over an FR-144 unbounded collection or an FR-143 recursive value type is admitted as an inductive proof: an authored base case plus an authored inductive step over the domain's well-founded order. | Test (TC-250) |
| FR-347-AC-2 | The base case and inductive step are lowered as one target-neutral induction obligation and negotiated with a registered induction-capable backend, never discharged by exhaustive enumeration. | Test (TC-250) |
| FR-347-AC-3 | A failing base case or a failing inductive step returns refuted with its own counterexample, distinguishing which part failed; an unproved obligation is inconclusive rather than narrowed to finitely many checked instances. | Test (TC-250) |
| FR-347-AC-4 | A claim no registered induction-capable backend can discharge settles `unsupported` with a warning naming the required capability. | Test (TC-250) |

## Dependencies

- [FR-171](../protocol/FR-171-evaluate-dynamic-control.md) supplies the
  proved-invariant/well-founded-variant precedent this requirement
  generalizes beyond bounded iteration.
- [FR-144](../type-model/FR-144-preserve-complete-collection-algebra.md)
  defines the unbounded collection types this requirement's domain may range
  over.
- [FR-143](../type-model/FR-143-evaluate-record-tuple-recursive-values.md)
  defines the recursive value types this requirement's domain may range over.
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
