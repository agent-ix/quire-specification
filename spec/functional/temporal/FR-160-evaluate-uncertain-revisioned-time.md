---
id: FR-160
title: "Evaluate uncertain, partial and revisioned temporal inputs"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-002
    type: references
  - target: ix://agent-ix/quire-specification/FR-113
    type: references
---

# FR-160: Evaluate uncertain, partial and revisioned temporal inputs

## Description

When temporal inputs are partial, clock-uncertain or superseded, the evaluator SHALL propagate their exact possibility and revision sets into the temporal result.

## Inputs

Checked formula, known/missing/conflicting atom valuations, exact clock intervals,
observation revisions, progress/closure and resource limits.

## Outputs

Definite true/false, pending/indeterminate, incomplete, refused or failed result
with revision lineage.

## Behavior

Partial atoms carry possible truth values and reasons. Clock intervals use exact
unit arithmetic and establish order only when intervals require it. A revisioned
repair invalidates the affected suffix/window, recomputes under the same package
and links retained superseded and replacement results. Missing values and
midpoint guesses never become false facts.

## Possibility and revision semantics

An atom carries a nonempty possibility set over `{true, false}` plus a reason:
known values are singleton sets; missing and conflicting evidence retain both
possibilities but have distinct reasons. Boolean and temporal operators lift
pointwise over all consistent valuations. A singleton result is definite; a
two-valued result is `pending` when declared progress could resolve it and
`indeterminate` when the closed evidence remains conflicting. An empty
possibility set is an inconsistent-input refusal, not false.

An event time is an exact closed interval `[earliest, latest]` in a canonical
unit. Event `a` is definitely before `b` only when `a.latest < b.earliest`; it is
possibly before when `a.earliest < b.latest`. Overlap preserves both admissible
orders. A repair identifies the superseded revision and atom/event identities,
invalidates their temporal dependency closure, and recomputes that closure in
event-time order. Unaffected results retain their identities; replaced results
link both the prior and replacement evidence digests.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-160-AC-1 | Known, missing and conflicting valuations produce their sound multi-valued temporal outcomes. | Test (TC-199) |
| FR-160-AC-2 | Overlapping clock intervals retain ordering uncertainty while disjoint intervals establish the required order. | Test (TC-199) |
| FR-160-AC-3 | An out-of-order revision recomputes exactly the affected result region and preserves old/new lineage. | Test (TC-199) |
| FR-160-AC-4 | Missing and conflicting atoms may share a possibility set but remain distinguishable by reason and closure behavior. | Test (TC-199) |
| FR-160-AC-5 | Interval ordering uses endpoints exactly and never substitutes a midpoint or ingestion order. | Test (TC-199) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
