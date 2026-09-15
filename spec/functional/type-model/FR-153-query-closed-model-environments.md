---
id: FR-153
title: "Query explicit closed model environments"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-043
    type: references
  - target: ix://agent-ix/quire-specification/AD-006
    type: references
---

# FR-153: Query explicit closed model environments

## Description

When evaluating lookup or all-instances, the evaluator SHALL query only the
explicit typed environment and population closure bound to the request.

## Inputs

Qualified model/type identity, environment snapshot, membership/closure
authority and query bounds.

## Outputs

A typed value or bounded collection, unavailable/incomplete result, or refusal.

## Behavior

Lookup uses a qualified key and declares absent-key behavior. `allInstances`
returns the complete selected population only when closure authority is present;
otherwise it is incomplete. Neither operation consults a process-global registry
or invents objects from samples.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-153-AC-1 | Closed all-instances returns every and only member of the selected typed population in deterministic exposure order. | Test (TC-198) |
| FR-153-AC-2 | Unknown closure returns incomplete and a missing qualified key follows its declared absence rule. | Test (TC-198) |
| FR-153-AC-3 | Foreign authority/type keys, duplicate conflicting entries or exceeded bounds refuse without ambient fallback. | Test (TC-198) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
