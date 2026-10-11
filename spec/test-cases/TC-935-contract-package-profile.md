---
id: TC-935
title: "ContractPackage retains the explicit lowering-profile reference"
type: TC
relationships:
  - target: ix://agent-ix/quire-specification/FR-330
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-1
    type: verifies
  - target: ix://agent-ix/quire-specification/FR-330-AC-5
    type: verifies
---
# TC-935: ContractPackage retains the explicit lowering-profile reference

## Description

Verify that the explicit `package.lowering_profile` reference is retained
separately from the source selections and that caller resource limits are not
package settings.

## Test Procedure

Use an admitted checked package whose lock selects source profiles with
distinct semantic laws, and lower the same request under one explicit lowering
profile reference with two different caller resource limits high enough for
both runs to complete. For one represented request, compare the emitted typed
node with its originating FR-322 checked-package node. Inspect the closed
`{authority, identity}` reference, lock, typed graph values, and package
identity. Separately add an extra member to the profile reference by adding
`"settings": {}` inside `package.lowering_profile`, and add an extra free-standing
`settings` member at the ContractPackage root. Vary only the caller limits.

## Expected Results

Each admitted result carries exactly the explicit `package.lowering_profile`
reference in its closed `{authority, identity}` form, separately from the
source-profile selections; semantic values are those admitted by the lock and
typed graph and match the originating checked-package node, with no default or
second settings map. An extra profile-reference member is refused as
`invalid_package`/`unknown-member` at `/package/lowering_profile`; the extra
root `settings` member is refused as `invalid_package`/`unknown-member` at
`""`. A change to an included semantic value changes the graph and identity.
Changing only caller resource limits across those successful runs does not
change ContractPackage members or identity.
