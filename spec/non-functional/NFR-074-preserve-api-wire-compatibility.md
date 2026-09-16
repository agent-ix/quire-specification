---
id: NFR-074
title: "Preserve lifecycle API and wire compatibility"
type: NFR
quality_attribute: compatibility
relationships:
  - target: ix://agent-ix/quire-specification/FR-300
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-133
    type: constrains
  - target: ix://agent-ix/quire-specification/FR-322
    type: constrains
---

# NFR-074: Preserve lifecycle API and wire compatibility

## Statement

The complete-V1 lifecycle SHALL preserve historical selected semantics and use
explicit version negotiation for every incompatible Rust API or wire change.

## Scope

Public Rust traits/types, CLI machine output, WASM, plugin/provider wires,
packages, counterexamples and evidence intake.

## Rationale

Compatibility must not be achieved by silently accepting changed or incomplete
meaning.

## Measurement and Evaluation

| Metric | Target | Threshold | Method |
| --- | --- | --- | --- |
| Historical corpus semantic regressions | 0 | 0 | Compatibility corpus |
| Incompatible changes with new selection/migration | 100% | 100% | API/wire diff inspection |

## Verification

Run historical and complete-V1 corpora through old/new readers and verify exact
acceptance, explicit refusal and migration correspondence.

For CheckedPackage V1 to V2, migration is a re-link and re-check operation, not
a byte transformation. The migration operation requires the exact authored
source documents, selected definition artifacts and compiled-model documents
named by the V1 lock, executes the V2 linker/checker, and on success emits both
the separately identified V2 package and a
`quire.checked-package-migration-correspondence/v1` record. That record binds the
source V1 package key, target V2 package key and a total ordered mapping from
each V1 node key to the corresponding V2 node key with basis
`relinked-identical` or `relinked-rekeyed`; it is excluded from both package
identities.

Missing, stale, ambiguous or incompatible reconstruction inputs refuse before a
V2 package is exposed. The migration result is the closed typed
`MigrationOutcome` sum `relinked { correspondence }` or
`refused { code, subject }`, whose code vocabulary is exactly
`migration_input_missing`, `migration_input_stale`,
`migration_input_ambiguous`, `migration_target_incompatible` and
`migration_byte_only_reconstruction`; supplying only V1 checked-package bytes is
the distinct byte-only cause, not a missing-input alias. A migration must not infer owner, qualified declaration,
enum-member order, dimension terms, unit conversion or other V2 nominal evidence
from V1 checked-package bytes alone.
