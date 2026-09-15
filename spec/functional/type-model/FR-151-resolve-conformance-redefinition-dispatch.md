---
id: FR-151
title: "Resolve conformance, redefinition and closed dispatch"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-010
    type: implements
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
  - target: ix://agent-ix/quire-specification/FR-150
    type: references
---

# FR-151: Resolve conformance, redefinition and closed dispatch

## Description

When checking specialization or a dispatched call, the model checker SHALL
apply the selected conformance, subsetting, redefinition and method-precedence
rules over a closed declaration set.

## Inputs

Original/effective declarations, inheritance graph, variance rules,
subsetting/redefinition edges and complete method set.

## Outputs

Validated effective declarations and selected method, or located cycle,
incompatibility, ambiguity or incomplete-closure refusal.

## Behavior

Substitutability checks value, multiplicity, pre/postcondition and effect
variance under the selected profile. Redefinition retains correspondence and
cannot weaken inherited invariants. Dispatch selects the unique most-specific
applicable method; registration or source order never resolves ambiguity.

## Conformance and dispatch rules

A subtype preserves inherited invariants, weakens preconditions, strengthens
postconditions, accepts parameter types contravariantly, returns result types
covariantly and introduces no effect outside the inherited effect set.
Multiplicity conformance requires the subtype lower bound to be at least the
supertype lower bound and its upper bound to be no greater. Subsetting may only
narrow the represented values. Redefinition must name exactly one inherited
member and satisfy all of these rules.

Dispatch filters the closed method set by declaration identity, arity, parameter
conformance and precondition. Method `a` is more specific than `b` only when all
of `a`'s parameters conform to `b`'s and at least one is strictly narrower. The
unique undominated method is selected; zero candidates, multiple undominated
candidates or an unclosed method set have no dispatch value.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-151-AC-1 | A compatible specialization/redefinition yields one effective member with complete provenance. | Test (TC-196) |
| FR-151-AC-2 | An inheritance cycle, weakened invariant or incompatible variance refuses with the contributing edges. | Test (TC-196) |
| FR-151-AC-3 | Zero or multiple most-specific methods return no dispatch result and identify missing closure or ambiguity. | Test (TC-196) |
| FR-151-AC-4 | Parameter, result, precondition, postcondition, multiplicity and effect variance are checked independently. | Test (TC-196) |
| FR-151-AC-5 | Registration and source order cannot change the selected method or resolve an ambiguity. | Test (TC-196) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
