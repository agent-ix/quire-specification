---
id: FR-191
title: "Analyze relational and finite-trace-set hyperproperties"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
---

# FR-191: Analyze relational and finite-trace-set hyperproperties

## Description

When a relational or hyperproperty request is checked, the analyzer SHALL bind
each quantified execution/trace variable, correspondence and observation scope
explicitly.

## Inputs

Checked relational syntax, paired/multiple execution subjects, trace domains,
quantifier prefix, alignment relation and finite analysis bounds.

## Outputs

Proved, refuted with paired/trace-set counterexample, inconclusive, unsupported
or failed result.

## Behavior

Relational expressions compare typed values across named executions only.
Finite-trace-set hyperproperties quantify over the declared finite trace domain
and preserve quantifier order. Trace alignment, declassification/assumptions and
population completeness are explicit. No execution is duplicated or correlated
by local name alone.

## Relational semantics

A trace variable ranges over one declared closed trace population. Quantifiers
are evaluated left to right over that population, preserving repetitions only
when distinct trace identities are selected. Expressions may compare values at
explicit aligned positions or correlated event identities. Alignment is a
total relation for every value the formula reads; an absent or ambiguous match
is incomplete. Declassification and environment assumptions restrict the
population before quantification and are part of result identity.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-191-AC-1 | A two-execution relational property uses explicit input/output correspondences and returns a paired witness/counterexample. | Test (TC-213) |
| FR-191-AC-2 | Reversing a mixed quantifier prefix can change the finite trace-set result and is preserved. | Test (TC-213) |
| FR-191-AC-3 | Missing trace closure/alignment or exceeded domain bounds returns incomplete/requires-bound. | Test (TC-213) |
| FR-191-AC-4 | Universal/existential quantifier order is preserved and ranges over exact trace identities from the closed population. | Test (TC-213) |
| FR-191-AC-5 | An absent or ambiguous alignment returns incomplete without comparing unrelated positions by index or name. | Test (TC-213) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
