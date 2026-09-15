---
id: FR-307
title: "Package reusable semantic and domain libraries"
type: FR
relationships:
  - target: ix://agent-ix/quire-specification/FR-002
    type: references
  - target: ix://agent-ix/quire-specification/FR-132
    type: references
---

# FR-307: Package reusable semantic and domain libraries

## Description

When importing a reusable Quire library, the linker SHALL resolve its exported
declarations, semantic definitions, model dependencies and compatibility/migration
contracts through the package lock.

## Inputs

Library package, export table, dependencies, version constraints and migrations.

## Outputs

Resolved qualified exports and dependency closure, or conflict/cycle/version/
migration refusal.

## Behavior

Libraries may export types, functions, predicates, protocols, plans and view
definitions without acquiring a second source authority. Imports are qualified;
diamond dependencies must select byte-identical definitions or an explicit
compatible unification. Migration emits a new subject and correspondence.

## Acceptance Criteria

| ID | Criteria | Verification |
| --- | --- | --- |
| FR-307-AC-1 | A compatible dependency diamond resolves one exact export identity and reproducible lock. | Test (TC-227) |
| FR-307-AC-2 | Conflicting definitions, import cycles or ambiguous unqualified names refuse with dependency paths. | Test (TC-227) |
| FR-307-AC-3 | Migration preserves old/new identities and does not relabel historical evidence. | Test (TC-227) |

## Dependencies

The selected subsystem architecture and every typed frontmatter relationship
are upstream contracts. Downstream implementation and qualification work must
preserve those exact selections and cannot infer support from this specification.
