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

## Plan execution graph

Each plan node binds one requirement/claim identity, semantic-subject digest,
method profile, provider identity/options, assumptions, limits and expected
evidence kind. Edges are typed `requires`, `validates`, `replays` or `aggregates`.
The coordinator topologically dispatches ready nodes, records a result for every
node and never substitutes one evidence kind for another. A failed `requires`
edge blocks only descendants; independent nodes still run. Aggregation applies
an authored decision rule after all required inputs have terminal dispositions.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-192-AC-1 | A complete acyclic plan dispatches methods in dependency order and accounts for every requested evidence output. | Test (TC-214) |
| FR-192-AC-2 | A missing reference/provider or unsupported method is reported per request without deleting independent work. | Test (TC-214) |
| FR-192-AC-3 | A circular dependency without a selected sound fixed-point rule refuses before provider execution. | Test (TC-214) |
| FR-192-AC-4 | Every plan node yields exactly one accounted disposition, including blocked, unsupported and not-run nodes. | Test (TC-214) |
| FR-192-AC-5 | Failure blocks only typed descendants and cannot erase or relabel independent sibling evidence. | Test (TC-214) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
