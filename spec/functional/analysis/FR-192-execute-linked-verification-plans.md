---
id: FR-192
title: "Execute linked verification plans"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-009
    type: references
---

# FR-192: Execute linked verification plans

## Description

When executing a verification plan, the planner SHALL resolve every referenced
clause, method, domain, assumption, dependency, bound and evidence output before
dispatching a provider.

## Inputs

Versioned plan, checked package, method-provider manifests, ecosystem lock and
resource policy.

## Outputs

Dependency-ordered method requests/results and evidence obligations, or typed
plan/link/cycle refusal.

## Behavior

Plans are linked artifacts outside the clause AST. Method selection cannot alter
source admission. Dependencies form an explicit graph; unsupported methods and
failed prerequisites preserve independent sibling results. Each output names the
requirement, assumption, method, provider and semantic subject it addresses.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-192-AC-1 | A complete acyclic plan dispatches methods in dependency order and accounts for every requested evidence output. | Test (TC-214) |
| FR-192-AC-2 | A missing reference/provider or unsupported method is reported per request without deleting independent work. | Test (TC-214) |
| FR-192-AC-3 | A circular dependency without a selected sound fixed-point rule refuses before provider execution. | Test (TC-214) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
