---
id: SR-2026103011
title: "scope-boundary review of STD-617 nested equality identity"
type: SpecReview
analysis: scope-boundary
scope: "agent-ix/quire-specification; FR-322 checked_term_data equality versus protocol_locations responsibility"
review_set: subset
relationships:
  - { target: ix://agent-ix/quire-specification/FR-322, type: reviews }
---

## Summary

Ticket: STD-617. PR: agent-ix/quire-specification#558. No independent defects found in this method’s scope. Equality identity is expressed as application plus occurrence plus term_path; zero-based application-argument traversal and numeric path ordering are specified. Protocol locations retain a separate per-application term_ordinal. The #556 carrier key is extended, while #557’s target union is disjoint from the equality member.

## Verdict

No findings in this method’s scope. The reviewed carrier and boundaries are internally coherent, subject to the EQ-R6 vector issue recorded in the base/failure-domain/evidence/object review artifacts.

## Findings

| ID | Severity | Summary | Refs |
| --- | --- | --- | --- |
| FND-001 | low | No findings (placeholder) | - |

## Scope

| Unit | Path | Role | Excerpt |
| --- | --- | --- | --- |
| FR-322-AC-288 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined | `Equality records ... preserve the exact root application/occurrence, term_path ... Positive equality vectors include nested equalities under and roots ...` |
| FR-322-AC-289 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined | `The reader joins each equality row to one local root expression/application node ... follows term_path by zero-based application-argument indexes ... path ending at a non-equality application ... refuses ...` |
| FR-322-AC-292 | spec/objects/interfaces/FR-322-checked-package-artifact.md | examined | `Schema-valid reader mutations cover ... a path ending at a non-equality application ... Reader-level mutations have the expected refusal code, cause and exact JSON pointer.` |
| TC-233 | spec/test-cases/TC-233-published-checked-package-and-scalar-contract-vectors.md | examined | `A path to a non-equality application refuses ... duplicate (...) rows refuse ...` |

## Re-review at a2a3d2f977594396491805eb4e1c93becfdafabe

The typed selectors cover the general `CheckedTerm` traversal needed for nested equality while leaving protocol-location identity on `term_ordinal`. No new scope-boundary defect was found; EQ-R6 remains unresolved.
