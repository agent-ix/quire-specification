---
id: FR-133
title: "Extend declarations and semantic families uniformly"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-032
    type: references
  - target: ix://agent-ix/quire-specification/FR-131
    type: references
---

# FR-133: Extend declarations and semantic families uniformly

## Description

When adding a declaration form, semantic family or provider contract, the
extension mechanism SHALL require a new exact definition, dependency edges,
typed package node and compatibility proof.

## Inputs

Historical definitions, proposed definition, dependency graph and compatibility
vectors.

## Outputs

An explicitly selectable extension or a cycle, collision or compatibility
refusal.

## Behavior

An extension cannot steal syntax, reinterpret historical bytes, weaken inherited
rules, mutate another family AST or gain permission from an installed backend.
Consumers that do not select or understand it preserve the source and report
the unknown required capability.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-133-AC-1 | A versioned extension with an acyclic dependency graph and passing compatibility corpus is selectable without changing historical results. | Test (TC-183) |
| FR-133-AC-2 | A dependency cycle, identifier collision or semantic weakening refuses with all involved definitions. | Test (TC-183) |
| FR-133-AC-3 | An older consumer rejects an unknown required extension while retaining its source and exact identity. | Test (TC-183) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
