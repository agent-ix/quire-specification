---
id: FR-310
title: "Link every semantic subject to assurance obligations"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/US-011
    type: implements
  - target: ix://agent-ix/quire-specification/AD-011
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

## Evidence graph contract

Graph nodes use typed identities for source document, package, model/binding,
clause, semantic subject, implementation, method/provider, run, artifact,
evidence and decision. Edges use the closed relations `derived-from`,
`evaluates`, `implements`, `generated-by`, `replays`, `supports`, `contradicts`
and `decides`, each with scope and provenance. A discharge path must begin at
the exact requirement/claim, reach independently produced evidence and end at
an attributed decision; cycles without an independent evidence node confer no
support.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-310-AC-1 | Every subsystem result traces to exact requirement, assumptions, method/provider and semantic subject. | Test (TC-230) |
| FR-310-AC-2 | Cross-kind identity substitution or stale subject revision refuses evidence attachment. | Test (TC-230) |
| FR-310-AC-3 | Unsupported circular discharge remains unresolved and visible. | Test (TC-230) |

## Dependencies

- [Complete-V1 grammar](../../../proposals/quire-v1/shared-grammar.md) owns every
  source form used by this requirement.
- [Subsystem and interface index](../../subsystems/index.md) identifies the
  typed producer/consumer boundary and dependency direction.
- Every requirement linked in frontmatter is a normative prerequisite.
  Implementation ordering may add enablement edges but cannot change these
  semantics or infer implementation availability from specification acceptance.
