---
id: FR-310
title: "Link every semantic subject to assurance obligations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/AD-011
    type: references
  - target: ix://agent-ix/quire-specification/FR-017
    type: references
---

# FR-310: Link every semantic subject to assurance obligations

## Description

When recording verification evidence, the producer SHALL link the exact semantic
subject independently to its requirements, assumptions, methods, provider runs
and evidence artifacts.

## Inputs

Semantic subject identity, requirement/assumption/method identities, result and
artifact references.

## Outputs

Typed traceability graph or identity/kind/cycle refusal.

## Behavior

No source, package, clause, method, run, artifact or evidence identity substitutes
for another. Links name their relation and scope. One item may address multiple
obligations but each discharge is explicit. Circular evidence claims cannot
establish themselves.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-310-AC-1 | Every subsystem result traces to exact requirement, assumptions, method/provider and semantic subject. | Test (TC-230) |
| FR-310-AC-2 | Cross-kind identity substitution or stale subject revision refuses evidence attachment. | Test (TC-230) |
| FR-310-AC-3 | Unsupported circular discharge remains unresolved and visible. | Test (TC-230) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
