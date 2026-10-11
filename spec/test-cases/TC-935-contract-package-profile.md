---
id: TC-935
title: "ContractPackage binds meaning to the selected lowering profile"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
---
# TC-935: ContractPackage binds meaning to the selected lowering profile

## Description

Verify that the explicit `package.lowering_profile` and semantic settings have
one authoritative representation and that caller resource limits are not
package settings.

## Test Procedure

Use an admitted checked package whose lock selects source profiles with
distinct semantic laws, and lower the same request under an explicit compatible
lowering profile with two different caller resource limits high enough for
both runs to complete. Inspect the
lowering profile, lock, typed graph values, and package identity. Then choose a
lowering profile that requires a conflicting source semantic selection, add a
duplicate free-standing settings map, and vary only the caller limits.

## Expected Results

Each admitted result names exactly the explicit `package.lowering_profile`
`DefinitionRef`, separately from the source-profile selections; semantic
values are those admitted by the lock and typed graph, with no default or
second settings map.
The conflicting profile is refused before node emission and does not replace
the lock's semantic selections. A change to an included semantic value
changes the graph and identity. A duplicate settings representation is
refused. Changing only caller resource limits across those successful runs
does not change ContractPackage members or identity.
