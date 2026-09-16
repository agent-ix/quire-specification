---
id: FR-010
title: "Evaluate finite parent reachability"
type: FR
relationships:
  - target: "ix://agent-ix/quire-specification/US-003"
    type: traces_to
---
# FR-010: Evaluate finite parent reachability

## Description

When parent reachability is evaluated, the evaluator shall search for the target along one or more admitted edges.

## Inputs

Complete finite population, same-observation typed start/target and an optional reference field.

## Outputs

Exact reachability value or resource incompleteness.

## Behavior

Visited identities suppress repeated expansion after the reached target is tested. A self-loop and a cycle returning to start therefore satisfy reachability to self. Optional absence terminates traversal. Runtime work ceilings never become an implicit authored path-depth bound.

Under a package selecting `quire.model.complete/v1`, V1-TYPE-024 traversal of
typed model relationships and inheritance is owned by
[FR-043](FR-043-evaluate-finite-graph-relations.md): `reaches(a, b, e)` over a
closed population binding keeps this requirement's semantics (the start is not
reached without an edge, and a self-loop or a cycle returning to the start
reaches it), with conformance to the edge owner, most-specific-type identity and
the `graph.*` charges defined there and by `quire.value.accounting/v1`.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-010-AC-1 | An isolated object does not reach itself. | Test |
| FR-010-AC-2 | A self-loop reaches itself. | Test |
| FR-010-AC-3 | A two-object cycle reaches its start. | Test |
| FR-010-AC-4 | Exhausted graph-expansion budget receives resource_exhausted. | Test |

## Dependencies

- [US-003](../../usecase/US-003-assess-finite-state.md) supplies the user need.
- [Detailed contract or implementation evidence](../../../proposals/state-core/state-semantics.md) supplies the scoped context.

## Status

Draft. Specification review and prerequisite acceptance remain distinct from existing code/tests. No acceptance criterion is claimed satisfied solely because this artifact has been authored.
